use std::borrow::Cow;
use std::collections::{HashMap, HashSet};
use std::sync::LazyLock;
use std::time::{SystemTime, UNIX_EPOCH};

use serde::Deserialize;

use crate::error::{LiterLmError, Result};

/// Return the current Unix epoch timestamp in seconds.
///
/// Used by provider transformers to populate the `created` field in
/// OpenAI-compatible response objects. Falls back to `0` if the system
/// clock is before the epoch (should never happen in practice).
pub(crate) fn unix_timestamp_secs() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0)
}

/// The streaming wire format a provider uses for its response stream.
///
/// Most providers use standard Server-Sent Events (SSE).  AWS Bedrock uses
/// a proprietary binary EventStream framing.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StreamFormat {
    /// Standard Server-Sent Events (text/event-stream).
    Sse,
    /// AWS EventStream binary framing (application/vnd.amazon.eventstream).
    AwsEventStream,
}

// Embed the generated providers registry at compile time.
// Path: crates/liter-lm/src/provider/mod.rs → ../../../../schemas/providers.json
const PROVIDERS_JSON: &str = include_str!("../../../../schemas/providers.json");

/// Lazy-initialised registry parsed from the embedded JSON.
/// Stores a `Result` so that parse failures surface at call time rather than
/// panicking the process (fix for the `.expect()` on LazyLock).
static REGISTRY: LazyLock<std::result::Result<ProviderRegistry, String>> =
    LazyLock::new(|| serde_json::from_str(PROVIDERS_JSON).map_err(|e| e.to_string()));

/// Access the registry, returning an error if the embedded JSON was invalid.
fn registry() -> Result<&'static ProviderRegistry> {
    REGISTRY.as_ref().map_err(|e| LiterLmError::ServerError {
        message: format!("embedded schemas/providers.json is invalid: {e}"),
    })
}

// ── Registry types (deserialised from providers.json) ────────────────────────

#[derive(Debug, Deserialize)]
struct ProviderRegistry {
    providers: Vec<ProviderConfig>,
    /// Set of complex provider names for O(1) lookup.
    ///
    /// Deserialized from a JSON array; converted to a `HashSet` for fast
    /// membership tests in the hot `detect_provider` path.
    #[serde(default, deserialize_with = "deserialize_hashset")]
    complex_providers: HashSet<String>,
}

fn deserialize_hashset<'de, D>(deserializer: D) -> std::result::Result<HashSet<String>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let vec = Vec::<String>::deserialize(deserializer)?;
    Ok(vec.into_iter().collect())
}

/// Static configuration for a single provider entry in providers.json.
#[derive(Debug, Clone, Deserialize)]
pub struct ProviderConfig {
    pub name: String,
    pub display_name: Option<String>,
    pub base_url: Option<String>,
    pub auth: Option<AuthConfig>,
    pub endpoints: Option<Vec<String>>,
    pub model_prefixes: Option<Vec<String>>,
    /// Internal parameter mapping overrides for this provider.
    /// Not part of the public API; use provider-specific transform_request instead.
    /// Reserved for future use: loaded from providers.json but not yet consumed.
    #[allow(dead_code)]
    pub(crate) param_mappings: Option<HashMap<String, String>>,
}

/// Auth scheme used by a provider.
#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum AuthType {
    /// Standard `Authorization: Bearer <key>` header.
    Bearer,
    /// `x-api-key: <key>` header (also handles `"header"` and `"x-api-key"` aliases).
    #[serde(alias = "header", alias = "x-api-key")]
    ApiKey,
    /// No authentication header required.
    None,
    /// Unrecognised auth scheme — falls back to bearer.
    #[serde(other)]
    Unknown,
}

/// Auth configuration block.
#[derive(Debug, Clone, Deserialize)]
pub struct AuthConfig {
    #[serde(rename = "type")]
    pub auth_type: AuthType,
    pub env_var: Option<String>,
}

// ── Provider trait ───────────────────────────────────────────────────────────

/// A provider defines how to reach an LLM API endpoint.
pub trait Provider: Send + Sync {
    /// Validate provider configuration at construction time.
    ///
    /// Called by [`DefaultClient::new`] immediately after the provider is
    /// resolved.  Returning an error here surfaces misconfiguration early
    /// (e.g. missing Azure `base_url`) rather than on the first request.
    ///
    /// The default implementation is a no-op; providers with required
    /// configuration fields (like Azure) override this.
    fn validate(&self) -> Result<()> {
        Ok(())
    }

    /// Provider name (e.g., "openai").
    fn name(&self) -> &str;

    /// Base URL (e.g., "https://api.openai.com/v1").
    fn base_url(&self) -> &str;

