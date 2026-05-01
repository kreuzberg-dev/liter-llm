---
title: "Go API Reference"
---

## Go API Reference <span class="version-badge">v1.4.0-rc.27</span>

### Functions

#### CreateClient()

Create a new LLM client with simple scalar configuration.

This is the primary binding entry-point. All parameters except `api_key`
are optional — omitting them uses the same defaults as
`ClientConfigBuilder`.

**Errors:**

Returns `LiterLlmError` if the underlying HTTP client cannot be
constructed, or if the resolved provider configuration is invalid.

**Signature:**

```go
func CreateClient(apiKey string, baseUrl string, timeoutSecs uint64, maxRetries uint32, modelHint string) (DefaultClient, error)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `ApiKey` | `string` | Yes | The api key |
| `BaseUrl` | `*string` | No | The base url |
| `TimeoutSecs` | `*uint64` | No | The timeout secs |
| `MaxRetries` | `*uint32` | No | The max retries |
| `ModelHint` | `*string` | No | The model hint |

**Returns:** `DefaultClient`

**Errors:** Returns `error`.


---

#### CreateClientFromJson()

Create a new LLM client from a JSON string.

The JSON object accepts the same fields as `liter-llm.toml` (snake_case).

**Errors:**

Returns `LiterLlmError.BadRequest` if `json` is not valid JSON or
contains unknown fields.

**Signature:**

```go
func CreateClientFromJson(json string) (DefaultClient, error)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `Json` | `string` | Yes | The json |

**Returns:** `DefaultClient`

**Errors:** Returns `error`.


---

#### RegisterCustomProvider()

Register a custom provider in the global runtime registry.

The provider will be checked **before** all built-in providers during model
detection. If a provider with the same `name` already exists it is replaced.

**Errors:**

Returns an error if the config is invalid (empty name, empty base_url, or
no model prefixes).

**Signature:**

```go
func RegisterCustomProvider(config CustomProviderConfig) error
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `Config` | `CustomProviderConfig` | Yes | The configuration options |

**Returns:** ``

**Errors:** Returns `error`.


---

#### UnregisterCustomProvider()

Remove a previously registered custom provider by name.

Returns `true` if a provider with the given name was found and removed,
`false` if no such provider existed.

**Errors:**

Returns an error only if the internal lock is poisoned.

**Signature:**

```go
func UnregisterCustomProvider(name string) (bool, error)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `Name` | `string` | Yes | The name |

**Returns:** `bool`

**Errors:** Returns `error`.


---

### Types

#### ApiError

Inner error object.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Message` | `string` | — | Message |
| `ErrorType` | `string` | — | Error type |
| `Param` | `*string` | `nil` | Param |
| `Code` | `*string` | `nil` | Code |


---

#### AssistantMessage

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Content` | `*string` | `nil` | The extracted text content |
| `Name` | `*string` | `nil` | The name |
| `ToolCalls` | `*[]ToolCall` | `nil` | Tool calls |
| `Refusal` | `*string` | `nil` | Refusal |
| `FunctionCall` | `*FunctionCall` | `nil` | Deprecated legacy function_call field; retained for API compatibility. |


---

#### AudioContent

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Data` | `string` | — | Base64-encoded audio data. |
| `Format` | `string` | — | Audio format (e.g., "wav", "mp3", "ogg"). |


---

#### BatchClient

Batch processing operations (create, list, retrieve, cancel).

##### Methods

###### CreateBatch()

Create a new batch job.

**Signature:**

```go
func (o *BatchClient) CreateBatch(req CreateBatchRequest) (BatchObject, error)
```

###### RetrieveBatch()

Retrieve a batch by ID.

**Signature:**

```go
func (o *BatchClient) RetrieveBatch(batchId string) (BatchObject, error)
```

###### ListBatches()

List batches, optionally filtered by query parameters.

**Signature:**

```go
func (o *BatchClient) ListBatches(query BatchListQuery) (BatchListResponse, error)
```

###### CancelBatch()

Cancel an in-progress batch.

**Signature:**

```go
func (o *BatchClient) CancelBatch(batchId string) (BatchObject, error)
```


---

#### ChatCompletionChunk

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Id` | `string` | — | Unique identifier |
| `Object` | `string` | — | Always `"chat.completion.chunk"` from OpenAI-compatible APIs.  Stored as a plain `String` so non-standard provider values do not fail parsing. |
| `Created` | `uint64` | — | Created |
| `Model` | `string` | — | Model |
| `Choices` | `[]StreamChoice` | `nil` | Choices |
| `Usage` | `*Usage` | `nil` | Usage (usage) |
| `SystemFingerprint` | `*string` | `nil` | System fingerprint |
| `ServiceTier` | `*string` | `nil` | Service tier |


---

#### ChatCompletionRequest

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Model` | `string` | — | Model |
| `Messages` | `[]Message` | `nil` | Messages |
| `Temperature` | `*float64` | `nil` | Temperature |
| `TopP` | `*float64` | `nil` | Top p |
| `N` | `*uint32` | `nil` | N |
| `Stream` | `*bool` | `nil` | Whether to stream the response. Managed by the client layer — do not set directly. |
| `Stop` | `*StopSequence` | `nil` | Stop (stop sequence) |
| `MaxTokens` | `*uint64` | `nil` | Maximum tokens |
| `PresencePenalty` | `*float64` | `nil` | Presence penalty |
| `FrequencyPenalty` | `*float64` | `nil` | Frequency penalty |
| `LogitBias` | `*map[string]float64` | `nil` | Token bias map.  Uses `BTreeMap` (sorted keys) for deterministic serialization order — important when hashing or signing requests. |
| `User` | `*string` | `nil` | User |
| `Tools` | `*[]ChatCompletionTool` | `nil` | Tools |
| `ToolChoice` | `*ToolChoice` | `nil` | Tool choice (tool choice) |
| `ParallelToolCalls` | `*bool` | `nil` | Parallel tool calls |
| `ResponseFormat` | `*ResponseFormat` | `nil` | Response format (response format) |
| `StreamOptions` | `*StreamOptions` | `nil` | Stream options (stream options) |
| `Seed` | `*int64` | `nil` | Seed |
| `ReasoningEffort` | `*ReasoningEffort` | `nil` | Reasoning effort (reasoning effort) |
| `ExtraBody` | `*interface{}` | `nil` | Provider-specific extra parameters merged into the request body. Use for guardrails, safety settings, grounding config, etc. |


---

#### ChatCompletionResponse

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Id` | `string` | — | Unique identifier |
| `Object` | `string` | — | Always `"chat.completion"` from OpenAI-compatible APIs.  Stored as a plain `String` so non-standard provider values do not break deserialization. |
| `Created` | `uint64` | — | Created |
| `Model` | `string` | — | Model |
| `Choices` | `[]Choice` | `nil` | Choices |
| `Usage` | `*Usage` | `nil` | Usage (usage) |
| `SystemFingerprint` | `*string` | `nil` | System fingerprint |
| `ServiceTier` | `*string` | `nil` | Service tier |

##### Methods

###### EstimatedCost()

