use std::borrow::Cow;

use serde_json::Value;

use crate::error::Result;
use crate::provider::Provider;

/// Parameters that Mistral does not support and should be stripped from requests.
const UNSUPPORTED_PARAMS: &[&str] = &[
    "parallel_tool_calls",
    "logit_bias",
    "presence_penalty",
    "frequency_penalty",
];

/// Mistral AI provider.
///
/// Mistral's API is largely OpenAI-compatible with a few differences:
/// - `tool_choice: "required"` must be mapped to `"any"`.
/// - Several OpenAI parameters are not supported and must be stripped.
/// - Response format is OpenAI-compatible (no transform needed).
pub struct MistralProvider;

impl Provider for MistralProvider {
    fn name(&self) -> &str {
        "mistral"
    }

    fn base_url(&self) -> &str {
        "https://api.mistral.ai/v1"
    }

    fn auth_header<'a>(&'a self, api_key: &'a str) -> Option<(Cow<'static, str>, Cow<'a, str>)> {
        Some((Cow::Borrowed("Authorization"), Cow::Owned(format!("Bearer {api_key}"))))
    }

    fn matches_model(&self, model: &str) -> bool {
        model.starts_with("mistral-")
            || model.starts_with("codestral-")
            || model.starts_with("pixtral-")
            || model.starts_with("mistral/")
    }

    fn strip_model_prefix<'m>(&self, model: &'m str) -> &'m str {
        model.strip_prefix("mistral/").unwrap_or(model)
    }

    /// Transform the request body for Mistral compatibility.
    ///
    /// - Maps `tool_choice: "required"` to `"any"` (Mistral's equivalent).
    /// - Strips unsupported parameters: `parallel_tool_calls`, `logit_bias`,
    ///   `presence_penalty`, `frequency_penalty`.
    fn transform_request(&self, body: &mut Value) -> Result<()> {
        if let Some(obj) = body.as_object_mut() {
            // Map tool_choice "required" -> "any".
            if let Some(tc) = obj.get_mut("tool_choice")
                && tc.as_str() == Some("required")
            {
                *tc = Value::String("any".to_owned());
            }

            // Strip unsupported parameters.
            for param in UNSUPPORTED_PARAMS {
                obj.remove(*param);
            }
        }
        Ok(())
    }

    // Response is OpenAI-compatible; no transform_response override needed.
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[test]
    fn test_mistral_name_and_base_url() {
        let provider = MistralProvider;
        assert_eq!(provider.name(), "mistral");
        assert_eq!(provider.base_url(), "https://api.mistral.ai/v1");
    }

    #[test]
    fn test_mistral_auth_header() {
        let provider = MistralProvider;
        let (name, value) = provider.auth_header("test-key").expect("should return auth header");
        assert_eq!(name, "Authorization");
        assert_eq!(value, "Bearer test-key");
    }

    #[test]
    fn test_mistral_matches_model() {
        let provider = MistralProvider;
        assert!(provider.matches_model("mistral-large-latest"));
        assert!(provider.matches_model("mistral-small-latest"));
        assert!(provider.matches_model("codestral-latest"));
        assert!(provider.matches_model("pixtral-large-latest"));
        assert!(provider.matches_model("mistral/mistral-large-latest"));
        assert!(!provider.matches_model("gpt-4"));
        assert!(!provider.matches_model("claude-3"));
        assert!(!provider.matches_model("command-r"));
    }

    #[test]
    fn test_mistral_strip_prefix() {
        let provider = MistralProvider;
        assert_eq!(
            provider.strip_model_prefix("mistral/mistral-large-latest"),
            "mistral-large-latest"
        );
        assert_eq!(
            provider.strip_model_prefix("mistral-large-latest"),
            "mistral-large-latest"
        );
    }

    #[test]
    fn test_mistral_endpoints_are_openai_compatible() {
        let provider = MistralProvider;
        assert_eq!(provider.chat_completions_path(), "/chat/completions");
        assert_eq!(provider.embeddings_path(), "/embeddings");
        assert_eq!(provider.models_path(), "/models");
    }

    #[test]
    fn test_mistral_transform_request_maps_tool_choice() {
        let provider = MistralProvider;
        let mut body = json!({
            "model": "mistral-large-latest",
            "messages": [{"role": "user", "content": "hello"}],
            "tool_choice": "required"
        });
        provider.transform_request(&mut body).expect("transform should succeed");
        assert_eq!(body["tool_choice"], "any");
    }

    #[test]
    fn test_mistral_transform_request_preserves_other_tool_choices() {
        let provider = MistralProvider;

        let mut body = json!({"tool_choice": "auto"});
        provider.transform_request(&mut body).expect("transform should succeed");
        assert_eq!(body["tool_choice"], "auto");

        let mut body = json!({"tool_choice": "none"});
        provider.transform_request(&mut body).expect("transform should succeed");
        assert_eq!(body["tool_choice"], "none");
    }

    #[test]
    fn test_mistral_transform_request_strips_unsupported_params() {
        let provider = MistralProvider;
        let mut body = json!({
            "model": "mistral-large-latest",
            "messages": [{"role": "user", "content": "hello"}],
            "parallel_tool_calls": true,
            "logit_bias": {"123": 1.0},
            "presence_penalty": 0.5,
            "frequency_penalty": 0.5,
            "temperature": 0.7
        });
        provider.transform_request(&mut body).expect("transform should succeed");

        assert!(body.get("parallel_tool_calls").is_none());
        assert!(body.get("logit_bias").is_none());
        assert!(body.get("presence_penalty").is_none());
        assert!(body.get("frequency_penalty").is_none());
        // Supported params preserved.
        assert_eq!(body["temperature"], 0.7);
        assert_eq!(body["model"], "mistral-large-latest");
    }

    #[test]
    fn test_mistral_transform_request_no_tool_choice() {
        let provider = MistralProvider;
        let mut body = json!({
            "model": "mistral-large-latest",
            "messages": [{"role": "user", "content": "hello"}]
        });
        provider.transform_request(&mut body).expect("transform should succeed");
        assert!(body.get("tool_choice").is_none());
    }
}
