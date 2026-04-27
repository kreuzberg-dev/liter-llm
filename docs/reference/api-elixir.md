---
title: "Elixir API Reference"
---

## Elixir API Reference <span class="version-badge">v1.4.0-rc.3</span>

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

```elixir
@spec create_client(api_key, base_url, timeout_secs, max_retries, model_hint) :: {:ok, term()} | {:error, term()}
def create_client(api_key, base_url, timeout_secs, max_retries, model_hint)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `api_key` | `String.t()` | Yes | The api key |
| `base_url` | `String.t() | nil` | No | The base url |
| `timeout_secs` | `integer() | nil` | No | The timeout secs |
| `max_retries` | `integer() | nil` | No | The max retries |
| `model_hint` | `String.t() | nil` | No | The model hint |

**Returns:** `DefaultClient`

**Errors:** Returns `{:error, reason}`


---

#### create_client_from_json()

Create a new LLM client from a JSON string.

The JSON object accepts the same fields as `liter-llm.toml` (snake_case).

**Errors:**

Returns `LiterLlmError.BadRequest` if `json` is not valid JSON or
contains unknown fields.

**Signature:**

```elixir
@spec create_client_from_json(json) :: {:ok, term()} | {:error, term()}
def create_client_from_json(json)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `json` | `String.t()` | Yes | The json |

**Returns:** `DefaultClient`

**Errors:** Returns `{:error, reason}`


---

#### register_custom_provider()

Register a custom provider in the global runtime registry.

The provider will be checked **before** all built-in providers during model
detection. If a provider with the same `name` already exists it is replaced.

**Errors:**

Returns an error if the config is invalid (empty name, empty base_url, or
no model prefixes).

**Signature:**

```elixir
@spec register_custom_provider(config) :: {:ok, term()} | {:error, term()}
def register_custom_provider(config)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `config` | `CustomProviderConfig` | Yes | The configuration options |

**Returns:** `:ok`

**Errors:** Returns `{:error, reason}`


---

#### unregister_custom_provider()

Remove a previously registered custom provider by name.

Returns `true` if a provider with the given name was found and removed,
`false` if no such provider existed.

**Errors:**

Returns an error only if the internal lock is poisoned.

**Signature:**

```elixir
@spec unregister_custom_provider(name) :: {:ok, term()} | {:error, term()}
def unregister_custom_provider(name)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `name` | `String.t()` | Yes | The name |

**Returns:** `boolean()`

**Errors:** Returns `{:error, reason}`


---

### Types

#### ApiError

Inner error object.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `message` | `String.t()` | — | Message |
| `error_type` | `String.t()` | — | Error type |
| `param` | `String.t() | nil` | `nil` | Param |
| `code` | `String.t() | nil` | `nil` | Code |


---

#### AssistantMessage

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `content` | `String.t() | nil` | `nil` | The extracted text content |
| `name` | `String.t() | nil` | `nil` | The name |
| `tool_calls` | `list(ToolCall) | nil` | `[]` | Tool calls |
| `refusal` | `String.t() | nil` | `nil` | Refusal |
| `function_call` | `FunctionCall | nil` | `nil` | Deprecated legacy function_call field; retained for API compatibility. |


---

#### AudioContent

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `data` | `String.t()` | — | Base64-encoded audio data. |
| `format` | `String.t()` | — | Audio format (e.g., "wav", "mp3", "ogg"). |


---

#### BatchClient

Batch processing operations (create, list, retrieve, cancel).

##### Functions

###### create_batch()

Create a new batch job.

**Signature:**

```elixir
def create_batch(req)
```

###### retrieve_batch()

Retrieve a batch by ID.

**Signature:**

```elixir
def retrieve_batch(batch_id)
```

###### list_batches()

List batches, optionally filtered by query parameters.

**Signature:**

```elixir
def list_batches(query)
```

###### cancel_batch()

Cancel an in-progress batch.

**Signature:**

```elixir
def cancel_batch(batch_id)
```


---

#### ChatCompletionChunk

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `id` | `String.t()` | — | Unique identifier |
| `object` | `String.t()` | — | Always `"chat.completion.chunk"` from OpenAI-compatible APIs.  Stored as a plain `String` so non-standard provider values do not fail parsing. |
| `created` | `integer()` | — | Created |
| `model` | `String.t()` | — | Model |
| `choices` | `list(StreamChoice)` | `[]` | Choices |
| `usage` | `Usage | nil` | `nil` | Usage (usage) |
| `system_fingerprint` | `String.t() | nil` | `nil` | System fingerprint |
| `service_tier` | `String.t() | nil` | `nil` | Service tier |


---

#### ChatCompletionRequest

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `model` | `String.t()` | — | Model |
| `messages` | `list(Message)` | `[]` | Messages |
| `temperature` | `float() | nil` | `nil` | Temperature |
| `top_p` | `float() | nil` | `nil` | Top p |
| `n` | `integer() | nil` | `nil` | N |
| `stream` | `boolean() | nil` | `nil` | Whether to stream the response. Managed by the client layer — do not set directly. |
| `stop` | `StopSequence | nil` | `nil` | Stop (stop sequence) |
| `max_tokens` | `integer() | nil` | `nil` | Maximum tokens |
| `presence_penalty` | `float() | nil` | `nil` | Presence penalty |
| `frequency_penalty` | `float() | nil` | `nil` | Frequency penalty |
| `logit_bias` | `map() | nil` | `%{}` | Token bias map.  Uses `BTreeMap` (sorted keys) for deterministic serialization order — important when hashing or signing requests. |
| `user` | `String.t() | nil` | `nil` | User |
| `tools` | `list(ChatCompletionTool) | nil` | `[]` | Tools |
| `tool_choice` | `ToolChoice | nil` | `nil` | Tool choice (tool choice) |
| `parallel_tool_calls` | `boolean() | nil` | `nil` | Parallel tool calls |
| `response_format` | `ResponseFormat | nil` | `nil` | Response format (response format) |
| `stream_options` | `StreamOptions | nil` | `nil` | Stream options (stream options) |
| `seed` | `integer() | nil` | `nil` | Seed |
| `reasoning_effort` | `ReasoningEffort | nil` | `nil` | Reasoning effort (reasoning effort) |
| `extra_body` | `term() | nil` | `nil` | Provider-specific extra parameters merged into the request body. Use for guardrails, safety settings, grounding config, etc. |