Estimate the cost of this response based on embedded pricing data.

Returns `nil` if:
- the `model` field is not present in the embedded pricing registry, or
- the `usage` field is absent from the response.

**Signature:**

```go
func (o *ChatCompletionResponse) EstimatedCost() *float64
```


---

#### ChatCompletionTool

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `ToolType` | `ToolType` | — | Tool type (tool type) |
| `Function` | `FunctionDefinition` | — | Function (function definition) |


---

#### Choice

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Index` | `uint32` | — | Index |
| `Message` | `AssistantMessage` | — | Message (assistant message) |
| `FinishReason` | `*FinishReason` | `nil` | Finish reason (finish reason) |


---

#### ClientConfig

Configuration for an LLM client.

`api_key` is stored as a `SecretString` so it is zeroed on drop and never
printed accidentally.  Access it via `secrecy.ExposeSecret`.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `ApiKey` | `string` | — | API key for authentication (stored as a secret). |
| `BaseUrl` | `*string` | `nil` | Override base URL.  When set, all requests go here regardless of model name, and provider auto-detection is skipped. |
| `Timeout` | `time.Duration` | — | Request timeout. |
| `MaxRetries` | `uint32` | — | Maximum number of retries on 429 / 5xx responses. |
| `CredentialProvider` | `*CredentialProvider` | `nil` | Optional dynamic credential provider for token-based auth (Azure AD, Vertex OAuth2) or refreshable credentials (AWS STS). When set, the client calls `resolve()` before each request to obtain a fresh credential.  When `None`, the static `api_key` is used. |
| `LoadEnv` | `bool` | — | Automatically load the API key from the provider's environment variable when no explicit key is provided. When `True` (the default) and `api_key` is empty, `DefaultClient.new` reads the provider's designated environment variable (e.g. `OPENAI_API_KEY` for OpenAI).  Set to `False` to suppress this behaviour and require the caller to supply the key explicitly. Has no effect on WASM targets, where `std.env.var` is unavailable. |

##### Methods

###### Headers()

Return the extra headers as an ordered slice of `(name, value)` pairs.

**Signature:**

```go
func (o *ClientConfig) Headers() [](string, string)
```

###### Fmt()

**Signature:**

```go
func (o *ClientConfig) Fmt(f Formatter) Unknown
```


---

#### ClientConfigBuilder

Builder for `ClientConfig`.

Construct with `ClientConfigBuilder.new` and call builder methods to
customise the configuration, then call `ClientConfigBuilder.build` to
obtain a `ClientConfig`.

##### Methods

###### FromEnv()

Create a builder with no explicit API key.

`load_env` is `true` by default, so the key will be read from the
provider's environment variable (e.g. `OPENAI_API_KEY`) at client
construction time.  Call `.load_env(false)` to opt out.

**Signature:**

```go
func (o *ClientConfigBuilder) FromEnv() ClientConfigBuilder
```

###### LoadEnv()

Enable or disable automatic API key loading from environment variables.

When `true` (the default) and no explicit `api_key` was provided,
`DefaultClient.new` reads the provider's designated environment
variable.  Set to `false` to require an explicit key.

Has no effect on WASM targets.

**Signature:**

```go
func (o *ClientConfigBuilder) LoadEnv(enabled bool) ClientConfigBuilder
```

###### BaseUrl()

Override the provider base URL for all requests.

**Signature:**

```go
func (o *ClientConfigBuilder) BaseUrl(url string) ClientConfigBuilder
```

###### Timeout()

Set the per-request timeout (default: 60 s).

**Signature:**

```go
func (o *ClientConfigBuilder) Timeout(timeout time.Duration) ClientConfigBuilder
```

###### MaxRetries()

Set the maximum number of retries on 429 / 5xx responses (default: 3).

**Signature:**

```go
func (o *ClientConfigBuilder) MaxRetries(retries uint32) ClientConfigBuilder
```

###### CredentialProvider()

Set a dynamic credential provider for token-based or refreshable auth.

When configured, the client calls `resolve()` before each request
instead of using the static `api_key` for authentication.

**Signature:**

```go
func (o *ClientConfigBuilder) CredentialProvider(provider CredentialProvider) ClientConfigBuilder
```

###### Header()

Add a custom header sent on every request.

Returns an error if either `key` or `value` is not a valid HTTP header
name / value.

This method is only available when the `native-http` feature is enabled
because header validation relies on `reqwest`'s header types.

**Signature:**

```go
func (o *ClientConfigBuilder) Header(key string, value string) (ClientConfigBuilder, error)
```

###### Cache()

Set the response cache configuration for the Tower middleware stack.

When set, bindings and advanced Rust users can read this from the
built `ClientConfig` to construct a
`CacheLayer`.

**Signature:**

```go
func (o *ClientConfigBuilder) Cache(config CacheConfig) ClientConfigBuilder
```

###### CacheStore()

Set a custom cache store backend for the Tower cache middleware.

When set alongside `cache`, the cache layer will use
this store instead of the default in-memory LRU.

**Signature:**

```go
func (o *ClientConfigBuilder) CacheStore(store CacheStore) ClientConfigBuilder
```

###### Budget()

Set the budget enforcement configuration for the Tower middleware stack.

When set, bindings and advanced Rust users can read this from the
built `ClientConfig` to construct a
`BudgetLayer`.

**Signature:**

```go
func (o *ClientConfigBuilder) Budget(config BudgetConfig) ClientConfigBuilder
```

###### Hook()

Add a single hook to the Tower hooks middleware stack.

Hooks are invoked sequentially in registration order at request
lifecycle points (pre-request, post-response, on-error).

**Signature:**

```go
func (o *ClientConfigBuilder) Hook(hook LlmHook) ClientConfigBuilder
```

###### Hooks()

Set the full list of hooks for the Tower hooks middleware stack,
replacing any previously registered hooks.

Hooks are invoked sequentially in registration order.

**Signature:**

```go
func (o *ClientConfigBuilder) Hooks(hooks []LlmHook) ClientConfigBuilder
```

###### Cooldown()

Set the cooldown duration after transient errors.

When set, the client rejects requests with `ServiceUnavailable` for
the given duration after a transient error (rate limit, timeout,
server error).

**Signature:**

```go
func (o *ClientConfigBuilder) Cooldown(duration time.Duration) ClientConfigBuilder
```

###### RateLimit()

Set per-model rate limiting configuration.

When set, requests exceeding the configured RPM or TPM limits are
rejected with `LiterLlmError.RateLimited`.

**Signature:**

```go
func (o *ClientConfigBuilder) RateLimit(config RateLimitConfig) ClientConfigBuilder
```

###### HealthCheck()

Set the background health check interval.

When set, the client periodically probes the provider and rejects
requests when the provider is unhealthy.

**Signature:**

```go
func (o *ClientConfigBuilder) HealthCheck(interval time.Duration) ClientConfigBuilder
```

###### CostTracking()

Enable or disable per-request cost tracking.

When enabled, estimated USD cost is recorded on the current tracing
span as `gen_ai.usage.cost`.

**Signature:**

```go
func (o *ClientConfigBuilder) CostTracking(enabled bool) ClientConfigBuilder
```

###### Tracing()

Enable or disable OpenTelemetry-compatible tracing spans.

When enabled, every request is wrapped in a `gen_ai` tracing span
with semantic convention attributes.

**Signature:**

```go
func (o *ClientConfigBuilder) Tracing(enabled bool) ClientConfigBuilder
```

###### Build()

Consume the builder and return the completed `ClientConfig`.

**Signature:**

```go
func (o *ClientConfigBuilder) Build() ClientConfig
```


---

#### CreateImageRequest

Request to create images from a text prompt.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Prompt` | `string` | — | Prompt |
| `Model` | `*string` | `nil` | Model |
| `N` | `*uint32` | `nil` | N |
| `Size` | `*string` | `nil` | Size in bytes |
| `Quality` | `*string` | `nil` | Quality |
| `Style` | `*string` | `nil` | Style |
| `ResponseFormat` | `*string` | `nil` | Response format |
| `User` | `*string` | `nil` | User |


