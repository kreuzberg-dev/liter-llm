---
title: "PHP API Reference"
---

## PHP API Reference <span class="version-badge">v1.2.2</span>

### Functions

#### createClient()

Create a new LLM client with simple scalar configuration.

This is the primary binding entry-point. All parameters except `api_key`
are optional — omitting them uses the same defaults as
`ClientConfigBuilder`.

**Errors:**

Returns `LiterLlmError` if the underlying HTTP client cannot be
constructed, or if the resolved provider configuration is invalid.

**Signature:**

```php
public static function createClient(string $apiKey, ?string $baseUrl = null, ?int $timeoutSecs = null, ?int $maxRetries = null, ?string $modelHint = null): DefaultClient
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `apiKey` | `string` | Yes | The api key |
| `baseUrl` | `?string` | No | The base url |
| `timeoutSecs` | `?int` | No | The timeout secs |
| `maxRetries` | `?int` | No | The max retries |
| `modelHint` | `?string` | No | The model hint |

**Returns:** `DefaultClient`

**Errors:** Throws `Error`.


---

#### createClientFromJson()

Create a new LLM client from a JSON string.

The JSON object accepts the same fields as `liter-llm.toml` (snake_case).

**Errors:**

Returns `LiterLlmError::BadRequest` if `json` is not valid JSON or
contains unknown fields.

**Signature:**

```php
public static function createClientFromJson(string $json): DefaultClient
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `json` | `string` | Yes | The json |

**Returns:** `DefaultClient`

**Errors:** Throws `Error`.


---

#### registerCustomProvider()

Register a custom provider in the global runtime registry.

The provider will be checked **before** all built-in providers during model
detection. If a provider with the same `name` already exists it is replaced.

**Errors:**

Returns an error if the config is invalid (empty name, empty base_url, or
no model prefixes).

**Signature:**

```php
public static function registerCustomProvider(CustomProviderConfig $config): void
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `config` | `CustomProviderConfig` | Yes | The configuration options |

**Returns:** `void`

**Errors:** Throws `Error`.


---

#### unregisterCustomProvider()

Remove a previously registered custom provider by name.

Returns `true` if a provider with the given name was found and removed,
`false` if no such provider existed.

**Errors:**

Returns an error only if the internal lock is poisoned.

**Signature:**

```php
public static function unregisterCustomProvider(string $name): bool
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `name` | `string` | Yes | The name |

**Returns:** `bool`

**Errors:** Throws `Error`.


---

### Types

#### ApiError

Inner error object.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `message` | `string` | — | Message |
| `errorType` | `string` | — | Error type |
| `param` | `?string` | `null` | Param |
| `code` | `?string` | `null` | Code |


---

#### AssistantMessage

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `content` | `?string` | `null` | The extracted text content |
| `name` | `?string` | `null` | The name |
| `toolCalls` | `?array<ToolCall>` | `[]` | Tool calls |
| `refusal` | `?string` | `null` | Refusal |
| `functionCall` | `?FunctionCall` | `null` | Deprecated legacy function_call field; retained for API compatibility. |


---

#### AudioContent

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `data` | `string` | — | Base64-encoded audio data. |
| `format` | `string` | — | Audio format (e.g., "wav", "mp3", "ogg"). |


---

#### BatchClient

Batch processing operations (create, list, retrieve, cancel).

##### Methods

###### createBatch()

Create a new batch job.

**Signature:**

```php
public function createBatch(CreateBatchRequest $req): BatchObject
```

###### retrieveBatch()

Retrieve a batch by ID.

**Signature:**

```php
public function retrieveBatch(string $batchId): BatchObject
```

###### listBatches()

List batches, optionally filtered by query parameters.

**Signature:**

```php
public function listBatches(BatchListQuery $query): BatchListResponse
```

###### cancelBatch()

Cancel an in-progress batch.

**Signature:**

```php
public function cancelBatch(string $batchId): BatchObject
```


---

#### ChatCompletionChunk

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `id` | `string` | — | Unique identifier |
| `object` | `string` | — | Always `"chat.completion.chunk"` from OpenAI-compatible APIs.  Stored as a plain `String` so non-standard provider values do not fail parsing. |
| `created` | `int` | — | Created |
| `model` | `string` | — | Model |
| `choices` | `array<StreamChoice>` | `[]` | Choices |
| `usage` | `?Usage` | `null` | Usage (usage) |
| `systemFingerprint` | `?string` | `null` | System fingerprint |
| `serviceTier` | `?string` | `null` | Service tier |


---

#### ChatCompletionRequest

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `model` | `string` | — | Model |
| `messages` | `array<Message>` | `[]` | Messages |
| `temperature` | `?float` | `null` | Temperature |
| `topP` | `?float` | `null` | Top p |
| `n` | `?int` | `null` | N |
| `stream` | `?bool` | `null` | Whether to stream the response. Managed by the client layer — do not set directly. |
| `stop` | `?StopSequence` | `null` | Stop (stop sequence) |
| `maxTokens` | `?int` | `null` | Maximum tokens |
| `presencePenalty` | `?float` | `null` | Presence penalty |
| `frequencyPenalty` | `?float` | `null` | Frequency penalty |
| `logitBias` | `?array<string, float>` | `{}` | Token bias map.  Uses `BTreeMap` (sorted keys) for deterministic serialization order — important when hashing or signing requests. |
| `user` | `?string` | `null` | User |
| `tools` | `?array<ChatCompletionTool>` | `[]` | Tools |
| `toolChoice` | `?ToolChoice` | `null` | Tool choice (tool choice) |
| `parallelToolCalls` | `?bool` | `null` | Parallel tool calls |
| `responseFormat` | `?ResponseFormat` | `null` | Response format (response format) |
| `streamOptions` | `?StreamOptions` | `null` | Stream options (stream options) |
| `seed` | `?int` | `null` | Seed |
| `reasoningEffort` | `?ReasoningEffort` | `null` | Reasoning effort (reasoning effort) |
| `extraBody` | `?mixed` | `null` | Provider-specific extra parameters merged into the request body. Use for guardrails, safety settings, grounding config, etc. |


