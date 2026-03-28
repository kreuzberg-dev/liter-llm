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

All methods return `Result<T, LiterLlmError>`. The error type is defined with `thiserror` and includes variants:

- `Authentication` -- API key rejected (401/403)
- `RateLimited` -- Rate limit exceeded (429)
- `BadRequest` -- Invalid request (400/422)
- `ContextWindowExceeded` -- Input too long
- `ContentPolicy` -- Content policy violation
- `NotFound` -- Model/resource not found (404)
- `ServerError` -- Provider 5xx error
- `ServiceUnavailable` -- Provider temporarily unavailable
- `Timeout` -- Request timeout
- `Network` -- Network error
- `Streaming` -- Stream parse error
- `EndpointNotSupported` -- Provider does not support endpoint
- `InvalidHeader` -- Custom header name or value is invalid
- `Serialization` -- JSON serialization error

```rust
use liter_llm::LiterLlmError;

match client.chat(request).await {
    Ok(response) => println!("{}", response.choices[0].message.content.as_deref().unwrap_or("")),
    Err(LiterLlmError::RateLimited { .. }) => eprintln!("Rate limited, retrying..."),
    Err(e) => eprintln!("Error: {e}"),
}
```

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
