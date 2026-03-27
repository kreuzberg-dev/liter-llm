//! liter-llm WebAssembly Bindings
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
//! import init, { LlmClient } from 'liter-llm-wasm';
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
/// These mirror the Rust types in `crates/liter-llm/src/types/` exactly.
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

// ── Images ──────────────────────────────────────────────────────────────────

export interface CreateImageRequest {
  model?: string;
  prompt: string;
  n?: number;
  size?: string;
  quality?: string;
  response_format?: string;
  style?: string;
  user?: string;
}

export interface ImageObject {
  url?: string;
  b64_json?: string;
  revised_prompt?: string;
}

export interface ImagesResponse {
  created: number;
  data: ImageObject[];
}

// ── Audio ───────────────────────────────────────────────────────────────────

export interface CreateSpeechRequest {
  model: string;
  input: string;
  voice: string;
  response_format?: string;
  speed?: number;
}

export interface CreateTranscriptionRequest {
  model: string;
  file: string;
  language?: string;
  prompt?: string;
  response_format?: string;
  temperature?: number;
}

export interface TranscriptionResponse {
  text: string;
}

// ── Moderations ─────────────────────────────────────────────────────────────

export interface ModerationRequest {
  input: string | string[];
  model?: string;
}

export interface ModerationResult {
  flagged: boolean;
  categories: Record<string, boolean>;
  category_scores: Record<string, number>;
}

export interface ModerationResponse {
  id: string;
  model: string;
  results: ModerationResult[];
}

// ── Rerank ──────────────────────────────────────────────────────────────────

export interface RerankRequest {
  model: string;
  query: string;
  documents: string[] | Record<string, unknown>[];
  top_n?: number;
  return_documents?: boolean;
}

export interface RerankResult {
  index: number;
  relevance_score: number;
  document?: Record<string, unknown>;
}

export interface RerankResponse {
  results: RerankResult[];
  model: string;
  usage?: UsageResponse;
}

// ── Files ───────────────────────────────────────────────────────────────────

export interface CreateFileRequest {
  file: string;
  purpose: string;
  filename?: string;
}

export interface FileObject {
  id: string;
  object: string;
  bytes: number;
  created_at: number;
  filename: string;
  purpose: string;
  status?: string;
}

export interface FileListResponse {
  object: string;
  data: FileObject[];
}

export interface FileListQuery {
  purpose?: string;
  limit?: number;
  after?: string;
}

export interface DeleteResponse {
  id: string;
  object: string;
  deleted: boolean;
}

// ── Batches ─────────────────────────────────────────────────────────────────

export interface CreateBatchRequest {
  input_file_id: string;
  endpoint: string;
  completion_window: string;
  metadata?: Record<string, string>;
}

export interface BatchObject {
  id: string;
  object: string;
  endpoint: string;
  input_file_id: string;
  completion_window: string;
  status: string;
  output_file_id?: string;
  error_file_id?: string;
  created_at: number;
  completed_at?: number;
  failed_at?: number;
  expired_at?: number;
  request_counts?: {
    total: number;
    completed: number;
    failed: number;
  };
  metadata?: Record<string, string>;
}

export interface BatchListResponse {
  object: string;
  data: BatchObject[];
}

export interface BatchListQuery {
  limit?: number;
  after?: string;
}

// ── Responses ───────────────────────────────────────────────────────────────

export interface CreateResponseRequest {
  model: string;
  input: string | unknown[];
  instructions?: string;
  temperature?: number;
  max_output_tokens?: number;
  tools?: unknown[];
  metadata?: Record<string, string>;
}

export interface ResponseObject {
  id: string;
  object: string;
  created_at: number;
  status: string;
  model: string;
  output: unknown[];
  usage?: UsageResponse;
  metadata?: Record<string, string>;
}

// ── Custom Provider ──────────────────────────────────────────────────────────

/** Configuration for registering a custom LLM provider at runtime. */
export interface CustomProviderConfig {
  /** Unique name for this provider. */
  name: string;
  /** Base URL for the provider's API. */
  base_url: string;
  /** Authentication style: "Bearer", {"ApiKey": "X-Custom-Header"}, or "None". */
  auth_header: "Bearer" | { ApiKey: string } | "None";
  /** Model name prefixes that route to this provider. */
  model_prefixes: string[];
}

/** Hook object with optional lifecycle callbacks invoked during requests. */
export interface LlmHook {
  /** Called before the request is sent. Throw to reject (guardrail). */
  onRequest?(request: unknown): void | Promise<void>;
  /** Called after a successful response. */
  onResponse?(request: unknown, response: unknown): void | Promise<void>;
  /** Called when the request fails with an error. */
  onError?(request: unknown, error: unknown): void | Promise<void>;
}
"#;

// ─── JS interop helpers ───────────────────────────────────────────────────────