---

#### ChatCompletionResponse

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `id` | `String.t()` | — | Unique identifier |
| `object` | `String.t()` | — | Always `"chat.completion"` from OpenAI-compatible APIs.  Stored as a plain `String` so non-standard provider values do not break deserialization. |
| `created` | `integer()` | — | Created |
| `model` | `String.t()` | — | Model |
| `choices` | `list(Choice)` | `[]` | Choices |
| `usage` | `Usage | nil` | `nil` | Usage (usage) |
| `system_fingerprint` | `String.t() | nil` | `nil` | System fingerprint |
| `service_tier` | `String.t() | nil` | `nil` | Service tier |

##### Functions

###### estimated_cost()

Estimate the cost of this response based on embedded pricing data.

Returns `nil` if:
- the `model` field is not present in the embedded pricing registry, or
- the `usage` field is absent from the response.

**Signature:**

```elixir
def estimated_cost()
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
| `index` | `integer()` | — | Index |
| `message` | `AssistantMessage` | — | Message (assistant message) |
| `finish_reason` | `FinishReason | nil` | `nil` | Finish reason (finish reason) |


---

#### ClientConfig

Configuration for an LLM client.

`api_key` is stored as a `SecretString` so it is zeroed on drop and never
printed accidentally.  Access it via `secrecy.ExposeSecret`.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `api_key` | `String.t()` | — | API key for authentication (stored as a secret). |
| `base_url` | `String.t() | nil` | `nil` | Override base URL.  When set, all requests go here regardless of model name, and provider auto-detection is skipped. |
| `timeout` | `integer()` | — | Request timeout. |
| `max_retries` | `integer()` | — | Maximum number of retries on 429 / 5xx responses. |
| `credential_provider` | `CredentialProvider | nil` | `nil` | Optional dynamic credential provider for token-based auth (Azure AD, Vertex OAuth2) or refreshable credentials (AWS STS). When set, the client calls `resolve()` before each request to obtain a fresh credential.  When `None`, the static `api_key` is used. |
| `load_env` | `boolean()` | — | Automatically load the API key from the provider's environment variable when no explicit key is provided. When `True` (the default) and `api_key` is empty, `DefaultClient.new` reads the provider's designated environment variable (e.g. `OPENAI_API_KEY` for OpenAI).  Set to `False` to suppress this behaviour and require the caller to supply the key explicitly. Has no effect on WASM targets, where `std.env.var` is unavailable. |

##### Functions

###### headers()

Return the extra headers as an ordered slice of `(name, value)` pairs.

**Signature:**

```elixir
def headers()
```

###### fmt()

**Signature:**

```elixir
def fmt(f)
```


---

#### ClientConfigBuilder

Builder for `ClientConfig`.

Construct with `ClientConfigBuilder.new` and call builder methods to
customise the configuration, then call `ClientConfigBuilder.build` to
obtain a `ClientConfig`.

##### Functions

###### from_env()

Create a builder with no explicit API key.

`load_env` is `true` by default, so the key will be read from the
provider's environment variable (e.g. `OPENAI_API_KEY`) at client
construction time.  Call `.load_env(false)` to opt out.

**Signature:**

```elixir
def from_env()
```

###### load_env()

Enable or disable automatic API key loading from environment variables.

When `true` (the default) and no explicit `api_key` was provided,
`DefaultClient.new` reads the provider's designated environment
variable.  Set to `false` to require an explicit key.

Has no effect on WASM targets.

**Signature:**

```elixir
def load_env(enabled)
```

###### base_url()

Override the provider base URL for all requests.

**Signature:**

```elixir
def base_url(url)
```

###### timeout()

Set the per-request timeout (default: 60 s).

**Signature:**

```elixir
def timeout(timeout)
```

###### max_retries()

Set the maximum number of retries on 429 / 5xx responses (default: 3).

**Signature:**

```elixir
def max_retries(retries)
```

###### credential_provider()

Set a dynamic credential provider for token-based or refreshable auth.

When configured, the client calls `resolve()` before each request
instead of using the static `api_key` for authentication.

**Signature:**

```elixir
def credential_provider(provider)
```

###### header()

Add a custom header sent on every request.

Returns an error if either `key` or `value` is not a valid HTTP header
name / value.

This method is only available when the `native-http` feature is enabled
because header validation relies on `reqwest`'s header types.

**Signature:**

```elixir
def header(key, value)
```

###### cache()

Set the response cache configuration for the Tower middleware stack.

When set, bindings and advanced Rust users can read this from the
built `ClientConfig` to construct a
`CacheLayer`.

**Signature:**

```elixir
def cache(config)
```

###### cache_store()

Set a custom cache store backend for the Tower cache middleware.

When set alongside `cache`, the cache layer will use
this store instead of the default in-memory LRU.

**Signature:**

```elixir
def cache_store(store)
```

###### budget()

Set the budget enforcement configuration for the Tower middleware stack.

When set, bindings and advanced Rust users can read this from the
built `ClientConfig` to construct a
`BudgetLayer`.

**Signature:**

```elixir
def budget(config)
```

###### hook()

Add a single hook to the Tower hooks middleware stack.

Hooks are invoked sequentially in registration order at request
lifecycle points (pre-request, post-response, on-error).

**Signature:**

```elixir
def hook(hook)
```

###### hooks()

Set the full list of hooks for the Tower hooks middleware stack,
replacing any previously registered hooks.

Hooks are invoked sequentially in registration order.

**Signature:**

```elixir
def hooks(hooks)
```

###### cooldown()

Set the cooldown duration after transient errors.

When set, the client rejects requests with `ServiceUnavailable` for
the given duration after a transient error (rate limit, timeout,
server error).

**Signature:**

```elixir
def cooldown(duration)
```

###### rate_limit()

Set per-model rate limiting configuration.

When set, requests exceeding the configured RPM or TPM limits are
rejected with `LiterLlmError.RateLimited`.

**Signature:**

```elixir
def rate_limit(config)
```

###### health_check()

Set the background health check interval.

When set, the client periodically probes the provider and rejects
requests when the provider is unhealthy.

**Signature:**

```elixir
def health_check(interval)
```

###### cost_tracking()

Enable or disable per-request cost tracking.

When enabled, estimated USD cost is recorded on the current tracing
span as `gen_ai.usage.cost`.

**Signature:**

```elixir
def cost_tracking(enabled)
```

###### tracing()

Enable or disable OpenTelemetry-compatible tracing spans.

When enabled, every request is wrapped in a `gen_ai` tracing span
with semantic convention attributes.

**Signature:**

```elixir
def tracing(enabled)
```

###### build()

Consume the builder and return the completed `ClientConfig`.

**Signature:**

```elixir
def build()
```


---

#### CreateImageRequest

Request to create images from a text prompt.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `prompt` | `String.t()` | — | Prompt |
| `model` | `String.t() | nil` | `nil` | Model |
| `n` | `integer() | nil` | `nil` | N |
| `size` | `String.t() | nil` | `nil` | Size in bytes |
| `quality` | `String.t() | nil` | `nil` | Quality |
| `style` | `String.t() | nil` | `nil` | Style |
| `response_format` | `String.t() | nil` | `nil` | Response format |
| `user` | `String.t() | nil` | `nil` | User |


---

#### CreateSpeechRequest

Request to generate speech audio from text.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `model` | `String.t()` | — | Model |
| `input` | `String.t()` | — | Input |
| `voice` | `String.t()` | — | Voice |
| `response_format` | `String.t() | nil` | `nil` | Response format |
| `speed` | `float() | nil` | `nil` | Speed |


---

#### CreateTranscriptionRequest

Request to transcribe audio into text.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `model` | `String.t()` | — | Model |
| `file` | `String.t()` | — | Base64-encoded audio file data. |
| `language` | `String.t() | nil` | `nil` | Language |
| `prompt` | `String.t() | nil` | `nil` | Prompt |
| `response_format` | `String.t() | nil` | `nil` | Response format |
| `temperature` | `float() | nil` | `nil` | Temperature |


---

#### CustomProviderConfig

Configuration for registering a custom LLM provider at runtime.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `name` | `String.t()` | — | Unique name for this provider (e.g., "my-provider"). |
| `base_url` | `String.t()` | — | Base URL for the provider's API (e.g., "<https://api.my-provider.com/v1">). |
| `auth_header` | `AuthHeaderFormat` | — | Authentication header format. |
| `model_prefixes` | `list(String.t())` | — | Model name prefixes that route to this provider (e.g., ["my-"]). |


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

##### Functions

###### new()

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

```elixir
def new(config, model_hint)
```

###### chat()

**Signature:**

```elixir
def chat(req)
```

###### chat_stream()

**Signature:**

```elixir
def chat_stream(req)
```

###### embed()

**Signature:**

```elixir
def embed(req)
```

###### list_models()

**Signature:**

```elixir
def list_models()
```

###### image_generate()

**Signature:**

```elixir
def image_generate(req)
```

###### speech()

**Signature:**

```elixir
def speech(req)
```

###### transcribe()

**Signature:**

```elixir
def transcribe(req)
```

###### moderate()

**Signature:**

```elixir
def moderate(req)
```

###### rerank()

**Signature:**

```elixir
def rerank(req)
```

###### search()

**Signature:**

```elixir
def search(req)
```

###### ocr()

**Signature:**

```elixir
def ocr(req)
```

###### chat_raw()

**Signature:**

```elixir
def chat_raw(req)
```

###### chat_stream_raw()

**Signature:**

```elixir
def chat_stream_raw(req)
```

###### embed_raw()

**Signature:**

```elixir
def embed_raw(req)
```

###### image_generate_raw()

**Signature:**

```elixir
def image_generate_raw(req)
```

###### transcribe_raw()

**Signature:**

```elixir
def transcribe_raw(req)
```

###### moderate_raw()

**Signature:**

```elixir
def moderate_raw(req)
```

###### rerank_raw()

**Signature:**

```elixir
def rerank_raw(req)
```

###### search_raw()

**Signature:**

```elixir
def search_raw(req)
```

###### ocr_raw()

**Signature:**

```elixir
def ocr_raw(req)
```

###### create_file()

**Signature:**

```elixir
def create_file(req)
```

###### retrieve_file()

**Signature:**

```elixir
def retrieve_file(file_id)
```

###### delete_file()

**Signature:**

```elixir
def delete_file(file_id)
```

###### list_files()

**Signature:**

```elixir
def list_files(query)
```

###### file_content()

**Signature:**

```elixir
def file_content(file_id)
```

###### create_batch()

**Signature:**

```elixir
def create_batch(req)
```

###### retrieve_batch()

**Signature:**

```elixir
def retrieve_batch(batch_id)
```

###### list_batches()

**Signature:**

```elixir
def list_batches(query)
```

###### cancel_batch()

**Signature:**

```elixir
def cancel_batch(batch_id)
```

###### create_response()

**Signature:**

```elixir
def create_response(req)
```

###### retrieve_response()

**Signature:**

```elixir
def retrieve_response(id)
```

###### cancel_response()

**Signature:**

```elixir
def cancel_response(id)
```


---

#### DeveloperMessage

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `content` | `String.t()` | — | The extracted text content |
| `name` | `String.t() | nil` | `nil` | The name |


---

#### DocumentContent

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `data` | `String.t()` | — | Base64-encoded document data or URL. |
| `media_type` | `String.t()` | — | MIME type (e.g., "application/pdf", "text/csv"). |


---

#### EmbeddingObject

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `object` | `String.t()` | — | Always `"embedding"` from OpenAI-compatible APIs.  Stored as a plain `String` so non-standard provider values do not break deserialization. |
| `embedding` | `list(float())` | — | Embedding |
| `index` | `integer()` | — | Index |


---

#### EmbeddingRequest

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `model` | `String.t()` | — | Model |
| `input` | `EmbeddingInput` | — | Input (embedding input) |
| `encoding_format` | `EmbeddingFormat | nil` | `nil` | Encoding format (embedding format) |
| `dimensions` | `integer() | nil` | `nil` | Dimensions |
| `user` | `String.t() | nil` | `nil` | User |


---

#### EmbeddingResponse

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `object` | `String.t()` | — | Always `"list"` from OpenAI-compatible APIs.  Stored as a plain `String` so non-standard provider values do not break deserialization. |
| `data` | `list(EmbeddingObject)` | — | Data |
| `model` | `String.t()` | — | Model |
| `usage` | `Usage | nil` | `nil` | Usage (usage) |

##### Functions

###### estimated_cost()

Estimate the cost of this embedding request based on embedded pricing data.

Returns `nil` if:
- the `model` field is not present in the embedded pricing registry, or
- the `usage` field is absent from the response.

Embedding models only charge for input tokens; output cost is zero.

**Signature:**

```elixir
def estimated_cost()
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
| `global_limit` | `float() | nil` | `nil` | Global limit |
| `model_limits` | `map() | nil` | `nil` | Model limits |
| `enforcement` | `String.t() | nil` | `nil` | Enforcement |


