---
title: "C API Reference"
---

## C API Reference <span class="version-badge">v1.4.0-rc.3</span>

### Functions

#### literllm_create_client()

Create a new LLM client with simple scalar configuration.

This is the primary binding entry-point. All parameters except `api_key`
are optional — omitting them uses the same defaults as
`ClientConfigBuilder`.

**Errors:**

Returns `LiterLlmError` if the underlying HTTP client cannot be
constructed, or if the resolved provider configuration is invalid.

**Signature:**

```c
LiterllmDefaultClient* literllm_create_client(const char* api_key, const char* base_url, uint64_t timeout_secs, uint32_t max_retries, const char* model_hint);
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `api_key` | `const char*` | Yes | The api key |
| `base_url` | `const char**` | No | The base url |
| `timeout_secs` | `uint64_t*` | No | The timeout secs |
| `max_retries` | `uint32_t*` | No | The max retries |
| `model_hint` | `const char**` | No | The model hint |

**Returns:** `LiterllmDefaultClient`

**Errors:** Returns `NULL` on error.


---

#### literllm_create_client_from_json()

Create a new LLM client from a JSON string.

The JSON object accepts the same fields as `liter-llm.toml` (snake_case).

**Errors:**

Returns `LiterLlmError.BadRequest` if `json` is not valid JSON or
contains unknown fields.

**Signature:**

```c
LiterllmDefaultClient* literllm_create_client_from_json(const char* json);
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `json` | `const char*` | Yes | The json |

**Returns:** `LiterllmDefaultClient`

**Errors:** Returns `NULL` on error.


---

#### literllm_register_custom_provider()

Register a custom provider in the global runtime registry.

The provider will be checked **before** all built-in providers during model
detection. If a provider with the same `name` already exists it is replaced.

**Errors:**

Returns an error if the config is invalid (empty name, empty base_url, or
no model prefixes).

**Signature:**

```c
void literllm_register_custom_provider(LiterllmCustomProviderConfig config);
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `config` | `LiterllmCustomProviderConfig` | Yes | The configuration options |

**Returns:** `void`

**Errors:** Returns `NULL` on error.


---

#### literllm_unregister_custom_provider()

Remove a previously registered custom provider by name.

Returns `true` if a provider with the given name was found and removed,
`false` if no such provider existed.

**Errors:**

Returns an error only if the internal lock is poisoned.

**Signature:**

```c
bool literllm_unregister_custom_provider(const char* name);
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `name` | `const char*` | Yes | The name |

**Returns:** `bool`

**Errors:** Returns `NULL` on error.


---

### Types

#### LiterllmApiError

Inner error object.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `message` | `const char*` | — | Message |
| `error_type` | `const char*` | — | Error type |
| `param` | `const char**` | `NULL` | Param |
| `code` | `const char**` | `NULL` | Code |


---

#### LiterllmAssistantMessage

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `content` | `const char**` | `NULL` | The extracted text content |
| `name` | `const char**` | `NULL` | The name |
| `tool_calls` | `LiterllmToolCall**` | `NULL` | Tool calls |
| `refusal` | `const char**` | `NULL` | Refusal |
| `function_call` | `LiterllmFunctionCall*` | `NULL` | Deprecated legacy function_call field; retained for API compatibility. |


---

#### LiterllmAudioContent

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `data` | `const char*` | — | Base64-encoded audio data. |
| `format` | `const char*` | — | Audio format (e.g., "wav", "mp3", "ogg"). |


---

#### LiterllmBatchClient

Batch processing operations (create, list, retrieve, cancel).

##### Methods

###### literllm_create_batch()

Create a new batch job.

**Signature:**

```c
LiterllmBatchObject literllm_create_batch(LiterllmCreateBatchRequest req);
```

###### literllm_retrieve_batch()

Retrieve a batch by ID.

**Signature:**

```c
LiterllmBatchObject literllm_retrieve_batch(const char* batch_id);
```

###### literllm_list_batches()

List batches, optionally filtered by query parameters.

**Signature:**

```c
LiterllmBatchListResponse literllm_list_batches(LiterllmBatchListQuery query);
```

###### literllm_cancel_batch()

Cancel an in-progress batch.

**Signature:**

```c
LiterllmBatchObject literllm_cancel_batch(const char* batch_id);
```


---

#### LiterllmChatCompletionChunk

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `id` | `const char*` | — | Unique identifier |
| `object` | `const char*` | — | Always `"chat.completion.chunk"` from OpenAI-compatible APIs.  Stored as a plain `String` so non-standard provider values do not fail parsing. |
| `created` | `uint64_t` | — | Created |
| `model` | `const char*` | — | Model |
| `choices` | `LiterllmStreamChoice*` | `NULL` | Choices |
| `usage` | `LiterllmUsage*` | `NULL` | Usage (usage) |
| `system_fingerprint` | `const char**` | `NULL` | System fingerprint |
| `service_tier` | `const char**` | `NULL` | Service tier |


---

#### LiterllmChatCompletionRequest

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `model` | `const char*` | — | Model |
| `messages` | `LiterllmMessage*` | `NULL` | Messages |
| `temperature` | `double*` | `NULL` | Temperature |
| `top_p` | `double*` | `NULL` | Top p |
| `n` | `uint32_t*` | `NULL` | N |
| `stream` | `bool*` | `NULL` | Whether to stream the response. Managed by the client layer — do not set directly. |
| `stop` | `LiterllmStopSequence*` | `NULL` | Stop (stop sequence) |
| `max_tokens` | `uint64_t*` | `NULL` | Maximum tokens |
| `presence_penalty` | `double*` | `NULL` | Presence penalty |
| `frequency_penalty` | `double*` | `NULL` | Frequency penalty |
| `logit_bias` | `void**` | `NULL` | Token bias map.  Uses `BTreeMap` (sorted keys) for deterministic serialization order — important when hashing or signing requests. |
| `user` | `const char**` | `NULL` | User |
| `tools` | `LiterllmChatCompletionTool**` | `NULL` | Tools |
| `tool_choice` | `LiterllmToolChoice*` | `NULL` | Tool choice (tool choice) |
| `parallel_tool_calls` | `bool*` | `NULL` | Parallel tool calls |
| `response_format` | `LiterllmResponseFormat*` | `NULL` | Response format (response format) |
| `stream_options` | `LiterllmStreamOptions*` | `NULL` | Stream options (stream options) |
| `seed` | `int64_t*` | `NULL` | Seed |
| `reasoning_effort` | `LiterllmReasoningEffort*` | `NULL` | Reasoning effort (reasoning effort) |
| `extra_body` | `void**` | `NULL` | Provider-specific extra parameters merged into the request body. Use for guardrails, safety settings, grounding config, etc. |


---

#### LiterllmChatCompletionResponse

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `id` | `const char*` | — | Unique identifier |
| `object` | `const char*` | — | Always `"chat.completion"` from OpenAI-compatible APIs.  Stored as a plain `String` so non-standard provider values do not break deserialization. |
| `created` | `uint64_t` | — | Created |
| `model` | `const char*` | — | Model |
| `choices` | `LiterllmChoice*` | `NULL` | Choices |
| `usage` | `LiterllmUsage*` | `NULL` | Usage (usage) |
| `system_fingerprint` | `const char**` | `NULL` | System fingerprint |
| `service_tier` | `const char**` | `NULL` | Service tier |

