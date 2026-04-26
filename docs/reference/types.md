---
title: "Types Reference"
---

## Types Reference

All types defined by the library, grouped by category. Types are shown using Rust as the canonical representation.

### Result Types

#### ModerationResult

A single moderation classification result.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `flagged` | `bool` | — | Flagged |
| `categories` | `ModerationCategories` | — | Categories (moderation categories) |
| `category_scores` | `ModerationCategoryScores` | — | Category scores (moderation category scores) |

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

#### SearchResult

An individual search result.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `title` | `String` | — | Title of the result. |
| `url` | `String` | — | URL of the result. |
| `snippet` | `String` | — | Text snippet / excerpt. |
| `date` | `Option<String>` | `None` | Publication or last-updated date, if available. |

---

### Configuration Types

See [Configuration Reference](configuration.md) for detailed defaults and language-specific representations.

#### SystemMessage

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `content` | `String` | — | The extracted text content |
| `name` | `Option<String>` | `Default::default()` | The name |

---

#### UserMessage

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `content` | `UserContent` | `UserContent::Text` | The extracted text content |
| `name` | `Option<String>` | `Default::default()` | The name |

---

#### ImageUrl

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `url` | `String` | — | Url |
| `detail` | `Option<ImageDetail>` | `Default::default()` | Detail (image detail) |

---

#### DocumentContent

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `data` | `String` | — | Base64-encoded document data or URL. |
| `media_type` | `String` | — | MIME type (e.g., "application/pdf", "text/csv"). |

---

#### AudioContent

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `data` | `String` | — | Base64-encoded audio data. |
| `format` | `String` | — | Audio format (e.g., "wav", "mp3", "ogg"). |

---

#### AssistantMessage

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `content` | `Option<String>` | `Default::default()` | The extracted text content |
| `name` | `Option<String>` | `Default::default()` | The name |
| `tool_calls` | `Vec<ToolCall>` | `vec![]` | Tool calls |
| `refusal` | `Option<String>` | `Default::default()` | Refusal |
| `function_call` | `Option<FunctionCall>` | `Default::default()` | Deprecated legacy function_call field; retained for API compatibility. |

---

#### ToolMessage

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `content` | `String` | — | The extracted text content |
| `tool_call_id` | `String` | — | Tool call id |
| `name` | `Option<String>` | `Default::default()` | The name |

---

#### DeveloperMessage

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `content` | `String` | — | The extracted text content |
| `name` | `Option<String>` | `Default::default()` | The name |

---

#### FunctionMessage

Deprecated legacy function-role message body.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `content` | `String` | — | The extracted text content |
| `name` | `String` | — | The name |

---

#### SpecificToolChoice

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `choice_type` | `ToolType` | `ToolType::Function` | Choice type (tool type) |
| `function` | `SpecificFunction` | — | Function (specific function) |

---

#### SpecificFunction

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `name` | `String` | — | The name |

---

#### JsonSchemaFormat

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `name` | `String` | — | The name |
| `description` | `Option<String>` | `Default::default()` | Human-readable description |
| `schema` | `serde_json::Value` | — | Schema |
| `strict` | `Option<bool>` | `Default::default()` | Strict |

---

#### Usage

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `prompt_tokens` | `u64` | — | Prompt tokens used. Defaults to 0 when absent (some providers omit this). |
| `completion_tokens` | `u64` | — | Completion tokens used. Defaults to 0 when absent (e.g. embedding responses). |
| `total_tokens` | `u64` | — | Total tokens used. Defaults to 0 when absent (some providers omit this). |

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
| `logit_bias` | `HashMap<String, f64>` | `HashMap::new()` | Token bias map.  Uses `BTreeMap` (sorted keys) for deterministic serialization order — important when hashing or signing requests. |
| `user` | `Option<String>` | `Default::default()` | User |
| `tools` | `Vec<ChatCompletionTool>` | `vec![]` | Tools |
| `tool_choice` | `Option<ToolChoice>` | `Default::default()` | Tool choice (tool choice) |
| `parallel_tool_calls` | `Option<bool>` | `Default::default()` | Parallel tool calls |
| `response_format` | `Option<ResponseFormat>` | `Default::default()` | Response format (response format) |
| `stream_options` | `Option<StreamOptions>` | `Default::default()` | Stream options (stream options) |
| `seed` | `Option<i64>` | `Default::default()` | Seed |
| `reasoning_effort` | `Option<ReasoningEffort>` | `Default::default()` | Reasoning effort (reasoning effort) |
| `extra_body` | `Option<serde_json::Value>` | `Default::default()` | Provider-specific extra parameters merged into the request body. Use for guardrails, safety settings, grounding config, etc. |

---

#### StreamOptions

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `include_usage` | `Option<bool>` | `Default::default()` | Include usage |

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

