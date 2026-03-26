use std::borrow::Cow;

use crate::error::{LiterLmError, Result};
use crate::provider::Provider;

/// Azure OpenAI provider.
///
/// Differences from the OpenAI-compatible baseline:
/// - Auth uses `api-key` instead of `Authorization: Bearer`.
/// - The base URL is **required** and must be set via `base_url` in
///   [`ClientConfig`]: `https://{resource}.openai.azure.com/openai`.
///   Azure has no single shared endpoint — each customer has a unique resource
///   URL.  Failing to supply `base_url` will produce a clear error at request
///   time (see [`AzureProvider::transform_request`]) rather than silently
///   sending to a malformed endpoint.
/// - Model names are routed via the `azure/` prefix which is stripped before
///   being sent in the request body.
///
/// # Configuration
///
/// ```rust,ignore
/// let config = ClientConfigBuilder::new("your-azure-api-key")
///     .base_url("https://my-resource.openai.azure.com/openai")
///     .build();
/// let client = DefaultClient::new(config, Some("azure/gpt-4"))?;
/// ```
pub struct AzureProvider;

/// Sentinel used as the `base_url` return value when no override is configured.
///
/// An empty string is an obviously-broken URL (`/chat/completions`) that fails
/// immediately at the HTTP layer.  The error is made explicit in
/// [`AzureProvider::transform_request`] before any network call goes out.
const AZURE_MISSING_BASE_URL: &str = "";

impl Provider for AzureProvider {
    fn name(&self) -> &str {
        "azure"
    }

    /// Azure base URL is always customer-specific.
    ///
    /// Returns an empty string when no `base_url` override is present in
    /// [`ClientConfig`].  The HTTP layer will surface a connection error, but
    /// [`AzureProvider::transform_request`] checks for this condition first
    /// and returns a descriptive [`LiterLmError::BadRequest`].
    fn base_url(&self) -> &str {
        AZURE_MISSING_BASE_URL
    }

    fn auth_header<'a>(&'a self, api_key: &'a str) -> Option<(Cow<'static, str>, Cow<'a, str>)> {
        // Azure uses api-key, not Authorization: Bearer.
        Some((Cow::Borrowed("api-key"), Cow::Borrowed(api_key)))
    }

    fn matches_model(&self, model: &str) -> bool {
        model.starts_with("azure/")
    }

    fn strip_model_prefix<'m>(&self, model: &'m str) -> &'m str {
        model.strip_prefix("azure/").unwrap_or(model)
    }

    /// Validate that a `base_url` override is present before the request goes out.
    ///
    /// Azure requires a customer-specific resource URL.  If the caller forgot to
    /// set `base_url` in [`ClientConfig`], this returns a [`LiterLmError::BadRequest`]
    /// with a clear message rather than letting the HTTP layer fail with an
    /// opaque connection error to an empty URL.
    fn transform_request(&self, body: &mut serde_json::Value) -> Result<()> {
        // `base_url()` returns an empty string when unconfigured.
        // Detect that sentinel and emit a useful error now, before any I/O.
        if self.base_url().is_empty() {
            return Err(LiterLmError::BadRequest {
                message: "Azure OpenAI requires `base_url` to be set in ClientConfig. \
                          Use the format: https://{resource}.openai.azure.com/openai"
                    .into(),
            });
        }
        let _ = body;
        Ok(())
    }
}