---

#### FileCacheConfig

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `max_entries` | `integer() | nil` | `nil` | Maximum entries |
| `ttl_seconds` | `integer() | nil` | `nil` | Ttl seconds |
| `backend` | `String.t() | nil` | `nil` | Backend |
| `backend_config` | `map() | nil` | `nil` | Backend config |


---

#### FileClient

File management operations (upload, list, retrieve, delete).

##### Functions

###### create_file()

Upload a file.

**Signature:**

```elixir
def create_file(req)
```

###### retrieve_file()

Retrieve metadata for a file.

**Signature:**

```elixir
def retrieve_file(file_id)
```

###### delete_file()

Delete a file.

**Signature:**

```elixir
def delete_file(file_id)
```

###### list_files()

List files, optionally filtered by query parameters.

**Signature:**

```elixir
def list_files(query)
```

###### file_content()

Retrieve the raw content of a file.

**Signature:**

```elixir
def file_content(file_id)
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
| `api_key` | `String.t() | nil` | `nil` | Api key |
| `base_url` | `String.t() | nil` | `nil` | Base url |
| `model_hint` | `String.t() | nil` | `nil` | Model hint |
| `timeout_secs` | `integer() | nil` | `nil` | Timeout secs |
| `max_retries` | `integer() | nil` | `nil` | Maximum retries |
| `extra_headers` | `map() | nil` | `nil` | Extra headers |
| `cache` | `FileCacheConfig | nil` | `nil` | Cache (file cache config) |
| `budget` | `FileBudgetConfig | nil` | `nil` | Budget (file budget config) |
| `cooldown_secs` | `integer() | nil` | `nil` | Cooldown secs |
| `rate_limit` | `FileRateLimitConfig | nil` | `nil` | Rate limit (file rate limit config) |
| `health_check_secs` | `integer() | nil` | `nil` | Health check secs |
| `cost_tracking` | `boolean() | nil` | `nil` | Cost tracking |
| `tracing` | `boolean() | nil` | `nil` | Tracing |
| `providers` | `list(FileProviderConfig) | nil` | `nil` | Providers |

##### Functions

###### from_toml_file()

Load from a TOML file path.

**Signature:**

```elixir
def from_toml_file(path)
```

###### from_toml_str()

Parse from a TOML string.

**Signature:**

```elixir
def from_toml_str(s)
```

###### discover()

Discover `liter-llm.toml` by walking from current directory to filesystem root.

Returns `Ok(None)` if no config file is found.

**Signature:**

```elixir
def discover()
```

###### into_builder()

Convert into a `ClientConfigBuilder`,
applying all fields that are set.

Fields not present in the TOML file use the builder's defaults.

**Signature:**

```elixir
def into_builder()
```

###### providers()

Get the custom provider configurations from this file config.

**Signature:**

```elixir
def providers()
```


---

#### FileProviderConfig

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `name` | `String.t()` | — | The name |
| `base_url` | `String.t()` | — | Base url |
| `auth_header` | `String.t() | nil` | `nil` | Auth header |
| `model_prefixes` | `list(String.t())` | — | Model prefixes |


---

#### FileRateLimitConfig

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `rpm` | `integer() | nil` | `nil` | Rpm |
| `tpm` | `integer() | nil` | `nil` | Tpm |
| `window_seconds` | `integer() | nil` | `nil` | Window seconds |


---

#### FunctionCall

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `name` | `String.t()` | — | The name |
| `arguments` | `String.t()` | — | Arguments |


---

#### FunctionDefinition

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `name` | `String.t()` | — | The name |
| `description` | `String.t() | nil` | `nil` | Human-readable description |
| `parameters` | `term() | nil` | `nil` | Parameters |
| `strict` | `boolean() | nil` | `nil` | Strict |


---

#### FunctionMessage

Deprecated legacy function-role message body.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `content` | `String.t()` | — | The extracted text content |
| `name` | `String.t()` | — | The name |


---

#### Image

A single generated image, returned as either a URL or base64 data.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `url` | `String.t() | nil` | `nil` | Url |
| `b64_json` | `String.t() | nil` | `nil` | B64 json |
| `revised_prompt` | `String.t() | nil` | `nil` | Revised prompt |


---

#### ImageUrl

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `url` | `String.t()` | — | Url |
| `detail` | `ImageDetail | nil` | `nil` | Detail (image detail) |


---

#### ImagesResponse

Response containing generated images.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `created` | `integer()` | — | Created |
| `data` | `list(Image)` | `[]` | Data |


---

#### JsonSchemaFormat

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `name` | `String.t()` | — | The name |
| `description` | `String.t() | nil` | `nil` | Human-readable description |
| `schema` | `term()` | — | Schema |
| `strict` | `boolean() | nil` | `nil` | Strict |


---

#### LiterLlmError

##### Functions

###### is_transient()

Returns `true` for errors that are worth retrying on a different service
or deployment (transient failures).

Used by `crate.tower.fallback.FallbackService` and
`crate.tower.router.Router` to decide whether to route to an
alternative endpoint.

**Signature:**

```elixir
def is_transient()
```

###### error_type()

Return the OpenTelemetry `error.type` string for this error variant.

Used by the tracing middleware to record the `error.type` span attribute
on failed requests per the GenAI semantic conventions.

**Signature:**

```elixir
def error_type()
```

###### from_status()

Create from an HTTP status code, an API error response body, and an
optional `Retry-After` duration already parsed from the response header.

The `retry_after` value is forwarded into `LiterLlmError.RateLimited`
so callers can honour the server-requested delay without re-parsing the
header.

**Signature:**

```elixir
def from_status(status, body, retry_after)
```


---

#### LlmClient

Core LLM client trait.

##### Functions

###### chat()

Send a chat completion request.

**Signature:**

```elixir
def chat(req)
```

###### chat_stream()

Send a streaming chat completion request.

**Signature:**

```elixir
def chat_stream(req)
```

###### embed()

Send an embedding request.

**Signature:**

```elixir
def embed(req)
```

###### list_models()

List available models.

**Signature:**

```elixir
def list_models()
```

###### image_generate()

Generate an image.

**Signature:**

```elixir
def image_generate(req)
```

###### speech()

Generate speech audio from text.

**Signature:**

```elixir
def speech(req)
```

###### transcribe()

Transcribe audio to text.

**Signature:**

```elixir
def transcribe(req)
```

###### moderate()

Check content against moderation policies.

**Signature:**

```elixir
def moderate(req)
```

###### rerank()

Rerank documents by relevance to a query.

**Signature:**

```elixir
def rerank(req)
```

###### search()

Perform a web/document search.

**Signature:**

```elixir
def search(req)
```

###### ocr()

Extract text from a document via OCR.

**Signature:**

```elixir
def ocr(req)
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

