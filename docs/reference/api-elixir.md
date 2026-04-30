---
title: "Elixir API Reference"
---

## Elixir API Reference <span class="version-badge">v1.4.0-rc.24</span>

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
| `pages` | `list(integer()) | nil` | `nil` | Specific pages to process (1-indexed). `nil` means all pages. |
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