---

#### ChatCompletionResponse

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `id` | `string` | — | Unique identifier |
| `object` | `string` | — | Always `"chat.completion"` from OpenAI-compatible APIs.  Stored as a plain `String` so non-standard provider values do not break deserialization. |
| `created` | `int` | — | Created |
| `model` | `string` | — | Model |
| `choices` | `array<Choice>` | `[]` | Choices |
| `usage` | `?Usage` | `null` | Usage (usage) |
| `systemFingerprint` | `?string` | `null` | System fingerprint |
| `serviceTier` | `?string` | `null` | Service tier |

##### Methods

###### estimatedCost()

Estimate the cost of this response based on embedded pricing data.

Returns `null` if:

- the `model` field is not present in the embedded pricing registry, or
- the `usage` field is absent from the response.

**Signature:**

```php
public function estimatedCost(): ?float
```


---

#### ChatCompletionTool

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `toolType` | `ToolType` | — | Tool type (tool type) |
| `function` | `FunctionDefinition` | — | Function (function definition) |


---

#### Choice

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `index` | `int` | — | Index |
| `message` | `AssistantMessage` | — | Message (assistant message) |
| `finishReason` | `?FinishReason` | `null` | Finish reason (finish reason) |


---

#### ClientConfig

Configuration for an LLM client.

`api_key` is stored as a `SecretString` so it is zeroed on drop and never
printed accidentally. Access it via `secrecy::ExposeSecret`.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `apiKey` | `string` | — | API key for authentication (stored as a secret). |
| `baseUrl` | `?string` | `null` | Override base URL.  When set, all requests go here regardless of model name, and provider auto-detection is skipped. |
| `timeout` | `float` | — | Request timeout. |
| `maxRetries` | `int` | — | Maximum number of retries on 429 / 5xx responses. |
| `credentialProvider` | `?CredentialProvider` | `null` | Optional dynamic credential provider for token-based auth (Azure AD, Vertex OAuth2) or refreshable credentials (AWS STS). When set, the client calls `resolve()` before each request to obtain a fresh credential.  When `None`, the static `api_key` is used. |

##### Methods

###### headers()

Return the extra headers as an ordered slice of `(name, value)` pairs.

**Signature:**

```php
public function headers(): array<array{string, string}>
```

###### fmt()

**Signature:**

```php
public function fmt(Formatter $f): Unknown
```


---

#### ClientConfigBuilder

Builder for `ClientConfig`.

Construct with `ClientConfigBuilder::new` and call builder methods to
customise the configuration, then call `ClientConfigBuilder::build` to
obtain a `ClientConfig`.

##### Methods

###### baseUrl()

Override the provider base URL for all requests.

**Signature:**

```php
public function baseUrl(string $url): ClientConfigBuilder
```

###### timeout()

Set the per-request timeout (default: 60 s).

**Signature:**

```php
public function timeout(float $timeout): ClientConfigBuilder
```

###### maxRetries()

Set the maximum number of retries on 429 / 5xx responses (default: 3).

**Signature:**

```php
public function maxRetries(int $retries): ClientConfigBuilder
```

###### credentialProvider()

Set a dynamic credential provider for token-based or refreshable auth.

When configured, the client calls `resolve()` before each request
instead of using the static `api_key` for authentication.

**Signature:**

```php
public function credentialProvider(CredentialProvider $provider): ClientConfigBuilder
```

###### header()

Add a custom header sent on every request.

Returns an error if either `key` or `value` is not a valid HTTP header
name / value.

This method is only available when the `native-http` feature is enabled
because header validation relies on `reqwest`'s header types.

**Signature:**

```php
public function header(string $key, string $value): ClientConfigBuilder
```

###### cache()

Set the response cache configuration for the Tower middleware stack.

When set, bindings and advanced Rust users can read this from the
built `ClientConfig` to construct a
`CacheLayer`.

**Signature:**

```php
public function cache(CacheConfig $config): ClientConfigBuilder
```

###### cacheStore()

Set a custom cache store backend for the Tower cache middleware.

When set alongside `cache`, the cache layer will use
this store instead of the default in-memory LRU.

**Signature:**

```php
public function cacheStore(CacheStore $store): ClientConfigBuilder
```

###### budget()

Set the budget enforcement configuration for the Tower middleware stack.

When set, bindings and advanced Rust users can read this from the
built `ClientConfig` to construct a
`BudgetLayer`.

**Signature:**

```php
public function budget(BudgetConfig $config): ClientConfigBuilder
```

###### hook()

Add a single hook to the Tower hooks middleware stack.

Hooks are invoked sequentially in registration order at request
lifecycle points (pre-request, post-response, on-error).

**Signature:**

```php
public function hook(LlmHook $hook): ClientConfigBuilder
```

###### hooks()

Set the full list of hooks for the Tower hooks middleware stack,
replacing any previously registered hooks.

Hooks are invoked sequentially in registration order.

**Signature:**

```php
public function hooks(array<LlmHook> $hooks): ClientConfigBuilder
```

###### cooldown()

Set the cooldown duration after transient errors.

When set, the client rejects requests with `ServiceUnavailable` for
the given duration after a transient error (rate limit, timeout,
server error).

**Signature:**

```php
public function cooldown(float $duration): ClientConfigBuilder
```

###### rateLimit()

Set per-model rate limiting configuration.

When set, requests exceeding the configured RPM or TPM limits are
rejected with `LiterLlmError::RateLimited`.

**Signature:**

```php
public function rateLimit(RateLimitConfig $config): ClientConfigBuilder
```