#### Choice

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `index` | `u32` | — | Index |
| `message` | `AssistantMessage` | — | Message (assistant message) |
| `finish_reason` | `Option<FinishReason>` | `Default::default()` | Finish reason (finish reason) |

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
| `tool_calls` | `Vec<StreamToolCall>` | `vec![]` | Tool calls |
| `function_call` | `Option<StreamFunctionCall>` | `Default::default()` | Deprecated legacy function_call delta; retained for API compatibility. |
| `refusal` | `Option<String>` | `Default::default()` | Refusal |

---

#### StreamToolCall

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `index` | `u32` | — | Index |
| `id` | `Option<String>` | `Default::default()` | Unique identifier |
| `call_type` | `Option<ToolType>` | `Default::default()` | Call type (tool type) |
| `function` | `Option<StreamFunctionCall>` | `Default::default()` | Function (stream function call) |

---

#### StreamFunctionCall

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `name` | `Option<String>` | `Default::default()` | The name |
| `arguments` | `Option<String>` | `Default::default()` | Arguments |

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

#### ImagesResponse

Response containing generated images.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `created` | `u64` | — | Created |
| `data` | `Vec<Image>` | `vec![]` | Data |

---

#### Image

A single generated image, returned as either a URL or base64 data.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `url` | `Option<String>` | `Default::default()` | Url |
| `b64_json` | `Option<String>` | `Default::default()` | B64 json |
| `revised_prompt` | `Option<String>` | `Default::default()` | Revised prompt |

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

#### TranscriptionResponse

Response from a transcription request.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `text` | `String` | — | Text |
| `language` | `Option<String>` | `Default::default()` | Language |
| `duration` | `Option<f64>` | `Default::default()` | Duration |
| `segments` | `Vec<TranscriptionSegment>` | `vec![]` | Segments |

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

#### SearchRequest

A search request.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `model` | `String` | — | The model/provider to use (e.g. `"brave/web-search"`, `"tavily/search"`). |
| `query` | `String` | — | The search query. |
| `max_results` | `Option<u32>` | `Default::default()` | Maximum number of results to return. |
| `search_domain_filter` | `Vec<String>` | `vec![]` | Domain filter — restrict results to specific domains. |
| `country` | `Option<String>` | `Default::default()` | Country code for localized results (ISO 3166-1 alpha-2). |

---

#### ModelsListResponse

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `object` | `String` | — | Always `"list"` from OpenAI-compatible APIs.  Stored as a plain `String` so non-standard provider values do not break deserialization. |
| `data` | `Vec<ModelObject>` | `vec![]` | Data |

---

#### ModelObject

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `id` | `String` | — | Unique identifier |
| `object` | `String` | — | Always `"model"` from OpenAI-compatible APIs.  Stored as a plain `String` so non-standard provider values do not break deserialization. |
| `created` | `u64` | — | Created |
| `owned_by` | `String` | — | Owned by |

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

### OCR Types

#### OcrRequest

An OCR request.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `model` | `String` | — | The model/provider to use (e.g. `"mistral/mistral-ocr-latest"`). |
| `document` | `OcrDocument` | — | The document to process. |
| `pages` | `Vec<u32>` | `None` | Specific pages to process (1-indexed). `None` means all pages. |
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

#### OcrPage

A single page of OCR output.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `index` | `u32` | — | Page index (0-based). |
| `markdown` | `String` | — | Extracted content as Markdown. |
| `images` | `Vec<OcrImage>` | `None` | Extracted images, if `include_image_base64` was set. |
| `dimensions` | `Option<PageDimensions>` | `None` | Page dimensions in pixels, if available. |

---

#### OcrImage

An image extracted from an OCR page.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `id` | `String` | — | Unique image identifier. |
| `image_base64` | `Option<String>` | `None` | Base64-encoded image data. |

---

### Other Types

#### LiterLlmError

*Opaque type — fields are not directly accessible.*

---

#### ChatCompletionTool

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `tool_type` | `ToolType` | — | Tool type (tool type) |
| `function` | `FunctionDefinition` | — | Function (function definition) |

---

#### FunctionDefinition

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `name` | `String` | — | The name |
| `description` | `Option<String>` | `None` | Human-readable description |
| `parameters` | `Option<serde_json::Value>` | `None` | Parameters |
| `strict` | `Option<bool>` | `None` | Strict |

---

#### ToolCall

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `id` | `String` | — | Unique identifier |
| `call_type` | `ToolType` | — | Call type (tool type) |
| `function` | `FunctionCall` | — | Function (function call) |

---

#### FunctionCall

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `name` | `String` | — | The name |
| `arguments` | `String` | — | Arguments |

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

#### EmbeddingObject

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `object` | `String` | — | Always `"embedding"` from OpenAI-compatible APIs.  Stored as a plain `String` so non-standard provider values do not break deserialization. |
| `embedding` | `Vec<f64>` | — | Embedding |
| `index` | `u32` | — | Index |

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

#### SearchResponse

A search response.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `results` | `Vec<SearchResult>` | — | The search results. |
| `model` | `String` | — | The model used. |

---

#### PageDimensions

Page dimensions in pixels.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `width` | `u32` | — | Width in pixels. |
| `height` | `u32` | — | Height in pixels. |

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

*Opaque type — fields are not directly accessible.*

---

