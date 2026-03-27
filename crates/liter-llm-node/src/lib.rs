#![deny(clippy::all)]

use std::collections::HashMap;
use std::sync::Arc;

use liter_llm::LlmClient as LlmClientTrait;
use liter_llm::{BatchClient, ClientConfigBuilder, DefaultClient, FileClient, ResponseClient};
use napi::bindgen_prelude::*;
use napi_derive::napi;

// ─── camelCase conversion ─────────────────────────────────────────────────────

/// Convert a snake_case identifier to camelCase.
///
/// Edge cases handled correctly:
/// - Leading underscores are preserved: `__foo` → `__foo`
/// - Consecutive underscores collapse: `foo__bar` → `fooBar` (second `_`
///   triggers capitalisation; the extra underscore is consumed, not doubled)
/// - A leading single underscore is preserved: `_foo` → `_foo`
fn snake_to_camel(s: &str) -> String {
    let mut result = String::with_capacity(s.len());
    let mut chars = s.chars().peekable();
    // Preserve any leading underscores verbatim — they signal internal / dunder
    // names that should not be title-cased.
    while chars.peek() == Some(&'_') {
        result.push('_');
        chars.next();
    }

    let mut pending_underscores: usize = 0;
    for ch in chars {
        if ch == '_' {
            pending_underscores += 1;
        } else if pending_underscores > 0 {
            // Non-leading underscores: the first one triggers capitalisation,
            // any extras were consecutive underscores that collapse.
            result.extend(ch.to_uppercase());
            pending_underscores = 0;
        } else {
            result.push(ch);
        }
    }
    // Append any trailing underscores that were consumed but had no following
    // character to uppercase (e.g. `__init__` → the final `__` must survive).
    for _ in 0..pending_underscores {
        result.push('_');
    }
    result
}

/// Recursively convert all object keys from snake_case to camelCase.
///
/// Note: `tool_calls[].function.arguments` is a JSON-encoded string
/// (`Value::String`), not a nested object, so the recursive descent stops
/// there naturally — the contents of `arguments` are never key-converted.
/// This is correct behaviour: the arguments payload must remain unchanged.
fn to_camel_case_keys(value: serde_json::Value) -> serde_json::Value {
    match value {
        serde_json::Value::Object(map) => {
            let converted = map
                .into_iter()
                .map(|(k, v)| (snake_to_camel(&k), to_camel_case_keys(v)))
                .collect();
            serde_json::Value::Object(converted)
        }
        serde_json::Value::Array(arr) => serde_json::Value::Array(arr.into_iter().map(to_camel_case_keys).collect()),
        other => other,
    }
}

/// Serialize a Rust value to a camelCase `serde_json::Value` for JS consumption.
fn to_js_value<T: serde::Serialize>(value: T) -> napi::Result<serde_json::Value> {
    let raw = serde_json::to_value(value).map_err(|e| napi::Error::new(Status::GenericFailure, e.to_string()))?;
    Ok(to_camel_case_keys(raw))
}

/// Convert a `liter_llm::LiterLlmError` into a NAPI `Error`.
///
/// The error kind is embedded in the message so that JS callers can inspect it
/// even though NAPI-RS only exposes a single `Status::GenericFailure` code.
fn to_napi_err(e: liter_llm::LiterLlmError) -> napi::Error {
    // Include the variant name for programmatic inspection in JS-land.
    let msg = format!("[{}] {}", error_kind_label(&e), e);
    napi::Error::new(Status::GenericFailure, msg)
}

/// Return a short, stable label for each error variant.
fn error_kind_label(e: &liter_llm::LiterLlmError) -> &'static str {
    match e {
        liter_llm::LiterLlmError::Authentication { .. } => "Authentication",
        liter_llm::LiterLlmError::RateLimited { .. } => "RateLimited",
        liter_llm::LiterLlmError::BadRequest { .. } => "BadRequest",
        liter_llm::LiterLlmError::ContextWindowExceeded { .. } => "ContextWindowExceeded",
        liter_llm::LiterLlmError::ContentPolicy { .. } => "ContentPolicy",
        liter_llm::LiterLlmError::NotFound { .. } => "NotFound",
        liter_llm::LiterLlmError::ServerError { .. } => "ServerError",
        liter_llm::LiterLlmError::ServiceUnavailable { .. } => "ServiceUnavailable",
        liter_llm::LiterLlmError::Timeout => "Timeout",
        liter_llm::LiterLlmError::Network(_) => "Network",
        liter_llm::LiterLlmError::Streaming { .. } => "Streaming",
        liter_llm::LiterLlmError::EndpointNotSupported { .. } => "EndpointNotSupported",
        liter_llm::LiterLlmError::InvalidHeader { .. } => "InvalidHeader",
        liter_llm::LiterLlmError::Serialization(_) => "Serialization",
        // IMPORTANT: Update this match when adding new LiterLlmError variants.
        _ => "Unknown",
    }
}

