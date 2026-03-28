//! liter-llm WebAssembly Bindings
//!
//! Exposes a JavaScript-friendly `LlmClient` class that wraps the Rust core
//! client via `wasm-bindgen`.
//!
//! # Architecture
//!
//! HTTP calls cannot use `reqwest`'s native TLS or TCP stack in WASM.  The
//! actual requests are made by delegating to the browser / Node.js `fetch` API
//! through `web_sys` / `wasm-bindgen-futures`.
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

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use js_sys::Promise;
use liter_llm_bindings_core::case::{to_camel_case_keys, to_snake_case_keys};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;

// ─── TypeScript type definitions ──────────────────────────────────────────────

/// Injected verbatim into the generated `.d.ts` file so TypeScript consumers
/// get full structural typing for every request and response object.
///
/// These mirror the Rust types in `crates/liter-llm/src/types/` exactly.
#[wasm_bindgen(typescript_custom_section)]
const TS_APPEND_CONTENT: &str = r#"
/** Cache configuration for response caching. */
export interface CacheOptions {
  /** Maximum number of cached entries (default: 256). */
  maxEntries?: number;
  /** Time-to-live for cached entries in seconds (default: 300). */
  ttlSeconds?: number;
}

/** Budget configuration for spending limits. */
export interface BudgetOptions {
  /** Maximum total spend across all models in USD. */
  globalLimit?: number;
  /** Per-model spending limits in USD, keyed by model name. */
  modelLimits?: Record<string, number>;
  /** Enforcement mode: `"soft"` (warn via console.warn) or `"hard"` (reject with error). Default: `"hard"`. */
  enforcement?: "soft" | "hard";
}

/** Rate limit configuration for request throttling. */
export interface RateLimitOptions {
  /** Maximum requests per minute. */
  rpm?: number;
  /** Maximum tokens per minute. */
  tpm?: number;
  /** Window size in seconds (default: 60). */
  windowSeconds?: number;
}

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
  /** Response cache configuration. */
  cache?: CacheOptions;
  /** Budget enforcement configuration. */
  budget?: BudgetOptions;
  /** Cooldown period in seconds between requests after errors. */
  cooldown?: number;
  /** Rate limit configuration for request throttling. */
  rateLimit?: RateLimitOptions;
  /** Health check interval in seconds. */
  healthCheck?: number;
  /** Enable cost tracking middleware. */
  costTracking?: boolean;
  /** Enable tracing middleware. */
  tracing?: boolean;
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
    let raw: serde_json::Value = serde_wasm_bindgen::from_value(value).map_err(js_err)?;
    Ok(to_snake_case_keys(raw))
}

/// Convert a `serde_json::Value` with snake_case keys to a `JsValue` with camelCase keys.
///
/// Uses `json_compatible()` serializer so that `serde_json::Map` objects are
/// produced as plain JS objects (`{}`), not JS `Map` instances.  Without this,
/// `serde_wasm_bindgen` 0.6 defaults to JS `Map` which appears as `{}` when
/// accessed via property syntax or `JSON.stringify`.
fn json_to_js_camel(value: serde_json::Value) -> Result<JsValue, JsValue> {
    let camel = to_camel_case_keys(value);
    camel
        .serialize(&serde_wasm_bindgen::Serializer::json_compatible())
        .map_err(js_err)
}

// ─── Custom provider config parsing ─────────────────────────────────────────

/// Parse an auth header format string using the same conventions as
/// `liter_llm_bindings_core::config::parse_auth_header`.
///
/// - `"none"` -> `AuthHeaderFormat::None`
/// - `"api-key:X-Custom"` -> `AuthHeaderFormat::ApiKey("X-Custom")`
/// - `"bearer"` or anything else -> `AuthHeaderFormat::Bearer`
fn parse_auth_header_format(s: &str) -> liter_llm::AuthHeaderFormat {
    let lower = s.to_lowercase();
    if lower == "none" {
        liter_llm::AuthHeaderFormat::None
    } else if let Some(header) = lower.strip_prefix("api-key:") {
        liter_llm::AuthHeaderFormat::ApiKey(header.to_string())
    } else {
        liter_llm::AuthHeaderFormat::Bearer
    }
}

/// Parse a `CustomProviderConfig` from a JSON value, using the same
/// conventions as the other bindings.
fn parse_provider_config_from_json(val: &serde_json::Value) -> Result<liter_llm::CustomProviderConfig, JsValue> {
    let name = val
        .get("name")
        .and_then(|v| v.as_str())
        .ok_or_else(|| js_err("missing 'name' field"))?
        .to_string();
    let base_url = val
        .get("base_url")
        .and_then(|v| v.as_str())
        .ok_or_else(|| js_err("missing 'base_url' field"))?
        .to_string();
    let auth_header = val
        .get("auth_header")
        .and_then(|v| v.as_str())
        .map(parse_auth_header_format)
        .unwrap_or(liter_llm::AuthHeaderFormat::Bearer);
    let model_prefixes = val
        .get("model_prefixes")
        .and_then(|v| v.as_array())
        .map(|arr| arr.iter().filter_map(|v| v.as_str().map(String::from)).collect())
        .unwrap_or_default();

    Ok(liter_llm::CustomProviderConfig {
        name,
        base_url,
        auth_header,
        model_prefixes,
    })
}

// ─── Client options ───────────────────────────────────────────────────────────

/// Rate limit configuration for request throttling.
///
/// Fields are accepted for forward compatibility and deserialized from JS
/// options, but not yet consumed in the WASM binding (which uses direct
/// `fetch` calls instead of the Rust-core middleware stack).
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
struct RateLimitOptionsConfig {
    /// Maximum requests per minute.
    #[serde(default)]
    rpm: Option<u32>,
    /// Maximum tokens per minute.
    #[serde(default)]
    tpm: Option<u64>,
    /// Window size in seconds (default: 60).
    #[serde(default = "default_rate_limit_window")]
    #[serde(alias = "windowSeconds")]
    window_seconds: u64,
}

