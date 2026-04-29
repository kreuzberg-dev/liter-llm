---
title: "Ruby API Reference"
---

## Ruby API Reference <span class="version-badge">v1.4.0-rc.12</span>

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

```ruby
def self.create_client(api_key, base_url: nil, timeout_secs: nil, max_retries: nil, model_hint: nil)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `api_key` | `String` | Yes | The api key |
| `base_url` | `String?` | No | The base url |
| `timeout_secs` | `Integer?` | No | The timeout secs |
| `max_retries` | `Integer?` | No | The max retries |
| `model_hint` | `String?` | No | The model hint |

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

```ruby
def self.create_client_from_json(json)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `json` | `String` | Yes | The json |

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

```ruby
def self.register_custom_provider(config)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `config` | `CustomProviderConfig` | Yes | The configuration options |

**Returns:** `nil`

**Errors:** Raises `Error`.


---

#### unregister_custom_provider()

Remove a previously registered custom provider by name.

Returns `true` if a provider with the given name was found and removed,
`false` if no such provider existed.

**Errors:**

Returns an error only if the internal lock is poisoned.

**Signature:**

```ruby
def self.unregister_custom_provider(name)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `name` | `String` | Yes | The name |

**Returns:** `Boolean`

**Errors:** Raises `Error`.


---

### Types

#### AssistantMessage

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `content` | `String?` | `nil` | The extracted text content |
| `name` | `String?` | `nil` | The name |
| `tool_calls` | `Array<ToolCall>?` | `[]` | Tool calls |
| `refusal` | `String?` | `nil` | Refusal |
| `function_call` | `FunctionCall?` | `nil` | Deprecated legacy function_call field; retained for API compatibility. |


---

#### AudioContent

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `data` | `String` | — | Base64-encoded audio data. |
| `format` | `String` | — | Audio format (e.g., "wav", "mp3", "ogg"). |


---

#### ChatCompletionChunk

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `id` | `String` | — | Unique identifier |
| `object` | `String` | — | Always `"chat.completion.chunk"` from OpenAI-compatible APIs.  Stored as a plain `String` so non-standard provider values do not fail parsing. |
| `created` | `Integer` | — | Created |
| `model` | `String` | — | Model |
| `choices` | `Array<StreamChoice>` | `[]` | Choices |
| `usage` | `Usage?` | `nil` | Usage (usage) |
| `system_fingerprint` | `String?` | `nil` | System fingerprint |
| `service_tier` | `String?` | `nil` | Service tier |


---

#### ChatCompletionRequest

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `model` | `String` | — | Model |
| `messages` | `Array<Message>` | `[]` | Messages |
| `temperature` | `Float?` | `nil` | Temperature |
| `top_p` | `Float?` | `nil` | Top p |
| `n` | `Integer?` | `nil` | N |
| `stream` | `Boolean?` | `nil` | Whether to stream the response. Managed by the client layer — do not set directly. |
| `stop` | `StopSequence?` | `nil` | Stop (stop sequence) |
| `max_tokens` | `Integer?` | `nil` | Maximum tokens |
| `presence_penalty` | `Float?` | `nil` | Presence penalty |
| `frequency_penalty` | `Float?` | `nil` | Frequency penalty |
| `logit_bias` | `Hash{String=>Float}?` | `{}` | Token bias map.  Uses `BTreeMap` (sorted keys) for deterministic serialization order — important when hashing or signing requests. |
| `user` | `String?` | `nil` | User |
| `tools` | `Array<ChatCompletionTool>?` | `[]` | Tools |
| `tool_choice` | `ToolChoice?` | `nil` | Tool choice (tool choice) |
| `parallel_tool_calls` | `Boolean?` | `nil` | Parallel tool calls |
| `response_format` | `ResponseFormat?` | `nil` | Response format (response format) |
| `stream_options` | `StreamOptions?` | `nil` | Stream options (stream options) |
| `seed` | `Integer?` | `nil` | Seed |
| `reasoning_effort` | `ReasoningEffort?` | `nil` | Reasoning effort (reasoning effort) |
| `extra_body` | `Object?` | `nil` | Provider-specific extra parameters merged into the request body. Use for guardrails, safety settings, grounding config, etc. |


