#![deny(clippy::all)]

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use liter_llm::LlmClient as LlmClientTrait;
use liter_llm::{BatchClient, ClientConfigBuilder, FileClient, ManagedClient, ResponseClient};
use napi::bindgen_prelude::*;
use napi::threadsafe_function::ThreadsafeFunction;
use napi_derive::napi;

use liter_llm_bindings_core::case::{to_camel_case_keys, to_snake_case_keys};
use liter_llm_bindings_core::error::error_kind_label;

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

/// Deserialize a `serde_json::Value` from JS into a Rust type, normalizing
/// camelCase keys to snake_case first so JS callers can pass either convention.
fn from_js_value<T: serde::de::DeserializeOwned>(value: serde_json::Value) -> napi::Result<T> {
    let normalized = to_snake_case_keys(value);
    serde_json::from_value(normalized).map_err(|e| napi::Error::new(Status::InvalidArg, e.to_string()))
}

// ─── NAPI Hook Bridge ────────────────────────────────────────────────────────

/// Concrete type alias for a `ThreadsafeFunction` that accepts a `String`
/// argument and returns `Promise<()>`.  The `false` const generic means
/// this is a strong (prevent-GC) reference.
type HookTsfn = ThreadsafeFunction<String, Promise<()>, String, napi::Status, false>;

/// A bridge that wraps JS hook callbacks as `ThreadsafeFunction`s so they can
/// be invoked from Rust async code running on the tokio/NAPI worker thread.
///
/// The JS hook object may define any combination of:
///   - `onRequest(requestJson: string): void | Promise<void>` — may throw to reject
///   - `onResponse(payloadJson: string): void | Promise<void>` — advisory
///   - `onError(payloadJson: string): void | Promise<void>` — advisory
///
/// Missing methods are silently ignored (no-op).
#[derive(Clone)]
struct NapiHookBridge {
    on_request: Option<Arc<HookTsfn>>,
    on_response: Option<Arc<HookTsfn>>,
    on_error: Option<Arc<HookTsfn>>,
}

impl NapiHookBridge {
    /// Create a new `NapiHookBridge` by extracting optional callback functions
    /// from a JS object.  Each callback is converted to a `ThreadsafeFunction`
    /// so it can be called from any thread.
    fn from_object(hook: &Object) -> napi::Result<Self> {
        let on_request = Self::extract_tsfn(hook, "onRequest")?;
        let on_response = Self::extract_tsfn(hook, "onResponse")?;
        let on_error = Self::extract_tsfn(hook, "onError")?;

        if on_request.is_none() && on_response.is_none() && on_error.is_none() {
            return Err(napi::Error::new(
                Status::InvalidArg,
                "hook object must define at least one of onRequest, onResponse, or onError",
            ));
        }

        Ok(Self {
            on_request: on_request.map(Arc::new),
            on_response: on_response.map(Arc::new),
            on_error: on_error.map(Arc::new),
        })
    }

    /// Try to extract a named function property from a JS object and convert it
    /// to a `ThreadsafeFunction<String, Promise<()>>`.  Returns `Ok(None)` if
    /// the property does not exist or is not a function.
    fn extract_tsfn(obj: &Object, name: &str) -> napi::Result<Option<HookTsfn>> {
        // Attempt to get the property as a Function.  If the property is missing
        // or not a function, return None rather than an error.
        let func: Function<String, Promise<()>> = match obj.get_named_property(name) {
            Ok(f) => f,
            Err(_) => return Ok(None),
        };
        let tsfn = func
            .build_threadsafe_function()
            .build_callback(|ctx| Ok(ctx.value))
            .map_err(|e| {
                napi::Error::new(
                    Status::GenericFailure,
                    format!("failed to build ThreadsafeFunction for hook '{name}': {e}"),
                )
            })?;
        Ok(Some(tsfn))
    }