---

#### CreateSpeechRequest

Request to generate speech audio from text.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Model` | `string` | — | Model |
| `Input` | `string` | — | Input |
| `Voice` | `string` | — | Voice |
| `ResponseFormat` | `*string` | `nil` | Response format |
| `Speed` | `*float64` | `nil` | Speed |


---

#### CreateTranscriptionRequest

Request to transcribe audio into text.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Model` | `string` | — | Model |
| `File` | `string` | — | Base64-encoded audio file data. |
| `Language` | `*string` | `nil` | Language |
| `Prompt` | `*string` | `nil` | Prompt |
| `ResponseFormat` | `*string` | `nil` | Response format |
| `Temperature` | `*float64` | `nil` | Temperature |


---

#### CustomProviderConfig

Configuration for registering a custom LLM provider at runtime.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Name` | `string` | — | Unique name for this provider (e.g., "my-provider"). |
| `BaseUrl` | `string` | — | Base URL for the provider's API (e.g., "<https://api.my-provider.com/v1">). |
| `AuthHeader` | `AuthHeaderFormat` | — | Authentication header format. |
| `ModelPrefixes` | `[]string` | — | Model name prefixes that route to this provider (e.g., ["my-"]). |


---

#### DefaultClient

Default client implementation backed by `reqwest`.

The provider is resolved at construction time from `model_hint` (or
defaults to OpenAI). However, individual requests can override the
provider when their model string contains a prefix that clearly
identifies a different provider (e.g. `"anthropic/claude-3"` will
route to Anthropic even if the client was built without a hint).

When the model prefix does not match any known provider, the
construction-time provider is used as the fallback.

The provider is stored behind an `Arc` so it can be shared cheaply into
async closures and streaming tasks that must be `'static`.

##### Methods

###### New()

Build a client.

`model_hint` guides provider auto-detection when no explicit
`base_url` override is present in the config.  For example, passing
`Some("groq/llama3-70b")` selects the Groq provider.  Pass `nil` to
default to OpenAI.

**Errors:**

Returns a wrapped `reqwest.Error` if the underlying HTTP client
cannot be constructed.  Header names and values are pre-validated by
`ClientConfigBuilder.header`, so they are inserted directly here.

**Signature:**

```go
func (o *DefaultClient) New(config ClientConfig, modelHint string) (DefaultClient, error)
```

###### Chat()

**Signature:**

```go
func (o *DefaultClient) Chat(req ChatCompletionRequest) (ChatCompletionResponse, error)
```

###### ChatStream()

**Signature:**

```go
func (o *DefaultClient) ChatStream(req ChatCompletionRequest) (BoxStream, error)
```

###### Embed()

**Signature:**

```go
func (o *DefaultClient) Embed(req EmbeddingRequest) (EmbeddingResponse, error)
```

###### ListModels()

**Signature:**

```go
func (o *DefaultClient) ListModels() (ModelsListResponse, error)
```

###### ImageGenerate()

**Signature:**

```go
func (o *DefaultClient) ImageGenerate(req CreateImageRequest) (ImagesResponse, error)
```

###### Speech()

**Signature:**

```go
func (o *DefaultClient) Speech(req CreateSpeechRequest) ([]byte, error)
```

###### Transcribe()

**Signature:**

```go
func (o *DefaultClient) Transcribe(req CreateTranscriptionRequest) (TranscriptionResponse, error)
```

###### Moderate()

**Signature:**

```go
func (o *DefaultClient) Moderate(req ModerationRequest) (ModerationResponse, error)
```

###### Rerank()

**Signature:**

```go
func (o *DefaultClient) Rerank(req RerankRequest) (RerankResponse, error)
```

###### Search()

**Signature:**

```go
func (o *DefaultClient) Search(req SearchRequest) (SearchResponse, error)
```

###### Ocr()

**Signature:**

```go
func (o *DefaultClient) Ocr(req OcrRequest) (OcrResponse, error)
```

###### ChatRaw()

**Signature:**

```go
func (o *DefaultClient) ChatRaw(req ChatCompletionRequest) (RawExchange, error)
```

###### ChatStreamRaw()

**Signature:**

```go
func (o *DefaultClient) ChatStreamRaw(req ChatCompletionRequest) (RawStreamExchange, error)
```

###### EmbedRaw()

**Signature:**

```go
func (o *DefaultClient) EmbedRaw(req EmbeddingRequest) (RawExchange, error)
```

###### ImageGenerateRaw()

**Signature:**

```go
func (o *DefaultClient) ImageGenerateRaw(req CreateImageRequest) (RawExchange, error)
```

###### TranscribeRaw()

**Signature:**

```go
func (o *DefaultClient) TranscribeRaw(req CreateTranscriptionRequest) (RawExchange, error)
```

###### ModerateRaw()

**Signature:**

```go
func (o *DefaultClient) ModerateRaw(req ModerationRequest) (RawExchange, error)
```

###### RerankRaw()

**Signature:**

```go
func (o *DefaultClient) RerankRaw(req RerankRequest) (RawExchange, error)
```

###### SearchRaw()

**Signature:**

```go
func (o *DefaultClient) SearchRaw(req SearchRequest) (RawExchange, error)
```

###### OcrRaw()

**Signature:**

```go
func (o *DefaultClient) OcrRaw(req OcrRequest) (RawExchange, error)
```

###### CreateFile()

**Signature:**

```go
func (o *DefaultClient) CreateFile(req CreateFileRequest) (FileObject, error)
```

###### RetrieveFile()

**Signature:**

```go
func (o *DefaultClient) RetrieveFile(fileId string) (FileObject, error)
```

###### DeleteFile()

**Signature:**

```go
func (o *DefaultClient) DeleteFile(fileId string) (DeleteResponse, error)
```

###### ListFiles()

**Signature:**

```go
func (o *DefaultClient) ListFiles(query FileListQuery) (FileListResponse, error)
```

###### FileContent()

**Signature:**

```go
func (o *DefaultClient) FileContent(fileId string) ([]byte, error)
```

###### CreateBatch()

**Signature:**

```go
func (o *DefaultClient) CreateBatch(req CreateBatchRequest) (BatchObject, error)
```

