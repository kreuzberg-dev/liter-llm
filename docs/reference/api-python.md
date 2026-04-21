---
title: "Python API Reference"
---

## Python API Reference <span class="version-badge">v1.2.2</span>

### Functions

#### create_client()

Create a new LLM client with simple scalar configuration.

This is the primary binding entry-point. All parameters except `api_key`
are optional — omitting them uses the same defaults as
`ClientConfigBuilder`.

**Errors:**

Returns `LiterLlmError` if the underlying HTTP client cannot be
constructed, or if the resolved provider configuration is invalid.

**Signature:**

```python
def create_client(api_key: str, base_url: str = None, timeout_secs: int = None, max_retries: int = None, model_hint: str = None) -> DefaultClient
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `api_key` | `str` | Yes | The api key |
| `base_url` | `str | None` | No | The base url |
| `timeout_secs` | `int | None` | No | The timeout secs |
| `max_retries` | `int | None` | No | The max retries |
| `model_hint` | `str | None` | No | The model hint |

**Returns:** `DefaultClient`

**Errors:** Raises `Error`.


---

#### create_client_from_json()

Create a new LLM client from a JSON string.

The JSON object accepts the same fields as `liter-llm.toml` (snake_case).

**Errors:**

Returns `LiterLlmError.BadRequest` if `json` is not valid JSON or
contains unknown fields.

**Signature:**

```python
def create_client_from_json(json: str) -> DefaultClient
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `json` | `str` | Yes | The json |

**Returns:** `DefaultClient`

**Errors:** Raises `Error`.


---

#### register_custom_provider()

Register a custom provider in the global runtime registry.

The provider will be checked **before** all built-in providers during model
detection. If a provider with the same `name` already exists it is replaced.

**Errors:**

Returns an error if the config is invalid (empty name, empty base_url, or
no model prefixes).

**Signature:**

```python
def register_custom_provider(config: CustomProviderConfig) -> None
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `config` | `CustomProviderConfig` | Yes | The configuration options |

**Returns:** `None`

**Errors:** Raises `Error`.


---

#### unregister_custom_provider()

Remove a previously registered custom provider by name.

Returns `True` if a provider with the given name was found and removed,
`False` if no such provider existed.

**Errors:**

Returns an error only if the internal lock is poisoned.

**Signature:**

```python
def unregister_custom_provider(name: str) -> bool
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `name` | `str` | Yes | The name |

**Returns:** `bool`

**Errors:** Raises `Error`.


---

### Types

#### ApiError

Inner error object.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `message` | `str` | — | Message |
| `error_type` | `str` | — | Error type |
| `param` | `str | None` | `None` | Param |
| `code` | `str | None` | `None` | Code |


---

#### AssistantMessage

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `content` | `str | None` | `None` | The extracted text content |
| `name` | `str | None` | `None` | The name |
| `tool_calls` | `list[ToolCall] | None` | `[]` | Tool calls |
| `refusal` | `str | None` | `None` | Refusal |
| `function_call` | `FunctionCall | None` | `None` | Deprecated legacy function_call field; retained for API compatibility. |


---

#### AudioContent

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `data` | `str` | — | Base64-encoded audio data. |
| `format` | `str` | — | Audio format (e.g., "wav", "mp3", "ogg"). |


---

#### BatchClient

Batch processing operations (create, list, retrieve, cancel).

##### Methods

###### create_batch()

Create a new batch job.

**Signature:**

```python
def create_batch(self, req: CreateBatchRequest) -> BatchObject
```

###### retrieve_batch()

Retrieve a batch by ID.

**Signature:**

```python
def retrieve_batch(self, batch_id: str) -> BatchObject
```

###### list_batches()

List batches, optionally filtered by query parameters.

**Signature:**

```python
def list_batches(self, query: BatchListQuery) -> BatchListResponse
```

###### cancel_batch()

Cancel an in-progress batch.

**Signature:**

```python
def cancel_batch(self, batch_id: str) -> BatchObject
```


---

#### ChatCompletionChunk

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `id` | `str` | — | Unique identifier |
| `object` | `str` | — | Always `"chat.completion.chunk"` from OpenAI-compatible APIs.  Stored as a plain `String` so non-standard provider values do not fail parsing. |
| `created` | `int` | — | Created |
| `model` | `str` | — | Model |
| `choices` | `list[StreamChoice]` | `[]` | Choices |
| `usage` | `Usage | None` | `None` | Usage (usage) |
| `system_fingerprint` | `str | None` | `None` | System fingerprint |
| `service_tier` | `str | None` | `None` | Service tier |


---

#### ChatCompletionRequest

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `model` | `str` | — | Model |
| `messages` | `list[Message]` | `[]` | Messages |
| `temperature` | `float | None` | `None` | Temperature |
| `top_p` | `float | None` | `None` | Top p |
| `n` | `int | None` | `None` | N |
| `stream` | `bool | None` | `None` | Whether to stream the response. Managed by the client layer — do not set directly. |
| `stop` | `StopSequence | None` | `None` | Stop (stop sequence) |
| `max_tokens` | `int | None` | `None` | Maximum tokens |
| `presence_penalty` | `float | None` | `None` | Presence penalty |
| `frequency_penalty` | `float | None` | `None` | Frequency penalty |
| `logit_bias` | `dict[str, float] | None` | `{}` | Token bias map.  Uses `BTreeMap` (sorted keys) for deterministic serialization order — important when hashing or signing requests. |
| `user` | `str | None` | `None` | User |
| `tools` | `list[ChatCompletionTool] | None` | `[]` | Tools |
| `tool_choice` | `ToolChoice | None` | `None` | Tool choice (tool choice) |
| `parallel_tool_calls` | `bool | None` | `None` | Parallel tool calls |
| `response_format` | `ResponseFormat | None` | `None` | Response format (response format) |
| `stream_options` | `StreamOptions | None` | `None` | Stream options (stream options) |
| `seed` | `int | None` | `None` | Seed |
| `reasoning_effort` | `ReasoningEffort | None` | `None` | Reasoning effort (reasoning effort) |
| `extra_body` | `Any | None` | `None` | Provider-specific extra parameters merged into the request body. Use for guardrails, safety settings, grounding config, etc. |