##### Functions

###### chat_raw()

Send a chat completion request and return the raw exchange.

The `raw_request` field contains the final JSON body sent to the
provider; `raw_response` contains the provider JSON before
normalization.

**Signature:**

```elixir
def chat_raw(req)
```

###### chat_stream_raw()

Send a streaming chat completion request and return the raw exchange.

Only `raw_request` is available upfront — the stream itself is
returned in `stream` and consumed incrementally.

**Signature:**

```elixir
def chat_stream_raw(req)
```

###### embed_raw()

Send an embedding request and return the raw exchange.

**Signature:**

```elixir
def embed_raw(req)
```

###### image_generate_raw()

Generate an image and return the raw exchange.

**Signature:**

```elixir
def image_generate_raw(req)
```

###### transcribe_raw()

Transcribe audio to text and return the raw exchange.

**Signature:**

```elixir
def transcribe_raw(req)
```

###### moderate_raw()

Check content against moderation policies and return the raw exchange.

**Signature:**

```elixir
def moderate_raw(req)
```

###### rerank_raw()

Rerank documents by relevance to a query and return the raw exchange.

**Signature:**

```elixir
def rerank_raw(req)
```

###### search_raw()

Perform a web/document search and return the raw exchange.

**Signature:**

```elixir
def search_raw(req)
```

