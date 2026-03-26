use std::borrow::Cow;

use crate::error::{LiterLmError, Result};
use crate::provider::Provider;
use crate::types::ChatCompletionChunk;

/// Google Vertex AI / Gemini provider.
///
/// Differences from the OpenAI-compatible baseline:
/// - Auth uses `Authorization: Bearer <token>` where the token is a Google
///   Cloud OAuth2 access token (obtained via ADC, service account, or
///   `gcloud auth print-access-token`).
/// - The base URL is **required** and must be set via `base_url` in
///   [`ClientConfig`]. It should be the Vertex AI Gemini endpoint, e.g.:
///   `https://us-central1-aiplatform.googleapis.com/v1/projects/{project}/locations/{region}`
/// - Model names are routed via the `vertex_ai/` prefix which is stripped
///   before being sent in the request body.
/// - The native Gemini `generateContent` format is used, not the OpenAI
///   `/chat/completions` path. Request and response are translated accordingly.
/// - Streaming uses SSE with `?alt=sse`; each chunk is a full `generateContent`
///   response JSON wrapped in a standard SSE `data:` line.
///
/// # Token management
///
/// Supply a pre-obtained access token as the `api_key` parameter.
/// Token refresh is the caller's responsibility.  A future release will add
/// ADC / service-account-based automatic refresh.
///
/// # Configuration
///
/// ```rust,ignore
/// let config = ClientConfigBuilder::new("ya29.your-access-token")
///     .base_url(
///         "https://us-central1-aiplatform.googleapis.com/v1/\
///          projects/my-project/locations/us-central1",
///     )
///     .build();
/// let client = DefaultClient::new(config, Some("vertex_ai/gemini-2.0-flash"))?;
/// ```
pub struct VertexAiProvider;

impl Provider for VertexAiProvider {
    fn name(&self) -> &str {
        "vertex_ai"
    }

    /// Vertex AI base URL is always customer- and region-specific.
    ///
    /// Returns an empty string when no `base_url` override is present in
    /// [`ClientConfig`].  The caller is expected to always supply a `base_url`.
    fn base_url(&self) -> &str {
        ""
    }