// ─── JS config objects ────────────────────────────────────────────────────────

/// Cache configuration for response caching.
#[napi(object)]
pub struct CacheOptions {
    /// Maximum number of cached entries (default: 256).
    pub max_entries: Option<u32>,
    /// Time-to-live for cached entries in seconds (default: 300).
    pub ttl_seconds: Option<u32>,
}

/// Budget configuration for spending limits.
#[napi(object)]
pub struct BudgetOptions {
    /// Maximum total spend across all models in USD.
    pub global_limit: Option<f64>,
    /// Per-model spending limits in USD, keyed by model name.
    pub model_limits: Option<HashMap<String, f64>>,
    /// Enforcement mode: `"soft"` (warn only) or `"hard"` (reject).
    /// Defaults to `"hard"`.
    pub enforcement: Option<String>,
}

/// Custom provider configuration for runtime registration.
#[napi(object)]
pub struct CustomProviderOptions {
    /// Unique name for this provider.
    pub name: String,
    /// Base URL for the provider's API.
    pub base_url: String,
    /// Authentication style: `"bearer"`, `"none"`, or a custom header name
    /// (e.g. `"X-Api-Key"`).
    pub auth_header: String,
    /// Model name prefixes that route to this provider.
    pub model_prefixes: Vec<String>,
}

/// Options accepted by the `LlmClient` constructor.
#[napi(object)]
pub struct LlmClientOptions {
    pub api_key: String,
    pub base_url: Option<String>,
    /// Optional model hint for provider auto-detection (e.g. `"groq/llama3-70b"`).
    /// Pass this when no `baseUrl` is set so the client can select the correct
    /// provider endpoint and auth style at construction time.
    pub model_hint: Option<String>,
    pub max_retries: Option<u32>,
    /// Timeout in seconds.
    ///
    /// Note: NAPI-RS `#[napi(object)]` does not support `u64` directly
    /// (no `FromNapiValue` impl); `u32` covers ~136 years which is sufficient
    /// for any realistic timeout.  The Python binding uses `u64` but the
    /// underlying `Duration::from_secs` accepts `u64`, so there is no semantic
    /// loss for valid timeout values.
    pub timeout_secs: Option<u32>,
    /// Response cache configuration.
    pub cache: Option<CacheOptions>,
    /// Budget enforcement configuration.
    pub budget: Option<BudgetOptions>,
    /// Extra headers sent on every request, as key-value pairs.
    pub extra_headers: Option<HashMap<String, String>>,
}

// ─── LlmClient ────────────────────────────────────────────────────────────────

/// Node.js-accessible LLM client wrapping `liter_llm::DefaultClient`.
#[napi]
pub struct LlmClient {
    inner: Arc<DefaultClient>,
}

#[napi]
impl LlmClient {
    /// Create a new `LlmClient`.
    ///
    /// ```js
    /// const client = new LlmClient({ apiKey: "sk-...", baseUrl: "https://..." });
    /// ```
    #[napi(constructor)]
    pub fn new(options: LlmClientOptions) -> napi::Result<Self> {
        let mut builder = ClientConfigBuilder::new(options.api_key);

        if let Some(url) = options.base_url {
            builder = builder.base_url(url);
        }
        if let Some(retries) = options.max_retries {
            builder = builder.max_retries(retries);
        }
        if let Some(secs) = options.timeout_secs {
            builder = builder.timeout(std::time::Duration::from_secs(u64::from(secs)));
        }

        // Cache configuration.
        if let Some(cache) = options.cache {
            let cache_config = liter_llm::tower::CacheConfig {
                max_entries: cache.max_entries.map(|n| n as usize).unwrap_or(256),
                ttl: std::time::Duration::from_secs(u64::from(cache.ttl_seconds.unwrap_or(300))),
            };
            builder = builder.cache(cache_config);
        }

        // Budget configuration.
        if let Some(budget) = options.budget {
            let enforcement = match budget.enforcement.as_deref() {
                Some("soft") => liter_llm::tower::Enforcement::Soft,
                _ => liter_llm::tower::Enforcement::Hard,
            };
            let budget_config = liter_llm::tower::BudgetConfig {
                global_limit: budget.global_limit,
                model_limits: budget.model_limits.unwrap_or_default(),
                enforcement,
            };
            builder = builder.budget(budget_config);
        }

        // Extra headers.
        if let Some(headers) = options.extra_headers {
            for (key, value) in headers {
                builder = builder
                    .header(key, value)
                    .map_err(|e| napi::Error::new(Status::InvalidArg, e.to_string()))?;
            }
        }

        let config = builder.build();
        let client = DefaultClient::new(config, options.model_hint.as_deref()).map_err(to_napi_err)?;
        Ok(Self {
            inner: Arc::new(client),
        })
    }

