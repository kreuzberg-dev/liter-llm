---
description: "liter-llm Rust API reference"
---

# Rust API Reference

## Installation

```toml
[dependencies]
liter-llm = { version = "1.0.0-rc.1", features = ["native-http"] }
```

The `native-http` feature enables the `DefaultClient` backed by `reqwest` and `tokio`.

## Client

### `ClientConfigBuilder`

Builder for `ClientConfig`. Create with `ClientConfigBuilder::new(api_key)`.

```rust
use liter_llm::{ClientConfigBuilder, DefaultClient};
use std::time::Duration;

let config = ClientConfigBuilder::new("sk-...")
    .base_url("https://api.openai.com/v1")
    .max_retries(3)
    .timeout(Duration::from_secs(60))
    .header("X-Custom", "value")?  // requires native-http feature
    .build();

let client = DefaultClient::new(config, Some("gpt-4"))?;
```

| Method | Description |
|--------|-------------|
| `new(api_key)` | Create builder with API key and defaults |
| `base_url(url)` | Override provider base URL |
| `max_retries(n)` | Set retry count for 429/5xx (default: 3) |
| `timeout(duration)` | Set request timeout (default: 60s) |
| `credential_provider(provider)` | Set dynamic credential provider (Azure AD, Vertex OAuth2) |
| `header(key, value)` | Add a custom header (native-http only) |
| `cache(config)` | Enable response caching: `CacheConfig { max_entries: 256, ttl_seconds: 300 }` |
| `budget(config)` | Enable budget tracking: `BudgetConfig { global_limit: Some(10.0), .. }` |
| `cooldown(duration)` | Set cooldown period after transient errors |
| `rate_limit(config)` | Set rate limiting: `RateLimitConfig { rpm: Some(60), tpm: Some(100_000) }` |
| `health_check(interval)` | Set health check interval |
| `cost_tracking(enabled)` | Enable per-request cost tracking |
| `tracing(enabled)` | Enable OpenTelemetry tracing spans |
| `build()` | Consume builder, return `ClientConfig` |

### `FileConfig`

Load client configuration from a TOML file (`liter-llm.toml`). Supports auto-discovery by searching the current directory and parent directories.

```rust
use liter_llm::{FileConfig, ManagedClient};

// Auto-discover liter-llm.toml in current or parent directories
if let Some(config) = FileConfig::discover()? {
    let client = ManagedClient::new(config.into_builder().build(), None)?;
}

// Load from explicit path
let config = FileConfig::from_toml_file("path/to/liter-llm.toml")?;
let client = ManagedClient::new(config.into_builder().build(), None)?;

// Parse from a TOML string
let toml_str = std::fs::read_to_string("liter-llm.toml")?;
let config = FileConfig::from_toml_str(&toml_str)?;

// Access custom providers defined in the config
for provider in config.providers() {
    println!("Provider: {}", provider.name);
}

// Convert to ClientConfigBuilder for further customization
let builder = config.into_builder();
let final_config = builder.timeout(Duration::from_secs(120)).build();
```

| Method | Description |
|--------|-------------|
| `from_toml_file(path)` | Load config from a TOML file at the given path |
| `from_toml_str(s)` | Parse config from a TOML string |
| `discover()` | Search current and parent directories for `liter-llm.toml`. Returns `Ok(Some(config))` if found, `Ok(None)` if not found. |
| `into_builder()` | Convert into a `ClientConfigBuilder` for further customization |
| `providers()` | Return the list of custom providers defined in the config file |

### `DefaultClient`

Implements `LlmClient`, `FileClient`, `BatchClient`, and `ResponseClient`.

```rust
let client = DefaultClient::new(config, model_hint)?;
```

The `model_hint` parameter (e.g. `Some("groq/llama3-70b")`) selects the provider at construction time. Pass `None` to default to OpenAI.

### Traits

#### `LlmClient`