    fn auth_header<'a>(&'a self, api_key: &'a str) -> Option<(Cow<'static, str>, Cow<'a, str>)> {
        // Vertex AI requires an OAuth2 Bearer token.
        Some((Cow::Borrowed("Authorization"), Cow::Owned(format!("Bearer {api_key}"))))
    }

    fn matches_model(&self, model: &str) -> bool {
        model.starts_with("vertex_ai/")
    }

    fn strip_model_prefix<'m>(&self, model: &'m str) -> &'m str {
        model.strip_prefix("vertex_ai/").unwrap_or(model)
    }

    /// Build the full URL for a Gemini API request.
    ///
    /// Chat completions → `{base}/models/{model}:generateContent`
    /// Embeddings       → `{base}/models/{model}:predict`
    /// Other paths      → `{base}{endpoint_path}`
    fn build_url(&self, endpoint_path: &str, model: &str) -> String {
        let base = self.base_url();
        if base.is_empty() {
            // Caller must supply a base_url; will fail at validate() / HTTP layer.
            return String::new();
        }
        let base = base.trim_end_matches('/');
        if endpoint_path.contains("chat/completions") {
            format!("{base}/models/{model}:generateContent")
        } else if endpoint_path.contains("embeddings") {
            format!("{base}/models/{model}:predict")
        } else {
            format!("{base}{endpoint_path}")
        }
    }

    /// Convert an OpenAI-style chat request to Gemini `generateContent` format.
    ///
    /// Key differences:
    /// - System messages are extracted to `system_instruction.parts[]`.
    /// - Assistant role becomes `model`.
    /// - Tool calls map to `functionCall` parts; tool results map to `functionResponse` parts.
    /// - Generation parameters live in `generationConfig`.
    fn transform_request(&self, body: &mut serde_json::Value) -> Result<()> {
        use serde_json::json;

        let messages = body
            .get("messages")
            .and_then(|v| v.as_array())
            .cloned()
            .unwrap_or_default();

        let mut system_parts: Vec<serde_json::Value> = vec![];
        let mut contents: Vec<serde_json::Value> = vec![];

        for msg in &messages {
            let role = msg.get("role").and_then(|r| r.as_str()).unwrap_or("");
            let content = msg.get("content");

            match role {
                "system" | "developer" => {
                    if let Some(text) = content.and_then(|c| c.as_str()) {
                        system_parts.push(json!({"text": text}));
                    }
                }
                "user" => {
                    let parts = if let Some(text) = content.and_then(|c| c.as_str()) {
                        vec![json!({"text": text})]
                    } else {
                        // Fallback for non-string content.
                        vec![json!({"text": ""})]
                    };
                    contents.push(json!({"role": "user", "parts": parts}));
                }
                "assistant" => {
                    let mut parts: Vec<serde_json::Value> = vec![];
                    if let Some(text) = content.and_then(|c| c.as_str())
                        && !text.is_empty()
                    {
                        parts.push(json!({"text": text}));
                    }
                    // Convert OpenAI tool_calls to Gemini functionCall parts.
                    if let Some(tool_calls) = msg.get("tool_calls").and_then(|t| t.as_array()) {
                        for tc in tool_calls {
                            let args: serde_json::Value = tc
                                .pointer("/function/arguments")
                                .and_then(|a| a.as_str())
                                .and_then(|s| serde_json::from_str(s).ok())
                                .unwrap_or_else(|| json!({}));
                            parts.push(json!({
                                "functionCall": {
                                    "name": tc.pointer("/function/name"),
                                    "args": args
                                }
                            }));
                        }
                    }
                    if parts.is_empty() {
                        parts.push(json!({"text": ""}));
                    }
                    // Gemini uses "model" role for assistant turns.
                    contents.push(json!({"role": "model", "parts": parts}));
                }
                "tool" => {
                    // Map tool result back to a user turn with a functionResponse part.
                    let name = msg
                        .get("name")
                        .or_else(|| msg.get("tool_call_id"))
                        .and_then(|n| n.as_str())
                        .unwrap_or("tool");
                    let result_content = content.cloned().unwrap_or(json!(null));
                    contents.push(json!({
                        "role": "user",
                        "parts": [{
                            "functionResponse": {
                                "name": name,
                                "response": {"result": result_content}
                            }
                        }]
                    }));
                }
                _ => {}
            }
        }

        // Build generationConfig from OpenAI parameters.
        let mut gen_config = json!({});
        if let Some(max_tokens) = body.get("max_tokens") {
            gen_config["maxOutputTokens"] = max_tokens.clone();
        }
        if let Some(temp) = body.get("temperature") {
            gen_config["temperature"] = temp.clone();
        }
        if let Some(top_p) = body.get("top_p") {
            gen_config["topP"] = top_p.clone();
        }
        if let Some(stop) = body.get("stop") {
            let sequences = if let Some(s) = stop.as_str() {
                vec![json!(s)]
            } else {
                stop.as_array().cloned().unwrap_or_default()
            };
            gen_config["stopSequences"] = json!(sequences);
        }

        let mut new_body = json!({"contents": contents});
        if !system_parts.is_empty() {
            new_body["system_instruction"] = json!({"parts": system_parts});
        }
        if let Some(obj) = gen_config.as_object()
            && !obj.is_empty()
        {
            new_body["generationConfig"] = gen_config;
        }

        *body = new_body;
        Ok(())
    }

    /// Normalize a Gemini `generateContent` response to OpenAI chat completion format.
    ///
    /// Gemini wraps the response in `candidates[0].content.parts[]`.
    /// Finish reasons use Gemini terminology (`STOP`, `MAX_TOKENS`, `SAFETY`, …)
    /// and are mapped to the OpenAI `finish_reason` set.
    fn transform_response(&self, body: &mut serde_json::Value) -> Result<()> {
        use serde_json::json;

        let candidate = body.pointer("/candidates/0").cloned();
        let finish_reason_raw = candidate
            .as_ref()
            .and_then(|c| c.get("finishReason"))
            .and_then(|f| f.as_str())
            .unwrap_or("STOP");
        let parts = candidate
            .as_ref()
            .and_then(|c| c.pointer("/content/parts"))
            .and_then(|p| p.as_array())
            .cloned()
            .unwrap_or_default();

        // Collect text content from parts.
        let text: String = parts
            .iter()
            .filter_map(|p| p.get("text").and_then(|t| t.as_str()))
            .collect::<Vec<_>>()
            .join("");

        // Collect functionCall parts and convert to OpenAI tool_calls.
        let tool_calls: Vec<serde_json::Value> = parts
            .iter()
            .filter_map(|p| {
                p.get("functionCall").map(|fc| {
                    let name = fc.get("name").and_then(|n| n.as_str()).unwrap_or("unknown");
                    let arguments = serde_json::to_string(fc.get("args").unwrap_or(&json!({}))).unwrap_or_default();
                    json!({
                        "id": format!("call_{name}"),
                        "type": "function",
                        "function": {
                            "name": fc.get("name"),
                            "arguments": arguments
                        }
                    })
                })
            })
            .collect();

        let finish_reason = match finish_reason_raw {
            "STOP" => "stop",
            "MAX_TOKENS" => "length",
            "SAFETY" | "RECITATION" | "BLOCKLIST" | "PROHIBITED_CONTENT" => "content_filter",
            "TOOL_CODE" | "FUNCTION_CALL" => "tool_calls",
            _ => "stop",
        };

        let prompt_tokens = body
            .pointer("/usageMetadata/promptTokenCount")
            .and_then(|v| v.as_u64())
            .unwrap_or(0);
        let completion_tokens = body
            .pointer("/usageMetadata/candidatesTokenCount")
            .and_then(|v| v.as_u64())
            .unwrap_or(0);

        let response_id = body.get("responseId").cloned().unwrap_or_else(|| json!("gemini-resp"));

        let content_value: serde_json::Value = if text.is_empty() { json!(null) } else { json!(text) };

        let mut message = json!({"role": "assistant", "content": content_value});
        if !tool_calls.is_empty() {
            message["tool_calls"] = json!(tool_calls);
        }

        *body = json!({
            "id": response_id,
            "object": "chat.completion",
            "created": 0u64,
            "model": "",
            "choices": [{
                "index": 0,
                "message": message,
                "finish_reason": finish_reason
            }],
            "usage": {
                "prompt_tokens": prompt_tokens,
                "completion_tokens": completion_tokens,
                "total_tokens": prompt_tokens + completion_tokens
            }
        });

        Ok(())
    }

    /// Parse a single SSE event from Gemini's streaming endpoint.
    ///
    /// Gemini streaming uses SSE with `?alt=sse`.  Each event data is a complete
    /// `generateContent` JSON response.  We reuse `transform_response` to
    /// normalize it into OpenAI format, then build a `ChatCompletionChunk` from
    /// the first choice's message content.
    fn parse_stream_event(&self, event_data: &str) -> Result<Option<ChatCompletionChunk>> {
        if event_data.trim().is_empty() || event_data == "[DONE]" {
            return Ok(None);
        }

        let mut body: serde_json::Value = serde_json::from_str(event_data).map_err(|e| LiterLmError::Streaming {
            message: format!("failed to parse Gemini SSE data: {e}"),
        })?;

        // Normalize to OpenAI chat completion format.
        self.transform_response(&mut body)?;

        // Extract fields from the normalized response.
        let id = body
            .get("id")
            .and_then(|v| v.as_str())
            .unwrap_or("gemini-resp")
            .to_owned();
        let model = body.get("model").and_then(|v| v.as_str()).unwrap_or("").to_owned();

        let choice = body.pointer("/choices/0");
        let content = choice
            .and_then(|c| c.pointer("/message/content"))
            .and_then(|v| v.as_str())
            .map(ToOwned::to_owned);
        let finish_reason_str = choice
            .and_then(|c| c.get("finish_reason"))
            .and_then(|v| v.as_str())
            .unwrap_or("");

        use crate::types::{FinishReason, StreamChoice, StreamDelta};

        let finish_reason = match finish_reason_str {
            "stop" => Some(FinishReason::Stop),
            "length" => Some(FinishReason::Length),
            "tool_calls" => Some(FinishReason::ToolCalls),
            "content_filter" => Some(FinishReason::ContentFilter),
            _ => None,
        };

        let chunk = ChatCompletionChunk {
            id,
            object: "chat.completion.chunk".to_owned(),
            created: 0,
            model,
            choices: vec![StreamChoice {
                index: 0,
                delta: StreamDelta {
                    role: Some("assistant".to_owned()),
                    content,
                    tool_calls: None,
                    function_call: None,
                    refusal: None,
                },
                finish_reason,
            }],
            usage: None,
            system_fingerprint: None,
            service_tier: None,
        };

        Ok(Some(chunk))
    }
}

