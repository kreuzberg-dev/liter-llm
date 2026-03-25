#![deny(clippy::all)]

use std::sync::Arc;

use liter_lm::LlmClient as LlmClientTrait;
use liter_lm::{ClientConfigBuilder, DefaultClient};
use napi::bindgen_prelude::*;
use napi_derive::napi;

// ─── camelCase conversion ─────────────────────────────────────────────────────

/// Convert a snake_case identifier to camelCase.
fn snake_to_camel(s: &str) -> String {
    let mut result = String::with_capacity(s.len());
    let mut next_upper = false;
    for ch in s.chars() {
        if ch == '_' {
            next_upper = true;
        } else if next_upper {
            result.extend(ch.to_uppercase());
            next_upper = false;
        } else {
            result.push(ch);
        }
    }
    result
}

/// Recursively convert all object keys from snake_case to camelCase.
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

/// Convert a `liter_lm::LiterLmError` into a NAPI `Error`.
fn to_napi_err(e: liter_lm::LiterLmError) -> napi::Error {
    napi::Error::new(Status::GenericFailure, e.to_string())
}

// ─── JS config object ─────────────────────────────────────────────────────────

/// Options accepted by the `LlmClient` constructor.
#[napi(object)]
pub struct LlmClientOptions {
    pub api_key: String,
    pub base_url: Option<String>,
    pub max_retries: Option<u32>,
    /// Timeout in seconds.
    pub timeout_secs: Option<u32>,
}

// ─── LlmClient ────────────────────────────────────────────────────────────────

/// Node.js-accessible LLM client wrapping `liter_lm::DefaultClient`.
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

        let config = builder.build();
        let client = DefaultClient::new(config, None).map_err(to_napi_err)?;
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
    #[napi]
    pub async fn chat(&self, request: serde_json::Value) -> napi::Result<serde_json::Value> {
        let req: liter_lm::ChatCompletionRequest =
            serde_json::from_value(request).map_err(|e| napi::Error::new(Status::InvalidArg, e.to_string()))?;

        let client = Arc::clone(&self.inner);
        let result = client.chat(req).await.map_err(to_napi_err)?;
        to_js_value(result)
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
        let req: liter_lm::EmbeddingRequest =
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
}

/// Returns the version of the liter-lm library.
#[napi]
pub fn version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}