###### healthCheck()

Set the background health check interval.

When set, the client periodically probes the provider and rejects
requests when the provider is unhealthy.

**Signature:**

```php
public function healthCheck(float $interval): ClientConfigBuilder
```

###### costTracking()

Enable or disable per-request cost tracking.

When enabled, estimated USD cost is recorded on the current tracing
span as `gen_ai.usage.cost`.

**Signature:**

```php
public function costTracking(bool $enabled): ClientConfigBuilder
```

###### tracing()

Enable or disable OpenTelemetry-compatible tracing spans.

When enabled, every request is wrapped in a `gen_ai` tracing span
with semantic convention attributes.

**Signature:**

```php
public function tracing(bool $enabled): ClientConfigBuilder
```

###### build()

Consume the builder and return the completed `ClientConfig`.

**Signature:**

```php
public function build(): ClientConfig
```


---

#### CreateImageRequest

Request to create images from a text prompt.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `prompt` | `string` | — | Prompt |
| `model` | `?string` | `null` | Model |
| `n` | `?int` | `null` | N |
| `size` | `?string` | `null` | Size in bytes |
| `quality` | `?string` | `null` | Quality |
| `style` | `?string` | `null` | Style |
| `responseFormat` | `?string` | `null` | Response format |
| `user` | `?string` | `null` | User |


---

#### CreateSpeechRequest

Request to generate speech audio from text.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `model` | `string` | — | Model |
| `input` | `string` | — | Input |
| `voice` | `string` | — | Voice |
| `responseFormat` | `?string` | `null` | Response format |
| `speed` | `?float` | `null` | Speed |


---

#### CreateTranscriptionRequest

Request to transcribe audio into text.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `model` | `string` | — | Model |
| `file` | `string` | — | Base64-encoded audio file data. |
| `language` | `?string` | `null` | Language |
| `prompt` | `?string` | `null` | Prompt |
| `responseFormat` | `?string` | `null` | Response format |
| `temperature` | `?float` | `null` | Temperature |


---

#### CustomProviderConfig

Configuration for registering a custom LLM provider at runtime.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `name` | `string` | — | Unique name for this provider (e.g., "my-provider"). |
| `baseUrl` | `string` | — | Base URL for the provider's API (e.g., "<https://api.my-provider.com/v1">). |
| `authHeader` | `AuthHeaderFormat` | — | Authentication header format. |
| `modelPrefixes` | `array<string>` | — | Model name prefixes that route to this provider (e.g., ["my-"]). |


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

###### new()

Build a client.

`model_hint` guides provider auto-detection when no explicit
`base_url` override is present in the config. For example, passing
`Some("groq/llama3-70b")` selects the Groq provider. Pass `null` to
default to OpenAI.

**Errors:**

Returns a wrapped `reqwest::Error` if the underlying HTTP client
cannot be constructed. Header names and values are pre-validated by
`ClientConfigBuilder::header`, so they are inserted directly here.

**Signature:**

```php
public static function new(ClientConfig $config, string $modelHint): DefaultClient
```

###### chat()

**Signature:**

```php
public function chat(ChatCompletionRequest $req): ChatCompletionResponse
```

###### chatStream()

**Signature:**

```php
public function chatStream(ChatCompletionRequest $req): BoxStream
```

###### embed()

**Signature:**

```php
public function embed(EmbeddingRequest $req): EmbeddingResponse
```

###### listModels()

**Signature:**

```php
public function listModels(): ModelsListResponse
```

###### imageGenerate()

**Signature:**

```php
public function imageGenerate(CreateImageRequest $req): ImagesResponse
```

###### speech()

**Signature:**

```php
public function speech(CreateSpeechRequest $req): string
```

###### transcribe()

**Signature:**

```php
public function transcribe(CreateTranscriptionRequest $req): TranscriptionResponse
```

###### moderate()

**Signature:**

```php
public function moderate(ModerationRequest $req): ModerationResponse
```

###### rerank()

**Signature:**

```php
public function rerank(RerankRequest $req): RerankResponse
```

###### search()

**Signature:**

```php
public function search(SearchRequest $req): SearchResponse
```

###### ocr()

**Signature:**

```php
public function ocr(OcrRequest $req): OcrResponse
```

###### chatRaw()

**Signature:**

```php
public function chatRaw(ChatCompletionRequest $req): RawExchange
```

###### chatStreamRaw()

**Signature:**

```php
public function chatStreamRaw(ChatCompletionRequest $req): RawStreamExchange
```

###### embedRaw()

**Signature:**

```php
public function embedRaw(EmbeddingRequest $req): RawExchange
```

###### imageGenerateRaw()

**Signature:**

```php
public function imageGenerateRaw(CreateImageRequest $req): RawExchange
```

###### transcribeRaw()

**Signature:**

```php
public function transcribeRaw(CreateTranscriptionRequest $req): RawExchange
```

###### moderateRaw()

**Signature:**

```php
public function moderateRaw(ModerationRequest $req): RawExchange
```

###### rerankRaw()

**Signature:**

```php
public function rerankRaw(RerankRequest $req): RawExchange
```

###### searchRaw()

**Signature:**

```php
public function searchRaw(SearchRequest $req): RawExchange
```

###### ocrRaw()

**Signature:**

```php
public function ocrRaw(OcrRequest $req): RawExchange
```

###### createFile()

**Signature:**

```php
public function createFile(CreateFileRequest $req): FileObject
```

###### retrieveFile()

**Signature:**

```php
public function retrieveFile(string $fileId): FileObject
```

###### deleteFile()

**Signature:**

```php
public function deleteFile(string $fileId): DeleteResponse
```

###### listFiles()

**Signature:**

```php
public function listFiles(FileListQuery $query): FileListResponse
```

###### fileContent()

**Signature:**