---

#### ChatCompletionResponse

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `id` | `str` | — | Unique identifier |
| `object` | `str` | — | Always `"chat.completion"` from OpenAI-compatible APIs.  Stored as a plain `String` so non-standard provider values do not break deserialization. |
| `created` | `int` | — | Created |
| `model` | `str` | — | Model |
| `choices` | `list[Choice]` | `[]` | Choices |
| `usage` | `Usage | None` | `None` | Usage (usage) |
| `system_fingerprint` | `str | None` | `None` | System fingerprint |
| `service_tier` | `str | None` | `None` | Service tier |

##### Methods

###### estimated_cost()

Estimate the cost of this response based on embedded pricing data.

Returns `None` if:

- the `model` field is not present in the embedded pricing registry, or
- the `usage` field is absent from the response.

**Signature:**

```python
def estimated_cost(self) -> float | None
```


---

#### ChatCompletionTool

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `tool_type` | `ToolType` | — | Tool type (tool type) |
| `function` | `FunctionDefinition` | — | Function (function definition) |


---

#### Choice

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `index` | `int` | — | Index |
| `message` | `AssistantMessage` | — | Message (assistant message) |
| `finish_reason` | `FinishReason | None` | `None` | Finish reason (finish reason) |


---

#### ClientConfig

Configuration for an LLM client.

`api_key` is stored as a `SecretString` so it is zeroed on drop and never
printed accidentally. Access it via `secrecy.ExposeSecret`.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `api_key` | `str` | — | API key for authentication (stored as a secret). |
| `base_url` | `str | None` | `None` | Override base URL.  When set, all requests go here regardless of model name, and provider auto-detection is skipped. |
| `timeout` | `float` | — | Request timeout. |
| `max_retries` | `int` | — | Maximum number of retries on 429 / 5xx responses. |
| `credential_provider` | `CredentialProvider | None` | `None` | Optional dynamic credential provider for token-based auth (Azure AD, Vertex OAuth2) or refreshable credentials (AWS STS). When set, the client calls `resolve()` before each request to obtain a fresh credential.  When `None`, the static `api_key` is used. |

##### Methods

###### headers()

Return the extra headers as an ordered slice of `(name, value)` pairs.

**Signature:**

```python
def headers(self) -> list[tuple[str, str]]
```

###### fmt()

**Signature:**

```python
def fmt(self, f: Formatter) -> Unknown
```


---

#### ClientConfigBuilder

Builder for `ClientConfig`.

Construct with `ClientConfigBuilder.new` and call builder methods to
customise the configuration, then call `ClientConfigBuilder.build` to
obtain a `ClientConfig`.

##### Methods

###### base_url()

Override the provider base URL for all requests.

**Signature:**

```python
def base_url(self, url: str) -> ClientConfigBuilder
```

###### timeout()

Set the per-request timeout (default: 60 s).

**Signature:**

```python
def timeout(self, timeout: float) -> ClientConfigBuilder
```

###### max_retries()

Set the maximum number of retries on 429 / 5xx responses (default: 3).

**Signature:**

```python
def max_retries(self, retries: int) -> ClientConfigBuilder
```

###### credential_provider()

Set a dynamic credential provider for token-based or refreshable auth.

When configured, the client calls `resolve()` before each request
instead of using the static `api_key` for authentication.

**Signature:**

```python
def credential_provider(self, provider: CredentialProvider) -> ClientConfigBuilder
```

###### header()

Add a custom header sent on every request.

Returns an error if either `key` or `value` is not a valid HTTP header
name / value.

This method is only available when the `native-http` feature is enabled
because header validation relies on `reqwest`'s header types.

**Signature:**

```python
def header(self, key: str, value: str) -> ClientConfigBuilder
```

###### cache()

Set the response cache configuration for the Tower middleware stack.

When set, bindings and advanced Rust users can read this from the
built `ClientConfig` to construct a
`CacheLayer`.

**Signature:**

```python
def cache(self, config: CacheConfig) -> ClientConfigBuilder
```

###### cache_store()

Set a custom cache store backend for the Tower cache middleware.

When set alongside `cache`, the cache layer will use
this store instead of the default in-memory LRU.

**Signature:**

```python
def cache_store(self, store: CacheStore) -> ClientConfigBuilder
```

###### budget()

Set the budget enforcement configuration for the Tower middleware stack.

When set, bindings and advanced Rust users can read this from the
built `ClientConfig` to construct a
`BudgetLayer`.

**Signature:**

```python
def budget(self, config: BudgetConfig) -> ClientConfigBuilder
```

###### hook()

Add a single hook to the Tower hooks middleware stack.

Hooks are invoked sequentially in registration order at request
lifecycle points (pre-request, post-response, on-error).

**Signature:**

```python
def hook(self, hook: LlmHook) -> ClientConfigBuilder
```

###### hooks()

Set the full list of hooks for the Tower hooks middleware stack,
replacing any previously registered hooks.

Hooks are invoked sequentially in registration order.

**Signature:**

```python
def hooks(self, hooks: list[LlmHook]) -> ClientConfigBuilder
```

###### cooldown()

Set the cooldown duration after transient errors.

When set, the client rejects requests with `ServiceUnavailable` for
the given duration after a transient error (rate limit, timeout,
server error).

**Signature:**

```python
def cooldown(self, duration: float) -> ClientConfigBuilder
```

###### rate_limit()

Set per-model rate limiting configuration.

When set, requests exceeding the configured RPM or TPM limits are
rejected with `LiterLlmError.RateLimited`.

**Signature:**

```python
def rate_limit(self, config: RateLimitConfig) -> ClientConfigBuilder
```

###### health_check()

Set the background health check interval.

When set, the client periodically probes the provider and rejects
requests when the provider is unhealthy.

**Signature:**

```python
def health_check(self, interval: float) -> ClientConfigBuilder
```

###### cost_tracking()

Enable or disable per-request cost tracking.

When enabled, estimated USD cost is recorded on the current tracing
span as `gen_ai.usage.cost`.

**Signature:**

```python
def cost_tracking(self, enabled: bool) -> ClientConfigBuilder
```