###### ocr_raw()

Extract text from a document via OCR and return the raw exchange.

**Signature:**

```elixir
def ocr_raw(req)
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

##### Functions

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

```elixir
def new(config, model_hint)
```

###### inner()

Return a reference to the underlying `DefaultClient`.

**Signature:**

```elixir
def inner()
```

###### budget_state()

Return the budget state handle, if budget middleware is configured.

Use this to query accumulated spend at runtime.

**Signature:**

```elixir
def budget_state()
```

###### has_middleware()

Return `true` when middleware is active (requests go through the Tower
service stack).

**Signature:**

```elixir
def has_middleware()
```

###### chat()

**Signature:**

```elixir
def chat(req)
```

###### chat_stream()

**Signature:**

```elixir
def chat_stream(req)
```

###### embed()

**Signature:**

```elixir
def embed(req)
```

###### list_models()

**Signature:**

```elixir
def list_models()
```

###### image_generate()

**Signature:**

```elixir
def image_generate(req)
```

###### speech()

**Signature:**

```elixir
def speech(req)
```

###### transcribe()

**Signature:**

```elixir
def transcribe(req)
```

###### moderate()

**Signature:**

```elixir
def moderate(req)
```

###### rerank()

**Signature:**

```elixir
def rerank(req)
```

###### search()

**Signature:**

```elixir
def search(req)
```

###### ocr()

**Signature:**

```elixir
def ocr(req)
```

###### create_file()

**Signature:**

```elixir
def create_file(req)
```

###### retrieve_file()

**Signature:**

```elixir
def retrieve_file(file_id)
```

###### delete_file()

**Signature:**

```elixir
def delete_file(file_id)
```

###### list_files()

**Signature:**

```elixir
def list_files(query)
```

###### file_content()

**Signature:**

```elixir
def file_content(file_id)
```

###### create_batch()

**Signature:**

```elixir
def create_batch(req)
```

###### retrieve_batch()

**Signature:**

```elixir
def retrieve_batch(batch_id)
```

###### list_batches()

**Signature:**

```elixir
def list_batches(query)
```

###### cancel_batch()

**Signature:**

```elixir
def cancel_batch(batch_id)
```

###### create_response()

**Signature:**

```elixir
def create_response(req)
```

###### retrieve_response()

**Signature:**

```elixir
def retrieve_response(id)
```

###### cancel_response()

**Signature:**

```elixir
def cancel_response(id)
```


---

#### ModelObject

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `id` | `String.t()` | — | Unique identifier |
| `object` | `String.t()` | — | Always `"model"` from OpenAI-compatible APIs.  Stored as a plain `String` so non-standard provider values do not break deserialization. |
| `created` | `integer()` | — | Created |
| `owned_by` | `String.t()` | — | Owned by |


---

#### ModelsListResponse

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `object` | `String.t()` | — | Always `"list"` from OpenAI-compatible APIs.  Stored as a plain `String` so non-standard provider values do not break deserialization. |
| `data` | `list(ModelObject)` | `[]` | Data |


---

#### ModerationCategories

Boolean flags for each moderation category.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `sexual` | `boolean()` | — | Sexual |
| `hate` | `boolean()` | — | Hate |
| `harassment` | `boolean()` | — | Harassment |
| `self_harm` | `boolean()` | — | Self harm |
| `sexual_minors` | `boolean()` | — | Sexual minors |
| `hate_threatening` | `boolean()` | — | Hate threatening |
| `violence_graphic` | `boolean()` | — | Violence graphic |
| `self_harm_intent` | `boolean()` | — | Self harm intent |
| `self_harm_instructions` | `boolean()` | — | Self harm instructions |
| `harassment_threatening` | `boolean()` | — | Harassment threatening |
| `violence` | `boolean()` | — | Violence |


---

#### ModerationCategoryScores

Confidence scores for each moderation category.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `sexual` | `float()` | — | Sexual |
| `hate` | `float()` | — | Hate |
| `harassment` | `float()` | — | Harassment |
| `self_harm` | `float()` | — | Self harm |
| `sexual_minors` | `float()` | — | Sexual minors |
| `hate_threatening` | `float()` | — | Hate threatening |
| `violence_graphic` | `float()` | — | Violence graphic |
| `self_harm_intent` | `float()` | — | Self harm intent |
| `self_harm_instructions` | `float()` | — | Self harm instructions |
| `harassment_threatening` | `float()` | — | Harassment threatening |
| `violence` | `float()` | — | Violence |


---

#### ModerationRequest

Request to classify content for policy violations.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `input` | `ModerationInput` | — | Input (moderation input) |
| `model` | `String.t() | nil` | `nil` | Model |


---

#### ModerationResponse

Response from the moderation endpoint.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `id` | `String.t()` | — | Unique identifier |
| `model` | `String.t()` | — | Model |
| `results` | `list(ModerationResult)` | — | Results |


---

#### ModerationResult

A single moderation classification result.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `flagged` | `boolean()` | — | Flagged |
| `categories` | `ModerationCategories` | — | Categories (moderation categories) |
| `category_scores` | `ModerationCategoryScores` | — | Category scores (moderation category scores) |


---

#### OcrImage

An image extracted from an OCR page.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `id` | `String.t()` | — | Unique image identifier. |
| `image_base64` | `String.t() | nil` | `nil` | Base64-encoded image data. |


---

#### OcrPage

A single page of OCR output.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `index` | `integer()` | — | Page index (0-based). |
| `markdown` | `String.t()` | — | Extracted content as Markdown. |
| `images` | `list(OcrImage) | nil` | `nil` | Extracted images, if `include_image_base64` was set. |
| `dimensions` | `PageDimensions | nil` | `nil` | Page dimensions in pixels, if available. |


---

#### OcrRequest

An OCR request.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `model` | `String.t()` | — | The model/provider to use (e.g. `"mistral/mistral-ocr-latest"`). |
| `document` | `OcrDocument` | — | The document to process. |
| `pages` | `list(integer()) | nil` | `nil` | Specific pages to process (1-indexed). `None` means all pages. |
| `include_image_base64` | `boolean() | nil` | `nil` | Whether to include base64-encoded images of each page. |


---

#### OcrResponse

An OCR response.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `pages` | `list(OcrPage)` | — | Extracted pages. |
| `model` | `String.t()` | — | The model used. |
| `usage` | `Usage | nil` | `nil` | Token usage, if reported by the provider. |


---

#### PageDimensions

Page dimensions in pixels.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `width` | `integer()` | — | Width in pixels. |
| `height` | `integer()` | — | Height in pixels. |


---

#### RerankRequest

Request to rerank documents by relevance to a query.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `model` | `String.t()` | — | Model |
| `query` | `String.t()` | — | Query |
| `documents` | `list(RerankDocument)` | — | Documents |
| `top_n` | `integer() | nil` | `nil` | Top n |
| `return_documents` | `boolean() | nil` | `nil` | Return documents |


---

#### RerankResponse

Response from the rerank endpoint.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `id` | `String.t() | nil` | `nil` | Unique identifier |
| `results` | `list(RerankResult)` | — | Results |
| `meta` | `term() | nil` | `nil` | Meta |


---

#### RerankResult

A single reranked document with its relevance score.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `index` | `integer()` | — | Index |
| `relevance_score` | `float()` | — | Relevance score |
| `document` | `RerankResultDocument | nil` | `nil` | Document (rerank result document) |


---

#### RerankResultDocument

The text content of a reranked document, returned when `return_documents` is true.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `text` | `String.t()` | — | Text |


---

#### ResponseClient

Responses API operations (create, retrieve, cancel).

##### Functions

###### create_response()

Create a new response.

**Signature:**

```elixir
def create_response(req)
```

###### retrieve_response()

Retrieve a response by ID.

**Signature:**

```elixir
def retrieve_response(id)
```

###### cancel_response()

Cancel an in-progress response.

**Signature:**

```elixir
def cancel_response(id)
```


---

#### SearchRequest

A search request.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `model` | `String.t()` | — | The model/provider to use (e.g. `"brave/web-search"`, `"tavily/search"`). |
| `query` | `String.t()` | — | The search query. |
| `max_results` | `integer() | nil` | `nil` | Maximum number of results to return. |
| `search_domain_filter` | `list(String.t()) | nil` | `[]` | Domain filter — restrict results to specific domains. |
| `country` | `String.t() | nil` | `nil` | Country code for localized results (ISO 3166-1 alpha-2). |


---

#### SearchResponse

A search response.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `results` | `list(SearchResult)` | — | The search results. |
| `model` | `String.t()` | — | The model used. |


---

#### SearchResult

An individual search result.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `title` | `String.t()` | — | Title of the result. |
| `url` | `String.t()` | — | URL of the result. |
| `snippet` | `String.t()` | — | Text snippet / excerpt. |
| `date` | `String.t() | nil` | `nil` | Publication or last-updated date, if available. |


---

#### SpecificFunction

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `name` | `String.t()` | — | The name |


---

#### SpecificToolChoice

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `choice_type` | `ToolType` | `:function` | Choice type (tool type) |
| `function` | `SpecificFunction` | — | Function (specific function) |


---

#### StreamChoice

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `index` | `integer()` | — | Index |
| `delta` | `StreamDelta` | — | Delta (stream delta) |
| `finish_reason` | `FinishReason | nil` | `nil` | Finish reason (finish reason) |


---

#### StreamDelta

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `role` | `String.t() | nil` | `nil` | Role |
| `content` | `String.t() | nil` | `nil` | The extracted text content |
| `tool_calls` | `list(StreamToolCall) | nil` | `[]` | Tool calls |
| `function_call` | `StreamFunctionCall | nil` | `nil` | Deprecated legacy function_call delta; retained for API compatibility. |
| `refusal` | `String.t() | nil` | `nil` | Refusal |


---

#### StreamFunctionCall

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `name` | `String.t() | nil` | `nil` | The name |
| `arguments` | `String.t() | nil` | `nil` | Arguments |


---

#### StreamOptions

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `include_usage` | `boolean() | nil` | `nil` | Include usage |


---

#### StreamToolCall

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `index` | `integer()` | — | Index |
| `id` | `String.t() | nil` | `nil` | Unique identifier |
| `call_type` | `ToolType | nil` | `nil` | Call type (tool type) |
| `function` | `StreamFunctionCall | nil` | `nil` | Function (stream function call) |


---

#### SystemMessage

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `content` | `String.t()` | — | The extracted text content |
| `name` | `String.t() | nil` | `nil` | The name |


---

#### ToolCall

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `id` | `String.t()` | — | Unique identifier |
| `call_type` | `ToolType` | — | Call type (tool type) |
| `function` | `FunctionCall` | — | Function (function call) |


---

#### ToolMessage

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `content` | `String.t()` | — | The extracted text content |
| `tool_call_id` | `String.t()` | — | Tool call id |
| `name` | `String.t() | nil` | `nil` | The name |


---

#### TranscriptionResponse

Response from a transcription request.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `text` | `String.t()` | — | Text |
| `language` | `String.t() | nil` | `nil` | Language |
| `duration` | `float() | nil` | `nil` | Duration |
| `segments` | `list(TranscriptionSegment) | nil` | `[]` | Segments |


---

#### TranscriptionSegment

A segment of transcribed audio with timing information.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `id` | `integer()` | — | Unique identifier |
| `start` | `float()` | — | Start |
| `end` | `float()` | — | End |
| `text` | `String.t()` | — | Text |


---

#### Usage

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `prompt_tokens` | `integer()` | — | Prompt tokens used. Defaults to 0 when absent (some providers omit this). |
| `completion_tokens` | `integer()` | — | Completion tokens used. Defaults to 0 when absent (e.g. embedding responses). |
| `total_tokens` | `integer()` | — | Total tokens used. Defaults to 0 when absent (some providers omit this). |


---

#### UserMessage

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `content` | `UserContent` | `:text` | The extracted text content |
| `name` | `String.t() | nil` | `nil` | The name |


---

### Enums

#### Message

A chat message in a conversation.

| Value | Description |
|-------|-------------|
| `system` | System — Fields: `0`: `SystemMessage` |
| `user` | User — Fields: `0`: `UserMessage` |
| `assistant` | Assistant — Fields: `0`: `AssistantMessage` |
| `tool` | Tool — Fields: `0`: `ToolMessage` |
| `developer` | Developer — Fields: `0`: `DeveloperMessage` |
| `function` | Deprecated legacy function-role message; retained for API compatibility. — Fields: `0`: `FunctionMessage` |


---

#### UserContent

| Value | Description |
|-------|-------------|
| `text` | Text format — Fields: `0`: `String.t()` |
| `parts` | Parts — Fields: `0`: `list(ContentPart)` |


---

#### ContentPart

| Value | Description |
|-------|-------------|
| `text` | Text format — Fields: `text`: `String.t()` |
| `image_url` | Image url — Fields: `image_url`: `ImageUrl` |
| `document` | Document — Fields: `document`: `DocumentContent` |
| `input_audio` | Input audio — Fields: `input_audio`: `AudioContent` |


---

#### ImageDetail

| Value | Description |
|-------|-------------|
| `low` | Low |
| `high` | High |
| `auto` | Auto |


---

#### ToolType

The type discriminator for tool/tool-call objects.

Per the OpenAI spec this is always `"function"`. Using an enum enforces
that constraint at the type level and rejects any other value on
deserialization.

| Value | Description |
|-------|-------------|
| `function` | Function |


---

#### ToolChoice

| Value | Description |
|-------|-------------|
| `mode` | Mode — Fields: `0`: `ToolChoiceMode` |
| `specific` | Specific — Fields: `0`: `SpecificToolChoice` |


---

#### ToolChoiceMode

| Value | Description |
|-------|-------------|
| `auto` | Auto |
| `required` | Required |
| `none` | None |


---

#### ResponseFormat

| Value | Description |
|-------|-------------|
| `text` | Text format |
| `json_object` | Json object |
| `json_schema` | Json schema — Fields: `json_schema`: `JsonSchemaFormat` |


---

#### StopSequence

| Value | Description |
|-------|-------------|
| `single` | Single — Fields: `0`: `String.t()` |
| `multiple` | Multiple — Fields: `0`: `list(String.t())` |


---

#### FinishReason

Why a choice stopped generating tokens.

| Value | Description |
|-------|-------------|
| `stop` | Stop |
| `length` | Length |
| `tool_calls` | Tool calls |
| `content_filter` | Content filter |
| `function_call` | Deprecated legacy finish reason; retained for API compatibility. |
| `other` | Catch-all for unknown finish reasons returned by non-OpenAI providers. Note: this intentionally does **not** carry the original string (e.g. `Other(String)`).  Using `#[serde(other)]` requires a unit variant, and switching to `#[serde(untagged)]` would change deserialization semantics for all variants.  The original value can be recovered by inspecting the raw JSON if needed. |


