use std::sync::Arc;
use std::time::Duration;

use secrecy::SecretString;

use crate::auth::CredentialProvider;
#[cfg(any(feature = "native-http", feature = "wasm-http"))]
use crate::error::{LiterLlmError, Result};
#[cfg(feature = "tower")]
use crate::tower::{BudgetConfig, CacheConfig, CacheStore, LlmHook, RateLimitConfig};

/// Configuration for an LLM client.
///
/// `api_key` is stored as a [`SecretString`] so it is zeroed on drop and never
/// printed accidentally.  Access it via [`secrecy::ExposeSecret`].
#[derive(Clone)]
pub struct ClientConfig {
    /// API key for authentication (stored as a secret).
    pub api_key: SecretString,
    /// Override base URL.  When set, all requests go here regardless of model
    /// name, and provider auto-detection is skipped.
    pub base_url: Option<String>,
    /// Request timeout.
    pub timeout: Duration,
    /// Maximum number of retries on 429 / 5xx responses.
    pub max_retries: u32,
    /// Extra headers sent on every request.
    ///
    /// Use `Vec<(String, String)>` rather than `HashMap` to preserve insertion
    /// order and avoid non-deterministic iteration when building the reqwest
    /// `HeaderMap`.  Access via [`ClientConfig::headers`]; do not mutate
    /// directly from outside this crate.
    pub(crate) extra_headers: Vec<(String, String)>,
    /// Optional dynamic credential provider for token-based auth
    /// (Azure AD, Vertex OAuth2) or refreshable credentials (AWS STS).
    ///
    /// When set, the client calls `resolve()` before each request to obtain
    /// a fresh credential.  When `None`, the static `api_key` is used.
    pub credential_provider: Option<Arc<dyn CredentialProvider>>,

    /// Configuration for the response cache Tower middleware layer.
    ///
    /// When set, bindings and advanced Rust users can use this to construct
    /// a [`CacheLayer`](crate::tower::CacheLayer) in their Tower stack.
    #[cfg(feature = "tower")]
    pub cache_config: Option<CacheConfig>,

    /// Custom cache store backend for the cache Tower middleware layer.
    ///
    /// When set alongside `cache_config`, the cache layer will use this
    /// store instead of the default in-memory LRU.
    #[cfg(feature = "tower")]
    pub cache_store: Option<Arc<dyn CacheStore>>,

    /// Configuration for the budget enforcement Tower middleware layer.
    ///
    /// When set, bindings and advanced Rust users can use this to construct
    /// a [`BudgetLayer`](crate::tower::BudgetLayer) in their Tower stack.
    #[cfg(feature = "tower")]
    pub budget_config: Option<BudgetConfig>,

    /// User-defined hooks for the hooks Tower middleware layer.
    ///
    /// These hooks are invoked at request lifecycle points (pre-request,
    /// post-response, on-error) when a
    /// [`HooksLayer`](crate::tower::HooksLayer) is constructed from this
    /// config.
    #[cfg(feature = "tower")]
    pub hooks: Vec<Arc<dyn LlmHook>>,

    /// Cooldown duration after transient errors (rate limit, timeout, server error).
    /// When set, the client rejects requests with `ServiceUnavailable` during cooldown.
    #[cfg(feature = "tower")]
    pub cooldown_duration: Option<Duration>,

    /// Per-model rate limiting configuration (RPM/TPM).
    #[cfg(feature = "tower")]
    pub rate_limit_config: Option<RateLimitConfig>,

    /// Background health check interval. When set, periodically probes the provider
    /// and rejects requests when the provider is unhealthy.
    #[cfg(feature = "tower")]
    pub health_check_interval: Option<Duration>,

    /// Enable per-request cost tracking. Costs are accumulated atomically and
    /// logged via `tracing::info`.
    #[cfg(feature = "tower")]
    pub enable_cost_tracking: bool,

    /// Enable OpenTelemetry-compatible tracing spans for every request.
    #[cfg(feature = "tower")]
    pub enable_tracing: bool,

    /// Automatically load the API key from the provider's environment variable
    /// when no explicit key is provided.
    ///
    /// When `true` (the default) and `api_key` is empty, [`DefaultClient::new`]
    /// reads the provider's designated environment variable (e.g.
    /// `OPENAI_API_KEY` for OpenAI).  Set to `false` to suppress this behaviour
    /// and require the caller to supply the key explicitly.
    ///
    /// Has no effect on WASM targets, where `std::env::var` is unavailable.
    pub load_env: bool,
}