---

#### ChatCompletionResponse

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `id` | `String` | — | Unique identifier |
| `object` | `String` | — | Always `"chat.completion"` from OpenAI-compatible APIs.  Stored as a plain `String` so non-standard provider values do not break deserialization. |
| `created` | `Integer` | — | Created |
| `model` | `String` | — | Model |
| `choices` | `Array<Choice>` | `[]` | Choices |
| `usage` | `Usage?` | `nil` | Usage (usage) |
| `system_fingerprint` | `String?` | `nil` | System fingerprint |
| `service_tier` | `String?` | `nil` | Service tier |


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
| `index` | `Integer` | — | Index |
| `message` | `AssistantMessage` | — | Message (assistant message) |
| `finish_reason` | `FinishReason?` | `nil` | Finish reason (finish reason) |


---

#### CreateImageRequest

Request to create images from a text prompt.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `prompt` | `String` | — | Prompt |
| `model` | `String?` | `nil` | Model |
| `n` | `Integer?` | `nil` | N |
| `size` | `String?` | `nil` | Size in bytes |
| `quality` | `String?` | `nil` | Quality |
| `style` | `String?` | `nil` | Style |
| `response_format` | `String?` | `nil` | Response format |
| `user` | `String?` | `nil` | User |


---

#### CreateSpeechRequest

Request to generate speech audio from text.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `model` | `String` | — | Model |
| `input` | `String` | — | Input |
| `voice` | `String` | — | Voice |
| `response_format` | `String?` | `nil` | Response format |
| `speed` | `Float?` | `nil` | Speed |


---

#### CreateTranscriptionRequest

Request to transcribe audio into text.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `model` | `String` | — | Model |
| `file` | `String` | — | Base64-encoded audio file data. |
| `language` | `String?` | `nil` | Language |
| `prompt` | `String?` | `nil` | Prompt |
| `response_format` | `String?` | `nil` | Response format |
| `temperature` | `Float?` | `nil` | Temperature |


---

#### CustomProviderConfig

Configuration for registering a custom LLM provider at runtime.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `name` | `String` | — | Unique name for this provider (e.g., "my-provider"). |
| `base_url` | `String` | — | Base URL for the provider's API (e.g., "<https://api.my-provider.com/v1">). |
| `auth_header` | `AuthHeaderFormat` | — | Authentication header format. |
| `model_prefixes` | `Array<String>` | — | Model name prefixes that route to this provider (e.g., ["my-"]). |


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

###### chat()

**Signature:**

```ruby
def chat(req)
```

###### chat_stream()

**Signature:**

```ruby
def chat_stream(req)
```

###### embed()

**Signature:**

```ruby
def embed(req)
```

###### list_models()

**Signature:**

```ruby
def list_models()
```

###### image_generate()

**Signature:**

```ruby
def image_generate(req)
```

###### transcribe()

**Signature:**

```ruby
def transcribe(req)
```

###### moderate()

**Signature:**

```ruby
def moderate(req)
```

###### rerank()

**Signature:**

```ruby
def rerank(req)
```

###### search()

**Signature:**

```ruby
def search(req)
```


---

#### DeveloperMessage

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `content` | `String` | — | The extracted text content |
| `name` | `String?` | `nil` | The name |


---

#### DocumentContent

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `data` | `String` | — | Base64-encoded document data or URL. |
| `media_type` | `String` | — | MIME type (e.g., "application/pdf", "text/csv"). |


---

#### EmbeddingObject

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `object` | `String` | — | Always `"embedding"` from OpenAI-compatible APIs.  Stored as a plain `String` so non-standard provider values do not break deserialization. |
| `embedding` | `Array<Float>` | — | Embedding |
| `index` | `Integer` | — | Index |


---

#### EmbeddingRequest

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `model` | `String` | — | Model |
| `input` | `EmbeddingInput` | — | Input (embedding input) |
| `encoding_format` | `EmbeddingFormat?` | `nil` | Encoding format (embedding format) |
| `dimensions` | `Integer?` | `nil` | Dimensions |
| `user` | `String?` | `nil` | User |


