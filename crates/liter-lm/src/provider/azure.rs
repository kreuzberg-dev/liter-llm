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
///   `2024-10-21`).
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
    ///   `2024-10-21`).
    #[must_use]
    pub fn new() -> Self {
        let base_url = std::env::var("AZURE_OPENAI_ENDPOINT")
            .or_else(|_| std::env::var("AZURE_ENDPOINT"))
            .unwrap_or_default()
            .trim_end_matches('/')
            .to_owned();

        let api_version = std::env::var("AZURE_API_VERSION").unwrap_or_else(|_| "2024-10-21".to_owned());

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
        format!(
            "{}/openai/deployments/{}{}?api-version={}",
            self.base_url, model, endpoint_path, self.api_version
        )
    }

    /// Remove `model` from the request body.
    ///
    /// Azure routes to a specific deployment via the URL (see [`build_url`]);
    /// including `model` in the body causes a 400 error from the API.
    ///
    /// [`build_url`]: AzureProvider::build_url
    fn transform_request(&self, body: &mut serde_json::Value) -> Result<()> {
        if let Some(obj) = body.as_object_mut() {
            obj.remove("model");
        }
        Ok(())
    }
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
    fn api_version_env_var_respected() {
        // Override env var and reconstruct — use a unique var value to avoid
        // polluting other tests.
        // SAFETY: This test runs single-threaded in its own process segment;
        // mutating env vars is safe when no other thread reads them concurrently.
        // We restore all vars before the assertions so concurrent test runs are
        // not affected (cargo test isolates each #[test] in its own thread, not
        // process, so this test must not be run with --test-threads > 1 if env
        // isolation is required).
        unsafe {
            std::env::set_var("AZURE_OPENAI_ENDPOINT", "https://test.openai.azure.com");
            std::env::set_var("AZURE_API_VERSION", "2099-01-01");
        }
        let provider = AzureProvider::new();
        unsafe {
            std::env::remove_var("AZURE_OPENAI_ENDPOINT");
            std::env::remove_var("AZURE_API_VERSION");
        }

        assert_eq!(provider.base_url, "https://test.openai.azure.com");
        assert_eq!(provider.api_version, "2099-01-01");
    }

    #[test]
    fn azure_endpoint_fallback_env_var() {
        // AZURE_ENDPOINT should be used when AZURE_OPENAI_ENDPOINT is not set.
        // SAFETY: Same rationale as api_version_env_var_respected above.
        unsafe {
            std::env::remove_var("AZURE_OPENAI_ENDPOINT");
            std::env::set_var("AZURE_ENDPOINT", "https://fallback.openai.azure.com");
        }
        let provider = AzureProvider::new();
        unsafe {
            std::env::remove_var("AZURE_ENDPOINT");
        }

        assert_eq!(provider.base_url, "https://fallback.openai.azure.com");
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
}
