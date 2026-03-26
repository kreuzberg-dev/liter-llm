use std::borrow::Cow;

use crate::error::Result;
use crate::provider::Provider;
use crate::types::ChatCompletionChunk;

use super::vertex::{parse_gemini_stream_event, transform_gemini_request, transform_gemini_response};

/// Google AI Studio base URL (Gemini API via API key).
const BASE_URL: &str = "https://generativelanguage.googleapis.com/v1beta";

/// Google AI Studio (Gemini) provider.
///
/// Uses the same Gemini `generateContent` format as Vertex AI but with:
/// - Different base URL: `https://generativelanguage.googleapis.com/v1beta`
/// - API key authentication via `x-goog-api-key` header (not OAuth2).
/// - Model routing via `gemini/` or `google_ai/` prefix.
///
/// Request/response translation is shared with [`super::vertex::VertexAiProvider`]
/// via the `pub(crate)` Gemini transform functions.
///
/// # Configuration
///
/// ```rust,ignore
/// let config = ClientConfigBuilder::new("AIza...your-api-key").build();
/// let client = DefaultClient::new(config, Some("gemini/gemini-2.0-flash"))?;
///
/// // Or using the google_ai/ prefix:
/// let client = DefaultClient::new(config, Some("google_ai/gemini-2.0-flash"))?;
/// ```
pub struct GoogleAiProvider;

impl Provider for GoogleAiProvider {
    fn name(&self) -> &str {
        "google_ai"
    }

    fn base_url(&self) -> &str {
        BASE_URL
    }

    /// Google AI Studio uses `x-goog-api-key` header for authentication.
    fn auth_header<'a>(&'a self, api_key: &'a str) -> Option<(Cow<'static, str>, Cow<'a, str>)> {
        Some((Cow::Borrowed("x-goog-api-key"), Cow::Borrowed(api_key)))
    }

    fn matches_model(&self, model: &str) -> bool {
        model.starts_with("gemini/") || model.starts_with("google_ai/")
    }

    fn strip_model_prefix<'m>(&self, model: &'m str) -> &'m str {
        model
            .strip_prefix("gemini/")
            .or_else(|| model.strip_prefix("google_ai/"))
            .unwrap_or(model)
    }

    /// Build the full URL for a Google AI Studio request.
    ///
    /// Chat completions → `{base}/models/{model}:generateContent`
    /// Embeddings       → `{base}/models/{model}:embedContent`
    /// Other paths      → `{base}{endpoint_path}`
    fn build_url(&self, endpoint_path: &str, model: &str) -> String {
        let base = self.base_url().trim_end_matches('/');
        if endpoint_path.contains("chat/completions") {
            format!("{base}/models/{model}:generateContent")
        } else if endpoint_path.contains("embeddings") {
            format!("{base}/models/{model}:embedContent")
        } else {
            format!("{base}{endpoint_path}")
        }
    }

    fn transform_request(&self, body: &mut serde_json::Value) -> Result<()> {
        transform_gemini_request(body)
    }

    fn transform_response(&self, body: &mut serde_json::Value) -> Result<()> {
        transform_gemini_response(body)
    }

    /// Build the streaming URL: appends `?alt=sse` to enable SSE streaming.
    fn build_stream_url(&self, endpoint_path: &str, model: &str) -> String {
        let url = self.build_url(endpoint_path, model);
        format!("{url}?alt=sse")
    }

    fn parse_stream_event(&self, event_data: &str) -> Result<Option<ChatCompletionChunk>> {
        parse_gemini_stream_event(event_data)
    }
}