impl ClientConfig {
    /// Create a config with the given API key and sensible defaults.
    pub fn new(api_key: impl Into<String>) -> Self {
        Self {
            api_key: SecretString::from(api_key.into()),
            base_url: None,
            timeout: Duration::from_secs(60),
            max_retries: 3,
            extra_headers: Vec::new(),
            credential_provider: None,
            load_env: true,
            #[cfg(feature = "tower")]
            cache_config: None,
            #[cfg(feature = "tower")]
            cache_store: None,
            #[cfg(feature = "tower")]
            budget_config: None,
            #[cfg(feature = "tower")]
            hooks: Vec::new(),
            #[cfg(feature = "tower")]
            cooldown_duration: None,
            #[cfg(feature = "tower")]
            rate_limit_config: None,
            #[cfg(feature = "tower")]
            health_check_interval: None,
            #[cfg(feature = "tower")]
            enable_cost_tracking: false,
            #[cfg(feature = "tower")]
            enable_tracing: false,
        }
    }

    /// Return the extra headers as an ordered slice of `(name, value)` pairs.
    pub fn headers(&self) -> &[(String, String)] {
        &self.extra_headers
    }
}

/// Note: intentionally does *not* implement `Debug` so the secret key is never
/// accidentally logged via `{:?}`.
impl std::fmt::Debug for ClientConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Redact all header values — they may contain API keys or secrets.
        let redacted_headers: Vec<(&str, &str)> = self
            .extra_headers
            .iter()
            .map(|(k, _v)| (k.as_str(), "[redacted]"))
            .collect();
        let mut dbg = f.debug_struct("ClientConfig");
        dbg.field("api_key", &"[redacted]")
            .field("base_url", &self.base_url)
            .field("timeout", &self.timeout)
            .field("max_retries", &self.max_retries)
            .field("extra_headers", &redacted_headers)
            .field("load_env", &self.load_env)
            .field(
                "credential_provider",
                &self.credential_provider.as_ref().map(|_| "[configured]"),
            );

        #[cfg(feature = "tower")]
        {
            dbg.field("cache_config", &self.cache_config)
                .field("cache_store", &self.cache_store.as_ref().map(|_| "[configured]"))
                .field("budget_config", &self.budget_config)
                .field("hooks_count", &self.hooks.len())
                .field("cooldown_duration", &self.cooldown_duration)
                .field("rate_limit_config", &self.rate_limit_config)
                .field("health_check_interval", &self.health_check_interval)
                .field("enable_cost_tracking", &self.enable_cost_tracking)
                .field("enable_tracing", &self.enable_tracing);
        }

        dbg.finish()
    }
}

/// Builder for [`ClientConfig`].
///
/// Construct with [`ClientConfigBuilder::new`] and call builder methods to
/// customise the configuration, then call [`ClientConfigBuilder::build`] to
/// obtain a [`ClientConfig`].
#[must_use]
pub struct ClientConfigBuilder {
    pub(crate) config: ClientConfig,
}

impl ClientConfigBuilder {
    /// Create a new builder with the given API key and sensible defaults.
    pub fn new(api_key: impl Into<String>) -> Self {
        Self {
            config: ClientConfig::new(api_key),
        }
    }

    /// Create a builder with no explicit API key.
    ///
    /// `load_env` is `true` by default, so the key will be read from the
    /// provider's environment variable (e.g. `OPENAI_API_KEY`) at client
    /// construction time.  Call `.load_env(false)` to opt out.
    pub fn from_env() -> Self {
        Self {
            config: ClientConfig::new(""),
        }
    }

    /// Enable or disable automatic API key loading from environment variables.
    ///
    /// When `true` (the default) and no explicit `api_key` was provided,
    /// [`DefaultClient::new`] reads the provider's designated environment
    /// variable.  Set to `false` to require an explicit key.
    ///
    /// Has no effect on WASM targets.
    pub fn load_env(mut self, enabled: bool) -> Self {
        self.config.load_env = enabled;
        self
    }

    /// Override the provider base URL for all requests.
    pub fn base_url(mut self, url: impl Into<String>) -> Self {
        self.config.base_url = Some(url.into());
        self
    }

