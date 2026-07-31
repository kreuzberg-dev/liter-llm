use std::borrow::Cow;
use std::collections::{HashMap, HashSet};
use std::sync::LazyLock;
use std::time::{SystemTime, UNIX_EPOCH};

use serde::{Deserialize, Serialize};

use crate::error::{LiterLlmError, Result};

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
///
/// Deserialized from the `streaming_format` JSON field via [`serde`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum StreamFormat {
    /// Standard Server-Sent Events (text/event-stream).
    #[default]
    Sse,
    /// AWS EventStream binary framing (application/vnd.amazon.eventstream).
    AwsEventStream,
}

/// Static capability flags for a provider.
///
/// Each flag indicates whether the provider's models *generally* support that
/// feature.  For providers that aggregate many underlying models (e.g. Bedrock,
/// OpenRouter, vLLM) the flags reflect the superset of available model
/// capabilities — a flag being `true` means at least one model supports the
/// feature, not every model.
///
/// All flags default to `false` so that newly added providers are safe.
///
/// Access via the crate-level [`capabilities`] function:
///
/// ```rust
/// use liter_llm::capabilities;
///
/// let caps = capabilities("openai");
/// assert!(caps.function_calling);
/// assert!(caps.vision);
///
/// // Unknown providers return a default-all-false reference.
/// let unknown = capabilities("my-private-model");
/// assert!(!unknown.function_calling);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct ProviderCapabilities {
    /// The provider accepts image input in chat messages.
    pub vision: bool,
    /// The provider supports extended-thinking / reasoning tokens.
    pub reasoning: bool,
    /// The provider supports JSON-mode or `response_format` structured output.
    pub structured_output: bool,
    /// The provider supports tool / function calling.
    pub function_calling: bool,
    /// The provider accepts audio as input.
    pub audio_in: bool,
    /// The provider can generate audio / TTS output.
    pub audio_out: bool,
    /// The provider accepts video as input.
    pub video_in: bool,
}

/// Static all-false sentinel returned by [`capabilities`] for unknown providers.
static DEFAULT_CAPABILITIES: ProviderCapabilities = ProviderCapabilities {
    vision: false,
    reasoning: false,
    structured_output: false,
    function_calling: false,
    audio_in: false,
    audio_out: false,
    video_in: false,
};

/// Return the capability flags for a named provider.
///
/// Performs an O(n) linear scan over the embedded registry (165 entries).
/// Returns an owned value so bindings can pass capability data without
/// borrowing registry internals.
///
/// For unknown `provider_name` values the function returns an all-`false`
/// sentinel so callers never need to handle `Option`.
pub fn capabilities(provider_name: &str) -> ProviderCapabilities {
    let Ok(reg) = REGISTRY.as_ref() else {
        return DEFAULT_CAPABILITIES;
    };
    for entry in &reg.providers {
        if entry.config.name == provider_name {
            return entry.capabilities;
        }
    }
    DEFAULT_CAPABILITIES
}

const PROVIDERS_JSON: &str = include_str!("../../schemas/providers.json");

/// Lazy-initialised registry parsed from the embedded JSON.
/// Stores a `Result` so that parse failures surface at call time rather than
/// panicking the process (fix for the `.expect()` on LazyLock).
static REGISTRY: LazyLock<std::result::Result<ProviderRegistry, String>> = LazyLock::new(|| {
    serde_json::from_str::<ProviderRegistryRaw>(PROVIDERS_JSON)
        .map(ProviderRegistry::from_raw)
        .map_err(|e| e.to_string())
});

/// Access the registry, returning an error if the embedded JSON was invalid.
fn registry() -> Result<&'static ProviderRegistry> {
    REGISTRY.as_ref().map_err(|e| LiterLlmError::ServerError {
        message: format!("embedded schemas/providers.json is invalid: {e}"),
        status: 500,
    })
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
struct RegionalEndpointConfig {
    region: String,
    openai_base_url: String,
    anthropic_base_url: String,
    docs_root: String,
}

/// Internal JSON shape: each provider entry with capability flags and streaming
/// format, stored separately from the public [`ProviderConfig`] schema.
#[derive(Debug, Deserialize)]
struct ProviderEntry {
    #[serde(flatten)]
    config: ProviderConfig,
    #[serde(default)]
    capabilities: ProviderCapabilities,
    /// Protocol-specific base URLs available in each supported region.
    #[allow(dead_code)]
    #[serde(default)]
    regional_endpoints: Vec<RegionalEndpointConfig>,
    /// Streaming wire format for this provider.
    ///
    /// Deserialized from `providers.json` and available to future workstreams
    /// via `ProviderEntry` (e.g. to drive per-provider streaming-format routing
    /// without hardcoding the Bedrock special-case in `detect_provider`).
    #[allow(dead_code)]
    #[serde(default, rename = "streaming_format")]
    stream_format: StreamFormat,
}