```php
public function fileContent(string $fileId): string
```

###### createBatch()

**Signature:**

```php
public function createBatch(CreateBatchRequest $req): BatchObject
```

###### retrieveBatch()

**Signature:**

```php
public function retrieveBatch(string $batchId): BatchObject
```

###### listBatches()

**Signature:**

```php
public function listBatches(BatchListQuery $query): BatchListResponse
```

###### cancelBatch()

**Signature:**

```php
public function cancelBatch(string $batchId): BatchObject
```

###### createResponse()

**Signature:**

```php
public function createResponse(CreateResponseRequest $req): ResponseObject
```

###### retrieveResponse()

**Signature:**

```php
public function retrieveResponse(string $id): ResponseObject
```

###### cancelResponse()

**Signature:**

```php
public function cancelResponse(string $id): ResponseObject
```


---

#### DeveloperMessage

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `content` | `string` | — | The extracted text content |
| `name` | `?string` | `null` | The name |


---

#### DocumentContent

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `data` | `string` | — | Base64-encoded document data or URL. |
| `mediaType` | `string` | — | MIME type (e.g., "application/pdf", "text/csv"). |


---

#### EmbeddingObject

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `object` | `string` | — | Always `"embedding"` from OpenAI-compatible APIs.  Stored as a plain `String` so non-standard provider values do not break deserialization. |
| `embedding` | `array<float>` | — | Embedding |
| `index` | `int` | — | Index |


---

#### EmbeddingRequest

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `model` | `string` | — | Model |
| `input` | `EmbeddingInput` | — | Input (embedding input) |
| `encodingFormat` | `?EmbeddingFormat` | `null` | Encoding format (embedding format) |
| `dimensions` | `?int` | `null` | Dimensions |
| `user` | `?string` | `null` | User |


---

#### EmbeddingResponse

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `object` | `string` | — | Always `"list"` from OpenAI-compatible APIs.  Stored as a plain `String` so non-standard provider values do not break deserialization. |
| `data` | `array<EmbeddingObject>` | — | Data |
| `model` | `string` | — | Model |
| `usage` | `?Usage` | `null` | Usage (usage) |

##### Methods

###### estimatedCost()

Estimate the cost of this embedding request based on embedded pricing data.

Returns `null` if:

- the `model` field is not present in the embedded pricing registry, or
- the `usage` field is absent from the response.

Embedding models only charge for input tokens; output cost is zero.

**Signature:**

```php
public function estimatedCost(): ?float
```


---

#### ErrorResponse

Error response from an OpenAI-compatible API.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `error` | `ApiError` | — | Error (api error) |


---

#### FileBudgetConfig

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `globalLimit` | `?float` | `null` | Global limit |
| `modelLimits` | `?array<string, float>` | `null` | Model limits |
| `enforcement` | `?string` | `null` | Enforcement |


---

#### FileCacheConfig

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `maxEntries` | `?int` | `null` | Maximum entries |
| `ttlSeconds` | `?int` | `null` | Ttl seconds |
| `backend` | `?string` | `null` | Backend |
| `backendConfig` | `?array<string, string>` | `null` | Backend config |


---

#### FileClient

File management operations (upload, list, retrieve, delete).

##### Methods

###### createFile()

Upload a file.

**Signature:**

```php
public function createFile(CreateFileRequest $req): FileObject
```

###### retrieveFile()

Retrieve metadata for a file.

**Signature:**

```php
public function retrieveFile(string $fileId): FileObject
```

###### deleteFile()

Delete a file.

**Signature:**

```php
public function deleteFile(string $fileId): DeleteResponse
```

###### listFiles()

List files, optionally filtered by query parameters.

**Signature:**

```php
public function listFiles(FileListQuery $query): FileListResponse
```

###### fileContent()

Retrieve the raw content of a file.

**Signature:**

```php
public function fileContent(string $fileId): string
```


---

#### FileConfig

TOML file representation of client configuration.

All fields are optional — missing fields use defaults from `ClientConfigBuilder`.
Convert to a builder via `FileConfig::into_builder`.

## Example `liter-llm.toml`

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
| `apiKey` | `?string` | `null` | Api key |
| `baseUrl` | `?string` | `null` | Base url |
| `modelHint` | `?string` | `null` | Model hint |
| `timeoutSecs` | `?int` | `null` | Timeout secs |
| `maxRetries` | `?int` | `null` | Maximum retries |
| `extraHeaders` | `?array<string, string>` | `null` | Extra headers |
| `cache` | `?FileCacheConfig` | `null` | Cache (file cache config) |
| `budget` | `?FileBudgetConfig` | `null` | Budget (file budget config) |
| `cooldownSecs` | `?int` | `null` | Cooldown secs |
| `rateLimit` | `?FileRateLimitConfig` | `null` | Rate limit (file rate limit config) |
| `healthCheckSecs` | `?int` | `null` | Health check secs |
| `costTracking` | `?bool` | `null` | Cost tracking |
| `tracing` | `?bool` | `null` | Tracing |
| `providers` | `?array<FileProviderConfig>` | `null` | Providers |

### Methods

#### fromTomlFile()

Load from a TOML file path.

**Signature:**

```php
public static function fromTomlFile(Path $path): FileConfig
```

##### fromTomlStr()

Parse from a TOML string.

**Signature:**

```php
public static function fromTomlStr(string $s): FileConfig
```

###### discover()

Discover `liter-llm.toml` by walking from current directory to filesystem root.

Returns `Ok(None)` if no config file is found.

**Signature:**

```php
public static function discover(): ?FileConfig
```

###### intoBuilder()

Convert into a `ClientConfigBuilder`,
applying all fields that are set.

Fields not present in the TOML file use the builder's defaults.

**Signature:**

```php
public function intoBuilder(): ClientConfigBuilder
```

###### providers()

Get the custom provider configurations from this file config.