    /// Set the per-request timeout (default: 60 s).
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.config.timeout = timeout;
        self
    }

    /// Set the maximum number of retries on 429 / 5xx responses (default: 3).
    pub fn max_retries(mut self, retries: u32) -> Self {
        self.config.max_retries = retries;
        self
    }

    /// Set a dynamic credential provider for token-based or refreshable auth.
    ///
    /// When configured, the client calls `resolve()` before each request
    /// instead of using the static `api_key` for authentication.
    pub fn credential_provider(mut self, provider: Arc<dyn CredentialProvider>) -> Self {
        self.config.credential_provider = Some(provider);
        self
    }

    /// Add a custom header sent on every request.
    ///
    /// Returns an error if either `key` or `value` is not a valid HTTP header
    /// name / value.
    ///
    /// This method is only available when the `native-http` feature is enabled
    /// because header validation relies on `reqwest`'s header types.
    #[cfg(any(feature = "native-http", feature = "wasm-http"))]
    pub fn header(mut self, key: impl Into<String>, value: impl Into<String>) -> Result<Self> {
        let key = key.into();
        let value = value.into();

        // Validate header name.
        reqwest::header::HeaderName::from_bytes(key.as_bytes()).map_err(|e| LiterLlmError::InvalidHeader {
            name: key.clone(),
            reason: e.to_string(),
        })?;

        // Validate header value.
        reqwest::header::HeaderValue::from_str(&value).map_err(|e| LiterLlmError::InvalidHeader {
            name: key.clone(),
            reason: e.to_string(),
        })?;

        self.config.extra_headers.push((key, value));
        Ok(self)
    }

    /// Set the response cache configuration for the Tower middleware stack.
    ///
    /// When set, bindings and advanced Rust users can read this from the
    /// built [`ClientConfig`] to construct a
    /// [`CacheLayer`](crate::tower::CacheLayer).
    #[cfg(feature = "tower")]
    pub fn cache(mut self, config: CacheConfig) -> Self {
        self.config.cache_config = Some(config);
        self
    }

    /// Set a custom cache store backend for the Tower cache middleware.
    ///
    /// When set alongside [`cache`](Self::cache), the cache layer will use
    /// this store instead of the default in-memory LRU.
    #[cfg(feature = "tower")]
    pub fn cache_store(mut self, store: Arc<dyn CacheStore>) -> Self {
        self.config.cache_store = Some(store);
        self
    }

    /// Set the budget enforcement configuration for the Tower middleware stack.
    ///
    /// When set, bindings and advanced Rust users can read this from the
    /// built [`ClientConfig`] to construct a
    /// [`BudgetLayer`](crate::tower::BudgetLayer).
    #[cfg(feature = "tower")]
    pub fn budget(mut self, config: BudgetConfig) -> Self {
        self.config.budget_config = Some(config);
        self
    }

    /// Add a single hook to the Tower hooks middleware stack.
    ///
    /// Hooks are invoked sequentially in registration order at request
    /// lifecycle points (pre-request, post-response, on-error).
    #[cfg(feature = "tower")]
    pub fn hook(mut self, hook: Arc<dyn LlmHook>) -> Self {
        self.config.hooks.push(hook);
        self
    }

    /// Set the full list of hooks for the Tower hooks middleware stack,
    /// replacing any previously registered hooks.
    ///
    /// Hooks are invoked sequentially in registration order.
    #[cfg(feature = "tower")]
    pub fn hooks(mut self, hooks: Vec<Arc<dyn LlmHook>>) -> Self {
        self.config.hooks = hooks;
        self
    }

    /// Set the cooldown duration after transient errors.
    ///
    /// When set, the client rejects requests with `ServiceUnavailable` for
    /// the given duration after a transient error (rate limit, timeout,
    /// server error).
    #[cfg(feature = "tower")]
    pub fn cooldown(mut self, duration: Duration) -> Self {
        self.config.cooldown_duration = Some(duration);
        self
    }

    /// Set per-model rate limiting configuration.
    ///
    /// When set, requests exceeding the configured RPM or TPM limits are
    /// rejected with [`LiterLlmError::RateLimited`](crate::error::LiterLlmError::RateLimited).
    #[cfg(feature = "tower")]
    pub fn rate_limit(mut self, config: RateLimitConfig) -> Self {
        self.config.rate_limit_config = Some(config);
        self
    }

    /// Set the background health check interval.
    ///
    /// When set, the client periodically probes the provider and rejects
    /// requests when the provider is unhealthy.
    #[cfg(feature = "tower")]
    pub fn health_check(mut self, interval: Duration) -> Self {
        self.config.health_check_interval = Some(interval);
        self
    }

    /// Enable or disable per-request cost tracking.
    ///
    /// When enabled, estimated USD cost is recorded on the current tracing
    /// span as `gen_ai.usage.cost`.
    #[cfg(feature = "tower")]
    pub fn cost_tracking(mut self, enabled: bool) -> Self {
        self.config.enable_cost_tracking = enabled;
        self
    }

    /// Enable or disable OpenTelemetry-compatible tracing spans.
    ///
    /// When enabled, every request is wrapped in a `gen_ai` tracing span
    /// with semantic convention attributes.
    #[cfg(feature = "tower")]
    pub fn tracing(mut self, enabled: bool) -> Self {
        self.config.enable_tracing = enabled;
        self
    }

    /// Consume the builder and return the completed [`ClientConfig`].
    #[must_use]
    pub fn build(self) -> ClientConfig {
        self.config
    }
}