fn default_rate_limit_window() -> u64 {
    60
}

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
    /// Response cache configuration.
    #[serde(default)]
    cache: Option<CacheOptionsConfig>,
    /// Budget enforcement configuration.
    #[serde(default)]
    budget: Option<BudgetOptionsConfig>,
    /// Cooldown period in seconds between requests after errors.
    /// Accepted for forward compatibility; not yet consumed in WASM binding.
    #[serde(default)]
    #[allow(dead_code)]
    cooldown: Option<u64>,
    /// Rate limit configuration for request throttling.
    /// Accepted for forward compatibility; not yet consumed in WASM binding.
    #[serde(default)]
    #[allow(dead_code)]
    rate_limit: Option<RateLimitOptionsConfig>,
    /// Health check interval in seconds.
    /// Accepted for forward compatibility; not yet consumed in WASM binding.
    #[serde(default)]
    #[allow(dead_code)]
    health_check: Option<u64>,
    /// Enable cost tracking middleware.
    /// Accepted for forward compatibility; not yet consumed in WASM binding.
    #[serde(default)]
    #[allow(dead_code)]
    cost_tracking: Option<bool>,
    /// Enable tracing middleware.
    /// Accepted for forward compatibility; not yet consumed in WASM binding.
    #[serde(default)]
    #[allow(dead_code)]
    tracing: Option<bool>,
}

/// Deserialized cache configuration from JS.
///
/// Accepts both camelCase and snake_case field names so that JS callers
/// can use either convention.
#[derive(Debug, Deserialize)]
struct CacheOptionsConfig {
    #[serde(alias = "maxEntries")]
    #[serde(default = "default_cache_max_entries")]
    max_entries: usize,
    #[serde(alias = "ttlSeconds")]
    #[serde(default = "default_cache_ttl_seconds")]
    ttl_seconds: u32,
}

/// Deserialized budget configuration from JS.
///
/// Accepts both camelCase and snake_case field names.
#[derive(Debug, Deserialize)]
struct BudgetOptionsConfig {
    #[serde(alias = "globalLimit")]
    #[serde(default)]
    global_limit: Option<f64>,
    #[serde(alias = "modelLimits")]
    #[serde(default)]
    model_limits: Option<HashMap<String, f64>>,
    #[serde(default = "default_enforcement")]
    enforcement: String,
}

fn default_max_retries() -> u32 {
    3
}

fn default_timeout_secs() -> u64 {
    60
}

fn default_cache_max_entries() -> usize {
    256
}

fn default_cache_ttl_seconds() -> u32 {
    300
}

fn default_enforcement() -> String {
    "hard".to_string()
}

// ─── Budget state ────────────────────────────────────────────────────────────

/// Budget enforcement mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Enforcement {
    Hard,
    Soft,
}

/// Tracks spending for budget enforcement.  All values are in USD.
///
/// WASM is single-threaded, so simple `f64` fields suffice (no atomics needed).
#[derive(Debug)]
struct BudgetState {
    global_spend: f64,
    model_spend: HashMap<String, f64>,
}

impl BudgetState {
    fn new() -> Self {
        Self {
            global_spend: 0.0,
            model_spend: HashMap::new(),
        }
    }

    /// Record a cost against the global and per-model counters.
    fn record(&mut self, model: &str, usd: f64) {
        self.global_spend += usd;
        *self.model_spend.entry(model.to_owned()).or_insert(0.0) += usd;
    }
}

/// Budget configuration stored on the client.
#[derive(Debug, Clone)]
struct BudgetConfig {
    global_limit: Option<f64>,
    model_limits: HashMap<String, f64>,
    enforcement: Enforcement,
}

/// Check budget limits before a request.  Returns `Err(JsValue)` for hard
/// enforcement when the budget is exceeded, or calls `console.warn` for soft.
fn check_budget(config: &BudgetConfig, state: &BudgetState, model: &str) -> Result<(), JsValue> {
    // Global limit check.
    if let Some(limit) = config.global_limit
        && state.global_spend >= limit
    {
        let msg = format!(
            "global budget exceeded: spent ${:.6}, limit ${:.6}",
            state.global_spend, limit,
        );
        if config.enforcement == Enforcement::Hard {
            return Err(js_err(&msg));
        }
        console_warn(&msg);
    }

    // Per-model limit check.
    if let Some(&limit) = config.model_limits.get(model) {
        let spent = state.model_spend.get(model).copied().unwrap_or(0.0);
        if spent >= limit {
            let msg = format!(
                "model {model} budget exceeded: spent ${:.6}, limit ${:.6}",
                spent, limit,
            );
            if config.enforcement == Enforcement::Hard {
                return Err(js_err(&msg));
            }
            console_warn(&msg);
        }
    }

    Ok(())
}

/// Record the cost from a response's usage data against the budget state.
///
/// Extracts `usage.prompt_tokens` and `usage.completion_tokens` from the JS
/// response object and uses `liter_llm::cost::completion_cost` to estimate
/// the USD cost.
fn record_budget_cost(state: &mut BudgetState, model: &str, response: &JsValue) {
    // Try to extract usage from the response JS object.
    let usage = js_sys::Reflect::get(response, &"usage".into()).ok();
    if let Some(usage_val) = usage {
        if usage_val.is_undefined() || usage_val.is_null() {
            return;
        }
        let prompt = js_sys::Reflect::get(&usage_val, &"promptTokens".into())
            .ok()
            .and_then(|v| v.as_f64())
            .map(|f| f as u64)
            .unwrap_or(0);
        let completion = js_sys::Reflect::get(&usage_val, &"completionTokens".into())
            .ok()
            .and_then(|v| v.as_f64())
            .map(|f| f as u64)
            .unwrap_or(0);

        if let Some(usd) = liter_llm::cost::completion_cost(model, prompt, completion) {
            state.record(model, usd);
        }
    }
}

