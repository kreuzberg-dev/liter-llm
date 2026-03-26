use std::borrow::Cow;

use serde_json::Value;

use crate::error::Result;
use crate::provider::{Provider, unix_timestamp_secs};

/// Cohere provider (Command model family).
///
/// Differences from the OpenAI-compatible baseline:
/// - Chat endpoint is `/chat` instead of `/chat/completions`.
/// - Rerank endpoint is `/rerank` instead of the default path.
/// - `stream` and `stream_options` are transport-level and must be stripped.
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
    fn transform_request(&self, body: &mut Value) -> Result<()> {
        if let Some(obj) = body.as_object_mut() {
            obj.remove("stream");
            obj.remove("stream_options");
        }
        Ok(())
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
    fn test_cohere_transform_request_strips_stream() {
        let provider = CohereProvider;
        let mut body = json!({
            "model": "command-r-plus",
            "messages": [{"role": "user", "content": "hello"}],
            "stream": true,
            "stream_options": {"include_usage": true}
        });
        provider.transform_request(&mut body).expect("transform should succeed");
        assert!(body.get("stream").is_none());
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
}
