//! liter-lm PHP Bindings
//!
//! Exposes the liter-lm Rust core to PHP using `ext-php-rs`.
//!
//! # Architecture
//!
//! PHP is synchronous (single-threaded request model), so all async Rust
//! futures are driven to completion with `tokio::runtime::Runtime::block_on`.
//! A single Tokio runtime is created once per PHP worker process and reused
//! for the lifetime of that worker.
//!
//! All methods accept / return JSON strings to avoid the complexity of mapping
//! deeply nested Rust types to PHP objects.  PHP code decodes with
//! `json_decode`.
//!
//! # Example (PHP)
//!
//! ```php
//! <?php
//! $client = new \LiterLm\LlmClient('sk-...', 'https://api.openai.com/v1');
//!
//! $response = json_decode($client->chat(json_encode([
//!     'model'    => 'gpt-4',
//!     'messages' => [['role' => 'user', 'content' => 'Hello']],
//! ])), true);
//!
//! echo $response['choices'][0]['message']['content'];
//! ```

#![cfg_attr(windows, feature(abi_vectorcall))]

use ext_php_rs::prelude::*;
use liter_lm::{ClientConfigBuilder, DefaultClient, LlmClient};
use once_cell::sync::Lazy;

// ─── Tokio runtime ────────────────────────────────────────────────────────────

/// Shared Tokio runtime for blocking on async calls.
///
/// PHP workers are long-lived processes (FPM), so we create one runtime per
/// process and keep it alive.  Construction errors are stored as a string and
/// surfaced as PHP exceptions at call time rather than panicking at startup.
static RUNTIME: Lazy<Result<tokio::runtime::Runtime, String>> = Lazy::new(|| {
    tokio::runtime::Builder::new_multi_thread()
        .worker_threads(2)
        .enable_all()
        .thread_name("liter-lm-php")
        .build()
        .map_err(|e| format!("Failed to create Tokio runtime: {e}"))
});

/// Obtain a reference to the global runtime, or return a PHP exception.
fn runtime() -> PhpResult<&'static tokio::runtime::Runtime> {
    RUNTIME.as_ref().map_err(|e| PhpException::from(e.clone()))
}

// ─── LlmClient PHP class ──────────────────────────────────────────────────────

/// PHP class wrapping the liter-lm Rust client.
///
/// All request/response types use JSON strings so that PHP code can work with
/// standard `json_encode` / `json_decode` without needing custom PHP classes.
#[php_class]
#[php(name = "LiterLm\\LlmClient")]
pub struct PhpLlmClient {
    inner: DefaultClient,
}

#[php_impl]
impl PhpLlmClient {
    /// Create a new `LlmClient`.
    ///
    /// @param string      $apiKey      API key for authentication.
    /// @param string|null $baseUrl     Override provider base URL (optional).
    /// @param int         $maxRetries  Retries on 429 / 5xx.  Defaults to 3.
    /// @param int         $timeoutSecs Request timeout in seconds.  Defaults to 60.
    pub fn __construct(
        api_key: String,
        base_url: Option<String>,
        max_retries: Option<u32>,
        timeout_secs: Option<u64>,
    ) -> PhpResult<Self> {
        let mut builder = ClientConfigBuilder::new(api_key);

        if let Some(url) = base_url {
            builder = builder.base_url(url);
        }
        if let Some(retries) = max_retries {
            builder = builder.max_retries(retries);
        }
        if let Some(secs) = timeout_secs {
            builder = builder.timeout(std::time::Duration::from_secs(secs));
        }

        let config = builder.build();
        let client = DefaultClient::new(config, None).map_err(|e| PhpException::from(e.to_string()))?;

        Ok(Self { inner: client })
    }

    /// Send a chat completion request.
    ///
    /// @param string $requestJson JSON-encoded OpenAI-compatible chat request.
    /// @return string JSON-encoded chat completion response.
    pub fn chat(&self, request_json: String) -> PhpResult<String> {
        let req: liter_lm::ChatCompletionRequest = serde_json::from_str(&request_json)
            .map_err(|e| PhpException::from(format!("invalid chat request JSON: {e}")))?;

        let rt = runtime()?;
        let response = rt
            .block_on(self.inner.chat(req))
            .map_err(|e| PhpException::from(e.to_string()))?;

        serde_json::to_string(&response).map_err(|e| PhpException::from(format!("serialization error: {e}")))
    }

    /// Send an embedding request.
    ///
    /// @param string $requestJson JSON-encoded OpenAI-compatible embeddings request.
    /// @return string JSON-encoded embedding response.
    pub fn embed(&self, request_json: String) -> PhpResult<String> {
        let req: liter_lm::EmbeddingRequest = serde_json::from_str(&request_json)
            .map_err(|e| PhpException::from(format!("invalid embed request JSON: {e}")))?;

        let rt = runtime()?;
        let response = rt
            .block_on(self.inner.embed(req))
            .map_err(|e| PhpException::from(e.to_string()))?;

        serde_json::to_string(&response).map_err(|e| PhpException::from(format!("serialization error: {e}")))
    }

    /// List available models from the provider.
    ///
    /// @return string JSON-encoded models list response.
    #[php(name = "listModels")]
    pub fn list_models(&self) -> PhpResult<String> {
        let rt = runtime()?;
        let response = rt
            .block_on(self.inner.list_models())
            .map_err(|e| PhpException::from(e.to_string()))?;

        serde_json::to_string(&response).map_err(|e| PhpException::from(format!("serialization error: {e}")))
    }
}

// ─── Module-level function ────────────────────────────────────────────────────

/// Returns the version of the liter-lm library.
///
/// @return string Semver version string (e.g., "0.1.0").
#[php_function]
pub fn liter_lm_version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

// ─── Module registration ──────────────────────────────────────────────────────

/// Register the `liter_lm` PHP extension module.
#[php_module]
pub fn get_module(module: ModuleBuilder) -> ModuleBuilder {
    module
        .function(wrap_function!(liter_lm_version))
        .class::<PhpLlmClient>()
}