```rust
pub trait LlmClient: Send + Sync {
    fn chat(&self, req: ChatCompletionRequest) -> BoxFuture<'_, ChatCompletionResponse>;
    fn chat_stream(&self, req: ChatCompletionRequest) -> BoxFuture<'_, BoxStream<'_, ChatCompletionChunk>>;
    fn embed(&self, req: EmbeddingRequest) -> BoxFuture<'_, EmbeddingResponse>;
    fn list_models(&self) -> BoxFuture<'_, ModelsListResponse>;
    fn image_generate(&self, req: CreateImageRequest) -> BoxFuture<'_, ImagesResponse>;
    fn speech(&self, req: CreateSpeechRequest) -> BoxFuture<'_, bytes::Bytes>;
    fn transcribe(&self, req: CreateTranscriptionRequest) -> BoxFuture<'_, TranscriptionResponse>;
    fn moderate(&self, req: ModerationRequest) -> BoxFuture<'_, ModerationResponse>;
    fn rerank(&self, req: RerankRequest) -> BoxFuture<'_, RerankResponse>;
    fn search(&self, req: SearchRequest) -> BoxFuture<'_, SearchResponse>;
    fn ocr(&self, req: OcrRequest) -> BoxFuture<'_, OcrResponse>;
}
```

#### `FileClient`

```rust
pub trait FileClient: Send + Sync {
    fn create_file(&self, req: CreateFileRequest) -> BoxFuture<'_, FileObject>;
    fn retrieve_file(&self, file_id: &str) -> BoxFuture<'_, FileObject>;
    fn delete_file(&self, file_id: &str) -> BoxFuture<'_, DeleteResponse>;
    fn list_files(&self, query: Option<FileListQuery>) -> BoxFuture<'_, FileListResponse>;
    fn file_content(&self, file_id: &str) -> BoxFuture<'_, bytes::Bytes>;
}
```

#### `BatchClient`

```rust
pub trait BatchClient: Send + Sync {
    fn create_batch(&self, req: CreateBatchRequest) -> BoxFuture<'_, BatchObject>;
    fn retrieve_batch(&self, batch_id: &str) -> BoxFuture<'_, BatchObject>;
    fn list_batches(&self, query: Option<BatchListQuery>) -> BoxFuture<'_, BatchListResponse>;
    fn cancel_batch(&self, batch_id: &str) -> BoxFuture<'_, BatchObject>;
}
```

#### `ResponseClient`

```rust
pub trait ResponseClient: Send + Sync {
    fn create_response(&self, req: CreateResponseRequest) -> BoxFuture<'_, ResponseObject>;
    fn retrieve_response(&self, id: &str) -> BoxFuture<'_, ResponseObject>;
    fn cancel_response(&self, id: &str) -> BoxFuture<'_, ResponseObject>;
}
```

### Provider Registration

```rust
use liter_llm::{register_custom_provider, unregister_custom_provider, CustomProviderConfig};

register_custom_provider(CustomProviderConfig {
    name: "my-provider".into(),
    base_url: "https://my-llm.example.com/v1".into(),
    auth_header: "Authorization".into(),
    model_prefixes: vec!["my-provider/".into()],
})?;

// Remove later
unregister_custom_provider("my-provider");
```

### Hooks

Implement the `LlmHook` trait for lifecycle callbacks:

```rust
use liter_llm::LlmHook;

struct LoggingHook;

impl LlmHook for LoggingHook {
    fn on_request(&self, request: &ChatCompletionRequest) -> Result<()> {
        println!("Sending: {}", request.model);
        Ok(())
    }
    fn on_response(&self, _request: &ChatCompletionRequest, response: &ChatCompletionResponse) {
        if let Some(usage) = &response.usage {
            println!("Tokens: {}", usage.total_tokens);
        }
    }
    fn on_error(&self, _request: &ChatCompletionRequest, error: &LiterLlmError) {
        eprintln!("Error: {error}");
    }
}
```

### Budget Tracking

```rust
let used = client.budget_used();
println!("Budget used: ${used:.2}");
```

### Type Aliases

```rust
pub type BoxFuture<'a, T> = Pin<Box<dyn Future<Output = Result<T>> + Send + 'a>>;
pub type BoxStream<'a, T> = Pin<Box<dyn Stream<Item = Result<T>> + Send + 'a>>;
```

## Types

All types derive `Serialize`, `Deserialize`, `Debug`, `Clone`.

### `ChatCompletionRequest`