##### Methods

###### literllm_estimated_cost()

Estimate the cost of this response based on embedded pricing data.

Returns `NULL` if:
- the `model` field is not present in the embedded pricing registry, or
- the `usage` field is absent from the response.

**Signature:**

```c
double* literllm_estimated_cost();
```


---

#### LiterllmChatCompletionTool

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `tool_type` | `LiterllmToolType` | — | Tool type (tool type) |
| `function` | `LiterllmFunctionDefinition` | — | Function (function definition) |


---

#### LiterllmChoice

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `index` | `uint32_t` | — | Index |
| `message` | `LiterllmAssistantMessage` | — | Message (assistant message) |
| `finish_reason` | `LiterllmFinishReason*` | `NULL` | Finish reason (finish reason) |


---

#### LiterllmClientConfig

Configuration for an LLM client.

`api_key` is stored as a `SecretString` so it is zeroed on drop and never
printed accidentally.  Access it via `secrecy.ExposeSecret`.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `api_key` | `const char*` | — | API key for authentication (stored as a secret). |
| `base_url` | `const char**` | `NULL` | Override base URL.  When set, all requests go here regardless of model name, and provider auto-detection is skipped. |
| `timeout` | `uint64_t` | — | Request timeout. |
| `max_retries` | `uint32_t` | — | Maximum number of retries on 429 / 5xx responses. |
| `credential_provider` | `LiterllmCredentialProvider*` | `NULL` | Optional dynamic credential provider for token-based auth (Azure AD, Vertex OAuth2) or refreshable credentials (AWS STS). When set, the client calls `resolve()` before each request to obtain a fresh credential.  When `None`, the static `api_key` is used. |
| `load_env` | `bool` | — | Automatically load the API key from the provider's environment variable when no explicit key is provided. When `True` (the default) and `api_key` is empty, `DefaultClient.new` reads the provider's designated environment variable (e.g. `OPENAI_API_KEY` for OpenAI).  Set to `False` to suppress this behaviour and require the caller to supply the key explicitly. Has no effect on WASM targets, where `std.env.var` is unavailable. |

##### Methods

###### literllm_headers()

Return the extra headers as an ordered slice of `(name, value)` pairs.

**Signature:**

```c
void** literllm_headers();
```

###### literllm_fmt()

**Signature:**

```c
LiterllmUnknown literllm_fmt(LiterllmFormatter f);
```


---

#### LiterllmClientConfigBuilder

Builder for `ClientConfig`.

Construct with `ClientConfigBuilder.new` and call builder methods to
customise the configuration, then call `ClientConfigBuilder.build` to
obtain a `ClientConfig`.

##### Methods

###### literllm_from_env()

Create a builder with no explicit API key.

`load_env` is `true` by default, so the key will be read from the
provider's environment variable (e.g. `OPENAI_API_KEY`) at client
construction time.  Call `.load_env(false)` to opt out.

**Signature:**

```c
LiterllmClientConfigBuilder literllm_from_env();
```

###### literllm_load_env()

Enable or disable automatic API key loading from environment variables.

When `true` (the default) and no explicit `api_key` was provided,
`DefaultClient.new` reads the provider's designated environment
variable.  Set to `false` to require an explicit key.

Has no effect on WASM targets.

**Signature:**

```c
LiterllmClientConfigBuilder literllm_load_env(bool enabled);
```

###### literllm_base_url()

Override the provider base URL for all requests.

**Signature:**

```c
LiterllmClientConfigBuilder literllm_base_url(const char* url);
```

###### literllm_timeout()

Set the per-request timeout (default: 60 s).

**Signature:**

```c
LiterllmClientConfigBuilder literllm_timeout(uint64_t timeout);
```

###### literllm_max_retries()

Set the maximum number of retries on 429 / 5xx responses (default: 3).

**Signature:**

```c
LiterllmClientConfigBuilder literllm_max_retries(uint32_t retries);
```

###### literllm_credential_provider()

Set a dynamic credential provider for token-based or refreshable auth.

When configured, the client calls `resolve()` before each request
instead of using the static `api_key` for authentication.

**Signature:**

```c
LiterllmClientConfigBuilder literllm_credential_provider(LiterllmCredentialProvider provider);
```

###### literllm_header()

Add a custom header sent on every request.

Returns an error if either `key` or `value` is not a valid HTTP header
name / value.

This method is only available when the `native-http` feature is enabled
because header validation relies on `reqwest`'s header types.

**Signature:**

```c
LiterllmClientConfigBuilder literllm_header(const char* key, const char* value);
```

###### literllm_cache()

Set the response cache configuration for the Tower middleware stack.

When set, bindings and advanced Rust users can read this from the
built `ClientConfig` to construct a
`CacheLayer`.

**Signature:**

```c
LiterllmClientConfigBuilder literllm_cache(LiterllmCacheConfig config);
```

###### literllm_cache_store()

Set a custom cache store backend for the Tower cache middleware.

When set alongside `cache`, the cache layer will use
this store instead of the default in-memory LRU.

**Signature:**

```c
LiterllmClientConfigBuilder literllm_cache_store(LiterllmCacheStore store);
```

###### literllm_budget()

Set the budget enforcement configuration for the Tower middleware stack.

When set, bindings and advanced Rust users can read this from the
built `ClientConfig` to construct a
`BudgetLayer`.

**Signature:**

```c
LiterllmClientConfigBuilder literllm_budget(LiterllmBudgetConfig config);
```

###### literllm_hook()

Add a single hook to the Tower hooks middleware stack.

Hooks are invoked sequentially in registration order at request
lifecycle points (pre-request, post-response, on-error).

**Signature:**

```c
LiterllmClientConfigBuilder literllm_hook(LiterllmLlmHook hook);
```

###### literllm_hooks()

Set the full list of hooks for the Tower hooks middleware stack,
replacing any previously registered hooks.

Hooks are invoked sequentially in registration order.

**Signature:**

```c
LiterllmClientConfigBuilder literllm_hooks(LiterllmLlmHook* hooks);
```

###### literllm_cooldown()

Set the cooldown duration after transient errors.

When set, the client rejects requests with `ServiceUnavailable` for
the given duration after a transient error (rate limit, timeout,
server error).

**Signature:**

```c
LiterllmClientConfigBuilder literllm_cooldown(uint64_t duration);
```

###### literllm_rate_limit()

Set per-model rate limiting configuration.

When set, requests exceeding the configured RPM or TPM limits are
rejected with `LiterLlmError.RateLimited`.

**Signature:**

```c
LiterllmClientConfigBuilder literllm_rate_limit(LiterllmRateLimitConfig config);
```

###### literllm_health_check()

Set the background health check interval.

When set, the client periodically probes the provider and rejects
requests when the provider is unhealthy.

**Signature:**