    /// Send a chat completion request.
    ///
    /// Accepts a plain JS object matching the OpenAI Chat Completions API.
    /// Returns a `Promise<object>` resolving to a `ChatCompletionResponse`.
    ///
    /// ```js
    /// const resp = await client.chat({ model: "gpt-4", messages: [{ role: "user", content: "Hi" }] });
    /// console.log(resp.choices[0].message.content);
    /// ```
    pub async fn chat(&self, request: serde_json::Value) -> napi::Result<serde_json::Value> {
        let req: liter_llm::ChatCompletionRequest =
            serde_json::from_value(request).map_err(|e| napi::Error::new(Status::InvalidArg, e.to_string()))?;

        let client = Arc::clone(&self.inner);
        let result = client.chat(req).await.map_err(to_napi_err)?;
        to_js_value(result)
    }

    /// Collect all streaming chat completion chunks into an array.
    ///
    /// **Note: This method buffers all chunks before returning.**  The full SSE
    /// stream is consumed on the Rust side and the resolved Promise contains a
    /// JS array of chunk objects.  No data is surfaced to JavaScript until the
    /// stream completes.  For true incremental streaming (chunk-by-chunk as the
    /// model generates), use the callback-based API (coming soon).
    ///
    /// ```js
    /// const chunks = await client.chatStream({ model: "gpt-4", messages: [...], stream: true });
    /// for (const chunk of chunks) {
    ///   process.stdout.write(chunk.choices[0]?.delta?.content ?? "");
    /// }
    /// ```
    #[napi(js_name = "chatStream")]
    pub async fn chat_stream(&self, request: serde_json::Value) -> napi::Result<Vec<serde_json::Value>> {
        let req: liter_llm::ChatCompletionRequest =
            serde_json::from_value(request).map_err(|e| napi::Error::new(Status::InvalidArg, e.to_string()))?;

        // The core client's chat_stream sets stream=true internally via
        // prepare_request; we must not set it here (the field is pub(crate)).
        let client = Arc::clone(&self.inner);

        // Collect all SSE chunks into a Vec before returning to JavaScript.
        // This avoids the complexity of AsyncIterable bindings in NAPI-RS while
        // still exercising the full streaming code path end-to-end.
        let stream = client.chat_stream(req).await.map_err(to_napi_err)?;
        let chunks = collect_chunk_stream(stream).await.map_err(to_napi_err)?;

        chunks.into_iter().map(to_js_value).collect::<napi::Result<Vec<_>>>()
    }

    /// Send an embedding request.
    ///
    /// Accepts a plain JS object matching the OpenAI Embeddings API.
    /// Returns a `Promise<object>` resolving to an `EmbeddingResponse`.
    ///
    /// ```js
    /// const resp = await client.embed({ model: "text-embedding-3-small", input: "Hello" });
    /// console.log(resp.data[0].embedding);
    /// ```
    pub async fn embed(&self, request: serde_json::Value) -> napi::Result<serde_json::Value> {
        let req: liter_llm::EmbeddingRequest =
            serde_json::from_value(request).map_err(|e| napi::Error::new(Status::InvalidArg, e.to_string()))?;

        let client = Arc::clone(&self.inner);
        let result = client.embed(req).await.map_err(to_napi_err)?;
        to_js_value(result)
    }