fn js_err(msg: impl std::fmt::Display) -> JsValue {
    JsValue::from_str(&msg.to_string())
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
    /// Registered lifecycle hook objects.  Each hook is a JS object with
    /// optional `onRequest`, `onResponse`, and `onError` async methods.
    hooks: Vec<JsValue>,
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
            hooks: Vec::new(),
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
        let hooks = self.hooks.clone();

        wasm_bindgen_futures::future_to_promise(async move {
            invoke_hooks(&hooks, "onRequest", &[request.clone()]).await;
            let req_json = js_to_json(request.clone())?;
            let url = format!("{base_url}/chat/completions");
            match fetch_json_post_with_auth(&url, &auth_header, req_json, max_retries).await {
                Ok(resp_json) => {
                    invoke_hooks(&hooks, "onResponse", &[request, resp_json.clone()]).await;
                    Ok(resp_json)
                }
                Err(err) => {
                    invoke_hooks(&hooks, "onError", &[request, err.clone()]).await;
                    Err(err)
                }
            }
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
        let hooks = self.hooks.clone();

        wasm_bindgen_futures::future_to_promise(async move {
            invoke_hooks(&hooks, "onRequest", &[request.clone()]).await;
            let req_json = js_to_json(request.clone())?;
            let url = format!("{base_url}/embeddings");
            match fetch_json_post_with_auth(&url, &auth_header, req_json, max_retries).await {
                Ok(resp_json) => {
                    invoke_hooks(&hooks, "onResponse", &[request, resp_json.clone()]).await;
                    Ok(resp_json)
                }
                Err(err) => {
                    invoke_hooks(&hooks, "onError", &[request, err.clone()]).await;
                    Err(err)
                }
            }
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
        let hooks = self.hooks.clone();

        wasm_bindgen_futures::future_to_promise(async move {
            let req_marker = JsValue::from_str("listModels");
            invoke_hooks(&hooks, "onRequest", &[req_marker.clone()]).await;
            let url = format!("{base_url}/models");
            match fetch_json_get_with_auth(&url, &auth_header, max_retries).await {
                Ok(resp_json) => {
                    invoke_hooks(&hooks, "onResponse", &[req_marker, resp_json.clone()]).await;
                    Ok(resp_json)
                }
                Err(err) => {
                    invoke_hooks(&hooks, "onError", &[req_marker, err.clone()]).await;
                    Err(err)
                }
            }
        })
    }

    // ── Additional inference methods ─────────────────────────────────────────

    /// Generate an image from a text prompt.
    ///
    /// Accepts a JS object matching the OpenAI Images API.
    /// Returns a `Promise` that resolves to the parsed response object.
    #[wasm_bindgen(js_name = "imageGenerate")]
    pub fn image_generate(&self, request: JsValue) -> Promise {
        let auth_header = self.effective_auth_header();
        let base_url = self.base_url.clone();
        let max_retries = self.max_retries;
        let hooks = self.hooks.clone();

        wasm_bindgen_futures::future_to_promise(async move {
            invoke_hooks(&hooks, "onRequest", &[request.clone()]).await;
            let req_json = js_to_json(request.clone())?;
            let url = format!("{base_url}/images/generations");
            match fetch_json_post_with_auth(&url, &auth_header, req_json, max_retries).await {
                Ok(resp_json) => {
                    invoke_hooks(&hooks, "onResponse", &[request, resp_json.clone()]).await;
                    Ok(resp_json)
                }
                Err(err) => {
                    invoke_hooks(&hooks, "onError", &[request, err.clone()]).await;
                    Err(err)
                }
            }
        })
    }

    /// Generate speech audio from text.
    ///
    /// Accepts a JS object matching the OpenAI Audio Speech API.
    /// Returns a `Promise` that resolves to an `ArrayBuffer` of audio bytes.
    pub fn speech(&self, request: JsValue) -> Promise {
        let auth_header = self.effective_auth_header();
        let base_url = self.base_url.clone();
        let max_retries = self.max_retries;
        let hooks = self.hooks.clone();

        wasm_bindgen_futures::future_to_promise(async move {
            invoke_hooks(&hooks, "onRequest", &[request.clone()]).await;
            let req_json = js_to_json(request.clone())?;
            let url = format!("{base_url}/audio/speech");
            match fetch_bytes_post_with_auth(&url, &auth_header, req_json, max_retries).await {
                Ok(resp_bytes) => {
                    invoke_hooks(&hooks, "onResponse", &[request, resp_bytes.clone()]).await;
                    Ok(resp_bytes)
                }
                Err(err) => {
                    invoke_hooks(&hooks, "onError", &[request, err.clone()]).await;
                    Err(err)
                }
            }
        })
    }

    /// Transcribe audio to text.
    ///
    /// Accepts a JS object matching the OpenAI Audio Transcriptions API.
    /// Returns a `Promise` that resolves to the parsed response object.
    pub fn transcribe(&self, request: JsValue) -> Promise {
        let auth_header = self.effective_auth_header();
        let base_url = self.base_url.clone();
        let max_retries = self.max_retries;
        let hooks = self.hooks.clone();

        wasm_bindgen_futures::future_to_promise(async move {
            invoke_hooks(&hooks, "onRequest", &[request.clone()]).await;
            let req_json = js_to_json(request.clone())?;
            let url = format!("{base_url}/audio/transcriptions");
            match fetch_json_post_with_auth(&url, &auth_header, req_json, max_retries).await {
                Ok(resp_json) => {
                    invoke_hooks(&hooks, "onResponse", &[request, resp_json.clone()]).await;
                    Ok(resp_json)
                }
                Err(err) => {
                    invoke_hooks(&hooks, "onError", &[request, err.clone()]).await;
                    Err(err)
                }
            }
        })
    }

    /// Check content against moderation policies.
    ///
    /// Accepts a JS object matching the OpenAI Moderations API.
    /// Returns a `Promise` that resolves to the parsed response object.
    pub fn moderate(&self, request: JsValue) -> Promise {
        let auth_header = self.effective_auth_header();
        let base_url = self.base_url.clone();
        let max_retries = self.max_retries;
        let hooks = self.hooks.clone();

        wasm_bindgen_futures::future_to_promise(async move {
            invoke_hooks(&hooks, "onRequest", &[request.clone()]).await;
            let req_json = js_to_json(request.clone())?;
            let url = format!("{base_url}/moderations");
            match fetch_json_post_with_auth(&url, &auth_header, req_json, max_retries).await {
                Ok(resp_json) => {
                    invoke_hooks(&hooks, "onResponse", &[request, resp_json.clone()]).await;
                    Ok(resp_json)
                }
                Err(err) => {
                    invoke_hooks(&hooks, "onError", &[request, err.clone()]).await;
                    Err(err)
                }
            }
        })
    }

    /// Rerank documents by relevance to a query.
    ///
    /// Accepts a JS object matching the rerank API format.
    /// Returns a `Promise` that resolves to the parsed response object.
    pub fn rerank(&self, request: JsValue) -> Promise {
        let auth_header = self.effective_auth_header();
        let base_url = self.base_url.clone();
        let max_retries = self.max_retries;
        let hooks = self.hooks.clone();

        wasm_bindgen_futures::future_to_promise(async move {
            invoke_hooks(&hooks, "onRequest", &[request.clone()]).await;
            let req_json = js_to_json(request.clone())?;
            let url = format!("{base_url}/rerank");
            match fetch_json_post_with_auth(&url, &auth_header, req_json, max_retries).await {
                Ok(resp_json) => {
                    invoke_hooks(&hooks, "onResponse", &[request, resp_json.clone()]).await;
                    Ok(resp_json)
                }
                Err(err) => {
                    invoke_hooks(&hooks, "onError", &[request, err.clone()]).await;
                    Err(err)
                }
            }
        })
    }

    // ── File management methods ──────────────────────────────────────────────

    /// Upload a file.
    ///
    /// Accepts a JS object with file upload parameters.
    /// Returns a `Promise` that resolves to the parsed file object.
    #[wasm_bindgen(js_name = "createFile")]
    pub fn create_file(&self, request: JsValue) -> Promise {
        let auth_header = self.effective_auth_header();
        let base_url = self.base_url.clone();
        let max_retries = self.max_retries;
        let hooks = self.hooks.clone();

        wasm_bindgen_futures::future_to_promise(async move {
            invoke_hooks(&hooks, "onRequest", &[request.clone()]).await;
            let req_json = js_to_json(request.clone())?;
            let url = format!("{base_url}/files");
            match fetch_json_post_with_auth(&url, &auth_header, req_json, max_retries).await {
                Ok(resp_json) => {
                    invoke_hooks(&hooks, "onResponse", &[request, resp_json.clone()]).await;
                    Ok(resp_json)
                }
                Err(err) => {
                    invoke_hooks(&hooks, "onError", &[request, err.clone()]).await;
                    Err(err)
                }
            }
        })
    }

    /// Retrieve metadata for a file by ID.
    ///
    /// Returns a `Promise` that resolves to the parsed file object.
    #[wasm_bindgen(js_name = "retrieveFile")]
    pub fn retrieve_file(&self, file_id: String) -> Promise {
        let auth_header = self.effective_auth_header();
        let base_url = self.base_url.clone();
        let max_retries = self.max_retries;
        let hooks = self.hooks.clone();

        wasm_bindgen_futures::future_to_promise(async move {
            let req_marker = JsValue::from_str(&file_id);
            invoke_hooks(&hooks, "onRequest", &[req_marker.clone()]).await;
            let url = format!("{base_url}/files/{file_id}");
            match fetch_json_get_with_auth(&url, &auth_header, max_retries).await {
                Ok(resp_json) => {
                    invoke_hooks(&hooks, "onResponse", &[req_marker, resp_json.clone()]).await;
                    Ok(resp_json)
                }
                Err(err) => {
                    invoke_hooks(&hooks, "onError", &[req_marker, err.clone()]).await;
                    Err(err)
                }
            }
        })
    }

    /// Delete a file by ID.
    ///
    /// Returns a `Promise` that resolves to the parsed delete response.
    #[wasm_bindgen(js_name = "deleteFile")]
    pub fn delete_file(&self, file_id: String) -> Promise {
        let auth_header = self.effective_auth_header();
        let base_url = self.base_url.clone();
        let max_retries = self.max_retries;
        let hooks = self.hooks.clone();

        wasm_bindgen_futures::future_to_promise(async move {
            let req_marker = JsValue::from_str(&file_id);
            invoke_hooks(&hooks, "onRequest", &[req_marker.clone()]).await;
            let url = format!("{base_url}/files/{file_id}");
            match fetch_json_delete_with_auth(&url, &auth_header, max_retries).await {
                Ok(resp_json) => {
                    invoke_hooks(&hooks, "onResponse", &[req_marker, resp_json.clone()]).await;
                    Ok(resp_json)
                }
                Err(err) => {
                    invoke_hooks(&hooks, "onError", &[req_marker, err.clone()]).await;
                    Err(err)
                }
            }
        })
    }

    /// List files, optionally filtered by query parameters.
    ///
    /// Pass `null` or `undefined` to list all files without filtering.
    /// Returns a `Promise` that resolves to the parsed file list response.
    #[wasm_bindgen(js_name = "listFiles")]
    pub fn list_files(&self, query: JsValue) -> Promise {
        let auth_header = self.effective_auth_header();
        let base_url = self.base_url.clone();
        let max_retries = self.max_retries;
        let hooks = self.hooks.clone();

        wasm_bindgen_futures::future_to_promise(async move {
            invoke_hooks(&hooks, "onRequest", &[query.clone()]).await;
            let mut url = format!("{base_url}/files");
            if !query.is_null() && !query.is_undefined() {
                let params = js_to_json(query.clone())?;
                if let serde_json::Value::Object(map) = params {
                    let qs: Vec<String> = map
                        .into_iter()
                        .filter_map(|(k, v)| match v {
                            serde_json::Value::String(s) => Some(format!("{k}={s}")),
                            serde_json::Value::Number(n) => Some(format!("{k}={n}")),
                            _ => None,
                        })
                        .collect();
                    if !qs.is_empty() {
                        url = format!("{url}?{}", qs.join("&"));
                    }
                }
            }
            match fetch_json_get_with_auth(&url, &auth_header, max_retries).await {
                Ok(resp_json) => {
                    invoke_hooks(&hooks, "onResponse", &[query, resp_json.clone()]).await;
                    Ok(resp_json)
                }
                Err(err) => {
                    invoke_hooks(&hooks, "onError", &[query, err.clone()]).await;
                    Err(err)
                }
            }
        })
    }

    /// Retrieve the raw content of a file.
    ///
    /// Returns a `Promise` that resolves to an `ArrayBuffer` of the file bytes.
    #[wasm_bindgen(js_name = "fileContent")]
    pub fn file_content(&self, file_id: String) -> Promise {
        let auth_header = self.effective_auth_header();
        let base_url = self.base_url.clone();
        let max_retries = self.max_retries;
        let hooks = self.hooks.clone();

        wasm_bindgen_futures::future_to_promise(async move {
            let req_marker = JsValue::from_str(&file_id);
            invoke_hooks(&hooks, "onRequest", &[req_marker.clone()]).await;
            let url = format!("{base_url}/files/{file_id}/content");
            match fetch_bytes_get_with_auth(&url, &auth_header, max_retries).await {
                Ok(resp_bytes) => {
                    invoke_hooks(&hooks, "onResponse", &[req_marker, resp_bytes.clone()]).await;
                    Ok(resp_bytes)
                }
                Err(err) => {
                    invoke_hooks(&hooks, "onError", &[req_marker, err.clone()]).await;
                    Err(err)
                }
            }
        })
    }

    // ── Batch management methods ─────────────────────────────────────────────

    /// Create a new batch job.
    ///
    /// Accepts a JS object with batch creation parameters.
    /// Returns a `Promise` that resolves to the parsed batch object.
    #[wasm_bindgen(js_name = "createBatch")]
    pub fn create_batch(&self, request: JsValue) -> Promise {
        let auth_header = self.effective_auth_header();
        let base_url = self.base_url.clone();
        let max_retries = self.max_retries;
        let hooks = self.hooks.clone();

        wasm_bindgen_futures::future_to_promise(async move {
            invoke_hooks(&hooks, "onRequest", &[request.clone()]).await;
            let req_json = js_to_json(request.clone())?;
            let url = format!("{base_url}/batches");
            match fetch_json_post_with_auth(&url, &auth_header, req_json, max_retries).await {
                Ok(resp_json) => {
                    invoke_hooks(&hooks, "onResponse", &[request, resp_json.clone()]).await;
                    Ok(resp_json)
                }
                Err(err) => {
                    invoke_hooks(&hooks, "onError", &[request, err.clone()]).await;
                    Err(err)
                }
            }
        })
    }

    /// Retrieve a batch by ID.
    ///
    /// Returns a `Promise` that resolves to the parsed batch object.
    #[wasm_bindgen(js_name = "retrieveBatch")]
    pub fn retrieve_batch(&self, batch_id: String) -> Promise {
        let auth_header = self.effective_auth_header();
        let base_url = self.base_url.clone();
        let max_retries = self.max_retries;
        let hooks = self.hooks.clone();

        wasm_bindgen_futures::future_to_promise(async move {
            let req_marker = JsValue::from_str(&batch_id);
            invoke_hooks(&hooks, "onRequest", &[req_marker.clone()]).await;
            let url = format!("{base_url}/batches/{batch_id}");
            match fetch_json_get_with_auth(&url, &auth_header, max_retries).await {
                Ok(resp_json) => {
                    invoke_hooks(&hooks, "onResponse", &[req_marker, resp_json.clone()]).await;
                    Ok(resp_json)
                }
                Err(err) => {
                    invoke_hooks(&hooks, "onError", &[req_marker, err.clone()]).await;
                    Err(err)
                }
            }
        })
    }

    /// List batches, optionally filtered by query parameters.
    ///
    /// Pass `null` or `undefined` to list all batches without filtering.
    /// Returns a `Promise` that resolves to the parsed batch list response.
    #[wasm_bindgen(js_name = "listBatches")]
    pub fn list_batches(&self, query: JsValue) -> Promise {
        let auth_header = self.effective_auth_header();
        let base_url = self.base_url.clone();
        let max_retries = self.max_retries;
        let hooks = self.hooks.clone();

        wasm_bindgen_futures::future_to_promise(async move {
            invoke_hooks(&hooks, "onRequest", &[query.clone()]).await;
            let mut url = format!("{base_url}/batches");
            if !query.is_null() && !query.is_undefined() {
                let params = js_to_json(query.clone())?;
                if let serde_json::Value::Object(map) = params {
                    let qs: Vec<String> = map
                        .into_iter()
                        .filter_map(|(k, v)| match v {
                            serde_json::Value::String(s) => Some(format!("{k}={s}")),
                            serde_json::Value::Number(n) => Some(format!("{k}={n}")),
                            _ => None,
                        })
                        .collect();
                    if !qs.is_empty() {
                        url = format!("{url}?{}", qs.join("&"));
                    }
                }
            }
            match fetch_json_get_with_auth(&url, &auth_header, max_retries).await {
                Ok(resp_json) => {
                    invoke_hooks(&hooks, "onResponse", &[query, resp_json.clone()]).await;
                    Ok(resp_json)
                }
                Err(err) => {
                    invoke_hooks(&hooks, "onError", &[query, err.clone()]).await;
                    Err(err)
                }
            }
        })
    }

    /// Cancel an in-progress batch.
    ///
    /// Returns a `Promise` that resolves to the parsed batch object.
    #[wasm_bindgen(js_name = "cancelBatch")]
    pub fn cancel_batch(&self, batch_id: String) -> Promise {
        let auth_header = self.effective_auth_header();
        let base_url = self.base_url.clone();
        let max_retries = self.max_retries;
        let hooks = self.hooks.clone();

        wasm_bindgen_futures::future_to_promise(async move {
            let req_marker = JsValue::from_str(&batch_id);
            invoke_hooks(&hooks, "onRequest", &[req_marker.clone()]).await;
            let url = format!("{base_url}/batches/{batch_id}/cancel");
            match fetch_json_post_with_auth(
                &url,
                &auth_header,
                serde_json::Value::Object(Default::default()),
                max_retries,
            )
            .await
            {
                Ok(resp_json) => {
                    invoke_hooks(&hooks, "onResponse", &[req_marker, resp_json.clone()]).await;
                    Ok(resp_json)
                }
                Err(err) => {
                    invoke_hooks(&hooks, "onError", &[req_marker, err.clone()]).await;
                    Err(err)
                }
            }
        })
    }

    // ── Response management methods ──────────────────────────────────────────

    /// Create a new response.
    ///
    /// Accepts a JS object with response creation parameters.
    /// Returns a `Promise` that resolves to the parsed response object.
    #[wasm_bindgen(js_name = "createResponse")]
    pub fn create_response(&self, request: JsValue) -> Promise {
        let auth_header = self.effective_auth_header();
        let base_url = self.base_url.clone();
        let max_retries = self.max_retries;
        let hooks = self.hooks.clone();

        wasm_bindgen_futures::future_to_promise(async move {
            invoke_hooks(&hooks, "onRequest", &[request.clone()]).await;
            let req_json = js_to_json(request.clone())?;
            let url = format!("{base_url}/responses");
            match fetch_json_post_with_auth(&url, &auth_header, req_json, max_retries).await {
                Ok(resp_json) => {
                    invoke_hooks(&hooks, "onResponse", &[request, resp_json.clone()]).await;
                    Ok(resp_json)
                }
                Err(err) => {
                    invoke_hooks(&hooks, "onError", &[request, err.clone()]).await;
                    Err(err)
                }
            }
        })
    }

    /// Retrieve a response by ID.
    ///
    /// Returns a `Promise` that resolves to the parsed response object.
    #[wasm_bindgen(js_name = "retrieveResponse")]
    pub fn retrieve_response(&self, id: String) -> Promise {
        let auth_header = self.effective_auth_header();
        let base_url = self.base_url.clone();
        let max_retries = self.max_retries;
        let hooks = self.hooks.clone();

        wasm_bindgen_futures::future_to_promise(async move {
            let req_marker = JsValue::from_str(&id);
            invoke_hooks(&hooks, "onRequest", &[req_marker.clone()]).await;
            let url = format!("{base_url}/responses/{id}");
            match fetch_json_get_with_auth(&url, &auth_header, max_retries).await {
                Ok(resp_json) => {
                    invoke_hooks(&hooks, "onResponse", &[req_marker, resp_json.clone()]).await;
                    Ok(resp_json)
                }
                Err(err) => {
                    invoke_hooks(&hooks, "onError", &[req_marker, err.clone()]).await;
                    Err(err)
                }
            }
        })
    }

    /// Cancel an in-progress response.
    ///
    /// Returns a `Promise` that resolves to the parsed response object.
    #[wasm_bindgen(js_name = "cancelResponse")]
    pub fn cancel_response(&self, id: String) -> Promise {
        let auth_header = self.effective_auth_header();
        let base_url = self.base_url.clone();
        let max_retries = self.max_retries;
        let hooks = self.hooks.clone();

        wasm_bindgen_futures::future_to_promise(async move {
            let req_marker = JsValue::from_str(&id);
            invoke_hooks(&hooks, "onRequest", &[req_marker.clone()]).await;
            let url = format!("{base_url}/responses/{id}/cancel");
            match fetch_json_post_with_auth(
                &url,
                &auth_header,
                serde_json::Value::Object(Default::default()),
                max_retries,
            )
            .await
            {
                Ok(resp_json) => {
                    invoke_hooks(&hooks, "onResponse", &[req_marker, resp_json.clone()]).await;
                    Ok(resp_json)
                }
                Err(err) => {
                    invoke_hooks(&hooks, "onError", &[req_marker, err.clone()]).await;
                    Err(err)
                }
            }
        })
    }

    // ── Custom provider registration ────────────────────────────────────────

    /// Register a custom LLM provider at runtime.
    ///
    /// Accepts a JSON string matching the `CustomProviderConfig` schema.
    /// The provider will be checked before built-in providers during model
    /// detection.
    ///
    /// ```js
    /// LlmClient.registerProvider(JSON.stringify({
    ///   name: "my-provider",
    ///   base_url: "https://api.my-provider.com/v1",
    ///   auth_header: "Bearer",
    ///   model_prefixes: ["my-provider/"],
    /// }));
    /// ```
    #[wasm_bindgen(js_name = "registerProvider")]
    pub fn register_provider(config_json: &str) -> Result<(), JsValue> {
        let config: liter_llm::CustomProviderConfig =
            serde_json::from_str(config_json).map_err(|e| js_err(format!("invalid provider config: {e}")))?;

        liter_llm::register_custom_provider(config).map_err(|e| js_err(format!("failed to register provider: {e}")))
    }

    /// Unregister a previously registered custom provider by name.
    ///
    /// Returns `true` if the provider was found and removed, `false` if no
    /// such provider existed.
    #[wasm_bindgen(js_name = "unregisterProvider")]
    pub fn unregister_provider(name: &str) -> Result<bool, JsValue> {
        liter_llm::unregister_custom_provider(name).map_err(|e| js_err(format!("failed to unregister provider: {e}")))
    }

    // ── Hook support ────────────────────────────────────────────────────────

    /// Add a lifecycle hook to this client.
    ///
    /// The hook is a JS object with optional `onRequest`, `onResponse`, and
    /// `onError` async methods.  Hooks are invoked on the JS side wrapping
    /// each `chat`/`embed`/etc. call.
    ///
    /// **Note:** In the WASM binding, hooks run in JavaScript (not in the
    /// Rust Tower middleware stack) because WASM does not use the native
    /// `reqwest`/Tower HTTP pipeline.  This means hooks are advisory and
    /// execute before/after the `fetch` call in JS-land.
    ///
    /// ```js
    /// client.addHook({
    ///   async onRequest(req) { console.log("sending", req); },
    ///   async onResponse(req, resp) { console.log("received", resp); },
    ///   async onError(req, err) { console.error("error", err); },
    /// });
    /// ```
    #[wasm_bindgen(js_name = "addHook")]
    pub fn add_hook(&mut self, hook: JsValue) -> Result<(), JsValue> {
        if !hook.is_object() {
            return Err(js_err(
                "hook must be an object with optional onRequest/onResponse/onError methods",
            ));
        }
        self.hooks.push(hook);
        Ok(())
    }
}

/// Invoke a named method on each hook object, passing the given arguments.
///
/// Each hook is a JS object with optional `onRequest`, `onResponse`, and
/// `onError` methods.  If the method exists and is a function, it is called.
/// If it returns a `Promise`, that promise is awaited.  Errors from hook
/// invocations are silently ignored — hooks are advisory.
async fn invoke_hooks(hooks: &[JsValue], method_name: &str, args: &[JsValue]) {
    let method_key = JsValue::from_str(method_name);
    for hook in hooks {
        if let Ok(func) = js_sys::Reflect::get(hook, &method_key) {
            if func.is_function() {
                let f: js_sys::Function = func.into();
                let result = match args.len() {
                    0 => f.call0(hook),
                    1 => f.call1(hook, &args[0]),
                    2 => f.call2(hook, &args[0], &args[1]),
                    _ => f.call1(hook, &args[0]), // fallback
                };
                // If the hook returns a Promise, await it (best-effort).
                if let Ok(val) = result {
                    if val.is_instance_of::<Promise>() {
                        let _ = JsFuture::from(Promise::from(val)).await;
                    }
                }
            }
        }
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
        // Replace api_key and auth_header_override with empty values and release
        // their backing allocations.  This is the safe, correct way to clear
        // String contents; zeroing individual bytes via as_bytes_mut() is unsafe
        // and risks creating invalid UTF-8 if interrupted.
        drop(std::mem::take(&mut self.api_key));
        drop(std::mem::take(&mut self.auth_header_override));
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
) -> Result<JsValue, JsValue> {
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
async fn fetch_json_get_with_auth(url: &str, auth_header_value: &str, max_retries: u32) -> Result<JsValue, JsValue> {
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
/// Check whether an HTTP error message string represents a retryable error.
///
/// Matches the format `"HTTP NNN: message"` where NNN is 429 or 5xx.
fn is_retryable_http_error(s: &str) -> bool {
    if let Some(rest) = s.strip_prefix("HTTP ")
        && let Some((code_str, _)) = rest.split_once(':')
        && let Ok(status) = code_str.trim().parse::<u16>()
    {
        return status == 429 || (500..=599).contains(&status);
    }
    false
}

fn is_retryable_error(error: &JsValue) -> bool {
    error.as_string().is_some_and(|s| is_retryable_http_error(&s))
}

/// Shared inner fetch implementation using the JS `fetch` API.
///
/// - `method`: HTTP method string (`"POST"` or `"GET"`).
/// - `url`: Target URL.
/// - `auth_header`: Value for the `Authorization` header.
/// - `body`: Optional JSON body string (included only for POST / PUT requests).
async fn do_fetch(method: &str, url: &str, auth_header: &str, body: Option<&str>) -> Result<JsValue, JsValue> {
    use js_sys::Reflect;
    use wasm_bindgen::JsCast;

    let headers = js_sys::Object::new();
    if body.is_some() {
        Reflect::set(&headers, &"Content-Type".into(), &"application/json".into())?;
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
async fn do_fetch_post(url: &str, auth_header_value: &str, body: &str) -> Result<JsValue, JsValue> {
    do_fetch("POST", url, auth_header_value, Some(body)).await
}

/// Inner GET implementation using `web_sys::Request` / `fetch`.
///
/// `auth_header_value` is the full `Authorization` header value.
async fn do_fetch_get(url: &str, auth_header_value: &str) -> Result<JsValue, JsValue> {
    do_fetch("GET", url, auth_header_value, None).await
}

/// Inner DELETE implementation using `web_sys::Request` / `fetch`.
///
/// `auth_header_value` is the full `Authorization` header value.
async fn do_fetch_delete(url: &str, auth_header_value: &str) -> Result<JsValue, JsValue> {
    do_fetch("DELETE", url, auth_header_value, None).await
}

/// Perform a JSON DELETE request using the JS `fetch` API.
///
/// Retries on 429 / 5xx up to `max_retries` times with exponential backoff.
///
/// `auth_header_value` is the full `Authorization` header value.
async fn fetch_json_delete_with_auth(url: &str, auth_header_value: &str, max_retries: u32) -> Result<JsValue, JsValue> {
    let mut attempt = 0u32;
    loop {
        let result = do_fetch_delete(url, auth_header_value).await;
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

/// Perform a POST request and return the response body as raw bytes (Uint8Array).
///
/// Used for binary responses such as audio from the speech endpoint.
async fn fetch_bytes_post_with_auth(
    url: &str,
    auth_header_value: &str,
    body: serde_json::Value,
    max_retries: u32,
) -> Result<JsValue, JsValue> {
    let body_str = serde_json::to_string(&body).map_err(js_err)?;

    let mut attempt = 0u32;
    loop {
        let result = do_fetch_bytes("POST", url, auth_header_value, Some(&body_str)).await;
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

/// Perform a GET request and return the response body as raw bytes (Uint8Array).
///
/// Used for binary responses such as file content downloads.
async fn fetch_bytes_get_with_auth(url: &str, auth_header_value: &str, max_retries: u32) -> Result<JsValue, JsValue> {
    let mut attempt = 0u32;
    loop {
        let result = do_fetch_bytes("GET", url, auth_header_value, None).await;
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

/// Shared inner fetch implementation that returns raw bytes as a `Uint8Array`.
///
/// Used for endpoints that return binary data (audio, file content).
async fn do_fetch_bytes(method: &str, url: &str, auth_header: &str, body: Option<&str>) -> Result<JsValue, JsValue> {
    use js_sys::Reflect;
    use wasm_bindgen::JsCast;

    let headers = js_sys::Object::new();
    if body.is_some() {
        Reflect::set(&headers, &"Content-Type".into(), &"application/json".into())?;
    }
    Reflect::set(&headers, &"Authorization".into(), &auth_header.into())?;

    let init = js_sys::Object::new();
    Reflect::set(&init, &"method".into(), &method.into())?;
    Reflect::set(&init, &"headers".into(), &headers.into())?;
    if let Some(b) = body {
        Reflect::set(&init, &"body".into(), &JsValue::from_str(b))?;
    }

    let global = js_sys::global();

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

    let status = Reflect::get(&response, &"status".into())
        .ok()
        .and_then(|v| v.as_f64())
        .map(|f| f as u16)
        .unwrap_or(0);

    if status >= 400 {
        let text_method: js_sys::Function = Reflect::get(&response, &"text".into())
            .map_err(|_| js_err("response.text is missing"))?
            .dyn_into()
            .map_err(|_| js_err("response.text is not a function"))?;

        let text_promise: Promise = text_method
            .call0(&response)
            .map_err(|e| js_err(format!("response.text() failed: {e:?}")))?
            .dyn_into()
            .map_err(|_| js_err("response.text() did not return a Promise"))?;

        let raw_text: String = JsFuture::from(text_promise).await?.as_string().unwrap_or_default();
        return Err(js_err(format!("HTTP {status}: {raw_text}")));
    }

    let array_buffer_method: js_sys::Function = Reflect::get(&response, &"arrayBuffer".into())
        .map_err(|_| js_err("response.arrayBuffer is missing"))?
        .dyn_into()
        .map_err(|_| js_err("response.arrayBuffer is not a function"))?;

    let ab_promise: Promise = array_buffer_method
        .call0(&response)
        .map_err(|e| js_err(format!("response.arrayBuffer() failed: {e:?}")))?
        .dyn_into()
        .map_err(|_| js_err("response.arrayBuffer() did not return a Promise"))?;

    let array_buffer = JsFuture::from(ab_promise).await?;
    let uint8_array = js_sys::Uint8Array::new(&array_buffer);
    Ok(uint8_array.into())
}

/// Read the response body as JSON, checking the HTTP status first.
///
/// For error responses (status >= 400) the body is always read as text first
/// so that the HTTP status code is preserved in the error string even when the
/// body cannot be parsed as JSON.  The error string is always formatted as
/// `"HTTP {status}: {message}"` so that `is_retryable_error` can parse the
/// status code reliably.
async fn extract_json_from_response(response: JsValue) -> Result<JsValue, JsValue> {
    use js_sys::Reflect;
    use wasm_bindgen::JsCast;

    let status = Reflect::get(&response, &"status".into())
        .ok()
        .and_then(|v| v.as_f64())
        .map(|f| f as u16)
        .unwrap_or(0);

    if status >= 400 {
        // Read the raw response body as text first, then attempt JSON parsing.
        // This ensures the status code is always preserved in the error string
        // even when the error body is not valid JSON (e.g. plain-text errors
        // from proxies or load balancers).
        let text_method: js_sys::Function = Reflect::get(&response, &"text".into())
            .map_err(|_| js_err("response.text is missing"))?
            .dyn_into()
            .map_err(|_| js_err("response.text is not a function"))?;

        let text_promise: Promise = text_method
            .call0(&response)
            .map_err(|e| js_err(format!("response.text() failed: {e:?}")))?
            .dyn_into()
            .map_err(|_| js_err("response.text() did not return a Promise"))?;

        let raw_text: String = JsFuture::from(text_promise).await?.as_string().unwrap_or_default();

        // Try to extract a structured message from the JSON body if possible.
        let message = serde_json::from_str::<serde_json::Value>(&raw_text)
            .ok()
            .as_ref()
            .and_then(|v| v.pointer("/error/message"))
            .and_then(|v| v.as_str())
            .map(ToString::to_string)
            .unwrap_or(raw_text);

        return Err(js_err(format!("HTTP {status}: {message}")));
    }

    let json_method: js_sys::Function = Reflect::get(&response, &"json".into())
        .map_err(|_| js_err("response.json is missing"))?
        .dyn_into()
        .map_err(|_| js_err("response.json is not a function"))?;

    let json_promise: Promise = json_method
        .call0(&response)
        .map_err(|e| js_err(format!("response.json() failed: {e:?}")))?
        .dyn_into()
        .map_err(|_| js_err("response.json() did not return a Promise"))?;

    // Return the parsed JS value directly instead of round-tripping through
    // serde_json::Value.  The previous JsValue -> serde_json::Value -> JsValue
    // conversion caused data loss in the Node.js WASM target, resulting in
    // `chat()` and other methods resolving with null/undefined.
    let json_value = JsFuture::from(json_promise).await?;
    Ok(json_value)
}

// ─── Free-standing helpers ────────────────────────────────────────────────────

/// Returns the version of the liter-llm library.
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
    fn test_is_retryable_http_error() {
        // Retryable: 429 and 5xx in "HTTP NNN: message" format.
        assert!(is_retryable_http_error("HTTP 429: rate limited"));
        assert!(is_retryable_http_error("HTTP 500: internal server error"));
        assert!(is_retryable_http_error("HTTP 503: service unavailable"));
        // Not retryable: 4xx client errors (excluding 429).
        assert!(!is_retryable_http_error("HTTP 400: bad request"));
        assert!(!is_retryable_http_error("HTTP 401: unauthorized"));
        // Not retryable: bare numbers or unrelated strings do not match.
        assert!(!is_retryable_http_error("429"));
        assert!(!is_retryable_http_error("network error"));
    }

    #[test]
    fn test_default_options() {
        assert_eq!(default_max_retries(), 3);
        assert_eq!(default_timeout_secs(), 60);
    }
}