/// Intermediate deserialization target for `providers.json`.
#[derive(Debug, Deserialize)]
struct ProviderRegistryRaw {
    providers: Vec<ProviderEntry>,
    /// Set of complex provider names for O(1) lookup.
    ///
    /// Deserialized from a JSON array; converted to a `HashSet` for fast
    /// membership tests in the hot `detect_provider` path.
    #[serde(default, deserialize_with = "deserialize_hashset")]
    complex_providers: HashSet<String>,
}

/// The parsed and indexed registry.
///
/// `providers` holds the full entries including capability flags and streaming
/// format (for internal routing and capability lookup).  `configs` is a
/// pre-extracted public-config slice returned by [`all_providers`].
#[derive(Debug)]
struct ProviderRegistry {
    providers: Vec<ProviderEntry>,
    configs: Vec<ProviderConfig>,
    complex_providers: HashSet<String>,
}

impl ProviderRegistry {
    fn from_raw(raw: ProviderRegistryRaw) -> Self {
        let configs = raw.providers.iter().map(|e| e.config.clone()).collect();
        Self {
            configs,
            providers: raw.providers,
            complex_providers: raw.complex_providers,
        }
    }
}

fn deserialize_hashset<'de, D>(deserializer: D) -> std::result::Result<HashSet<String>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let vec = Vec::<String>::deserialize(deserializer)?;
    Ok(vec.into_iter().collect())
}

/// Static configuration for a single provider entry in providers.json.
///
/// This struct deliberately does not include capability flags or streaming
/// format, which are accessed via the [`capabilities`] function.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderConfig {
    /// Provider identifier (matches the entry key in providers.json).
    pub name: String,
    /// Human-readable provider name shown in UIs.
    pub display_name: Option<String>,
    /// Base URL used as the default for this provider's HTTP client.
    pub base_url: Option<String>,
    /// Authentication scheme metadata (auth type + env var holding the key).
    pub auth: Option<AuthConfig>,
    /// Supported endpoint kinds (e.g. `chat`, `embeddings`).
    pub endpoints: Option<Vec<String>>,
    /// Model-name prefixes claimed by this provider (e.g. `["gpt-", "o1-"]`).
    pub model_prefixes: Option<Vec<String>>,
    /// Parameter key renaming for this provider.
    ///
    /// Each entry maps an OpenAI-spec field name (e.g. `"max_completion_tokens"`)
    /// to the name this provider expects (e.g. `"max_tokens"`).  Applied
    /// automatically by `ConfigDrivenProvider::transform_request`.
    pub param_mappings: Option<HashMap<String, String>>,
}

/// Auth scheme used by a provider.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
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
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthConfig {
    /// Auth scheme classification.
    #[serde(rename = "type")]
    pub auth_type: AuthType,
    /// Name of the environment variable that holds the API key (e.g. `"OPENAI_API_KEY"`).
    /// Holds the variable name, never the secret value.
    #[cfg_attr(target_arch = "wasm32", allow(dead_code))]
    pub env_var: Option<String>,
}

/// A provider defines how to reach an LLM API endpoint.
pub(crate) trait Provider: Send + Sync {
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

    /// Name of the environment variable that holds the API key for this provider.
    ///
    /// Returns `None` for providers that do not use an API key (e.g. auth type
    /// `none`), or for providers whose key source is handled out-of-band (e.g.
    /// AWS Bedrock credentials resolved via the AWS SDK).
    ///
    /// Used by [`DefaultClient::new`] to auto-load the API key from the
    /// environment when `load_env` is enabled and no explicit key was provided.
    #[cfg_attr(target_arch = "wasm32", allow(dead_code))]
    fn env_var(&self) -> Option<&str> {
        None
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

    /// Compute request-dependent headers based on the request body.
    ///
    /// Called by the client for each request. Use this for headers that
    /// vary per-request, like Anthropic's `anthropic-beta` which depends
    /// on whether thinking or hosted tools are enabled.
    ///
    /// The default implementation returns an empty vector.
    fn dynamic_headers(&self, _body: &serde_json::Value) -> Vec<(String, String)> {
        vec![]
    }

    /// Whether this provider matches a given model string.
    fn matches_model(&self, model: &str) -> bool;

    /// Strip any provider-routing prefix from a model name before sending it
    /// in the request body.
    ///
    /// E.g. `"groq/llama3-70b"` → `"llama3-70b"`.
    /// Returns the model name unchanged when no prefix is present.
    fn strip_model_prefix<'m>(&self, model: &'m str) -> &'m str {
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