```c
LiterllmClientConfigBuilder literllm_health_check(uint64_t interval);
```

###### literllm_cost_tracking()

Enable or disable per-request cost tracking.

When enabled, estimated USD cost is recorded on the current tracing
span as `gen_ai.usage.cost`.

**Signature:**

```c
LiterllmClientConfigBuilder literllm_cost_tracking(bool enabled);
```

###### literllm_tracing()

Enable or disable OpenTelemetry-compatible tracing spans.

When enabled, every request is wrapped in a `gen_ai` tracing span
with semantic convention attributes.

**Signature:**

```c
LiterllmClientConfigBuilder literllm_tracing(bool enabled);
```

###### literllm_build()

Consume the builder and return the completed `ClientConfig`.

**Signature:**

```c
LiterllmClientConfig literllm_build();
```


---

#### LiterllmCreateImageRequest

Request to create images from a text prompt.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `prompt` | `const char*` | — | Prompt |
| `model` | `const char**` | `NULL` | Model |
| `n` | `uint32_t*` | `NULL` | N |
| `size` | `const char**` | `NULL` | Size in bytes |
| `quality` | `const char**` | `NULL` | Quality |
| `style` | `const char**` | `NULL` | Style |
| `response_format` | `const char**` | `NULL` | Response format |
| `user` | `const char**` | `NULL` | User |


---

#### LiterllmCreateSpeechRequest

Request to generate speech audio from text.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `model` | `const char*` | — | Model |
| `input` | `const char*` | — | Input |
| `voice` | `const char*` | — | Voice |
| `response_format` | `const char**` | `NULL` | Response format |
| `speed` | `double*` | `NULL` | Speed |


---

#### LiterllmCreateTranscriptionRequest

Request to transcribe audio into text.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `model` | `const char*` | — | Model |
| `file` | `const char*` | — | Base64-encoded audio file data. |
| `language` | `const char**` | `NULL` | Language |
| `prompt` | `const char**` | `NULL` | Prompt |
| `response_format` | `const char**` | `NULL` | Response format |
| `temperature` | `double*` | `NULL` | Temperature |


---

#### LiterllmCustomProviderConfig

Configuration for registering a custom LLM provider at runtime.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `name` | `const char*` | — | Unique name for this provider (e.g., "my-provider"). |
| `base_url` | `const char*` | — | Base URL for the provider's API (e.g., "<https://api.my-provider.com/v1">). |
| `auth_header` | `LiterllmAuthHeaderFormat` | — | Authentication header format. |
| `model_prefixes` | `const char**` | — | Model name prefixes that route to this provider (e.g., ["my-"]). |


---

#### LiterllmDefaultClient

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

###### literllm_new()

Build a client.

`model_hint` guides provider auto-detection when no explicit
`base_url` override is present in the config.  For example, passing
`Some("groq/llama3-70b")` selects the Groq provider.  Pass `NULL` to
default to OpenAI.

**Errors:**

Returns a wrapped `reqwest.Error` if the underlying HTTP client
cannot be constructed.  Header names and values are pre-validated by
`ClientConfigBuilder.header`, so they are inserted directly here.

**Signature:**

```c
LiterllmDefaultClient literllm_new(LiterllmClientConfig config, const char* model_hint);
```

###### literllm_chat()

**Signature:**

```c
LiterllmChatCompletionResponse literllm_chat(LiterllmChatCompletionRequest req);
```

###### literllm_chat_stream()

**Signature:**

```c
LiterllmBoxStream literllm_chat_stream(LiterllmChatCompletionRequest req);
```

###### literllm_embed()

**Signature:**

```c
LiterllmEmbeddingResponse literllm_embed(LiterllmEmbeddingRequest req);
```

###### literllm_list_models()

**Signature:**

```c
LiterllmModelsListResponse literllm_list_models();
```

###### literllm_image_generate()

**Signature:**

```c
LiterllmImagesResponse literllm_image_generate(LiterllmCreateImageRequest req);
```

###### literllm_speech()

**Signature:**

```c
const uint8_t* literllm_speech(LiterllmCreateSpeechRequest req);
```

###### literllm_transcribe()

**Signature:**

```c
LiterllmTranscriptionResponse literllm_transcribe(LiterllmCreateTranscriptionRequest req);
```

###### literllm_moderate()

**Signature:**

```c
LiterllmModerationResponse literllm_moderate(LiterllmModerationRequest req);
```

###### literllm_rerank()

**Signature:**

```c
LiterllmRerankResponse literllm_rerank(LiterllmRerankRequest req);
```

###### literllm_search()

**Signature:**

```c
LiterllmSearchResponse literllm_search(LiterllmSearchRequest req);
```

###### literllm_ocr()

**Signature:**

```c
LiterllmOcrResponse literllm_ocr(LiterllmOcrRequest req);
```

###### literllm_chat_raw()

**Signature:**

```c
LiterllmRawExchange literllm_chat_raw(LiterllmChatCompletionRequest req);
```

###### literllm_chat_stream_raw()

**Signature:**

```c
LiterllmRawStreamExchange literllm_chat_stream_raw(LiterllmChatCompletionRequest req);
```

###### literllm_embed_raw()

**Signature:**

```c
LiterllmRawExchange literllm_embed_raw(LiterllmEmbeddingRequest req);
```

###### literllm_image_generate_raw()

**Signature:**

```c
LiterllmRawExchange literllm_image_generate_raw(LiterllmCreateImageRequest req);
```

###### literllm_transcribe_raw()

**Signature:**

```c
LiterllmRawExchange literllm_transcribe_raw(LiterllmCreateTranscriptionRequest req);
```

###### literllm_moderate_raw()

**Signature:**

```c
LiterllmRawExchange literllm_moderate_raw(LiterllmModerationRequest req);
```

###### literllm_rerank_raw()

**Signature:**

```c
LiterllmRawExchange literllm_rerank_raw(LiterllmRerankRequest req);
```

###### literllm_search_raw()

**Signature:**

```c
LiterllmRawExchange literllm_search_raw(LiterllmSearchRequest req);
```

###### literllm_ocr_raw()

**Signature:**

```c
LiterllmRawExchange literllm_ocr_raw(LiterllmOcrRequest req);
```

###### literllm_create_file()

**Signature:**

```c
LiterllmFileObject literllm_create_file(LiterllmCreateFileRequest req);
```

###### literllm_retrieve_file()

**Signature:**

```c
LiterllmFileObject literllm_retrieve_file(const char* file_id);
```

###### literllm_delete_file()

**Signature:**

```c
LiterllmDeleteResponse literllm_delete_file(const char* file_id);
```

###### literllm_list_files()

**Signature:**

```c
LiterllmFileListResponse literllm_list_files(LiterllmFileListQuery query);
```

###### literllm_file_content()

**Signature:**

```c
const uint8_t* literllm_file_content(const char* file_id);
```

###### literllm_create_batch()

**Signature:**

```c
LiterllmBatchObject literllm_create_batch(LiterllmCreateBatchRequest req);
```

###### literllm_retrieve_batch()

**Signature:**

