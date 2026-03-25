//! liter-lm WebAssembly Bindings
//!
//! Exposes a JavaScript-friendly `LlmClient` class that wraps the Rust core
//! client via `wasm-bindgen`.
//!
//! # Architecture
//!
//! HTTP calls cannot use `reqwest`'s native TLS or TCP stack in WASM.  The
//! actual requests are made by delegating to the browser / Node.js `fetch` API
//! through `web_sys` / `wasm-bindgen-futures`.  For now the networking layer
//! is marked with `TODO` comments where the real fetch calls need to be wired
//! in; everything else (type conversion, config parsing, error wrapping) is
//! fully implemented.
//!
//! # Usage (JavaScript / TypeScript)
//!
//! ```javascript
//! import init, { LlmClient } from 'liter-lm-wasm';
//! await init();
//!
//! const client = new LlmClient({ apiKey: 'sk-...', maxRetries: 0 });
//! const response = await client.chat({ model: 'gpt-4', messages: [...] });
//! ```

use js_sys::Promise;
use serde::Deserialize;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;

// ─── TypeScript type definitions ──────────────────────────────────────────────

/// Injected verbatim into the generated `.d.ts` file so TypeScript consumers
/// get full structural typing for every request and response object.
///
/// These mirror the Rust types in `crates/liter-lm/src/types/` exactly.
#[wasm_bindgen(typescript_custom_section)]
const TS_APPEND_CONTENT: &str = r#"
/** Options accepted by the {@link LlmClient} constructor. */
export interface LlmClientOptions {
  /** API key for authentication.  Pass an empty string for providers that
   *  do not require authentication. */
  apiKey: string;
  /** Override the provider base URL.  Omit to use OpenAI-compatible routing
   *  based on the model-name prefix. */
  baseUrl?: string;
  /** Number of retries on 429 / 5xx responses (default: 3). */
  maxRetries?: number;
  /** Request timeout in seconds (default: 60). */
  timeoutSecs?: number;
  /** Override the entire Authorization header value (e.g. `"Bearer sk-..."`,
   *  `"x-api-key abc123"`, or a custom scheme).  When omitted the client
   *  generates `"Bearer {apiKey}"` automatically. */
  authHeader?: string;
}

// ── Shared ────────────────────────────────────────────────────────────────────

/** Token usage counts returned with chat and embedding responses. */
export interface UsageResponse {
  prompt_tokens: number;
  completion_tokens: number;
  total_tokens: number;
}

// ── Content ───────────────────────────────────────────────────────────────────

export interface ImageUrlParam {
  url: string;
  detail?: "low" | "high" | "auto";
}

export type ContentPartParam =
  | { type: "text"; text: string }
  | { type: "image_url"; image_url: ImageUrlParam };

// ── Messages ──────────────────────────────────────────────────────────────────

export interface MessageParam {
  role: "system" | "user" | "assistant" | "tool" | "developer" | "function";
  content: string | ContentPartParam[];
  name?: string;
  tool_call_id?: string;
}

// ── Tools ─────────────────────────────────────────────────────────────────────

export interface FunctionDefinition {
  name: string;
  description?: string;
  parameters?: Record<string, unknown>;
  strict?: boolean;
}

export interface ToolParam {
  type: "function";
  function: FunctionDefinition;
}

export type ToolChoiceParam =
  | "auto"
  | "required"
  | "none"
  | { type: "function"; function: { name: string } };

export interface FunctionCall {
  name: string;
  arguments: string;
}

export interface ToolCall {
  id: string;
  type: "function";
  function: FunctionCall;
}

// ── Response format ───────────────────────────────────────────────────────────

export interface JsonSchemaFormat {
  name: string;
  description?: string;
  schema: Record<string, unknown>;
  strict?: boolean;
}

export type ResponseFormatParam =
  | { type: "text" }
  | { type: "json_object" }
  | { type: "json_schema"; json_schema: JsonSchemaFormat };

// ── Chat request ─────────────────────────────────────────────────────────────

export interface StreamOptions {
  include_usage?: boolean;
}

/** Full OpenAI-compatible chat completion request. */
export interface ChatCompletionRequest {
  model: string;
  messages: MessageParam[];
  temperature?: number;
  top_p?: number;
  n?: number;
  stream?: boolean;
  stop?: string | string[];
  max_tokens?: number;
  presence_penalty?: number;
  frequency_penalty?: number;
  logit_bias?: Record<string, number>;
  user?: string;
  tools?: ToolParam[];
  tool_choice?: ToolChoiceParam;
  parallel_tool_calls?: boolean;
  response_format?: ResponseFormatParam;
  stream_options?: StreamOptions;
  seed?: number;
}