    /// Invoke the `onRequest` hook.  Returns `Err` if the JS callback throws
    /// (enabling guardrail rejection patterns).  Returns `Ok(())` if no
    /// `onRequest` callback is registered.
    async fn invoke_on_request(&self, request_json: &str) -> napi::Result<()> {
        if let Some(ref tsfn) = self.on_request {
            let promise = tsfn.call_async(request_json.to_owned()).await?;
            promise.await?;
        }
        Ok(())
    }

    /// Invoke the `onResponse` hook.  Errors are silently ignored since
    /// response hooks are advisory.
    async fn invoke_on_response(&self, payload_json: &str) {
        if let Some(ref tsfn) = self.on_response
            && let Ok(promise) = tsfn.call_async(payload_json.to_owned()).await
        {
            let _ = promise.await;
        }
    }

    /// Invoke the `onError` hook.  Errors are silently ignored since
    /// error hooks are advisory.
    async fn invoke_on_error(&self, payload_json: &str) {
        if let Some(ref tsfn) = self.on_error
            && let Ok(promise) = tsfn.call_async(payload_json.to_owned()).await
        {
            let _ = promise.await;
        }
    }
}

/// Invoke `onRequest` on all hooks sequentially.  Short-circuits on first error
/// (JS callback threw), enabling guardrail rejection.
async fn invoke_hooks_on_request(hooks: &[NapiHookBridge], request_json: &str) -> napi::Result<()> {
    for hook in hooks {
        hook.invoke_on_request(request_json).await?;
    }
    Ok(())
}

/// Invoke `onResponse` on all hooks sequentially.  Errors are silently ignored.
async fn invoke_hooks_on_response(hooks: &[NapiHookBridge], payload_json: &str) {
    for hook in hooks {
        hook.invoke_on_response(payload_json).await;
    }
}