    /// Build the authorization header as `Some((header-name, header-value))`.
    ///
    /// Returns `None` when the provider requires no authentication header
    /// (e.g. local models or providers with `auth: none`).  Callers must skip
    /// inserting any header when `None` is returned.
    ///
    /// When `Some`, returns a static header name and a borrowed-or-owned value
    /// to avoid allocating the header name string on every request.
    fn auth_header<'a>(&'a self, api_key: &'a str) -> Option<(Cow<'static, str>, Cow<'a, str>)>;

    /// Additional static headers required by this provider beyond the auth header.
    ///
    /// Most providers return an empty slice.  Use this for provider-mandated
    /// headers like Anthropic's `anthropic-version`.
    fn extra_headers(&self) -> &'static [(&'static str, &'static str)] {
        &[]
    }

    /// Whether this provider matches a given model string.
    fn matches_model(&self, model: &str) -> bool;

    /// Strip any provider-routing prefix from a model name before sending it
    /// in the request body.
    ///
    /// E.g. `"groq/llama3-70b"` → `"llama3-70b"`.
    /// Returns the model name unchanged when no prefix is present.
    fn strip_model_prefix<'m>(&self, model: &'m str) -> &'m str {
        // Try "name/" prefix without allocating.
        if let Some(rest) = model.strip_prefix(self.name())
            && let Some(stripped) = rest.strip_prefix('/')
        {
            return stripped;
        }
        model
    }

    /// Path for chat completions endpoint.
    fn chat_completions_path(&self) -> &str {
        "/chat/completions"
    }

    /// Path for embeddings endpoint.
    fn embeddings_path(&self) -> &str {
        "/embeddings"
    }

    /// Path for list models endpoint.
    fn models_path(&self) -> &str {
        "/models"
    }

    /// Whether streaming is supported.
    #[allow(dead_code)] // reserved for future provider-capability checking
    fn supports_streaming(&self) -> bool {
        true
    }

    /// Transform the request body before sending, if needed.
    fn transform_request(&self, body: &mut serde_json::Value) -> Result<()> {
        let _ = body;
        Ok(())
    }

    /// Transform the raw response JSON before deserialization into canonical types.
    ///
    /// Providers returning non-OpenAI formats (Anthropic, Bedrock, Vertex) override
    /// this to normalize their native response into OpenAI-compatible JSON.
    /// The default implementation is a no-op (OpenAI-compatible responses pass through
    /// unchanged).
    fn transform_response(&self, _body: &mut serde_json::Value) -> Result<()> {
        Ok(())
    }

    /// Build the full URL for a specific endpoint and model.
    ///
    /// Default: `{base_url}{endpoint_path}`.  Providers like Azure and Bedrock
    /// override this to embed deployment names, model IDs, or query parameters
    /// into the URL.
    fn build_url(&self, endpoint_path: &str, _model: &str) -> String {
        format!("{}{}", self.base_url(), endpoint_path)
    }

    /// Parse a single SSE event data string into a `ChatCompletionChunk`.
    ///
    /// Default: OpenAI format (straight JSON parse).
    /// Anthropic and Vertex override for their native streaming event formats.
    ///
    /// The `[DONE]` sentinel is handled at the SSE parser level before this
    /// method is called, so implementations do not need to check for it.
    ///
    /// Returns `Ok(Some(chunk))` for a successfully parsed event.
    /// Returns `Ok(None)` to skip this event (continue reading the stream).
    /// Returns `Err` when the event cannot be parsed.
    fn parse_stream_event(&self, event_data: &str) -> Result<Option<crate::types::ChatCompletionChunk>> {
        serde_json::from_str::<crate::types::ChatCompletionChunk>(event_data)
            .map(Some)
            .map_err(|e| LiterLmError::Streaming {
                message: format!("failed to parse SSE data: {e}"),
            })
    }

    /// The streaming wire format this provider uses.
    ///
    /// Default: [`StreamFormat::Sse`].  Override for providers that use
    /// non-SSE framing (e.g. AWS Bedrock EventStream).
    fn stream_format(&self) -> StreamFormat {
        StreamFormat::Sse
    }

    /// Build the full URL for a streaming request.
    ///
    /// Default: delegates to [`Provider::build_url`].  Providers whose
    /// streaming endpoint differs from the non-streaming one (e.g. Bedrock
    /// uses `/converse-stream` vs `/converse`) override this.
    fn build_stream_url(&self, endpoint_path: &str, model: &str) -> String {
        self.build_url(endpoint_path, model)
    }

    /// Compute dynamic signing headers for the outgoing request.
    ///
    /// Called by the client just before sending each request.  The default
    /// implementation returns an empty vector (no extra signing required).
    ///
    /// Providers that use request-signing (e.g. AWS Bedrock with SigV4) override
    /// this to return the computed `Authorization`, `x-amz-date`, and
    /// `x-amz-security-token` headers.  The returned headers are merged with the
    /// provider's static [`Provider::extra_headers`] before the request is sent.
    ///
    /// # Arguments
    ///
    /// - `method`: HTTP method string, e.g. `"POST"`.
    /// - `url`: Full request URL including path and query string.
    /// - `body`: Serialised request body bytes (used in the payload hash).
    fn signing_headers(&self, method: &str, url: &str, body: &[u8]) -> Vec<(String, String)> {
        let _ = (method, url, body);
        vec![]
    }
}

