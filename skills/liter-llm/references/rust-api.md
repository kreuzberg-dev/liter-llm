# Rust API Reference

## Installation

```toml
[dependencies]
liter-llm = { version = "1.0.0-rc.1", features = ["native-http"] }
```

## Feature Flags

| Feature | Description | Dependencies |
|---------|-------------|--------------|
| `native-http` | Native HTTP stack (reqwest + tokio). **Default.** Required for `DefaultClient`. | `reqwest`, `tokio`, `memchr`, `base64` |
| `tracing` | Structured tracing via `tracing` crate. Adds `#[instrument]` spans on HTTP functions. | `tracing` |
| `tower` | Tower middleware integration (all layers below). Implies `tracing`. | `tower`, `tower-http`, `dashmap`, `futures-util` |
| `otel` | OpenTelemetry export bridge via `tracing-opentelemetry`. Implies `tracing`. | `tracing-opentelemetry`, `opentelemetry` |
| `bedrock` | AWS Bedrock SigV4 signing. Implies `native-http`. | `aws-credential-types`, `aws-sigv4` |
| `azure-auth` | Azure AD OAuth2 credential provider (client-credentials flow). Implies `native-http`. | -- |
| `vertex-auth` | Google Vertex AI OAuth2 credential provider (service-account JWT flow). Implies `native-http`. | `jsonwebtoken` |
| `bedrock-auth` | AWS STS Web Identity credential provider (EKS / IRSA). Implies `native-http`. | -- |
| `tokenizer` | Token counting via HuggingFace tokenizers. Lazy-cached tokenizer loading. | `tokenizers` |
| `opendal-cache` | OpenDAL-backed cache store (S3, GCS, etc.). Implies `tower`. | `opendal` |
| `full` | All features: `native-http` + `tower` + `tracing` + `otel` + `bedrock` + `tokenizer` + all auth + `opendal-cache`. | all of the above |

Default feature: `native-http`.

---

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
    .header("X-Custom", "value")?
    .cache(CacheConfig { max_entries: 256, ttl_seconds: 300 })
    .budget(BudgetConfig {
        global_limit: Some(10.0),
        model_limits: HashMap::new(),
        enforcement: Enforcement::Hard,
    })
    .cooldown(Duration::from_secs(5))
    .rate_limit(RateLimitConfig { rpm: Some(60), tpm: Some(100_000) })
    .health_check(Duration::from_secs(30))
    .cost_tracking(true)
    .tracing(true)
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
| `header(key, value)` | Add a custom header (`native-http` only). Returns `Result`. |
| `cache(config)` | Enable response caching (`tower` feature) |
| `budget(config)` | Enable budget tracking (`tower` feature) |
| `cooldown(duration)` | Set cooldown period after transient errors (`tower` feature) |
| `rate_limit(config)` | Set rate limiting (`tower` feature) |
| `health_check(interval)` | Set health check interval (`tower` feature) |
| `cost_tracking(enabled)` | Enable per-request cost tracking (`tower` feature) |
| `tracing(enabled)` | Enable OpenTelemetry tracing spans (`tower` feature) |
| `build()` | Consume builder, return `ClientConfig` |

### `DefaultClient`

Implements `LlmClient`, `FileClient`, `BatchClient`, and `ResponseClient`. Uses `reqwest` for HTTP. Requires `native-http` feature.

```rust
let client = DefaultClient::new(config, model_hint)?;
```

The `model_hint` parameter (e.g. `Some("groq/llama3-70b")`) selects the provider at construction time. Pass `None` to default to OpenAI.

### `ManagedClient`

Wraps `DefaultClient` with optional Tower middleware stack (cache, budget, hooks, cooldown, rate limiting, health checks, cost tracking, tracing). Requires `tower` feature.

When no middleware is configured, delegates directly to `DefaultClient` with zero overhead. When middleware is configured, routes requests through a Tower service stack.

```rust
use liter_llm::{ClientConfigBuilder, ManagedClient};

let config = ClientConfigBuilder::new("sk-...")
    .cache(CacheConfig { max_entries: 256, ttl_seconds: 300 })
    .budget(BudgetConfig { global_limit: Some(10.0), ..Default::default() })
    .build();

let client = ManagedClient::new(config, Some("gpt-4"))?;
// Use exactly like DefaultClient -- same trait impls
let response = client.chat(request).await?;
```

`ManagedClient` implements `LlmClient`, `FileClient`, `BatchClient`, and `ResponseClient`. It is `Send + Sync` and safe to share via `Arc`.

---

## Traits

### `LlmClient`

The unified API trait for all LLM operations.