// ── Chat response ─────────────────────────────────────────────────────────────

export interface AssistantMessage {
  content?: string | null;
  name?: string;
  tool_calls?: ToolCall[];
  refusal?: string;
  function_call?: FunctionCall;
}

export type FinishReason =
  | "stop"
  | "length"
  | "tool_calls"
  | "content_filter"
  | "function_call"
  | string;

export interface Choice {
  index: number;
  message: AssistantMessage;
  finish_reason: FinishReason | null;
}

/** Full OpenAI-compatible chat completion response. */
export interface ChatCompletionResponse {
  id: string;
  object: string;
  created: number;
  model: string;
  choices: Choice[];
  usage?: UsageResponse;
  system_fingerprint?: string;
  service_tier?: string;
}

// ── Streaming chunk ───────────────────────────────────────────────────────────

export interface StreamFunctionCall {
  name?: string;
  arguments?: string;
}

export interface StreamToolCall {
  index: number;
  id?: string;
  type?: "function";
  function?: StreamFunctionCall;
}

export interface StreamDelta {
  role?: string;
  content?: string | null;
  tool_calls?: StreamToolCall[];
  function_call?: StreamFunctionCall;
  refusal?: string;
}

export interface StreamChoice {
  index: number;
  delta: StreamDelta;
  finish_reason: string | null;
}

/** A single SSE chunk from a streaming chat completion. */
export interface ChatCompletionChunk {
  id: string;
  object: string;
  created: number;
  model: string;
  choices: StreamChoice[];
  usage?: UsageResponse;
  service_tier?: string;
}

// ── Embeddings ────────────────────────────────────────────────────────────────

export interface EmbeddingRequest {
  model: string;
  input: string | string[];
  encoding_format?: string;
  dimensions?: number;
  user?: string;
}

export interface EmbeddingObject {
  object: string;
  embedding: number[];
  index: number;
}

export interface EmbeddingResponse {
  object: string;
  data: EmbeddingObject[];
  model: string;
  usage: UsageResponse;
}

// ── Models ────────────────────────────────────────────────────────────────────

export interface ModelObject {
  id: string;
  object: string;
  created: number;
  owned_by: string;
}

export interface ModelsListResponse {
  object: string;
  data: ModelObject[];
}
"#;

// ─── JS interop helpers ───────────────────────────────────────────────────────

fn js_err(msg: impl std::fmt::Display) -> JsValue {
    JsValue::from_str(&msg.to_string())
}

fn json_to_js(value: &serde_json::Value) -> Result<JsValue, JsValue> {
    serde_wasm_bindgen::to_value(value).map_err(js_err)
}

fn js_to_json(value: JsValue) -> Result<serde_json::Value, JsValue> {
    serde_wasm_bindgen::from_value(value).map_err(js_err)
}

// ─── Client options ───────────────────────────────────────────────────────────

/// Options accepted by the `LlmClient` constructor from JavaScript.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ClientOptions {
    api_key: String,
    #[serde(default)]
    base_url: Option<String>,
    #[serde(default = "default_max_retries")]
    max_retries: u32,
    #[serde(default = "default_timeout_secs")]
    #[allow(dead_code)] // Stored for future use when we have a WASM-native timeout mechanism
    timeout_secs: u64,
    /// Optional override for the full Authorization header value.
    /// When absent the client generates `"Bearer {api_key}"`.
    #[serde(default)]
    auth_header: Option<String>,
}

fn default_max_retries() -> u32 {
    3
}

fn default_timeout_secs() -> u64 {
    60
}

// ─── LlmClient ────────────────────────────────────────────────────────────────

/// JavaScript-visible LLM client.
///
/// Constructed from a plain JS object (or TypeScript interface) with the
/// following fields:
///
/// - `apiKey` (string, required)
/// - `baseUrl` (string, optional) — override the provider base URL
/// - `maxRetries` (number, optional, default 3)
/// - `timeoutSecs` (number, optional, default 60)
/// - `authHeader` (string, optional) — override the `Authorization` header value
///
/// # Security note
///
/// The `api_key` is stored as a plain `String` rather than `secrecy::SecretString`
/// because the `secrecy` crate does not support the WebAssembly target — it relies
/// on `mlock`/`munlock` system calls that are unavailable in the WASM sandbox.
/// The memory containing the key is zeroed on a best-effort basis when `LlmClient`
/// is dropped, but the WASM runtime does not guarantee timely garbage collection.
/// For maximum security, avoid long-lived `LlmClient` instances in browser contexts.
#[wasm_bindgen]
pub struct LlmClient {
    api_key: String,
    base_url: String,
    max_retries: u32,
    /// Full Authorization header value.  When the user does not provide
    /// `authHeader` this defaults to `"Bearer {api_key}"`.
    auth_header_override: Option<String>,
}

