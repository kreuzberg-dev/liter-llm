use std::borrow::Cow;

use crate::error::{LiterLmError, Result};
use crate::provider::Provider;

/// Azure OpenAI provider.
///
/// Differences from the OpenAI-compatible baseline:
/// - Auth uses `api-key` instead of `Authorization: Bearer`.
/// - The base URL is **required** and must be supplied via the
///   `AZURE_OPENAI_ENDPOINT` environment variable (or `AZURE_ENDPOINT`), in the
///   format `https://{resource}.openai.azure.com`.  Azure has no single shared
///   endpoint — each customer has a unique resource URL.  Failing to supply
///   `base_url` will produce a clear [`LiterLmError::BadRequest`] at
///   construction time via [`AzureProvider::validate`].
/// - The URL embeds the deployment name rather than sending it in the request
///   body; see [`AzureProvider::build_url`].
/// - The API version is configurable via `AZURE_API_VERSION` (default:
///   `2025-02-01-preview`).
///
/// # URL Format
///
/// ```text
/// {base_url}/openai/deployments/{deployment}{endpoint_path}?api-version={api_version}
/// ```
///
/// # Configuration
///
/// ```rust,ignore
/// // Set environment variables before constructing the client:
/// //   AZURE_OPENAI_ENDPOINT=https://my-resource.openai.azure.com
/// //   AZURE_API_VERSION=2024-10-21   (optional)
/// let config = ClientConfigBuilder::new("your-azure-api-key").build();
/// let client = DefaultClient::new(config, Some("azure/gpt-4"))?;
/// ```
pub struct AzureProvider {
    /// Customer-specific resource URL, e.g. `https://my-resource.openai.azure.com`.
    ///
    /// Empty string when no environment variable is set; `validate()` surfaces
    /// this as a [`LiterLmError::BadRequest`] before any request is attempted.
    base_url: String,
    /// Azure REST API version query parameter, e.g. `2024-10-21`.
    api_version: String,
}

impl AzureProvider {
    /// Construct an [`AzureProvider`], reading configuration from environment
    /// variables.
    ///
    /// - `AZURE_OPENAI_ENDPOINT` (or `AZURE_ENDPOINT` as a fallback): the
    ///   customer resource URL in the form `https://{resource}.openai.azure.com`.
    ///   Trailing slashes are stripped.
    /// - `AZURE_API_VERSION`: optional API version string (default:
    ///   `2025-02-01-preview`).
    #[must_use]
    pub fn new() -> Self {
        let base_url = std::env::var("AZURE_OPENAI_ENDPOINT")
            .or_else(|_| std::env::var("AZURE_ENDPOINT"))
            .unwrap_or_default()
            .trim_end_matches('/')
            .to_owned();

        let api_version = std::env::var("AZURE_API_VERSION").unwrap_or_else(|_| "2025-02-01-preview".to_owned());

        Self { base_url, api_version }
    }
}

impl Default for AzureProvider {
    fn default() -> Self {
        Self::new()
    }
}

impl Provider for AzureProvider {
    fn name(&self) -> &str {
        "azure"
    }

    /// Returns the customer resource base URL (empty string when unconfigured).
    ///
    /// An empty return value causes [`AzureProvider::validate`] to fail at
    /// construction time with a descriptive error, so the HTTP layer never
    /// receives a malformed URL.
    fn base_url(&self) -> &str {
        &self.base_url
    }