```c
LiterllmBatchObject literllm_retrieve_batch(const char* batch_id);
```

###### literllm_list_batches()

**Signature:**

```c
LiterllmBatchListResponse literllm_list_batches(LiterllmBatchListQuery query);
```

###### literllm_cancel_batch()

**Signature:**

```c
LiterllmBatchObject literllm_cancel_batch(const char* batch_id);
```

###### literllm_create_response()

**Signature:**

```c
LiterllmResponseObject literllm_create_response(LiterllmCreateResponseRequest req);
```

###### literllm_retrieve_response()

**Signature:**

```c
LiterllmResponseObject literllm_retrieve_response(const char* id);
```

###### literllm_cancel_response()

**Signature:**

```c
LiterllmResponseObject literllm_cancel_response(const char* id);
```


---

#### LiterllmDeveloperMessage

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `content` | `const char*` | — | The extracted text content |
| `name` | `const char**` | `NULL` | The name |


---

#### LiterllmDocumentContent

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `data` | `const char*` | — | Base64-encoded document data or URL. |
| `media_type` | `const char*` | — | MIME type (e.g., "application/pdf", "text/csv"). |


---

#### LiterllmEmbeddingObject

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `object` | `const char*` | — | Always `"embedding"` from OpenAI-compatible APIs.  Stored as a plain `String` so non-standard provider values do not break deserialization. |
| `embedding` | `double*` | — | Embedding |
| `index` | `uint32_t` | — | Index |


---

#### LiterllmEmbeddingRequest

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `model` | `const char*` | — | Model |
| `input` | `LiterllmEmbeddingInput` | — | Input (embedding input) |
| `encoding_format` | `LiterllmEmbeddingFormat*` | `NULL` | Encoding format (embedding format) |
| `dimensions` | `uint32_t*` | `NULL` | Dimensions |
| `user` | `const char**` | `NULL` | User |


---

#### LiterllmEmbeddingResponse

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `object` | `const char*` | — | Always `"list"` from OpenAI-compatible APIs.  Stored as a plain `String` so non-standard provider values do not break deserialization. |
| `data` | `LiterllmEmbeddingObject*` | — | Data |
| `model` | `const char*` | — | Model |
| `usage` | `LiterllmUsage*` | `NULL` | Usage (usage) |

##### Methods

###### literllm_estimated_cost()

Estimate the cost of this embedding request based on embedded pricing data.

Returns `NULL` if:
- the `model` field is not present in the embedded pricing registry, or
- the `usage` field is absent from the response.

Embedding models only charge for input tokens; output cost is zero.

**Signature:**

```c
double* literllm_estimated_cost();
```


---

#### LiterllmErrorResponse

Error response from an OpenAI-compatible API.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `error` | `LiterllmApiError` | — | Error (api error) |


---

#### LiterllmFileBudgetConfig

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `global_limit` | `double*` | `NULL` | Global limit |
| `model_limits` | `void**` | `NULL` | Model limits |
| `enforcement` | `const char**` | `NULL` | Enforcement |


---

#### LiterllmFileCacheConfig

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `max_entries` | `uintptr_t*` | `NULL` | Maximum entries |
| `ttl_seconds` | `uint64_t*` | `NULL` | Ttl seconds |
| `backend` | `const char**` | `NULL` | Backend |
| `backend_config` | `void**` | `NULL` | Backend config |


---

#### LiterllmFileClient

File management operations (upload, list, retrieve, delete).

##### Methods

###### literllm_create_file()

Upload a file.

**Signature:**

```c
LiterllmFileObject literllm_create_file(LiterllmCreateFileRequest req);
```

###### literllm_retrieve_file()

Retrieve metadata for a file.

**Signature:**

```c
LiterllmFileObject literllm_retrieve_file(const char* file_id);
```

###### literllm_delete_file()

Delete a file.

**Signature:**

```c
LiterllmDeleteResponse literllm_delete_file(const char* file_id);
```

###### literllm_list_files()

List files, optionally filtered by query parameters.

**Signature:**

```c
LiterllmFileListResponse literllm_list_files(LiterllmFileListQuery query);
```

###### literllm_file_content()

Retrieve the raw content of a file.

**Signature:**

```c
const uint8_t* literllm_file_content(const char* file_id);
```


---

#### LiterllmFileConfig

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
| `api_key` | `const char**` | `NULL` | Api key |
| `base_url` | `const char**` | `NULL` | Base url |
| `model_hint` | `const char**` | `NULL` | Model hint |
| `timeout_secs` | `uint64_t*` | `NULL` | Timeout secs |
| `max_retries` | `uint32_t*` | `NULL` | Maximum retries |
| `extra_headers` | `void**` | `NULL` | Extra headers |
| `cache` | `LiterllmFileCacheConfig*` | `NULL` | Cache (file cache config) |
| `budget` | `LiterllmFileBudgetConfig*` | `NULL` | Budget (file budget config) |
| `cooldown_secs` | `uint64_t*` | `NULL` | Cooldown secs |
| `rate_limit` | `LiterllmFileRateLimitConfig*` | `NULL` | Rate limit (file rate limit config) |
| `health_check_secs` | `uint64_t*` | `NULL` | Health check secs |
| `cost_tracking` | `bool*` | `NULL` | Cost tracking |
| `tracing` | `bool*` | `NULL` | Tracing |
| `providers` | `LiterllmFileProviderConfig**` | `NULL` | Providers |

##### Methods

###### literllm_from_toml_file()

Load from a TOML file path.

**Signature:**

```c
LiterllmFileConfig literllm_from_toml_file(LiterllmPath path);
```

###### literllm_from_toml_str()

Parse from a TOML string.

**Signature:**

```c
LiterllmFileConfig literllm_from_toml_str(const char* s);
```

###### literllm_discover()

Discover `liter-llm.toml` by walking from current directory to filesystem root.

Returns `Ok(None)` if no config file is found.

**Signature:**

```c
LiterllmFileConfig* literllm_discover();
```

###### literllm_into_builder()

Convert into a `ClientConfigBuilder`,
applying all fields that are set.

Fields not present in the TOML file use the builder's defaults.

**Signature:**

```c
LiterllmClientConfigBuilder literllm_into_builder();
```

###### literllm_providers()

Get the custom provider configurations from this file config.

**Signature:**

```c
LiterllmFileProviderConfig* literllm_providers();
```


---

#### LiterllmFileProviderConfig

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `name` | `const char*` | — | The name |
| `base_url` | `const char*` | — | Base url |
| `auth_header` | `const char**` | `NULL` | Auth header |
| `model_prefixes` | `const char**` | — | Model prefixes |


---

#### LiterllmFileRateLimitConfig

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `rpm` | `uint32_t*` | `NULL` | Rpm |
| `tpm` | `uint64_t*` | `NULL` | Tpm |
| `window_seconds` | `uint64_t*` | `NULL` | Window seconds |


---

#### LiterllmFunctionCall

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `name` | `const char*` | — | The name |
| `arguments` | `const char*` | — | Arguments |


---