###### tracing()

Enable or disable OpenTelemetry-compatible tracing spans.

When enabled, every request is wrapped in a `gen_ai` tracing span
with semantic convention attributes.

**Signature:**

```python
def tracing(self, enabled: bool) -> ClientConfigBuilder
```

###### build()

Consume the builder and return the completed `ClientConfig`.

**Signature:**

```python
def build(self) -> ClientConfig
```


---

#### CreateImageRequest

Request to create images from a text prompt.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `prompt` | `str` | — | Prompt |
| `model` | `str | None` | `None` | Model |
| `n` | `int | None` | `None` | N |
| `size` | `str | None` | `None` | Size in bytes |
| `quality` | `str | None` | `None` | Quality |
| `style` | `str | None` | `None` | Style |
| `response_format` | `str | None` | `None` | Response format |
| `user` | `str | None` | `None` | User |


---

#### CreateSpeechRequest

Request to generate speech audio from text.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `model` | `str` | — | Model |
| `input` | `str` | — | Input |
| `voice` | `str` | — | Voice |
| `response_format` | `str | None` | `None` | Response format |
| `speed` | `float | None` | `None` | Speed |


---

#### CreateTranscriptionRequest

Request to transcribe audio into text.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `model` | `str` | — | Model |
| `file` | `str` | — | Base64-encoded audio file data. |
| `language` | `str | None` | `None` | Language |
| `prompt` | `str | None` | `None` | Prompt |
| `response_format` | `str | None` | `None` | Response format |
| `temperature` | `float | None` | `None` | Temperature |


---

#### CustomProviderConfig

Configuration for registering a custom LLM provider at runtime.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `name` | `str` | — | Unique name for this provider (e.g., "my-provider"). |
| `base_url` | `str` | — | Base URL for the provider's API (e.g., "<https://api.my-provider.com/v1">). |
| `auth_header` | `AuthHeaderFormat` | — | Authentication header format. |
| `model_prefixes` | `list[str]` | — | Model name prefixes that route to this provider (e.g., ["my-"]). |


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
`Some("groq/llama3-70b")` selects the Groq provider. Pass `None` to
default to OpenAI.

**Errors:**

Returns a wrapped `reqwest.Error` if the underlying HTTP client
cannot be constructed. Header names and values are pre-validated by
`ClientConfigBuilder.header`, so they are inserted directly here.

**Signature:**

```python
@staticmethod
def new(config: ClientConfig, model_hint: str) -> DefaultClient
```

###### chat()

**Signature:**

```python
def chat(self, req: ChatCompletionRequest) -> ChatCompletionResponse
```

###### chat_stream()

**Signature:**

```python
def chat_stream(self, req: ChatCompletionRequest) -> BoxStream
```

###### embed()

**Signature:**

```python
def embed(self, req: EmbeddingRequest) -> EmbeddingResponse
```

###### list_models()

**Signature:**

```python
def list_models(self) -> ModelsListResponse
```

###### image_generate()

**Signature:**

```python
def image_generate(self, req: CreateImageRequest) -> ImagesResponse
```

###### speech()

**Signature:**

```python
def speech(self, req: CreateSpeechRequest) -> bytes
```

###### transcribe()

**Signature:**

```python
def transcribe(self, req: CreateTranscriptionRequest) -> TranscriptionResponse
```

###### moderate()

**Signature:**

```python
def moderate(self, req: ModerationRequest) -> ModerationResponse
```

###### rerank()

**Signature:**

```python
def rerank(self, req: RerankRequest) -> RerankResponse
```

###### search()

**Signature:**

```python
def search(self, req: SearchRequest) -> SearchResponse
```

###### ocr()

**Signature:**

```python
def ocr(self, req: OcrRequest) -> OcrResponse
```

###### chat_raw()

**Signature:**

```python
def chat_raw(self, req: ChatCompletionRequest) -> RawExchange
```

###### chat_stream_raw()

**Signature:**

```python
def chat_stream_raw(self, req: ChatCompletionRequest) -> RawStreamExchange
```

###### embed_raw()

**Signature:**

```python
def embed_raw(self, req: EmbeddingRequest) -> RawExchange
```

###### image_generate_raw()

**Signature:**

```python
def image_generate_raw(self, req: CreateImageRequest) -> RawExchange
```

###### transcribe_raw()

**Signature:**

```python
def transcribe_raw(self, req: CreateTranscriptionRequest) -> RawExchange
```

###### moderate_raw()

**Signature:**

```python
def moderate_raw(self, req: ModerationRequest) -> RawExchange
```

###### rerank_raw()

**Signature:**

```python
def rerank_raw(self, req: RerankRequest) -> RawExchange
```

###### search_raw()

**Signature:**

```python
def search_raw(self, req: SearchRequest) -> RawExchange
```

###### ocr_raw()

**Signature:**

```python
def ocr_raw(self, req: OcrRequest) -> RawExchange
```

###### create_file()

**Signature:**

```python
def create_file(self, req: CreateFileRequest) -> FileObject
```

###### retrieve_file()

**Signature:**

```python
def retrieve_file(self, file_id: str) -> FileObject
```

###### delete_file()

**Signature:**

```python
def delete_file(self, file_id: str) -> DeleteResponse
```

###### list_files()

**Signature:**

```python
def list_files(self, query: FileListQuery) -> FileListResponse
```

###### file_content()

**Signature:**

```python
def file_content(self, file_id: str) -> bytes
```

###### create_batch()

**Signature:**

```python
def create_batch(self, req: CreateBatchRequest) -> BatchObject
```

###### retrieve_batch()

**Signature:**

```python
def retrieve_batch(self, batch_id: str) -> BatchObject
```

###### list_batches()

**Signature:**

```python
def list_batches(self, query: BatchListQuery) -> BatchListResponse
```

###### cancel_batch()

**Signature:**

```python
def cancel_batch(self, batch_id: str) -> BatchObject
```

###### create_response()

**Signature:**

```python
def create_response(self, req: CreateResponseRequest) -> ResponseObject
```

###### retrieve_response()

**Signature:**

```python
def retrieve_response(self, id: str) -> ResponseObject
```