    /// Path for image generations endpoint.
    fn image_generations_path(&self) -> &str {
        "/images/generations"
    }

    /// Path for text-to-speech endpoint.
    fn audio_speech_path(&self) -> &str {
        "/audio/speech"
    }

    /// Path for audio transcription endpoint.
    fn audio_transcriptions_path(&self) -> &str {
        "/audio/transcriptions"
    }

    /// Path for content moderation endpoint.
    fn moderations_path(&self) -> &str {
        "/moderations"
    }

    /// Path for document reranking endpoint.
    fn rerank_path(&self) -> &str {
        "/rerank"
    }

    /// Path for the files management endpoint (e.g. POST /files, GET /files/{id}).
    fn files_path(&self) -> &str {
        "/files"
    }

    /// Path for the batches management endpoint (e.g. POST /batches, GET /batches/{id}).
    fn batches_path(&self) -> &str {
        "/batches"
    }

    /// Path for the responses endpoint (e.g. POST /responses).
    fn responses_path(&self) -> &str {
        "/responses"
    }

    /// Path for the web/document search endpoint.
    fn search_path(&self) -> &str {
        "/search"
    }

    /// Path for the OCR (optical character recognition) endpoint.
    fn ocr_path(&self) -> &str {
        "/ocr"
    }

    /// Whether streaming is supported.
    #[allow(dead_code)]
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
            .map_err(|e| LiterLlmError::Streaming {
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

pub(crate) mod anthropic;
pub(crate) mod azure;
pub(crate) mod bedrock;
pub(crate) mod cohere;
pub mod custom;
pub(crate) mod github_copilot;
pub(crate) mod google_ai;
pub(crate) mod mistral;
pub mod outbound_policy;
pub(crate) mod vertex;

pub use outbound_policy::{
    OutboundPolicy, current_policy, set_outbound_policy, validate_outbound_url, validate_outbound_url_sync,
};

/// Built-in OpenAI provider.
pub(crate) struct OpenAiProvider;

impl Provider for OpenAiProvider {
    fn name(&self) -> &str {
        "openai"
    }

    fn base_url(&self) -> &str {
        "https://api.openai.com/v1"
    }

    fn env_var(&self) -> Option<&str> {
        Some("OPENAI_API_KEY")
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
            || model.starts_with("openai/")
    }

    fn strip_model_prefix<'m>(&self, model: &'m str) -> &'m str {
        model.strip_prefix("openai/").unwrap_or(model)
    }

    /// Hoist OpenAI audio response data into the `content` field.
    ///
    /// OpenAI audio-preview models return `choices[].message.audio = { data,
    /// transcript, format, ... }` alongside a null `content`.  Normalise this
    /// into `choices[].message.content` as a parts array so that
    /// [`AssistantContent::Parts`] deserialization works correctly.
    ///
    /// Layout emitted:
    /// - A `{ "type": "text", "text": transcript }` part when `transcript` is
    ///   present and non-empty.
    /// - A `{ "type": "output_audio", "audio": { "data": …, "format": … } }` part.
    ///
    /// For non-audio responses this is a no-op.
    fn transform_response(&self, body: &mut serde_json::Value) -> Result<()> {
        transform_openai_audio_response(body)
    }
}

/// Hoist an OpenAI audio response's `message.audio` field into `message.content`.
///
/// Mutates each choice in-place; returns `Ok(())` unconditionally.
pub(crate) fn transform_openai_audio_response(body: &mut serde_json::Value) -> Result<()> {
    use serde_json::json;

    let choices = match body.get_mut("choices").and_then(|c| c.as_array_mut()) {
        Some(c) => c,
        None => return Ok(()),
    };

    for choice in choices {
        let audio = match choice.pointer("/message/audio") {
            Some(a) if !a.is_null() => a.clone(),
            _ => continue,
        };

        let data = audio.get("data").and_then(|v| v.as_str()).unwrap_or("").to_owned();
        let format = audio.get("format").and_then(|v| v.as_str()).unwrap_or("wav").to_owned();
        let transcript = audio
            .get("transcript")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_owned();

        let mut parts: Vec<serde_json::Value> = vec![];
        if !transcript.is_empty() {
            parts.push(json!({"type": "text", "text": transcript}));
        }
        if !data.is_empty() {
            parts.push(json!({
                "type": "output_audio",
                "audio": {"data": data, "format": format}
            }));
        }

        if !parts.is_empty()
            && let Some(message) = choice.get_mut("message")
        {
            message["content"] = json!(parts);
        }
    }

    Ok(())
}

/// A generic OpenAI-compatible provider (configurable base_url + bearer auth).
pub(crate) struct OpenAiCompatibleProvider {
    pub name: String,
    pub base_url: String,
    /// Environment variable name for the API key, if known.
    #[cfg_attr(target_arch = "wasm32", allow(dead_code))]
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

