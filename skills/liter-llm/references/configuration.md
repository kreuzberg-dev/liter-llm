# Configuration Reference

All client configuration options in one place.

## ClientConfig Fields

| Field | Rust Type | Default | Description |
|-------|-----------|---------|-------------|
| `api_key` | `SecretString` | **required** | Provider API key. Wrapped in `secrecy::SecretString`, never logged or serialized. |
| `base_url` | `Option<String>` | from registry | Override provider base URL. Skips auto-detection when set. |
| `timeout` | `Duration` | 60s | Per-request timeout. |
| `max_retries` | `u32` | 3 | Retries on 429/5xx with exponential backoff. |
| `extra_headers` | `Vec<(String, String)>` | `[]` | Custom headers sent on every request. |
| `credential_provider` | `Option<Arc<dyn CredentialProvider>>` | `None` | Dynamic auth (Azure AD, Vertex OAuth2, AWS STS). |
| `cache_config` | `Option<CacheConfig>` | `None` | Response cache settings (requires `tower` feature). |
| `cache_store` | `Option<Arc<dyn CacheStore>>` | `None` | Custom cache backend (overrides default in-memory LRU). |
| `budget_config` | `Option<BudgetConfig>` | `None` | Spending budget enforcement. |
| `hooks` | `Vec<Arc<dyn LlmHook>>` | `[]` | Lifecycle hooks (pre-request, post-response, on-error). |
| `cooldown_duration` | `Option<Duration>` | `None` | Circuit breaker after transient errors. |
| `rate_limit_config` | `Option<RateLimitConfig>` | `None` | Per-model RPM/TPM limits. |
| `health_check_interval` | `Option<Duration>` | `None` | Background health probe interval. |
| `enable_cost_tracking` | `bool` | `false` | Record `gen_ai.usage.cost` on tracing spans. |
| `enable_tracing` | `bool` | `false` | OpenTelemetry GenAI semantic convention spans. |

## API Key Management

| Provider | Environment Variable |
|----------|---------------------|
| OpenAI | `OPENAI_API_KEY` |
| Anthropic | `ANTHROPIC_API_KEY` |
| Google (Gemini) | `GEMINI_API_KEY` |
| Groq | `GROQ_API_KEY` |
| Mistral | `MISTRAL_API_KEY` |
| Cohere | `CO_API_KEY` |
| AWS Bedrock | `AWS_ACCESS_KEY_ID` + `AWS_SECRET_ACCESS_KEY` |

Keys are wrapped in `secrecy::SecretString` internally -- never logged, serialized, or included in error messages.

## Model Hints

Pre-resolve a provider at construction. All requests use that provider without prefix lookup:

```python
# Python -- all requests route to OpenAI, no "openai/" prefix needed
client = LlmClient(api_key="sk-...", model_hint="openai")
response = await client.chat(model="gpt-4o", messages=[...])
```

```typescript
// TypeScript
const client = new LlmClient({ apiKey: "sk-...", modelHint: "openai" });
```

## Custom Base URLs

Override `base_url` for local inference servers or proxies:

```python
# Ollama
client = LlmClient(api_key="unused", base_url="http://localhost:11434/v1")
# Corporate proxy
client = LlmClient(api_key="sk-...", base_url="https://llm-proxy.internal.company.com/v1")
```

```typescript
const client = new LlmClient({ apiKey: "unused", baseUrl: "http://localhost:11434/v1" });
```

## Cache

### In-Memory (Default)

```python
# Python
client = LlmClient(
    api_key="sk-...",
    cache={"max_entries": 256, "ttl_seconds": 300},
)
```

```typescript
// TypeScript
const client = new LlmClient({
  apiKey: process.env.OPENAI_API_KEY!,
  cache: { maxEntries: 256, ttlSeconds: 300 },
});
```

| Option | Type | Default | Description |
|--------|------|---------|-------------|
| `max_entries` | int | 256 | Maximum cached responses |
| `ttl_seconds` | int | 300 | Time-to-live in seconds |

### OpenDAL Backends

Supports 40+ backends via Apache OpenDAL: Redis, S3, GCS, filesystem, Azure Blob, etc.

```python
# Redis
client = LlmClient(
    api_key="sk-...",
    cache={"backend": "redis", "backend_config": {"connection_string": "redis://localhost"}, "ttl_seconds": 3600},
)

# S3
client = LlmClient(
    api_key="sk-...",
    cache={"backend": "s3", "backend_config": {"bucket": "my-cache", "region": "us-east-1"}, "ttl_seconds": 3600},
)

# Filesystem
client = LlmClient(
    api_key="sk-...",
    cache={"backend": "fs", "backend_config": {"root": "/tmp/llm-cache"}, "ttl_seconds": 3600},
)

# GCS
client = LlmClient(
    api_key="sk-...",
    cache={"backend": "gcs", "backend_config": {"bucket": "my-cache"}, "ttl_seconds": 3600},
)
```

