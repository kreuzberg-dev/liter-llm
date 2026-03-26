use std::borrow::Cow;

use crate::provider::Provider;

/// Google Vertex AI provider.
///
/// Differences from the OpenAI-compatible baseline:
/// - Auth uses `Authorization: Bearer <token>` where the token is a Google
///   Cloud OAuth2 access token (obtained via ADC, service account, or
///   `gcloud auth print-access-token`).
/// - The base URL is **required** and must be set via `base_url` in
///   [`ClientConfig`] because it encodes the GCP region and project:
///   `https://{region}-aiplatform.googleapis.com/v1beta1/projects/{project}/locations/{region}/endpoints/openapi`
/// - Model names are routed via the `vertex_ai/` prefix which is stripped
///   before being sent in the request body.
/// - The OpenAI-compatible endpoint is used (`/chat/completions`,
///   `/embeddings`), so no message format translation is required.
///
/// # Token management
///
/// For v1.1, supply a pre-obtained access token as the `api_key` parameter.
/// Token refresh is the caller's responsibility.  A future release will add
/// ADC / service-account-based automatic refresh.
///
/// # Configuration
///
/// ```rust,ignore
/// let config = ClientConfigBuilder::new("ya29.your-access-token")
///     .base_url(
///         "https://us-central1-aiplatform.googleapis.com/v1beta1/\
///          projects/my-project/locations/us-central1/endpoints/openapi",
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
    /// [`ClientConfig`].  The caller is expected to always supply a `base_url`
    /// pointing at their Vertex AI OpenAI-compatible endpoint.
    fn base_url(&self) -> &str {
        ""
    }

    fn auth_header<'a>(&'a self, api_key: &'a str) -> Option<(Cow<'static, str>, Cow<'a, str>)> {
        // Vertex AI requires an OAuth2 Bearer token, not a plain API key.
        Some((Cow::Borrowed("Authorization"), Cow::Owned(format!("Bearer {api_key}"))))
    }

    fn matches_model(&self, model: &str) -> bool {
        model.starts_with("vertex_ai/")
    }

    fn strip_model_prefix<'m>(&self, model: &'m str) -> &'m str {
        model.strip_prefix("vertex_ai/").unwrap_or(model)
    }
}