#### LiterllmFunctionDefinition

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `name` | `const char*` | — | The name |
| `description` | `const char**` | `NULL` | Human-readable description |
| `parameters` | `void**` | `NULL` | Parameters |
| `strict` | `bool*` | `NULL` | Strict |


---

#### LiterllmFunctionMessage

Deprecated legacy function-role message body.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `content` | `const char*` | — | The extracted text content |
| `name` | `const char*` | — | The name |


---

#### LiterllmImage

A single generated image, returned as either a URL or base64 data.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `url` | `const char**` | `NULL` | Url |
| `b64_json` | `const char**` | `NULL` | B64 json |
| `revised_prompt` | `const char**` | `NULL` | Revised prompt |


---

#### LiterllmImageUrl

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `url` | `const char*` | — | Url |
| `detail` | `LiterllmImageDetail*` | `NULL` | Detail (image detail) |


---

#### LiterllmImagesResponse

Response containing generated images.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `created` | `uint64_t` | — | Created |
| `data` | `LiterllmImage*` | `NULL` | Data |


---

#### LiterllmJsonSchemaFormat

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `name` | `const char*` | — | The name |
| `description` | `const char**` | `NULL` | Human-readable description |
| `schema` | `void*` | — | Schema |
| `strict` | `bool*` | `NULL` | Strict |


---

#### LiterllmLiterLlmError

##### Methods

###### literllm_is_transient()

Returns `true` for errors that are worth retrying on a different service
or deployment (transient failures).

Used by `crate.tower.fallback.FallbackService` and
`crate.tower.router.Router` to decide whether to route to an
alternative endpoint.

**Signature:**

```c
bool literllm_is_transient();
```

###### literllm_error_type()

Return the OpenTelemetry `error.type` string for this error variant.

Used by the tracing middleware to record the `error.type` span attribute
on failed requests per the GenAI semantic conventions.

**Signature:**

```c
const char* literllm_error_type();
```

###### literllm_from_status()

Create from an HTTP status code, an API error response body, and an
optional `Retry-After` duration already parsed from the response header.

The `retry_after` value is forwarded into `LiterLlmError.RateLimited`
so callers can honour the server-requested delay without re-parsing the
header.

**Signature:**

```c
LiterllmLiterLlmError literllm_from_status(uint16_t status, const char* body, uint64_t retry_after);
```


---

#### LiterllmLlmClient

Core LLM client trait.

##### Methods

###### literllm_chat()

Send a chat completion request.

**Signature:**

```c
LiterllmChatCompletionResponse literllm_chat(LiterllmChatCompletionRequest req);
```

###### literllm_chat_stream()

Send a streaming chat completion request.

**Signature:**

```c
LiterllmBoxStream literllm_chat_stream(LiterllmChatCompletionRequest req);
```

###### literllm_embed()

Send an embedding request.

**Signature:**

```c
LiterllmEmbeddingResponse literllm_embed(LiterllmEmbeddingRequest req);
```

###### literllm_list_models()

List available models.

**Signature:**

```c
LiterllmModelsListResponse literllm_list_models();
```

###### literllm_image_generate()

Generate an image.

**Signature:**

```c
LiterllmImagesResponse literllm_image_generate(LiterllmCreateImageRequest req);
```

###### literllm_speech()

Generate speech audio from text.

**Signature:**

```c
const uint8_t* literllm_speech(LiterllmCreateSpeechRequest req);
```

###### literllm_transcribe()

Transcribe audio to text.

**Signature:**

```c
LiterllmTranscriptionResponse literllm_transcribe(LiterllmCreateTranscriptionRequest req);
```

###### literllm_moderate()

Check content against moderation policies.

**Signature:**

```c
LiterllmModerationResponse literllm_moderate(LiterllmModerationRequest req);
```

###### literllm_rerank()

Rerank documents by relevance to a query.

**Signature:**

```c
LiterllmRerankResponse literllm_rerank(LiterllmRerankRequest req);
```

###### literllm_search()

Perform a web/document search.

**Signature:**

```c
LiterllmSearchResponse literllm_search(LiterllmSearchRequest req);
```

###### literllm_ocr()

Extract text from a document via OCR.

**Signature:**

```c
LiterllmOcrResponse literllm_ocr(LiterllmOcrRequest req);
```


---

#### LiterllmLlmClientRaw

Extension of `LlmClient` that returns raw request/response data
alongside the typed response.

Every `_raw` method mirrors its counterpart on `LlmClient` but wraps the
result in a `RawExchange` that exposes the final request body (after
`transform_request`) and the raw provider response (before
`transform_response`). This is useful for debugging provider-specific
transformations, capturing wire-level data, or implementing custom parsing.

##### Methods

###### literllm_chat_raw()

Send a chat completion request and return the raw exchange.

The `raw_request` field contains the final JSON body sent to the
provider; `raw_response` contains the provider JSON before
normalization.

**Signature:**

```c
LiterllmRawExchange literllm_chat_raw(LiterllmChatCompletionRequest req);
```

###### literllm_chat_stream_raw()

Send a streaming chat completion request and return the raw exchange.

Only `raw_request` is available upfront — the stream itself is
returned in `stream` and consumed incrementally.

**Signature:**

```c
LiterllmRawStreamExchange literllm_chat_stream_raw(LiterllmChatCompletionRequest req);
```

###### literllm_embed_raw()

Send an embedding request and return the raw exchange.

**Signature:**

```c
LiterllmRawExchange literllm_embed_raw(LiterllmEmbeddingRequest req);
```

###### literllm_image_generate_raw()

Generate an image and return the raw exchange.

**Signature:**

```c
LiterllmRawExchange literllm_image_generate_raw(LiterllmCreateImageRequest req);
```

###### literllm_transcribe_raw()

Transcribe audio to text and return the raw exchange.

**Signature:**

```c
LiterllmRawExchange literllm_transcribe_raw(LiterllmCreateTranscriptionRequest req);
```

###### literllm_moderate_raw()

Check content against moderation policies and return the raw exchange.

**Signature:**

```c
LiterllmRawExchange literllm_moderate_raw(LiterllmModerationRequest req);
```

###### literllm_rerank_raw()

Rerank documents by relevance to a query and return the raw exchange.

**Signature:**

```c
LiterllmRawExchange literllm_rerank_raw(LiterllmRerankRequest req);
```

###### literllm_search_raw()

Perform a web/document search and return the raw exchange.

**Signature:**

```c
LiterllmRawExchange literllm_search_raw(LiterllmSearchRequest req);
```

###### literllm_ocr_raw()

Extract text from a document via OCR and return the raw exchange.

**Signature:**

```c
LiterllmRawExchange literllm_ocr_raw(LiterllmOcrRequest req);
```


---

#### LiterllmManagedClient

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

###### literllm_new()

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

```c
LiterllmManagedClient literllm_new(LiterllmClientConfig config, const char* model_hint);
```

###### literllm_inner()

Return a reference to the underlying `DefaultClient`.

**Signature:**

```c
LiterllmDefaultClient literllm_inner();
```

###### literllm_budget_state()

Return the budget state handle, if budget middleware is configured.