---

#### EmbeddingResponse

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `object` | `String` | — | Always `"list"` from OpenAI-compatible APIs.  Stored as a plain `String` so non-standard provider values do not break deserialization. |
| `data` | `Array<EmbeddingObject>` | — | Data |
| `model` | `String` | — | Model |
| `usage` | `Usage?` | `nil` | Usage (usage) |


---

#### FunctionCall

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `name` | `String` | — | The name |
| `arguments` | `String` | — | Arguments |


---

#### FunctionDefinition

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `name` | `String` | — | The name |
| `description` | `String?` | `nil` | Human-readable description |
| `parameters` | `Object?` | `nil` | Parameters |
| `strict` | `Boolean?` | `nil` | Strict |


---

#### FunctionMessage

Deprecated legacy function-role message body.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `content` | `String` | — | The extracted text content |
| `name` | `String` | — | The name |


---

#### Image

A single generated image, returned as either a URL or base64 data.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `url` | `String?` | `nil` | Url |
| `b64_json` | `String?` | `nil` | B64 json |
| `revised_prompt` | `String?` | `nil` | Revised prompt |


---

#### ImageUrl

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `url` | `String` | — | Url |
| `detail` | `ImageDetail?` | `nil` | Detail (image detail) |


---

#### ImagesResponse

Response containing generated images.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `created` | `Integer` | — | Created |
| `data` | `Array<Image>` | `[]` | Data |


---

#### JsonSchemaFormat

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `name` | `String` | — | The name |
| `description` | `String?` | `nil` | Human-readable description |
| `schema` | `Object` | — | Schema |
| `strict` | `Boolean?` | `nil` | Strict |


---

#### LiterLlmError


---

#### ModelObject

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `id` | `String` | — | Unique identifier |
| `object` | `String` | — | Always `"model"` from OpenAI-compatible APIs.  Stored as a plain `String` so non-standard provider values do not break deserialization. |
| `created` | `Integer` | — | Created |
| `owned_by` | `String` | — | Owned by |


---

#### ModelsListResponse

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `object` | `String` | — | Always `"list"` from OpenAI-compatible APIs.  Stored as a plain `String` so non-standard provider values do not break deserialization. |
| `data` | `Array<ModelObject>` | `[]` | Data |


---

#### ModerationCategories

Boolean flags for each moderation category.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `sexual` | `Boolean` | — | Sexual |
| `hate` | `Boolean` | — | Hate |
| `harassment` | `Boolean` | — | Harassment |
| `self_harm` | `Boolean` | — | Self harm |
| `sexual_minors` | `Boolean` | — | Sexual minors |
| `hate_threatening` | `Boolean` | — | Hate threatening |
| `violence_graphic` | `Boolean` | — | Violence graphic |
| `self_harm_intent` | `Boolean` | — | Self harm intent |
| `self_harm_instructions` | `Boolean` | — | Self harm instructions |
| `harassment_threatening` | `Boolean` | — | Harassment threatening |
| `violence` | `Boolean` | — | Violence |


---

#### ModerationCategoryScores

Confidence scores for each moderation category.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `sexual` | `Float` | — | Sexual |
| `hate` | `Float` | — | Hate |
| `harassment` | `Float` | — | Harassment |
| `self_harm` | `Float` | — | Self harm |
| `sexual_minors` | `Float` | — | Sexual minors |
| `hate_threatening` | `Float` | — | Hate threatening |
| `violence_graphic` | `Float` | — | Violence graphic |
| `self_harm_intent` | `Float` | — | Self harm intent |
| `self_harm_instructions` | `Float` | — | Self harm instructions |
| `harassment_threatening` | `Float` | — | Harassment threatening |
| `violence` | `Float` | — | Violence |


---

#### ModerationRequest

Request to classify content for policy violations.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `input` | `ModerationInput` | — | Input (moderation input) |
| `model` | `String?` | `nil` | Model |