```rust
pub trait LlmClient: Send + Sync {
    fn chat(&self, req: ChatCompletionRequest)
        -> BoxFuture<'_, ChatCompletionResponse>;

    fn chat_stream(&self, req: ChatCompletionRequest)
        -> BoxFuture<'_, BoxStream<'_, ChatCompletionChunk>>;

    fn embed(&self, req: EmbeddingRequest)
        -> BoxFuture<'_, EmbeddingResponse>;

    fn list_models(&self)
        -> BoxFuture<'_, ModelsListResponse>;

    fn image_generate(&self, req: CreateImageRequest)
        -> BoxFuture<'_, ImagesResponse>;

    fn speech(&self, req: CreateSpeechRequest)
        -> BoxFuture<'_, bytes::Bytes>;

    fn transcribe(&self, req: CreateTranscriptionRequest)
        -> BoxFuture<'_, TranscriptionResponse>;

    fn moderate(&self, req: ModerationRequest)
        -> BoxFuture<'_, ModerationResponse>;

    fn rerank(&self, req: RerankRequest)
        -> BoxFuture<'_, RerankResponse>;

    fn search(&self, req: SearchRequest)
        -> BoxFuture<'_, SearchResponse>;

    fn ocr(&self, req: OcrRequest)
        -> BoxFuture<'_, OcrResponse>;
}
```

### `FileClient`

File upload, retrieval, listing, deletion, and content download.

```rust
pub trait FileClient: Send + Sync {
    fn create_file(&self, req: CreateFileRequest)
        -> BoxFuture<'_, FileObject>;

    fn retrieve_file(&self, file_id: &str)
        -> BoxFuture<'_, FileObject>;

    fn delete_file(&self, file_id: &str)
        -> BoxFuture<'_, DeleteResponse>;

    fn list_files(&self, query: Option<FileListQuery>)
        -> BoxFuture<'_, FileListResponse>;

    fn file_content(&self, file_id: &str)
        -> BoxFuture<'_, bytes::Bytes>;
}
```

### `BatchClient`

Batch job management.

```rust
pub trait BatchClient: Send + Sync {
    fn create_batch(&self, req: CreateBatchRequest)
        -> BoxFuture<'_, BatchObject>;

    fn retrieve_batch(&self, batch_id: &str)
        -> BoxFuture<'_, BatchObject>;

    fn list_batches(&self, query: Option<BatchListQuery>)
        -> BoxFuture<'_, BatchListResponse>;

    fn cancel_batch(&self, batch_id: &str)
        -> BoxFuture<'_, BatchObject>;
}
```

### `ResponseClient`

Responses API management.

```rust
pub trait ResponseClient: Send + Sync {
    fn create_response(&self, req: CreateResponseRequest)
        -> BoxFuture<'_, ResponseObject>;

    fn retrieve_response(&self, id: &str)
        -> BoxFuture<'_, ResponseObject>;

    fn cancel_response(&self, id: &str)
        -> BoxFuture<'_, ResponseObject>;
}
```

### `LlmHook`

Lifecycle hook trait for request/response/error events.

```rust
pub trait LlmHook: Send + Sync {
    fn on_request(&self, request: &ChatCompletionRequest) -> Result<()> {
        Ok(())
    }
    fn on_response(
        &self,
        _request: &ChatCompletionRequest,
        _response: &ChatCompletionResponse,
    ) {}
    fn on_error(
        &self,
        _request: &ChatCompletionRequest,
        _error: &LiterLlmError,
    ) {}
}
```

All methods have default no-op implementations.

### Type Aliases

```rust
pub type BoxFuture<'a, T> = Pin<Box<dyn Future<Output = Result<T>> + Send + 'a>>;
pub type BoxStream<'a, T> = Pin<Box<dyn Stream<Item = Result<T>> + Send + 'a>>;
```

---

## Tower Middleware Layers

All layers require the `tower` feature. They wrap any `tower::Service<LlmRequest, Response = LlmResponse, Error = LiterLlmError>`.

### `LlmService`

Base Tower service wrapping any `LlmClient`.

```rust
use liter_llm::tower::LlmService;

let service = LlmService::new(client);
```

### Layer Reference

| Layer | Service | Description |
|-------|---------|-------------|
| `CacheLayer` | `CacheService` | In-memory response caching for non-streaming requests |
| `BudgetLayer` | `BudgetService` | Global and per-model spending budget enforcement (hard reject or soft warn) |
| `HooksLayer` | `HooksService` | User-defined pre/post request hooks for guardrails, logging, auditing |
| `CooldownLayer` | `CooldownService` | Deployment cooldowns after transient errors |
| `CostTrackingLayer` | `CostTrackingService` | Emit `gen_ai.usage.cost` tracing span attribute from embedded pricing data |
| `ModelRateLimitLayer` | `ModelRateLimitService` | Per-model RPM / TPM rate limiting |
| `HealthCheckLayer` | `HealthCheckService` | Periodic health probes with automatic request rejection on failure |
| `TracingLayer` | `TracingService` | OTEL-compatible tracing middleware |
| `FallbackLayer` | `FallbackService` | Route to a backup service on transient errors |

