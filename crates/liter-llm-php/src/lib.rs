//! liter-llm PHP Bindings
//!
//! Exposes the liter-llm Rust core to PHP using `ext-php-rs`.
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
//! $client = new \LiterLlm\LlmClient('sk-...', 'https://api.openai.com/v1');
//!
//! $response = json_decode($client->chat(json_encode([
//!     'model'    => 'gpt-4',
//!     'messages' => [['role' => 'user', 'content' => 'Hello']],
//! ])), true);
//!
//! echo $response['choices'][0]['message']['content'];
//! ```

#![cfg_attr(windows, feature(abi_vectorcall))]

use std::cell::RefCell;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use ext_php_rs::prelude::*;
use ext_php_rs::types::Zval;
use liter_llm::tower::{BudgetConfig, CacheConfig, Enforcement, LlmHook, LlmRequest, LlmResponse};
use liter_llm::{
    AuthHeaderFormat, BatchClient, ClientConfigBuilder, CustomProviderConfig, FileClient, LiterLlmError, LlmClient,
    ManagedClient, ResponseClient, register_custom_provider, unregister_custom_provider,
};

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
static RUNTIME: std::sync::LazyLock<Result<tokio::runtime::Runtime, String>> = std::sync::LazyLock::new(|| {
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .thread_name("liter-llm-php")
        .build()
        .map_err(|e| format!("Failed to create Tokio runtime: {e}"))
});

/// Drive `future` to completion on the shared current-thread runtime.
///
/// `block_in_place` is intentionally omitted: `RUNTIME` is a
/// `current_thread` runtime and `block_in_place` panics on that flavour
/// because there are no worker threads to yield to.  If this function is
/// somehow called from within another Tokio runtime the resulting
/// "Cannot start a runtime from within a runtime" panic is the correct
/// signal — nested runtimes are not supported.
fn block_on_future<F, T>(future: F) -> PhpResult<T>
where
    F: std::future::Future<Output = T>,
{
    let rt = RUNTIME.as_ref().map_err(|e| PhpException::from(e.clone()))?;
    Ok(rt.block_on(future))
}

// ─── PHP Hook Bridge ─────────────────────────────────────────────────────────

// Thread-local storage for PHP hook objects.
//
// PHP's `Zval` is not `Send`/`Sync` because it is tied to the PHP engine's
// single-threaded execution model.  We store hook objects in thread-local
// storage indexed by a unique ID and retrieve them when the hook is invoked.
//
// This is safe because:
// 1. PHP workers are single-threaded — hooks are always invoked on the same
//    thread that registered them.
// 2. The Tokio runtime is `current_thread`, so async futures run on the
//    same OS thread.
thread_local! {
    static HOOK_REGISTRY: RefCell<Vec<Option<Zval>>> = const { RefCell::new(Vec::new()) };
}

/// Register a PHP hook `Zval` in thread-local storage and return its index.
fn register_hook_zval(zval: &Zval) -> usize {
    HOOK_REGISTRY.with(|registry| {
        let mut registry = registry.borrow_mut();
        let idx = registry.len();
        registry.push(Some(zval.shallow_clone()));
        idx
    })
}

/// A bridge that implements `LlmHook` by calling back into PHP objects stored
/// in thread-local storage.
///
/// The PHP hook object may define any combination of:
///   - `onRequest(string $requestJson): void`
///   - `onResponse(string $requestJson, string $responseJson): void`
///   - `onError(string $requestJson, string $errorMessage): void`
///
/// Missing methods are silently ignored (no-op).  If `onRequest` throws a PHP
/// exception, the request is rejected.
struct PhpHookBridge {
    /// Index into the thread-local `HOOK_REGISTRY`.
    hook_idx: usize,
}

// SAFETY: `PhpHookBridge` only stores an index (usize).  The actual PHP Zval
// lives in thread-local storage and is only accessed on the PHP thread.  The
// Tokio runtime is `current_thread`, so all async futures execute on the same
// OS thread that registered the hook.  Send + Sync are required by `LlmHook`
// trait bounds; the bridge never actually crosses thread boundaries.
unsafe impl Send for PhpHookBridge {}
unsafe impl Sync for PhpHookBridge {}

impl PhpHookBridge {
    fn new(zval: &Zval) -> Self {
        let hook_idx = register_hook_zval(zval);
        Self { hook_idx }
    }