---

#### ReasoningEffort

Controls how much reasoning effort the model should use.

| Value | Description |
|-------|-------------|
| `low` | Low |
| `medium` | Medium |
| `high` | High |


---

#### EmbeddingFormat

The format in which the embedding vectors are returned.

| Value | Description |
|-------|-------------|
| `float` | 32-bit floating-point numbers (default). |
| `base64` | Base64-encoded string representation of the floats. |


---

#### EmbeddingInput

| Value | Description |
|-------|-------------|
| `single` | Single — Fields: `0`: `String.t()` |
| `multiple` | Multiple — Fields: `0`: `list(String.t())` |


---

#### ModerationInput

Input to the moderation endpoint — a single string or multiple strings.

| Value | Description |
|-------|-------------|
| `single` | Single — Fields: `0`: `String.t()` |
| `multiple` | Multiple — Fields: `0`: `list(String.t())` |


---

#### RerankDocument

A document to be reranked — either a plain string or an object with a text field.

| Value | Description |
|-------|-------------|
| `text` | Text format — Fields: `0`: `String.t()` |
| `object` | Object — Fields: `text`: `String.t()` |


---

#### OcrDocument

Document input for OCR — either a URL or inline base64 data.

| Value | Description |
|-------|-------------|
| `url` | A publicly accessible document URL. — Fields: `url`: `String.t()` |
| `base64` | Inline base64-encoded document data. — Fields: `data`: `String.t()`, `media_type`: `String.t()` |