###### RetrieveBatch()

**Signature:**

```go
func (o *DefaultClient) RetrieveBatch(batchId string) (BatchObject, error)
```

###### ListBatches()

**Signature:**

```go
func (o *DefaultClient) ListBatches(query BatchListQuery) (BatchListResponse, error)
```

###### CancelBatch()

**Signature:**

```go
func (o *DefaultClient) CancelBatch(batchId string) (BatchObject, error)
```

###### CreateResponse()

**Signature:**

```go
func (o *DefaultClient) CreateResponse(req CreateResponseRequest) (ResponseObject, error)
```

###### RetrieveResponse()

**Signature:**

```go
func (o *DefaultClient) RetrieveResponse(id string) (ResponseObject, error)
```

###### CancelResponse()

**Signature:**

```go
func (o *DefaultClient) CancelResponse(id string) (ResponseObject, error)
```


---

#### DeveloperMessage

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Content` | `string` | — | The extracted text content |
| `Name` | `*string` | `nil` | The name |


---

#### DocumentContent

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Data` | `string` | — | Base64-encoded document data or URL. |
| `MediaType` | `string` | — | MIME type (e.g., "application/pdf", "text/csv"). |


---

#### EmbeddingObject

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Object` | `string` | — | Always `"embedding"` from OpenAI-compatible APIs.  Stored as a plain `String` so non-standard provider values do not break deserialization. |
| `Embedding` | `[]float64` | — | Embedding |
| `Index` | `uint32` | — | Index |


---

#### EmbeddingRequest

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Model` | `string` | — | Model |
| `Input` | `EmbeddingInput` | — | Input (embedding input) |
| `EncodingFormat` | `*EmbeddingFormat` | `nil` | Encoding format (embedding format) |
| `Dimensions` | `*uint32` | `nil` | Dimensions |
| `User` | `*string` | `nil` | User |


---

#### EmbeddingResponse

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Object` | `string` | — | Always `"list"` from OpenAI-compatible APIs.  Stored as a plain `String` so non-standard provider values do not break deserialization. |
| `Data` | `[]EmbeddingObject` | — | Data |
| `Model` | `string` | — | Model |
| `Usage` | `*Usage` | `nil` | Usage (usage) |

##### Methods

###### EstimatedCost()

Estimate the cost of this embedding request based on embedded pricing data.

Returns `nil` if:
- the `model` field is not present in the embedded pricing registry, or
- the `usage` field is absent from the response.

Embedding models only charge for input tokens; output cost is zero.

**Signature:**

```go
func (o *EmbeddingResponse) EstimatedCost() *float64
```


---

#### ErrorResponse

Error response from an OpenAI-compatible API.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Error` | `ApiError` | — | Error (api error) |


---

#### FileBudgetConfig

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `GlobalLimit` | `*float64` | `nil` | Global limit |
| `ModelLimits` | `*map[string]float64` | `nil` | Model limits |
| `Enforcement` | `*string` | `nil` | Enforcement |


---

#### FileCacheConfig

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `MaxEntries` | `*int` | `nil` | Maximum entries |
| `TtlSeconds` | `*uint64` | `nil` | Ttl seconds |
| `Backend` | `*string` | `nil` | Backend |
| `BackendConfig` | `*map[string]string` | `nil` | Backend config |


---

#### FileClient

File management operations (upload, list, retrieve, delete).

##### Methods

###### CreateFile()

Upload a file.

**Signature:**

```go
func (o *FileClient) CreateFile(req CreateFileRequest) (FileObject, error)
```

###### RetrieveFile()

Retrieve metadata for a file.

**Signature:**

```go
func (o *FileClient) RetrieveFile(fileId string) (FileObject, error)
```

###### DeleteFile()

Delete a file.

**Signature:**

```go
func (o *FileClient) DeleteFile(fileId string) (DeleteResponse, error)
```

###### ListFiles()

List files, optionally filtered by query parameters.

**Signature:**

```go
func (o *FileClient) ListFiles(query FileListQuery) (FileListResponse, error)
```

###### FileContent()

Retrieve the raw content of a file.

**Signature:**

```go
func (o *FileClient) FileContent(fileId string) ([]byte, error)
```


---

#### FileConfig

TOML file representation of client configuration.

All fields are optional — missing fields use defaults from `ClientConfigBuilder`.
Convert to a builder via `FileConfig.into_builder`.

# Example `liter-llm.toml`

```toml
api_key = "sk-..."
base_url = "<https://api.openai.com/v1">
timeout_secs = 120
max_retries = 5

[cache]
max_entries = 512
ttl_seconds = 600
backend = "memory"

[budget]
global_limit = 50.0
enforcement = "hard"

[[providers]]
name = "my-provider"
base_url = "<https://my-llm.example.com/v1">
model_prefixes = ["my-provider/"]
```

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `ApiKey` | `*string` | `nil` | Api key |
| `BaseUrl` | `*string` | `nil` | Base url |
| `ModelHint` | `*string` | `nil` | Model hint |
| `TimeoutSecs` | `*uint64` | `nil` | Timeout secs |
| `MaxRetries` | `*uint32` | `nil` | Maximum retries |
| `ExtraHeaders` | `*map[string]string` | `nil` | Extra headers |
| `Cache` | `*FileCacheConfig` | `nil` | Cache (file cache config) |
| `Budget` | `*FileBudgetConfig` | `nil` | Budget (file budget config) |
| `CooldownSecs` | `*uint64` | `nil` | Cooldown secs |
| `RateLimit` | `*FileRateLimitConfig` | `nil` | Rate limit (file rate limit config) |
| `HealthCheckSecs` | `*uint64` | `nil` | Health check secs |
| `CostTracking` | `*bool` | `nil` | Cost tracking |
| `Tracing` | `*bool` | `nil` | Tracing |
| `Providers` | `*[]FileProviderConfig` | `nil` | Providers |

##### Methods

###### FromTomlFile()

Load from a TOML file path.

**Signature:**

```go
func (o *FileConfig) FromTomlFile(path Path) (FileConfig, error)
```

###### FromTomlStr()

Parse from a TOML string.

**Signature:**

```go
func (o *FileConfig) FromTomlStr(s string) (FileConfig, error)
```

###### Discover()

Discover `liter-llm.toml` by walking from current directory to filesystem root.

Returns `Ok(None)` if no config file is found.

**Signature:**

```go
func (o *FileConfig) Discover() (*FileConfig, error)
```

###### IntoBuilder()

Convert into a `ClientConfigBuilder`,
applying all fields that are set.

Fields not present in the TOML file use the builder's defaults.

**Signature:**

```go
func (o *FileConfig) IntoBuilder() ClientConfigBuilder
```

###### Providers()

Get the custom provider configurations from this file config.

**Signature:**

```go
func (o *FileConfig) Providers() []FileProviderConfig
```


---