    /// List available models from the provider.
    ///
    /// Returns a `Promise<object>` resolving to a `ModelsListResponse`.
    ///
    /// ```js
    /// const resp = await client.listModels();
    /// console.log(resp.data.map(m => m.id));
    /// ```
    #[napi(js_name = "listModels")]
    pub async fn list_models(&self) -> napi::Result<serde_json::Value> {
        let client = Arc::clone(&self.inner);
        let result = client.list_models().await.map_err(to_napi_err)?;
        to_js_value(result)
    }

    // ── Additional inference methods ─────────────────────────────────────────

    /// Generate an image from a text prompt.
    ///
    /// Accepts a plain JS object matching the OpenAI Images API.
    /// Returns a `Promise<object>` resolving to an `ImagesResponse`.
    ///
    /// ```js
    /// const resp = await client.imageGenerate({ model: "dall-e-3", prompt: "A sunset" });
    /// console.log(resp.data[0].url);
    /// ```
    #[napi(js_name = "imageGenerate")]
    pub async fn image_generate(&self, request: serde_json::Value) -> napi::Result<serde_json::Value> {
        let req: liter_llm::CreateImageRequest =
            serde_json::from_value(request).map_err(|e| napi::Error::new(Status::InvalidArg, e.to_string()))?;

        let client = Arc::clone(&self.inner);
        let result = client.image_generate(req).await.map_err(to_napi_err)?;
        to_js_value(result)
    }

    /// Generate speech audio from text.
    ///
    /// Accepts a plain JS object matching the OpenAI Audio Speech API.
    /// Returns a `Promise<Buffer>` containing the raw audio bytes.
    ///
    /// ```js
    /// const buf = await client.speech({ model: "tts-1", input: "Hello", voice: "alloy" });
    /// fs.writeFileSync("output.mp3", buf);
    /// ```
    pub async fn speech(&self, request: serde_json::Value) -> napi::Result<Buffer> {
        let req: liter_llm::CreateSpeechRequest =
            serde_json::from_value(request).map_err(|e| napi::Error::new(Status::InvalidArg, e.to_string()))?;

        let client = Arc::clone(&self.inner);
        let result = client.speech(req).await.map_err(to_napi_err)?;
        Ok(result.to_vec().into())
    }

    /// Transcribe audio to text.
    ///
    /// Accepts a plain JS object matching the OpenAI Audio Transcriptions API.
    /// Returns a `Promise<object>` resolving to a `TranscriptionResponse`.
    ///
    /// ```js
    /// const resp = await client.transcribe({ model: "whisper-1", file: base64Audio });
    /// console.log(resp.text);
    /// ```
    pub async fn transcribe(&self, request: serde_json::Value) -> napi::Result<serde_json::Value> {
        let req: liter_llm::CreateTranscriptionRequest =
            serde_json::from_value(request).map_err(|e| napi::Error::new(Status::InvalidArg, e.to_string()))?;

        let client = Arc::clone(&self.inner);
        let result = client.transcribe(req).await.map_err(to_napi_err)?;
        to_js_value(result)
    }

    /// Check content against moderation policies.
    ///
    /// Accepts a plain JS object matching the OpenAI Moderations API.
    /// Returns a `Promise<object>` resolving to a `ModerationResponse`.
    ///
    /// ```js
    /// const resp = await client.moderate({ model: "text-moderation-latest", input: "some text" });
    /// console.log(resp.results[0].flagged);
    /// ```
    pub async fn moderate(&self, request: serde_json::Value) -> napi::Result<serde_json::Value> {
        let req: liter_llm::ModerationRequest =
            serde_json::from_value(request).map_err(|e| napi::Error::new(Status::InvalidArg, e.to_string()))?;

        let client = Arc::clone(&self.inner);
        let result = client.moderate(req).await.map_err(to_napi_err)?;
        to_js_value(result)
    }

    /// Rerank documents by relevance to a query.
    ///
    /// Accepts a plain JS object matching the rerank API format.
    /// Returns a `Promise<object>` resolving to a `RerankResponse`.
    ///
    /// ```js
    /// const resp = await client.rerank({ model: "rerank-v1", query: "q", documents: ["a", "b"] });
    /// console.log(resp.results);
    /// ```
    pub async fn rerank(&self, request: serde_json::Value) -> napi::Result<serde_json::Value> {
        let req: liter_llm::RerankRequest =
            serde_json::from_value(request).map_err(|e| napi::Error::new(Status::InvalidArg, e.to_string()))?;

        let client = Arc::clone(&self.inner);
        let result = client.rerank(req).await.map_err(to_napi_err)?;
        to_js_value(result)
    }