### Additional re-exports

| Type | Description |
|------|-------------|
| `Router` | Multi-provider routing with configurable `RoutingStrategy` |
| `CacheBackend` / `CacheStore` / `InMemoryStore` | Cache storage abstraction and in-memory implementation |
| `OpenDalCacheStore` | OpenDAL-backed cache store (requires `opendal-cache` feature) |
| `BudgetConfig` / `BudgetState` / `Enforcement` | Budget configuration and state types |
| `RateLimitConfig` | Rate limit configuration (`rpm`, `tpm`) |
| `LlmRequest` / `LlmResponse` | Request/response enums crossing the Tower `Service` boundary |

### Composing a custom middleware stack

```rust
use liter_llm::tower::{
    CostTrackingLayer, LlmService, TracingLayer, CacheLayer, BudgetLayer,
    CacheConfig, BudgetConfig, InMemoryStore,
};
use tower::ServiceBuilder;

let client = DefaultClient::new(config, None)?;
let service = ServiceBuilder::new()
    .layer(TracingLayer)
    .layer(CostTrackingLayer)
    .layer(CacheLayer::new(CacheConfig {
        max_entries: 256,
        ttl_seconds: 300,
    }, InMemoryStore::new()))
    .layer(BudgetLayer::new(BudgetConfig {
        global_limit: Some(10.0),
        ..Default::default()
    }))
    .service(LlmService::new(client));
```

---

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

### `Message` enum

```rust
pub enum Message {
    System(SystemMessage),
    User(UserMessage),
    Assistant(AssistantMessage),
    Tool(ToolMessage),
    Developer(DeveloperMessage),
}
```

### `SystemMessage`

| Field | Type |
|-------|------|
| `content` | `String` |
| `name` | `Option<String>` |

### `UserMessage`

| Field | Type |
|-------|------|
| `content` | `UserContent` |
| `name` | `Option<String>` |

`UserContent` is an enum: `Text(String)` or `Parts(Vec<ContentPart>)`.

### `AssistantMessage`

| Field | Type |
|-------|------|
| `content` | `Option<String>` |
| `tool_calls` | `Option<Vec<ToolCall>>` |
| `refusal` | `Option<String>` |

### `ToolMessage`

| Field | Type |
|-------|------|
| `content` | `String` |
| `tool_call_id` | `String` |

### `ChatCompletionResponse`

| Field | Type |
|-------|------|
| `id` | `String` |
| `model` | `String` |
| `choices` | `Vec<Choice>` |
| `usage` | `Option<Usage>` |
| `created` | `u64` |

### `Choice`

| Field | Type |
|-------|------|
| `index` | `u32` |
| `message` | `AssistantMessage` |
| `finish_reason` | `Option<String>` |

### `ChatCompletionChunk`

| Field | Type |
|-------|------|
| `id` | `String` |
| `model` | `String` |
| `choices` | `Vec<StreamChoice>` |
| `usage` | `Option<Usage>` |

### `StreamChoice`

| Field | Type |
|-------|------|
| `index` | `u32` |
| `delta` | `Delta` |
| `finish_reason` | `Option<String>` |

### `Usage`

| Field | Type |
|-------|------|
| `prompt_tokens` | `u64` |
| `completion_tokens` | `u64` |
| `total_tokens` | `u64` |

### `EmbeddingRequest`

| Field | Type | Required |
|-------|------|----------|
| `model` | `String` | yes |
| `input` | `EmbeddingInput` | yes |
| `encoding_format` | `Option<String>` | no |
| `dimensions` | `Option<u64>` | no |
| `user` | `Option<String>` | no |

### `EmbeddingResponse`

| Field | Type |
|-------|------|
| `data` | `Vec<EmbeddingObject>` |
| `model` | `String` |
| `usage` | `Usage` |

### `ModelsListResponse`

| Field | Type |
|-------|------|
| `data` | `Vec<ModelObject>` |

---

## Error Handling

All methods return `Result<T, LiterLlmError>`. The error type is defined with `thiserror`.

### `LiterLlmError` variants

| Variant | Trigger |
|---------|---------|
| `Authentication` | API key rejected (HTTP 401/403) |
| `RateLimited` | Rate limit exceeded (HTTP 429) |
| `BadRequest` | Invalid request (HTTP 400/422) |
| `ContextWindowExceeded` | Input too long for model's context window |
| `ContentPolicy` | Content policy violation |
| `NotFound` | Model/resource not found (HTTP 404) |
| `ServerError` | Provider 5xx error |
| `ServiceUnavailable` | Provider temporarily unavailable (HTTP 502/503) |
| `Timeout` | Request timeout |
| `Network` | Network-level error |
| `Streaming` | Stream parse error |
| `EndpointNotSupported` | Provider does not support the endpoint |
| `InvalidHeader` | Custom header name or value is invalid |
| `Serialization` | JSON serialization/deserialization error |

