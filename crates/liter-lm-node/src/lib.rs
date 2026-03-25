#![deny(clippy::all)]

use std::sync::Arc;

use liter_lm::LlmClient as LlmClientTrait;
use liter_lm::{ClientConfigBuilder, DefaultClient};
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
    let mut next_upper = false;

    // Preserve any leading underscores verbatim — they signal internal / dunder
    // names that should not be title-cased.
    while chars.peek() == Some(&'_') {
        result.push('_');
        chars.next();
        next_upper = false; // reset so the first real char is lower-cased
    }

    for ch in chars {
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

/// Convert a `liter_lm::LiterLmError` into a NAPI `Error`.
///
/// The error kind is embedded in the message so that JS callers can inspect it
/// even though NAPI-RS only exposes a single `Status::GenericFailure` code.
fn to_napi_err(e: liter_lm::LiterLmError) -> napi::Error {
    // Include the variant name for programmatic inspection in JS-land.
    let msg = format!("[{}] {}", error_kind_label(&e), e);
    napi::Error::new(Status::GenericFailure, msg)
}

/// Return a short, stable label for each error variant.
fn error_kind_label(e: &liter_lm::LiterLmError) -> &'static str {
    match e {
        liter_lm::LiterLmError::Authentication { .. } => "Authentication",
        liter_lm::LiterLmError::RateLimited { .. } => "RateLimited",
        liter_lm::LiterLmError::BadRequest { .. } => "BadRequest",
        liter_lm::LiterLmError::ContextWindowExceeded { .. } => "ContextWindowExceeded",
        liter_lm::LiterLmError::ContentPolicy { .. } => "ContentPolicy",
        liter_lm::LiterLmError::NotFound { .. } => "NotFound",
        liter_lm::LiterLmError::ServerError { .. } => "ServerError",
        liter_lm::LiterLmError::ServiceUnavailable { .. } => "ServiceUnavailable",
        liter_lm::LiterLmError::Timeout => "Timeout",
        liter_lm::LiterLmError::Network(_) => "Network",
        liter_lm::LiterLmError::Streaming { .. } => "Streaming",
        liter_lm::LiterLmError::EndpointNotSupported { .. } => "EndpointNotSupported",
        liter_lm::LiterLmError::InvalidHeader { .. } => "InvalidHeader",
        liter_lm::LiterLmError::Serialization(_) => "Serialization",
        // IMPORTANT: Update this match when adding new LiterLmError variants.
        _ => "Unknown",
    }
}

// ─── JS config object ─────────────────────────────────────────────────────────

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
    #[napi(ts_return_type = "Promise<import('../../../packages/typescript/src/types').ChatCompletionResponse>")]
    pub async fn chat(&self, request: serde_json::Value) -> napi::Result<serde_json::Value> {
        let req: liter_lm::ChatCompletionRequest =
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
    #[napi(
        js_name = "chatStream",
        ts_return_type = "Promise<import('../../../packages/typescript/src/types').ChatCompletionChunk[]>"
    )]
    pub async fn chat_stream(&self, request: serde_json::Value) -> napi::Result<Vec<serde_json::Value>> {
        let mut req: liter_lm::ChatCompletionRequest =
            serde_json::from_value(request).map_err(|e| napi::Error::new(Status::InvalidArg, e.to_string()))?;

        // Ensure the streaming flag is set so the core client opens an SSE stream.
        req.stream = Some(true);

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
    #[napi(ts_return_type = "Promise<import('../../../packages/typescript/src/types').EmbeddingResponse>")]
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
    #[napi(
        js_name = "listModels",
        ts_return_type = "Promise<import('../../../packages/typescript/src/types').ModelsListResponse>"
    )]
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

// ─── Stream helpers ───────────────────────────────────────────────────────────

/// Drain a `BoxStream` of `ChatCompletionChunk`s into a `Vec`, short-circuiting
/// on the first error.
async fn collect_chunk_stream(
    stream: liter_lm::BoxStream<'_, liter_lm::ChatCompletionChunk>,
) -> liter_lm::Result<Vec<liter_lm::ChatCompletionChunk>> {
    use std::future::Future;
    use std::pin::Pin;
    use std::task::{Context, Poll};

    // Drive the stream to completion using a simple poll loop bridged to async.
    // We use `tokio::pin!` via the async block to avoid lifetime issues.
    struct StreamCollector<'a> {
        stream: liter_lm::BoxStream<'a, liter_lm::ChatCompletionChunk>,
        items: Vec<liter_lm::ChatCompletionChunk>,
    }

    impl Future for StreamCollector<'_> {
        type Output = liter_lm::Result<Vec<liter_lm::ChatCompletionChunk>>;

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