    // ── File management methods ──────────────────────────────────────────────

    /// Upload a file.
    ///
    /// Accepts a plain JS object with `file` (base64-encoded), `purpose`, and
    /// optional `filename` fields.
    /// Returns a `Promise<object>` resolving to a `FileObject`.
    ///
    /// ```js
    /// const resp = await client.createFile({ file: base64Data, purpose: "assistants" });
    /// console.log(resp.id);
    /// ```
    #[napi(js_name = "createFile")]
    pub async fn create_file(&self, request: serde_json::Value) -> napi::Result<serde_json::Value> {
        let req: liter_llm::CreateFileRequest =
            serde_json::from_value(request).map_err(|e| napi::Error::new(Status::InvalidArg, e.to_string()))?;

        let client = Arc::clone(&self.inner);
        let result = client.create_file(req).await.map_err(to_napi_err)?;
        to_js_value(result)
    }

    /// Retrieve metadata for a file by ID.
    ///
    /// Returns a `Promise<object>` resolving to a `FileObject`.
    ///
    /// ```js
    /// const file = await client.retrieveFile("file-abc123");
    /// console.log(file.filename);
    /// ```
    #[napi(js_name = "retrieveFile")]
    pub async fn retrieve_file(&self, file_id: String) -> napi::Result<serde_json::Value> {
        let client = Arc::clone(&self.inner);
        let result = client.retrieve_file(&file_id).await.map_err(to_napi_err)?;
        to_js_value(result)
    }

    /// Delete a file by ID.
    ///
    /// Returns a `Promise<object>` resolving to a `DeleteResponse`.
    ///
    /// ```js
    /// const resp = await client.deleteFile("file-abc123");
    /// console.log(resp.deleted);
    /// ```
    #[napi(js_name = "deleteFile")]
    pub async fn delete_file(&self, file_id: String) -> napi::Result<serde_json::Value> {
        let client = Arc::clone(&self.inner);
        let result = client.delete_file(&file_id).await.map_err(to_napi_err)?;
        to_js_value(result)
    }

    /// List files, optionally filtered by query parameters.
    ///
    /// Pass `null` or `undefined` to list all files without filtering.
    /// Returns a `Promise<object>` resolving to a `FileListResponse`.
    ///
    /// ```js
    /// const resp = await client.listFiles({ purpose: "assistants" });
    /// console.log(resp.data.map(f => f.id));
    /// ```
    #[napi(js_name = "listFiles")]
    pub async fn list_files(&self, query: Option<serde_json::Value>) -> napi::Result<serde_json::Value> {
        let parsed: Option<liter_llm::FileListQuery> = query
            .map(|v| serde_json::from_value(v).map_err(|e| napi::Error::new(Status::InvalidArg, e.to_string())))
            .transpose()?;

        let client = Arc::clone(&self.inner);
        let result = client.list_files(parsed).await.map_err(to_napi_err)?;
        to_js_value(result)
    }

    /// Retrieve the raw content of a file.
    ///
    /// Returns a `Promise<Buffer>` containing the file bytes.
    ///
    /// ```js
    /// const buf = await client.fileContent("file-abc123");
    /// fs.writeFileSync("downloaded.jsonl", buf);
    /// ```
    #[napi(js_name = "fileContent")]
    pub async fn file_content(&self, file_id: String) -> napi::Result<Buffer> {
        let client = Arc::clone(&self.inner);
        let result = client.file_content(&file_id).await.map_err(to_napi_err)?;
        Ok(result.to_vec().into())
    }

    // ── Batch management methods ─────────────────────────────────────────────

    /// Create a new batch job.
    ///
    /// Accepts a plain JS object with batch creation parameters.
    /// Returns a `Promise<object>` resolving to a `BatchObject`.
    ///
    /// ```js
    /// const batch = await client.createBatch({ inputFileId: "file-abc", endpoint: "/v1/chat/completions" });
    /// console.log(batch.id);
    /// ```
    #[napi(js_name = "createBatch")]
    pub async fn create_batch(&self, request: serde_json::Value) -> napi::Result<serde_json::Value> {
        let req: liter_llm::CreateBatchRequest =
            serde_json::from_value(request).map_err(|e| napi::Error::new(Status::InvalidArg, e.to_string()))?;

        let client = Arc::clone(&self.inner);
        let result = client.create_batch(req).await.map_err(to_napi_err)?;
        to_js_value(result)
    }