### Error handling pattern

```rust
use liter_llm::LiterLlmError;

match client.chat(request).await {
    Ok(response) => {
        println!("{}", response.choices[0].message.content.as_deref().unwrap_or(""));
    }
    Err(LiterLlmError::RateLimited { .. }) => {
        eprintln!("Rate limited, retrying...");
    }
    Err(LiterLlmError::Authentication { .. }) => {
        eprintln!("Invalid API key");
    }
    Err(LiterLlmError::ContextWindowExceeded { .. }) => {
        eprintln!("Input too long, truncating...");
    }
    Err(e) => {
        eprintln!("Error: {e}");
    }
}
```

---

## Provider System

### `Provider` trait

```rust
pub trait Provider: Send + Sync {
    /// Validate configuration at construction time.
    fn validate(&self) -> Result<()> { Ok(()) }

    /// Provider name (e.g., "openai").
    fn name(&self) -> &str;

    /// Base URL (e.g., "https://api.openai.com/v1").
    fn base_url(&self) -> &str;

    /// Build the authorization header as `Some((header-name, header-value))`.
    fn auth_header(&self, api_key: &str) -> Option<(String, String)>;
}
```

### Provider detection

Providers are resolved at client construction time from the embedded `schemas/providers.json` registry. Models are routed by name prefix convention.

```rust
// "groq/llama3-70b" -> Groq provider
// "anthropic/claude-3-opus" -> Anthropic provider
// "gpt-4" -> OpenAI (default, no prefix)
```

### `detect_provider(model: &str) -> Option<Box<dyn Provider>>`

Resolves a provider from the registry by model name prefix. Returns `None` if no provider matches (falls back to OpenAI).

### Custom providers

#### `CustomProviderConfig`

```rust
pub struct CustomProviderConfig {
    pub name: String,
    pub base_url: String,
    pub auth_header: AuthHeaderFormat,
    pub model_prefixes: Vec<String>,
}
```

#### `register_custom_provider(config) -> Result<()>`

Register a custom provider at runtime.

```rust
use liter_llm::{register_custom_provider, CustomProviderConfig};

register_custom_provider(CustomProviderConfig {
    name: "my-provider".into(),
    base_url: "https://my-llm.example.com/v1".into(),
    auth_header: AuthHeaderFormat::Bearer,
    model_prefixes: vec!["my-provider/".into()],
})?;
```

#### `unregister_custom_provider(name: &str)`

Remove a previously registered custom provider.

```rust
use liter_llm::unregister_custom_provider;
unregister_custom_provider("my-provider");
```

### Budget tracking

```rust
let used: f64 = client.budget_used();
println!("Budget used: ${used:.2}");
```

---

## Complete Example

```rust
use liter_llm::{
    ClientConfigBuilder, DefaultClient, LlmClient,
    ChatCompletionRequest, Message, UserMessage, UserContent,
};

#[tokio::main]
async fn main() -> liter_llm::Result<()> {
    let config = ClientConfigBuilder::new(
        std::env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY not set")
    ).build();

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

### Streaming example

```rust
use futures_core::Stream;
use futures_util::StreamExt;

let request = ChatCompletionRequest {
    model: "gpt-4".into(),
    messages: vec![Message::User(UserMessage {
        content: UserContent::Text("Tell me a joke".into()),
        name: None,
    })],
    ..Default::default()
};

let mut stream = client.chat_stream(request).await?;
while let Some(chunk) = stream.next().await {
    let chunk = chunk?;
    if let Some(content) = &chunk.choices[0].delta.content {
        print!("{content}");
    }
}
```

### ManagedClient with middleware

```rust
use liter_llm::{
    ClientConfigBuilder, ManagedClient, LlmClient,
    tower::{CacheConfig, BudgetConfig, RateLimitConfig, Enforcement},
};
use std::time::Duration;

let config = ClientConfigBuilder::new("sk-...")
    .cache(CacheConfig { max_entries: 256, ttl_seconds: 300 })
    .budget(BudgetConfig {
        global_limit: Some(10.0),
        model_limits: Default::default(),
        enforcement: Enforcement::Hard,
    })
    .rate_limit(RateLimitConfig { rpm: Some(60), tpm: Some(100_000) })
    .cooldown(Duration::from_secs(5))
    .health_check(Duration::from_secs(30))
    .cost_tracking(true)
    .tracing(true)
    .build();

let client = ManagedClient::new(config, Some("gpt-4"))?;

// Use exactly like DefaultClient
let response = client.chat(request).await?;
println!("Budget used: ${:.2}", client.budget_used());
```