pub mod anthropic;
pub mod azure;
pub mod bedrock;
pub mod vertex;

// ── Built-in providers ───────────────────────────────────────────────────────

/// Built-in OpenAI provider.
pub struct OpenAiProvider;

impl Provider for OpenAiProvider {
    fn name(&self) -> &str {
        "openai"
    }

    fn base_url(&self) -> &str {
        "https://api.openai.com/v1"
    }

    fn auth_header<'a>(&'a self, api_key: &'a str) -> Option<(Cow<'static, str>, Cow<'a, str>)> {
        Some((Cow::Borrowed("Authorization"), Cow::Owned(format!("Bearer {api_key}"))))
    }

    fn matches_model(&self, model: &str) -> bool {
        model.starts_with("gpt-")
            || model.starts_with("o1-")
            || model.starts_with("o3-")
            || model.starts_with("o4-")
            || model == "o1"
            || model == "o3"
            || model == "o4"
            || model.starts_with("dall-e-")
            || model.starts_with("whisper-")
            || model.starts_with("tts-")
            || model.starts_with("text-embedding-")
            || model.starts_with("chatgpt-")
    }

    fn strip_model_prefix<'m>(&self, model: &'m str) -> &'m str {
        // OpenAI models have no routing prefix.
        model
    }
}

/// A generic OpenAI-compatible provider (configurable base_url + bearer auth).
pub struct OpenAiCompatibleProvider {
    pub name: String,
    pub base_url: String,
    #[allow(dead_code)] // reserved for future env-var based key injection
    pub env_var: Option<&'static str>,
    pub model_prefixes: Vec<String>,
}

impl Provider for OpenAiCompatibleProvider {
    fn name(&self) -> &str {
        &self.name
    }

    fn base_url(&self) -> &str {
        &self.base_url
    }

    fn auth_header<'a>(&'a self, api_key: &'a str) -> Option<(Cow<'static, str>, Cow<'a, str>)> {
        Some((Cow::Borrowed("Authorization"), Cow::Owned(format!("Bearer {api_key}"))))
    }

    fn matches_model(&self, model: &str) -> bool {
        self.model_prefixes
            .iter()
            .any(|prefix| model.starts_with(prefix.as_str()))
    }
}

/// A data-driven provider backed by a [`ProviderConfig`] entry from providers.json.
///
/// Used for simple providers that are fully described by their JSON config.
/// Complex providers (AWS Bedrock, Vertex AI, etc.) use dedicated implementations.
///
/// # Construction
///
/// Construct only via [`ConfigDrivenProvider::new`], which is intentionally
/// `pub(crate)` — callers outside this crate must go through [`detect_provider`].
///
/// # `base_url` contract
///
/// [`Provider::base_url`] returns an empty string when the provider config has
/// no `base_url` entry.  This is safe because [`detect_provider`] guards the
/// `base_url.is_some()` condition before constructing a `ConfigDrivenProvider`,
/// so a correctly-routed request will never produce an empty URL.  A manually
/// constructed instance (hypothetically) would produce a clearly-broken URL
/// (`/chat/completions`) that fails immediately at the HTTP layer.
pub struct ConfigDrivenProvider {
    config: &'static ProviderConfig,
}

impl ConfigDrivenProvider {
    #[must_use]
    pub(crate) fn new(config: &'static ProviderConfig) -> Self {
        Self { config }
    }
}

impl Provider for ConfigDrivenProvider {
    fn name(&self) -> &str {
        &self.config.name
    }

    fn base_url(&self) -> &str {
        // Return an empty string when unconfigured; `transform_request` or the
        // HTTP layer will surface a useful error before any network call goes out.
        self.config.base_url.as_deref().unwrap_or("")
    }

    fn auth_header<'a>(&'a self, api_key: &'a str) -> Option<(Cow<'static, str>, Cow<'a, str>)> {
        let auth_type = self
            .config
            .auth
            .as_ref()
            .map(|a| &a.auth_type)
            .unwrap_or(&AuthType::Bearer);