###### cancel_response()

**Signature:**

```python
def cancel_response(self, id: str) -> ResponseObject
```


---

#### DeveloperMessage

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `content` | `str` | — | The extracted text content |
| `name` | `str | None` | `None` | The name |


---

#### DocumentContent

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `data` | `str` | — | Base64-encoded document data or URL. |
| `media_type` | `str` | — | MIME type (e.g., "application/pdf", "text/csv"). |


---

#### EmbeddingObject

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `object` | `str` | — | Always `"embedding"` from OpenAI-compatible APIs.  Stored as a plain `String` so non-standard provider values do not break deserialization. |
| `embedding` | `list[float]` | — | Embedding |
| `index` | `int` | — | Index |


---

#### EmbeddingRequest

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `model` | `str` | — | Model |
| `input` | `EmbeddingInput` | — | Input (embedding input) |
| `encoding_format` | `EmbeddingFormat | None` | `None` | Encoding format (embedding format) |
| `dimensions` | `int | None` | `None` | Dimensions |
| `user` | `str | None` | `None` | User |


---

#### EmbeddingResponse

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `object` | `str` | — | Always `"list"` from OpenAI-compatible APIs.  Stored as a plain `String` so non-standard provider values do not break deserialization. |
| `data` | `list[EmbeddingObject]` | — | Data |
| `model` | `str` | — | Model |
| `usage` | `Usage | None` | `None` | Usage (usage) |

##### Methods

###### estimated_cost()

Estimate the cost of this embedding request based on embedded pricing data.

Returns `None` if:

- the `model` field is not present in the embedded pricing registry, or
- the `usage` field is absent from the response.

Embedding models only charge for input tokens; output cost is zero.

**Signature:**

```python
def estimated_cost(self) -> float | None
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
| `global_limit` | `float | None` | `None` | Global limit |
| `model_limits` | `dict[str, float] | None` | `None` | Model limits |
| `enforcement` | `str | None` | `None` | Enforcement |


---

#### FileCacheConfig

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `max_entries` | `int | None` | `None` | Maximum entries |
| `ttl_seconds` | `int | None` | `None` | Ttl seconds |
| `backend` | `str | None` | `None` | Backend |
| `backend_config` | `dict[str, str] | None` | `None` | Backend config |


---

#### FileClient

File management operations (upload, list, retrieve, delete).

##### Methods

###### create_file()

Upload a file.

**Signature:**

```python
def create_file(self, req: CreateFileRequest) -> FileObject
```

###### retrieve_file()

Retrieve metadata for a file.

**Signature:**

```python
def retrieve_file(self, file_id: str) -> FileObject
```

###### delete_file()

Delete a file.

**Signature:**

```python
def delete_file(self, file_id: str) -> DeleteResponse
```

###### list_files()

List files, optionally filtered by query parameters.

**Signature:**

```python
def list_files(self, query: FileListQuery) -> FileListResponse
```

###### file_content()

Retrieve the raw content of a file.

**Signature:**

```python
def file_content(self, file_id: str) -> bytes
```


---

#### FileConfig

TOML file representation of client configuration.

All fields are optional — missing fields use defaults from `ClientConfigBuilder`.
Convert to a builder via `FileConfig.into_builder`.

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
| `api_key` | `str | None` | `None` | Api key |
| `base_url` | `str | None` | `None` | Base url |
| `model_hint` | `str | None` | `None` | Model hint |
| `timeout_secs` | `int | None` | `None` | Timeout secs |
| `max_retries` | `int | None` | `None` | Maximum retries |
| `extra_headers` | `dict[str, str] | None` | `None` | Extra headers |
| `cache` | `FileCacheConfig | None` | `None` | Cache (file cache config) |
| `budget` | `FileBudgetConfig | None` | `None` | Budget (file budget config) |
| `cooldown_secs` | `int | None` | `None` | Cooldown secs |
| `rate_limit` | `FileRateLimitConfig | None` | `None` | Rate limit (file rate limit config) |
| `health_check_secs` | `int | None` | `None` | Health check secs |
| `cost_tracking` | `bool | None` | `None` | Cost tracking |
| `tracing` | `bool | None` | `None` | Tracing |
| `providers` | `list[FileProviderConfig] | None` | `None` | Providers |

### Methods

#### from_toml_file()

Load from a TOML file path.

**Signature:**

```python
@staticmethod
def from_toml_file(path: Path) -> FileConfig
```

##### from_toml_str()

Parse from a TOML string.

**Signature:**

```python
@staticmethod
def from_toml_str(s: str) -> FileConfig
```

###### discover()

Discover `liter-llm.toml` by walking from current directory to filesystem root.

Returns `Ok(None)` if no config file is found.

**Signature:**

```python
@staticmethod
def discover() -> FileConfig | None
```

###### into_builder()

Convert into a `ClientConfigBuilder`,
applying all fields that are set.

Fields not present in the TOML file use the builder's defaults.

**Signature:**

```python
def into_builder(self) -> ClientConfigBuilder
```

###### providers()

Get the custom provider configurations from this file config.

**Signature:**

```python
def providers(self) -> list[FileProviderConfig]
```


---

##### FileProviderConfig

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `name` | `str` | — | The name |
| `base_url` | `str` | — | Base url |
| `auth_header` | `str | None` | `None` | Auth header |
| `model_prefixes` | `list[str]` | — | Model prefixes |


---

##### FileRateLimitConfig

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `rpm` | `int | None` | `None` | Rpm |
| `tpm` | `int | None` | `None` | Tpm |
| `window_seconds` | `int | None` | `None` | Window seconds |


---

##### FunctionCall

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `name` | `str` | — | The name |
| `arguments` | `str` | — | Arguments |


---

##### FunctionDefinition

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `name` | `str` | — | The name |
| `description` | `str | None` | `None` | Human-readable description |
| `parameters` | `Any | None` | `None` | Parameters |
| `strict` | `bool | None` | `None` | Strict |


---

##### FunctionMessage

Deprecated legacy function-role message body.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `content` | `str` | — | The extracted text content |
| `name` | `str` | — | The name |


---

##### Image

A single generated image, returned as either a URL or base64 data.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `url` | `str | None` | `None` | Url |
| `b64_json` | `str | None` | `None` | B64 json |
| `revised_prompt` | `str | None` | `None` | Revised prompt |


---

##### ImageUrl

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `url` | `str` | — | Url |
| `detail` | `ImageDetail | None` | `None` | Detail (image detail) |


---

##### ImagesResponse

Response containing generated images.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `created` | `int` | — | Created |
| `data` | `list[Image]` | `[]` | Data |


---

##### JsonSchemaFormat

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `name` | `str` | — | The name |
| `description` | `str | None` | `None` | Human-readable description |
| `schema` | `Any` | — | Schema |
| `strict` | `bool | None` | `None` | Strict |


---

##### LiterLlmError

###### Methods

###### is_transient()

Returns `True` for errors that are worth retrying on a different service
or deployment (transient failures).

Used by `crate.tower.fallback.FallbackService` and
`crate.tower.router.Router` to decide whether to route to an
alternative endpoint.

**Signature:**

```python
def is_transient(self) -> bool
```

###### error_type()

Return the OpenTelemetry `error.type` string for this error variant.

Used by the tracing middleware to record the `error.type` span attribute
on failed requests per the GenAI semantic conventions.

**Signature:**

```python
def error_type(self) -> str
```

###### from_status()

Create from an HTTP status code, an API error response body, and an
optional `Retry-After` duration already parsed from the response header.

The `retry_after` value is forwarded into `LiterLlmError.RateLimited`
so callers can honour the server-requested delay without re-parsing the
header.

**Signature:**

```python
@staticmethod
def from_status(status: int, body: str, retry_after: float) -> LiterLlmError
```


---

##### LlmClient

Core LLM client trait.

###### Methods

###### chat()

Send a chat completion request.

**Signature:**

```python
def chat(self, req: ChatCompletionRequest) -> ChatCompletionResponse
```

###### chat_stream()

Send a streaming chat completion request.

**Signature:**

```python
def chat_stream(self, req: ChatCompletionRequest) -> BoxStream
```

###### embed()

Send an embedding request.

**Signature:**

```python
def embed(self, req: EmbeddingRequest) -> EmbeddingResponse
```

###### list_models()

List available models.

**Signature:**

```python
def list_models(self) -> ModelsListResponse
```

###### image_generate()

Generate an image.

**Signature:**

```python
def image_generate(self, req: CreateImageRequest) -> ImagesResponse
```

###### speech()

Generate speech audio from text.

**Signature:**

```python
def speech(self, req: CreateSpeechRequest) -> bytes
```

###### transcribe()

Transcribe audio to text.

**Signature:**

```python
def transcribe(self, req: CreateTranscriptionRequest) -> TranscriptionResponse
```

###### moderate()

Check content against moderation policies.

**Signature:**

```python
def moderate(self, req: ModerationRequest) -> ModerationResponse
```

###### rerank()

Rerank documents by relevance to a query.

**Signature:**

```python
def rerank(self, req: RerankRequest) -> RerankResponse
```

###### search()

Perform a web/document search.

**Signature:**

```python
def search(self, req: SearchRequest) -> SearchResponse
```

###### ocr()

Extract text from a document via OCR.

**Signature:**

```python
def ocr(self, req: OcrRequest) -> OcrResponse
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