Use this to query accumulated spend at runtime.

**Signature:**

```c
LiterllmBudgetState* literllm_budget_state();
```

###### literllm_has_middleware()

Return `true` when middleware is active (requests go through the Tower
service stack).

**Signature:**

```c
bool literllm_has_middleware();
```

###### literllm_chat()

**Signature:**

```c
LiterllmChatCompletionResponse literllm_chat(LiterllmChatCompletionRequest req);
```

###### literllm_chat_stream()

**Signature:**

```c
LiterllmBoxStream literllm_chat_stream(LiterllmChatCompletionRequest req);
```

###### literllm_embed()

**Signature:**

```c
LiterllmEmbeddingResponse literllm_embed(LiterllmEmbeddingRequest req);
```

###### literllm_list_models()

**Signature:**

```c
LiterllmModelsListResponse literllm_list_models();
```

###### literllm_image_generate()

**Signature:**

```c
LiterllmImagesResponse literllm_image_generate(LiterllmCreateImageRequest req);
```

###### literllm_speech()

**Signature:**

```c
const uint8_t* literllm_speech(LiterllmCreateSpeechRequest req);
```

###### literllm_transcribe()

**Signature:**

```c
LiterllmTranscriptionResponse literllm_transcribe(LiterllmCreateTranscriptionRequest req);
```

###### literllm_moderate()

**Signature:**

```c
LiterllmModerationResponse literllm_moderate(LiterllmModerationRequest req);
```

###### literllm_rerank()

**Signature:**

```c
LiterllmRerankResponse literllm_rerank(LiterllmRerankRequest req);
```

###### literllm_search()

**Signature:**

```c
LiterllmSearchResponse literllm_search(LiterllmSearchRequest req);
```

###### literllm_ocr()

**Signature:**

```c
LiterllmOcrResponse literllm_ocr(LiterllmOcrRequest req);
```

###### literllm_create_file()

**Signature:**

```c
LiterllmFileObject literllm_create_file(LiterllmCreateFileRequest req);
```

###### literllm_retrieve_file()

**Signature:**

```c
LiterllmFileObject literllm_retrieve_file(const char* file_id);
```

###### literllm_delete_file()

**Signature:**

```c
LiterllmDeleteResponse literllm_delete_file(const char* file_id);
```

###### literllm_list_files()

**Signature:**

```c
LiterllmFileListResponse literllm_list_files(LiterllmFileListQuery query);
```

###### literllm_file_content()

**Signature:**

```c
const uint8_t* literllm_file_content(const char* file_id);
```

###### literllm_create_batch()

**Signature:**

```c
LiterllmBatchObject literllm_create_batch(LiterllmCreateBatchRequest req);
```

###### literllm_retrieve_batch()

**Signature:**

```c
LiterllmBatchObject literllm_retrieve_batch(const char* batch_id);
```

###### literllm_list_batches()

**Signature:**

```c
LiterllmBatchListResponse literllm_list_batches(LiterllmBatchListQuery query);
```

###### literllm_cancel_batch()

**Signature:**

```c
LiterllmBatchObject literllm_cancel_batch(const char* batch_id);
```

###### literllm_create_response()

**Signature:**

```c
LiterllmResponseObject literllm_create_response(LiterllmCreateResponseRequest req);
```

###### literllm_retrieve_response()

**Signature:**

```c
LiterllmResponseObject literllm_retrieve_response(const char* id);
```

###### literllm_cancel_response()

**Signature:**

```c
LiterllmResponseObject literllm_cancel_response(const char* id);
```


---

#### LiterllmModelObject

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `id` | `const char*` | — | Unique identifier |
| `object` | `const char*` | — | Always `"model"` from OpenAI-compatible APIs.  Stored as a plain `String` so non-standard provider values do not break deserialization. |
| `created` | `uint64_t` | — | Created |
| `owned_by` | `const char*` | — | Owned by |


---

#### LiterllmModelsListResponse

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `object` | `const char*` | — | Always `"list"` from OpenAI-compatible APIs.  Stored as a plain `String` so non-standard provider values do not break deserialization. |
| `data` | `LiterllmModelObject*` | `NULL` | Data |


---

#### LiterllmModerationCategories

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

#### LiterllmModerationCategoryScores

Confidence scores for each moderation category.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `sexual` | `double` | — | Sexual |
| `hate` | `double` | — | Hate |
| `harassment` | `double` | — | Harassment |
| `self_harm` | `double` | — | Self harm |
| `sexual_minors` | `double` | — | Sexual minors |
| `hate_threatening` | `double` | — | Hate threatening |
| `violence_graphic` | `double` | — | Violence graphic |
| `self_harm_intent` | `double` | — | Self harm intent |
| `self_harm_instructions` | `double` | — | Self harm instructions |
| `harassment_threatening` | `double` | — | Harassment threatening |
| `violence` | `double` | — | Violence |


---

#### LiterllmModerationRequest

Request to classify content for policy violations.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `input` | `LiterllmModerationInput` | — | Input (moderation input) |
| `model` | `const char**` | `NULL` | Model |


---

#### LiterllmModerationResponse

Response from the moderation endpoint.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `id` | `const char*` | — | Unique identifier |
| `model` | `const char*` | — | Model |
| `results` | `LiterllmModerationResult*` | — | Results |


---

#### LiterllmModerationResult

A single moderation classification result.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `flagged` | `bool` | — | Flagged |
| `categories` | `LiterllmModerationCategories` | — | Categories (moderation categories) |
| `category_scores` | `LiterllmModerationCategoryScores` | — | Category scores (moderation category scores) |


---

#### LiterllmOcrImage

An image extracted from an OCR page.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `id` | `const char*` | — | Unique image identifier. |
| `image_base64` | `const char**` | `NULL` | Base64-encoded image data. |


---

#### LiterllmOcrPage

A single page of OCR output.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `index` | `uint32_t` | — | Page index (0-based). |
| `markdown` | `const char*` | — | Extracted content as Markdown. |
| `images` | `LiterllmOcrImage**` | `NULL` | Extracted images, if `include_image_base64` was set. |
| `dimensions` | `LiterllmPageDimensions*` | `NULL` | Page dimensions in pixels, if available. |


---

#### LiterllmOcrRequest

An OCR request.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `model` | `const char*` | — | The model/provider to use (e.g. `"mistral/mistral-ocr-latest"`). |
| `document` | `LiterllmOcrDocument` | — | The document to process. |
| `pages` | `uint32_t**` | `NULL` | Specific pages to process (1-indexed). `None` means all pages. |
| `include_image_base64` | `bool*` | `NULL` | Whether to include base64-encoded images of each page. |


---

#### LiterllmOcrResponse

An OCR response.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `pages` | `LiterllmOcrPage*` | — | Extracted pages. |
| `model` | `const char*` | — | The model used. |
| `usage` | `LiterllmUsage*` | `NULL` | Token usage, if reported by the provider. |


---

#### LiterllmPageDimensions

Page dimensions in pixels.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `width` | `uint32_t` | — | Width in pixels. |
| `height` | `uint32_t` | — | Height in pixels. |