// ── Unit tests ────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;
    use crate::provider::Provider;

    fn provider() -> GoogleAiProvider {
        GoogleAiProvider
    }

    // ── base_url ──────────────────────────────────────────────────────────────

    #[test]
    fn base_url_is_generative_language_api() {
        let p = provider();
        assert_eq!(p.base_url(), "https://generativelanguage.googleapis.com/v1beta");
    }

    // ── auth_header ──────────────────────────────────────────────────────────

    #[test]
    fn auth_header_uses_x_goog_api_key() {
        let p = provider();
        let (name, value) = p.auth_header("test-key").unwrap();
        assert_eq!(name.as_ref(), "x-goog-api-key");
        assert_eq!(value.as_ref(), "test-key");
    }

    // ── model matching ───────────────────────────────────────────────────────

    #[test]
    fn matches_gemini_prefix() {
        let p = provider();
        assert!(p.matches_model("gemini/gemini-2.0-flash"));
        assert!(p.matches_model("google_ai/gemini-2.0-flash"));
        assert!(!p.matches_model("vertex_ai/gemini-2.0-flash"));
        assert!(!p.matches_model("gpt-4"));
    }

    #[test]
    fn strip_model_prefix_gemini() {
        let p = provider();
        assert_eq!(p.strip_model_prefix("gemini/gemini-2.0-flash"), "gemini-2.0-flash");
        assert_eq!(p.strip_model_prefix("google_ai/gemini-pro"), "gemini-pro");
        assert_eq!(p.strip_model_prefix("gemini-2.0-flash"), "gemini-2.0-flash");
    }

    // ── build_url ─────────────────────────────────────────────────────────────

    #[test]
    fn build_url_chat_completions() {
        let p = provider();
        let url = p.build_url("/chat/completions", "gemini-2.0-flash");
        assert_eq!(
            url,
            "https://generativelanguage.googleapis.com/v1beta/models/gemini-2.0-flash:generateContent"
        );
    }

    #[test]
    fn build_url_embeddings() {
        let p = provider();
        let url = p.build_url("/embeddings", "text-embedding-004");
        assert_eq!(
            url,
            "https://generativelanguage.googleapis.com/v1beta/models/text-embedding-004:embedContent"
        );
    }

    #[test]
    fn build_stream_url_appends_alt_sse() {
        let p = provider();
        let url = p.build_stream_url("/chat/completions", "gemini-2.0-flash");
        assert!(url.ends_with("?alt=sse"));
        assert!(url.contains(":generateContent"));
    }

    // ── transform_request (delegates to shared Gemini logic) ─────────────────

    #[test]
    fn transform_request_basic_chat() {
        let p = provider();
        let mut body = json!({
            "messages": [
                {"role": "system", "content": "You are helpful."},
                {"role": "user", "content": "Hello!"}
            ],
            "max_tokens": 100
        });

        p.transform_request(&mut body).unwrap();

        assert_eq!(body["systemInstruction"]["parts"][0]["text"], "You are helpful.");
        assert_eq!(body["contents"][0]["role"], "user");
        assert_eq!(body["contents"][0]["parts"][0]["text"], "Hello!");
        assert_eq!(body["generationConfig"]["maxOutputTokens"], 100);
    }

    #[test]
    fn transform_request_safety_settings() {
        let p = provider();
        let mut body = json!({
            "messages": [{"role": "user", "content": "hi"}],
            "extra_body": {
                "safety_settings": [
                    {"category": "HARM_CATEGORY_HATE_SPEECH", "threshold": "BLOCK_MEDIUM_AND_ABOVE"}
                ]
            }
        });

        p.transform_request(&mut body).unwrap();

        let settings = body["safetySettings"].as_array().unwrap();
        assert_eq!(settings.len(), 1);
        assert_eq!(settings[0]["category"], "HARM_CATEGORY_HATE_SPEECH");
    }

    #[test]
    fn transform_request_cached_content() {
        let p = provider();
        let mut body = json!({
            "messages": [{"role": "user", "content": "hi"}],
            "extra_body": {
                "cached_content": "cachedContents/abc123"
            }
        });

        p.transform_request(&mut body).unwrap();

        assert_eq!(body["cachedContent"], "cachedContents/abc123");
    }

    // ── transform_response (delegates to shared Gemini logic) ────────────────

    #[test]
    fn transform_response_basic() {
        let p = provider();
        let mut body = json!({
            "candidates": [{
                "content": {
                    "role": "model",
                    "parts": [{"text": "Hello from Google AI!"}]
                },
                "finishReason": "STOP"
            }],
            "usageMetadata": {
                "promptTokenCount": 5,
                "candidatesTokenCount": 4
            }
        });

        p.transform_response(&mut body).unwrap();

        assert_eq!(body["object"], "chat.completion");
        assert_eq!(body["choices"][0]["message"]["content"], "Hello from Google AI!");
        assert_eq!(body["choices"][0]["finish_reason"], "stop");
        assert_eq!(body["usage"]["prompt_tokens"], 5);
        assert_eq!(body["usage"]["completion_tokens"], 4);
    }

    // ── parse_stream_event (delegates to shared Gemini logic) ────────────────

    #[test]
    fn parse_stream_event_empty_returns_none() {
        let p = provider();
        let result = p.parse_stream_event("").unwrap();
        assert!(result.is_none());
    }

    #[test]
    fn parse_stream_event_basic_chunk() {
        let p = provider();
        let event_data = r#"{
            "candidates": [{
                "content": {"role": "model", "parts": [{"text": "Hi"}]},
                "finishReason": "STOP"
            }],
            "usageMetadata": {"promptTokenCount": 3, "candidatesTokenCount": 1}
        }"#;

        let chunk = p.parse_stream_event(event_data).unwrap().unwrap();

        assert_eq!(chunk.object, "chat.completion.chunk");
        assert_eq!(chunk.choices[0].delta.content.as_deref(), Some("Hi"));
    }
}
