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
#[wasm_bindgen]
pub struct LlmClient {
    api_key: String,
    base_url: String,
    max_retries: u32,
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
        })
    }

    /// Send a chat completion request.
    ///
    /// Accepts a JS object matching the OpenAI Chat Completions request shape.
    /// Returns a `Promise` that resolves to the parsed response object.
    pub fn chat(&self, request: JsValue) -> Promise {
        let api_key = self.api_key.clone();
        let base_url = self.base_url.clone();
        let max_retries = self.max_retries;

        wasm_bindgen_futures::future_to_promise(async move {
            let req_json = js_to_json(request)?;
            let url = format!("{base_url}/chat/completions");
            let resp_json = fetch_json_post(&url, &api_key, req_json, max_retries).await?;
            json_to_js(&resp_json)
        })
    }

    /// Send an embedding request.
    ///
    /// Accepts a JS object matching the OpenAI Embeddings request shape.
    /// Returns a `Promise` that resolves to the parsed response object.
    pub fn embed(&self, request: JsValue) -> Promise {
        let api_key = self.api_key.clone();
        let base_url = self.base_url.clone();
        let max_retries = self.max_retries;

        wasm_bindgen_futures::future_to_promise(async move {
            let req_json = js_to_json(request)?;
            let url = format!("{base_url}/embeddings");
            let resp_json = fetch_json_post(&url, &api_key, req_json, max_retries).await?;
            json_to_js(&resp_json)
        })
    }

    /// List available models.
    ///
    /// Returns a `Promise` that resolves to the parsed models list object.
    #[wasm_bindgen(js_name = "listModels")]
    pub fn list_models(&self) -> Promise {
        let api_key = self.api_key.clone();
        let base_url = self.base_url.clone();
        let max_retries = self.max_retries;

        wasm_bindgen_futures::future_to_promise(async move {
            let url = format!("{base_url}/models");
            let resp_json = fetch_json_get(&url, &api_key, max_retries).await?;
            json_to_js(&resp_json)
        })
    }

    /// Return the library version string.
    pub fn version(&self) -> String {
        env!("CARGO_PKG_VERSION").to_string()
    }
}

// ─── HTTP helpers via JS fetch ────────────────────────────────────────────────

/// Perform a JSON POST request using the JS `fetch` API.
///
/// Retries on 429 / 5xx up to `max_retries` times.
async fn fetch_json_post(
    url: &str,
    api_key: &str,
    body: serde_json::Value,
    max_retries: u32,
) -> Result<serde_json::Value, JsValue> {
    let body_str = serde_json::to_string(&body).map_err(js_err)?;

    let mut attempt = 0u32;
    loop {
        let result = do_fetch_post(url, api_key, &body_str).await;
        match result {
            Ok(value) => return Ok(value),
            Err(e) if attempt < max_retries && is_retryable_error(&e) => {
                attempt += 1;
            }
            Err(e) => return Err(e),
        }
    }
}

/// Perform a JSON GET request using the JS `fetch` API.
///
/// Retries on 429 / 5xx up to `max_retries` times.
async fn fetch_json_get(url: &str, api_key: &str, max_retries: u32) -> Result<serde_json::Value, JsValue> {
    let mut attempt = 0u32;
    loop {
        let result = do_fetch_get(url, api_key).await;
        match result {
            Ok(value) => return Ok(value),
            Err(e) if attempt < max_retries && is_retryable_error(&e) => {
                attempt += 1;
            }
            Err(e) => return Err(e),
        }
    }
}

/// Returns `true` if the error string suggests a retryable failure (429 / 5xx).
fn is_retryable_error(error: &JsValue) -> bool {
    if let Some(s) = error.as_string() {
        s.contains("429") || s.contains("500") || s.contains("502") || s.contains("503")
    } else {
        false
    }
}

/// Inner POST implementation using `web_sys::Request` / `fetch`.
async fn do_fetch_post(url: &str, api_key: &str, body: &str) -> Result<serde_json::Value, JsValue> {
    use js_sys::Reflect;
    use wasm_bindgen::JsCast;

    // Build headers object.
    let headers = js_sys::Object::new();
    Reflect::set(&headers, &"Content-Type".into(), &"application/json".into())?;
    Reflect::set(&headers, &"Authorization".into(), &format!("Bearer {api_key}").into())?;

    // Build init object.
    let init = js_sys::Object::new();
    Reflect::set(&init, &"method".into(), &"POST".into())?;
    Reflect::set(&init, &"headers".into(), &headers.into())?;
    Reflect::set(&init, &"body".into(), &JsValue::from_str(body))?;

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

/// Inner GET implementation using `web_sys::Request` / `fetch`.
async fn do_fetch_get(url: &str, api_key: &str) -> Result<serde_json::Value, JsValue> {
    use js_sys::Reflect;
    use wasm_bindgen::JsCast;

    let headers = js_sys::Object::new();
    Reflect::set(&headers, &"Authorization".into(), &format!("Bearer {api_key}").into())?;

    let init = js_sys::Object::new();
    Reflect::set(&init, &"method".into(), &"GET".into())?;
    Reflect::set(&init, &"headers".into(), &headers.into())?;

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
    extract_json_from_response(response).await
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
        assert!(is_retryable_error(&JsValue::from_str("429")));
        assert!(is_retryable_error(&JsValue::from_str("HTTP 503")));
        assert!(!is_retryable_error(&JsValue::from_str("400")));
    }

    #[test]
    fn test_default_options() {
        assert_eq!(default_max_retries(), 3);
        assert_eq!(default_timeout_secs(), 60);
    }
}
