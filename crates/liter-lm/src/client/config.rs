use std::time::Duration;

use secrecy::SecretString;

use crate::error::{LiterLmError, Result};

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
        f.debug_struct("ClientConfig")
            .field("api_key", &"[redacted]")
            .field("base_url", &self.base_url)
            .field("timeout", &self.timeout)
            .field("max_retries", &self.max_retries)
            .field("extra_headers", &redacted_headers)
            .finish()
    }
}

/// Builder for [`ClientConfig`].
///
/// Construct with [`ClientConfigBuilder::new`] and call builder methods to
/// customise the configuration, then call [`ClientConfigBuilder::build`] to
/// obtain a [`ClientConfig`].
#[must_use]
pub struct ClientConfigBuilder {
    config: ClientConfig,
}

impl ClientConfigBuilder {
    /// Create a new builder with the given API key and sensible defaults.
    pub fn new(api_key: impl Into<String>) -> Self {
        Self {
            config: ClientConfig::new(api_key),
        }
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

    /// Add a custom header sent on every request.
    ///
    /// Returns an error if either `key` or `value` is not a valid HTTP header
    /// name / value.
    ///
    /// This method is only available when the `native-http` feature is enabled
    /// because header validation relies on `reqwest`'s header types.
    #[cfg(feature = "native-http")]
    pub fn header(mut self, key: impl Into<String>, value: impl Into<String>) -> Result<Self> {
        let key = key.into();
        let value = value.into();

        // Validate header name.
        reqwest::header::HeaderName::from_bytes(key.as_bytes()).map_err(|e| LiterLmError::InvalidHeader {
            name: key.clone(),
            reason: e.to_string(),
        })?;

        // Validate header value.
        reqwest::header::HeaderValue::from_str(&value).map_err(|e| LiterLmError::InvalidHeader {
            name: key.clone(),
            reason: e.to_string(),
        })?;

        self.config.extra_headers.push((key, value));
        Ok(self)
    }

    /// Consume the builder and return the completed [`ClientConfig`].
    #[must_use]
    pub fn build(self) -> ClientConfig {
        self.config
    }
}