#### FileProviderConfig

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Name` | `string` | — | The name |
| `BaseUrl` | `string` | — | Base url |
| `AuthHeader` | `*string` | `nil` | Auth header |
| `ModelPrefixes` | `[]string` | — | Model prefixes |


---

#### FileRateLimitConfig

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Rpm` | `*uint32` | `nil` | Rpm |
| `Tpm` | `*uint64` | `nil` | Tpm |
| `WindowSeconds` | `*uint64` | `nil` | Window seconds |


---

#### FunctionCall

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Name` | `string` | — | The name |
| `Arguments` | `string` | — | Arguments |


---

#### FunctionDefinition

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Name` | `string` | — | The name |
| `Description` | `*string` | `nil` | Human-readable description |
| `Parameters` | `*interface{}` | `nil` | Parameters |
| `Strict` | `*bool` | `nil` | Strict |


---

#### FunctionMessage

Deprecated legacy function-role message body.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Content` | `string` | — | The extracted text content |
| `Name` | `string` | — | The name |


---

#### Image

A single generated image, returned as either a URL or base64 data.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Url` | `*string` | `nil` | Url |
| `B64Json` | `*string` | `nil` | B64 json |
| `RevisedPrompt` | `*string` | `nil` | Revised prompt |


---

#### ImageUrl

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Url` | `string` | — | Url |
| `Detail` | `*ImageDetail` | `nil` | Detail (image detail) |


---

#### ImagesResponse

Response containing generated images.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Created` | `uint64` | — | Created |
| `Data` | `[]Image` | `nil` | Data |


---

#### JsonSchemaFormat

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Name` | `string` | — | The name |
| `Description` | `*string` | `nil` | Human-readable description |
| `Schema` | `interface{}` | — | Schema |
| `Strict` | `*bool` | `nil` | Strict |


---

#### LiterLlmError

##### Methods

###### IsTransient()

Returns `true` for errors that are worth retrying on a different service
or deployment (transient failures).

Used by `crate.tower.fallback.FallbackService` and
`crate.tower.router.Router` to decide whether to route to an
alternative endpoint.

**Signature:**

```go
func (o *LiterLlmError) IsTransient() bool
```

###### ErrorType()

Return the OpenTelemetry `error.type` string for this error variant.

Used by the tracing middleware to record the `error.type` span attribute
on failed requests per the GenAI semantic conventions.

**Signature:**

```go
func (o *LiterLlmError) ErrorType() string
```

###### FromStatus()

Create from an HTTP status code, an API error response body, and an
optional `Retry-After` duration already parsed from the response header.

The `retry_after` value is forwarded into `LiterLlmError.RateLimited`
so callers can honour the server-requested delay without re-parsing the
header.

**Signature:**

```go
func (o *LiterLlmError) FromStatus(status uint16, body string, retryAfter time.Duration) LiterLlmError
```


---

#### LlmClient

Core LLM client trait.

##### Methods

###### Chat()

Send a chat completion request.

**Signature:**

```go
func (o *LlmClient) Chat(req ChatCompletionRequest) (ChatCompletionResponse, error)
```

###### ChatStream()

Send a streaming chat completion request.

**Signature:**

```go
func (o *LlmClient) ChatStream(req ChatCompletionRequest) (BoxStream, error)
```

###### Embed()

Send an embedding request.

**Signature:**

```go
func (o *LlmClient) Embed(req EmbeddingRequest) (EmbeddingResponse, error)
```

###### ListModels()

List available models.

**Signature:**

```go
func (o *LlmClient) ListModels() (ModelsListResponse, error)
```

###### ImageGenerate()

Generate an image.

**Signature:**

```go
func (o *LlmClient) ImageGenerate(req CreateImageRequest) (ImagesResponse, error)
```

###### Speech()

Generate speech audio from text.

**Signature:**

```go
func (o *LlmClient) Speech(req CreateSpeechRequest) ([]byte, error)
```

###### Transcribe()

Transcribe audio to text.

**Signature:**

```go
func (o *LlmClient) Transcribe(req CreateTranscriptionRequest) (TranscriptionResponse, error)
```

###### Moderate()

Check content against moderation policies.

**Signature:**

```go
func (o *LlmClient) Moderate(req ModerationRequest) (ModerationResponse, error)
```

###### Rerank()

Rerank documents by relevance to a query.

**Signature:**

```go
func (o *LlmClient) Rerank(req RerankRequest) (RerankResponse, error)
```

###### Search()

Perform a web/document search.

**Signature:**

```go
func (o *LlmClient) Search(req SearchRequest) (SearchResponse, error)
```

###### Ocr()

Extract text from a document via OCR.

**Signature:**

```go
func (o *LlmClient) Ocr(req OcrRequest) (OcrResponse, error)
```


---

#### LlmClientRaw

Extension of `LlmClient` that returns raw request/response data
alongside the typed response.

Every `_raw` method mirrors its counterpart on `LlmClient` but wraps the
result in a `RawExchange` that exposes the final request body (after
`transform_request`) and the raw provider response (before
`transform_response`). This is useful for debugging provider-specific
transformations, capturing wire-level data, or implementing custom parsing.

##### Methods

###### ChatRaw()

Send a chat completion request and return the raw exchange.

The `raw_request` field contains the final JSON body sent to the
provider; `raw_response` contains the provider JSON before
normalization.

**Signature:**

```go
func (o *LlmClientRaw) ChatRaw(req ChatCompletionRequest) (RawExchange, error)
```

###### ChatStreamRaw()

Send a streaming chat completion request and return the raw exchange.

Only `raw_request` is available upfront — the stream itself is
returned in `stream` and consumed incrementally.

**Signature:**

```go
func (o *LlmClientRaw) ChatStreamRaw(req ChatCompletionRequest) (RawStreamExchange, error)
```

###### EmbedRaw()

Send an embedding request and return the raw exchange.

**Signature:**

```go
func (o *LlmClientRaw) EmbedRaw(req EmbeddingRequest) (RawExchange, error)
```

###### ImageGenerateRaw()

Generate an image and return the raw exchange.

**Signature:**

```go
func (o *LlmClientRaw) ImageGenerateRaw(req CreateImageRequest) (RawExchange, error)
```

###### TranscribeRaw()

Transcribe audio to text and return the raw exchange.

**Signature:**

```go
func (o *LlmClientRaw) TranscribeRaw(req CreateTranscriptionRequest) (RawExchange, error)
```

###### ModerateRaw()

Check content against moderation policies and return the raw exchange.

**Signature:**

```go
func (o *LlmClientRaw) ModerateRaw(req ModerationRequest) (RawExchange, error)
```

###### RerankRaw()

Rerank documents by relevance to a query and return the raw exchange.

**Signature:**

```go
func (o *LlmClientRaw) RerankRaw(req RerankRequest) (RawExchange, error)
```

###### SearchRaw()

Perform a web/document search and return the raw exchange.

**Signature:**

```go
func (o *LlmClientRaw) SearchRaw(req SearchRequest) (RawExchange, error)
```

###### OcrRaw()

Extract text from a document via OCR and return the raw exchange.

**Signature:**

```go
func (o *LlmClientRaw) OcrRaw(req OcrRequest) (RawExchange, error)
```


---

#### ManagedClient

A managed LLM client that wraps `DefaultClient` with optional Tower
middleware (cache, cooldown, rate limiting, health checks, cost tracking,
budget, hooks, tracing).

Construct via `ManagedClient.new`.  If the provided `ClientConfig`
contains any middleware configuration the corresponding Tower layers are
composed into a service stack.  Otherwise requests pass straight through
to the inner `DefaultClient`.

`ManagedClient` implements `LlmClient` and can be used everywhere a
`DefaultClient` is expected.

##### Methods

###### New()

Build a managed client.

`model_hint` guides provider auto-detection — see
`DefaultClient.new` for details.

If the config contains any middleware settings (cache, budget, hooks,
cooldown, rate limit, health check, cost tracking, tracing) the
corresponding Tower layers are composed into a service stack.
Otherwise requests pass straight through to the inner client.

**Errors:**

Returns an error if the underlying `DefaultClient` cannot be
constructed (e.g. invalid headers or HTTP client build failure).

**Signature:**

```go
func (o *ManagedClient) New(config ClientConfig, modelHint string) (ManagedClient, error)
```

###### Inner()

Return a reference to the underlying `DefaultClient`.

**Signature:**

```go
func (o *ManagedClient) Inner() DefaultClient
```

###### BudgetState()

Return the budget state handle, if budget middleware is configured.

Use this to query accumulated spend at runtime.

**Signature:**

```go
func (o *ManagedClient) BudgetState() *BudgetState
```

###### HasMiddleware()

Return `true` when middleware is active (requests go through the Tower
service stack).

**Signature:**

```go
func (o *ManagedClient) HasMiddleware() bool
```

###### Chat()

**Signature:**

```go
func (o *ManagedClient) Chat(req ChatCompletionRequest) (ChatCompletionResponse, error)
```

###### ChatStream()

**Signature:**

```go
func (o *ManagedClient) ChatStream(req ChatCompletionRequest) (BoxStream, error)
```

###### Embed()

**Signature:**

```go
func (o *ManagedClient) Embed(req EmbeddingRequest) (EmbeddingResponse, error)
```

###### ListModels()

**Signature:**

```go
func (o *ManagedClient) ListModels() (ModelsListResponse, error)
```

###### ImageGenerate()

**Signature:**

```go
func (o *ManagedClient) ImageGenerate(req CreateImageRequest) (ImagesResponse, error)
```

###### Speech()

**Signature:**

```go
func (o *ManagedClient) Speech(req CreateSpeechRequest) ([]byte, error)
```

###### Transcribe()

**Signature:**

```go
func (o *ManagedClient) Transcribe(req CreateTranscriptionRequest) (TranscriptionResponse, error)
```

###### Moderate()

**Signature:**

```go
func (o *ManagedClient) Moderate(req ModerationRequest) (ModerationResponse, error)
```

###### Rerank()

**Signature:**

```go
func (o *ManagedClient) Rerank(req RerankRequest) (RerankResponse, error)
```

###### Search()

**Signature:**

```go
func (o *ManagedClient) Search(req SearchRequest) (SearchResponse, error)
```

###### Ocr()

**Signature:**

```go
func (o *ManagedClient) Ocr(req OcrRequest) (OcrResponse, error)
```

###### CreateFile()

**Signature:**

```go
func (o *ManagedClient) CreateFile(req CreateFileRequest) (FileObject, error)
```

###### RetrieveFile()

**Signature:**

```go
func (o *ManagedClient) RetrieveFile(fileId string) (FileObject, error)
```

###### DeleteFile()

**Signature:**

```go
func (o *ManagedClient) DeleteFile(fileId string) (DeleteResponse, error)
```

###### ListFiles()

**Signature:**

```go
func (o *ManagedClient) ListFiles(query FileListQuery) (FileListResponse, error)
```

###### FileContent()

**Signature:**

```go
func (o *ManagedClient) FileContent(fileId string) ([]byte, error)
```

###### CreateBatch()

**Signature:**

```go
func (o *ManagedClient) CreateBatch(req CreateBatchRequest) (BatchObject, error)
```

###### RetrieveBatch()

**Signature:**

```go
func (o *ManagedClient) RetrieveBatch(batchId string) (BatchObject, error)
```

###### ListBatches()

**Signature:**

```go
func (o *ManagedClient) ListBatches(query BatchListQuery) (BatchListResponse, error)
```

###### CancelBatch()

**Signature:**

```go
func (o *ManagedClient) CancelBatch(batchId string) (BatchObject, error)
```

###### CreateResponse()

**Signature:**

```go
func (o *ManagedClient) CreateResponse(req CreateResponseRequest) (ResponseObject, error)
```

###### RetrieveResponse()

**Signature:**

```go
func (o *ManagedClient) RetrieveResponse(id string) (ResponseObject, error)
```

###### CancelResponse()

**Signature:**

```go
func (o *ManagedClient) CancelResponse(id string) (ResponseObject, error)
```


---

#### ModelObject

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Id` | `string` | — | Unique identifier |
| `Object` | `string` | — | Always `"model"` from OpenAI-compatible APIs.  Stored as a plain `String` so non-standard provider values do not break deserialization. |
| `Created` | `uint64` | — | Created |
| `OwnedBy` | `string` | — | Owned by |


