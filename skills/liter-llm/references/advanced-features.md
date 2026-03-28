# Advanced Features Reference

## Search Endpoint

Web and document search across supported providers. Available on providers like Brave, Tavily, Serper, DuckDuckGo, SearXNG, Exa AI, Linkup, Google PSE, DataForSEO, Firecrawl, Parallel AI, and Milvus.

### Types

```rust
// SearchRequest
pub struct SearchRequest {
    pub model: String,                              // e.g. "brave/web-search"
    pub query: String,                              // search query
    pub max_results: Option<u32>,                   // max results to return
    pub search_domain_filter: Option<Vec<String>>,  // restrict to domains
    pub country: Option<String>,                    // ISO 3166-1 alpha-2
}

// SearchResponse
pub struct SearchResponse {
    pub results: Vec<SearchResult>,
    pub model: String,
}

// SearchResult
pub struct SearchResult {
    pub title: String,
    pub url: String,
    pub snippet: String,
    pub date: Option<String>,
}
```

### Examples

```python
# Python
response = await client.search(
    model="brave/search",
    query="latest AI news",
    max_results=10,
)
for result in response.results:
    print(f"{result.title}: {result.url}")
```

```typescript
// TypeScript
const response = await client.search({
  model: "brave/search",
  query: "latest AI news",
  maxResults: 10,
});
```

```go
// Go
resp, err := client.Search(ctx, &literllm.SearchRequest{
    Model: "brave/search", Query: "latest AI news", MaxResults: 10,
})
```

---

## OCR Endpoint

Extract text from documents and images with Markdown output. Supported by providers like Mistral (pixtral), Azure AI Document Intelligence, and others.

### Types

```rust
// OcrRequest
pub struct OcrRequest {
    pub model: String,                        // e.g. "mistral/mistral-ocr-latest"
    pub document: OcrDocument,                // URL or inline base64
    pub pages: Option<Vec<u32>>,              // specific pages (1-indexed)
    pub include_image_base64: Option<bool>,   // include page images
}

// OcrDocument -- tagged enum
pub enum OcrDocument {
    Url { url: String },
    Base64 { data: String, media_type: String },
}

// OcrResponse
pub struct OcrResponse {
    pub pages: Vec<OcrPage>,
    pub model: String,
    pub usage: Option<Usage>,
}

// OcrPage
pub struct OcrPage {
    pub index: u32,              // 0-based page index
    pub markdown: String,        // extracted content as Markdown
    pub images: Option<Vec<OcrImage>>,
    pub dimensions: Option<PageDimensions>,
}
```

### Examples

```python
# Python -- from URL
response = await client.ocr(
    model="mistral/mistral-ocr-latest",
    document={"type": "document_url", "url": "https://example.com/doc.pdf"},
)
for page in response.pages:
    print(page.markdown)

# Python -- from file bytes
import base64
response = await client.ocr(
    model="mistral/mistral-ocr-latest",
    document={"type": "base64", "data": base64.b64encode(file_bytes).decode(), "media_type": "application/pdf"},
    pages=[1, 2, 3],
)
```

```typescript
// TypeScript
const response = await client.ocr({
  model: "mistral/mistral-ocr-latest",
  document: { type: "document_url", url: "https://example.com/doc.pdf" },
});
```

---

## OpenDAL Cache Backends

The `CacheBackend` enum supports in-memory or any OpenDAL-backed storage:

```rust
pub enum CacheBackend {
    Memory,                              // Default: in-memory LRU
    OpenDal { scheme: String, config: HashMap<String, String> },
}
```

The `OpenDalCacheStore` implements `CacheStore` using an `opendal::Operator`:

```rust
// Build from scheme + config map
let store = OpenDalCacheStore::from_config(
    "redis",
    HashMap::from([("connection_string".into(), "redis://localhost".into())]),
    "llm-cache/",  // key prefix
    Duration::from_secs(3600),
)?;

// Or from a pre-configured operator
let store = OpenDalCacheStore::new(operator, "llm-cache/", Duration::from_secs(3600));
```

Supported schemes include: `memory`, `redis`, `s3`, `gcs`, `azblob`, `fs`, `ftp`, `hdfs`, `webhdfs`, `cos`, `oss`, `obs`, `supabase`, and 25+ more via Apache OpenDAL.

Entries are stored as JSON with embedded TTL. Backend failures are non-fatal -- they log a warning and behave as cache misses.

---

## Tower Middleware Stack

The `tower` feature provides 10 composable middleware layers. Each is a standard `tower::Layer` + `tower::Service`.

### All Layers

| Layer | Service | Purpose |
|-------|---------|---------|
| `TracingLayer` | `TracingService` | OpenTelemetry GenAI semantic convention spans |
| `CostTrackingLayer` | `CostTrackingService` | Record `gen_ai.usage.cost` from embedded pricing |
| `CacheLayer` | `CacheService` | Response caching (in-memory or OpenDAL) |
| `BudgetLayer` | `BudgetService` | Global and per-model spending enforcement |
| `HooksLayer` | `HooksService` | User-defined pre/post request callbacks |
| `ModelRateLimitLayer` | `ModelRateLimitService` | Per-model RPM/TPM rate limiting |
| `CooldownLayer` | `CooldownService` | Circuit breaker after transient errors |
| `HealthCheckLayer` | `HealthCheckService` | Periodic provider probes, reject on unhealthy |
| `FallbackLayer` | `FallbackService` | Route to backup service on transient errors |
| `Router` | -- | Route requests to different services by `RoutingStrategy` |