| Field | Type | Required |
|-------|------|----------|
| `model` | `String` | yes |
| `messages` | `Vec<Message>` | yes |
| `temperature` | `Option<f64>` | no |
| `top_p` | `Option<f64>` | no |
| `max_tokens` | `Option<u64>` | no |
| `tools` | `Option<Vec<ChatCompletionTool>>` | no |
| `tool_choice` | `Option<ToolChoice>` | no |
| `response_format` | `Option<ResponseFormat>` | no |

### `ChatCompletionResponse`

| Field | Type |
|-------|------|
| `id` | `String` |
| `model` | `String` |
| `choices` | `Vec<Choice>` |
| `usage` | `Option<Usage>` |
| `created` | `u64` |

### `ChatCompletionChunk`

| Field | Type |
|-------|------|
| `id` | `String` |
| `model` | `String` |
| `choices` | `Vec<StreamChoice>` |
| `usage` | `Option<Usage>` |

## Error Handling

All methods return `Result<T, LiterLlmError>`. The error type is defined with `thiserror` and has 17 variants. Use `e.is_transient()` to check whether the Tower fallback layer would retry the error on a different endpoint, and `e.error_type()` to record a stable label in traces.

| Variant | HTTP Status | Trigger | Transient? |
|---------|-------------|---------|------------|
| `Authentication { message }` | 401, 403 | API key rejected. | no |
| `RateLimited { message, retry_after }` | 429 | Rate limit exceeded; `retry_after` is parsed from the header when present. | yes |
| `BadRequest { message }` | 400, 422 | Malformed request or unsupported parameter. | no |
| `ContextWindowExceeded { message }` | 400, 422 | Prompt exceeds the context window. | no |
| `ContentPolicy { message }` | 400, 422 | Content policy violation. | no |
| `NotFound { message }` | 404 | Model or resource not found. | no |
| `ServerError { message }` | 500 | Provider 5xx. | yes |
| `ServiceUnavailable { message }` | 502, 503, 504 | Provider temporarily unavailable. | yes |
| `Timeout` | 408 | Request timed out. | yes |
| `Network(reqwest::Error)` | n/a | Transport failure. Available with the `native-http` feature. | yes |
| `Streaming { message }` | n/a | Stream parse, CRC, or UTF-8 failure during SSE or EventStream reads. | no |
| `EndpointNotSupported { endpoint, provider }` | n/a | Provider crate does not implement the endpoint. | no |
| `InvalidHeader { name, reason }` | n/a | Custom header name or value failed validation. | no |
| `Serialization(serde_json::Error)` | n/a | JSON encode or decode failure. | no |
| `BudgetExceeded { message, model }` | 402 | Budget cap hit. | no |
| `HookRejected { message }` | n/a | A registered hook rejected the request. | no |
| `InternalError { message }` | n/a | Library bug. Should never surface in normal operation. | no |

```rust
use liter_llm::LiterLlmError;

match client.chat(request).await {
    Ok(response) => println!("{}", response.choices[0].message.content.as_deref().unwrap_or("")),
    Err(e) if e.is_transient() => eprintln!("transient, retrying: {e}"),
    Err(LiterLlmError::RateLimited { retry_after, .. }) => {
        eprintln!("rate limited, retry after {retry_after:?}")
    }
    Err(LiterLlmError::BudgetExceeded { message, model }) => {
        eprintln!("budget exceeded ({model:?}): {message}")
    }
    Err(e) => eprintln!("Error ({}): {e}", e.error_type()),
}
```

See [Error Handling](../usage/error-handling.md) for the canonical taxonomy and retry semantics shared across every binding.

## Example

```rust
use liter_llm::{
    ClientConfigBuilder, DefaultClient, LlmClient,
    ChatCompletionRequest, Message, UserMessage, UserContent,
};

#[tokio::main]
async fn main() -> liter_llm::Result<()> {
    let config = ClientConfigBuilder::new(std::env::var("OPENAI_API_KEY").unwrap())
        .build();
    let client = DefaultClient::new(config, None)?;

    let request = ChatCompletionRequest {
        model: "gpt-4".into(),
        messages: vec![Message::User(UserMessage {
            content: UserContent::Text("Hello!".into()),
            name: None,
        })],
        ..Default::default()
    };

    let response = client.chat(request).await?;
    println!("{}", response.choices[0].message.content.as_deref().unwrap_or(""));
    Ok(())
}
```