###### chat_raw()

Send a chat completion request and return the raw exchange.

The `raw_request` field contains the final JSON body sent to the
provider; `raw_response` contains the provider JSON before
normalization.

**Signature:**

```python
def chat_raw(self, req: ChatCompletionRequest) -> RawExchange
```

###### chat_stream_raw()

Send a streaming chat completion request and return the raw exchange.

Only `raw_request` is available upfront — the stream itself is
returned in `stream` and consumed incrementally.

**Signature:**

```python
def chat_stream_raw(self, req: ChatCompletionRequest) -> RawStreamExchange
```

###### embed_raw()

Send an embedding request and return the raw exchange.

**Signature:**

```python
def embed_raw(self, req: EmbeddingRequest) -> RawExchange
```

###### image_generate_raw()

Generate an image and return the raw exchange.

**Signature:**

```python
def image_generate_raw(self, req: CreateImageRequest) -> RawExchange
```

###### transcribe_raw()

Transcribe audio to text and return the raw exchange.

**Signature:**

```python
def transcribe_raw(self, req: CreateTranscriptionRequest) -> RawExchange
```

###### moderate_raw()

Check content against moderation policies and return the raw exchange.

**Signature:**

```python
def moderate_raw(self, req: ModerationRequest) -> RawExchange
```

###### rerank_raw()

Rerank documents by relevance to a query and return the raw exchange.

**Signature:**

```python
def rerank_raw(self, req: RerankRequest) -> RawExchange
```

###### search_raw()

Perform a web/document search and return the raw exchange.

**Signature:**

```python
def search_raw(self, req: SearchRequest) -> RawExchange
```

###### ocr_raw()

Extract text from a document via OCR and return the raw exchange.

**Signature:**

```python
def ocr_raw(self, req: OcrRequest) -> RawExchange
```


---

##### ManagedClient

A managed LLM client that wraps `DefaultClient` with optional Tower
middleware (cache, cooldown, rate limiting, health checks, cost tracking,
budget, hooks, tracing).

Construct via `ManagedClient.new`. If the provided `ClientConfig`
contains any middleware configuration the corresponding Tower layers are
composed into a service stack. Otherwise requests pass straight through
to the inner `DefaultClient`.

`ManagedClient` implements `LlmClient` and can be used everywhere a
`DefaultClient` is expected.

###### Methods

###### new()

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

```python
@staticmethod
def new(config: ClientConfig, model_hint: str) -> ManagedClient
```

###### inner()

Return a reference to the underlying `DefaultClient`.

**Signature:**

```python
def inner(self) -> DefaultClient
```

###### budget_state()

Return the budget state handle, if budget middleware is configured.

Use this to query accumulated spend at runtime.

**Signature:**

```python
def budget_state(self) -> BudgetState | None
```

###### has_middleware()

Return `True` when middleware is active (requests go through the Tower
service stack).

**Signature:**

