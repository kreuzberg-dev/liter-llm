use std::borrow::Cow;

use crate::error::Result;
use crate::provider::Provider;

/// Azure OpenAI provider.
///
/// Differences from the OpenAI-compatible baseline:
/// - Auth uses `api-key` instead of `Authorization: Bearer`.
/// - The base URL is customer-specific (`https://{resource}.openai.azure.com/openai`).
///   When used with the e2e mock server, the caller overrides `base_url` in the
///   `ClientConfig`, so no hardcoded URL is needed here.
/// - Model names are routed via the `azure/` prefix which is stripped before
///   being sent in the request body.
pub struct AzureProvider;

impl Provider for AzureProvider {
    fn name(&self) -> &str {
        "azure"
    }

    /// Azure base URL is always customer-specific.
    ///
    /// In production callers should override `base_url` in [`ClientConfig`] to
    /// `https://{resource}.openai.azure.com/openai`.  The empty string here
    /// causes an obvious failure at the HTTP layer if no override is provided,
    /// rather than silently hitting a wrong endpoint.
    fn base_url(&self) -> &str {
        ""
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

    fn transform_request(&self, _body: &mut serde_json::Value) -> Result<()> {
        Ok(())
    }
}