**Signature:**

```php
public function providers(): array<FileProviderConfig>
```


---

##### FileProviderConfig

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `name` | `string` | — | The name |
| `baseUrl` | `string` | — | Base url |
| `authHeader` | `?string` | `null` | Auth header |
| `modelPrefixes` | `array<string>` | — | Model prefixes |


---

##### FileRateLimitConfig

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `rpm` | `?int` | `null` | Rpm |
| `tpm` | `?int` | `null` | Tpm |
| `windowSeconds` | `?int` | `null` | Window seconds |


---

##### FunctionCall

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `name` | `string` | — | The name |
| `arguments` | `string` | — | Arguments |


---

##### FunctionDefinition

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `name` | `string` | — | The name |
| `description` | `?string` | `null` | Human-readable description |
| `parameters` | `?mixed` | `null` | Parameters |
| `strict` | `?bool` | `null` | Strict |


---

##### FunctionMessage

Deprecated legacy function-role message body.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `content` | `string` | — | The extracted text content |
| `name` | `string` | — | The name |


---

##### Image

A single generated image, returned as either a URL or base64 data.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `url` | `?string` | `null` | Url |
| `b64Json` | `?string` | `null` | B64 json |
| `revisedPrompt` | `?string` | `null` | Revised prompt |


---

##### ImageUrl

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `url` | `string` | — | Url |
| `detail` | `?ImageDetail` | `null` | Detail (image detail) |


---

##### ImagesResponse

Response containing generated images.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `created` | `int` | — | Created |
| `data` | `array<Image>` | `[]` | Data |


---

##### JsonSchemaFormat

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `name` | `string` | — | The name |
| `description` | `?string` | `null` | Human-readable description |
| `schema` | `mixed` | — | Schema |
| `strict` | `?bool` | `null` | Strict |


---

##### LiterLlmError

###### Methods

###### isTransient()

Returns `true` for errors that are worth retrying on a different service
or deployment (transient failures).

Used by `crate::tower::fallback::FallbackService` and
`crate::tower::router::Router` to decide whether to route to an
alternative endpoint.

**Signature:**

```php
public function isTransient(): bool
```

###### errorType()

Return the OpenTelemetry `error.type` string for this error variant.

Used by the tracing middleware to record the `error.type` span attribute
on failed requests per the GenAI semantic conventions.

**Signature:**

```php
public function errorType(): string
```

###### fromStatus()

Create from an HTTP status code, an API error response body, and an
optional `Retry-After` duration already parsed from the response header.

The `retry_after` value is forwarded into `LiterLlmError::RateLimited`
so callers can honour the server-requested delay without re-parsing the
header.

**Signature:**

```php
public static function fromStatus(int $status, string $body, float $retryAfter): LiterLlmError
```


---

##### LlmClient

Core LLM client trait.

###### Methods

###### chat()

Send a chat completion request.

**Signature:**

```php
public function chat(ChatCompletionRequest $req): ChatCompletionResponse
```

###### chatStream()

Send a streaming chat completion request.

**Signature:**

```php
public function chatStream(ChatCompletionRequest $req): BoxStream
```

###### embed()

Send an embedding request.

**Signature:**

```php
public function embed(EmbeddingRequest $req): EmbeddingResponse
```

###### listModels()

List available models.

**Signature:**

```php
public function listModels(): ModelsListResponse
```

###### imageGenerate()

Generate an image.

**Signature:**

```php
public function imageGenerate(CreateImageRequest $req): ImagesResponse
```

###### speech()

Generate speech audio from text.

**Signature:**

```php
public function speech(CreateSpeechRequest $req): string
```

###### transcribe()

Transcribe audio to text.

**Signature:**

```php
public function transcribe(CreateTranscriptionRequest $req): TranscriptionResponse
```

###### moderate()

Check content against moderation policies.

**Signature:**

```php
public function moderate(ModerationRequest $req): ModerationResponse
```

###### rerank()

Rerank documents by relevance to a query.

**Signature:**

```php
public function rerank(RerankRequest $req): RerankResponse
```

###### search()

Perform a web/document search.

**Signature:**

```php
public function search(SearchRequest $req): SearchResponse
```

###### ocr()

Extract text from a document via OCR.

**Signature:**

```php
public function ocr(OcrRequest $req): OcrResponse
```


---

##### LlmClientRaw

Extension of `LlmClient` that returns raw request/response data
alongside the typed response.

Every `_raw` method mirrors its counterpart on `LlmClient` but wraps the
result in a `RawExchange` that exposes the final request body (after
`transform_request`) and the raw provider response (before
`transform_response`). This is useful for debugging provider-specific
transformations, capturing wire-level data, or implementing custom parsing.

###### Methods

###### chatRaw()

Send a chat completion request and return the raw exchange.

The `raw_request` field contains the final JSON body sent to the
provider; `raw_response` contains the provider JSON before
normalization.

**Signature:**

```php
public function chatRaw(ChatCompletionRequest $req): RawExchange
```

###### chatStreamRaw()

Send a streaming chat completion request and return the raw exchange.

Only `raw_request` is available upfront — the stream itself is
returned in `stream` and consumed incrementally.

**Signature:**

```php
public function chatStreamRaw(ChatCompletionRequest $req): RawStreamExchange
```

###### embedRaw()

Send an embedding request and return the raw exchange.

**Signature:**

```php
public function embedRaw(EmbeddingRequest $req): RawExchange
```

###### imageGenerateRaw()

Generate an image and return the raw exchange.

**Signature:**

```php
public function imageGenerateRaw(CreateImageRequest $req): RawExchange
```

###### transcribeRaw()

Transcribe audio to text and return the raw exchange.

**Signature:**

```php
public function transcribeRaw(CreateTranscriptionRequest $req): RawExchange
```

###### moderateRaw()

