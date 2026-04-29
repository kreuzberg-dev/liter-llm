---
title: "C API Reference"
---

## C API Reference <span class="version-badge">v1.4.0-rc.14</span>

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

###### literllm_chat()

**Signature:**

```c
LiterllmChatCompletionResponse literllm_chat(LiterllmChatCompletionRequest req);
```

###### literllm_chat_stream()

**Signature:**

```c
const char* literllm_chat_stream(LiterllmChatCompletionRequest req);
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
| `pages` | `uint32_t**` | `NULL` | Specific pages to process (1-indexed). `NULL` means all pages. |
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