---

#### ModelsListResponse

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Object` | `string` | — | Always `"list"` from OpenAI-compatible APIs.  Stored as a plain `String` so non-standard provider values do not break deserialization. |
| `Data` | `[]ModelObject` | `nil` | Data |


---

#### ModerationCategories

Boolean flags for each moderation category.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Sexual` | `bool` | — | Sexual |
| `Hate` | `bool` | — | Hate |
| `Harassment` | `bool` | — | Harassment |
| `SelfHarm` | `bool` | — | Self harm |
| `SexualMinors` | `bool` | — | Sexual minors |
| `HateThreatening` | `bool` | — | Hate threatening |
| `ViolenceGraphic` | `bool` | — | Violence graphic |
| `SelfHarmIntent` | `bool` | — | Self harm intent |
| `SelfHarmInstructions` | `bool` | — | Self harm instructions |
| `HarassmentThreatening` | `bool` | — | Harassment threatening |
| `Violence` | `bool` | — | Violence |


---

#### ModerationCategoryScores

Confidence scores for each moderation category.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Sexual` | `float64` | — | Sexual |
| `Hate` | `float64` | — | Hate |
| `Harassment` | `float64` | — | Harassment |
| `SelfHarm` | `float64` | — | Self harm |
| `SexualMinors` | `float64` | — | Sexual minors |
| `HateThreatening` | `float64` | — | Hate threatening |
| `ViolenceGraphic` | `float64` | — | Violence graphic |
| `SelfHarmIntent` | `float64` | — | Self harm intent |
| `SelfHarmInstructions` | `float64` | — | Self harm instructions |
| `HarassmentThreatening` | `float64` | — | Harassment threatening |
| `Violence` | `float64` | — | Violence |


---

#### ModerationRequest

Request to classify content for policy violations.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Input` | `ModerationInput` | — | Input (moderation input) |
| `Model` | `*string` | `nil` | Model |


---

#### ModerationResponse