    /// Retrieve a batch by ID.
    ///
    /// Returns a `Promise<object>` resolving to a `BatchObject`.
    ///
    /// ```js
    /// const batch = await client.retrieveBatch("batch_abc123");
    /// console.log(batch.status);
    /// ```
    #[napi(js_name = "retrieveBatch")]
    pub async fn retrieve_batch(&self, batch_id: String) -> napi::Result<serde_json::Value> {
        let client = Arc::clone(&self.inner);
        let result = client.retrieve_batch(&batch_id).await.map_err(to_napi_err)?;
        to_js_value(result)
    }

    /// List batches, optionally filtered by query parameters.
    ///
    /// Pass `null` or `undefined` to list all batches without filtering.
    /// Returns a `Promise<object>` resolving to a `BatchListResponse`.
    ///
    /// ```js
    /// const resp = await client.listBatches();
    /// console.log(resp.data.map(b => b.id));
    /// ```
    #[napi(js_name = "listBatches")]
    pub async fn list_batches(&self, query: Option<serde_json::Value>) -> napi::Result<serde_json::Value> {
        let parsed: Option<liter_llm::BatchListQuery> = query
            .map(|v| serde_json::from_value(v).map_err(|e| napi::Error::new(Status::InvalidArg, e.to_string())))
            .transpose()?;

        let client = Arc::clone(&self.inner);
        let result = client.list_batches(parsed).await.map_err(to_napi_err)?;
        to_js_value(result)
    }

    /// Cancel an in-progress batch.
    ///
    /// Returns a `Promise<object>` resolving to the cancelled `BatchObject`.
    ///
    /// ```js
    /// const batch = await client.cancelBatch("batch_abc123");
    /// console.log(batch.status); // "cancelling"
    /// ```
    #[napi(js_name = "cancelBatch")]
    pub async fn cancel_batch(&self, batch_id: String) -> napi::Result<serde_json::Value> {
        let client = Arc::clone(&self.inner);
        let result = client.cancel_batch(&batch_id).await.map_err(to_napi_err)?;
        to_js_value(result)
    }

    // ── Response management methods ──────────────────────────────────────────

    /// Create a new response.
    ///
    /// Accepts a plain JS object with response creation parameters.
    /// Returns a `Promise<object>` resolving to a `ResponseObject`.
    ///
    /// ```js
    /// const resp = await client.createResponse({ model: "gpt-4", input: "Hello" });
    /// console.log(resp.id);
    /// ```
    #[napi(js_name = "createResponse")]
    pub async fn create_response(&self, request: serde_json::Value) -> napi::Result<serde_json::Value> {
        let req: liter_llm::CreateResponseRequest =
            serde_json::from_value(request).map_err(|e| napi::Error::new(Status::InvalidArg, e.to_string()))?;

        let client = Arc::clone(&self.inner);
        let result = client.create_response(req).await.map_err(to_napi_err)?;
        to_js_value(result)
    }

    /// Retrieve a response by ID.
    ///
    /// Returns a `Promise<object>` resolving to a `ResponseObject`.
    ///
    /// ```js
    /// const resp = await client.retrieveResponse("resp_abc123");
    /// console.log(resp.status);
    /// ```
    #[napi(js_name = "retrieveResponse")]
    pub async fn retrieve_response(&self, id: String) -> napi::Result<serde_json::Value> {
        let client = Arc::clone(&self.inner);
        let result = client.retrieve_response(&id).await.map_err(to_napi_err)?;
        to_js_value(result)
    }

    /// Cancel an in-progress response.
    ///
    /// Returns a `Promise<object>` resolving to the cancelled `ResponseObject`.
    ///
    /// ```js
    /// const resp = await client.cancelResponse("resp_abc123");
    /// console.log(resp.status); // "cancelled"
    /// ```
    #[napi(js_name = "cancelResponse")]
    pub async fn cancel_response(&self, id: String) -> napi::Result<serde_json::Value> {
        let client = Arc::clone(&self.inner);
        let result = client.cancel_response(&id).await.map_err(to_napi_err)?;
        to_js_value(result)
    }

    // ── Custom provider registration ────────────────────────────────────────

