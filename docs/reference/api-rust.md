---
title: "Rust API Reference"
---

## Rust API Reference <span class="version-badge">v1.4.0-rc.2</span>

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

```rust
pub fn create_client(api_key: &str, base_url: Option<String>, timeout_secs: Option<u64>, max_retries: Option<u32>, model_hint: Option<String>) -> Result<DefaultClient, Error>
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `api_key` | `String` | Yes | The api key |
| `base_url` | `Option<String>` | No | The base url |
| `timeout_secs` | `Option<u64>` | No | The timeout secs |
| `max_retries` | `Option<u32>` | No | The max retries |
| `model_hint` | `Option<String>` | No | The model hint |

**Returns:** `DefaultClient`

**Errors:** Returns `Err(Error)`.


---

#### create_client_from_json()

Create a new LLM client from a JSON string.

The JSON object accepts the same fields as `liter-llm.toml` (snake_case).

**Errors:**

Returns `LiterLlmError.BadRequest` if `json` is not valid JSON or
contains unknown fields.

**Signature:**

```rust
pub fn create_client_from_json(json: &str) -> Result<DefaultClient, Error>
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `json` | `String` | Yes | The json |

**Returns:** `DefaultClient`

**Errors:** Returns `Err(Error)`.


---

#### register_custom_provider()

Register a custom provider in the global runtime registry.

The provider will be checked **before** all built-in providers during model
detection. If a provider with the same `name` already exists it is replaced.

**Errors:**

Returns an error if the config is invalid (empty name, empty base_url, or
no model prefixes).

**Signature:**

```rust
pub fn register_custom_provider(config: CustomProviderConfig) -> Result<(), Error>
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `config` | `CustomProviderConfig` | Yes | The configuration options |

**Returns:** `()`

**Errors:** Returns `Err(Error)`.


---

#### unregister_custom_provider()

Remove a previously registered custom provider by name.

Returns `true` if a provider with the given name was found and removed,
`false` if no such provider existed.

**Errors:**

Returns an error only if the internal lock is poisoned.

**Signature:**

```rust
pub fn unregister_custom_provider(name: &str) -> Result<bool, Error>
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `name` | `String` | Yes | The name |

**Returns:** `bool`

**Errors:** Returns `Err(Error)`.


---

### Types

#### AssistantMessage

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `content` | `Option<String>` | `Default::default()` | The extracted text content |
| `name` | `Option<String>` | `Default::default()` | The name |
| `tool_calls` | `Option<Vec<ToolCall>>` | `vec![]` | Tool calls |
| `refusal` | `Option<String>` | `Default::default()` | Refusal |
| `function_call` | `Option<FunctionCall>` | `Default::default()` | Deprecated legacy function_call field; retained for API compatibility. |


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
| `created` | `u64` | — | Created |
| `model` | `String` | — | Model |
| `choices` | `Vec<StreamChoice>` | `vec![]` | Choices |
| `usage` | `Option<Usage>` | `Default::default()` | Usage (usage) |
| `system_fingerprint` | `Option<String>` | `Default::default()` | System fingerprint |
| `service_tier` | `Option<String>` | `Default::default()` | Service tier |


---

#### ChatCompletionRequest

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `model` | `String` | — | Model |
| `messages` | `Vec<Message>` | `vec![]` | Messages |
| `temperature` | `Option<f64>` | `Default::default()` | Temperature |
| `top_p` | `Option<f64>` | `Default::default()` | Top p |
| `n` | `Option<u32>` | `Default::default()` | N |
| `stream` | `Option<bool>` | `Default::default()` | Whether to stream the response. Managed by the client layer — do not set directly. |
| `stop` | `Option<StopSequence>` | `Default::default()` | Stop (stop sequence) |
| `max_tokens` | `Option<u64>` | `Default::default()` | Maximum tokens |
| `presence_penalty` | `Option<f64>` | `Default::default()` | Presence penalty |
| `frequency_penalty` | `Option<f64>` | `Default::default()` | Frequency penalty |
| `logit_bias` | `Option<HashMap<String, f64>>` | `HashMap::new()` | Token bias map.  Uses `BTreeMap` (sorted keys) for deterministic serialization order — important when hashing or signing requests. |
| `user` | `Option<String>` | `Default::default()` | User |
| `tools` | `Option<Vec<ChatCompletionTool>>` | `vec![]` | Tools |
| `tool_choice` | `Option<ToolChoice>` | `Default::default()` | Tool choice (tool choice) |
| `parallel_tool_calls` | `Option<bool>` | `Default::default()` | Parallel tool calls |
| `response_format` | `Option<ResponseFormat>` | `Default::default()` | Response format (response format) |
| `stream_options` | `Option<StreamOptions>` | `Default::default()` | Stream options (stream options) |
| `seed` | `Option<i64>` | `Default::default()` | Seed |
| `reasoning_effort` | `Option<ReasoningEffort>` | `Default::default()` | Reasoning effort (reasoning effort) |
| `extra_body` | `Option<serde_json::Value>` | `Default::default()` | Provider-specific extra parameters merged into the request body. Use for guardrails, safety settings, grounding config, etc. |


