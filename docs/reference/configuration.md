---
title: "Configuration Reference"
---

## Configuration Reference

This page documents all configuration types and their defaults across all languages.

### SystemMessage

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `content` | `str` | — | The extracted text content |
| `name` | `str | None` | `None` | The name |

---

### UserMessage

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `content` | `UserContent` | `UserContent.TEXT` | The extracted text content |
| `name` | `str | None` | `None` | The name |

---

### ImageUrl

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `url` | `str` | — | Url |
| `detail` | `ImageDetail | None` | `None` | Detail (image detail) |

---

### DocumentContent

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `data` | `str` | — | Base64-encoded document data or URL. |
| `media_type` | `str` | — | MIME type (e.g., "application/pdf", "text/csv"). |

---

### AudioContent

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `data` | `str` | — | Base64-encoded audio data. |
| `format` | `str` | — | Audio format (e.g., "wav", "mp3", "ogg"). |

---

### AssistantMessage

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `content` | `str | None` | `None` | The extracted text content |
| `name` | `str | None` | `None` | The name |
| `tool_calls` | `list[ToolCall] | None` | `[]` | Tool calls |
| `refusal` | `str | None` | `None` | Refusal |
| `function_call` | `FunctionCall | None` | `None` | Deprecated legacy function_call field; retained for API compatibility. |

---

### ToolMessage

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `content` | `str` | — | The extracted text content |
| `tool_call_id` | `str` | — | Tool call id |
| `name` | `str | None` | `None` | The name |

---

### DeveloperMessage

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `content` | `str` | — | The extracted text content |
| `name` | `str | None` | `None` | The name |

---

### FunctionMessage

Deprecated legacy function-role message body.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `content` | `str` | — | The extracted text content |
| `name` | `str` | — | The name |

---

### SpecificToolChoice

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `choice_type` | `ToolType` | `ToolType.FUNCTION` | Choice type (tool type) |
| `function` | `SpecificFunction` | — | Function (specific function) |

---

### SpecificFunction

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `name` | `str` | — | The name |

---

### JsonSchemaFormat

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `name` | `str` | — | The name |
| `description` | `str | None` | `None` | Human-readable description |
| `schema` | `Any` | — | Schema |
| `strict` | `bool | None` | `None` | Strict |

---

### Usage

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `prompt_tokens` | `int` | — | Prompt tokens used. Defaults to 0 when absent (some providers omit this). |
| `completion_tokens` | `int` | — | Completion tokens used. Defaults to 0 when absent (e.g. embedding responses). |
| `total_tokens` | `int` | — | Total tokens used. Defaults to 0 when absent (some providers omit this). |

---

### ChatCompletionRequest

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

### StreamOptions

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `include_usage` | `bool | None` | `None` | Include usage |

---

### ChatCompletionResponse

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

---

### Choice

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `index` | `int` | — | Index |
| `message` | `AssistantMessage` | — | Message (assistant message) |
| `finish_reason` | `FinishReason | None` | `None` | Finish reason (finish reason) |

---

### ChatCompletionChunk

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

### StreamChoice

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `index` | `int` | — | Index |
| `delta` | `StreamDelta` | — | Delta (stream delta) |
| `finish_reason` | `FinishReason | None` | `None` | Finish reason (finish reason) |

---

### StreamDelta

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `role` | `str | None` | `None` | Role |
| `content` | `str | None` | `None` | The extracted text content |
| `tool_calls` | `list[StreamToolCall] | None` | `[]` | Tool calls |
| `function_call` | `StreamFunctionCall | None` | `None` | Deprecated legacy function_call delta; retained for API compatibility. |
| `refusal` | `str | None` | `None` | Refusal |

---

### StreamToolCall

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `index` | `int` | — | Index |
| `id` | `str | None` | `None` | Unique identifier |
| `call_type` | `ToolType | None` | `None` | Call type (tool type) |
| `function` | `StreamFunctionCall | None` | `None` | Function (stream function call) |

---

### StreamFunctionCall

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `name` | `str | None` | `None` | The name |
| `arguments` | `str | None` | `None` | Arguments |

---

### CreateImageRequest

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

### ImagesResponse

Response containing generated images.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `created` | `int` | — | Created |
| `data` | `list[Image]` | `[]` | Data |

---

### Image

A single generated image, returned as either a URL or base64 data.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `url` | `str | None` | `None` | Url |
| `b64_json` | `str | None` | `None` | B64 json |
| `revised_prompt` | `str | None` | `None` | Revised prompt |

---

### CreateSpeechRequest