/// Call `console.warn(msg)` in the JS runtime.
fn console_warn(msg: &str) {
    let console = js_sys::Reflect::get(&js_sys::global(), &"console".into()).ok();
    if let Some(c) = console {
        let warn_fn = js_sys::Reflect::get(&c, &"warn".into())
            .ok()
            .and_then(|f| f.dyn_into::<js_sys::Function>().ok());
        if let Some(f) = warn_fn {
            let _ = f.call1(&c, &JsValue::from_str(msg));
        }
    }
}

// ─── LRU cache ───────────────────────────────────────────────────────────────

/// A simple LRU cache entry with TTL.
#[derive(Debug, Clone)]
struct CacheEntry {
    /// The cached JS response serialized as JSON string (for cloning across
    /// async boundaries).
    response_json: String,
    /// Timestamp (ms since epoch) when this entry was inserted.
    inserted_at: f64,
}

/// Simple LRU cache backed by an ordered `Vec`.
///
/// WASM is single-threaded, so no synchronization is needed.
#[derive(Debug)]
struct LruCache {
    max_entries: usize,
    ttl_ms: f64,
    /// Ordered from least-recently-used (front) to most-recently-used (back).
    entries: Vec<(String, CacheEntry)>,
}

impl LruCache {
    fn new(max_entries: usize, ttl_seconds: u32) -> Self {
        Self {
            max_entries,
            ttl_ms: f64::from(ttl_seconds) * 1000.0,
            entries: Vec::with_capacity(max_entries),
        }
    }

    /// Look up a cache entry by key.  Returns `Some(json_string)` if found
    /// and not expired, promoting it to most-recently-used.
    fn get(&mut self, key: &str) -> Option<String> {
        let now = js_sys::Date::now();
        let pos = self.entries.iter().position(|(k, _)| k == key)?;
        let (_, entry) = &self.entries[pos];

        if now - entry.inserted_at > self.ttl_ms {
            // Expired — remove it.
            self.entries.remove(pos);
            return None;
        }

        // Promote to most-recently-used (move to back).
        let item = self.entries.remove(pos);
        let json = item.1.response_json.clone();
        self.entries.push(item);
        Some(json)
    }

    /// Insert a response into the cache, evicting the LRU entry if at capacity.
    fn insert(&mut self, key: String, response_json: String) {
        let now = js_sys::Date::now();

        // Remove existing entry for this key if present.
        if let Some(pos) = self.entries.iter().position(|(k, _)| k == &key) {
            self.entries.remove(pos);
        }

        // Evict LRU if at capacity.
        if self.entries.len() >= self.max_entries {
            self.entries.remove(0);
        }

        self.entries.push((
            key,
            CacheEntry {
                response_json,
                inserted_at: now,
            },
        ));
    }
}

/// Create a deterministic cache key by hashing the request body JSON string.
///
/// Uses a simple FNV-1a hash since we don't need cryptographic strength.
fn cache_key(url: &str, body: &serde_json::Value) -> String {
    let input = format!("{url}:{body}");
    let mut hash: u64 = 0xcbf2_9ce4_8422_2325;
    for byte in input.as_bytes() {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(0x0100_0000_01b3);
    }
    format!("{hash:016x}")
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
/// - `cache` (object, optional) — LRU cache config with `maxEntries` and `ttlSeconds`
/// - `budget` (object, optional) — budget enforcement with `globalLimit`, `modelLimits`, `enforcement`
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
    /// Optional budget configuration and state.
    budget_config: Option<BudgetConfig>,
    /// Budget state is wrapped in `Rc<RefCell>` so it can be shared with async
    /// futures.  WASM is single-threaded so `RefCell` is safe.
    budget_state: Rc<RefCell<BudgetState>>,
    /// Optional LRU response cache, wrapped in `Rc<RefCell>` for the same reason.
    cache: Rc<RefCell<Option<LruCache>>>,
}