---

#### ChatCompletionResponse

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `id` | `String` | — | Unique identifier |
| `object` | `String` | — | Always `"chat.completion"` from OpenAI-compatible APIs.  Stored as a plain `String` so non-standard provider values do not break deserialization. |
| `created` | `u64` | — | Created |
| `model` | `String` | — | Model |
| `choices` | `Vec<Choice>` | `vec![]` | Choices |
| `usage` | `Option<Usage>` | `Default::default()` | Usage (usage) |
| `system_fingerprint` | `Option<String>` | `Default::default()` | System fingerprint |
| `service_tier` | `Option<String>` | `Default::default()` | Service tier |


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
| `index` | `u32` | — | Index |
| `message` | `AssistantMessage` | — | Message (assistant message) |
| `finish_reason` | `Option<FinishReason>` | `Default::default()` | Finish reason (finish reason) |


---

#### CreateImageRequest

Request to create images from a text prompt.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `prompt` | `String` | — | Prompt |
| `model` | `Option<String>` | `Default::default()` | Model |
| `n` | `Option<u32>` | `Default::default()` | N |
| `size` | `Option<String>` | `Default::default()` | Size in bytes |
| `quality` | `Option<String>` | `Default::default()` | Quality |
| `style` | `Option<String>` | `Default::default()` | Style |
| `response_format` | `Option<String>` | `Default::default()` | Response format |
| `user` | `Option<String>` | `Default::default()` | User |


---

#### CreateSpeechRequest

Request to generate speech audio from text.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `model` | `String` | — | Model |
| `input` | `String` | — | Input |
| `voice` | `String` | — | Voice |
| `response_format` | `Option<String>` | `Default::default()` | Response format |
| `speed` | `Option<f64>` | `Default::default()` | Speed |


---

#### CreateTranscriptionRequest

Request to transcribe audio into text.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `model` | `String` | — | Model |
| `file` | `String` | — | Base64-encoded audio file data. |
| `language` | `Option<String>` | `Default::default()` | Language |
| `prompt` | `Option<String>` | `Default::default()` | Prompt |
| `response_format` | `Option<String>` | `Default::default()` | Response format |
| `temperature` | `Option<f64>` | `Default::default()` | Temperature |


---

#### CustomProviderConfig

Configuration for registering a custom LLM provider at runtime.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `name` | `String` | — | Unique name for this provider (e.g., "my-provider"). |
| `base_url` | `String` | — | Base URL for the provider's API (e.g., "<https://api.my-provider.com/v1">). |
| `auth_header` | `AuthHeaderFormat` | — | Authentication header format. |
| `model_prefixes` | `Vec<String>` | — | Model name prefixes that route to this provider (e.g., ["my-"]). |


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

```rust
pub fn chat(&self, req: ChatCompletionRequest) -> ChatCompletionResponse
```

###### chat_stream()

**Signature:**

```rust
pub fn chat_stream(&self, req: ChatCompletionRequest) -> String
```

###### embed()

**Signature:**

```rust
pub fn embed(&self, req: EmbeddingRequest) -> EmbeddingResponse
```

###### list_models()

**Signature:**

```rust
pub fn list_models(&self) -> ModelsListResponse
```

###### image_generate()

**Signature:**