Request to generate speech audio from text.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `model` | `str` | — | Model |
| `input` | `str` | — | Input |
| `voice` | `str` | — | Voice |
| `response_format` | `str | None` | `None` | Response format |
| `speed` | `float | None` | `None` | Speed |

---

### CreateTranscriptionRequest

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

### TranscriptionResponse

Response from a transcription request.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `text` | `str` | — | Text |
| `language` | `str | None` | `None` | Language |
| `duration` | `float | None` | `None` | Duration |
| `segments` | `list[TranscriptionSegment] | None` | `[]` | Segments |

---

### TranscriptionSegment

A segment of transcribed audio with timing information.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `id` | `int` | — | Unique identifier |
| `start` | `float` | — | Start |
| `end` | `float` | — | End |
| `text` | `str` | — | Text |

---

### SearchRequest

A search request.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `model` | `str` | — | The model/provider to use (e.g. `"brave/web-search"`, `"tavily/search"`). |
| `query` | `str` | — | The search query. |
| `max_results` | `int | None` | `None` | Maximum number of results to return. |
| `search_domain_filter` | `list[str] | None` | `[]` | Domain filter — restrict results to specific domains. |
| `country` | `str | None` | `None` | Country code for localized results (ISO 3166-1 alpha-2). |

---

### ModelsListResponse

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `object` | `str` | — | Always `"list"` from OpenAI-compatible APIs.  Stored as a plain `String` so non-standard provider values do not break deserialization. |
| `data` | `list[ModelObject]` | `[]` | Data |

---

### ModelObject

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `id` | `str` | — | Unique identifier |
| `object` | `str` | — | Always `"model"` from OpenAI-compatible APIs.  Stored as a plain `String` so non-standard provider values do not break deserialization. |
| `created` | `int` | — | Created |
| `owned_by` | `str` | — | Owned by |

---

### ClientConfig

Configuration for an LLM client.

`api_key` is stored as a `SecretString` so it is zeroed on drop and never
printed accidentally.  Access it via `secrecy.ExposeSecret`.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `api_key` | `str` | — | API key for authentication (stored as a secret). |
| `base_url` | `str | None` | `None` | Override base URL.  When set, all requests go here regardless of model name, and provider auto-detection is skipped. |
| `timeout` | `float` | — | Request timeout. |
| `max_retries` | `int` | — | Maximum number of retries on 429 / 5xx responses. |
| `credential_provider` | `CredentialProvider | None` | `None` | Optional dynamic credential provider for token-based auth (Azure AD, Vertex OAuth2) or refreshable credentials (AWS STS). When set, the client calls `resolve()` before each request to obtain a fresh credential.  When `None`, the static `api_key` is used. |
| `load_env` | `bool` | — | Automatically load the API key from the provider's environment variable when no explicit key is provided. When `True` (the default) and `api_key` is empty, `DefaultClient.new` reads the provider's designated environment variable (e.g. `OPENAI_API_KEY` for OpenAI).  Set to `False` to suppress this behaviour and require the caller to supply the key explicitly. Has no effect on WASM targets, where `std.env.var` is unavailable. |

---

### FileConfig

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

---

### FileCacheConfig

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `max_entries` | `int | None` | `None` | Maximum entries |
| `ttl_seconds` | `int | None` | `None` | Ttl seconds |
| `backend` | `str | None` | `None` | Backend |
| `backend_config` | `dict[str, str] | None` | `None` | Backend config |

---

### FileBudgetConfig

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `global_limit` | `float | None` | `None` | Global limit |
| `model_limits` | `dict[str, float] | None` | `None` | Model limits |
| `enforcement` | `str | None` | `None` | Enforcement |

---

### FileRateLimitConfig

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `rpm` | `int | None` | `None` | Rpm |
| `tpm` | `int | None` | `None` | Tpm |
| `window_seconds` | `int | None` | `None` | Window seconds |

---

### FileProviderConfig

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `name` | `str` | — | The name |
| `base_url` | `str` | — | Base url |
| `auth_header` | `str | None` | `None` | Auth header |
| `model_prefixes` | `list[str]` | — | Model prefixes |

---

### CustomProviderConfig

Configuration for registering a custom LLM provider at runtime.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `name` | `str` | — | Unique name for this provider (e.g., "my-provider"). |
| `base_url` | `str` | — | Base URL for the provider's API (e.g., "<https://api.my-provider.com/v1">). |
| `auth_header` | `AuthHeaderFormat` | — | Authentication header format. |
| `model_prefixes` | `list[str]` | — | Model name prefixes that route to this provider (e.g., ["my-"]). |

---