Response from the moderation endpoint.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Id` | `string` | — | Unique identifier |
| `Model` | `string` | — | Model |
| `Results` | `[]ModerationResult` | — | Results |


---

#### ModerationResult

A single moderation classification result.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Flagged` | `bool` | — | Flagged |
| `Categories` | `ModerationCategories` | — | Categories (moderation categories) |
| `CategoryScores` | `ModerationCategoryScores` | — | Category scores (moderation category scores) |


---

#### OcrImage

An image extracted from an OCR page.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Id` | `string` | — | Unique image identifier. |
| `ImageBase64` | `*string` | `nil` | Base64-encoded image data. |


---

#### OcrPage

A single page of OCR output.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Index` | `uint32` | — | Page index (0-based). |
| `Markdown` | `string` | — | Extracted content as Markdown. |
| `Images` | `*[]OcrImage` | `nil` | Extracted images, if `include_image_base64` was set. |
| `Dimensions` | `*PageDimensions` | `nil` | Page dimensions in pixels, if available. |


---

#### OcrRequest

An OCR request.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Model` | `string` | — | The model/provider to use (e.g. `"mistral/mistral-ocr-latest"`). |
| `Document` | `OcrDocument` | — | The document to process. |
| `Pages` | `*[]uint32` | `nil` | Specific pages to process (1-indexed). `None` means all pages. |
| `IncludeImageBase64` | `*bool` | `nil` | Whether to include base64-encoded images of each page. |


---

#### OcrResponse

An OCR response.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Pages` | `[]OcrPage` | — | Extracted pages. |
| `Model` | `string` | — | The model used. |
| `Usage` | `*Usage` | `nil` | Token usage, if reported by the provider. |


---

#### PageDimensions

Page dimensions in pixels.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Width` | `uint32` | — | Width in pixels. |
| `Height` | `uint32` | — | Height in pixels. |


---

#### PromptTokensDetails

Breakdown of tokens used in the prompt portion of a request.

`cached_tokens` is included in `Usage.prompt_tokens` — it is *not* an
additional charge on top of the prompt token count. When pricing supports
a `cache_read_input_token_cost`, the cached portion is billed at the
discounted rate and the remainder at the regular input rate.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `CachedTokens` | `uint64` | — | Cached tokens present in the prompt. Defaults to 0 when absent. |
| `AudioTokens` | `uint64` | — | Audio input tokens present in the prompt. Defaults to 0 when absent. |


---

#### RerankRequest

Request to rerank documents by relevance to a query.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Model` | `string` | — | Model |
| `Query` | `string` | — | Query |
| `Documents` | `[]RerankDocument` | — | Documents |
| `TopN` | `*uint32` | `nil` | Top n |
| `ReturnDocuments` | `*bool` | `nil` | Return documents |


---

#### RerankResponse

Response from the rerank endpoint.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Id` | `*string` | `nil` | Unique identifier |
| `Results` | `[]RerankResult` | — | Results |
| `Meta` | `*interface{}` | `nil` | Meta |


---

#### RerankResult

A single reranked document with its relevance score.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Index` | `uint32` | — | Index |
| `RelevanceScore` | `float64` | — | Relevance score |
| `Document` | `*RerankResultDocument` | `nil` | Document (rerank result document) |


---

#### RerankResultDocument

The text content of a reranked document, returned when `return_documents` is true.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Text` | `string` | — | Text |


---

#### ResponseClient

Responses API operations (create, retrieve, cancel).

##### Methods

###### CreateResponse()

Create a new response.

**Signature:**

```go
func (o *ResponseClient) CreateResponse(req CreateResponseRequest) (ResponseObject, error)
```

###### RetrieveResponse()

Retrieve a response by ID.

**Signature:**

```go
func (o *ResponseClient) RetrieveResponse(id string) (ResponseObject, error)
```

###### CancelResponse()

Cancel an in-progress response.

**Signature:**

```go
func (o *ResponseClient) CancelResponse(id string) (ResponseObject, error)
```


---

#### SearchRequest

A search request.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Model` | `string` | — | The model/provider to use (e.g. `"brave/web-search"`, `"tavily/search"`). |
| `Query` | `string` | — | The search query. |
| `MaxResults` | `*uint32` | `nil` | Maximum number of results to return. |
| `SearchDomainFilter` | `*[]string` | `nil` | Domain filter — restrict results to specific domains. |
| `Country` | `*string` | `nil` | Country code for localized results (ISO 3166-1 alpha-2). |


---

#### SearchResponse

A search response.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Results` | `[]SearchResult` | — | The search results. |
| `Model` | `string` | — | The model used. |


---

#### SearchResult

An individual search result.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Title` | `string` | — | Title of the result. |
| `Url` | `string` | — | URL of the result. |
| `Snippet` | `string` | — | Text snippet / excerpt. |
| `Date` | `*string` | `nil` | Publication or last-updated date, if available. |


---

#### SpecificFunction

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Name` | `string` | — | The name |


---

#### SpecificToolChoice

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `ChoiceType` | `ToolType` | `ToolType.Function` | Choice type (tool type) |
| `Function` | `SpecificFunction` | — | Function (specific function) |


---

#### StreamChoice

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Index` | `uint32` | — | Index |
| `Delta` | `StreamDelta` | — | Delta (stream delta) |
| `FinishReason` | `*FinishReason` | `nil` | Finish reason (finish reason) |


---

#### StreamDelta

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Role` | `*string` | `nil` | Role |
| `Content` | `*string` | `nil` | The extracted text content |
| `ToolCalls` | `*[]StreamToolCall` | `nil` | Tool calls |
| `FunctionCall` | `*StreamFunctionCall` | `nil` | Deprecated legacy function_call delta; retained for API compatibility. |
| `Refusal` | `*string` | `nil` | Refusal |


---

#### StreamFunctionCall

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Name` | `*string` | `nil` | The name |
| `Arguments` | `*string` | `nil` | Arguments |


---

#### StreamOptions

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `IncludeUsage` | `*bool` | `nil` | Include usage |


---

#### StreamToolCall

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Index` | `uint32` | — | Index |
| `Id` | `*string` | `nil` | Unique identifier |
| `CallType` | `*ToolType` | `nil` | Call type (tool type) |
| `Function` | `*StreamFunctionCall` | `nil` | Function (stream function call) |


---

#### SystemMessage

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Content` | `string` | — | The extracted text content |
| `Name` | `*string` | `nil` | The name |


---

#### ToolCall

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Id` | `string` | — | Unique identifier |
| `CallType` | `ToolType` | — | Call type (tool type) |
| `Function` | `FunctionCall` | — | Function (function call) |


---

#### ToolMessage

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Content` | `string` | — | The extracted text content |
| `ToolCallId` | `string` | — | Tool call id |
| `Name` | `*string` | `nil` | The name |


---

#### TranscriptionResponse

Response from a transcription request.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Text` | `string` | — | Text |
| `Language` | `*string` | `nil` | Language |
| `Duration` | `*float64` | `nil` | Duration |
| `Segments` | `*[]TranscriptionSegment` | `nil` | Segments |


---

#### TranscriptionSegment

A segment of transcribed audio with timing information.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Id` | `uint32` | — | Unique identifier |
| `Start` | `float64` | — | Start |
| `End` | `float64` | — | End |
| `Text` | `string` | — | Text |