---

#### ModerationResponse

Response from the moderation endpoint.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `id` | `String` | — | Unique identifier |
| `model` | `String` | — | Model |
| `results` | `Array<ModerationResult>` | — | Results |


---

#### ModerationResult

A single moderation classification result.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `flagged` | `Boolean` | — | Flagged |
| `categories` | `ModerationCategories` | — | Categories (moderation categories) |
| `category_scores` | `ModerationCategoryScores` | — | Category scores (moderation category scores) |


---

#### OcrImage

An image extracted from an OCR page.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `id` | `String` | — | Unique image identifier. |
| `image_base64` | `String?` | `nil` | Base64-encoded image data. |


---

#### OcrPage

A single page of OCR output.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `index` | `Integer` | — | Page index (0-based). |
| `markdown` | `String` | — | Extracted content as Markdown. |
| `images` | `Array<OcrImage>?` | `nil` | Extracted images, if `include_image_base64` was set. |
| `dimensions` | `PageDimensions?` | `nil` | Page dimensions in pixels, if available. |


---

#### OcrRequest

An OCR request.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `model` | `String` | — | The model/provider to use (e.g. `"mistral/mistral-ocr-latest"`). |
| `document` | `OcrDocument` | — | The document to process. |
| `pages` | `Array<Integer>?` | `nil` | Specific pages to process (1-indexed). `nil` means all pages. |
| `include_image_base64` | `Boolean?` | `nil` | Whether to include base64-encoded images of each page. |


---

#### OcrResponse

An OCR response.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `pages` | `Array<OcrPage>` | — | Extracted pages. |
| `model` | `String` | — | The model used. |
| `usage` | `Usage?` | `nil` | Token usage, if reported by the provider. |


---

#### PageDimensions

Page dimensions in pixels.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `width` | `Integer` | — | Width in pixels. |
| `height` | `Integer` | — | Height in pixels. |


---

#### RerankRequest

Request to rerank documents by relevance to a query.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `model` | `String` | — | Model |
| `query` | `String` | — | Query |
| `documents` | `Array<RerankDocument>` | — | Documents |
| `top_n` | `Integer?` | `nil` | Top n |
| `return_documents` | `Boolean?` | `nil` | Return documents |


---

#### RerankResponse

Response from the rerank endpoint.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `id` | `String?` | `nil` | Unique identifier |
| `results` | `Array<RerankResult>` | — | Results |
| `meta` | `Object?` | `nil` | Meta |


---

#### RerankResult

A single reranked document with its relevance score.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `index` | `Integer` | — | Index |
| `relevance_score` | `Float` | — | Relevance score |
| `document` | `RerankResultDocument?` | `nil` | Document (rerank result document) |


---

#### RerankResultDocument

The text content of a reranked document, returned when `return_documents` is true.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `text` | `String` | — | Text |


---

#### SearchRequest

A search request.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `model` | `String` | — | The model/provider to use (e.g. `"brave/web-search"`, `"tavily/search"`). |
| `query` | `String` | — | The search query. |
| `max_results` | `Integer?` | `nil` | Maximum number of results to return. |
| `search_domain_filter` | `Array<String>?` | `[]` | Domain filter — restrict results to specific domains. |
| `country` | `String?` | `nil` | Country code for localized results (ISO 3166-1 alpha-2). |


---

#### SearchResponse

A search response.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `results` | `Array<SearchResult>` | — | The search results. |
| `model` | `String` | — | The model used. |


---

#### SearchResult

An individual search result.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `title` | `String` | — | Title of the result. |
| `url` | `String` | — | URL of the result. |
| `snippet` | `String` | — | Text snippet / excerpt. |
| `date` | `String?` | `nil` | Publication or last-updated date, if available. |


---

#### SpecificFunction

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `name` | `String` | — | The name |


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
| `index` | `Integer` | — | Index |
| `delta` | `StreamDelta` | — | Delta (stream delta) |
| `finish_reason` | `FinishReason?` | `nil` | Finish reason (finish reason) |


---