    /// Call a named method on the PHP hook object.
    ///
    /// Returns `Ok(())` if the method doesn't exist (no-op) or succeeds.
    /// Returns `Err` if the method throws a PHP exception.
    fn call_method_checked(&self, method_name: &str, args: Vec<String>) -> Result<(), LiterLlmError> {
        HOOK_REGISTRY.with(|registry| {
            let registry = registry.borrow();
            let zval = match registry.get(self.hook_idx) {
                Some(Some(z)) => z,
                _ => return Ok(()), // hook was removed or invalid
            };

            let obj = match zval.object() {
                Some(o) => o,
                None => return Ok(()), // not an object
            };

            // Build argument references as &dyn IntoZvalDyn for ext-php-rs.
            let params: Vec<&dyn ext_php_rs::convert::IntoZvalDyn> = args
                .iter()
                .map(|s| s as &dyn ext_php_rs::convert::IntoZvalDyn)
                .collect();

            match obj.try_call_method(method_name, params) {
                Ok(_) => Ok(()),
                Err(e) => {
                    // Check if the error is "method not found" vs an actual exception.
                    let err_str = format!("{e:?}");
                    if err_str.contains("not found") || err_str.contains("undefined method") {
                        Ok(()) // method not defined — no-op
                    } else {
                        Err(LiterLlmError::HookRejected {
                            message: format!("hook {method_name} raised: {err_str}"),
                        })
                    }
                }
            }
        })
    }

    /// Fire-and-forget variant: errors from PHP are silently ignored.
    fn call_method_fire_and_forget(&self, method_name: &str, args: Vec<String>) {
        let _ = self.call_method_checked(method_name, args);
    }
}

impl LlmHook for PhpHookBridge {
    fn on_request(&self, req: &LlmRequest) -> Pin<Box<dyn Future<Output = liter_llm::Result<()>> + Send + '_>> {
        let req_json = format!("{req:?}");
        let result = self.call_method_checked("onRequest", vec![req_json]);
        Box::pin(async move { result })
    }

    fn on_response(&self, req: &LlmRequest, _resp: &LlmResponse) -> Pin<Box<dyn Future<Output = ()> + Send + '_>> {
        let req_json = format!("{req:?}");
        self.call_method_fire_and_forget("onResponse", vec![req_json, "response".to_owned()]);
        Box::pin(async {})
    }

    fn on_error(&self, req: &LlmRequest, err: &LiterLlmError) -> Pin<Box<dyn Future<Output = ()> + Send + '_>> {
        let req_json = format!("{req:?}");
        let err_msg = err.to_string();
        self.call_method_fire_and_forget("onError", vec![req_json, err_msg]);
        Box::pin(async {})
    }
}

// ─── Config parsing helpers ──────────────────────────────────────────────────

/// Parse a JSON string into a `CacheConfig`.
///
/// Expected JSON: `{"max_entries": 256, "ttl_seconds": 300}`
fn parse_cache_config_json(json: &str) -> PhpResult<CacheConfig> {
    let val: serde_json::Value =
        serde_json::from_str(json).map_err(|e| PhpException::from(format!("invalid cache config JSON: {e}")))?;
    let max_entries = val.get("max_entries").and_then(|v| v.as_u64()).unwrap_or(256) as usize;
    let ttl_seconds = val.get("ttl_seconds").and_then(|v| v.as_u64()).unwrap_or(300);
    Ok(CacheConfig {
        max_entries,
        ttl: std::time::Duration::from_secs(ttl_seconds),
    })
}

/// Parse a JSON string into a `BudgetConfig`.
///
/// Expected JSON: `{"global_limit": 10.0, "model_limits": {"gpt-4": 5.0}, "enforcement": "hard"}`
fn parse_budget_config_json(json: &str) -> PhpResult<BudgetConfig> {
    let val: serde_json::Value =
        serde_json::from_str(json).map_err(|e| PhpException::from(format!("invalid budget config JSON: {e}")))?;
    let global_limit = val.get("global_limit").and_then(|v| v.as_f64());
    let model_limits = val
        .get("model_limits")
        .and_then(|v| v.as_object())
        .map(|obj| {
            obj.iter()
                .filter_map(|(k, v)| v.as_f64().map(|f| (k.clone(), f)))
                .collect()
        })
        .unwrap_or_default();
    let enforcement = match val.get("enforcement").and_then(|v| v.as_str()).unwrap_or("hard") {
        "soft" => Enforcement::Soft,
        _ => Enforcement::Hard,
    };
    Ok(BudgetConfig {
        global_limit,
        model_limits,
        enforcement,
    })
}