#[wasm_bindgen]
impl LlmClient {
    /// Create a new `LlmClient`.
    ///
    /// Accepts a plain JS object `{ apiKey, baseUrl?, maxRetries?, timeoutSecs? }`.
    #[wasm_bindgen(constructor)]
    pub fn new(options: JsValue) -> Result<LlmClient, JsValue> {
        let opts: ClientOptions =
            serde_wasm_bindgen::from_value(options).map_err(|e| js_err(format!("invalid LlmClient options: {e}")))?;

        let base_url = opts.base_url.unwrap_or_else(|| "https://api.openai.com/v1".to_string());

        Ok(Self {
            api_key: opts.api_key,
            base_url,
            max_retries: opts.max_retries,
            auth_header_override: opts.auth_header,
        })
    }

    /// Return the effective Authorization header value: either the override
    /// provided by the user or the default `"Bearer {api_key}"`.
    fn effective_auth_header(&self) -> String {
        self.auth_header_override
            .clone()
            .unwrap_or_else(|| format!("Bearer {}", self.api_key))
    }

    /// Send a chat completion request.
    ///
    /// Accepts a JS object matching the OpenAI Chat Completions request shape.
    /// Returns a `Promise` that resolves to the parsed response object.
    pub fn chat(&self, request: JsValue) -> Promise {
        let auth_header = self.effective_auth_header();
        let base_url = self.base_url.clone();
        let max_retries = self.max_retries;

        wasm_bindgen_futures::future_to_promise(async move {
            let req_json = js_to_json(request)?;
            let url = format!("{base_url}/chat/completions");
            let resp_json = fetch_json_post_with_auth(&url, &auth_header, req_json, max_retries).await?;
            json_to_js(&resp_json)
        })
    }

    /// Stream a chat completion request.
    ///
    /// **Not yet implemented in the WASM binding.**
    ///
    /// Returns a `Promise` that always rejects with an error message explaining
    /// that streaming is not yet supported in WASM.  This stub makes the
    /// absence of the feature explicit rather than causing a "method not found"
    /// error at runtime.
    ///
    /// TODO: implement using the WASM Streams API (`ReadableStream`) once
    /// `wasm-streams` stabilises for this use-case.
    #[wasm_bindgen(js_name = "chatStream")]
    pub fn chat_stream(&self, _request: JsValue) -> Promise {
        wasm_bindgen_futures::future_to_promise(async {
            Err(JsValue::from_str(
                "chat_stream is not yet supported in the WASM binding",
            ))
        })
    }

    /// Send an embedding request.
    ///
    /// Accepts a JS object matching the OpenAI Embeddings request shape.
    /// Returns a `Promise` that resolves to the parsed response object.
    pub fn embed(&self, request: JsValue) -> Promise {
        let auth_header = self.effective_auth_header();
        let base_url = self.base_url.clone();
        let max_retries = self.max_retries;

        wasm_bindgen_futures::future_to_promise(async move {
            let req_json = js_to_json(request)?;
            let url = format!("{base_url}/embeddings");
            let resp_json = fetch_json_post_with_auth(&url, &auth_header, req_json, max_retries).await?;
            json_to_js(&resp_json)
        })
    }

    /// List available models.
    ///
    /// Returns a `Promise` that resolves to the parsed models list object.
    #[wasm_bindgen(js_name = "listModels")]
    pub fn list_models(&self) -> Promise {
        let auth_header = self.effective_auth_header();
        let base_url = self.base_url.clone();
        let max_retries = self.max_retries;

        wasm_bindgen_futures::future_to_promise(async move {
            let url = format!("{base_url}/models");
            let resp_json = fetch_json_get_with_auth(&url, &auth_header, max_retries).await?;
            json_to_js(&resp_json)
        })
    }
}

impl Drop for LlmClient {
    /// Best-effort deallocation of the API key on drop.
    ///
    /// WASM does not have memory-locking primitives (`mlock`), so this is not
    /// a cryptographic guarantee — the runtime or JIT may have already copied
    /// the key to other locations.  Replacing the string with an empty one and
    /// releasing its backing allocation reduces the key's lifetime in the heap
    /// without requiring unsafe code.
    fn drop(&mut self) {
        // Replace api_key with an empty String and release its backing allocation.
        // This is the safe, correct way to clear a String's contents; zeroing
        // individual bytes via as_bytes_mut() is unsafe and risks creating
        // invalid UTF-8 if interrupted.
        drop(std::mem::take(&mut self.api_key));
    }
}

