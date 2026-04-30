use std::borrow::Cow;

use serde_json::Value;

use crate::error::{LiterLlmError, Result};
use crate::provider::{Provider, unix_timestamp_secs};
use crate::types::{ChatCompletionChunk, FinishReason, StreamChoice, StreamDelta, StreamFunctionCall, StreamToolCall};

/// Cohere provider (Command model family).
///
/// Differences from the OpenAI-compatible baseline:
/// - Chat endpoint is `/chat` instead of `/chat/completions`.
/// - Rerank endpoint is `/rerank` instead of the default path.
/// - `stream_options` is an OpenAI-specific field and must be stripped; `stream` is kept (Cohere v2 requires it).
/// - Finish reasons use Cohere-specific names (`COMPLETE`, `MAX_TOKENS`, `TOOL_CALL`).
/// - Usage is reported under `tokens.input_tokens` / `tokens.output_tokens`.
/// - Response may lack `object` and `created` fields.
pub struct CohereProvider;

impl Provider for CohereProvider {
    fn name(&self) -> &str {
        "cohere"
    }

    fn base_url(&self) -> &str {
        "https://api.cohere.com/v2"
    }

    fn auth_header<'a>(&'a self, api_key: &'a str) -> Option<(Cow<'static, str>, Cow<'a, str>)> {
        Some((Cow::Borrowed("Authorization"), Cow::Owned(format!("Bearer {api_key}"))))
    }

    fn matches_model(&self, model: &str) -> bool {
        model.starts_with("command-r") || model.starts_with("command-") || model.starts_with("cohere/")
    }

    fn strip_model_prefix<'m>(&self, model: &'m str) -> &'m str {
        model.strip_prefix("cohere/").unwrap_or(model)
    }

    /// Cohere uses `/chat` instead of `/chat/completions`.
    fn chat_completions_path(&self) -> &str {
        "/chat"
    }

    /// Cohere uses `/rerank` at the v2 base.
    fn rerank_path(&self) -> &str {
        "/rerank"
    }

    /// Strip transport-level parameters that Cohere does not accept in the body.
    ///
    /// Note: Cohere v2 requires `stream` in the body, so only `stream_options`
    /// (an OpenAI-specific field) is removed.
    fn transform_request(&self, body: &mut Value) -> Result<()> {
        if let Some(obj) = body.as_object_mut() {
            obj.remove("stream_options");
        }
        Ok(())
    }

    /// Parse a Cohere v2 streaming SSE event into a `ChatCompletionChunk`.
    ///
    /// Cohere v2 streaming events use a `type` field to distinguish event kinds:
    /// - `stream-start`: beginning of stream, emit role = assistant
    /// - `content-delta`: text content token, extract from `delta.text`
    /// - `tool-call-start`: start of a tool call with id and function name
    /// - `tool-call-delta`: partial tool call arguments
    /// - `tool-call-end`: end of a tool call (skipped)
    /// - `stream-end`: end of stream with finish reason and usage
    fn parse_stream_event(&self, event_data: &str) -> Result<Option<ChatCompletionChunk>> {
        let v: Value = serde_json::from_str(event_data).map_err(|e| LiterLlmError::Streaming {
            message: format!("failed to parse Cohere SSE event: {e}"),
        })?;

        let event_type = v.get("type").and_then(|t| t.as_str()).unwrap_or("");

        match event_type {
            "stream-start" => {
                let id = v.get("generation_id").and_then(|g| g.as_str()).unwrap_or("").to_owned();

                Ok(Some(ChatCompletionChunk {
                    id,
                    object: "chat.completion.chunk".to_owned(),
                    created: unix_timestamp_secs(),
                    model: String::new(),
                    choices: vec![StreamChoice {
                        index: 0,
                        delta: StreamDelta {
                            role: Some("assistant".to_owned()),
                            content: None,
                            tool_calls: None,
                            function_call: None,
                            refusal: None,
                        },
                        finish_reason: None,
                    }],
                    usage: None,
                    system_fingerprint: None,
                    service_tier: None,
                }))
            }

            "content-delta" => {
                let text = v
                    .pointer("/delta/text")
                    .and_then(|t| t.as_str())
                    .unwrap_or("")
                    .to_owned();

                Ok(Some(ChatCompletionChunk {
                    id: String::new(),
                    object: "chat.completion.chunk".to_owned(),
                    created: unix_timestamp_secs(),
                    model: String::new(),
                    choices: vec![StreamChoice {
                        index: 0,
                        delta: StreamDelta {
                            role: None,
                            content: Some(text),
                            tool_calls: None,
                            function_call: None,
                            refusal: None,
                        },
                        finish_reason: None,
                    }],
                    usage: None,
                    system_fingerprint: None,
                    service_tier: None,
                }))
            }

            "tool-call-start" => {
                let index = v.get("index").and_then(|i| i.as_u64()).unwrap_or(0) as u32;
                let tool_id = v.pointer("/delta/id").and_then(|i| i.as_str()).unwrap_or("").to_owned();
                let tool_name = v
                    .pointer("/delta/function/name")
                    .and_then(|n| n.as_str())
                    .unwrap_or("")
                    .to_owned();

                Ok(Some(ChatCompletionChunk {
                    id: String::new(),
                    object: "chat.completion.chunk".to_owned(),
                    created: unix_timestamp_secs(),
                    model: String::new(),
                    choices: vec![StreamChoice {
                        index: 0,
                        delta: StreamDelta {
                            role: None,
                            content: None,
                            tool_calls: Some(vec![StreamToolCall {
                                index,
                                id: Some(tool_id),
                                call_type: Some(crate::types::ToolType::Function),
                                function: Some(StreamFunctionCall {
                                    name: Some(tool_name),
                                    arguments: None,
                                }),
                            }]),
                            function_call: None,
                            refusal: None,
                        },
                        finish_reason: None,
                    }],
                    usage: None,
                    system_fingerprint: None,
                    service_tier: None,
                }))
            }

            "tool-call-delta" => {
                let index = v.get("index").and_then(|i| i.as_u64()).unwrap_or(0) as u32;
                let arguments = v
                    .pointer("/delta/function/arguments")
                    .and_then(|a| a.as_str())
                    .unwrap_or("")
                    .to_owned();

                Ok(Some(ChatCompletionChunk {
                    id: String::new(),
                    object: "chat.completion.chunk".to_owned(),
                    created: unix_timestamp_secs(),
                    model: String::new(),
                    choices: vec![StreamChoice {
                        index: 0,
                        delta: StreamDelta {
                            role: None,
                            content: None,
                            tool_calls: Some(vec![StreamToolCall {
                                index,
                                id: None,
                                call_type: None,
                                function: Some(StreamFunctionCall {
                                    name: None,
                                    arguments: Some(arguments),
                                }),
                            }]),
                            function_call: None,
                            refusal: None,
                        },
                        finish_reason: None,
                    }],
                    usage: None,
                    system_fingerprint: None,
                    service_tier: None,
                }))
            }

            "tool-call-end" => Ok(None),

            "stream-end" => {
                let finish_reason = v
                    .get("finish_reason")
                    .and_then(|r| r.as_str())
                    .map(map_cohere_finish_reason);

                let usage = extract_cohere_stream_usage(&v);

                Ok(Some(ChatCompletionChunk {
                    id: String::new(),
                    object: "chat.completion.chunk".to_owned(),
                    created: unix_timestamp_secs(),
                    model: String::new(),
                    choices: vec![StreamChoice {
                        index: 0,
                        delta: StreamDelta {
                            role: None,
                            content: None,
                            tool_calls: None,
                            function_call: None,
                            refusal: None,
                        },
                        finish_reason,
                    }],
                    usage,
                    system_fingerprint: None,
                    service_tier: None,
                }))
            }

            // Unknown event types are silently skipped.
            _ => Ok(None),
        }
    }

    /// Normalize Cohere response format to OpenAI-compatible JSON.
    ///
    /// - Maps finish reasons: `COMPLETE` -> `stop`, `MAX_TOKENS` -> `length`,
    ///   `TOOL_CALL` -> `tool_calls`.
    /// - Normalizes usage from `tokens.{input,output}_tokens` to
    ///   `usage.{prompt,completion,total}_tokens`.
    /// - Ensures `object` and `created` fields are present.
    fn transform_response(&self, body: &mut Value) -> Result<()> {
        // Map finish reasons in choices.
        if let Some(choices) = body.get_mut("choices").and_then(Value::as_array_mut) {
            for choice in choices {
                if let Some(reason) = choice.get("finish_reason").and_then(Value::as_str) {
                    let mapped = match reason {
                        "COMPLETE" => "stop",
                        "MAX_TOKENS" => "length",
                        "TOOL_CALL" => "tool_calls",
                        other => other,
                    };
                    choice["finish_reason"] = Value::String(mapped.to_owned());
                }
            }
        }

        // Normalize usage from Cohere's `tokens` format.
        if body.get("usage").is_none()
            && let Some(tokens) = body.get("tokens")
        {
            let input = tokens.get("input_tokens").and_then(Value::as_u64).unwrap_or(0);
            let output = tokens.get("output_tokens").and_then(Value::as_u64).unwrap_or(0);
            body["usage"] = serde_json::json!({
                "prompt_tokens": input,
                "completion_tokens": output,
                "total_tokens": input + output,
            });
        }

        // Ensure standard OpenAI fields are present.
        if body.get("object").is_none() {
            body["object"] = Value::String("chat.completion".to_owned());
        }
        if body.get("created").is_none() {
            body["created"] = Value::Number(unix_timestamp_secs().into());
        }

        Ok(())
    }
}