---

#### AuthHeaderFormat

How the API key is sent in the HTTP request.

| Value | Description |
|-------|-------------|
| `bearer` | Bearer token: `Authorization: Bearer <key>` |
| `api_key` | Custom header: e.g., `X-Api-Key: <key>` — Fields: `0`: `String.t()` |
| `none` | No authentication required. |


---

### Errors

#### LiterLlmError

All errors that can occur when using `liter-llm`.

| Variant | Description |
|---------|-------------|
| `authentication` | authentication failed: {message} |
| `rate_limited` | rate limited: {message} |
| `bad_request` | bad request: {message} |
| `context_window_exceeded` | context window exceeded: {message} |
| `content_policy` | content policy violation: {message} |
| `not_found` | not found: {message} |
| `server_error` | server error: {message} |
| `service_unavailable` | service unavailable: {message} |
| `timeout` | request timeout |
| `streaming` | A catch-all for errors that occur during streaming response processing. This variant covers multiple sub-conditions including UTF-8 decoding failures, CRC/checksum mismatches (AWS EventStream), JSON parse errors in individual SSE chunks, and buffer overflow conditions.  The `message` field contains a human-readable description of the specific failure. |
| `endpoint_not_supported` | provider {provider} does not support {endpoint} |
| `invalid_header` | invalid header {name:?}: {reason} |
| `serialization` | serialization error: {0} |
| `budget_exceeded` | budget exceeded: {message} |
| `hook_rejected` | hook rejected: {message} |
| `internal_error` | An internal logic error (e.g. unexpected Tower response variant). This should never surface in normal operation — if it does, it indicates a bug in the library. |


---