```python
def has_middleware(self) -> bool
```

###### chat()

**Signature:**

```python
def chat(self, req: ChatCompletionRequest) -> ChatCompletionResponse
```

###### chat_stream()

**Signature:**

```python
def chat_stream(self, req: ChatCompletionRequest) -> BoxStream
```

###### embed()

**Signature:**

```python
def embed(self, req: EmbeddingRequest) -> EmbeddingResponse
```

###### list_models()

**Signature:**

```python
def list_models(self) -> ModelsListResponse
```

###### image_generate()

**Signature:**

```python
def image_generate(self, req: CreateImageRequest) -> ImagesResponse
```

###### speech()

**Signature:**

```python
def speech(self, req: CreateSpeechRequest) -> bytes
```

###### transcribe()

**Signature:**

```python
def transcribe(self, req: CreateTranscriptionRequest) -> TranscriptionResponse
```

###### moderate()

**Signature:**

```python
def moderate(self, req: ModerationRequest) -> ModerationResponse
```

###### rerank()

**Signature:**

```python
def rerank(self, req: RerankRequest) -> RerankResponse
```

###### search()

**Signature:**

```python
def search(self, req: SearchRequest) -> SearchResponse
```

###### ocr()

**Signature:**

```python
def ocr(self, req: OcrRequest) -> OcrResponse
```

###### create_file()

**Signature:**

```python
def create_file(self, req: CreateFileRequest) -> FileObject
```

###### retrieve_file()

**Signature:**

```python
def retrieve_file(self, file_id: str) -> FileObject
```

###### delete_file()

**Signature:**

```python
def delete_file(self, file_id: str) -> DeleteResponse
```

###### list_files()

**Signature:**

```python
def list_files(self, query: FileListQuery) -> FileListResponse
```

###### file_content()

**Signature:**

```python
def file_content(self, file_id: str) -> bytes
```

###### create_batch()

**Signature:**

```python
def create_batch(self, req: CreateBatchRequest) -> BatchObject
```

###### retrieve_batch()

**Signature:**

```python
def retrieve_batch(self, batch_id: str) -> BatchObject
```

###### list_batches()

**Signature:**

```python
def list_batches(self, query: BatchListQuery) -> BatchListResponse
```

###### cancel_batch()

**Signature:**

```python
def cancel_batch(self, batch_id: str) -> BatchObject
```

###### create_response()

**Signature:**

```python
def create_response(self, req: CreateResponseRequest) -> ResponseObject
```

###### retrieve_response()

**Signature:**

```python
def retrieve_response(self, id: str) -> ResponseObject
```

###### cancel_response()

**Signature:**

```python
def cancel_response(self, id: str) -> ResponseObject
```


---

##### ModelObject

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `id` | `str` | — | Unique identifier |
| `object` | `str` | — | Always `"model"` from OpenAI-compatible APIs.  Stored as a plain `String` so non-standard provider values do not break deserialization. |
| `created` | `int` | — | Created |
| `owned_by` | `str` | — | Owned by |


---

##### ModelsListResponse

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `object` | `str` | — | Always `"list"` from OpenAI-compatible APIs.  Stored as a plain `String` so non-standard provider values do not break deserialization. |
| `data` | `list[ModelObject]` | `[]` | Data |


---

##### ModerationCategories

Boolean flags for each moderation category.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `sexual` | `bool` | — | Sexual |
| `hate` | `bool` | — | Hate |
| `harassment` | `bool` | — | Harassment |
| `self_harm` | `bool` | — | Self harm |
| `sexual_minors` | `bool` | — | Sexual minors |
| `hate_threatening` | `bool` | — | Hate threatening |
| `violence_graphic` | `bool` | — | Violence graphic |
| `self_harm_intent` | `bool` | — | Self harm intent |
| `self_harm_instructions` | `bool` | — | Self harm instructions |
| `harassment_threatening` | `bool` | — | Harassment threatening |
| `violence` | `bool` | — | Violence |


---

##### ModerationCategoryScores

Confidence scores for each moderation category.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `sexual` | `float` | — | Sexual |
| `hate` | `float` | — | Hate |
| `harassment` | `float` | — | Harassment |
| `self_harm` | `float` | — | Self harm |
| `sexual_minors` | `float` | — | Sexual minors |
| `hate_threatening` | `float` | — | Hate threatening |
| `violence_graphic` | `float` | — | Violence graphic |
| `self_harm_intent` | `float` | — | Self harm intent |
| `self_harm_instructions` | `float` | — | Self harm instructions |
| `harassment_threatening` | `float` | — | Harassment threatening |
| `violence` | `float` | — | Violence |


---

##### ModerationRequest

Request to classify content for policy violations.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `input` | `ModerationInput` | — | Input (moderation input) |
| `model` | `str | None` | `None` | Model |


---

##### ModerationResponse

Response from the moderation endpoint.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `id` | `str` | — | Unique identifier |
| `model` | `str` | — | Model |
| `results` | `list[ModerationResult]` | — | Results |


---

##### ModerationResult

A single moderation classification result.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `flagged` | `bool` | — | Flagged |
| `categories` | `ModerationCategories` | — | Categories (moderation categories) |
| `category_scores` | `ModerationCategoryScores` | — | Category scores (moderation category scores) |


---

##### OcrImage

An image extracted from an OCR page.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `id` | `str` | — | Unique image identifier. |
| `image_base64` | `str | None` | `None` | Base64-encoded image data. |


---

##### OcrPage

A single page of OCR output.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `index` | `int` | — | Page index (0-based). |
| `markdown` | `str` | — | Extracted content as Markdown. |
| `images` | `list[OcrImage] | None` | `None` | Extracted images, if `include_image_base64` was set. |
| `dimensions` | `PageDimensions | None` | `None` | Page dimensions in pixels, if available. |


---

##### OcrRequest

An OCR request.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `model` | `str` | — | The model/provider to use (e.g. `"mistral/mistral-ocr-latest"`). |
| `document` | `OcrDocument` | — | The document to process. |
| `pages` | `list[int] | None` | `None` | Specific pages to process (1-indexed). `None` means all pages. |
| `include_image_base64` | `bool | None` | `None` | Whether to include base64-encoded images of each page. |