Check content against moderation policies and return the raw exchange.

**Signature:**

```php
public function moderateRaw(ModerationRequest $req): RawExchange
```

###### rerankRaw()

Rerank documents by relevance to a query and return the raw exchange.

**Signature:**

```php
public function rerankRaw(RerankRequest $req): RawExchange
```

###### searchRaw()

Perform a web/document search and return the raw exchange.

**Signature:**

```php
public function searchRaw(SearchRequest $req): RawExchange
```

###### ocrRaw()

Extract text from a document via OCR and return the raw exchange.

**Signature:**

```php
public function ocrRaw(OcrRequest $req): RawExchange
```


---

##### ManagedClient

A managed LLM client that wraps `DefaultClient` with optional Tower
middleware (cache, cooldown, rate limiting, health checks, cost tracking,
budget, hooks, tracing).

Construct via `ManagedClient::new`. If the provided `ClientConfig`
contains any middleware configuration the corresponding Tower layers are
composed into a service stack. Otherwise requests pass straight through
to the inner `DefaultClient`.

`ManagedClient` implements `LlmClient` and can be used everywhere a
`DefaultClient` is expected.

###### Methods

###### new()

Build a managed client.

`model_hint` guides provider auto-detection — see
`DefaultClient::new` for details.

If the config contains any middleware settings (cache, budget, hooks,
cooldown, rate limit, health check, cost tracking, tracing) the
corresponding Tower layers are composed into a service stack.
Otherwise requests pass straight through to the inner client.

**Errors:**

Returns an error if the underlying `DefaultClient` cannot be
constructed (e.g. invalid headers or HTTP client build failure).

**Signature:**

```php
public static function new(ClientConfig $config, string $modelHint): ManagedClient
```

###### inner()

Return a reference to the underlying `DefaultClient`.

**Signature:**

```php
public function inner(): DefaultClient
```

###### budgetState()

Return the budget state handle, if budget middleware is configured.

Use this to query accumulated spend at runtime.

**Signature:**

```php
public function budgetState(): ?BudgetState
```

###### hasMiddleware()

Return `true` when middleware is active (requests go through the Tower
service stack).

**Signature:**

```php
public function hasMiddleware(): bool
```

###### chat()

**Signature:**

```php
public function chat(ChatCompletionRequest $req): ChatCompletionResponse
```

###### chatStream()

**Signature:**

```php
public function chatStream(ChatCompletionRequest $req): BoxStream
```

###### embed()

**Signature:**

```php
public function embed(EmbeddingRequest $req): EmbeddingResponse
```

###### listModels()

**Signature:**

```php
public function listModels(): ModelsListResponse
```

###### imageGenerate()

**Signature:**

```php
public function imageGenerate(CreateImageRequest $req): ImagesResponse
```

###### speech()

**Signature:**

```php
public function speech(CreateSpeechRequest $req): string
```

###### transcribe()

**Signature:**

```php
public function transcribe(CreateTranscriptionRequest $req): TranscriptionResponse
```

###### moderate()

**Signature:**

```php
public function moderate(ModerationRequest $req): ModerationResponse
```

###### rerank()

**Signature:**

```php
public function rerank(RerankRequest $req): RerankResponse
```

###### search()

**Signature:**

```php
public function search(SearchRequest $req): SearchResponse
```

###### ocr()

**Signature:**

```php
public function ocr(OcrRequest $req): OcrResponse
```

###### createFile()

**Signature:**

```php
public function createFile(CreateFileRequest $req): FileObject
```

###### retrieveFile()

**Signature:**

```php
public function retrieveFile(string $fileId): FileObject
```

###### deleteFile()

**Signature:**

```php
public function deleteFile(string $fileId): DeleteResponse
```

###### listFiles()

**Signature:**

```php
public function listFiles(FileListQuery $query): FileListResponse
```

###### fileContent()

**Signature:**

```php
public function fileContent(string $fileId): string
```

###### createBatch()

**Signature:**

```php
public function createBatch(CreateBatchRequest $req): BatchObject
```

###### retrieveBatch()

**Signature:**

```php
public function retrieveBatch(string $batchId): BatchObject
```

###### listBatches()

**Signature:**

```php
public function listBatches(BatchListQuery $query): BatchListResponse
```

###### cancelBatch()

**Signature:**

```php
public function cancelBatch(string $batchId): BatchObject
```

###### createResponse()

**Signature:**

```php
public function createResponse(CreateResponseRequest $req): ResponseObject
```

###### retrieveResponse()

**Signature:**

```php
public function retrieveResponse(string $id): ResponseObject
```

###### cancelResponse()

**Signature:**

```php
public function cancelResponse(string $id): ResponseObject
```


---

##### ModelObject

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `id` | `string` | — | Unique identifier |
| `object` | `string` | — | Always `"model"` from OpenAI-compatible APIs.  Stored as a plain `String` so non-standard provider values do not break deserialization. |
| `created` | `int` | — | Created |
| `ownedBy` | `string` | — | Owned by |


---

##### ModelsListResponse

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `object` | `string` | — | Always `"list"` from OpenAI-compatible APIs.  Stored as a plain `String` so non-standard provider values do not break deserialization. |
| `data` | `array<ModelObject>` | `[]` | Data |


---

##### ModerationCategories

Boolean flags for each moderation category.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `sexual` | `bool` | — | Sexual |
| `hate` | `bool` | — | Hate |
| `harassment` | `bool` | — | Harassment |
| `selfHarm` | `bool` | — | Self harm |
| `sexualMinors` | `bool` | — | Sexual minors |
| `hateThreatening` | `bool` | — | Hate threatening |
| `violenceGraphic` | `bool` | — | Violence graphic |
| `selfHarmIntent` | `bool` | — | Self harm intent |
| `selfHarmInstructions` | `bool` | — | Self harm instructions |
| `harassmentThreatening` | `bool` | — | Harassment threatening |
| `violence` | `bool` | — | Violence |