```rust
pub fn image_generate(&self, req: CreateImageRequest) -> ImagesResponse
```

###### transcribe()

**Signature:**

```rust
pub fn transcribe(&self, req: CreateTranscriptionRequest) -> TranscriptionResponse
```

###### moderate()

**Signature:**

```rust
pub fn moderate(&self, req: ModerationRequest) -> ModerationResponse
```

###### rerank()

**Signature:**

```rust
pub fn rerank(&self, req: RerankRequest) -> RerankResponse
```

###### search()

**Signature:**

```rust
pub fn search(&self, req: SearchRequest) -> SearchResponse
```


---

#### DeveloperMessage

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `content` | `String` | — | The extracted text content |
| `name` | `Option<String>` | `Default::default()` | The name |


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
| `embedding` | `Vec<f64>` | — | Embedding |
| `index` | `u32` | — | Index |


---

#### EmbeddingRequest

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `model` | `String` | — | Model |
| `input` | `EmbeddingInput` | — | Input (embedding input) |
| `encoding_format` | `Option<EmbeddingFormat>` | `None` | Encoding format (embedding format) |
| `dimensions` | `Option<u32>` | `None` | Dimensions |
| `user` | `Option<String>` | `None` | User |


---

#### EmbeddingResponse

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `object` | `String` | — | Always `"list"` from OpenAI-compatible APIs.  Stored as a plain `String` so non-standard provider values do not break deserialization. |
| `data` | `Vec<EmbeddingObject>` | — | Data |
| `model` | `String` | — | Model |
| `usage` | `Option<Usage>` | `None` | Usage (usage) |


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
| `description` | `Option<String>` | `None` | Human-readable description |
| `parameters` | `Option<serde_json::Value>` | `None` | Parameters |
| `strict` | `Option<bool>` | `None` | Strict |


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
| `url` | `Option<String>` | `Default::default()` | Url |
| `b64_json` | `Option<String>` | `Default::default()` | B64 json |
| `revised_prompt` | `Option<String>` | `Default::default()` | Revised prompt |


---

#### ImageUrl

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `url` | `String` | — | Url |
| `detail` | `Option<ImageDetail>` | `Default::default()` | Detail (image detail) |


---

#### ImagesResponse

Response containing generated images.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `created` | `u64` | — | Created |
| `data` | `Vec<Image>` | `vec![]` | Data |


---

#### JsonSchemaFormat

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `name` | `String` | — | The name |
| `description` | `Option<String>` | `Default::default()` | Human-readable description |
| `schema` | `serde_json::Value` | — | Schema |
| `strict` | `Option<bool>` | `Default::default()` | Strict |


---

#### LiterLlmError


---

#### ModelObject

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `id` | `String` | — | Unique identifier |
| `object` | `String` | — | Always `"model"` from OpenAI-compatible APIs.  Stored as a plain `String` so non-standard provider values do not break deserialization. |
| `created` | `u64` | — | Created |
| `owned_by` | `String` | — | Owned by |


---

#### ModelsListResponse

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `object` | `String` | — | Always `"list"` from OpenAI-compatible APIs.  Stored as a plain `String` so non-standard provider values do not break deserialization. |
| `data` | `Vec<ModelObject>` | `vec![]` | Data |


---

#### ModerationCategories

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

#### ModerationCategoryScores

Confidence scores for each moderation category.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `sexual` | `f64` | — | Sexual |
| `hate` | `f64` | — | Hate |
| `harassment` | `f64` | — | Harassment |
| `self_harm` | `f64` | — | Self harm |
| `sexual_minors` | `f64` | — | Sexual minors |
| `hate_threatening` | `f64` | — | Hate threatening |
| `violence_graphic` | `f64` | — | Violence graphic |
| `self_harm_intent` | `f64` | — | Self harm intent |
| `self_harm_instructions` | `f64` | — | Self harm instructions |
| `harassment_threatening` | `f64` | — | Harassment threatening |
| `violence` | `f64` | — | Violence |


---

#### ModerationRequest

Request to classify content for policy violations.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `input` | `ModerationInput` | — | Input (moderation input) |
| `model` | `Option<String>` | `None` | Model |


---

#### ModerationResponse