---

##### OcrResponse

An OCR response.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `pages` | `list[OcrPage]` | — | Extracted pages. |
| `model` | `str` | — | The model used. |
| `usage` | `Usage | None` | `None` | Token usage, if reported by the provider. |


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
| `model` | `str` | — | Model |
| `query` | `str` | — | Query |
| `documents` | `list[RerankDocument]` | — | Documents |
| `top_n` | `int | None` | `None` | Top n |
| `return_documents` | `bool | None` | `None` | Return documents |


---

##### RerankResponse

Response from the rerank endpoint.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `id` | `str | None` | `None` | Unique identifier |
| `results` | `list[RerankResult]` | — | Results |
| `meta` | `Any | None` | `None` | Meta |


---

##### RerankResult

A single reranked document with its relevance score.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `index` | `int` | — | Index |
| `relevance_score` | `float` | — | Relevance score |
| `document` | `RerankResultDocument | None` | `None` | Document (rerank result document) |


---

##### RerankResultDocument

The text content of a reranked document, returned when `return_documents` is true.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `text` | `str` | — | Text |


---

##### ResponseClient

Responses API operations (create, retrieve, cancel).

###### Methods

###### create_response()

Create a new response.

**Signature:**

```python
def create_response(self, req: CreateResponseRequest) -> ResponseObject
```

###### retrieve_response()

Retrieve a response by ID.

**Signature:**

```python
def retrieve_response(self, id: str) -> ResponseObject
```

###### cancel_response()

Cancel an in-progress response.

**Signature:**

```python
def cancel_response(self, id: str) -> ResponseObject
```


---

##### SearchRequest

A search request.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `model` | `str` | — | The model/provider to use (e.g. `"brave/web-search"`, `"tavily/search"`). |
| `query` | `str` | — | The search query. |
| `max_results` | `int | None` | `None` | Maximum number of results to return. |
| `search_domain_filter` | `list[str] | None` | `[]` | Domain filter — restrict results to specific domains. |
| `country` | `str | None` | `None` | Country code for localized results (ISO 3166-1 alpha-2). |


---

##### SearchResponse

A search response.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `results` | `list[SearchResult]` | — | The search results. |
| `model` | `str` | — | The model used. |


---

##### SearchResult

An individual search result.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `title` | `str` | — | Title of the result. |
| `url` | `str` | — | URL of the result. |
| `snippet` | `str` | — | Text snippet / excerpt. |
| `date` | `str | None` | `None` | Publication or last-updated date, if available. |


---

##### SpecificFunction

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `name` | `str` | — | The name |


---

##### SpecificToolChoice

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `choice_type` | `ToolType` | `ToolType.FUNCTION` | Choice type (tool type) |
| `function` | `SpecificFunction` | — | Function (specific function) |


---

##### StreamChoice

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `index` | `int` | — | Index |
| `delta` | `StreamDelta` | — | Delta (stream delta) |
| `finish_reason` | `FinishReason | None` | `None` | Finish reason (finish reason) |


---

##### StreamDelta

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `role` | `str | None` | `None` | Role |
| `content` | `str | None` | `None` | The extracted text content |
| `tool_calls` | `list[StreamToolCall] | None` | `[]` | Tool calls |
| `function_call` | `StreamFunctionCall | None` | `None` | Deprecated legacy function_call delta; retained for API compatibility. |
| `refusal` | `str | None` | `None` | Refusal |


---

##### StreamFunctionCall

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `name` | `str | None` | `None` | The name |
| `arguments` | `str | None` | `None` | Arguments |


---

##### StreamOptions

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `include_usage` | `bool | None` | `None` | Include usage |


---

##### StreamToolCall

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `index` | `int` | — | Index |
| `id` | `str | None` | `None` | Unique identifier |
| `call_type` | `ToolType | None` | `None` | Call type (tool type) |
| `function` | `StreamFunctionCall | None` | `None` | Function (stream function call) |


---

##### SystemMessage

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `content` | `str` | — | The extracted text content |
| `name` | `str | None` | `None` | The name |


---

##### ToolCall

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `id` | `str` | — | Unique identifier |
| `call_type` | `ToolType` | — | Call type (tool type) |
| `function` | `FunctionCall` | — | Function (function call) |


---

##### ToolMessage

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `content` | `str` | — | The extracted text content |
| `tool_call_id` | `str` | — | Tool call id |
| `name` | `str | None` | `None` | The name |


---

##### TranscriptionResponse

Response from a transcription request.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `text` | `str` | — | Text |
| `language` | `str | None` | `None` | Language |
| `duration` | `float | None` | `None` | Duration |
| `segments` | `list[TranscriptionSegment] | None` | `[]` | Segments |


---

##### TranscriptionSegment

A segment of transcribed audio with timing information.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `id` | `int` | — | Unique identifier |
| `start` | `float` | — | Start |
| `end` | `float` | — | End |
| `text` | `str` | — | Text |


---

##### Usage

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `prompt_tokens` | `int` | — | Prompt tokens used. Defaults to 0 when absent (some providers omit this). |
| `completion_tokens` | `int` | — | Completion tokens used. Defaults to 0 when absent (e.g. embedding responses). |
| `total_tokens` | `int` | — | Total tokens used. Defaults to 0 when absent (some providers omit this). |


---

##### UserMessage

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `content` | `UserContent` | `UserContent.TEXT` | The extracted text content |
| `name` | `str | None` | `None` | The name |


---

#### Enums

##### Message

A chat message in a conversation.

| Value | Description |
|-------|-------------|
| `SYSTEM` | System — Fields: `0`: `SystemMessage` |
| `USER` | User — Fields: `0`: `UserMessage` |
| `ASSISTANT` | Assistant — Fields: `0`: `AssistantMessage` |
| `TOOL` | Tool — Fields: `0`: `ToolMessage` |
| `DEVELOPER` | Developer — Fields: `0`: `DeveloperMessage` |
| `FUNCTION` | Deprecated legacy function-role message; retained for API compatibility. — Fields: `0`: `FunctionMessage` |


---

##### UserContent