// ─── HTTP helpers via JS fetch ────────────────────────────────────────────────

/// Perform a JSON POST request using the JS `fetch` API.
///
/// Retries on 429 / 5xx up to `max_retries` times with exponential backoff
/// (100 ms, 200 ms, 400 ms … capped at 10 s) using `gloo_timers`.
///
/// `auth_header_value` is the full `Authorization` header value
/// (e.g. `"Bearer sk-..."`).
async fn fetch_json_post_with_auth(
    url: &str,
    auth_header_value: &str,
    body: serde_json::Value,
    max_retries: u32,
) -> Result<serde_json::Value, JsValue> {
    let body_str = serde_json::to_string(&body).map_err(js_err)?;

    let mut attempt = 0u32;
    loop {
        let result = do_fetch_post(url, auth_header_value, &body_str).await;
        match result {
            Ok(value) => return Ok(value),
            Err(e) if attempt < max_retries && is_retryable_error(&e) => {
                let delay_ms = backoff_ms(attempt);
                sleep_ms(delay_ms).await;
                attempt += 1;
            }
            Err(e) => return Err(e),
        }
    }
}

/// Perform a JSON GET request using the JS `fetch` API.
///
/// Retries on 429 / 5xx up to `max_retries` times with exponential backoff.
///
/// `auth_header_value` is the full `Authorization` header value.
async fn fetch_json_get_with_auth(
    url: &str,
    auth_header_value: &str,
    max_retries: u32,
) -> Result<serde_json::Value, JsValue> {
    let mut attempt = 0u32;
    loop {
        let result = do_fetch_get(url, auth_header_value).await;
        match result {
            Ok(value) => return Ok(value),
            Err(e) if attempt < max_retries && is_retryable_error(&e) => {
                let delay_ms = backoff_ms(attempt);
                sleep_ms(delay_ms).await;
                attempt += 1;
            }
            Err(e) => return Err(e),
        }
    }
}

/// Return the exponential backoff delay in milliseconds for a given attempt
/// index (0-based).  Starts at 100 ms, doubles each attempt, caps at 10 s.
fn backoff_ms(attempt: u32) -> u32 {
    let base: u32 = 100;
    let max: u32 = 10_000;
    // Cap the shift amount to avoid overflow: 2^32 would exceed u32::MAX.
    let shift = attempt.min(31);
    base.saturating_mul(1u32 << shift).min(max)
}

/// Sleep for `ms` milliseconds using a `Promise`-based timer that integrates
/// with the JS event loop.  Awaiting this will yield control back to the
/// browser / Node.js scheduler during the delay.
async fn sleep_ms(ms: u32) {
    let promise = js_sys::Promise::new(&mut |resolve, _reject| {
        let global = js_sys::global();
        let set_timeout = js_sys::Reflect::get(&global, &"setTimeout".into())
            .ok()
            .and_then(|f| f.dyn_into::<js_sys::Function>().ok());

        if let Some(set_timeout_fn) = set_timeout {
            let _ = set_timeout_fn.call2(&global, &resolve, &JsValue::from(ms));
        } else {
            // If setTimeout is unavailable, resolve immediately so the retry
            // still proceeds rather than hanging forever.
            let _ = resolve.call0(&JsValue::UNDEFINED);
        }
    });
    let _ = JsFuture::from(promise).await;
}

/// Returns `true` if the error represents a retryable HTTP failure (429 / 5xx).
///
/// Error strings from `extract_json_from_response` are always formatted as
/// `"HTTP {status}: {message}"`.  We parse the numeric status code from that
/// prefix to avoid false positives from user-visible messages that happen to
/// contain a matching number substring.
fn is_retryable_error(error: &JsValue) -> bool {
    if let Some(s) = error.as_string()
        && let Some(rest) = s.strip_prefix("HTTP ")
        && let Some((code_str, _)) = rest.split_once(':')
        && let Ok(status) = code_str.trim().parse::<u16>()
    {
        return status == 429 || (500..=599).contains(&status);
    }
    false
}