Response from the moderation endpoint.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `id` | `String` | — | Unique identifier |
| `model` | `String` | — | Model |
| `results` | `Vec<ModerationResult>` | — | Results |


---

#### ModerationResult

A single moderation classification result.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `flagged` | `bool` | — | Flagged |
| `categories` | `ModerationCategories` | — | Categories (moderation categories) |
| `category_scores` | `ModerationCategoryScores` | — | Category scores (moderation category scores) |


---

#### OcrImage

An image extracted from an OCR page.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `id` | `String` | — | Unique image identifier. |
| `image_base64` | `Option<String>` | `None` | Base64-encoded image data. |


---

#### OcrPage

A single page of OCR output.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `index` | `u32` | — | Page index (0-based). |
| `markdown` | `String` | — | Extracted content as Markdown. |
| `images` | `Option<Vec<OcrImage>>` | `None` | Extracted images, if `include_image_base64` was set. |
| `dimensions` | `Option<PageDimensions>` | `None` | Page dimensions in pixels, if available. |


---

#### OcrRequest

An OCR request.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `model` | `String` | — | The model/provider to use (e.g. `"mistral/mistral-ocr-latest"`). |
| `document` | `OcrDocument` | — | The document to process. |
| `pages` | `Option<Vec<u32>>` | `None` | Specific pages to process (1-indexed). `None` means all pages. |
| `include_image_base64` | `Option<bool>` | `None` | Whether to include base64-encoded images of each page. |


---

#### OcrResponse

An OCR response.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `pages` | `Vec<OcrPage>` | — | Extracted pages. |
| `model` | `String` | — | The model used. |
| `usage` | `Option<Usage>` | `None` | Token usage, if reported by the provider. |


---

#### PageDimensions

Page dimensions in pixels.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `width` | `u32` | — | Width in pixels. |
| `height` | `u32` | — | Height in pixels. |


---

#### RerankRequest

Request to rerank documents by relevance to a query.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `model` | `String` | — | Model |
| `query` | `String` | — | Query |
| `documents` | `Vec<RerankDocument>` | — | Documents |
| `top_n` | `Option<u32>` | `None` | Top n |
| `return_documents` | `Option<bool>` | `None` | Return documents |


---

#### RerankResponse

Response from the rerank endpoint.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `id` | `Option<String>` | `None` | Unique identifier |
| `results` | `Vec<RerankResult>` | — | Results |
| `meta` | `Option<serde_json::Value>` | `None` | Meta |


---

#### RerankResult

A single reranked document with its relevance score.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `index` | `u32` | — | Index |
| `relevance_score` | `f64` | — | Relevance score |
| `document` | `Option<RerankResultDocument>` | `None` | Document (rerank result document) |


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
| `max_results` | `Option<u32>` | `Default::default()` | Maximum number of results to return. |
| `search_domain_filter` | `Option<Vec<String>>` | `vec![]` | Domain filter — restrict results to specific domains. |
| `country` | `Option<String>` | `Default::default()` | Country code for localized results (ISO 3166-1 alpha-2). |


---

#### SearchResponse

A search response.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `results` | `Vec<SearchResult>` | — | The search results. |
| `model` | `String` | — | The model used. |


---

#### SearchResult

An individual search result.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `title` | `String` | — | Title of the result. |
| `url` | `String` | — | URL of the result. |
| `snippet` | `String` | — | Text snippet / excerpt. |
| `date` | `Option<String>` | `None` | Publication or last-updated date, if available. |


---

#### SpecificFunction

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `name` | `String` | — | The name |


---

#### SpecificToolChoice

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `choice_type` | `ToolType` | `ToolType::Function` | Choice type (tool type) |
| `function` | `SpecificFunction` | — | Function (specific function) |


---

#### StreamChoice

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `index` | `u32` | — | Index |
| `delta` | `StreamDelta` | — | Delta (stream delta) |
| `finish_reason` | `Option<FinishReason>` | `Default::default()` | Finish reason (finish reason) |


---

#### StreamDelta

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `role` | `Option<String>` | `Default::default()` | Role |
| `content` | `Option<String>` | `Default::default()` | The extracted text content |
| `tool_calls` | `Option<Vec<StreamToolCall>>` | `vec![]` | Tool calls |
| `function_call` | `Option<StreamFunctionCall>` | `Default::default()` | Deprecated legacy function_call delta; retained for API compatibility. |
| `refusal` | `Option<String>` | `Default::default()` | Refusal |