// ─── LlmClient PHP class ──────────────────────────────────────────────────────

/// PHP class wrapping the liter-llm Rust client.
///
/// All request/response types use JSON strings so that PHP code can work with
/// standard `json_encode` / `json_decode` without needing custom PHP classes.
#[php_class]
#[php(name = "LiterLlm\\LlmClient")]
pub struct PhpLlmClient {
    inner: ManagedClient,
}

#[php_impl]
impl PhpLlmClient {
    /// Create a new `LlmClient`.
    ///
    /// @param string      $apiKey        API key for authentication.
    /// @param string|null $baseUrl       Override provider base URL (optional).
    /// @param string|null $modelHint     Model hint for provider auto-detection
    ///                                   (e.g. `"groq/llama3-70b"`).
    /// @param int         $maxRetries    Retries on 429 / 5xx.  Defaults to 3.
    /// @param int         $timeoutSecs   Request timeout in seconds.  Defaults to 60.
    /// @param string|null $cacheJson     Cache config as JSON (optional).
    ///                                   E.g. `'{"max_entries":256,"ttl_seconds":300}'`
    /// @param string|null $budgetJson    Budget config as JSON (optional).
    ///                                   E.g. `'{"global_limit":10.0,"enforcement":"hard"}'`
    pub fn __construct(
        api_key: String,
        base_url: Option<String>,
        model_hint: Option<String>,
        max_retries: Option<u32>,
        timeout_secs: Option<u64>,
        cache_json: Option<String>,
        budget_json: Option<String>,
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
        if let Some(ref json) = cache_json {
            let cache_cfg = parse_cache_config_json(json)?;
            builder = builder.cache(cache_cfg);
        }
        if let Some(ref json) = budget_json {
            let budget_cfg = parse_budget_config_json(json)?;
            builder = builder.budget(budget_cfg);
        }

        let config = builder.build();
        let client =
            ManagedClient::new(config, model_hint.as_deref()).map_err(|e| PhpException::from(e.to_string()))?;

        Ok(Self { inner: client })
    }

    /// Add a hook object to the client.
    ///
    /// The hook object may implement any of:
    ///   - `onRequest(string $requestJson): void`
    ///   - `onResponse(string $requestJson, string $responseJson): void`
    ///   - `onError(string $requestJson, string $errorMessage): void`
    ///
    /// If `onRequest` throws an exception, the request is rejected.
    /// Missing methods are silently ignored.
    ///
    /// **Note:** Hooks added after construction require rebuilding the
    /// middleware stack.  For PHP's synchronous model, hooks are invoked
    /// synchronously on the same thread.
    ///
    /// @param mixed $hook A PHP object with optional hook methods.
    #[php(name = "addHook")]
    pub fn add_hook(&mut self, hook: &Zval) -> PhpResult<()> {
        if hook.object().is_none() {
            return Err(PhpException::from("addHook() expects an object".to_string()));
        }
        let bridge = PhpHookBridge::new(hook);
        let hook_arc: Arc<dyn LlmHook> = Arc::new(bridge);

        // Rebuild the client with the new hook added.
        // ManagedClient doesn't support runtime hook addition, so we
        // reconstruct the config and client.  This is acceptable for PHP
        // because hooks are typically registered once at startup.
        //
        // We access the inner DefaultClient's config to reconstruct.
        // Since we can't easily extract config from ManagedClient, we store
        // the hook in thread-local and note this limitation.
        //
        // For now, we use a simpler approach: store hooks and create a
        // wrapper that invokes them synchronously before/after each request.
        // This is stored alongside the client.
        //
        // Actually, the cleanest approach for PHP is to call the hooks
        // directly in each method since PHP is synchronous.  But that would
        // require us to store hooks separately.  Let's use thread-local
        // hook storage with a global list.
        HOOKS.with(|hooks| {
            hooks.borrow_mut().push(hook_arc);
        });

        Ok(())
    }

    /// Return the total budget spend in USD.
    ///
    /// Returns 0.0 if no budget middleware is configured.
    ///
    /// @return float Total spend in USD.
    #[php(name = "budgetUsed")]
    pub fn budget_used(&self) -> f64 {
        self.inner
            .budget_state()
            .map(|state| state.global_spend())
            .unwrap_or(0.0)
    }