---

##### ModerationCategoryScores

Confidence scores for each moderation category.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `sexual` | `float` | — | Sexual |
| `hate` | `float` | — | Hate |
| `harassment` | `float` | — | Harassment |
| `selfHarm` | `float` | — | Self harm |
| `sexualMinors` | `float` | — | Sexual minors |
| `hateThreatening` | `float` | — | Hate threatening |
| `violenceGraphic` | `float` | — | Violence graphic |
| `selfHarmIntent` | `float` | — | Self harm intent |
| `selfHarmInstructions` | `float` | — | Self harm instructions |
| `harassmentThreatening` | `float` | — | Harassment threatening |
| `violence` | `float` | — | Violence |


---

##### ModerationRequest

Request to classify content for policy violations.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `input` | `ModerationInput` | — | Input (moderation input) |
| `model` | `?string` | `null` | Model |


---

##### ModerationResponse

Response from the moderation endpoint.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `id` | `string` | — | Unique identifier |
| `model` | `string` | — | Model |
| `results` | `array<ModerationResult>` | — | Results |


---

##### ModerationResult

A single moderation classification result.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `flagged` | `bool` | — | Flagged |
| `categories` | `ModerationCategories` | — | Categories (moderation categories) |
| `categoryScores` | `ModerationCategoryScores` | — | Category scores (moderation category scores) |


---

##### OcrImage

An image extracted from an OCR page.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `id` | `string` | — | Unique image identifier. |
| `imageBase64` | `?string` | `null` | Base64-encoded image data. |


---

##### OcrPage

A single page of OCR output.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `index` | `int` | — | Page index (0-based). |
| `markdown` | `string` | — | Extracted content as Markdown. |
| `images` | `?array<OcrImage>` | `null` | Extracted images, if `include_image_base64` was set. |
| `dimensions` | `?PageDimensions` | `null` | Page dimensions in pixels, if available. |


---

##### OcrRequest

An OCR request.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `model` | `string` | — | The model/provider to use (e.g. `"mistral/mistral-ocr-latest"`). |
| `document` | `OcrDocument` | — | The document to process. |
| `pages` | `?array<int>` | `null` | Specific pages to process (1-indexed). `None` means all pages. |
| `includeImageBase64` | `?bool` | `null` | Whether to include base64-encoded images of each page. |


---

##### OcrResponse

An OCR response.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `pages` | `array<OcrPage>` | — | Extracted pages. |
| `model` | `string` | — | The model used. |
| `usage` | `?Usage` | `null` | Token usage, if reported by the provider. |


---

##### PageDimensions

Page dimensions in pixels.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `width` | `int` | — | Width in pixels. |
| `height` | `int` | — | Height in pixels. |


---

##### RerankRequest

Request to rerank documents by relevance to a query.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `model` | `string` | — | Model |
| `query` | `string` | — | Query |
| `documents` | `array<RerankDocument>` | — | Documents |
| `topN` | `?int` | `null` | Top n |
| `returnDocuments` | `?bool` | `null` | Return documents |


---

##### RerankResponse

Response from the rerank endpoint.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `id` | `?string` | `null` | Unique identifier |
| `results` | `array<RerankResult>` | — | Results |
| `meta` | `?mixed` | `null` | Meta |


---

##### RerankResult

A single reranked document with its relevance score.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `index` | `int` | — | Index |
| `relevanceScore` | `float` | — | Relevance score |
| `document` | `?RerankResultDocument` | `null` | Document (rerank result document) |


---

##### RerankResultDocument

The text content of a reranked document, returned when `return_documents` is true.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `text` | `string` | — | Text |


---

##### ResponseClient

Responses API operations (create, retrieve, cancel).

###### Methods

###### createResponse()

Create a new response.

**Signature:**

```php
public function createResponse(CreateResponseRequest $req): ResponseObject
```

###### retrieveResponse()

Retrieve a response by ID.

**Signature:**

```php
public function retrieveResponse(string $id): ResponseObject
```

###### cancelResponse()

Cancel an in-progress response.

**Signature:**

```php
public function cancelResponse(string $id): ResponseObject
```


---

##### SearchRequest

A search request.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `model` | `string` | — | The model/provider to use (e.g. `"brave/web-search"`, `"tavily/search"`). |
| `query` | `string` | — | The search query. |
| `maxResults` | `?int` | `null` | Maximum number of results to return. |
| `searchDomainFilter` | `?array<string>` | `[]` | Domain filter — restrict results to specific domains. |
| `country` | `?string` | `null` | Country code for localized results (ISO 3166-1 alpha-2). |


---

##### SearchResponse

A search response.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `results` | `array<SearchResult>` | — | The search results. |
| `model` | `string` | — | The model used. |


---

##### SearchResult

An individual search result.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `title` | `string` | — | Title of the result. |
| `url` | `string` | — | URL of the result. |
| `snippet` | `string` | — | Text snippet / excerpt. |
| `date` | `?string` | `null` | Publication or last-updated date, if available. |


---

##### SpecificFunction

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `name` | `string` | — | The name |


---

##### SpecificToolChoice

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `choiceType` | `ToolType` | `ToolType::Function` | Choice type (tool type) |
| `function` | `SpecificFunction` | — | Function (specific function) |


---

##### StreamChoice

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `index` | `int` | — | Index |
| `delta` | `StreamDelta` | — | Delta (stream delta) |
| `finishReason` | `?FinishReason` | `null` | Finish reason (finish reason) |


---

##### StreamDelta

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `role` | `?string` | `null` | Role |
| `content` | `?string` | `null` | The extracted text content |
| `toolCalls` | `?array<StreamToolCall>` | `[]` | Tool calls |
| `functionCall` | `?StreamFunctionCall` | `null` | Deprecated legacy function_call delta; retained for API compatibility. |
| `refusal` | `?string` | `null` | Refusal |


