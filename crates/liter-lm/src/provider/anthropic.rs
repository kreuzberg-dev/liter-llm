use std::borrow::Cow;

use crate::error::Result;
use crate::provider::Provider;

/// Anthropic provider (Claude model family).
///
/// Differences from the OpenAI-compatible baseline:
/// - Auth uses `x-api-key` instead of `Authorization: Bearer`.
/// - Requires a mandatory `anthropic-version` header on every request.
/// - Model names start with `claude-` or are routed via the `anthropic/` prefix.
pub struct AnthropicProvider;

impl Provider for AnthropicProvider {
    fn name(&self) -> &str {
        "anthropic"
    }

    fn base_url(&self) -> &str {
        "https://api.anthropic.com/v1"
    }

    fn auth_header<'a>(&'a self, api_key: &'a str) -> Option<(Cow<'static, str>, Cow<'a, str>)> {
        // Anthropic uses x-api-key, not Authorization: Bearer.
        Some((Cow::Borrowed("x-api-key"), Cow::Borrowed(api_key)))
    }

    fn extra_headers(&self) -> Vec<(Cow<'static, str>, Cow<'static, str>)> {
        vec![(Cow::Borrowed("anthropic-version"), Cow::Borrowed("2023-06-01"))]
    }

    fn matches_model(&self, model: &str) -> bool {
        model.starts_with("claude-") || model.starts_with("anthropic/")
    }

    fn strip_model_prefix<'m>(&self, model: &'m str) -> &'m str {
        model.strip_prefix("anthropic/").unwrap_or(model)
    }

    fn transform_request(&self, _body: &mut serde_json::Value) -> Result<()> {
        Ok(())
    }
}