    fn env_var(&self) -> Option<&str> {
        self.env_var
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
pub(crate) struct ConfigDrivenProvider {
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
        self.config.base_url.as_deref().unwrap_or("")
    }

    fn env_var(&self) -> Option<&str> {
        self.config.auth.as_ref().and_then(|a| a.env_var.as_deref())
    }

    fn transform_request(&self, body: &mut serde_json::Value) -> Result<()> {
        if let Some(mappings) = &self.config.param_mappings
            && let Some(obj) = body.as_object_mut()
        {
            for (from, to) in mappings {
                if let Some(val) = obj.remove(from.as_str()) {
                    obj.insert(to.clone(), val);
                }
            }
        }
        Ok(())
    }

    fn auth_header<'a>(&'a self, api_key: &'a str) -> Option<(Cow<'static, str>, Cow<'a, str>)> {
        let auth_type = self
            .config
            .auth
            .as_ref()
            .map(|a| &a.auth_type)
            .unwrap_or(&AuthType::Bearer);

        match auth_type {
            AuthType::None => None,
            AuthType::ApiKey => Some((Cow::Borrowed("x-api-key"), Cow::Borrowed(api_key))),
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

/// Detect which provider to use based on model name.
///
/// Strategy:
/// 1. OpenAI hardcoded patterns (gpt-*, o1-*, text-embedding-*, …).
/// 2. Anthropic: `claude-*` model names or `anthropic/` prefix.
/// 3. Azure: `azure/` prefix.
/// 4. Google AI Studio: `gemini/` or `google_ai/` prefix.
/// 5. Vertex AI: `vertex_ai/` prefix.
/// 6. AWS Bedrock: `bedrock/` prefix.
/// 7. `"provider/"` prefix — look up the prefix in the registry.
/// 8. Walk all registry entries and check their `model_prefixes`.
///
/// Returns `None` when no built-in provider matches.  The caller should fall
/// back to a config-specified `base_url` or default to [`OpenAiProvider`].
///
/// Complex providers (those listed in `complex_providers` in providers.json)
/// are excluded from config-driven routing because they require custom
/// auth/request logic beyond simple bearer tokens.
pub(crate) fn detect_provider(model: &str) -> Option<Box<dyn Provider>> {
    if let Some(provider) = custom::detect_custom_provider(model) {
        return Some(provider);
    }

    let openai = OpenAiProvider;
    if openai.matches_model(model) {
        return Some(Box::new(openai));
    }

    let anthropic = anthropic::AnthropicProvider;
    if anthropic.matches_model(model) {
        return Some(Box::new(anthropic));
    }

    if model.starts_with("azure/") {
        return Some(Box::new(azure::AzureProvider::new()));
    }

    if model.starts_with("gemini/") || model.starts_with("google_ai/") {
        return Some(Box::new(google_ai::GoogleAiProvider));
    }

    if model.starts_with("vertex_ai/") {
        return Some(Box::new(vertex::VertexAiProvider::from_env()));
    }

    if model.starts_with("bedrock/") {
        return Some(Box::new(bedrock::BedrockProvider::from_env()));
    }

    if model.starts_with("command-") || model.starts_with("cohere/") {
        return Some(Box::new(cohere::CohereProvider));
    }

    if model.starts_with("mistral-")
        || model.starts_with("codestral-")
        || model.starts_with("pixtral-")
        || model.starts_with("mistral/")
    {
        return Some(Box::new(mistral::MistralProvider));
    }

    if model.starts_with("github_copilot/") {
        return Some(Box::new(github_copilot::GithubCopilotProvider::from_env()));
    }

    let reg = match REGISTRY.as_ref() {
        Ok(r) => r,
        Err(_) => return None,
    };

    if let Some((prefix, _)) = model.split_once('/')
        && let Some(entry) = reg.providers.iter().find(|e| e.config.name == prefix)
        && entry.config.base_url.is_some()
        && !reg.complex_providers.contains(&entry.config.name)
    {
        return Some(Box::new(ConfigDrivenProvider::new(&entry.config)));
    }

    for entry in &reg.providers {
        if reg.complex_providers.contains(&entry.config.name) {
            continue;
        }
        if let Some(prefixes) = &entry.config.model_prefixes {
            let matches = prefixes
                .iter()
                .any(|p| model.starts_with(p.as_str()) && !p.ends_with('/'));
            if matches && entry.config.base_url.is_some() {
                return Some(Box::new(ConfigDrivenProvider::new(&entry.config)));
            }
        }
    }

    None
}

/// Return all provider configs from the registry.
///
/// Useful for tooling, documentation generation, or runtime enumeration.
/// Returns the public [`ProviderConfig`] slice (without capability flags).
/// To query capability flags for a specific provider use [`capabilities`].
pub fn all_providers() -> Result<&'static [ProviderConfig]> {
    Ok(&registry()?.configs)
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

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn minimax_regional_endpoints_are_registered() {
        let registry = registry().expect("registry should load");
        let provider = registry
            .providers
            .iter()
            .find(|entry| entry.config.name == "minimax")
            .expect("MiniMax provider should be registered");

        assert!(provider.capabilities.reasoning);
        assert_eq!(
            provider.regional_endpoints,
            vec![
                RegionalEndpointConfig {
                    region: "global_en".into(),
                    openai_base_url: "https://api.minimax.io/v1".into(),
                    anthropic_base_url: "https://api.minimax.io/anthropic".into(),
                    docs_root: "https://platform.minimax.io/docs".into(),
                },
                RegionalEndpointConfig {
                    region: "cn_zh".into(),
                    openai_base_url: "https://api.minimaxi.com/v1".into(),
                    anthropic_base_url: "https://api.minimaxi.com/anthropic".into(),
                    docs_root: "https://platform.minimaxi.com/docs".into(),
                },
            ]
        );
    }

    #[test]
    fn transform_response_audio_message_hoisted() {
        let mut body = json!({
            "id": "chatcmpl-audio-123",
            "object": "chat.completion",
            "created": 1700000000u64,
            "model": "gpt-4o-audio-preview",
            "choices": [{
                "index": 0,
                "message": {
                    "role": "assistant",
                    "content": null,
                    "audio": {
                        "id": "audio-abc",
                        "data": "aGVsbG8=",
                        "transcript": "hello",
                        "format": "wav",
                        "expires_at": 9999999999u64
                    }
                },
                "finish_reason": "stop"
            }]
        });

        transform_openai_audio_response(&mut body).expect("transform must succeed");

        let content = body
            .pointer("/choices/0/message/content")
            .expect("content must be present");
        assert!(content.is_array(), "content must be a parts array, got: {content}");
        let parts = content.as_array().expect("array");
        assert_eq!(parts.len(), 2);
        assert_eq!(parts[0]["type"], "text");
        assert_eq!(parts[0]["text"], "hello");
        assert_eq!(parts[1]["type"], "output_audio");
        assert_eq!(parts[1]["audio"]["data"], "aGVsbG8=");
        assert_eq!(parts[1]["audio"]["format"], "wav");
    }

    #[test]
    fn transform_response_audio_no_op_when_no_audio_field() {
        let mut body = json!({
            "choices": [{
                "message": {
                    "role": "assistant",
                    "content": "plain text"
                }
            }]
        });
        let original = body.clone();
        transform_openai_audio_response(&mut body).expect("transform must succeed");
        assert_eq!(body, original, "body must be unchanged when no audio field is present");
    }

    #[test]
    fn parse_stream_event_tolerates_trailing_metadata_event_without_id() {
        // ~keep OpenCode Zen/Go emits an `inference-cost` event (no `id`, empty
        // `choices`) right before `[DONE]`; the default parser must yield an empty
        // chunk rather than aborting the stream with `missing field 'id'` (#155).
        let payload = r#"{"choices":[],"x-opencode-type":"inference-cost","cost":"0.00001400","normalizedUsage":{"inputTokens":84,"outputTokens":8}}"#;
        let chunk = OpenAiProvider
            .parse_stream_event(payload)
            .expect("trailing metadata event must not error")
            .expect("event should decode to a chunk, not be skipped");
        assert!(chunk.id.is_empty());
        assert!(chunk.choices.is_empty());
    }

    #[test]
    fn parse_stream_event_parses_normal_chunk_with_id() {
        let payload = r#"{"id":"chatcmpl-1","object":"chat.completion.chunk","created":1700000000,"model":"gpt-4o","choices":[{"index":0,"delta":{"content":"hi"},"finish_reason":null}]}"#;
        let chunk = OpenAiProvider
            .parse_stream_event(payload)
            .expect("valid chunk must parse")
            .expect("valid chunk must yield Some");
        assert_eq!(chunk.id, "chatcmpl-1");
        assert_eq!(chunk.choices[0].delta.content.as_deref(), Some("hi"));
    }
}