#[wasm_bindgen]
impl LlmClient {
    /// Create a new `LlmClient`.
    ///
    /// Accepts a plain JS object `{ apiKey, baseUrl?, maxRetries?, timeoutSecs?, cache?, budget? }`.
    #[wasm_bindgen(constructor)]
    pub fn new(options: JsValue) -> Result<LlmClient, JsValue> {
        let opts: ClientOptions =
            serde_wasm_bindgen::from_value(options).map_err(|e| js_err(format!("invalid LlmClient options: {e}")))?;

        let base_url = opts.base_url.unwrap_or_else(|| "https://api.openai.com/v1".to_string());

        let budget_config = opts.budget.map(|b| BudgetConfig {
            global_limit: b.global_limit,
            model_limits: b.model_limits.unwrap_or_default(),
            enforcement: if b.enforcement == "soft" {
                Enforcement::Soft
            } else {
                Enforcement::Hard
            },
        });

        let cache = opts.cache.map(|c| LruCache::new(c.max_entries, c.ttl_seconds));

        Ok(Self {
            api_key: opts.api_key,
            base_url,
            max_retries: opts.max_retries,
            auth_header_override: opts.auth_header,
            hooks: Vec::new(),
            budget_config,
            budget_state: Rc::new(RefCell::new(BudgetState::new())),
            cache: Rc::new(RefCell::new(cache)),
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
    ///
    /// When a cache is configured, non-streaming responses are cached by
    /// request body hash and returned from cache on subsequent identical
    /// requests within the TTL window.
    ///
    /// When a budget is configured, spending is checked before the request
    /// and recorded after a successful response.
    pub fn chat(&self, request: JsValue) -> Promise {
        let auth_header = self.effective_auth_header();
        let base_url = self.base_url.clone();
        let max_retries = self.max_retries;
        let hooks = self.hooks.clone();
        let budget_config = self.budget_config.clone();
        let budget_state_rc = Rc::clone(&self.budget_state);
        let cache_rc = Rc::clone(&self.cache);

        wasm_bindgen_futures::future_to_promise(async move {
            invoke_hooks(&hooks, "onRequest", std::slice::from_ref(&request)).await?;
            let req_json = js_to_json(request.clone())?;
            let url = format!("{base_url}/chat/completions");

            // Extract model name for budget tracking.
            let model = req_json
                .get("model")
                .and_then(|v| v.as_str())
                .unwrap_or("unknown")
                .to_owned();

            // Budget pre-flight check.
            if let Some(ref cfg) = budget_config {
                let state = budget_state_rc.borrow();
                check_budget(cfg, &state, &model)?;
            }

            // Cache lookup.
            let cache_key_str = cache_key(&url, &req_json);
            let cached_hit = {
                let mut cache_guard = cache_rc.borrow_mut();
                if let Some(ref mut lru) = *cache_guard {
                    lru.get(&cache_key_str)
                } else {
                    None
                }
            };
            if let Some(cached_json) = cached_hit {
                // Parse the cached JSON string back into a JsValue.
                let parsed: serde_json::Value = serde_json::from_str(&cached_json).map_err(js_err)?;
                let js_val = parsed
                    .serialize(&serde_wasm_bindgen::Serializer::json_compatible())
                    .map_err(js_err)?;
                let _ = invoke_hooks(&hooks, "onResponse", &[request, js_val.clone()]).await;
                return Ok(js_val);
            }

            match fetch_json_post_with_auth(&url, &auth_header, req_json, max_retries).await {
                Ok(resp_json) => {
                    // Budget post-flight: record cost.
                    if budget_config.is_some() {
                        let mut state = budget_state_rc.borrow_mut();
                        record_budget_cost(&mut state, &model, &resp_json);
                    }

                    // Cache the response.
                    {
                        let mut cache_guard = cache_rc.borrow_mut();
                        if let Some(ref mut lru) = *cache_guard {
                            // Serialize the JS response to a JSON string for caching.
                            if let Ok(val) = serde_wasm_bindgen::from_value::<serde_json::Value>(resp_json.clone())
                                && let Ok(json_str) = serde_json::to_string(&val)
                            {
                                lru.insert(cache_key_str, json_str);
                            }
                        }
                    }

                    let _ = invoke_hooks(&hooks, "onResponse", &[request, resp_json.clone()]).await;
                    Ok(resp_json)
                }
                Err(err) => {
                    let _ = invoke_hooks(&hooks, "onError", &[request, err.clone()]).await;
                    Err(err)
                }
            }
        })
    }

    /// Stream a chat completion request.
    ///
    /// Returns a `Promise` that resolves to a `ReadableStream` of parsed SSE
    /// chunks.  Each chunk is a `ChatCompletionChunk` object (parsed JSON).
    ///
    /// The implementation uses the JS `fetch` API with streaming response body
    /// reading, parses Server-Sent Events (SSE) `data:` lines, and enqueues
    /// the parsed JSON objects into a new `ReadableStream` that the caller can
    /// consume with `getReader()` or `for await...of`.
    ///
    /// Budget pre-flight checks are applied before the request.  Budget cost
    /// is **not** recorded for streaming responses because usage data is only
    /// available in the final chunk (if `stream_options.include_usage` is set)
    /// and the stream is consumed asynchronously by the caller.
    #[wasm_bindgen(js_name = "chatStream")]
    pub fn chat_stream(&self, request: JsValue) -> Promise {
        let auth_header = self.effective_auth_header();
        let base_url = self.base_url.clone();
        let hooks = self.hooks.clone();
        let budget_config = self.budget_config.clone();
        let budget_state_rc = Rc::clone(&self.budget_state);

        wasm_bindgen_futures::future_to_promise(async move {
            invoke_hooks(&hooks, "onRequest", std::slice::from_ref(&request)).await?;
            let mut req_json = js_to_json(request.clone())?;

            // Extract model name for budget tracking.
            let model = req_json
                .get("model")
                .and_then(|v| v.as_str())
                .unwrap_or("unknown")
                .to_owned();

            // Budget pre-flight check.
            if let Some(ref cfg) = budget_config {
                let state = budget_state_rc.borrow();
                check_budget(cfg, &state, &model)?;
            }

            // Force stream: true on the request.
            if let serde_json::Value::Object(ref mut map) = req_json {
                map.insert("stream".to_string(), serde_json::Value::Bool(true));
            }

            // Make the fetch request and get the response object (not parsed as JSON).
            let response = do_fetch_raw(
                "POST",
                &format!("{base_url}/chat/completions"),
                &auth_header,
                Some(&serde_json::to_string(&req_json).map_err(js_err)?),
            )
            .await?;

            // Check HTTP status.
            let status = js_sys::Reflect::get(&response, &"status".into())
                .ok()
                .and_then(|v| v.as_f64())
                .map(|f| f as u16)
                .unwrap_or(0);

            if status >= 400 {
                let text_method: js_sys::Function = js_sys::Reflect::get(&response, &"text".into())
                    .map_err(|_| js_err("response.text is missing"))?
                    .dyn_into()
                    .map_err(|_| js_err("response.text is not a function"))?;
                let text_promise: Promise = text_method
                    .call0(&response)
                    .map_err(|e| js_err(format!("response.text() failed: {e:?}")))?
                    .dyn_into()
                    .map_err(|_| js_err("response.text() did not return a Promise"))?;
                let raw_text: String = JsFuture::from(text_promise).await?.as_string().unwrap_or_default();
                let message = serde_json::from_str::<serde_json::Value>(&raw_text)
                    .ok()
                    .as_ref()
                    .and_then(|v| v.pointer("/error/message"))
                    .and_then(|v| v.as_str())
                    .map(ToString::to_string)
                    .unwrap_or(raw_text);
                let _ = invoke_hooks(&hooks, "onError", &[request, JsValue::from_str(&message)]).await;
                return Err(js_err(format!("HTTP {status}: {message}")));
            }

            // Get the response body as a ReadableStream.
            let body =
                js_sys::Reflect::get(&response, &"body".into()).map_err(|_| js_err("response.body is missing"))?;
            if body.is_null() || body.is_undefined() {
                return Err(js_err(
                    "response.body is null — streaming not supported in this environment",
                ));
            }

            // Create a TransformStream that parses SSE lines into JSON objects.
            // We use JS to create the readable stream that wraps the SSE parsing.
            let readable_stream =
                create_sse_transform_stream(&body, budget_config, budget_state_rc, &model, hooks.clone(), request)?;

            Ok(readable_stream)
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
            invoke_hooks(&hooks, "onRequest", std::slice::from_ref(&request)).await?;
            let req_json = js_to_json(request.clone())?;
            let url = format!("{base_url}/embeddings");
            match fetch_json_post_with_auth(&url, &auth_header, req_json, max_retries).await {
                Ok(resp_json) => {
                    let _ = invoke_hooks(&hooks, "onResponse", &[request, resp_json.clone()]).await;
                    Ok(resp_json)
                }
                Err(err) => {
                    let _ = invoke_hooks(&hooks, "onError", &[request, err.clone()]).await;
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
            invoke_hooks(&hooks, "onRequest", std::slice::from_ref(&req_marker)).await?;
            let url = format!("{base_url}/models");
            match fetch_json_get_with_auth(&url, &auth_header, max_retries).await {
                Ok(resp_json) => {
                    let _ = invoke_hooks(&hooks, "onResponse", &[req_marker, resp_json.clone()]).await;
                    Ok(resp_json)
                }
                Err(err) => {
                    let _ = invoke_hooks(&hooks, "onError", &[req_marker, err.clone()]).await;
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
            invoke_hooks(&hooks, "onRequest", std::slice::from_ref(&request)).await?;
            let req_json = js_to_json(request.clone())?;
            let url = format!("{base_url}/images/generations");
            match fetch_json_post_with_auth(&url, &auth_header, req_json, max_retries).await {
                Ok(resp_json) => {
                    let _ = invoke_hooks(&hooks, "onResponse", &[request, resp_json.clone()]).await;
                    Ok(resp_json)
                }
                Err(err) => {
                    let _ = invoke_hooks(&hooks, "onError", &[request, err.clone()]).await;
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
            invoke_hooks(&hooks, "onRequest", std::slice::from_ref(&request)).await?;
            let req_json = js_to_json(request.clone())?;
            let url = format!("{base_url}/audio/speech");
            match fetch_bytes_post_with_auth(&url, &auth_header, req_json, max_retries).await {
                Ok(resp_bytes) => {
                    let _ = invoke_hooks(&hooks, "onResponse", &[request, resp_bytes.clone()]).await;
                    Ok(resp_bytes)
                }
                Err(err) => {
                    let _ = invoke_hooks(&hooks, "onError", &[request, err.clone()]).await;
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
            invoke_hooks(&hooks, "onRequest", std::slice::from_ref(&request)).await?;
            let req_json = js_to_json(request.clone())?;
            let url = format!("{base_url}/audio/transcriptions");
            match fetch_json_post_with_auth(&url, &auth_header, req_json, max_retries).await {
                Ok(resp_json) => {
                    let _ = invoke_hooks(&hooks, "onResponse", &[request, resp_json.clone()]).await;
                    Ok(resp_json)
                }
                Err(err) => {
                    let _ = invoke_hooks(&hooks, "onError", &[request, err.clone()]).await;
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
            invoke_hooks(&hooks, "onRequest", std::slice::from_ref(&request)).await?;
            let req_json = js_to_json(request.clone())?;
            let url = format!("{base_url}/moderations");
            match fetch_json_post_with_auth(&url, &auth_header, req_json, max_retries).await {
                Ok(resp_json) => {
                    let _ = invoke_hooks(&hooks, "onResponse", &[request, resp_json.clone()]).await;
                    Ok(resp_json)
                }
                Err(err) => {
                    let _ = invoke_hooks(&hooks, "onError", &[request, err.clone()]).await;
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
            invoke_hooks(&hooks, "onRequest", std::slice::from_ref(&request)).await?;
            let req_json = js_to_json(request.clone())?;
            let url = format!("{base_url}/rerank");
            match fetch_json_post_with_auth(&url, &auth_header, req_json, max_retries).await {
                Ok(resp_json) => {
                    let _ = invoke_hooks(&hooks, "onResponse", &[request, resp_json.clone()]).await;
                    Ok(resp_json)
                }
                Err(err) => {
                    let _ = invoke_hooks(&hooks, "onError", &[request, err.clone()]).await;
                    Err(err)
                }
            }
        })
    }

    /// Perform a web/document search.
    ///
    /// Accepts a JS object matching the search API format.
    /// Returns a `Promise` that resolves to the parsed search response object.
    pub fn search(&self, request: JsValue) -> Promise {
        let auth_header = self.effective_auth_header();
        let base_url = self.base_url.clone();
        let max_retries = self.max_retries;
        let hooks = self.hooks.clone();

        wasm_bindgen_futures::future_to_promise(async move {
            invoke_hooks(&hooks, "onRequest", std::slice::from_ref(&request)).await?;
            let req_json = js_to_json(request.clone())?;
            let url = format!("{base_url}/search");
            match fetch_json_post_with_auth(&url, &auth_header, req_json, max_retries).await {
                Ok(resp_json) => {
                    let _ = invoke_hooks(&hooks, "onResponse", &[request, resp_json.clone()]).await;
                    Ok(resp_json)
                }
                Err(err) => {
                    let _ = invoke_hooks(&hooks, "onError", &[request, err.clone()]).await;
                    Err(err)
                }
            }
        })
    }

    /// Extract text from a document via OCR.
    ///
    /// Accepts a JS object matching the OCR API format.
    /// Returns a `Promise` that resolves to the parsed OCR response object.
    pub fn ocr(&self, request: JsValue) -> Promise {
        let auth_header = self.effective_auth_header();
        let base_url = self.base_url.clone();
        let max_retries = self.max_retries;
        let hooks = self.hooks.clone();

        wasm_bindgen_futures::future_to_promise(async move {
            invoke_hooks(&hooks, "onRequest", std::slice::from_ref(&request)).await?;
            let req_json = js_to_json(request.clone())?;
            let url = format!("{base_url}/ocr");
            match fetch_json_post_with_auth(&url, &auth_header, req_json, max_retries).await {
                Ok(resp_json) => {
                    let _ = invoke_hooks(&hooks, "onResponse", &[request, resp_json.clone()]).await;
                    Ok(resp_json)
                }
                Err(err) => {
                    let _ = invoke_hooks(&hooks, "onError", &[request, err.clone()]).await;
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
            invoke_hooks(&hooks, "onRequest", std::slice::from_ref(&request)).await?;
            let req_json = js_to_json(request.clone())?;
            let url = format!("{base_url}/files");
            match fetch_json_post_with_auth(&url, &auth_header, req_json, max_retries).await {
                Ok(resp_json) => {
                    let _ = invoke_hooks(&hooks, "onResponse", &[request, resp_json.clone()]).await;
                    Ok(resp_json)
                }
                Err(err) => {
                    let _ = invoke_hooks(&hooks, "onError", &[request, err.clone()]).await;
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
            invoke_hooks(&hooks, "onRequest", std::slice::from_ref(&req_marker)).await?;
            let url = format!("{base_url}/files/{file_id}");
            match fetch_json_get_with_auth(&url, &auth_header, max_retries).await {
                Ok(resp_json) => {
                    let _ = invoke_hooks(&hooks, "onResponse", &[req_marker, resp_json.clone()]).await;
                    Ok(resp_json)
                }
                Err(err) => {
                    let _ = invoke_hooks(&hooks, "onError", &[req_marker, err.clone()]).await;
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
            invoke_hooks(&hooks, "onRequest", std::slice::from_ref(&req_marker)).await?;
            let url = format!("{base_url}/files/{file_id}");
            match fetch_json_delete_with_auth(&url, &auth_header, max_retries).await {
                Ok(resp_json) => {
                    let _ = invoke_hooks(&hooks, "onResponse", &[req_marker, resp_json.clone()]).await;
                    Ok(resp_json)
                }
                Err(err) => {
                    let _ = invoke_hooks(&hooks, "onError", &[req_marker, err.clone()]).await;
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
            invoke_hooks(&hooks, "onRequest", std::slice::from_ref(&query)).await?;
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
                    let _ = invoke_hooks(&hooks, "onResponse", &[query, resp_json.clone()]).await;
                    Ok(resp_json)
                }
                Err(err) => {
                    let _ = invoke_hooks(&hooks, "onError", &[query, err.clone()]).await;
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
            invoke_hooks(&hooks, "onRequest", std::slice::from_ref(&req_marker)).await?;
            let url = format!("{base_url}/files/{file_id}/content");
            match fetch_bytes_get_with_auth(&url, &auth_header, max_retries).await {
                Ok(resp_bytes) => {
                    let _ = invoke_hooks(&hooks, "onResponse", &[req_marker, resp_bytes.clone()]).await;
                    Ok(resp_bytes)
                }
                Err(err) => {
                    let _ = invoke_hooks(&hooks, "onError", &[req_marker, err.clone()]).await;
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
            invoke_hooks(&hooks, "onRequest", std::slice::from_ref(&request)).await?;
            let req_json = js_to_json(request.clone())?;
            let url = format!("{base_url}/batches");
            match fetch_json_post_with_auth(&url, &auth_header, req_json, max_retries).await {
                Ok(resp_json) => {
                    let _ = invoke_hooks(&hooks, "onResponse", &[request, resp_json.clone()]).await;
                    Ok(resp_json)
                }
                Err(err) => {
                    let _ = invoke_hooks(&hooks, "onError", &[request, err.clone()]).await;
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
            invoke_hooks(&hooks, "onRequest", std::slice::from_ref(&req_marker)).await?;
            let url = format!("{base_url}/batches/{batch_id}");
            match fetch_json_get_with_auth(&url, &auth_header, max_retries).await {
                Ok(resp_json) => {
                    let _ = invoke_hooks(&hooks, "onResponse", &[req_marker, resp_json.clone()]).await;
                    Ok(resp_json)
                }
                Err(err) => {
                    let _ = invoke_hooks(&hooks, "onError", &[req_marker, err.clone()]).await;
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
            invoke_hooks(&hooks, "onRequest", std::slice::from_ref(&query)).await?;
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
                    let _ = invoke_hooks(&hooks, "onResponse", &[query, resp_json.clone()]).await;
                    Ok(resp_json)
                }
                Err(err) => {
                    let _ = invoke_hooks(&hooks, "onError", &[query, err.clone()]).await;
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
            invoke_hooks(&hooks, "onRequest", std::slice::from_ref(&req_marker)).await?;
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
                    let _ = invoke_hooks(&hooks, "onResponse", &[req_marker, resp_json.clone()]).await;
                    Ok(resp_json)
                }
                Err(err) => {
                    let _ = invoke_hooks(&hooks, "onError", &[req_marker, err.clone()]).await;
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
            invoke_hooks(&hooks, "onRequest", std::slice::from_ref(&request)).await?;
            let req_json = js_to_json(request.clone())?;
            let url = format!("{base_url}/responses");
            match fetch_json_post_with_auth(&url, &auth_header, req_json, max_retries).await {
                Ok(resp_json) => {
                    let _ = invoke_hooks(&hooks, "onResponse", &[request, resp_json.clone()]).await;
                    Ok(resp_json)
                }
                Err(err) => {
                    let _ = invoke_hooks(&hooks, "onError", &[request, err.clone()]).await;
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
            invoke_hooks(&hooks, "onRequest", std::slice::from_ref(&req_marker)).await?;
            let url = format!("{base_url}/responses/{id}");
            match fetch_json_get_with_auth(&url, &auth_header, max_retries).await {
                Ok(resp_json) => {
                    let _ = invoke_hooks(&hooks, "onResponse", &[req_marker, resp_json.clone()]).await;
                    Ok(resp_json)
                }
                Err(err) => {
                    let _ = invoke_hooks(&hooks, "onError", &[req_marker, err.clone()]).await;
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
            invoke_hooks(&hooks, "onRequest", std::slice::from_ref(&req_marker)).await?;
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
                    let _ = invoke_hooks(&hooks, "onResponse", &[req_marker, resp_json.clone()]).await;
                    Ok(resp_json)
                }
                Err(err) => {
                    let _ = invoke_hooks(&hooks, "onError", &[req_marker, err.clone()]).await;
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
        let json_value: serde_json::Value =
            serde_json::from_str(config_json).map_err(|e| js_err(format!("invalid provider config JSON: {e}")))?;

        let config = parse_provider_config_from_json(&json_value)?;

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

    // ── Budget introspection ────────────────────────────────────────────────

    /// Return the current total global spend in USD.
    ///
    /// Returns `0.0` if no budget is configured or no requests have been made.
    #[wasm_bindgen(getter, js_name = "budgetUsed")]
    pub fn budget_used(&self) -> f64 {
        self.budget_state.borrow().global_spend
    }
}

/// Invoke a named method on each hook object, passing the given arguments.
///
/// Each hook is a JS object with optional `onRequest`, `onResponse`, and
/// `onError` methods.  If the method exists and is a function, it is called.
/// If it returns a `Promise`, that promise is awaited.
///
/// Errors from hook invocations (both synchronous throws and rejected
/// Promises) are propagated to the caller as `Err(JsValue)`.  Callers
/// that treat hooks as advisory (e.g. `onResponse`, `onError`) should
/// discard the result; callers that use hooks as guardrails
/// (e.g. `onRequest`) should propagate the error with `?`.
async fn invoke_hooks(hooks: &[JsValue], method_name: &str, args: &[JsValue]) -> Result<(), JsValue> {
    let method_key = JsValue::from_str(method_name);
    for hook in hooks {
        if let Ok(func) = js_sys::Reflect::get(hook, &method_key)
            && func.is_function()
        {
            let f: js_sys::Function = func.into();
            let result = match args.len() {
                0 => f.call0(hook),
                1 => f.call1(hook, &args[0]),
                2 => f.call2(hook, &args[0], &args[1]),
                _ => f.call1(hook, &args[0]), // fallback
            };
            match result {
                Ok(val) if val.is_instance_of::<Promise>() => {
                    // Await the Promise; propagate rejections.
                    JsFuture::from(Promise::from(val)).await?;
                }
                Ok(_) => { /* Synchronous return — nothing to do. */ }
                Err(e) => return Err(e),
            }
        }
    }
    Ok(())
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

/// Shared inner fetch implementation that returns the raw `Response` object
/// without consuming the body.  Used for streaming where we need to read the
/// response body as a `ReadableStream`.
async fn do_fetch_raw(method: &str, url: &str, auth_header: &str, body: Option<&str>) -> Result<JsValue, JsValue> {
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

    JsFuture::from(response_promise).await
}

/// Create a `ReadableStream` that reads SSE chunks from the fetch response
/// body and enqueues parsed JSON objects.
///
/// The implementation:
/// 1. Gets a reader from the response body `ReadableStream`
/// 2. Reads `Uint8Array` chunks, decodes them to text
/// 3. Splits on `\n` boundaries, looks for `data: ` prefixed lines
/// 4. Parses the JSON payload and enqueues it into the output stream
/// 5. Stops on `data: [DONE]`
///
/// If a budget is configured, the final chunk's usage data (if present) is
/// used to record cost.
fn create_sse_transform_stream(
    body: &JsValue,
    budget_config: Option<BudgetConfig>,
    budget_state_rc: Rc<RefCell<BudgetState>>,
    model: &str,
    hooks: Vec<JsValue>,
    request: JsValue,
) -> Result<JsValue, JsValue> {
    use wasm_bindgen::JsCast;

    // Get a reader from the response body.
    let get_reader: js_sys::Function = js_sys::Reflect::get(body, &"getReader".into())
        .map_err(|_| js_err("response.body.getReader is missing"))?
        .dyn_into()
        .map_err(|_| js_err("response.body.getReader is not a function"))?;
    let reader = get_reader
        .call0(body)
        .map_err(|e| js_err(format!("getReader() failed: {e:?}")))?;

    let model = model.to_owned();

    // Use JS to create a ReadableStream with a pull-based source that reads
    // from the SSE reader.  This is done via inline JS evaluation because
    // creating a ReadableStream with an async pull function from Rust/wasm-bindgen
    // is extremely verbose otherwise.
    //
    // The approach: we store the reader and parsing state in a closure, and
    // create a ReadableStream whose `pull` method reads from the reader,
    // parses SSE lines, and enqueues JSON objects.
    let js_code = js_sys::Function::new_with_args(
        "reader, onChunk, onDone, convertKeys",
        r#"
        let buffer = '';
        return new ReadableStream({
            async pull(controller) {
                while (true) {
                    const { done, value } = await reader.read();
                    if (done) {
                        onDone();
                        controller.close();
                        return;
                    }
                    const text = new TextDecoder().decode(value);
                    buffer += text;
                    const lines = buffer.split('\n');
                    buffer = lines.pop() || '';
                    for (const line of lines) {
                        const trimmed = line.trim();
                        if (!trimmed || trimmed.startsWith(':')) continue;
                        if (trimmed.startsWith('data: ')) {
                            const data = trimmed.slice(6);
                            if (data === '[DONE]') {
                                onDone();
                                controller.close();
                                return;
                            }
                            try {
                                const parsed = JSON.parse(data);
                                onChunk(parsed);
                                const converted = convertKeys(JSON.stringify(parsed));
                                controller.enqueue(converted);
                            } catch (e) {
                                // Skip malformed JSON lines.
                            }
                        }
                    }
                }
            }
        });
        "#,
    );

    // Create the onChunk callback that records budget cost from the last chunk.
    let budget_config_clone = budget_config.clone();
    let model_clone = model.clone();
    let on_chunk = Closure::wrap(Box::new(move |chunk: JsValue| {
        // If this chunk has usage data, record it for budget tracking.
        if budget_config_clone.is_some() {
            let usage = js_sys::Reflect::get(&chunk, &"usage".into()).ok();
            if let Some(ref u) = usage
                && !u.is_undefined()
                && !u.is_null()
            {
                let prompt = js_sys::Reflect::get(u, &"prompt_tokens".into())
                    .ok()
                    .and_then(|v| v.as_f64())
                    .map(|f| f as u64)
                    .unwrap_or(0);
                let completion = js_sys::Reflect::get(u, &"completion_tokens".into())
                    .ok()
                    .and_then(|v| v.as_f64())
                    .map(|f| f as u64)
                    .unwrap_or(0);
                if let Some(usd) = liter_llm::cost::completion_cost(&model_clone, prompt, completion) {
                    let mut state = budget_state_rc.borrow_mut();
                    state.record(&model_clone, usd);
                }
            }
        }
    }) as Box<dyn FnMut(JsValue)>);

    // Create the onDone callback that invokes hooks.
    let on_done = Closure::wrap(Box::new(move || {
        // Fire-and-forget: we cannot await from a sync closure, but the hooks
        // are advisory anyway.
        let hooks_ref = hooks.clone();
        let req_ref = request.clone();
        wasm_bindgen_futures::spawn_local(async move {
            let _ = invoke_hooks(&hooks_ref, "onResponse", &[req_ref, JsValue::from_str("stream_done")]).await;
        });
    }) as Box<dyn FnMut()>);

    // Create the convertKeys callback that converts a JSON string from
    // snake_case keys to camelCase keys, returning a parsed JS object.
    let convert_keys = Closure::wrap(Box::new(|json_str: String| -> JsValue {
        let parsed: serde_json::Value = match serde_json::from_str(&json_str) {
            Ok(v) => v,
            Err(_) => return JsValue::NULL,
        };
        let camel = to_camel_case_keys(parsed);
        camel
            .serialize(&serde_wasm_bindgen::Serializer::json_compatible())
            .unwrap_or(JsValue::NULL)
    }) as Box<dyn FnMut(String) -> JsValue>);

    let args = js_sys::Array::new();
    args.push(&reader);
    args.push(on_chunk.as_ref());
    args.push(on_done.as_ref());
    args.push(convert_keys.as_ref());

    let stream = js_code
        .apply(&JsValue::UNDEFINED, &args)
        .map_err(|e| js_err(format!("failed to create SSE stream: {e:?}")))?;

    // Leak the closures so they remain alive for the lifetime of the stream.
    // In WASM, the stream is consumed by JS which holds references to these
    // callbacks; dropping them would cause a use-after-free.
    on_chunk.forget();
    on_done.forget();
    convert_keys.forget();

    Ok(stream)
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

    // Parse the response JSON and convert keys from snake_case to camelCase
    // so JS consumers get idiomatic camelCase fields (promptTokens, finishReason, etc.).
    let json_value = JsFuture::from(json_promise).await?;
    let raw: serde_json::Value = serde_wasm_bindgen::from_value(json_value).map_err(js_err)?;
    json_to_js_camel(raw)
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

    // ── Budget state tests ──────────────────────────────────────────────────

    #[test]
    fn budget_state_records_spend() {
        let mut state = BudgetState::new();
        assert_eq!(state.global_spend, 0.0);

        state.record("gpt-4", 0.05);
        assert!((state.global_spend - 0.05).abs() < 1e-10);
        assert!((state.model_spend["gpt-4"] - 0.05).abs() < 1e-10);

        state.record("gpt-4", 0.03);
        assert!((state.global_spend - 0.08).abs() < 1e-10);
        assert!((state.model_spend["gpt-4"] - 0.08).abs() < 1e-10);
    }

    #[test]
    fn budget_state_tracks_multiple_models() {
        let mut state = BudgetState::new();
        state.record("gpt-4", 0.05);
        state.record("gpt-3.5-turbo", 0.01);

        assert!((state.global_spend - 0.06).abs() < 1e-10);
        assert!((state.model_spend["gpt-4"] - 0.05).abs() < 1e-10);
        assert!((state.model_spend["gpt-3.5-turbo"] - 0.01).abs() < 1e-10);
    }

    // Note: `check_budget` tests cannot run on native targets because
    // the function returns `Result<(), JsValue>` and `JsValue` panics
    // on non-wasm32 platforms.  These are tested via the wasm-bindgen-test
    // suite instead.

    // ── Cache key tests ─────────────────────────────────────────────────────

    #[test]
    fn cache_key_deterministic() {
        let body = serde_json::json!({"model": "gpt-4", "messages": [{"role": "user", "content": "hello"}]});
        let key1 = cache_key("https://api.openai.com/v1/chat/completions", &body);
        let key2 = cache_key("https://api.openai.com/v1/chat/completions", &body);
        assert_eq!(key1, key2, "same input should produce same key");
    }

    #[test]
    fn cache_key_differs_for_different_inputs() {
        let body1 = serde_json::json!({"model": "gpt-4"});
        let body2 = serde_json::json!({"model": "gpt-3.5-turbo"});
        let key1 = cache_key("https://api.openai.com/v1/chat/completions", &body1);
        let key2 = cache_key("https://api.openai.com/v1/chat/completions", &body2);
        assert_ne!(key1, key2, "different inputs should produce different keys");
    }

    #[test]
    fn cache_key_differs_for_different_urls() {
        let body = serde_json::json!({"model": "gpt-4"});
        let key1 = cache_key("https://api.openai.com/v1/chat/completions", &body);
        let key2 = cache_key("https://api.example.com/v1/chat/completions", &body);
        assert_ne!(key1, key2, "different URLs should produce different keys");
    }
}