/// Invoke `onError` on all hooks sequentially.  Errors are silently ignored.
async fn invoke_hooks_on_error(hooks: &[NapiHookBridge], payload_json: &str) {
    for hook in hooks {
        hook.invoke_on_error(payload_json).await;
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

/// Rate limit configuration for request throttling.
#[napi(object)]
pub struct RateLimitOptions {
    /// Maximum requests per minute.
    pub rpm: Option<u32>,
    /// Maximum tokens per minute.
    pub tpm: Option<f64>,
    /// Window size in seconds (default: 60).
    pub window_seconds: Option<u32>,
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
    /// Cooldown period in seconds between requests after errors.
    pub cooldown: Option<u32>,
    /// Rate limit configuration for request throttling.
    pub rate_limit: Option<RateLimitOptions>,
    /// Health check interval in seconds.
    pub health_check: Option<u32>,
    /// Enable cost tracking middleware.
    pub cost_tracking: Option<bool>,
    /// Enable tracing middleware.
    pub tracing: Option<bool>,
}

// ─── LlmClient ────────────────────────────────────────────────────────────────

/// Node.js-accessible LLM client wrapping `liter_llm::ManagedClient`.
///
/// Lifecycle hooks (`addHook`) are stored and invoked at the binding layer
/// (before/after each API call) using `ThreadsafeFunction` to bridge back
/// into JavaScript.  Hooks are invoked sequentially in registration order.
#[napi]
pub struct LlmClient {
    inner: Arc<ManagedClient>,
    hooks: Arc<Mutex<Vec<NapiHookBridge>>>,
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
                backend: Default::default(),
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

        // Cooldown configuration.
        if let Some(secs) = options.cooldown {
            builder = builder.cooldown(std::time::Duration::from_secs(u64::from(secs)));
        }

        // Rate limit configuration.
        if let Some(rl) = options.rate_limit {
            let rl_config = liter_llm::tower::RateLimitConfig {
                rpm: rl.rpm,
                tpm: rl.tpm.map(|v| v as u64),
                window: std::time::Duration::from_secs(u64::from(rl.window_seconds.unwrap_or(60))),
            };
            builder = builder.rate_limit(rl_config);
        }

        // Health check configuration.
        if let Some(secs) = options.health_check {
            builder = builder.health_check(std::time::Duration::from_secs(u64::from(secs)));
        }

        // Cost tracking.
        if options.cost_tracking.unwrap_or(false) {
            builder = builder.cost_tracking(true);
        }

        // Tracing.
        if options.tracing.unwrap_or(false) {
            builder = builder.tracing(true);
        }

        let config = builder.build();
        let client = ManagedClient::new(config, options.model_hint.as_deref()).map_err(to_napi_err)?;
        Ok(Self {
            inner: Arc::new(client),
            hooks: Arc::new(Mutex::new(Vec::new())),
        })
    }

    /// Register a lifecycle hook object.
    ///
    /// The hook should be a plain JS object with optional `onRequest`,
    /// `onResponse`, and `onError` callback functions:
    ///
    /// ```js
    /// client.addHook({
    ///   onRequest(req) { console.log("sending", JSON.parse(req)); },
    ///   onResponse(payload) { console.log("received", JSON.parse(payload)); },
    ///   onError(payload) { console.error("error", JSON.parse(payload)); },
    /// });
    /// ```
    ///
    /// Hooks are invoked in registration order around each API call.
    ///
    /// - `onRequest` receives a JSON string of the request.  Throw to reject
    ///   the request (guardrail pattern).
    /// - `onResponse` receives a JSON string with `{ request, response }`.
    ///   Errors are silently ignored.
    /// - `onError` receives a JSON string with `{ request, error }`.
    ///   Errors are silently ignored.
    ///
    /// All callbacks may be sync or async (returning a Promise).
    #[napi(js_name = "addHook")]
    pub fn add_hook(&self, hook: Object) -> napi::Result<()> {
        let bridge = NapiHookBridge::from_object(&hook)?;
        let mut hooks = self
            .hooks
            .lock()
            .map_err(|e| napi::Error::new(Status::GenericFailure, format!("hook lock poisoned: {e}")))?;
        hooks.push(bridge);
        Ok(())
    }

    /// Return the total spend tracked by the budget middleware (USD).
    ///
    /// Returns `0.0` if no budget configuration was provided at construction.
    #[napi(getter, js_name = "budgetUsed")]
    pub fn budget_used(&self) -> f64 {
        self.inner.budget_state().map(|s| s.global_spend()).unwrap_or(0.0)
    }

    /// Return a snapshot of the current hooks for use in async methods.
    ///
    /// We take a snapshot (clone) so that the `Mutex` is not held across
    /// `.await` points.
    fn snapshot_hooks(&self) -> Vec<NapiHookBridge> {
        self.hooks.lock().unwrap_or_else(|e| e.into_inner()).clone()
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
    #[napi]
    pub async fn chat(&self, request: serde_json::Value) -> napi::Result<serde_json::Value> {
        let req: liter_llm::ChatCompletionRequest = from_js_value(request.clone())?;

        let hooks = self.snapshot_hooks();
        let req_json = serde_json::to_string(&request).unwrap_or_default();
        invoke_hooks_on_request(&hooks, &req_json).await?;

        let client = Arc::clone(&self.inner);
        match client.chat(req).await {
            Ok(result) => {
                let js_val = to_js_value(&result)?;
                let payload = serde_json::json!({ "request": request, "response": js_val });
                invoke_hooks_on_response(&hooks, &payload.to_string()).await;
                Ok(js_val)
            }
            Err(e) => {
                let payload = serde_json::json!({ "request": request, "error": e.to_string() });
                invoke_hooks_on_error(&hooks, &payload.to_string()).await;
                Err(to_napi_err(e))
            }
        }
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
        let req: liter_llm::ChatCompletionRequest = from_js_value(request.clone())?;

        let hooks = self.snapshot_hooks();
        let req_json = serde_json::to_string(&request).unwrap_or_default();
        invoke_hooks_on_request(&hooks, &req_json).await?;

        let client = Arc::clone(&self.inner);

        let stream = match client.chat_stream(req).await {
            Ok(s) => s,
            Err(e) => {
                let payload = serde_json::json!({ "request": request, "error": e.to_string() });
                invoke_hooks_on_error(&hooks, &payload.to_string()).await;
                return Err(to_napi_err(e));
            }
        };
        match collect_chunk_stream(stream).await {
            Ok(chunks) => {
                let js_chunks: napi::Result<Vec<_>> = chunks.into_iter().map(to_js_value).collect();
                let js_chunks = js_chunks?;
                let resp_val = serde_json::Value::Array(js_chunks.clone());
                let payload = serde_json::json!({ "request": request, "response": resp_val });
                invoke_hooks_on_response(&hooks, &payload.to_string()).await;
                Ok(js_chunks)
            }
            Err(e) => {
                let payload = serde_json::json!({ "request": request, "error": e.to_string() });
                invoke_hooks_on_error(&hooks, &payload.to_string()).await;
                Err(to_napi_err(e))
            }
        }
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
    #[napi]
    pub async fn embed(&self, request: serde_json::Value) -> napi::Result<serde_json::Value> {
        let req: liter_llm::EmbeddingRequest = from_js_value(request.clone())?;

        let hooks = self.snapshot_hooks();
        let req_json = serde_json::to_string(&request).unwrap_or_default();
        invoke_hooks_on_request(&hooks, &req_json).await?;

        let client = Arc::clone(&self.inner);
        match client.embed(req).await {
            Ok(result) => {
                let js_val = to_js_value(&result)?;
                let payload = serde_json::json!({ "request": request, "response": js_val });
                invoke_hooks_on_response(&hooks, &payload.to_string()).await;
                Ok(js_val)
            }
            Err(e) => {
                let payload = serde_json::json!({ "request": request, "error": e.to_string() });
                invoke_hooks_on_error(&hooks, &payload.to_string()).await;
                Err(to_napi_err(e))
            }
        }
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
        let req_marker = serde_json::json!({"action": "listModels"});

        let hooks = self.snapshot_hooks();
        let req_json = req_marker.to_string();
        invoke_hooks_on_request(&hooks, &req_json).await?;

        let client = Arc::clone(&self.inner);
        match client.list_models().await {
            Ok(result) => {
                let js_val = to_js_value(&result)?;
                let payload = serde_json::json!({ "request": req_marker, "response": js_val });
                invoke_hooks_on_response(&hooks, &payload.to_string()).await;
                Ok(js_val)
            }
            Err(e) => {
                let payload = serde_json::json!({ "request": req_marker, "error": e.to_string() });
                invoke_hooks_on_error(&hooks, &payload.to_string()).await;
                Err(to_napi_err(e))
            }
        }
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
        let req: liter_llm::CreateImageRequest = from_js_value(request.clone())?;

        let hooks = self.snapshot_hooks();
        let req_json = serde_json::to_string(&request).unwrap_or_default();
        invoke_hooks_on_request(&hooks, &req_json).await?;

        let client = Arc::clone(&self.inner);
        match client.image_generate(req).await {
            Ok(result) => {
                let js_val = to_js_value(&result)?;
                let payload = serde_json::json!({ "request": request, "response": js_val });
                invoke_hooks_on_response(&hooks, &payload.to_string()).await;
                Ok(js_val)
            }
            Err(e) => {
                let payload = serde_json::json!({ "request": request, "error": e.to_string() });
                invoke_hooks_on_error(&hooks, &payload.to_string()).await;
                Err(to_napi_err(e))
            }
        }
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
    #[napi]
    pub async fn speech(&self, request: serde_json::Value) -> napi::Result<Buffer> {
        let req: liter_llm::CreateSpeechRequest = from_js_value(request.clone())?;

        let hooks = self.snapshot_hooks();
        let req_json = serde_json::to_string(&request).unwrap_or_default();
        invoke_hooks_on_request(&hooks, &req_json).await?;

        let client = Arc::clone(&self.inner);
        match client.speech(req).await {
            Ok(result) => {
                let resp_marker = serde_json::json!({"bytes_length": result.len()});
                let payload = serde_json::json!({ "request": request, "response": resp_marker });
                invoke_hooks_on_response(&hooks, &payload.to_string()).await;
                Ok(result.to_vec().into())
            }
            Err(e) => {
                let payload = serde_json::json!({ "request": request, "error": e.to_string() });
                invoke_hooks_on_error(&hooks, &payload.to_string()).await;
                Err(to_napi_err(e))
            }
        }
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
    #[napi]
    pub async fn transcribe(&self, request: serde_json::Value) -> napi::Result<serde_json::Value> {
        let req: liter_llm::CreateTranscriptionRequest = from_js_value(request.clone())?;

        let hooks = self.snapshot_hooks();
        let req_json = serde_json::to_string(&request).unwrap_or_default();
        invoke_hooks_on_request(&hooks, &req_json).await?;

        let client = Arc::clone(&self.inner);
        match client.transcribe(req).await {
            Ok(result) => {
                let js_val = to_js_value(&result)?;
                let payload = serde_json::json!({ "request": request, "response": js_val });
                invoke_hooks_on_response(&hooks, &payload.to_string()).await;
                Ok(js_val)
            }
            Err(e) => {
                let payload = serde_json::json!({ "request": request, "error": e.to_string() });
                invoke_hooks_on_error(&hooks, &payload.to_string()).await;
                Err(to_napi_err(e))
            }
        }
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
    #[napi]
    pub async fn moderate(&self, request: serde_json::Value) -> napi::Result<serde_json::Value> {
        let req: liter_llm::ModerationRequest = from_js_value(request.clone())?;

        let hooks = self.snapshot_hooks();
        let req_json = serde_json::to_string(&request).unwrap_or_default();
        invoke_hooks_on_request(&hooks, &req_json).await?;

        let client = Arc::clone(&self.inner);
        match client.moderate(req).await {
            Ok(result) => {
                let js_val = to_js_value(&result)?;
                let payload = serde_json::json!({ "request": request, "response": js_val });
                invoke_hooks_on_response(&hooks, &payload.to_string()).await;
                Ok(js_val)
            }
            Err(e) => {
                let payload = serde_json::json!({ "request": request, "error": e.to_string() });
                invoke_hooks_on_error(&hooks, &payload.to_string()).await;
                Err(to_napi_err(e))
            }
        }
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
    #[napi]
    pub async fn rerank(&self, request: serde_json::Value) -> napi::Result<serde_json::Value> {
        let req: liter_llm::RerankRequest = from_js_value(request.clone())?;

        let hooks = self.snapshot_hooks();
        let req_json = serde_json::to_string(&request).unwrap_or_default();
        invoke_hooks_on_request(&hooks, &req_json).await?;

        let client = Arc::clone(&self.inner);
        match client.rerank(req).await {
            Ok(result) => {
                let js_val = to_js_value(&result)?;
                let payload = serde_json::json!({ "request": request, "response": js_val });
                invoke_hooks_on_response(&hooks, &payload.to_string()).await;
                Ok(js_val)
            }
            Err(e) => {
                let payload = serde_json::json!({ "request": request, "error": e.to_string() });
                invoke_hooks_on_error(&hooks, &payload.to_string()).await;
                Err(to_napi_err(e))
            }
        }
    }

    /// Perform a web/document search.
    ///
    /// Accepts a plain JS object matching the search API format.
    /// Returns a `Promise<object>` resolving to a `SearchResponse`.
    ///
    /// ```js
    /// const resp = await client.search({ model: "brave/web-search", query: "rust lang" });
    /// console.log(resp.results);
    /// ```
    #[napi]
    pub async fn search(&self, request: serde_json::Value) -> napi::Result<serde_json::Value> {
        let req: liter_llm::SearchRequest = from_js_value(request.clone())?;

        let hooks = self.snapshot_hooks();
        let req_json = serde_json::to_string(&request).unwrap_or_default();
        invoke_hooks_on_request(&hooks, &req_json).await?;

        let client = Arc::clone(&self.inner);
        match client.search(req).await {
            Ok(result) => {
                let js_val = to_js_value(&result)?;
                let payload = serde_json::json!({ "request": request, "response": js_val });
                invoke_hooks_on_response(&hooks, &payload.to_string()).await;
                Ok(js_val)
            }
            Err(e) => {
                let payload = serde_json::json!({ "request": request, "error": e.to_string() });
                invoke_hooks_on_error(&hooks, &payload.to_string()).await;
                Err(to_napi_err(e))
            }
        }
    }

    /// Extract text from a document via OCR.
    ///
    /// Accepts a plain JS object matching the OCR API format.
    /// Returns a `Promise<object>` resolving to an `OcrResponse`.
    ///
    /// ```js
    /// const resp = await client.ocr({ model: "mistral/mistral-ocr-latest", document: { type: "document_url", url: "..." } });
    /// console.log(resp.pages);
    /// ```
    #[napi]
    pub async fn ocr(&self, request: serde_json::Value) -> napi::Result<serde_json::Value> {
        let req: liter_llm::OcrRequest = from_js_value(request.clone())?;

        let hooks = self.snapshot_hooks();
        let req_json = serde_json::to_string(&request).unwrap_or_default();
        invoke_hooks_on_request(&hooks, &req_json).await?;

        let client = Arc::clone(&self.inner);
        match client.ocr(req).await {
            Ok(result) => {
                let js_val = to_js_value(&result)?;
                let payload = serde_json::json!({ "request": request, "response": js_val });
                invoke_hooks_on_response(&hooks, &payload.to_string()).await;
                Ok(js_val)
            }
            Err(e) => {
                let payload = serde_json::json!({ "request": request, "error": e.to_string() });
                invoke_hooks_on_error(&hooks, &payload.to_string()).await;
                Err(to_napi_err(e))
            }
        }
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
        let req: liter_llm::CreateFileRequest = from_js_value(request.clone())?;

        let hooks = self.snapshot_hooks();
        let req_json = serde_json::to_string(&request).unwrap_or_default();
        invoke_hooks_on_request(&hooks, &req_json).await?;

        let client = Arc::clone(&self.inner);
        match client.create_file(req).await {
            Ok(result) => {
                let js_val = to_js_value(&result)?;
                let payload = serde_json::json!({ "request": request, "response": js_val });
                invoke_hooks_on_response(&hooks, &payload.to_string()).await;
                Ok(js_val)
            }
            Err(e) => {
                let payload = serde_json::json!({ "request": request, "error": e.to_string() });
                invoke_hooks_on_error(&hooks, &payload.to_string()).await;
                Err(to_napi_err(e))
            }
        }
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
        let req_marker = serde_json::json!({"action": "retrieveFile", "fileId": &file_id});

        let hooks = self.snapshot_hooks();
        invoke_hooks_on_request(&hooks, &req_marker.to_string()).await?;

        let client = Arc::clone(&self.inner);
        match client.retrieve_file(&file_id).await {
            Ok(result) => {
                let js_val = to_js_value(&result)?;
                let payload = serde_json::json!({ "request": req_marker, "response": js_val });
                invoke_hooks_on_response(&hooks, &payload.to_string()).await;
                Ok(js_val)
            }
            Err(e) => {
                let payload = serde_json::json!({ "request": req_marker, "error": e.to_string() });
                invoke_hooks_on_error(&hooks, &payload.to_string()).await;
                Err(to_napi_err(e))
            }
        }
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
        let req_marker = serde_json::json!({"action": "deleteFile", "fileId": &file_id});

        let hooks = self.snapshot_hooks();
        invoke_hooks_on_request(&hooks, &req_marker.to_string()).await?;

        let client = Arc::clone(&self.inner);
        match client.delete_file(&file_id).await {
            Ok(result) => {
                let js_val = to_js_value(&result)?;
                let payload = serde_json::json!({ "request": req_marker, "response": js_val });
                invoke_hooks_on_response(&hooks, &payload.to_string()).await;
                Ok(js_val)
            }
            Err(e) => {
                let payload = serde_json::json!({ "request": req_marker, "error": e.to_string() });
                invoke_hooks_on_error(&hooks, &payload.to_string()).await;
                Err(to_napi_err(e))
            }
        }
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
        let req_marker = serde_json::json!({"action": "listFiles", "query": &query});

        let hooks = self.snapshot_hooks();
        invoke_hooks_on_request(&hooks, &req_marker.to_string()).await?;

        let parsed: Option<liter_llm::FileListQuery> = query.map(from_js_value).transpose()?;

        let client = Arc::clone(&self.inner);
        match client.list_files(parsed).await {
            Ok(result) => {
                let js_val = to_js_value(&result)?;
                let payload = serde_json::json!({ "request": req_marker, "response": js_val });
                invoke_hooks_on_response(&hooks, &payload.to_string()).await;
                Ok(js_val)
            }
            Err(e) => {
                let payload = serde_json::json!({ "request": req_marker, "error": e.to_string() });
                invoke_hooks_on_error(&hooks, &payload.to_string()).await;
                Err(to_napi_err(e))
            }
        }
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
        let req_marker = serde_json::json!({"action": "fileContent", "fileId": &file_id});

        let hooks = self.snapshot_hooks();
        invoke_hooks_on_request(&hooks, &req_marker.to_string()).await?;

        let client = Arc::clone(&self.inner);
        match client.file_content(&file_id).await {
            Ok(result) => {
                let resp_marker = serde_json::json!({"bytes_length": result.len()});
                let payload = serde_json::json!({ "request": req_marker, "response": resp_marker });
                invoke_hooks_on_response(&hooks, &payload.to_string()).await;
                Ok(result.to_vec().into())
            }
            Err(e) => {
                let payload = serde_json::json!({ "request": req_marker, "error": e.to_string() });
                invoke_hooks_on_error(&hooks, &payload.to_string()).await;
                Err(to_napi_err(e))
            }
        }
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
        let req: liter_llm::CreateBatchRequest = from_js_value(request.clone())?;

        let hooks = self.snapshot_hooks();
        let req_json = serde_json::to_string(&request).unwrap_or_default();
        invoke_hooks_on_request(&hooks, &req_json).await?;

        let client = Arc::clone(&self.inner);
        match client.create_batch(req).await {
            Ok(result) => {
                let js_val = to_js_value(&result)?;
                let payload = serde_json::json!({ "request": request, "response": js_val });
                invoke_hooks_on_response(&hooks, &payload.to_string()).await;
                Ok(js_val)
            }
            Err(e) => {
                let payload = serde_json::json!({ "request": request, "error": e.to_string() });
                invoke_hooks_on_error(&hooks, &payload.to_string()).await;
                Err(to_napi_err(e))
            }
        }
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
        let req_marker = serde_json::json!({"action": "retrieveBatch", "batchId": &batch_id});

        let hooks = self.snapshot_hooks();
        invoke_hooks_on_request(&hooks, &req_marker.to_string()).await?;

        let client = Arc::clone(&self.inner);
        match client.retrieve_batch(&batch_id).await {
            Ok(result) => {
                let js_val = to_js_value(&result)?;
                let payload = serde_json::json!({ "request": req_marker, "response": js_val });
                invoke_hooks_on_response(&hooks, &payload.to_string()).await;
                Ok(js_val)
            }
            Err(e) => {
                let payload = serde_json::json!({ "request": req_marker, "error": e.to_string() });
                invoke_hooks_on_error(&hooks, &payload.to_string()).await;
                Err(to_napi_err(e))
            }
        }
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
        let req_marker = serde_json::json!({"action": "listBatches", "query": &query});

        let hooks = self.snapshot_hooks();
        invoke_hooks_on_request(&hooks, &req_marker.to_string()).await?;

        let parsed: Option<liter_llm::BatchListQuery> = query.map(from_js_value).transpose()?;

        let client = Arc::clone(&self.inner);
        match client.list_batches(parsed).await {
            Ok(result) => {
                let js_val = to_js_value(&result)?;
                let payload = serde_json::json!({ "request": req_marker, "response": js_val });
                invoke_hooks_on_response(&hooks, &payload.to_string()).await;
                Ok(js_val)
            }
            Err(e) => {
                let payload = serde_json::json!({ "request": req_marker, "error": e.to_string() });
                invoke_hooks_on_error(&hooks, &payload.to_string()).await;
                Err(to_napi_err(e))
            }
        }
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
        let req_marker = serde_json::json!({"action": "cancelBatch", "batchId": &batch_id});

        let hooks = self.snapshot_hooks();
        invoke_hooks_on_request(&hooks, &req_marker.to_string()).await?;

        let client = Arc::clone(&self.inner);
        match client.cancel_batch(&batch_id).await {
            Ok(result) => {
                let js_val = to_js_value(&result)?;
                let payload = serde_json::json!({ "request": req_marker, "response": js_val });
                invoke_hooks_on_response(&hooks, &payload.to_string()).await;
                Ok(js_val)
            }
            Err(e) => {
                let payload = serde_json::json!({ "request": req_marker, "error": e.to_string() });
                invoke_hooks_on_error(&hooks, &payload.to_string()).await;
                Err(to_napi_err(e))
            }
        }
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
        let req: liter_llm::CreateResponseRequest = from_js_value(request.clone())?;

        let hooks = self.snapshot_hooks();
        let req_json = serde_json::to_string(&request).unwrap_or_default();
        invoke_hooks_on_request(&hooks, &req_json).await?;

        let client = Arc::clone(&self.inner);
        match client.create_response(req).await {
            Ok(result) => {
                let js_val = to_js_value(&result)?;
                let payload = serde_json::json!({ "request": request, "response": js_val });
                invoke_hooks_on_response(&hooks, &payload.to_string()).await;
                Ok(js_val)
            }
            Err(e) => {
                let payload = serde_json::json!({ "request": request, "error": e.to_string() });
                invoke_hooks_on_error(&hooks, &payload.to_string()).await;
                Err(to_napi_err(e))
            }
        }
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
        let req_marker = serde_json::json!({"action": "retrieveResponse", "id": &id});

        let hooks = self.snapshot_hooks();
        invoke_hooks_on_request(&hooks, &req_marker.to_string()).await?;

        let client = Arc::clone(&self.inner);
        match client.retrieve_response(&id).await {
            Ok(result) => {
                let js_val = to_js_value(&result)?;
                let payload = serde_json::json!({ "request": req_marker, "response": js_val });
                invoke_hooks_on_response(&hooks, &payload.to_string()).await;
                Ok(js_val)
            }
            Err(e) => {
                let payload = serde_json::json!({ "request": req_marker, "error": e.to_string() });
                invoke_hooks_on_error(&hooks, &payload.to_string()).await;
                Err(to_napi_err(e))
            }
        }
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
        let req_marker = serde_json::json!({"action": "cancelResponse", "id": &id});

        let hooks = self.snapshot_hooks();
        invoke_hooks_on_request(&hooks, &req_marker.to_string()).await?;

        let client = Arc::clone(&self.inner);
        match client.cancel_response(&id).await {
            Ok(result) => {
                let js_val = to_js_value(&result)?;
                let payload = serde_json::json!({ "request": req_marker, "response": js_val });
                invoke_hooks_on_response(&hooks, &payload.to_string()).await;
                Ok(js_val)
            }
            Err(e) => {
                let payload = serde_json::json!({ "request": req_marker, "error": e.to_string() });
                invoke_hooks_on_error(&hooks, &payload.to_string()).await;
                Err(to_napi_err(e))
            }
        }
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