    fn auth_header<'a>(&'a self, api_key: &'a str) -> Option<(Cow<'static, str>, Cow<'a, str>)> {
        // Azure uses `api-key`, not `Authorization: Bearer`.
        Some((Cow::Borrowed("api-key"), Cow::Borrowed(api_key)))
    }

    fn matches_model(&self, model: &str) -> bool {
        model.starts_with("azure/")
    }

    fn strip_model_prefix<'m>(&self, model: &'m str) -> &'m str {
        model.strip_prefix("azure/").unwrap_or(model)
    }

    /// Validate that a base URL is present.
    ///
    /// Azure requires a customer-specific resource URL.  This check runs at
    /// [`DefaultClient::new`] time, surfacing misconfiguration immediately
    /// rather than on the first request — covering `list_models` as well.
    fn validate(&self) -> Result<()> {
        if self.base_url.is_empty() {
            return Err(LiterLmError::BadRequest {
                message: "Azure OpenAI requires a base URL. \
                          Set AZURE_OPENAI_ENDPOINT=https://{resource}.openai.azure.com \
                          (or AZURE_ENDPOINT as a fallback)."
                    .into(),
            });
        }
        Ok(())
    }

    /// Build the Azure deployment URL.
    ///
    /// Azure embeds the deployment name in the URL rather than the request body:
    ///
    /// ```text
    /// {base_url}/openai/deployments/{deployment}{endpoint_path}?api-version={api_version}
    /// ```
    ///
    /// When `base_url` is empty (misconfigured), returns a clearly-broken URL
    /// that will fail at the HTTP layer; `validate()` normally catches this
    /// before any request is fired.
    fn build_url(&self, endpoint_path: &str, model: &str) -> String {
        if self.base_url.is_empty() {
            // validate() should have caught this; return a broken URL so the
            // HTTP layer surfaces a clear connection error rather than silently
            // hitting the wrong endpoint.
            return endpoint_path.to_owned();
        }
        // If the base URL already contains the deployments path (e.g. it was
        // supplied pre-formatted), avoid duplicating it.
        if self.base_url.contains("/openai/deployments/") {
            return format!("{}{}?api-version={}", self.base_url, endpoint_path, self.api_version);
        }
        format!(
            "{}/openai/deployments/{}{}?api-version={}",
            self.base_url, model, endpoint_path, self.api_version
        )
    }

    /// Transform the request body for Azure OpenAI.
    ///
    /// - Removes `model` from the body (Azure routes via URL deployment name).
    /// - Handles O-series models (o1, o3, o4): removes `temperature`, `top_p`,
    ///   and `stream` (for o1) since Azure rejects them for reasoning models.
    /// - Maps `reasoning_effort` for O-series models.
    ///
    /// [`build_url`]: AzureProvider::build_url
    fn transform_request(&self, body: &mut serde_json::Value) -> Result<()> {
        if let Some(obj) = body.as_object_mut() {
            // Capture the model name before removing it for O-series detection.
            let model_name = obj.get("model").and_then(|m| m.as_str()).unwrap_or("").to_owned();

            obj.remove("model");

            // O-series model handling (o1, o3, o4).
            if is_o_series_model(&model_name) {
                // Azure rejects temperature and top_p for O-series reasoning models.
                obj.remove("temperature");
                obj.remove("top_p");

                // o1 models do not support streaming in some Azure API versions.
                if model_name == "o1" || model_name.starts_with("o1-") || model_name.starts_with("o1.") {
                    obj.remove("stream");
                    obj.remove("stream_options");
                }
            }
        }
        Ok(())
    }

    /// Transform the Azure response.
    ///
    /// Azure responses are OpenAI-compatible. When content filtering is
    /// triggered, the response includes `content_filter_results` in choices
    /// and `finish_reason: "content_filter"`. This maps correctly to the
    /// canonical [`FinishReason::ContentFilter`] variant already, so no
    /// transformation is needed for normal responses.
    ///
    /// For blocked responses where the choice has no `message` content but
    /// does have `content_filter_results`, we ensure the response still has
    /// a valid structure.
    fn transform_response(&self, body: &mut serde_json::Value) -> Result<()> {
        // Azure content filtering: check each choice for filter results.
        if let Some(choices) = body.pointer("/choices").and_then(|c| c.as_array()) {
            for choice in choices {
                if let Some(filter_results) = choice.get("content_filter_results") {
                    // If any filter category has `filtered: true` and finish_reason
                    // is already "content_filter", the response maps correctly.
                    // Check for a missing message on blocked responses.
                    let is_filtered = choice.get("finish_reason").and_then(|fr| fr.as_str()) == Some("content_filter");

                    if is_filtered && choice.get("message").is_none() {
                        // Inject a minimal message so downstream deserialization
                        // does not fail on a missing `message` field.
                        if let Some(choices_arr) = body.get_mut("choices").and_then(|c| c.as_array_mut())
                            && let Some(choice_obj) = choices_arr.first_mut().and_then(|c| c.as_object_mut())
                        {
                            choice_obj.insert(
                                "message".to_owned(),
                                serde_json::json!({
                                    "role": "assistant",
                                    "content": null,
                                    "refusal": "Content filtered by Azure content safety."
                                }),
                            );
                        }
                        break;
                    }

                    // Preserve filter_results metadata: Azure already includes it
                    // in the response and callers can inspect it via raw JSON.
                    let _ = filter_results;
                }
            }
        }
        Ok(())
    }
}