// ── Unit tests ────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;
    use crate::provider::Provider;

    fn provider() -> VertexAiProvider {
        VertexAiProvider
    }

    // ── build_url ─────────────────────────────────────────────────────────────

    #[test]
    fn build_url_returns_empty_without_base() {
        // VertexAiProvider.base_url() returns "" when no override is set.
        // The build_url implementation must propagate that gracefully.
        let p = provider();
        let url = p.build_url("/chat/completions", "gemini-2.0-flash");
        assert!(url.is_empty(), "should return empty string without a base URL");
    }

    // ── transform_request ─────────────────────────────────────────────────────

    #[test]
    fn transform_request_basic_chat() {
        let p = provider();
        let mut body = json!({
            "messages": [
                {"role": "system", "content": "You are a helpful assistant."},
                {"role": "user", "content": "Hello!"}
            ],
            "max_tokens": 200,
            "temperature": 0.5
        });

        p.transform_request(&mut body).unwrap();

        // System instruction extracted.
        assert_eq!(
            body["system_instruction"]["parts"][0]["text"],
            "You are a helpful assistant."
        );

        // User message converted to Gemini format.
        assert_eq!(body["contents"][0]["role"], "user");
        assert_eq!(body["contents"][0]["parts"][0]["text"], "Hello!");

        // Generation config set.
        assert_eq!(body["generationConfig"]["maxOutputTokens"], 200);
        assert_eq!(body["generationConfig"]["temperature"], 0.5);
    }

    #[test]
    fn transform_request_assistant_becomes_model_role() {
        let p = provider();
        let mut body = json!({
            "messages": [
                {"role": "user", "content": "Hi"},
                {"role": "assistant", "content": "Hello there!"}
            ]
        });

        p.transform_request(&mut body).unwrap();

        assert_eq!(body["contents"][1]["role"], "model");
        assert_eq!(body["contents"][1]["parts"][0]["text"], "Hello there!");
    }

    #[test]
    fn transform_request_with_tool_calls() {
        let p = provider();
        let mut body = json!({
            "messages": [
                {"role": "user", "content": "What is the weather in Berlin?"},
                {
                    "role": "assistant",
                    "content": null,
                    "tool_calls": [{
                        "id": "call_1",
                        "type": "function",
                        "function": {"name": "get_weather", "arguments": "{\"city\":\"Berlin\"}"}
                    }]
                },
                {
                    "role": "tool",
                    "name": "get_weather",
                    "tool_call_id": "call_1",
                    "content": "Sunny, 22°C"
                }
            ]
        });

        p.transform_request(&mut body).unwrap();

        let contents = body["contents"].as_array().unwrap();
        assert_eq!(contents.len(), 3);

        // Assistant turn with functionCall part.
        let model_turn = &contents[1];
        assert_eq!(model_turn["role"], "model");
        let fn_call = &model_turn["parts"][0]["functionCall"];
        assert_eq!(fn_call["name"], "get_weather");
        assert_eq!(fn_call["args"]["city"], "Berlin");

        // Tool result as user turn with functionResponse.
        let tool_turn = &contents[2];
        assert_eq!(tool_turn["role"], "user");
        let fn_resp = &tool_turn["parts"][0]["functionResponse"];
        assert_eq!(fn_resp["name"], "get_weather");
    }

    #[test]
    fn transform_request_stop_sequences() {
        let p = provider();
        let mut body = json!({
            "messages": [{"role": "user", "content": "hi"}],
            "stop": ["END", "STOP"]
        });

        p.transform_request(&mut body).unwrap();

        let stop_seqs = body["generationConfig"]["stopSequences"].as_array().unwrap();
        assert_eq!(stop_seqs.len(), 2);
        assert_eq!(stop_seqs[0], "END");
        assert_eq!(stop_seqs[1], "STOP");
    }

    // ── transform_response ────────────────────────────────────────────────────

    #[test]
    fn transform_response_basic() {
        let p = provider();
        let mut body = json!({
            "responseId": "resp-gemini-123",
            "candidates": [{
                "content": {
                    "role": "model",
                    "parts": [{"text": "Hello from Gemini!"}]
                },
                "finishReason": "STOP"
            }],
            "usageMetadata": {
                "promptTokenCount": 8,
                "candidatesTokenCount": 6
            }
        });

        p.transform_response(&mut body).unwrap();

        assert_eq!(body["object"], "chat.completion");
        assert_eq!(body["id"], "resp-gemini-123");
        assert_eq!(body["choices"][0]["message"]["content"], "Hello from Gemini!");
        assert_eq!(body["choices"][0]["finish_reason"], "stop");
        assert_eq!(body["usage"]["prompt_tokens"], 8);
        assert_eq!(body["usage"]["completion_tokens"], 6);
        assert_eq!(body["usage"]["total_tokens"], 14);
    }

    #[test]
    fn transform_response_tool_calls() {
        let p = provider();
        let mut body = json!({
            "candidates": [{
                "content": {
                    "role": "model",
                    "parts": [{
                        "functionCall": {
                            "name": "get_weather",
                            "args": {"city": "Berlin"}
                        }
                    }]
                },
                "finishReason": "STOP"
            }],
            "usageMetadata": {"promptTokenCount": 10, "candidatesTokenCount": 5}
        });

        p.transform_response(&mut body).unwrap();

        let tool_calls = body["choices"][0]["message"]["tool_calls"].as_array().unwrap();
        assert_eq!(tool_calls.len(), 1);
        assert_eq!(tool_calls[0]["id"], "call_get_weather");
        assert_eq!(tool_calls[0]["function"]["name"], "get_weather");
        let args: serde_json::Value =
            serde_json::from_str(tool_calls[0]["function"]["arguments"].as_str().unwrap()).unwrap();
        assert_eq!(args["city"], "Berlin");
    }

    #[test]
    fn transform_response_finish_reason_mapping() {
        let p = provider();

        for (gemini_reason, expected_oai_reason) in [
            ("STOP", "stop"),
            ("MAX_TOKENS", "length"),
            ("SAFETY", "content_filter"),
            ("RECITATION", "content_filter"),
            ("BLOCKLIST", "content_filter"),
            ("PROHIBITED_CONTENT", "content_filter"),
            ("UNKNOWN_FUTURE_REASON", "stop"),
        ] {
            let mut body = json!({
                "candidates": [{
                    "content": {"role": "model", "parts": [{"text": ""}]},
                    "finishReason": gemini_reason
                }],
                "usageMetadata": {"promptTokenCount": 0, "candidatesTokenCount": 0}
            });
            p.transform_response(&mut body).unwrap();
            assert_eq!(
                body["choices"][0]["finish_reason"], expected_oai_reason,
                "Gemini finishReason '{gemini_reason}' should map to '{expected_oai_reason}'"
            );
        }
    }

    // ── parse_stream_event ────────────────────────────────────────────────────

    #[test]
    fn parse_stream_event_empty_returns_none() {
        let p = provider();
        let result = p.parse_stream_event("").unwrap();
        assert!(result.is_none());
    }

    #[test]
    fn parse_stream_event_done_returns_none() {
        let p = provider();
        let result = p.parse_stream_event("[DONE]").unwrap();
        assert!(result.is_none());
    }

    #[test]
    fn parse_stream_event_basic_chunk() {
        let p = provider();
        let event_data = r#"{
            "candidates": [{
                "content": {"role": "model", "parts": [{"text": "Hello"}]},
                "finishReason": "STOP"
            }],
            "usageMetadata": {"promptTokenCount": 5, "candidatesTokenCount": 2}
        }"#;

        let chunk = p.parse_stream_event(event_data).unwrap().unwrap();

        assert_eq!(chunk.object, "chat.completion.chunk");
        assert_eq!(chunk.choices[0].delta.content.as_deref(), Some("Hello"));
    }

    // ── model prefix / matching ───────────────────────────────────────────────

    #[test]
    fn strip_model_prefix() {
        let p = provider();
        assert_eq!(p.strip_model_prefix("vertex_ai/gemini-2.0-flash"), "gemini-2.0-flash");
        assert_eq!(p.strip_model_prefix("gemini-2.0-flash"), "gemini-2.0-flash");
    }

    #[test]
    fn matches_model() {
        let p = provider();
        assert!(p.matches_model("vertex_ai/gemini-2.0-flash"));
        assert!(!p.matches_model("gemini-2.0-flash"));
        assert!(!p.matches_model("gpt-4"));
    }
}