/// Map Cohere finish reason strings to OpenAI-compatible `FinishReason`.
fn map_cohere_finish_reason(reason: &str) -> FinishReason {
    match reason {
        "COMPLETE" => FinishReason::Stop,
        "MAX_TOKENS" => FinishReason::Length,
        "TOOL_CALL" => FinishReason::ToolCalls,
        _ => FinishReason::Other,
    }
}

/// Extract usage from a Cohere `stream-end` event.
///
/// Cohere v2 reports usage under `usage.billed_units.{input_tokens, output_tokens}`.
fn extract_cohere_stream_usage(v: &Value) -> Option<crate::types::Usage> {
    let billed = v.pointer("/usage/billed_units")?;
    let input = billed.get("input_tokens").and_then(|t| t.as_u64()).unwrap_or(0);
    let output = billed.get("output_tokens").and_then(|t| t.as_u64()).unwrap_or(0);

    Some(crate::types::Usage {
        prompt_tokens: input,
        completion_tokens: output,
        total_tokens: input + output,
        prompt_tokens_details: None,
    })
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[test]
    fn test_cohere_name_and_base_url() {
        let provider = CohereProvider;
        assert_eq!(provider.name(), "cohere");
        assert_eq!(provider.base_url(), "https://api.cohere.com/v2");
    }

    #[test]
    fn test_cohere_auth_header() {
        let provider = CohereProvider;
        let (name, value) = provider.auth_header("test-key").expect("should return auth header");
        assert_eq!(name, "Authorization");
        assert_eq!(value, "Bearer test-key");
    }

    #[test]
    fn test_cohere_matches_model() {
        let provider = CohereProvider;
        assert!(provider.matches_model("command-r-plus"));
        assert!(provider.matches_model("command-r"));
        assert!(provider.matches_model("command-light"));
        assert!(provider.matches_model("cohere/command-r-plus"));
        assert!(!provider.matches_model("gpt-4"));
        assert!(!provider.matches_model("claude-3"));
    }

    #[test]
    fn test_cohere_strip_prefix() {
        let provider = CohereProvider;
        assert_eq!(provider.strip_model_prefix("cohere/command-r"), "command-r");
        assert_eq!(provider.strip_model_prefix("command-r"), "command-r");
    }

    #[test]
    fn test_cohere_endpoints() {
        let provider = CohereProvider;
        assert_eq!(provider.chat_completions_path(), "/chat");
        assert_eq!(provider.rerank_path(), "/rerank");
    }

    #[test]
    fn test_cohere_transform_request_preserves_stream_strips_options() {
        let provider = CohereProvider;
        let mut body = json!({
            "model": "command-r-plus",
            "messages": [{"role": "user", "content": "hello"}],
            "stream": true,
            "stream_options": {"include_usage": true}
        });
        provider.transform_request(&mut body).expect("transform should succeed");
        // Cohere v2 needs `stream` in the body — only `stream_options` is removed.
        assert_eq!(body["stream"], true);
        assert!(body.get("stream_options").is_none());
        // Other fields preserved.
        assert_eq!(body["model"], "command-r-plus");
    }

    #[test]
    fn test_cohere_transform_response_finish_reasons() {
        let provider = CohereProvider;
        let mut body = json!({
            "choices": [
                {"finish_reason": "COMPLETE", "message": {"content": "hi"}},
                {"finish_reason": "MAX_TOKENS", "message": {"content": "..."}},
                {"finish_reason": "TOOL_CALL", "message": {"content": ""}}
            ]
        });
        provider
            .transform_response(&mut body)
            .expect("transform should succeed");

        let choices = body["choices"].as_array().expect("choices array");
        assert_eq!(choices[0]["finish_reason"], "stop");
        assert_eq!(choices[1]["finish_reason"], "length");
        assert_eq!(choices[2]["finish_reason"], "tool_calls");
    }

    #[test]
    fn test_cohere_transform_response_usage_normalization() {
        let provider = CohereProvider;
        let mut body = json!({
            "choices": [{"finish_reason": "COMPLETE"}],
            "tokens": {
                "input_tokens": 10,
                "output_tokens": 20
            }
        });
        provider
            .transform_response(&mut body)
            .expect("transform should succeed");

        let usage = &body["usage"];
        assert_eq!(usage["prompt_tokens"], 10);
        assert_eq!(usage["completion_tokens"], 20);
        assert_eq!(usage["total_tokens"], 30);
    }

    #[test]
    fn test_cohere_transform_response_adds_object_and_created() {
        let provider = CohereProvider;
        let mut body = json!({"choices": []});
        provider
            .transform_response(&mut body)
            .expect("transform should succeed");

        assert_eq!(body["object"], "chat.completion");
        assert!(body["created"].as_u64().is_some());
    }

    #[test]
    fn test_cohere_transform_response_preserves_existing_usage() {
        let provider = CohereProvider;
        let mut body = json!({
            "choices": [],
            "usage": {"prompt_tokens": 5, "completion_tokens": 10, "total_tokens": 15},
            "tokens": {"input_tokens": 99, "output_tokens": 99}
        });
        provider
            .transform_response(&mut body)
            .expect("transform should succeed");

        // Existing usage should not be overwritten.
        assert_eq!(body["usage"]["prompt_tokens"], 5);
    }

    // ── Streaming SSE parser tests ───────────────────────────────────────────

    #[test]
    fn test_parse_stream_event_stream_start() {
        let provider = CohereProvider;
        let event = r#"{"type":"stream-start","generation_id":"gen-123"}"#;
        let chunk = provider
            .parse_stream_event(event)
            .expect("should parse")
            .expect("should return Some");

        assert_eq!(chunk.id, "gen-123");
        assert_eq!(chunk.object, "chat.completion.chunk");
        assert_eq!(chunk.choices.len(), 1);
        assert_eq!(chunk.choices[0].delta.role.as_deref(), Some("assistant"));
        assert!(chunk.choices[0].delta.content.is_none());
        assert!(chunk.choices[0].finish_reason.is_none());
        assert!(chunk.usage.is_none());
    }

    #[test]
    fn test_parse_stream_event_content_delta() {
        let provider = CohereProvider;
        let event = r#"{"type":"content-delta","delta":{"type":"text_content","text":"Hello"}}"#;
        let chunk = provider
            .parse_stream_event(event)
            .expect("should parse")
            .expect("should return Some");

        assert_eq!(chunk.choices[0].delta.content.as_deref(), Some("Hello"));
        assert!(chunk.choices[0].delta.role.is_none());
        assert!(chunk.choices[0].delta.tool_calls.is_none());
    }

    #[test]
    fn test_parse_stream_event_content_delta_whitespace() {
        let provider = CohereProvider;
        let event = r#"{"type":"content-delta","delta":{"type":"text_content","text":" world"}}"#;
        let chunk = provider
            .parse_stream_event(event)
            .expect("should parse")
            .expect("should return Some");

        assert_eq!(chunk.choices[0].delta.content.as_deref(), Some(" world"));
    }

    #[test]
    fn test_parse_stream_event_tool_call_start() {
        let provider = CohereProvider;
        let event = r#"{"type":"tool-call-start","index":0,"delta":{"type":"tool_call","id":"tc-001","function":{"name":"get_weather","arguments":""}}}"#;
        let chunk = provider
            .parse_stream_event(event)
            .expect("should parse")
            .expect("should return Some");

        let tool_calls = chunk.choices[0]
            .delta
            .tool_calls
            .as_ref()
            .expect("should have tool_calls");
        assert_eq!(tool_calls.len(), 1);
        assert_eq!(tool_calls[0].index, 0);
        assert_eq!(tool_calls[0].id.as_deref(), Some("tc-001"));
        let func = tool_calls[0].function.as_ref().expect("should have function");
        assert_eq!(func.name.as_deref(), Some("get_weather"));
        assert!(func.arguments.is_none());
    }

    #[test]
    fn test_parse_stream_event_tool_call_delta() {
        let provider = CohereProvider;
        let event =
            r#"{"type":"tool-call-delta","index":0,"delta":{"type":"tool_call","function":{"arguments":"{\"ci"}}}"#;
        let chunk = provider
            .parse_stream_event(event)
            .expect("should parse")
            .expect("should return Some");

        let tool_calls = chunk.choices[0]
            .delta
            .tool_calls
            .as_ref()
            .expect("should have tool_calls");
        assert_eq!(tool_calls.len(), 1);
        assert_eq!(tool_calls[0].index, 0);
        assert!(tool_calls[0].id.is_none());
        let func = tool_calls[0].function.as_ref().expect("should have function");
        assert!(func.name.is_none());
        assert_eq!(func.arguments.as_deref(), Some("{\"ci"));
    }

    #[test]
    fn test_parse_stream_event_tool_call_end_returns_none() {
        let provider = CohereProvider;
        let event = r#"{"type":"tool-call-end","index":0}"#;
        let result = provider.parse_stream_event(event).expect("should parse");

        assert!(result.is_none());
    }

    #[test]
    fn test_parse_stream_event_stream_end_complete() {
        let provider = CohereProvider;
        let event = r#"{"type":"stream-end","finish_reason":"COMPLETE","usage":{"billed_units":{"input_tokens":10,"output_tokens":5}}}"#;
        let chunk = provider
            .parse_stream_event(event)
            .expect("should parse")
            .expect("should return Some");

        assert_eq!(chunk.choices[0].finish_reason, Some(FinishReason::Stop));
        let usage = chunk.usage.as_ref().expect("should have usage");
        assert_eq!(usage.prompt_tokens, 10);
        assert_eq!(usage.completion_tokens, 5);
        assert_eq!(usage.total_tokens, 15);
    }

    #[test]
    fn test_parse_stream_event_stream_end_max_tokens() {
        let provider = CohereProvider;
        let event = r#"{"type":"stream-end","finish_reason":"MAX_TOKENS","usage":{"billed_units":{"input_tokens":20,"output_tokens":100}}}"#;
        let chunk = provider
            .parse_stream_event(event)
            .expect("should parse")
            .expect("should return Some");

        assert_eq!(chunk.choices[0].finish_reason, Some(FinishReason::Length));
        let usage = chunk.usage.as_ref().expect("should have usage");
        assert_eq!(usage.prompt_tokens, 20);
        assert_eq!(usage.completion_tokens, 100);
        assert_eq!(usage.total_tokens, 120);
    }

    #[test]
    fn test_parse_stream_event_stream_end_tool_call() {
        let provider = CohereProvider;
        let event = r#"{"type":"stream-end","finish_reason":"TOOL_CALL","usage":{"billed_units":{"input_tokens":15,"output_tokens":8}}}"#;
        let chunk = provider
            .parse_stream_event(event)
            .expect("should parse")
            .expect("should return Some");

        assert_eq!(chunk.choices[0].finish_reason, Some(FinishReason::ToolCalls));
    }

    #[test]
    fn test_parse_stream_event_stream_end_no_usage() {
        let provider = CohereProvider;
        let event = r#"{"type":"stream-end","finish_reason":"COMPLETE"}"#;
        let chunk = provider
            .parse_stream_event(event)
            .expect("should parse")
            .expect("should return Some");

        assert_eq!(chunk.choices[0].finish_reason, Some(FinishReason::Stop));
        assert!(chunk.usage.is_none());
    }

    #[test]
    fn test_parse_stream_event_unknown_type_returns_none() {
        let provider = CohereProvider;
        let event = r#"{"type":"debug","message":"some debug info"}"#;
        let result = provider.parse_stream_event(event).expect("should parse");

        assert!(result.is_none());
    }

    #[test]
    fn test_parse_stream_event_invalid_json_returns_err() {
        let provider = CohereProvider;
        let result = provider.parse_stream_event("not valid json");

        assert!(result.is_err());
    }

    #[test]
    fn test_parse_stream_event_tool_call_start_index_1() {
        let provider = CohereProvider;
        let event = r#"{"type":"tool-call-start","index":1,"delta":{"type":"tool_call","id":"tc-002","function":{"name":"search","arguments":""}}}"#;
        let chunk = provider
            .parse_stream_event(event)
            .expect("should parse")
            .expect("should return Some");

        let tool_calls = chunk.choices[0]
            .delta
            .tool_calls
            .as_ref()
            .expect("should have tool_calls");
        assert_eq!(tool_calls[0].index, 1);
        assert_eq!(tool_calls[0].id.as_deref(), Some("tc-002"));
    }

    #[test]
    fn test_parse_stream_event_stream_end_unknown_finish_reason() {
        let provider = CohereProvider;
        let event = r#"{"type":"stream-end","finish_reason":"ERROR"}"#;
        let chunk = provider
            .parse_stream_event(event)
            .expect("should parse")
            .expect("should return Some");

        assert_eq!(chunk.choices[0].finish_reason, Some(FinishReason::Other));
    }
}
