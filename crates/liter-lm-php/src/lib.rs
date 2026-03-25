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

// ─── Tokio runtime ────────────────────────────────────────────────────────────

/// Shared Tokio runtime for blocking on async calls.
///
/// PHP workers are long-lived processes (FPM), so we create one runtime per
/// process and keep it alive.  A `current_thread` runtime is sufficient
/// because PHP's concurrency model is single-threaded per worker — there is
/// no benefit to a thread pool here, and `current_thread` avoids spawning
/// extra OS threads.
///
/// Construction errors are stored as a string and surfaced as PHP exceptions
/// at call time rather than panicking at startup.
///
/// ## Guard against calling from within an existing Tokio runtime
///
/// `block_on` panics if called from within an async context.  Although PHP
/// itself is synchronous, ext-php-rs could theoretically be embedded in a
/// context where a runtime is already active.  The `runtime()` helper checks
/// for this case and prefers `Handle::block_on` when a handle is available.
static RUNTIME: std::sync::LazyLock<Result<tokio::runtime::Runtime, String>> = std::sync::LazyLock::new(|| {
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .thread_name("liter-lm-php")
        .build()
        .map_err(|e| format!("Failed to create Tokio runtime: {e}"))
});

/// Obtain a reference to the global runtime, or return a PHP exception.
///
/// If a Tokio runtime is already active on the current thread (e.g. when
/// this code is called from within an async test harness), this helper reuses
/// the existing runtime handle via `Handle::block_on` rather than creating a
/// nested runtime, which would panic.
fn block_on_future<F, T>(future: F) -> PhpResult<T>
where
    F: std::future::Future<Output = T>,
{
    // Fast path: if we're already inside a Tokio runtime, reuse its handle.
    if let Ok(handle) = tokio::runtime::Handle::try_current() {
        // `block_in_place` yields the current OS thread to Tokio while
        // blocking on the future, avoiding a nested-runtime panic.
        return Ok(tokio::task::block_in_place(|| handle.block_on(future)));
    }

    // Normal path: drive the future on our own runtime.
    let rt = RUNTIME.as_ref().map_err(|e| PhpException::from(e.clone()))?;
    Ok(rt.block_on(future))
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
    /// @param string|null $modelHint   Model hint for provider auto-detection
    ///                                 (e.g. `"groq/llama3-70b"`).  Used when
    ///                                 $baseUrl is null.
    /// @param int         $maxRetries  Retries on 429 / 5xx.  Defaults to 3.
    /// @param int         $timeoutSecs Request timeout in seconds.  Defaults to 60.
    pub fn __construct(
        api_key: String,
        base_url: Option<String>,
        model_hint: Option<String>,
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
        let client =
            DefaultClient::new(config, model_hint.as_deref()).map_err(|e| PhpException::from(e.to_string()))?;

        Ok(Self { inner: client })
    }

    /// Send a chat completion request.
    ///
    /// @param string $requestJson JSON-encoded OpenAI-compatible chat request.
    /// @return string JSON-encoded chat completion response.
    pub fn chat(&self, request_json: String) -> PhpResult<String> {
        let req: liter_lm::ChatCompletionRequest = serde_json::from_str(&request_json)
            .map_err(|e| PhpException::from(format!("invalid chat request JSON: {e}")))?;

        let response = block_on_future(self.inner.chat(req))?.map_err(|e| PhpException::from(e.to_string()))?;

        serde_json::to_string(&response).map_err(|e| PhpException::from(format!("serialization error: {e}")))
    }

    /// Send a streaming chat completion request and collect all chunks.
    ///
    /// **Limitation:** PHP's synchronous execution model does not support true
    /// incremental streaming.  This method drives the full SSE stream to
    /// completion on the Rust side and returns all chunks as a JSON array.
    /// For real-time token-by-token output, consider the Node.js or Python
    /// bindings which support async iterators.
    ///
    /// @param string $requestJson JSON-encoded OpenAI-compatible chat request.
    ///                            The `"stream"` field is forced to `true`.
    /// @return string JSON-encoded array of `ChatCompletionChunk` objects.
    #[php(name = "chatStream")]
    pub fn chat_stream(&self, request_json: String) -> PhpResult<String> {
        use futures_core::Stream as FStream;
        use std::pin::Pin;

        let mut req: liter_lm::ChatCompletionRequest = serde_json::from_str(&request_json)
            .map_err(|e| PhpException::from(format!("invalid chat stream request JSON: {e}")))?;

        // Force streaming flag.
        req.stream = Some(true);

        // Collect all SSE chunks by blocking on the async stream.
        // Returns a PhpResult<Vec<_>> so we can propagate errors and then
        // serialise the collected chunks outside the async block.
        let items: Vec<liter_lm::ChatCompletionChunk> = block_on_future(async {
            let stream = self
                .inner
                .chat_stream(req)
                .await
                .map_err(|e| PhpException::from(e.to_string()))?;

            let mut collected: Vec<liter_lm::ChatCompletionChunk> = Vec::new();
            let mut pinned: Pin<Box<_>> = stream;
            loop {
                let next = std::future::poll_fn(|cx| FStream::poll_next(pinned.as_mut(), cx)).await;
                match next {
                    Some(Ok(chunk)) => collected.push(chunk),
                    Some(Err(e)) => return Err(PhpException::from(e.to_string())),
                    None => break,
                }
            }
            Ok(collected)
        })??;

        serde_json::to_string(&items).map_err(|e| PhpException::from(format!("serialization error: {e}")))
    }

    /// Send an embedding request.
    ///
    /// @param string $requestJson JSON-encoded OpenAI-compatible embeddings request.
    /// @return string JSON-encoded embedding response.
    pub fn embed(&self, request_json: String) -> PhpResult<String> {
        let req: liter_lm::EmbeddingRequest = serde_json::from_str(&request_json)
            .map_err(|e| PhpException::from(format!("invalid embed request JSON: {e}")))?;

        let response = block_on_future(self.inner.embed(req))?.map_err(|e| PhpException::from(e.to_string()))?;

        serde_json::to_string(&response).map_err(|e| PhpException::from(format!("serialization error: {e}")))
    }

    /// List available models from the provider.
    ///
    /// @return string JSON-encoded models list response.
    #[php(name = "listModels")]
    pub fn list_models(&self) -> PhpResult<String> {
        let response = block_on_future(self.inner.list_models())?.map_err(|e| PhpException::from(e.to_string()))?;

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