#### StreamDelta

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `role` | `String?` | `nil` | Role |
| `content` | `String?` | `nil` | The extracted text content |
| `tool_calls` | `Array<StreamToolCall>?` | `[]` | Tool calls |
| `function_call` | `StreamFunctionCall?` | `nil` | Deprecated legacy function_call delta; retained for API compatibility. |
| `refusal` | `String?` | `nil` | Refusal |


---

#### StreamFunctionCall

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `name` | `String?` | `nil` | The name |
| `arguments` | `String?` | `nil` | Arguments |


---

#### StreamOptions

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `include_usage` | `Boolean?` | `nil` | Include usage |


---

#### StreamToolCall

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `index` | `Integer` | — | Index |
| `id` | `String?` | `nil` | Unique identifier |
| `call_type` | `ToolType?` | `nil` | Call type (tool type) |
| `function` | `StreamFunctionCall?` | `nil` | Function (stream function call) |


---

#### SystemMessage

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `content` | `String` | — | The extracted text content |
| `name` | `String?` | `nil` | The name |


---

#### ToolCall

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `id` | `String` | — | Unique identifier |
| `call_type` | `ToolType` | — | Call type (tool type) |
| `function` | `FunctionCall` | — | Function (function call) |


---

#### ToolMessage

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `content` | `String` | — | The extracted text content |
| `tool_call_id` | `String` | — | Tool call id |
| `name` | `String?` | `nil` | The name |


---

#### TranscriptionResponse

Response from a transcription request.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `text` | `String` | — | Text |
| `language` | `String?` | `nil` | Language |
| `duration` | `Float?` | `nil` | Duration |
| `segments` | `Array<TranscriptionSegment>?` | `[]` | Segments |


---

#### TranscriptionSegment

A segment of transcribed audio with timing information.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `id` | `Integer` | — | Unique identifier |
| `start` | `Float` | — | Start |
| `end` | `Float` | — | End |
| `text` | `String` | — | Text |


---

#### Usage

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `prompt_tokens` | `Integer` | — | Prompt tokens used. Defaults to 0 when absent (some providers omit this). |
| `completion_tokens` | `Integer` | — | Completion tokens used. Defaults to 0 when absent (e.g. embedding responses). |
| `total_tokens` | `Integer` | — | Total tokens used. Defaults to 0 when absent (some providers omit this). |


---

#### UserMessage

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `content` | `UserContent` | `:text` | The extracted text content |
| `name` | `String?` | `nil` | The name |


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
| `text` | Text format — Fields: `0`: `String` |
| `parts` | Parts — Fields: `0`: `Array<ContentPart>` |


---

#### ContentPart

| Value | Description |
|-------|-------------|
| `text` | Text format — Fields: `text`: `String` |
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
| `single` | Single — Fields: `0`: `String` |
| `multiple` | Multiple — Fields: `0`: `Array<String>` |


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
| `single` | Single — Fields: `0`: `String` |
| `multiple` | Multiple — Fields: `0`: `Array<String>` |


---

#### ModerationInput

Input to the moderation endpoint — a single string or multiple strings.

| Value | Description |
|-------|-------------|
| `single` | Single — Fields: `0`: `String` |
| `multiple` | Multiple — Fields: `0`: `Array<String>` |


---

#### RerankDocument

A document to be reranked — either a plain string or an object with a text field.

| Value | Description |
|-------|-------------|
| `text` | Text format — Fields: `0`: `String` |
| `object` | Object — Fields: `text`: `String` |


---

#### OcrDocument

Document input for OCR — either a URL or inline base64 data.

| Value | Description |
|-------|-------------|
| `url` | A publicly accessible document URL. — Fields: `url`: `String` |
| `base64` | Inline base64-encoded document data. — Fields: `data`: `String`, `media_type`: `String` |


---

#### AuthHeaderFormat

How the API key is sent in the HTTP request.

| Value | Description |
|-------|-------------|
| `bearer` | Bearer token: `Authorization: Bearer <key>` |
| `api_key` | Custom header: e.g., `X-Api-Key: <key>` — Fields: `0`: `String` |
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