        match auth_type {
            // No auth header required; return None so callers skip it entirely.
            AuthType::None => None,
            AuthType::ApiKey => Some((Cow::Borrowed("x-api-key"), Cow::Borrowed(api_key))),
            // Bearer, Unknown, and anything else defaults to Bearer token.
            AuthType::Bearer | AuthType::Unknown => {
                Some((Cow::Borrowed("Authorization"), Cow::Owned(format!("Bearer {api_key}"))))
            }
        }
    }

    fn matches_model(&self, model: &str) -> bool {
        if let Some(prefixes) = &self.config.model_prefixes {
            prefixes.iter().any(|p| model.starts_with(p.as_str()))
        } else {
            false
        }
    }
}

// ── Provider detection ───────────────────────────────────────────────────────

/// Detect which provider to use based on model name.
///
/// Strategy:
/// 1. OpenAI hardcoded patterns (gpt-*, o1-*, text-embedding-*, …).
/// 2. Anthropic: `claude-*` model names or `anthropic/` prefix.
/// 3. Azure: `azure/` prefix.
/// 4. Vertex AI: `vertex_ai/` prefix.
/// 5. AWS Bedrock: `bedrock/` prefix.
/// 6. `"provider/"` prefix — look up the prefix in the registry.
/// 7. Walk all registry entries and check their `model_prefixes`.
///
/// Returns `None` when no built-in provider matches.  The caller should fall
/// back to a config-specified `base_url` or default to [`OpenAiProvider`].
///
/// Complex providers (those listed in `complex_providers` in providers.json)
/// are excluded from config-driven routing because they require custom
/// auth/request logic beyond simple bearer tokens.
pub fn detect_provider(model: &str) -> Option<Box<dyn Provider>> {
    // 1. OpenAI hardcoded patterns.
    let openai = OpenAiProvider;
    if openai.matches_model(model) {
        return Some(Box::new(openai));
    }

    // 2. Anthropic: "claude-*" model names or "anthropic/" prefix.
    let anthropic = anthropic::AnthropicProvider;
    if anthropic.matches_model(model) {
        return Some(Box::new(anthropic));
    }

    // 3. Azure: "azure/" prefix.
    if model.starts_with("azure/") {
        return Some(Box::new(azure::AzureProvider::new()));
    }

    // 4. Vertex AI: "vertex_ai/" prefix.
    if model.starts_with("vertex_ai/") {
        return Some(Box::new(vertex::VertexAiProvider::from_env()));
    }

    // 5. AWS Bedrock: "bedrock/" prefix.
    if model.starts_with("bedrock/") {
        return Some(Box::new(bedrock::BedrockProvider::from_env()));
    }

    // Grab the registry; if it failed to parse we cannot route.
    let reg = match REGISTRY.as_ref() {
        Ok(r) => r,
        Err(_) => return None,
    };

    // 6. Slash-prefix routing (e.g. "groq/llama3-70b").
    if let Some((prefix, _)) = model.split_once('/')
        && let Some(cfg) = reg.providers.iter().find(|p| p.name == prefix)
        && cfg.base_url.is_some()
        && !reg.complex_providers.contains(&cfg.name)
    {
        // cfg is &'static ProviderConfig because reg comes from LazyLock.
        // Only use the registry entry if it has a usable base_url and is not
        // a complex provider requiring dedicated auth logic.
        return Some(Box::new(ConfigDrivenProvider::new(cfg)));
    }

    // 7. Walk registry model_prefixes for unprefixed model names.
    for cfg in &reg.providers {
        if reg.complex_providers.contains(&cfg.name) {
            continue;
        }
        if let Some(prefixes) = &cfg.model_prefixes {
            let matches = prefixes
                .iter()
                .any(|p| model.starts_with(p.as_str()) && !p.ends_with('/'));
            if matches && cfg.base_url.is_some() {
                // cfg is &'static ProviderConfig because reg comes from LazyLock.
                return Some(Box::new(ConfigDrivenProvider::new(cfg)));
            }
        }
    }

    None
}

/// Return all provider configs from the registry.
///
/// Useful for tooling, documentation generation, or runtime enumeration.
pub fn all_providers() -> Result<&'static [ProviderConfig]> {
    Ok(&registry()?.providers)
}

/// Return the set of complex provider names.
///
/// Complex providers require custom auth/routing logic beyond simple bearer
/// tokens (e.g. AWS Bedrock SigV4, Vertex AI OAuth2).
///
/// The returned reference points into the static registry — no allocation.
pub fn complex_provider_names() -> Result<&'static HashSet<String>> {
    Ok(&registry()?.complex_providers)
}