---

##### StreamFunctionCall

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `name` | `?string` | `null` | The name |
| `arguments` | `?string` | `null` | Arguments |


---

##### StreamOptions

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `includeUsage` | `?bool` | `null` | Include usage |


---

##### StreamToolCall

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `index` | `int` | — | Index |
| `id` | `?string` | `null` | Unique identifier |
| `callType` | `?ToolType` | `null` | Call type (tool type) |
| `function` | `?StreamFunctionCall` | `null` | Function (stream function call) |


---

##### SystemMessage

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `content` | `string` | — | The extracted text content |
| `name` | `?string` | `null` | The name |


---

##### ToolCall

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `id` | `string` | — | Unique identifier |
| `callType` | `ToolType` | — | Call type (tool type) |
| `function` | `FunctionCall` | — | Function (function call) |


---

##### ToolMessage

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `content` | `string` | — | The extracted text content |
| `toolCallId` | `string` | — | Tool call id |
| `name` | `?string` | `null` | The name |


---

##### TranscriptionResponse

Response from a transcription request.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `text` | `string` | — | Text |
| `language` | `?string` | `null` | Language |
| `duration` | `?float` | `null` | Duration |
| `segments` | `?array<TranscriptionSegment>` | `[]` | Segments |


---

##### TranscriptionSegment

A segment of transcribed audio with timing information.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `id` | `int` | — | Unique identifier |
| `start` | `float` | — | Start |
| `end` | `float` | — | End |
| `text` | `string` | — | Text |


---

##### Usage

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `promptTokens` | `int` | — | Prompt tokens used. Defaults to 0 when absent (some providers omit this). |
| `completionTokens` | `int` | — | Completion tokens used. Defaults to 0 when absent (e.g. embedding responses). |
| `totalTokens` | `int` | — | Total tokens used. Defaults to 0 when absent (some providers omit this). |


---

##### UserMessage

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `content` | `UserContent` | `UserContent::Text` | The extracted text content |
| `name` | `?string` | `null` | The name |


---

#### Enums

##### Message

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

##### UserContent

| Value | Description |
|-------|-------------|
| `Text` | Text format — Fields: `0`: `string` |
| `Parts` | Parts — Fields: `0`: `array<ContentPart>` |


---

##### ContentPart

| Value | Description |
|-------|-------------|
| `Text` | Text format — Fields: `text`: `string` |
| `ImageUrl` | Image url — Fields: `imageUrl`: `ImageUrl` |
| `Document` | Document — Fields: `document`: `DocumentContent` |
| `InputAudio` | Input audio — Fields: `inputAudio`: `AudioContent` |


---

##### ImageDetail

| Value | Description |
|-------|-------------|
| `Low` | Low |
| `High` | High |
| `Auto` | Auto |


---

##### ToolType

The type discriminator for tool/tool-call objects. Per the OpenAI spec this
is always `"function"`. Using an enum enforces that constraint at the type
level and rejects any other value on deserialization.

| Value | Description |
|-------|-------------|
| `Function` | Function |


---

##### ToolChoice

| Value | Description |
|-------|-------------|
| `Mode` | Mode — Fields: `0`: `ToolChoiceMode` |
| `Specific` | Specific — Fields: `0`: `SpecificToolChoice` |


---

##### ToolChoiceMode

| Value | Description |
|-------|-------------|
| `Auto` | Auto |
| `Required` | Required |
| `None` | None |


---

##### ResponseFormat

| Value | Description |
|-------|-------------|
| `Text` | Text format |
| `JsonObject` | Json object |
| `JsonSchema` | Json schema — Fields: `jsonSchema`: `JsonSchemaFormat` |


---

##### StopSequence

| Value | Description |
|-------|-------------|
| `Single` | Single — Fields: `0`: `string` |
| `Multiple` | Multiple — Fields: `0`: `array<string>` |


---

##### FinishReason

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

##### ReasoningEffort

Controls how much reasoning effort the model should use.

| Value | Description |
|-------|-------------|
| `Low` | Low |
| `Medium` | Medium |
| `High` | High |


---

##### EmbeddingFormat

The format in which the embedding vectors are returned.

| Value | Description |
|-------|-------------|
| `Float` | 32-bit floating-point numbers (default). |
| `Base64` | Base64-encoded string representation of the floats. |


---

##### EmbeddingInput

| Value | Description |
|-------|-------------|
| `Single` | Single — Fields: `0`: `string` |
| `Multiple` | Multiple — Fields: `0`: `array<string>` |


---

##### ModerationInput

Input to the moderation endpoint — a single string or multiple strings.

| Value | Description |
|-------|-------------|
| `Single` | Single — Fields: `0`: `string` |
| `Multiple` | Multiple — Fields: `0`: `array<string>` |


---

##### RerankDocument

A document to be reranked — either a plain string or an object with a text field.

| Value | Description |
|-------|-------------|
| `Text` | Text format — Fields: `0`: `string` |
| `Object` | Object — Fields: `text`: `string` |


---

##### OcrDocument

Document input for OCR — either a URL or inline base64 data.

| Value | Description |
|-------|-------------|
| `Url` | A publicly accessible document URL. — Fields: `url`: `string` |
| `Base64` | Inline base64-encoded document data. — Fields: `data`: `string`, `mediaType`: `string` |


---

##### AuthHeaderFormat

How the API key is sent in the HTTP request.

| Value | Description |
|-------|-------------|
| `Bearer` | Bearer token: `Authorization: Bearer <key>` |
| `ApiKey` | Custom header: e.g., `X-Api-Key: <key>` — Fields: `0`: `string` |
| `None` | No authentication required. |


---

#### Errors

##### LiterLlmError

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