---

#### StreamFunctionCall

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `name` | `Option<String>` | `Default::default()` | The name |
| `arguments` | `Option<String>` | `Default::default()` | Arguments |


---

#### StreamOptions

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `include_usage` | `Option<bool>` | `Default::default()` | Include usage |


---

#### StreamToolCall

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `index` | `u32` | — | Index |
| `id` | `Option<String>` | `Default::default()` | Unique identifier |
| `call_type` | `Option<ToolType>` | `Default::default()` | Call type (tool type) |
| `function` | `Option<StreamFunctionCall>` | `Default::default()` | Function (stream function call) |


---

#### SystemMessage

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `content` | `String` | — | The extracted text content |
| `name` | `Option<String>` | `Default::default()` | The name |


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
| `name` | `Option<String>` | `Default::default()` | The name |


---

#### TranscriptionResponse

Response from a transcription request.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `text` | `String` | — | Text |
| `language` | `Option<String>` | `Default::default()` | Language |
| `duration` | `Option<f64>` | `Default::default()` | Duration |
| `segments` | `Option<Vec<TranscriptionSegment>>` | `vec![]` | Segments |


---

#### TranscriptionSegment

A segment of transcribed audio with timing information.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `id` | `u32` | — | Unique identifier |
| `start` | `f64` | — | Start |
| `end` | `f64` | — | End |
| `text` | `String` | — | Text |


---

#### Usage

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `prompt_tokens` | `u64` | — | Prompt tokens used. Defaults to 0 when absent (some providers omit this). |
| `completion_tokens` | `u64` | — | Completion tokens used. Defaults to 0 when absent (e.g. embedding responses). |
| `total_tokens` | `u64` | — | Total tokens used. Defaults to 0 when absent (some providers omit this). |


---

#### UserMessage

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `content` | `UserContent` | `UserContent::Text` | The extracted text content |
| `name` | `Option<String>` | `Default::default()` | The name |


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
| `Text` | Text format — Fields: `0`: `String` |
| `Parts` | Parts — Fields: `0`: `Vec<ContentPart>` |


---

#### ContentPart

| Value | Description |
|-------|-------------|
| `Text` | Text format — Fields: `text`: `String` |
| `ImageUrl` | Image url — Fields: `image_url`: `ImageUrl` |
| `Document` | Document — Fields: `document`: `DocumentContent` |
| `InputAudio` | Input audio — Fields: `input_audio`: `AudioContent` |


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
| `JsonSchema` | Json schema — Fields: `json_schema`: `JsonSchemaFormat` |


---

#### StopSequence

| Value | Description |
|-------|-------------|
| `Single` | Single — Fields: `0`: `String` |
| `Multiple` | Multiple — Fields: `0`: `Vec<String>` |


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
| `Single` | Single — Fields: `0`: `String` |
| `Multiple` | Multiple — Fields: `0`: `Vec<String>` |


---

#### ModerationInput

Input to the moderation endpoint — a single string or multiple strings.

| Value | Description |
|-------|-------------|
| `Single` | Single — Fields: `0`: `String` |
| `Multiple` | Multiple — Fields: `0`: `Vec<String>` |


---

#### RerankDocument

A document to be reranked — either a plain string or an object with a text field.

| Value | Description |
|-------|-------------|
| `Text` | Text format — Fields: `0`: `String` |
| `Object` | Object — Fields: `text`: `String` |


---

#### OcrDocument

Document input for OCR — either a URL or inline base64 data.

| Value | Description |
|-------|-------------|
| `Url` | A publicly accessible document URL. — Fields: `url`: `String` |
| `Base64` | Inline base64-encoded document data. — Fields: `data`: `String`, `media_type`: `String` |


---

#### AuthHeaderFormat

How the API key is sent in the HTTP request.

| Value | Description |
|-------|-------------|
| `Bearer` | Bearer token: `Authorization: Bearer <key>` |
| `ApiKey` | Custom header: e.g., `X-Api-Key: <key>` — Fields: `0`: `String` |
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

