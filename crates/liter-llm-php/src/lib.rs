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

use ext_php_rs::prelude::*;
use liter_llm::{BatchClient, ClientConfigBuilder, DefaultClient, FileClient, LlmClient, ResponseClient};

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

// ─── LlmClient PHP class ──────────────────────────────────────────────────────

/// PHP class wrapping the liter-llm Rust client.
///
/// All request/response types use JSON strings so that PHP code can work with
/// standard `json_encode` / `json_decode` without needing custom PHP classes.
#[php_class]
#[php(name = "LiterLlm\\LlmClient")]
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
        let req: liter_llm::ChatCompletionRequest = serde_json::from_str(&request_json)
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

        let req: liter_llm::ChatCompletionRequest = serde_json::from_str(&request_json)
            .map_err(|e| PhpException::from(format!("invalid chat stream request JSON: {e}")))?;

        // The core client's chat_stream sets stream=true internally via
        // prepare_request; we must not set it here (the field is pub(crate)).

        // Collect all SSE chunks by blocking on the async stream.
        // Returns a PhpResult<Vec<_>> so we can propagate errors and then
        // serialise the collected chunks outside the async block.
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

// ─── Module-level function ────────────────────────────────────────────────────

/// Returns the version of the liter-llm library.
///
/// @return string Semver version string (e.g., "0.1.0").
#[php_function]
pub fn liter_llm_version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

// ─── Module registration ──────────────────────────────────────────────────────

/// Register the `liter_llm` PHP extension module.
#[php_module]
pub fn get_module(module: ModuleBuilder) -> ModuleBuilder {
    module
        .function(wrap_function!(liter_llm_version))
        .class::<PhpLlmClient>()
}