| Value | Description |
|-------|-------------|
| `TEXT` | Text format — Fields: `0`: `str` |
| `PARTS` | Parts — Fields: `0`: `list[ContentPart]` |


---

##### ContentPart

| Value | Description |
|-------|-------------|
| `TEXT` | Text format — Fields: `text`: `str` |
| `IMAGE_URL` | Image url — Fields: `image_url`: `ImageUrl` |
| `DOCUMENT` | Document — Fields: `document`: `DocumentContent` |
| `INPUT_AUDIO` | Input audio — Fields: `input_audio`: `AudioContent` |


---

##### ImageDetail

| Value | Description |
|-------|-------------|
| `LOW` | Low |
| `HIGH` | High |
| `AUTO` | Auto |


---

##### ToolType

The type discriminator for tool/tool-call objects. Per the OpenAI spec this
is always `"function"`. Using an enum enforces that constraint at the type
level and rejects any other value on deserialization.

| Value | Description |
|-------|-------------|
| `FUNCTION` | Function |


---

##### ToolChoice

| Value | Description |
|-------|-------------|
| `MODE` | Mode — Fields: `0`: `ToolChoiceMode` |
| `SPECIFIC` | Specific — Fields: `0`: `SpecificToolChoice` |


---

##### ToolChoiceMode

| Value | Description |
|-------|-------------|
| `AUTO` | Auto |
| `REQUIRED` | Required |
| `NONE` | None |


---

##### ResponseFormat

| Value | Description |
|-------|-------------|
| `TEXT` | Text format |
| `JSON_OBJECT` | Json object |
| `JSON_SCHEMA` | Json schema — Fields: `json_schema`: `JsonSchemaFormat` |


---

##### StopSequence

| Value | Description |
|-------|-------------|
| `SINGLE` | Single — Fields: `0`: `str` |
| `MULTIPLE` | Multiple — Fields: `0`: `list[str]` |


---

##### FinishReason

Why a choice stopped generating tokens.

| Value | Description |
|-------|-------------|
| `STOP` | Stop |
| `LENGTH` | Length |
| `TOOL_CALLS` | Tool calls |
| `CONTENT_FILTER` | Content filter |
| `FUNCTION_CALL` | Deprecated legacy finish reason; retained for API compatibility. |
| `OTHER` | Catch-all for unknown finish reasons returned by non-OpenAI providers. Note: this intentionally does **not** carry the original string (e.g. `Other(String)`).  Using `#[serde(other)]` requires a unit variant, and switching to `#[serde(untagged)]` would change deserialization semantics for all variants.  The original value can be recovered by inspecting the raw JSON if needed. |


---

##### ReasoningEffort

Controls how much reasoning effort the model should use.

| Value | Description |
|-------|-------------|
| `LOW` | Low |
| `MEDIUM` | Medium |
| `HIGH` | High |


---

##### EmbeddingFormat

The format in which the embedding vectors are returned.

| Value | Description |
|-------|-------------|
| `FLOAT` | 32-bit floating-point numbers (default). |
| `BASE64` | Base64-encoded string representation of the floats. |


---

##### EmbeddingInput

| Value | Description |
|-------|-------------|
| `SINGLE` | Single — Fields: `0`: `str` |
| `MULTIPLE` | Multiple — Fields: `0`: `list[str]` |


---

##### ModerationInput

Input to the moderation endpoint — a single string or multiple strings.

| Value | Description |
|-------|-------------|
| `SINGLE` | Single — Fields: `0`: `str` |
| `MULTIPLE` | Multiple — Fields: `0`: `list[str]` |


---

##### RerankDocument

A document to be reranked — either a plain string or an object with a text field.

| Value | Description |
|-------|-------------|
| `TEXT` | Text format — Fields: `0`: `str` |
| `OBJECT` | Object — Fields: `text`: `str` |


---

##### OcrDocument

Document input for OCR — either a URL or inline base64 data.

| Value | Description |
|-------|-------------|
| `URL` | A publicly accessible document URL. — Fields: `url`: `str` |
| `BASE64` | Inline base64-encoded document data. — Fields: `data`: `str`, `media_type`: `str` |


---

##### AuthHeaderFormat

How the API key is sent in the HTTP request.

| Value | Description |
|-------|-------------|
| `BEARER` | Bearer token: `Authorization: Bearer <key>` |
| `API_KEY` | Custom header: e.g., `X-Api-Key: <key>` — Fields: `0`: `str` |
| `NONE` | No authentication required. |


---

#### Errors

##### LiterLlmError

All errors that can occur when using `liter-llm`.

**Base class:** `LiterLlmError(Exception)`

| Exception | Description |
|-----------|-------------|
| `Authentication(LiterLlmError)` | authentication failed: {message} |
| `RateLimited(LiterLlmError)` | rate limited: {message} |
| `BadRequest(LiterLlmError)` | bad request: {message} |
| `ContextWindowExceeded(LiterLlmError)` | context window exceeded: {message} |
| `ContentPolicy(LiterLlmError)` | content policy violation: {message} |
| `NotFound(LiterLlmError)` | not found: {message} |
| `ServerError(LiterLlmError)` | server error: {message} |
| `ServiceUnavailable(LiterLlmError)` | service unavailable: {message} |
| `Timeout(LiterLlmError)` | request timeout |
| `Streaming(LiterLlmError)` | A catch-all for errors that occur during streaming response processing. This variant covers multiple sub-conditions including UTF-8 decoding failures, CRC/checksum mismatches (AWS EventStream), JSON parse errors in individual SSE chunks, and buffer overflow conditions.  The `message` field contains a human-readable description of the specific failure. |
| `EndpointNotSupported(LiterLlmError)` | provider {provider} does not support {endpoint} |
| `InvalidHeader(LiterLlmError)` | invalid header {name:?}: {reason} |
| `Serialization(LiterLlmError)` | serialization error: {0} |
| `BudgetExceeded(LiterLlmError)` | budget exceeded: {message} |
| `HookRejected(LiterLlmError)` | hook rejected: {message} |
| `InternalError(LiterLlmError)` | An internal logic error (e.g. unexpected Tower response variant). This should never surface in normal operation — if it does, it indicates a bug in the library. |


---