```typescript
// TypeScript -- Redis example
const client = new LlmClient({
  apiKey: process.env.OPENAI_API_KEY!,
  cache: { backend: "redis", backendConfig: { connectionString: "redis://localhost" }, ttlSeconds: 3600 },
});
```

| Option | Type | Description |
|--------|------|-------------|
| `backend` | string | `"memory"`, `"redis"`, `"s3"`, `"fs"`, `"gcs"`, `"azblob"`, etc. |
| `backend_config` | map | Backend-specific config (connection strings, bucket names, paths) |
| `ttl_seconds` | int | Time-to-live for entries |

## Budget

Track and enforce spending limits. Costs are calculated from an embedded pricing registry based on token usage.

```python
# Python
client = LlmClient(
    api_key="sk-...",
    budget={"global_limit": 10.0, "model_limits": {"openai/gpt-4o": 5.0}, "enforcement": "hard"},
)
print(f"Budget used: ${client.budget_used:.2f}")
```

```typescript
// TypeScript
const client = new LlmClient({
  apiKey: process.env.OPENAI_API_KEY!,
  budget: { globalLimit: 10.0, modelLimits: { "openai/gpt-4o": 5.0 }, enforcement: "hard" },
});
console.log(`Budget used: $${client.budgetUsed.toFixed(2)}`);
```

| Option | Type | Description |
|--------|------|-------------|
| `global_limit` | float | Maximum total spend in USD |
| `model_limits` | map | Per-model spend limits (e.g. `{"openai/gpt-4o": 5.0}`) |
| `enforcement` | string | `"hard"` (reject over-budget requests) or `"soft"` (warn only) |

## Hooks

Lifecycle callbacks for request/response/error events. Useful for logging, guardrails, and auditing.

```python
# Python
class LoggingHook:
    def on_request(self, request):
        print(f"Sending request to {request['model']}")
    def on_response(self, request, response):
        print(f"Got response: {response.usage.total_tokens} tokens")
    def on_error(self, request, error):
        print(f"Error: {error}")

client = LlmClient(api_key="sk-...")
client.add_hook(LoggingHook())
```

```typescript
// TypeScript
const client = new LlmClient({ apiKey: process.env.OPENAI_API_KEY! });
client.addHook({
  onRequest(req) { console.log(`Sending: ${req.model}`); },
  onResponse(req, res) { console.log(`Tokens: ${res.usage?.totalTokens}`); },
  onError(req, err) { console.error(`Error: ${err}`); },
});
```

All three callbacks are optional -- implement only the ones you need.

## Cooldown (Circuit Breaker)

Reject requests for a period after transient errors (rate limit, timeout, server error):

```python
client = LlmClient(api_key="sk-...", cooldown_secs=30)
```

```typescript
const client = new LlmClient({ apiKey: "sk-...", cooldown: 30 });
```

## Rate Limiting

Client-side per-model rate limits (requests per minute and tokens per minute):

```python
client = LlmClient(api_key="sk-...", rate_limit={"rpm": 60, "tpm": 100000})
```

```typescript
const client = new LlmClient({ apiKey: "sk-...", rateLimit: { rpm: 60, tpm: 100000 } });
```

| Option | Type | Description |
|--------|------|-------------|
| `rpm` | int | Maximum requests per minute |
| `tpm` | int | Maximum tokens per minute |

## Health Checks

Background probes to detect provider availability. Requests are rejected when the provider is unhealthy:

```python
client = LlmClient(api_key="sk-...", health_check_secs=60)
```

```typescript
const client = new LlmClient({ apiKey: "sk-...", healthCheck: 60 });
```

## Cost Tracking

Record estimated USD cost per request on tracing spans as `gen_ai.usage.cost`:

```python
client = LlmClient(api_key="sk-...", cost_tracking=True)
```

```typescript
const client = new LlmClient({ apiKey: "sk-...", costTracking: true });
```

## Tracing

OpenTelemetry GenAI semantic convention spans for every request:

```python
client = LlmClient(api_key="sk-...", tracing=True)
```

```typescript
const client = new LlmClient({ apiKey: "sk-...", tracing: true });
```

## Custom Providers

Register custom providers for self-hosted or unsupported LLM endpoints at runtime:

```python
# Python
client = LlmClient(api_key="sk-...")
client.register_provider({
    "name": "my-provider",
    "base_url": "https://my-llm.example.com/v1",
    "auth_header": "Authorization",
    "model_prefixes": ["my-provider/"],
})
# Now use: model="my-provider/my-model"
```

```typescript
// TypeScript
client.registerProvider({
  name: "my-provider",
  baseUrl: "https://my-llm.example.com/v1",
  authHeader: "Authorization",
  modelPrefixes: ["my-provider/"],
});
```

Remove with `client.unregister_provider("my-provider")` / `client.unregisterProvider("my-provider")`.