    /// Register a custom LLM provider at runtime.
    ///
    /// The provider will be checked before all built-in providers during model
    /// detection.  If a provider with the same name already exists it is
    /// replaced.
    ///
    /// ```js
    /// client.registerProvider({
    ///   name: "my-provider",
    ///   baseUrl: "https://api.my-provider.com/v1",
    ///   authHeader: "bearer",
    ///   modelPrefixes: ["my-provider/"],
    /// });
    /// ```
    #[napi(js_name = "registerProvider")]
    pub fn register_provider(config: CustomProviderOptions) -> napi::Result<()> {
        let auth_header = match config.auth_header.to_lowercase().as_str() {
            "bearer" => liter_llm::AuthHeaderFormat::Bearer,
            "none" => liter_llm::AuthHeaderFormat::None,
            custom => liter_llm::AuthHeaderFormat::ApiKey(custom.to_owned()),
        };

        let provider_config = liter_llm::CustomProviderConfig {
            name: config.name,
            base_url: config.base_url,
            auth_header,
            model_prefixes: config.model_prefixes,
        };

        liter_llm::register_custom_provider(provider_config).map_err(to_napi_err)
    }

    /// Unregister a previously registered custom provider by name.
    ///
    /// Returns `true` if the provider was found and removed, `false` if no
    /// such provider existed.
    ///
    /// ```js
    /// const removed = client.unregisterProvider("my-provider");
    /// ```
    #[napi(js_name = "unregisterProvider")]
    pub fn unregister_provider(name: String) -> napi::Result<bool> {
        liter_llm::unregister_custom_provider(&name).map_err(to_napi_err)
    }
}

/// Returns the version of the liter-llm library.
#[napi]
pub fn version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

// ─── Stream helpers ───────────────────────────────────────────────────────────

/// Drain a `BoxStream` of `ChatCompletionChunk`s into a `Vec`, short-circuiting
/// on the first error.
async fn collect_chunk_stream(
    stream: liter_llm::BoxStream<'_, liter_llm::ChatCompletionChunk>,
) -> liter_llm::Result<Vec<liter_llm::ChatCompletionChunk>> {
    use std::future::Future;
    use std::pin::Pin;
    use std::task::{Context, Poll};

    // Drive the stream to completion using a simple poll loop bridged to async.
    // We use `tokio::pin!` via the async block to avoid lifetime issues.
    struct StreamCollector<'a> {
        stream: liter_llm::BoxStream<'a, liter_llm::ChatCompletionChunk>,
        items: Vec<liter_llm::ChatCompletionChunk>,
    }

    impl Future for StreamCollector<'_> {
        type Output = liter_llm::Result<Vec<liter_llm::ChatCompletionChunk>>;

        fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
            use futures_core::Stream as FStream;
            loop {
                match FStream::poll_next(self.stream.as_mut(), cx) {
                    Poll::Ready(Some(Ok(chunk))) => self.items.push(chunk),
                    Poll::Ready(Some(Err(e))) => return Poll::Ready(Err(e)),
                    Poll::Ready(None) => {
                        // Clone items out — can't move out of `self` easily via Pin.
                        let items = std::mem::take(&mut self.items);
                        return Poll::Ready(Ok(items));
                    }
                    Poll::Pending => return Poll::Pending,
                }
            }
        }
    }

    StreamCollector {
        stream,
        items: Vec::new(),
    }
    .await
}

// ─── Tests ────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::snake_to_camel;

    #[test]
    fn snake_to_camel_basic() {
        assert_eq!(snake_to_camel("foo_bar"), "fooBar");
        assert_eq!(snake_to_camel("foo_bar_baz"), "fooBarBaz");
    }

    #[test]
    fn snake_to_camel_no_underscores() {
        assert_eq!(snake_to_camel("foobar"), "foobar");
    }

    #[test]
    fn snake_to_camel_leading_single_underscore_preserved() {
        assert_eq!(snake_to_camel("_foo"), "_foo");
    }

    #[test]
    fn snake_to_camel_leading_double_underscore_preserved() {
        assert_eq!(snake_to_camel("__foo"), "__foo");
        assert_eq!(snake_to_camel("__init__"), "__init__");
    }

    #[test]
    fn snake_to_camel_consecutive_underscores_in_middle() {
        // Extra underscores collapse: `foo__bar` → `fooBar`
        assert_eq!(snake_to_camel("foo__bar"), "fooBar");
    }

    #[test]
    fn snake_to_camel_empty() {
        assert_eq!(snake_to_camel(""), "");
    }
}