---

#### LiterllmRerankRequest

Request to rerank documents by relevance to a query.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `model` | `const char*` | — | Model |
| `query` | `const char*` | — | Query |
| `documents` | `LiterllmRerankDocument*` | — | Documents |
| `top_n` | `uint32_t*` | `NULL` | Top n |
| `return_documents` | `bool*` | `NULL` | Return documents |


---

#### LiterllmRerankResponse

Response from the rerank endpoint.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `id` | `const char**` | `NULL` | Unique identifier |
| `results` | `LiterllmRerankResult*` | — | Results |
| `meta` | `void**` | `NULL` | Meta |


---

#### LiterllmRerankResult

A single reranked document with its relevance score.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `index` | `uint32_t` | — | Index |
| `relevance_score` | `double` | — | Relevance score |
| `document` | `LiterllmRerankResultDocument*` | `NULL` | Document (rerank result document) |


---

#### LiterllmRerankResultDocument

The text content of a reranked document, returned when `return_documents` is true.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `text` | `const char*` | — | Text |


---

#### LiterllmResponseClient

Responses API operations (create, retrieve, cancel).

##### Methods

###### literllm_create_response()

Create a new response.

**Signature:**

```c
LiterllmResponseObject literllm_create_response(LiterllmCreateResponseRequest req);
```

###### literllm_retrieve_response()

Retrieve a response by ID.

**Signature:**

```c
LiterllmResponseObject literllm_retrieve_response(const char* id);
```

###### literllm_cancel_response()

Cancel an in-progress response.

**Signature:**

```c
LiterllmResponseObject literllm_cancel_response(const char* id);
```


---

#### LiterllmSearchRequest

A search request.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `model` | `const char*` | — | The model/provider to use (e.g. `"brave/web-search"`, `"tavily/search"`). |
| `query` | `const char*` | — | The search query. |
| `max_results` | `uint32_t*` | `NULL` | Maximum number of results to return. |
| `search_domain_filter` | `const char***` | `NULL` | Domain filter — restrict results to specific domains. |
| `country` | `const char**` | `NULL` | Country code for localized results (ISO 3166-1 alpha-2). |


---

#### LiterllmSearchResponse

A search response.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `results` | `LiterllmSearchResult*` | — | The search results. |
| `model` | `const char*` | — | The model used. |


---

#### LiterllmSearchResult

An individual search result.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `title` | `const char*` | — | Title of the result. |
| `url` | `const char*` | — | URL of the result. |
| `snippet` | `const char*` | — | Text snippet / excerpt. |
| `date` | `const char**` | `NULL` | Publication or last-updated date, if available. |


---

#### LiterllmSpecificFunction

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `name` | `const char*` | — | The name |


---

#### LiterllmSpecificToolChoice

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `choice_type` | `LiterllmToolType` | `LITERLLM_LITERLLM_FUNCTION` | Choice type (tool type) |
| `function` | `LiterllmSpecificFunction` | — | Function (specific function) |


---

#### LiterllmStreamChoice

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `index` | `uint32_t` | — | Index |
| `delta` | `LiterllmStreamDelta` | — | Delta (stream delta) |
| `finish_reason` | `LiterllmFinishReason*` | `NULL` | Finish reason (finish reason) |


---

#### LiterllmStreamDelta

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `role` | `const char**` | `NULL` | Role |
| `content` | `const char**` | `NULL` | The extracted text content |
| `tool_calls` | `LiterllmStreamToolCall**` | `NULL` | Tool calls |
| `function_call` | `LiterllmStreamFunctionCall*` | `NULL` | Deprecated legacy function_call delta; retained for API compatibility. |
| `refusal` | `const char**` | `NULL` | Refusal |


---

#### LiterllmStreamFunctionCall

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `name` | `const char**` | `NULL` | The name |
| `arguments` | `const char**` | `NULL` | Arguments |


---

#### LiterllmStreamOptions

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `include_usage` | `bool*` | `NULL` | Include usage |


---

#### LiterllmStreamToolCall

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `index` | `uint32_t` | — | Index |
| `id` | `const char**` | `NULL` | Unique identifier |
| `call_type` | `LiterllmToolType*` | `NULL` | Call type (tool type) |
| `function` | `LiterllmStreamFunctionCall*` | `NULL` | Function (stream function call) |


---

#### LiterllmSystemMessage

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `content` | `const char*` | — | The extracted text content |
| `name` | `const char**` | `NULL` | The name |


---

#### LiterllmToolCall

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `id` | `const char*` | — | Unique identifier |
| `call_type` | `LiterllmToolType` | — | Call type (tool type) |
| `function` | `LiterllmFunctionCall` | — | Function (function call) |


---

#### LiterllmToolMessage

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `content` | `const char*` | — | The extracted text content |
| `tool_call_id` | `const char*` | — | Tool call id |
| `name` | `const char**` | `NULL` | The name |


---

#### LiterllmTranscriptionResponse

Response from a transcription request.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `text` | `const char*` | — | Text |
| `language` | `const char**` | `NULL` | Language |
| `duration` | `double*` | `NULL` | Duration |
| `segments` | `LiterllmTranscriptionSegment**` | `NULL` | Segments |


---

#### LiterllmTranscriptionSegment

A segment of transcribed audio with timing information.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `id` | `uint32_t` | — | Unique identifier |
| `start` | `double` | — | Start |
| `end` | `double` | — | End |
| `text` | `const char*` | — | Text |


---

#### LiterllmUsage

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `prompt_tokens` | `uint64_t` | — | Prompt tokens used. Defaults to 0 when absent (some providers omit this). |
| `completion_tokens` | `uint64_t` | — | Completion tokens used. Defaults to 0 when absent (e.g. embedding responses). |
| `total_tokens` | `uint64_t` | — | Total tokens used. Defaults to 0 when absent (some providers omit this). |


---

#### LiterllmUserMessage

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `content` | `LiterllmUserContent` | `LITERLLM_LITERLLM_TEXT` | The extracted text content |
| `name` | `const char**` | `NULL` | The name |


---

### Enums

#### LiterllmMessage

A chat message in a conversation.

| Value | Description |
|-------|-------------|
| `LITERLLM_SYSTEM` | System — Fields: `0`: `LiterllmSystemMessage` |
| `LITERLLM_USER` | User — Fields: `0`: `LiterllmUserMessage` |
| `LITERLLM_ASSISTANT` | Assistant — Fields: `0`: `LiterllmAssistantMessage` |
| `LITERLLM_TOOL` | Tool — Fields: `0`: `LiterllmToolMessage` |
| `LITERLLM_DEVELOPER` | Developer — Fields: `0`: `LiterllmDeveloperMessage` |
| `LITERLLM_FUNCTION` | Deprecated legacy function-role message; retained for API compatibility. — Fields: `0`: `LiterllmFunctionMessage` |


---

#### LiterllmUserContent

| Value | Description |
|-------|-------------|
| `LITERLLM_TEXT` | Text format — Fields: `0`: `const char*` |
| `LITERLLM_PARTS` | Parts — Fields: `0`: `LiterllmContentPart*` |