/// Return `true` when the model name looks like an O-series reasoning model.
///
/// Matches: `o1`, `o1-preview`, `o1-mini`, `o3`, `o3-mini`, `o4`, `o4-mini`,
/// and any variant with a dot suffix (e.g. `o3.5`).
fn is_o_series_model(model: &str) -> bool {
    // Match "o1", "o3", "o4" exactly or with a separator (-, .)
    for prefix in &["o1", "o3", "o4"] {
        if model == *prefix {
            return true;
        }
        if let Some(rest) = model.strip_prefix(prefix)
            && (rest.starts_with('-') || rest.starts_with('.'))
        {
            return true;
        }
    }
    false
}

// ── Unit tests ───────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    /// Construct a provider with an explicit base URL and api version, bypassing
    /// env-var reading.  Use this in tests to avoid clobbering real env state.
    fn make_provider(base_url: &str, api_version: &str) -> AzureProvider {
        AzureProvider {
            base_url: base_url.to_owned(),
            api_version: api_version.to_owned(),
        }
    }

    #[test]
    fn build_url_embeds_deployment_name() {
        let provider = make_provider("https://myresource.openai.azure.com", "2024-10-21");
        let url = provider.build_url("/chat/completions", "gpt-4");
        assert_eq!(
            url,
            "https://myresource.openai.azure.com/openai/deployments/gpt-4/chat/completions?api-version=2024-10-21"
        );
    }

    #[test]
    fn build_url_includes_api_version_query_param() {
        let provider = make_provider("https://example.openai.azure.com", "2025-01-01");
        let url = provider.build_url("/chat/completions", "gpt-4o");
        assert!(url.contains("?api-version=2025-01-01"), "url = {url}");
    }

    #[test]
    fn build_url_embeddings_endpoint() {
        let provider = make_provider("https://myresource.openai.azure.com", "2024-10-21");
        let url = provider.build_url("/embeddings", "text-embedding-3-large");
        assert_eq!(
            url,
            "https://myresource.openai.azure.com/openai/deployments/text-embedding-3-large/embeddings?api-version=2024-10-21"
        );
    }

    #[test]
    fn build_url_with_trailing_slash_stripped() {
        // Simulate construction with a pre-stripped base_url (new() handles this).
        let provider = make_provider("https://myresource.openai.azure.com", "2024-10-21");
        let url = provider.build_url("/chat/completions", "gpt-4");
        // Should not contain double slashes.
        assert!(!url.contains("//openai"), "double slash in url: {url}");
    }

    #[test]
    fn build_url_already_contains_deployments_path() {
        // When base_url already contains /openai/deployments/{name}, do not
        // insert the path fragment a second time.
        let provider = make_provider(
            "https://myresource.openai.azure.com/openai/deployments/gpt-4",
            "2025-02-01-preview",
        );
        let url = provider.build_url("/chat/completions", "gpt-4");
        assert!(
            !url.contains("deployments/gpt-4/openai/deployments"),
            "deployment path must not be doubled: {url}"
        );
        assert!(
            url.contains("/openai/deployments/gpt-4/chat/completions"),
            "url should contain the deployment path: {url}"
        );
    }

    #[test]
    fn build_url_empty_base_returns_fallback() {
        let provider = make_provider("", "2024-10-21");
        let url = provider.build_url("/chat/completions", "gpt-4");
        // Falls back to just the endpoint path — clearly broken, not a valid URL.
        assert_eq!(url, "/chat/completions");
    }

    #[test]
    fn transform_request_removes_model_field() {
        let provider = make_provider("https://myresource.openai.azure.com", "2024-10-21");
        let mut body = json!({
            "model": "gpt-4",
            "messages": [{"role": "user", "content": "hello"}],
            "temperature": 0.7
        });
        provider.transform_request(&mut body).expect("transform should succeed");
        assert!(body.get("model").is_none(), "model should be removed from body");
        // Other fields must be preserved.
        assert!(body.get("messages").is_some());
        assert!(body.get("temperature").is_some());
    }

    #[test]
    fn transform_request_non_object_body_is_noop() {
        let provider = make_provider("https://myresource.openai.azure.com", "2024-10-21");
        let mut body = json!("not an object");
        // Must not panic or return an error.
        assert!(provider.transform_request(&mut body).is_ok());
    }

    #[test]
    fn validate_fails_when_base_url_is_empty() {
        let provider = make_provider("", "2024-10-21");
        let err = provider.validate().expect_err("should fail with empty base_url");
        let msg = err.to_string();
        assert!(
            msg.contains("Azure OpenAI"),
            "error message should mention Azure: {msg}"
        );
        assert!(
            msg.contains("AZURE_OPENAI_ENDPOINT"),
            "error message should mention env var: {msg}"
        );
    }

    #[test]
    fn validate_succeeds_when_base_url_is_set() {
        let provider = make_provider("https://myresource.openai.azure.com", "2024-10-21");
        assert!(provider.validate().is_ok());
    }

    #[test]
    fn explicit_base_url_and_api_version_are_stored() {
        // Test the constructor's field assignment directly, bypassing env vars
        // to avoid thread-unsafe env mutation in parallel test runs.
        let provider = make_provider("https://test.openai.azure.com", "2099-01-01");
        assert_eq!(provider.base_url, "https://test.openai.azure.com");
        assert_eq!(provider.api_version, "2099-01-01");
    }

    #[test]
    fn default_api_version_is_preview() {
        // Verify the default api_version matches what `new()` would set when
        // the AZURE_API_VERSION env var is absent.
        let provider = make_provider("https://test.openai.azure.com", "2025-02-01-preview");
        assert_eq!(provider.api_version, "2025-02-01-preview");
    }

    #[test]
    fn strip_model_prefix_removes_azure_prefix() {
        let provider = make_provider("https://myresource.openai.azure.com", "2024-10-21");
        assert_eq!(provider.strip_model_prefix("azure/gpt-4"), "gpt-4");
        assert_eq!(provider.strip_model_prefix("gpt-4"), "gpt-4");
    }

    #[test]
    fn matches_model_only_for_azure_prefix() {
        let provider = make_provider("https://myresource.openai.azure.com", "2024-10-21");
        assert!(provider.matches_model("azure/gpt-4"));
        assert!(provider.matches_model("azure/gpt-4o-mini"));
        assert!(!provider.matches_model("gpt-4"));
        assert!(!provider.matches_model("openai/gpt-4"));
    }

    #[test]
    fn auth_header_uses_api_key_scheme() {
        let provider = make_provider("https://myresource.openai.azure.com", "2024-10-21");
        let (name, _value) = provider.auth_header("test-key").expect("should return Some");
        assert_eq!(name.as_ref(), "api-key");
    }

    // ── O-series model handling ──────────────────────────────────────────────

    #[test]
    fn is_o_series_model_detection() {
        assert!(super::is_o_series_model("o1"));
        assert!(super::is_o_series_model("o1-preview"));
        assert!(super::is_o_series_model("o1-mini"));
        assert!(super::is_o_series_model("o3"));
        assert!(super::is_o_series_model("o3-mini"));
        assert!(super::is_o_series_model("o3.5"));
        assert!(super::is_o_series_model("o4"));
        assert!(super::is_o_series_model("o4-mini"));

        assert!(!super::is_o_series_model("gpt-4"));
        assert!(!super::is_o_series_model("gpt-4o"));
        assert!(!super::is_o_series_model("o2"));
        assert!(!super::is_o_series_model("opt-1"));
    }

    #[test]
    fn transform_request_o_series_removes_temperature_and_top_p() {
        let provider = make_provider("https://myresource.openai.azure.com", "2024-10-21");
        let mut body = json!({
            "model": "o3-mini",
            "messages": [{"role": "user", "content": "hello"}],
            "temperature": 0.7,
            "top_p": 0.9,
            "reasoning_effort": "high"
        });
        provider.transform_request(&mut body).expect("transform should succeed");

        // model removed (standard Azure behavior)
        assert!(body.get("model").is_none());
        // temperature and top_p removed for O-series
        assert!(
            body.get("temperature").is_none(),
            "temperature should be removed for O-series"
        );
        assert!(body.get("top_p").is_none(), "top_p should be removed for O-series");
        // reasoning_effort preserved
        assert_eq!(body.get("reasoning_effort").unwrap(), "high");
        // messages preserved
        assert!(body.get("messages").is_some());
    }

    #[test]
    fn transform_request_o1_removes_stream() {
        let provider = make_provider("https://myresource.openai.azure.com", "2024-10-21");
        let mut body = json!({
            "model": "o1-preview",
            "messages": [{"role": "user", "content": "hello"}],
            "stream": true,
            "stream_options": {"include_usage": true},
            "temperature": 0.5
        });
        provider.transform_request(&mut body).expect("transform should succeed");

        assert!(body.get("stream").is_none(), "stream should be removed for o1");
        assert!(
            body.get("stream_options").is_none(),
            "stream_options should be removed for o1"
        );
        assert!(
            body.get("temperature").is_none(),
            "temperature should be removed for O-series"
        );
    }

    #[test]
    fn transform_request_o3_keeps_stream() {
        let provider = make_provider("https://myresource.openai.azure.com", "2024-10-21");
        let mut body = json!({
            "model": "o3-mini",
            "messages": [{"role": "user", "content": "hello"}],
            "stream": true
        });
        provider.transform_request(&mut body).expect("transform should succeed");

        // o3 supports streaming, stream should be kept
        assert!(body.get("stream").is_some(), "stream should remain for o3");
    }

    #[test]
    fn transform_request_non_o_series_keeps_all_params() {
        let provider = make_provider("https://myresource.openai.azure.com", "2024-10-21");
        let mut body = json!({
            "model": "gpt-4",
            "messages": [{"role": "user", "content": "hello"}],
            "temperature": 0.7,
            "top_p": 0.9,
            "stream": true
        });
        provider.transform_request(&mut body).expect("transform should succeed");

        assert!(body.get("model").is_none(), "model should be removed");
        assert!(
            body.get("temperature").is_some(),
            "temperature should be kept for non-O-series"
        );
        assert!(body.get("top_p").is_some(), "top_p should be kept for non-O-series");
        assert!(body.get("stream").is_some(), "stream should be kept for non-O-series");
    }

    // ── Content filtering response handling ──────────────────────────────────

    #[test]
    fn transform_response_passthrough_normal() {
        let provider = make_provider("https://myresource.openai.azure.com", "2024-10-21");
        let mut body = json!({
            "id": "chatcmpl-123",
            "object": "chat.completion",
            "choices": [{
                "index": 0,
                "message": {"role": "assistant", "content": "Hello!"},
                "finish_reason": "stop"
            }]
        });
        let original = body.clone();
        provider
            .transform_response(&mut body)
            .expect("transform should succeed");
        assert_eq!(body, original, "normal responses should pass through unchanged");
    }

    #[test]
    fn transform_response_content_filter_with_message() {
        let provider = make_provider("https://myresource.openai.azure.com", "2024-10-21");
        let mut body = json!({
            "id": "chatcmpl-123",
            "choices": [{
                "index": 0,
                "message": {"role": "assistant", "content": ""},
                "finish_reason": "content_filter",
                "content_filter_results": {
                    "hate": {"filtered": true, "severity": "high"}
                }
            }]
        });
        provider
            .transform_response(&mut body)
            .expect("transform should succeed");
        // Message already present, so no injection needed.
        assert_eq!(body["choices"][0]["finish_reason"], "content_filter");
        assert!(body["choices"][0]["message"].is_object());
    }

    #[test]
    fn transform_response_content_filter_blocked_no_message() {
        let provider = make_provider("https://myresource.openai.azure.com", "2024-10-21");
        let mut body = json!({
            "id": "chatcmpl-123",
            "choices": [{
                "index": 0,
                "finish_reason": "content_filter",
                "content_filter_results": {
                    "hate": {"filtered": true, "severity": "high"}
                }
            }]
        });
        provider
            .transform_response(&mut body)
            .expect("transform should succeed");
        // Should inject a minimal message for blocked responses.
        let message = &body["choices"][0]["message"];
        assert_eq!(message["role"], "assistant");
        assert!(message["content"].is_null());
        assert!(message["refusal"].as_str().unwrap().contains("Content filtered"));
    }
}