---

#### Usage

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `PromptTokens` | `uint64` | — | Prompt tokens used. Defaults to 0 when absent (some providers omit this). |
| `CompletionTokens` | `uint64` | — | Completion tokens used. Defaults to 0 when absent (e.g. embedding responses). |
| `TotalTokens` | `uint64` | — | Total tokens used. Defaults to 0 when absent (some providers omit this). |
| `PromptTokensDetails` | `*PromptTokensDetails` | `nil` | Breakdown of tokens used in the prompt, including cached tokens served at the provider's discounted cache-read rate. Absent when the provider does not return prompt-token details. |


---

#### UserMessage

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Content` | `UserContent` | `UserContent.Text` | The extracted text content |
| `Name` | `*string` | `nil` | The name |


---

### Enums

#### Message

A chat message in a conversation.

| Value | Description |
|-------|-------------|
| `System` | System — Fields: `0`: `SystemMessage` |
| `User` | User — Fields: `0`: `UserMessage` |
| `Assistant` | Assistant — Fields: `0`: `AssistantMessage` |
| `Tool` | Tool — Fields: `0`: `ToolMessage` |
| `Developer` | Developer — Fields: `0`: `DeveloperMessage` |
| `Function` | Deprecated legacy function-role message; retained for API compatibility. — Fields: `0`: `FunctionMessage` |


---

#### UserContent

| Value | Description |
|-------|-------------|
| `Text` | Text format — Fields: `0`: `string` |
| `Parts` | Parts — Fields: `0`: `[]ContentPart` |


---

#### ContentPart

| Value | Description |
|-------|-------------|
| `Text` | Text format — Fields: `Text`: `string` |
| `ImageUrl` | Image url — Fields: `ImageUrl`: `ImageUrl` |
| `Document` | Document — Fields: `Document`: `DocumentContent` |
| `InputAudio` | Input audio — Fields: `InputAudio`: `AudioContent` |


---

#### ImageDetail

| Value | Description |
|-------|-------------|
| `Low` | Low |
| `High` | High |
| `Auto` | Auto |


---

#### ToolType

The type discriminator for tool/tool-call objects.

Per the OpenAI spec this is always `"function"`. Using an enum enforces
that constraint at the type level and rejects any other value on
deserialization.

| Value | Description |
|-------|-------------|
| `Function` | Function |


---

#### ToolChoice

| Value | Description |
|-------|-------------|
| `Mode` | Mode — Fields: `0`: `ToolChoiceMode` |
| `Specific` | Specific — Fields: `0`: `SpecificToolChoice` |


---

#### ToolChoiceMode

| Value | Description |
|-------|-------------|
| `Auto` | Auto |
| `Required` | Required |
| `None` | None |


---

#### ResponseFormat

| Value | Description |
|-------|-------------|
| `Text` | Text format |
| `JsonObject` | Json object |
| `JsonSchema` | Json schema — Fields: `JsonSchema`: `JsonSchemaFormat` |


---

#### StopSequence

| Value | Description |
|-------|-------------|
| `Single` | Single — Fields: `0`: `string` |
| `Multiple` | Multiple — Fields: `0`: `[]string` |


---

#### FinishReason

Why a choice stopped generating tokens.

| Value | Description |
|-------|-------------|
| `Stop` | Stop |
| `Length` | Length |
| `ToolCalls` | Tool calls |
| `ContentFilter` | Content filter |
| `FunctionCall` | Deprecated legacy finish reason; retained for API compatibility. |
| `Other` | Catch-all for unknown finish reasons returned by non-OpenAI providers. Note: this intentionally does **not** carry the original string (e.g. `Other(String)`).  Using `#[serde(other)]` requires a unit variant, and switching to `#[serde(untagged)]` would change deserialization semantics for all variants.  The original value can be recovered by inspecting the raw JSON if needed. |


---

#### ReasoningEffort

Controls how much reasoning effort the model should use.

| Value | Description |
|-------|-------------|
| `Low` | Low |
| `Medium` | Medium |
| `High` | High |


---

#### EmbeddingFormat

The format in which the embedding vectors are returned.

| Value | Description |
|-------|-------------|
| `Float` | 32-bit floating-point numbers (default). |
| `Base64` | Base64-encoded string representation of the floats. |


---

#### EmbeddingInput

| Value | Description |
|-------|-------------|
| `Single` | Single — Fields: `0`: `string` |
| `Multiple` | Multiple — Fields: `0`: `[]string` |


---

#### ModerationInput

Input to the moderation endpoint — a single string or multiple strings.

| Value | Description |
|-------|-------------|
| `Single` | Single — Fields: `0`: `string` |
| `Multiple` | Multiple — Fields: `0`: `[]string` |


---

#### RerankDocument

A document to be reranked — either a plain string or an object with a text field.

| Value | Description |
|-------|-------------|
| `Text` | Text format — Fields: `0`: `string` |
| `Object` | Object — Fields: `Text`: `string` |


---

#### OcrDocument

Document input for OCR — either a URL or inline base64 data.

| Value | Description |
|-------|-------------|
| `Url` | A publicly accessible document URL. — Fields: `Url`: `string` |
| `Base64` | Inline base64-encoded document data. — Fields: `Data`: `string`, `MediaType`: `string` |


---

#### AuthHeaderFormat

How the API key is sent in the HTTP request.

| Value | Description |
|-------|-------------|
| `Bearer` | Bearer token: `Authorization: Bearer <key>` |
| `ApiKey` | Custom header: e.g., `X-Api-Key: <key>` — Fields: `0`: `string` |
| `None` | No authentication required. |


---

### Errors

#### LiterLlmError

All errors that can occur when using `liter-llm`.

| Variant | Description |
|---------|-------------|
| `Authentication` | authentication failed: {message} |
| `RateLimited` | rate limited: {message} |
| `BadRequest` | bad request: {message} |
| `ContextWindowExceeded` | context window exceeded: {message} |
| `ContentPolicy` | content policy violation: {message} |
| `NotFound` | not found: {message} |
| `ServerError` | server error: {message} |
| `ServiceUnavailable` | service unavailable: {message} |
| `Timeout` | request timeout |
| `Streaming` | A catch-all for errors that occur during streaming response processing. This variant covers multiple sub-conditions including UTF-8 decoding failures, CRC/checksum mismatches (AWS EventStream), JSON parse errors in individual SSE chunks, and buffer overflow conditions.  The `message` field contains a human-readable description of the specific failure. |
| `EndpointNotSupported` | provider {provider} does not support {endpoint} |
| `InvalidHeader` | invalid header {name:?}: {reason} |
| `Serialization` | serialization error: {0} |
| `BudgetExceeded` | budget exceeded: {message} |
| `HookRejected` | hook rejected: {message} |
| `InternalError` | An internal logic error (e.g. unexpected Tower response variant). This should never surface in normal operation — if it does, it indicates a bug in the library. |


---