### Composition Order

The `ManagedClient` composes layers in this order (outermost first):

```text
TracingLayer -> CostTrackingLayer -> BudgetLayer -> HooksLayer
-> CacheLayer -> RateLimitLayer -> CooldownLayer -> HealthCheckLayer
-> LlmService (inner)
```

### Manual ServiceBuilder Usage

```rust
use liter_llm::tower::{CostTrackingLayer, LlmService, TracingLayer, CacheLayer, BudgetLayer};
use tower::ServiceBuilder;

let client = liter_llm::DefaultClient::new(config, None)?;
let service = ServiceBuilder::new()
    .layer(TracingLayer)
    .layer(CostTrackingLayer)
    .layer(BudgetLayer::new(budget_config))
    .layer(CacheLayer::new(cache_config))
    .service(LlmService::new(client));
```

---

## OpenTelemetry Tracing

`TracingLayer` creates `tracing::info_span` named `"gen_ai"` with semantic convention attributes:

| Attribute | Source | Example |
|-----------|--------|---------|
| `gen_ai.operation.name` | Request variant | `"chat"`, `"embeddings"`, `"list_models"` |
| `gen_ai.request.model` | Request model field | `"openai/gpt-4o"` |
| `gen_ai.system` | Provider prefix from model | `"openai"` |
| `gen_ai.usage.input_tokens` | Response usage | `150` |
| `gen_ai.usage.output_tokens` | Response usage | `42` |
| `gen_ai.usage.cost` | CostTrackingLayer | `0.0023` |
| `gen_ai.response.id` | Response ID | `"chatcmpl-abc123"` |
| `gen_ai.response.model` | Actual model used | `"gpt-4o-2024-05-13"` |
| `gen_ai.response.finish_reasons` | Choices | `"stop"` |
| `error.type` | Error variant name | `"RateLimited"` |

Enable in bindings:

```python
client = LlmClient(api_key="sk-...", tracing=True)
```

---

## Cost Tracking and Budget Enforcement

### How Costs Are Calculated

`CostTrackingLayer` uses an embedded pricing registry to calculate costs:

1. After each successful response, extract the model name and token counts from `Usage`.
2. Look up the model in the pricing registry for input/output price per token.
3. Calculate: `cost = (input_tokens * input_price) + (output_tokens * output_price)`.
4. Record as `gen_ai.usage.cost` on the current tracing span.
5. If a `BudgetLayer` is in the stack, accumulate cost atomically in shared `BudgetState`.

No-op for models not in the pricing registry -- the span attribute is simply omitted.

### Budget Enforcement

`BudgetLayer` uses shared `BudgetState` (atomic `f64`) to track cumulative spend:

- **Hard enforcement** (`"hard"`): Reject requests with `BudgetExceeded` error when the limit is reached.
- **Soft enforcement** (`"soft"`): Log a warning but allow the request through.
- **Per-model limits**: Independent budgets tracked per model prefix.
- `budget_used()` returns the current cumulative spend at any time.

---

## Token Counting

Token counting uses HuggingFace tokenizers for accurate pre-request estimation:

- Models are mapped to their tokenizer (e.g. `gpt-4o` -> `cl100k_base`, `claude-3` -> `claude` tokenizer).
- Used by the rate limit layer (TPM enforcement) and budget layer (cost estimation).
- Fallback to response-reported `usage.prompt_tokens` / `usage.completion_tokens` for actual billing.

---

## Credential Providers

Dynamic credential providers for token-based or refreshable auth. The client calls `resolve()` before each request when configured.

### Trait

```rust
pub trait CredentialProvider: Send + Sync {
    fn resolve(&self) -> BoxFuture<'_, Credential>;
}

pub enum Credential {
    BearerToken(SecretString),           // Azure AD, Vertex OAuth2, OIDC
    AwsCredentials {                     // AWS STS for Bedrock
        access_key_id: SecretString,
        secret_access_key: SecretString,
        session_token: Option<SecretString>,
    },
}
```

### Azure AD (`azure-auth` feature)

```rust
use liter_llm::auth::azure_ad::AzureAdProvider;

let provider = AzureAdProvider::new(tenant_id, client_id, client_secret);
let config = ClientConfigBuilder::new("")
    .credential_provider(Arc::new(provider))
    .build();
```

Handles OAuth2 token acquisition and refresh automatically. Tokens are cached until near expiry.

### Vertex OAuth2 (`vertex-auth` feature)

```rust
use liter_llm::auth::vertex_oauth::VertexOAuth2Provider;

let provider = VertexOAuth2Provider::from_service_account("path/to/service-account.json")?;
let config = ClientConfigBuilder::new("")
    .credential_provider(Arc::new(provider))
    .build();
```

Uses Google service account JSON for OAuth2 token generation. Automatic refresh.

### AWS STS / Bedrock (`bedrock-auth` feature)

```rust
use liter_llm::auth::bedrock_sts::BedrockStsProvider;

let provider = BedrockStsProvider::new(region, role_arn);
let config = ClientConfigBuilder::new("")
    .credential_provider(Arc::new(provider))
    .build();
```

Assumes an IAM role via STS and provides SigV4-compatible credentials. Handles session token refresh.

### Static Provider

For simple cases, a static credential provider always returns the same bearer token:

```rust
let config = ClientConfigBuilder::new("sk-my-api-key").build();
// The api_key is used as a static Bearer token -- no credential_provider needed.
```