---

#### LiterllmContentPart

| Value | Description |
|-------|-------------|
| `LITERLLM_TEXT` | Text format — Fields: `text`: `const char*` |
| `LITERLLM_IMAGE_URL` | Image url — Fields: `image_url`: `LiterllmImageUrl` |
| `LITERLLM_DOCUMENT` | Document — Fields: `document`: `LiterllmDocumentContent` |
| `LITERLLM_INPUT_AUDIO` | Input audio — Fields: `input_audio`: `LiterllmAudioContent` |


---

#### LiterllmImageDetail

| Value | Description |
|-------|-------------|
| `LITERLLM_LOW` | Low |
| `LITERLLM_HIGH` | High |
| `LITERLLM_AUTO` | Auto |


---

#### LiterllmToolType

The type discriminator for tool/tool-call objects.

Per the OpenAI spec this is always `"function"`. Using an enum enforces
that constraint at the type level and rejects any other value on
deserialization.

| Value | Description |
|-------|-------------|
| `LITERLLM_FUNCTION` | Function |


---

#### LiterllmToolChoice

| Value | Description |
|-------|-------------|
| `LITERLLM_MODE` | Mode — Fields: `0`: `LiterllmToolChoiceMode` |
| `LITERLLM_SPECIFIC` | Specific — Fields: `0`: `LiterllmSpecificToolChoice` |


---

#### LiterllmToolChoiceMode

| Value | Description |
|-------|-------------|
| `LITERLLM_AUTO` | Auto |
| `LITERLLM_REQUIRED` | Required |
| `LITERLLM_NONE` | None |


---

#### LiterllmResponseFormat

| Value | Description |
|-------|-------------|
| `LITERLLM_TEXT` | Text format |
| `LITERLLM_JSON_OBJECT` | Json object |
| `LITERLLM_JSON_SCHEMA` | Json schema — Fields: `json_schema`: `LiterllmJsonSchemaFormat` |


---

#### LiterllmStopSequence

| Value | Description |
|-------|-------------|
| `LITERLLM_SINGLE` | Single — Fields: `0`: `const char*` |
| `LITERLLM_MULTIPLE` | Multiple — Fields: `0`: `const char**` |


---

#### LiterllmFinishReason

Why a choice stopped generating tokens.

| Value | Description |
|-------|-------------|
| `LITERLLM_STOP` | Stop |
| `LITERLLM_LENGTH` | Length |
| `LITERLLM_TOOL_CALLS` | Tool calls |
| `LITERLLM_CONTENT_FILTER` | Content filter |
| `LITERLLM_FUNCTION_CALL` | Deprecated legacy finish reason; retained for API compatibility. |
| `LITERLLM_OTHER` | Catch-all for unknown finish reasons returned by non-OpenAI providers. Note: this intentionally does **not** carry the original string (e.g. `Other(String)`).  Using `#[serde(other)]` requires a unit variant, and switching to `#[serde(untagged)]` would change deserialization semantics for all variants.  The original value can be recovered by inspecting the raw JSON if needed. |


---

#### LiterllmReasoningEffort

Controls how much reasoning effort the model should use.

| Value | Description |
|-------|-------------|
| `LITERLLM_LOW` | Low |
| `LITERLLM_MEDIUM` | Medium |
| `LITERLLM_HIGH` | High |


---

#### LiterllmEmbeddingFormat

The format in which the embedding vectors are returned.

| Value | Description |
|-------|-------------|
| `LITERLLM_FLOAT` | 32-bit floating-point numbers (default). |
| `LITERLLM_BASE64` | Base64-encoded string representation of the floats. |


---

#### LiterllmEmbeddingInput

| Value | Description |
|-------|-------------|
| `LITERLLM_SINGLE` | Single — Fields: `0`: `const char*` |
| `LITERLLM_MULTIPLE` | Multiple — Fields: `0`: `const char**` |


---

#### LiterllmModerationInput

Input to the moderation endpoint — a single string or multiple strings.

| Value | Description |
|-------|-------------|
| `LITERLLM_SINGLE` | Single — Fields: `0`: `const char*` |
| `LITERLLM_MULTIPLE` | Multiple — Fields: `0`: `const char**` |


---

#### LiterllmRerankDocument

A document to be reranked — either a plain string or an object with a text field.

| Value | Description |
|-------|-------------|
| `LITERLLM_TEXT` | Text format — Fields: `0`: `const char*` |
| `LITERLLM_OBJECT` | Object — Fields: `text`: `const char*` |


---

#### LiterllmOcrDocument

Document input for OCR — either a URL or inline base64 data.

| Value | Description |
|-------|-------------|
| `LITERLLM_URL` | A publicly accessible document URL. — Fields: `url`: `const char*` |
| `LITERLLM_BASE64` | Inline base64-encoded document data. — Fields: `data`: `const char*`, `media_type`: `const char*` |


---

#### LiterllmAuthHeaderFormat

How the API key is sent in the HTTP request.

| Value | Description |
|-------|-------------|
| `LITERLLM_BEARER` | Bearer token: `Authorization: Bearer <key>` |
| `LITERLLM_API_KEY` | Custom header: e.g., `X-Api-Key: <key>` — Fields: `0`: `const char*` |
| `LITERLLM_NONE` | No authentication required. |


---

### Errors

#### LiterllmLiterLlmError

All errors that can occur when using `liter-llm`.

| Variant | Description |
|---------|-------------|
| `LITERLLM_AUTHENTICATION` | authentication failed: {message} |
| `LITERLLM_RATE_LIMITED` | rate limited: {message} |
| `LITERLLM_BAD_REQUEST` | bad request: {message} |
| `LITERLLM_CONTEXT_WINDOW_EXCEEDED` | context window exceeded: {message} |
| `LITERLLM_CONTENT_POLICY` | content policy violation: {message} |
| `LITERLLM_NOT_FOUND` | not found: {message} |
| `LITERLLM_SERVER_ERROR` | server error: {message} |
| `LITERLLM_SERVICE_UNAVAILABLE` | service unavailable: {message} |
| `LITERLLM_TIMEOUT` | request timeout |
| `LITERLLM_STREAMING` | A catch-all for errors that occur during streaming response processing. This variant covers multiple sub-conditions including UTF-8 decoding failures, CRC/checksum mismatches (AWS EventStream), JSON parse errors in individual SSE chunks, and buffer overflow conditions.  The `message` field contains a human-readable description of the specific failure. |
| `LITERLLM_ENDPOINT_NOT_SUPPORTED` | provider {provider} does not support {endpoint} |
| `LITERLLM_INVALID_HEADER` | invalid header {name:?}: {reason} |
| `LITERLLM_SERIALIZATION` | serialization error: {0} |
| `LITERLLM_BUDGET_EXCEEDED` | budget exceeded: {message} |
| `LITERLLM_HOOK_REJECTED` | hook rejected: {message} |
| `LITERLLM_INTERNAL_ERROR` | An internal logic error (e.g. unexpected Tower response variant). This should never surface in normal operation — if it does, it indicates a bug in the library. |


---