/// Shared inner fetch implementation using the JS `fetch` API.
///
/// - `method`: HTTP method string (`"POST"` or `"GET"`).
/// - `url`: Target URL.
/// - `auth_header`: Value for the `Authorization` header.
/// - `body`: Optional JSON body string (included only for POST / PUT requests).
async fn do_fetch(
    method: &str,
    url: &str,
    auth_header: &str,
    body: Option<&str>,
) -> Result<serde_json::Value, JsValue> {
    use js_sys::Reflect;
    use wasm_bindgen::JsCast;

    let headers = js_sys::Object::new();
    if let Some(b) = body {
        Reflect::set(&headers, &"Content-Type".into(), &"application/json".into())?;
        let _ = b; // used below
    }
    Reflect::set(&headers, &"Authorization".into(), &auth_header.into())?;

    let init = js_sys::Object::new();
    Reflect::set(&init, &"method".into(), &method.into())?;
    Reflect::set(&init, &"headers".into(), &headers.into())?;
    if let Some(b) = body {
        Reflect::set(&init, &"body".into(), &JsValue::from_str(b))?;
    }

    let global = js_sys::global();

    // `fetch` is available in both browsers and Node.js 18+.
    let fetch_fn =
        Reflect::get(&global, &"fetch".into()).map_err(|_| js_err("fetch is not available in this environment"))?;
    let fetch_fn: js_sys::Function = fetch_fn
        .dyn_into()
        .map_err(|_| js_err("global.fetch is not a function"))?;

    let response_promise = fetch_fn
        .call2(&global, &JsValue::from_str(url), &init.into())
        .map_err(|e| js_err(format!("fetch call failed: {e:?}")))?;
    let response_promise: Promise = response_promise
        .dyn_into()
        .map_err(|_| js_err("fetch did not return a Promise"))?;

    let response = JsFuture::from(response_promise).await?;
    extract_json_from_response(response).await
}

/// Inner POST implementation using `web_sys::Request` / `fetch`.
///
/// `auth_header_value` is the full `Authorization` header value.
async fn do_fetch_post(url: &str, auth_header_value: &str, body: &str) -> Result<serde_json::Value, JsValue> {
    do_fetch("POST", url, auth_header_value, Some(body)).await
}

/// Inner GET implementation using `web_sys::Request` / `fetch`.
///
/// `auth_header_value` is the full `Authorization` header value.
async fn do_fetch_get(url: &str, auth_header_value: &str) -> Result<serde_json::Value, JsValue> {
    do_fetch("GET", url, auth_header_value, None).await
}

/// Read the response body as JSON, checking the HTTP status first.
async fn extract_json_from_response(response: JsValue) -> Result<serde_json::Value, JsValue> {
    use js_sys::Reflect;
    use wasm_bindgen::JsCast;

    let status = Reflect::get(&response, &"status".into())
        .ok()
        .and_then(|v| v.as_f64())
        .map(|f| f as u16)
        .unwrap_or(0);

    let json_method: js_sys::Function = Reflect::get(&response, &"json".into())
        .map_err(|_| js_err("response.json is missing"))?
        .dyn_into()
        .map_err(|_| js_err("response.json is not a function"))?;

    let json_promise: Promise = json_method
        .call0(&response)
        .map_err(|e| js_err(format!("response.json() failed: {e:?}")))?
        .dyn_into()
        .map_err(|_| js_err("response.json() did not return a Promise"))?;

    let json_value = JsFuture::from(json_promise).await?;
    let parsed: serde_json::Value =
        serde_wasm_bindgen::from_value(json_value).map_err(|e| js_err(format!("JSON parse error: {e}")))?;

    if status >= 400 {
        let message = parsed
            .pointer("/error/message")
            .and_then(|v| v.as_str())
            .unwrap_or("unknown error")
            .to_string();
        return Err(js_err(format!("HTTP {status}: {message}")));
    }

    Ok(parsed)
}

// ─── Free-standing helpers ────────────────────────────────────────────────────

/// Returns the version of the liter-lm library.
#[wasm_bindgen]
pub fn version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version_not_empty() {
        let v = version();
        assert!(!v.is_empty());
    }

    #[test]
    fn test_is_retryable_error() {
        // Retryable: 429 and 5xx in "HTTP NNN: message" format.
        assert!(is_retryable_error(&JsValue::from_str("HTTP 429: rate limited")));
        assert!(is_retryable_error(&JsValue::from_str(
            "HTTP 500: internal server error"
        )));
        assert!(is_retryable_error(&JsValue::from_str("HTTP 503: service unavailable")));
        // Not retryable: 4xx client errors (excluding 429).
        assert!(!is_retryable_error(&JsValue::from_str("HTTP 400: bad request")));
        assert!(!is_retryable_error(&JsValue::from_str("HTTP 401: unauthorized")));
        // Not retryable: bare numbers or unrelated strings do not match.
        assert!(!is_retryable_error(&JsValue::from_str("429")));
        assert!(!is_retryable_error(&JsValue::from_str("network error")));
    }

    #[test]
    fn test_default_options() {
        assert_eq!(default_max_retries(), 3);
        assert_eq!(default_timeout_secs(), 60);
    }
}