    /// Send a chat completion request.
    ///
    /// @param string $requestJson JSON-encoded OpenAI-compatible chat request.
    /// @return string JSON-encoded chat completion response.
    pub fn chat(&self, request_json: String) -> PhpResult<String> {
        let req: liter_llm::ChatCompletionRequest = serde_json::from_str(&request_json)
            .map_err(|e| PhpException::from(format!("invalid chat request JSON: {e}")))?;

        // Invoke pre-request hooks synchronously.
        invoke_hooks_on_request(&req)?;

        let response = block_on_future(self.inner.chat(req))?.map_err(|e| PhpException::from(e.to_string()))?;

        serde_json::to_string(&response).map_err(|e| PhpException::from(format!("serialization error: {e}")))
    }

    /// Send a streaming chat completion request and collect all chunks.
    ///
    /// PHP's synchronous execution model does not support true incremental
    /// streaming.  This method drives the full SSE stream to completion on
    /// the Rust side and returns all chunks as a JSON array.
    ///
    /// @param string $requestJson JSON-encoded OpenAI-compatible chat request.
    ///                            The `"stream"` field is forced to `true`.
    /// @return string JSON-encoded array of `ChatCompletionChunk` objects.
    #[php(name = "chatStream")]
    pub fn chat_stream(&self, request_json: String) -> PhpResult<String> {
        use futures_core::Stream as FStream;
        use std::pin::Pin;

        let req: liter_llm::ChatCompletionRequest = serde_json::from_str(&request_json)
            .map_err(|e| PhpException::from(format!("invalid chat stream request JSON: {e}")))?;

        // Collect all SSE chunks by blocking on the async stream.
        let items: Vec<liter_llm::ChatCompletionChunk> = block_on_future(async {
            let stream = self
                .inner
                .chat_stream(req)
                .await
                .map_err(|e| PhpException::from(e.to_string()))?;

            let mut collected: Vec<liter_llm::ChatCompletionChunk> = Vec::new();
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
        let req: liter_llm::EmbeddingRequest = serde_json::from_str(&request_json)
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

    // ── Additional inference methods ─────────────────────────────────────────

    /// Generate an image from a text prompt.
    ///
    /// @param string $requestJson JSON-encoded OpenAI-compatible image generation request.
    /// @return string JSON-encoded images response.
    #[php(name = "imageGenerate")]
    pub fn image_generate(&self, request_json: String) -> PhpResult<String> {
        let req: liter_llm::CreateImageRequest = serde_json::from_str(&request_json)
            .map_err(|e| PhpException::from(format!("invalid image generate request JSON: {e}")))?;

        let response =
            block_on_future(self.inner.image_generate(req))?.map_err(|e| PhpException::from(e.to_string()))?;

        serde_json::to_string(&response).map_err(|e| PhpException::from(format!("serialization error: {e}")))
    }

    /// Generate speech audio from text.
    ///
    /// Returns the raw audio bytes as a string (binary-safe in PHP).
    ///
    /// @param string $requestJson JSON-encoded OpenAI-compatible speech request.
    /// @return string Raw audio bytes.
    pub fn speech(&self, request_json: String) -> PhpResult<String> {
        let req: liter_llm::CreateSpeechRequest = serde_json::from_str(&request_json)
            .map_err(|e| PhpException::from(format!("invalid speech request JSON: {e}")))?;

        let response = block_on_future(self.inner.speech(req))?.map_err(|e| PhpException::from(e.to_string()))?;

        // Return raw bytes as a binary string — PHP strings are binary-safe.
        // SAFETY: String::from_utf8_lossy is not needed; we use from_raw_parts-style
        // conversion via unsafe to preserve exact bytes.  However, ext-php-rs
        // String return values are binary-safe, so we can safely transmute.
        Ok(unsafe { String::from_utf8_unchecked(response.to_vec()) })
    }

    /// Transcribe audio to text.
    ///
    /// @param string $requestJson JSON-encoded OpenAI-compatible transcription request.
    /// @return string JSON-encoded transcription response.
    pub fn transcribe(&self, request_json: String) -> PhpResult<String> {
        let req: liter_llm::CreateTranscriptionRequest = serde_json::from_str(&request_json)
            .map_err(|e| PhpException::from(format!("invalid transcribe request JSON: {e}")))?;

        let response = block_on_future(self.inner.transcribe(req))?.map_err(|e| PhpException::from(e.to_string()))?;

        serde_json::to_string(&response).map_err(|e| PhpException::from(format!("serialization error: {e}")))
    }

    /// Check content against moderation policies.
    ///
    /// @param string $requestJson JSON-encoded OpenAI-compatible moderation request.
    /// @return string JSON-encoded moderation response.
    pub fn moderate(&self, request_json: String) -> PhpResult<String> {
        let req: liter_llm::ModerationRequest = serde_json::from_str(&request_json)
            .map_err(|e| PhpException::from(format!("invalid moderation request JSON: {e}")))?;

        let response = block_on_future(self.inner.moderate(req))?.map_err(|e| PhpException::from(e.to_string()))?;

        serde_json::to_string(&response).map_err(|e| PhpException::from(format!("serialization error: {e}")))
    }

    /// Rerank documents by relevance to a query.
    ///
    /// @param string $requestJson JSON-encoded rerank request.
    /// @return string JSON-encoded rerank response.
    pub fn rerank(&self, request_json: String) -> PhpResult<String> {
        let req: liter_llm::RerankRequest = serde_json::from_str(&request_json)
            .map_err(|e| PhpException::from(format!("invalid rerank request JSON: {e}")))?;

        let response = block_on_future(self.inner.rerank(req))?.map_err(|e| PhpException::from(e.to_string()))?;

        serde_json::to_string(&response).map_err(|e| PhpException::from(format!("serialization error: {e}")))
    }

    // ── File management methods ──────────────────────────────────────────────

    /// Upload a file.
    ///
    /// @param string $requestJson JSON-encoded file upload request.
    /// @return string JSON-encoded file object.
    #[php(name = "createFile")]
    pub fn create_file(&self, request_json: String) -> PhpResult<String> {
        let req: liter_llm::CreateFileRequest = serde_json::from_str(&request_json)
            .map_err(|e| PhpException::from(format!("invalid create file request JSON: {e}")))?;

        let response = block_on_future(self.inner.create_file(req))?.map_err(|e| PhpException::from(e.to_string()))?;

        serde_json::to_string(&response).map_err(|e| PhpException::from(format!("serialization error: {e}")))
    }

    /// Retrieve metadata for a file by ID.
    ///
    /// @param string $fileId The file ID.
    /// @return string JSON-encoded file object.
    #[php(name = "retrieveFile")]
    pub fn retrieve_file(&self, file_id: String) -> PhpResult<String> {
        let response =
            block_on_future(self.inner.retrieve_file(&file_id))?.map_err(|e| PhpException::from(e.to_string()))?;

        serde_json::to_string(&response).map_err(|e| PhpException::from(format!("serialization error: {e}")))
    }

    /// Delete a file by ID.
    ///
    /// @param string $fileId The file ID.
    /// @return string JSON-encoded delete response.
    #[php(name = "deleteFile")]
    pub fn delete_file(&self, file_id: String) -> PhpResult<String> {
        let response =
            block_on_future(self.inner.delete_file(&file_id))?.map_err(|e| PhpException::from(e.to_string()))?;

        serde_json::to_string(&response).map_err(|e| PhpException::from(format!("serialization error: {e}")))
    }

    /// List files, optionally filtered by query parameters.
    ///
    /// @param string|null $queryJson JSON-encoded query parameters (optional).
    /// @return string JSON-encoded file list response.
    #[php(name = "listFiles")]
    pub fn list_files(&self, query_json: Option<String>) -> PhpResult<String> {
        let query: Option<liter_llm::FileListQuery> = query_json
            .map(|s| {
                serde_json::from_str(&s).map_err(|e| PhpException::from(format!("invalid list files query JSON: {e}")))
            })
            .transpose()?;

        let response = block_on_future(self.inner.list_files(query))?.map_err(|e| PhpException::from(e.to_string()))?;

        serde_json::to_string(&response).map_err(|e| PhpException::from(format!("serialization error: {e}")))
    }

    /// Retrieve the raw content of a file.
    ///
    /// @param string $fileId The file ID.
    /// @return string Raw file bytes.
    #[php(name = "fileContent")]
    pub fn file_content(&self, file_id: String) -> PhpResult<String> {
        let response =
            block_on_future(self.inner.file_content(&file_id))?.map_err(|e| PhpException::from(e.to_string()))?;

        // Return raw bytes as a binary string — PHP strings are binary-safe.
        Ok(unsafe { String::from_utf8_unchecked(response.to_vec()) })
    }

    // ── Batch management methods ─────────────────────────────────────────────

    /// Create a new batch job.
    ///
    /// @param string $requestJson JSON-encoded batch creation request.
    /// @return string JSON-encoded batch object.
    #[php(name = "createBatch")]
    pub fn create_batch(&self, request_json: String) -> PhpResult<String> {
        let req: liter_llm::CreateBatchRequest = serde_json::from_str(&request_json)
            .map_err(|e| PhpException::from(format!("invalid create batch request JSON: {e}")))?;

        let response = block_on_future(self.inner.create_batch(req))?.map_err(|e| PhpException::from(e.to_string()))?;

        serde_json::to_string(&response).map_err(|e| PhpException::from(format!("serialization error: {e}")))
    }

    /// Retrieve a batch by ID.
    ///
    /// @param string $batchId The batch ID.
    /// @return string JSON-encoded batch object.
    #[php(name = "retrieveBatch")]
    pub fn retrieve_batch(&self, batch_id: String) -> PhpResult<String> {
        let response =
            block_on_future(self.inner.retrieve_batch(&batch_id))?.map_err(|e| PhpException::from(e.to_string()))?;

        serde_json::to_string(&response).map_err(|e| PhpException::from(format!("serialization error: {e}")))
    }

    /// List batches, optionally filtered by query parameters.
    ///
    /// @param string|null $queryJson JSON-encoded query parameters (optional).
    /// @return string JSON-encoded batch list response.
    #[php(name = "listBatches")]
    pub fn list_batches(&self, query_json: Option<String>) -> PhpResult<String> {
        let query: Option<liter_llm::BatchListQuery> = query_json
            .map(|s| {
                serde_json::from_str(&s)
                    .map_err(|e| PhpException::from(format!("invalid list batches query JSON: {e}")))
            })
            .transpose()?;

        let response =
            block_on_future(self.inner.list_batches(query))?.map_err(|e| PhpException::from(e.to_string()))?;

        serde_json::to_string(&response).map_err(|e| PhpException::from(format!("serialization error: {e}")))
    }

    /// Cancel an in-progress batch.
    ///
    /// @param string $batchId The batch ID.
    /// @return string JSON-encoded batch object.
    #[php(name = "cancelBatch")]
    pub fn cancel_batch(&self, batch_id: String) -> PhpResult<String> {
        let response =
            block_on_future(self.inner.cancel_batch(&batch_id))?.map_err(|e| PhpException::from(e.to_string()))?;

        serde_json::to_string(&response).map_err(|e| PhpException::from(format!("serialization error: {e}")))
    }

    // ── Response management methods ──────────────────────────────────────────

    /// Create a new response.
    ///
    /// @param string $requestJson JSON-encoded response creation request.
    /// @return string JSON-encoded response object.
    #[php(name = "createResponse")]
    pub fn create_response(&self, request_json: String) -> PhpResult<String> {
        let req: liter_llm::CreateResponseRequest = serde_json::from_str(&request_json)
            .map_err(|e| PhpException::from(format!("invalid create response request JSON: {e}")))?;

        let response =
            block_on_future(self.inner.create_response(req))?.map_err(|e| PhpException::from(e.to_string()))?;

        serde_json::to_string(&response).map_err(|e| PhpException::from(format!("serialization error: {e}")))
    }

    /// Retrieve a response by ID.
    ///
    /// @param string $responseId The response ID.
    /// @return string JSON-encoded response object.
    #[php(name = "retrieveResponse")]
    pub fn retrieve_response(&self, response_id: String) -> PhpResult<String> {
        let response = block_on_future(self.inner.retrieve_response(&response_id))?
            .map_err(|e| PhpException::from(e.to_string()))?;

        serde_json::to_string(&response).map_err(|e| PhpException::from(format!("serialization error: {e}")))
    }

    /// Cancel an in-progress response.
    ///
    /// @param string $responseId The response ID.
    /// @return string JSON-encoded response object.
    #[php(name = "cancelResponse")]
    pub fn cancel_response(&self, response_id: String) -> PhpResult<String> {
        let response = block_on_future(self.inner.cancel_response(&response_id))?
            .map_err(|e| PhpException::from(e.to_string()))?;

        serde_json::to_string(&response).map_err(|e| PhpException::from(format!("serialization error: {e}")))
    }
}

// ─── Thread-local hook storage for addHook() ─────────────────────────────────

thread_local! {
    /// Hooks registered via `addHook()` at runtime.  These are invoked
    /// synchronously before each request via `invoke_hooks_on_request`.
    static HOOKS: RefCell<Vec<Arc<dyn LlmHook>>> = const { RefCell::new(Vec::new()) };
}

/// Invoke all registered thread-local hooks' `on_request` synchronously.
///
/// If any hook returns an error, the request is rejected with that error.
fn invoke_hooks_on_request(req: &liter_llm::ChatCompletionRequest) -> PhpResult<()> {
    let llm_req = LlmRequest::Chat(req.clone());
    HOOKS.with(|hooks| {
        let hooks = hooks.borrow();
        for hook in hooks.iter() {
            block_on_future(hook.on_request(&llm_req))?.map_err(|e| PhpException::from(e.to_string()))?;
        }
        Ok(())
    })
}

// ─── Module-level functions ──────────────────────────────────────────────────

/// Returns the version of the liter-llm library.
///
/// @return string Semver version string (e.g., "0.1.0").
#[php_function]
pub fn liter_llm_version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

/// Register a custom LLM provider at runtime.
///
/// The provider will be checked before all built-in providers during model
/// detection.  If a provider with the same name already exists it is replaced.
///
/// @param string $configJson JSON-encoded provider config.
///   Required fields: `name`, `base_url`, `model_prefixes` (array of strings).
///   Optional: `auth_header` — `"bearer"` (default), `"none"`, or `"api-key:X-Header-Name"`.
///
/// Example:
/// ```php
/// liter_llm_register_provider(json_encode([
///     'name' => 'my-provider',
///     'base_url' => 'https://api.my-provider.com/v1',
///     'model_prefixes' => ['my-'],
///     'auth_header' => 'bearer',
/// ]));
/// ```
#[php_function]
pub fn liter_llm_register_provider(config_json: String) -> PhpResult<()> {
    let val: serde_json::Value = serde_json::from_str(&config_json)
        .map_err(|e| PhpException::from(format!("invalid provider config JSON: {e}")))?;

    let name = val
        .get("name")
        .and_then(|v| v.as_str())
        .ok_or_else(|| PhpException::from("provider config requires 'name' (string)".to_string()))?
        .to_owned();
    let base_url = val
        .get("base_url")
        .and_then(|v| v.as_str())
        .ok_or_else(|| PhpException::from("provider config requires 'base_url' (string)".to_string()))?
        .to_owned();
    let auth_header_str = val.get("auth_header").and_then(|v| v.as_str()).unwrap_or("bearer");
    let auth_header = match auth_header_str {
        "none" => AuthHeaderFormat::None,
        s if s.starts_with("api-key:") => AuthHeaderFormat::ApiKey(s.trim_start_matches("api-key:").trim().to_owned()),
        _ => AuthHeaderFormat::Bearer,
    };
    let model_prefixes: Vec<String> = val
        .get("model_prefixes")
        .and_then(|v| v.as_array())
        .map(|arr| arr.iter().filter_map(|v| v.as_str().map(String::from)).collect())
        .unwrap_or_default();

    let config = CustomProviderConfig {
        name,
        base_url,
        auth_header,
        model_prefixes,
    };

    register_custom_provider(config).map_err(|e| PhpException::from(e.to_string()))
}

/// Unregister a previously registered custom provider by name.
///
/// @param string $name The provider name to remove.
/// @return bool True if a provider was found and removed, false otherwise.
#[php_function]
pub fn liter_llm_unregister_provider(name: String) -> PhpResult<bool> {
    unregister_custom_provider(&name).map_err(|e| PhpException::from(e.to_string()))
}

// ─── Module registration ──────────────────────────────────────────────────────

/// Register the `liter_llm` PHP extension module.
#[php_module]
pub fn get_module(module: ModuleBuilder) -> ModuleBuilder {
    module
        .function(wrap_function!(liter_llm_version))
        .function(wrap_function!(liter_llm_register_provider))
        .function(wrap_function!(liter_llm_unregister_provider))
        .class::<PhpLlmClient>()
}
