---
title: "Java API Reference"
---

## Java API Reference <span class="version-badge">v1.4.0-rc.27</span>

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

```java
public static DefaultClient createClient(String apiKey, String baseUrl, long timeoutSecs, int maxRetries, String modelHint) throws Error
```

**Parameters:**

| Name          | Type                | Required | Description      |
| ------------- | ------------------- | -------- | ---------------- |
| `apiKey`      | `String`            | Yes      | The api key      |
| `baseUrl`     | `Optional<String>`  | No       | The base url     |
| `timeoutSecs` | `Optional<Long>`    | No       | The timeout secs |
| `maxRetries`  | `Optional<Integer>` | No       | The max retries  |
| `modelHint`   | `Optional<String>`  | No       | The model hint   |

**Returns:** `DefaultClient`

**Errors:** Throws `ErrorException`.

---

#### createClientFromJson()

Create a new LLM client from a JSON string.

The JSON object accepts the same fields as `liter-llm.toml` (snake_case).

**Errors:**

Returns `LiterLlmError.BadRequest` if `json` is not valid JSON or
contains unknown fields.

**Signature:**

```java
public static DefaultClient createClientFromJson(String json) throws Error
```

**Parameters:**

| Name   | Type     | Required | Description |
| ------ | -------- | -------- | ----------- |
| `json` | `String` | Yes      | The json    |

**Returns:** `DefaultClient`

**Errors:** Throws `ErrorException`.

---

#### registerCustomProvider()

Register a custom provider in the global runtime registry.

The provider will be checked **before** all built-in providers during model
detection. If a provider with the same `name` already exists it is replaced.

**Errors:**

Returns an error if the config is invalid (empty name, empty base_url, or
no model prefixes).

**Signature:**

```java
public static void registerCustomProvider(CustomProviderConfig config) throws Error
```

**Parameters:**

| Name     | Type                   | Required | Description               |
| -------- | ---------------------- | -------- | ------------------------- |
| `config` | `CustomProviderConfig` | Yes      | The configuration options |

**Returns:** `void`

**Errors:** Throws `ErrorException`.

---

#### unregisterCustomProvider()

Remove a previously registered custom provider by name.

Returns `true` if a provider with the given name was found and removed,
`false` if no such provider existed.

**Errors:**

Returns an error only if the internal lock is poisoned.

**Signature:**

```java
public static boolean unregisterCustomProvider(String name) throws Error
```

**Parameters:**

| Name   | Type     | Required | Description |
| ------ | -------- | -------- | ----------- |
| `name` | `String` | Yes      | The name    |

**Returns:** `boolean`

**Errors:** Throws `ErrorException`.

---

### Types

#### AssistantMessage

| Field          | Type                       | Default                   | Description                                                            |
| -------------- | -------------------------- | ------------------------- | ---------------------------------------------------------------------- |
| `content`      | `Optional<String>`         | `null`                    | The extracted text content                                             |
| `name`         | `Optional<String>`         | `null`                    | The name                                                               |
| `toolCalls`    | `Optional<List<ToolCall>>` | `Collections.emptyList()` | Tool calls                                                             |
| `refusal`      | `Optional<String>`         | `null`                    | Refusal                                                                |
| `functionCall` | `Optional<FunctionCall>`   | `null`                    | Deprecated legacy function_call field; retained for API compatibility. |

---

#### AudioContent

| Field    | Type     | Default | Description                               |
| -------- | -------- | ------- | ----------------------------------------- |
| `data`   | `String` | —       | Base64-encoded audio data.                |
| `format` | `String` | —       | Audio format (e.g., "wav", "mp3", "ogg"). |

---

#### ChatCompletionChunk

| Field               | Type                 | Default                   | Description                                                                                                                                   |
| ------------------- | -------------------- | ------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------- |
| `id`                | `String`             | —                         | Unique identifier                                                                                                                             |
| `object`            | `String`             | —                         | Always `"chat.completion.chunk"` from OpenAI-compatible APIs. Stored as a plain `String` so non-standard provider values do not fail parsing. |
| `created`           | `long`               | —                         | Created                                                                                                                                       |
| `model`             | `String`             | —                         | Model                                                                                                                                         |
| `choices`           | `List<StreamChoice>` | `Collections.emptyList()` | Choices                                                                                                                                       |
| `usage`             | `Optional<Usage>`    | `null`                    | Usage (usage)                                                                                                                                 |
| `systemFingerprint` | `Optional<String>`   | `null`                    | System fingerprint                                                                                                                            |
| `serviceTier`       | `Optional<String>`   | `null`                    | Service tier                                                                                                                                  |

---

#### ChatCompletionRequest

| Field               | Type                                 | Default                   | Description                                                                                                                       |
| ------------------- | ------------------------------------ | ------------------------- | --------------------------------------------------------------------------------------------------------------------------------- |
| `model`             | `String`                             | —                         | Model                                                                                                                             |
| `messages`          | `List<Message>`                      | `Collections.emptyList()` | Messages                                                                                                                          |
| `temperature`       | `Optional<Double>`                   | `null`                    | Temperature                                                                                                                       |
| `topP`              | `Optional<Double>`                   | `null`                    | Top p                                                                                                                             |
| `n`                 | `Optional<Integer>`                  | `null`                    | N                                                                                                                                 |
| `stream`            | `Optional<Boolean>`                  | `null`                    | Whether to stream the response. Managed by the client layer — do not set directly.                                                |
| `stop`              | `Optional<StopSequence>`             | `null`                    | Stop (stop sequence)                                                                                                              |
| `maxTokens`         | `Optional<Long>`                     | `null`                    | Maximum tokens                                                                                                                    |
| `presencePenalty`   | `Optional<Double>`                   | `null`                    | Presence penalty                                                                                                                  |
| `frequencyPenalty`  | `Optional<Double>`                   | `null`                    | Frequency penalty                                                                                                                 |
| `logitBias`         | `Optional<Map<String, Double>>`      | `Collections.emptyMap()`  | Token bias map. Uses `BTreeMap` (sorted keys) for deterministic serialization order — important when hashing or signing requests. |
| `user`              | `Optional<String>`                   | `null`                    | User                                                                                                                              |
| `tools`             | `Optional<List<ChatCompletionTool>>` | `Collections.emptyList()` | Tools                                                                                                                             |
| `toolChoice`        | `Optional<ToolChoice>`               | `null`                    | Tool choice (tool choice)                                                                                                         |
| `parallelToolCalls` | `Optional<Boolean>`                  | `null`                    | Parallel tool calls                                                                                                               |
| `responseFormat`    | `Optional<ResponseFormat>`           | `null`                    | Response format (response format)                                                                                                 |
| `streamOptions`     | `Optional<StreamOptions>`            | `null`                    | Stream options (stream options)                                                                                                   |
| `seed`              | `Optional<Long>`                     | `null`                    | Seed                                                                                                                              |
| `reasoningEffort`   | `Optional<ReasoningEffort>`          | `null`                    | Reasoning effort (reasoning effort)                                                                                               |
| `extraBody`         | `Optional<Object>`                   | `null`                    | Provider-specific extra parameters merged into the request body. Use for guardrails, safety settings, grounding config, etc.      |

---

#### ChatCompletionResponse

| Field               | Type               | Default                   | Description                                                                                                                                      |
| ------------------- | ------------------ | ------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------ |
| `id`                | `String`           | —                         | Unique identifier                                                                                                                                |
| `object`            | `String`           | —                         | Always `"chat.completion"` from OpenAI-compatible APIs. Stored as a plain `String` so non-standard provider values do not break deserialization. |
| `created`           | `long`             | —                         | Created                                                                                                                                          |
| `model`             | `String`           | —                         | Model                                                                                                                                            |
| `choices`           | `List<Choice>`     | `Collections.emptyList()` | Choices                                                                                                                                          |
| `usage`             | `Optional<Usage>`  | `null`                    | Usage (usage)                                                                                                                                    |
| `systemFingerprint` | `Optional<String>` | `null`                    | System fingerprint                                                                                                                               |
| `serviceTier`       | `Optional<String>` | `null`                    | Service tier                                                                                                                                     |

---

#### ChatCompletionTool

| Field      | Type                 | Default | Description                    |
| ---------- | -------------------- | ------- | ------------------------------ |
| `toolType` | `ToolType`           | —       | Tool type (tool type)          |
| `function` | `FunctionDefinition` | —       | Function (function definition) |

---

#### Choice

| Field          | Type                     | Default | Description                   |
| -------------- | ------------------------ | ------- | ----------------------------- |
| `index`        | `int`                    | —       | Index                         |
| `message`      | `AssistantMessage`       | —       | Message (assistant message)   |
| `finishReason` | `Optional<FinishReason>` | `null`  | Finish reason (finish reason) |

---

#### CreateImageRequest

Request to create images from a text prompt.

| Field            | Type                | Default | Description     |
| ---------------- | ------------------- | ------- | --------------- |
| `prompt`         | `String`            | —       | Prompt          |
| `model`          | `Optional<String>`  | `null`  | Model           |
| `n`              | `Optional<Integer>` | `null`  | N               |
| `size`           | `Optional<String>`  | `null`  | Size in bytes   |
| `quality`        | `Optional<String>`  | `null`  | Quality         |
| `style`          | `Optional<String>`  | `null`  | Style           |
| `responseFormat` | `Optional<String>`  | `null`  | Response format |
| `user`           | `Optional<String>`  | `null`  | User            |

---

#### CreateSpeechRequest

Request to generate speech audio from text.

| Field            | Type               | Default | Description     |
| ---------------- | ------------------ | ------- | --------------- |
| `model`          | `String`           | —       | Model           |
| `input`          | `String`           | —       | Input           |
| `voice`          | `String`           | —       | Voice           |
| `responseFormat` | `Optional<String>` | `null`  | Response format |
| `speed`          | `Optional<Double>` | `null`  | Speed           |

---

#### CreateTranscriptionRequest

Request to transcribe audio into text.

| Field            | Type               | Default | Description                     |
| ---------------- | ------------------ | ------- | ------------------------------- |
| `model`          | `String`           | —       | Model                           |
| `file`           | `String`           | —       | Base64-encoded audio file data. |
| `language`       | `Optional<String>` | `null`  | Language                        |
| `prompt`         | `Optional<String>` | `null`  | Prompt                          |
| `responseFormat` | `Optional<String>` | `null`  | Response format                 |
| `temperature`    | `Optional<Double>` | `null`  | Temperature                     |

---

#### CustomProviderConfig

Configuration for registering a custom LLM provider at runtime.

| Field           | Type               | Default | Description                                                                 |
| --------------- | ------------------ | ------- | --------------------------------------------------------------------------- |
| `name`          | `String`           | —       | Unique name for this provider (e.g., "my-provider").                        |
| `baseUrl`       | `String`           | —       | Base URL for the provider's API (e.g., "<https://api.my-provider.com/v1">). |
| `authHeader`    | `AuthHeaderFormat` | —       | Authentication header format.                                               |
| `modelPrefixes` | `List<String>`     | —       | Model name prefixes that route to this provider (e.g., ["my-"]).            |

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

```java
public ChatCompletionResponse chat(ChatCompletionRequest req) throws Error
```

###### chatStream()

**Signature:**

```java
public String chatStream(ChatCompletionRequest req) throws Error
```

###### embed()

**Signature:**

```java
public EmbeddingResponse embed(EmbeddingRequest req) throws Error
```

###### listModels()

**Signature:**

```java
public ModelsListResponse listModels() throws Error
```

###### imageGenerate()

**Signature:**

```java
public ImagesResponse imageGenerate(CreateImageRequest req) throws Error
```

###### speech()

**Signature:**

```java
public byte[] speech(CreateSpeechRequest req) throws Error
```

###### transcribe()

**Signature:**

```java
public TranscriptionResponse transcribe(CreateTranscriptionRequest req) throws Error
```

###### moderate()

**Signature:**

```java
public ModerationResponse moderate(ModerationRequest req) throws Error
```

###### rerank()

**Signature:**

```java
public RerankResponse rerank(RerankRequest req) throws Error
```

###### search()

**Signature:**

```java
public SearchResponse search(SearchRequest req) throws Error
```

###### ocr()

**Signature:**

```java
public OcrResponse ocr(OcrRequest req) throws Error
```

###### createFile()

**Signature:**

```java
public String createFile(String req) throws Error
```

###### retrieveFile()

**Signature:**

```java
public String retrieveFile(String fileId) throws Error
```

###### deleteFile()

**Signature:**

```java
public String deleteFile(String fileId) throws Error
```

###### listFiles()

**Signature:**

```java
public String listFiles(String query) throws Error
```

###### fileContent()

**Signature:**

```java
public byte[] fileContent(String fileId) throws Error
```

###### createBatch()

**Signature:**

```java
public String createBatch(String req) throws Error
```

###### retrieveBatch()

**Signature:**

```java
public String retrieveBatch(String batchId) throws Error
```

###### listBatches()

**Signature:**

```java
public String listBatches(String query) throws Error
```

###### cancelBatch()

**Signature:**

```java
public String cancelBatch(String batchId) throws Error
```

###### createResponse()

**Signature:**

```java
public String createResponse(String req) throws Error
```

###### retrieveResponse()

**Signature:**

```java
public String retrieveResponse(String id) throws Error
```

###### cancelResponse()

**Signature:**

```java
public String cancelResponse(String id) throws Error
```

---

#### DeveloperMessage

| Field     | Type               | Default | Description                |
| --------- | ------------------ | ------- | -------------------------- |
| `content` | `String`           | —       | The extracted text content |
| `name`    | `Optional<String>` | `null`  | The name                   |

---

#### DocumentContent

| Field       | Type     | Default | Description                                      |
| ----------- | -------- | ------- | ------------------------------------------------ |
| `data`      | `String` | —       | Base64-encoded document data or URL.             |
| `mediaType` | `String` | —       | MIME type (e.g., "application/pdf", "text/csv"). |

---

#### EmbeddingObject

| Field       | Type           | Default | Description                                                                                                                                |
| ----------- | -------------- | ------- | ------------------------------------------------------------------------------------------------------------------------------------------ |
| `object`    | `String`       | —       | Always `"embedding"` from OpenAI-compatible APIs. Stored as a plain `String` so non-standard provider values do not break deserialization. |
| `embedding` | `List<Double>` | —       | Embedding                                                                                                                                  |
| `index`     | `int`          | —       | Index                                                                                                                                      |

---

#### EmbeddingRequest

| Field            | Type                        | Default | Description                        |
| ---------------- | --------------------------- | ------- | ---------------------------------- |
| `model`          | `String`                    | —       | Model                              |
| `input`          | `EmbeddingInput`            | —       | Input (embedding input)            |
| `encodingFormat` | `Optional<EmbeddingFormat>` | `null`  | Encoding format (embedding format) |
| `dimensions`     | `Optional<Integer>`         | `null`  | Dimensions                         |
| `user`           | `Optional<String>`          | `null`  | User                               |

---

#### EmbeddingResponse

| Field    | Type                    | Default | Description                                                                                                                           |
| -------- | ----------------------- | ------- | ------------------------------------------------------------------------------------------------------------------------------------- |
| `object` | `String`                | —       | Always `"list"` from OpenAI-compatible APIs. Stored as a plain `String` so non-standard provider values do not break deserialization. |
| `data`   | `List<EmbeddingObject>` | —       | Data                                                                                                                                  |
| `model`  | `String`                | —       | Model                                                                                                                                 |
| `usage`  | `Optional<Usage>`       | `null`  | Usage (usage)                                                                                                                         |

---

#### FunctionCall

| Field       | Type     | Default | Description |
| ----------- | -------- | ------- | ----------- |
| `name`      | `String` | —       | The name    |
| `arguments` | `String` | —       | Arguments   |

---

#### FunctionDefinition

| Field         | Type                | Default | Description                |
| ------------- | ------------------- | ------- | -------------------------- |
| `name`        | `String`            | —       | The name                   |
| `description` | `Optional<String>`  | `null`  | Human-readable description |
| `parameters`  | `Optional<Object>`  | `null`  | Parameters                 |
| `strict`      | `Optional<Boolean>` | `null`  | Strict                     |

---

#### FunctionMessage

Deprecated legacy function-role message body.

| Field     | Type     | Default | Description                |
| --------- | -------- | ------- | -------------------------- |
| `content` | `String` | —       | The extracted text content |
| `name`    | `String` | —       | The name                   |

---

#### Image

A single generated image, returned as either a URL or base64 data.

| Field           | Type               | Default | Description    |
| --------------- | ------------------ | ------- | -------------- |
| `url`           | `Optional<String>` | `null`  | Url            |
| `b64Json`       | `Optional<String>` | `null`  | B64 json       |
| `revisedPrompt` | `Optional<String>` | `null`  | Revised prompt |

---

#### ImageUrl

| Field    | Type                    | Default | Description           |
| -------- | ----------------------- | ------- | --------------------- |
| `url`    | `String`                | —       | Url                   |
| `detail` | `Optional<ImageDetail>` | `null`  | Detail (image detail) |

---

#### ImagesResponse

Response containing generated images.

| Field     | Type          | Default                   | Description |
| --------- | ------------- | ------------------------- | ----------- |
| `created` | `long`        | —                         | Created     |
| `data`    | `List<Image>` | `Collections.emptyList()` | Data        |

---

#### JsonSchemaFormat

| Field         | Type                | Default | Description                |
| ------------- | ------------------- | ------- | -------------------------- |
| `name`        | `String`            | —       | The name                   |
| `description` | `Optional<String>`  | `null`  | Human-readable description |
| `schema`      | `Object`            | —       | Schema                     |
| `strict`      | `Optional<Boolean>` | `null`  | Strict                     |

---

#### ModelObject

| Field     | Type     | Default | Description                                                                                                                            |
| --------- | -------- | ------- | -------------------------------------------------------------------------------------------------------------------------------------- |
| `id`      | `String` | —       | Unique identifier                                                                                                                      |
| `object`  | `String` | —       | Always `"model"` from OpenAI-compatible APIs. Stored as a plain `String` so non-standard provider values do not break deserialization. |
| `created` | `long`   | —       | Created                                                                                                                                |
| `ownedBy` | `String` | —       | Owned by                                                                                                                               |

---

#### ModelsListResponse

| Field    | Type                | Default                   | Description                                                                                                                           |
| -------- | ------------------- | ------------------------- | ------------------------------------------------------------------------------------------------------------------------------------- |
| `object` | `String`            | —                         | Always `"list"` from OpenAI-compatible APIs. Stored as a plain `String` so non-standard provider values do not break deserialization. |
| `data`   | `List<ModelObject>` | `Collections.emptyList()` | Data                                                                                                                                  |

---

#### ModerationCategories

Boolean flags for each moderation category.

| Field                   | Type      | Default | Description            |
| ----------------------- | --------- | ------- | ---------------------- |
| `sexual`                | `boolean` | —       | Sexual                 |
| `hate`                  | `boolean` | —       | Hate                   |
| `harassment`            | `boolean` | —       | Harassment             |
| `selfHarm`              | `boolean` | —       | Self harm              |
| `sexualMinors`          | `boolean` | —       | Sexual minors          |
| `hateThreatening`       | `boolean` | —       | Hate threatening       |
| `violenceGraphic`       | `boolean` | —       | Violence graphic       |
| `selfHarmIntent`        | `boolean` | —       | Self harm intent       |
| `selfHarmInstructions`  | `boolean` | —       | Self harm instructions |
| `harassmentThreatening` | `boolean` | —       | Harassment threatening |
| `violence`              | `boolean` | —       | Violence               |

---

#### ModerationCategoryScores

Confidence scores for each moderation category.

| Field                   | Type     | Default | Description            |
| ----------------------- | -------- | ------- | ---------------------- |
| `sexual`                | `double` | —       | Sexual                 |
| `hate`                  | `double` | —       | Hate                   |
| `harassment`            | `double` | —       | Harassment             |
| `selfHarm`              | `double` | —       | Self harm              |
| `sexualMinors`          | `double` | —       | Sexual minors          |
| `hateThreatening`       | `double` | —       | Hate threatening       |
| `violenceGraphic`       | `double` | —       | Violence graphic       |
| `selfHarmIntent`        | `double` | —       | Self harm intent       |
| `selfHarmInstructions`  | `double` | —       | Self harm instructions |
| `harassmentThreatening` | `double` | —       | Harassment threatening |
| `violence`              | `double` | —       | Violence               |

---

#### ModerationRequest

Request to classify content for policy violations.

| Field   | Type               | Default | Description              |
| ------- | ------------------ | ------- | ------------------------ |
| `input` | `ModerationInput`  | —       | Input (moderation input) |
| `model` | `Optional<String>` | `null`  | Model                    |

---

#### ModerationResponse

Response from the moderation endpoint.

| Field     | Type                     | Default | Description       |
| --------- | ------------------------ | ------- | ----------------- |
| `id`      | `String`                 | —       | Unique identifier |
| `model`   | `String`                 | —       | Model             |
| `results` | `List<ModerationResult>` | —       | Results           |

---

#### ModerationResult

A single moderation classification result.

| Field            | Type                       | Default | Description                                  |
| ---------------- | -------------------------- | ------- | -------------------------------------------- |
| `flagged`        | `boolean`                  | —       | Flagged                                      |
| `categories`     | `ModerationCategories`     | —       | Categories (moderation categories)           |
| `categoryScores` | `ModerationCategoryScores` | —       | Category scores (moderation category scores) |

---

#### OcrImage

An image extracted from an OCR page.

| Field         | Type               | Default | Description                |
| ------------- | ------------------ | ------- | -------------------------- |
| `id`          | `String`           | —       | Unique image identifier.   |
| `imageBase64` | `Optional<String>` | `null`  | Base64-encoded image data. |

---

#### OcrPage

A single page of OCR output.

| Field        | Type                       | Default | Description                                          |
| ------------ | -------------------------- | ------- | ---------------------------------------------------- |
| `index`      | `int`                      | —       | Page index (0-based).                                |
| `markdown`   | `String`                   | —       | Extracted content as Markdown.                       |
| `images`     | `Optional<List<OcrImage>>` | `null`  | Extracted images, if `include_image_base64` was set. |
| `dimensions` | `Optional<PageDimensions>` | `null`  | Page dimensions in pixels, if available.             |

---

#### OcrRequest

An OCR request.

| Field                | Type                      | Default | Description                                                      |
| -------------------- | ------------------------- | ------- | ---------------------------------------------------------------- |
| `model`              | `String`                  | —       | The model/provider to use (e.g. `"mistral/mistral-ocr-latest"`). |
| `document`           | `OcrDocument`             | —       | The document to process.                                         |
| `pages`              | `Optional<List<Integer>>` | `null`  | Specific pages to process (1-indexed). `null` means all pages.   |
| `includeImageBase64` | `Optional<Boolean>`       | `null`  | Whether to include base64-encoded images of each page.           |

---

#### OcrResponse

An OCR response.

| Field   | Type              | Default | Description                               |
| ------- | ----------------- | ------- | ----------------------------------------- |
| `pages` | `List<OcrPage>`   | —       | Extracted pages.                          |
| `model` | `String`          | —       | The model used.                           |
| `usage` | `Optional<Usage>` | `null`  | Token usage, if reported by the provider. |

---

#### PageDimensions

Page dimensions in pixels.

| Field    | Type  | Default | Description       |
| -------- | ----- | ------- | ----------------- |
| `width`  | `int` | —       | Width in pixels.  |
| `height` | `int` | —       | Height in pixels. |

---

#### PromptTokensDetails

Breakdown of tokens used in the prompt portion of a request.

`cached_tokens` is included in `Usage.prompt_tokens` — it is _not_ an
additional charge on top of the prompt token count. When pricing supports
a `cache_read_input_token_cost`, the cached portion is billed at the
discounted rate and the remainder at the regular input rate.

| Field          | Type   | Default | Description                                                          |
| -------------- | ------ | ------- | -------------------------------------------------------------------- |
| `cachedTokens` | `long` | —       | Cached tokens present in the prompt. Defaults to 0 when absent.      |
| `audioTokens`  | `long` | —       | Audio input tokens present in the prompt. Defaults to 0 when absent. |

---

#### RerankRequest

Request to rerank documents by relevance to a query.

| Field             | Type                   | Default | Description      |
| ----------------- | ---------------------- | ------- | ---------------- |
| `model`           | `String`               | —       | Model            |
| `query`           | `String`               | —       | Query            |
| `documents`       | `List<RerankDocument>` | —       | Documents        |
| `topN`            | `Optional<Integer>`    | `null`  | Top n            |
| `returnDocuments` | `Optional<Boolean>`    | `null`  | Return documents |

---

#### RerankResponse

Response from the rerank endpoint.

| Field     | Type                 | Default | Description       |
| --------- | -------------------- | ------- | ----------------- |
| `id`      | `Optional<String>`   | `null`  | Unique identifier |
| `results` | `List<RerankResult>` | —       | Results           |
| `meta`    | `Optional<Object>`   | `null`  | Meta              |

---

#### RerankResult

A single reranked document with its relevance score.

| Field            | Type                             | Default | Description                       |
| ---------------- | -------------------------------- | ------- | --------------------------------- |
| `index`          | `int`                            | —       | Index                             |
| `relevanceScore` | `double`                         | —       | Relevance score                   |
| `document`       | `Optional<RerankResultDocument>` | `null`  | Document (rerank result document) |

---

#### RerankResultDocument

The text content of a reranked document, returned when `return_documents` is true.

| Field  | Type     | Default | Description |
| ------ | -------- | ------- | ----------- |
| `text` | `String` | —       | Text        |

---

#### SearchRequest

A search request.

| Field                | Type                     | Default                   | Description                                                               |
| -------------------- | ------------------------ | ------------------------- | ------------------------------------------------------------------------- |
| `model`              | `String`                 | —                         | The model/provider to use (e.g. `"brave/web-search"`, `"tavily/search"`). |
| `query`              | `String`                 | —                         | The search query.                                                         |
| `maxResults`         | `Optional<Integer>`      | `null`                    | Maximum number of results to return.                                      |
| `searchDomainFilter` | `Optional<List<String>>` | `Collections.emptyList()` | Domain filter — restrict results to specific domains.                     |
| `country`            | `Optional<String>`       | `null`                    | Country code for localized results (ISO 3166-1 alpha-2).                  |

---

#### SearchResponse

A search response.

| Field     | Type                 | Default | Description         |
| --------- | -------------------- | ------- | ------------------- |
| `results` | `List<SearchResult>` | —       | The search results. |
| `model`   | `String`             | —       | The model used.     |

---

#### SearchResult

An individual search result.

| Field     | Type               | Default | Description                                     |
| --------- | ------------------ | ------- | ----------------------------------------------- |
| `title`   | `String`           | —       | Title of the result.                            |
| `url`     | `String`           | —       | URL of the result.                              |
| `snippet` | `String`           | —       | Text snippet / excerpt.                         |
| `date`    | `Optional<String>` | `null`  | Publication or last-updated date, if available. |

---

#### SpecificFunction

| Field  | Type     | Default | Description |
| ------ | -------- | ------- | ----------- |
| `name` | `String` | —       | The name    |

---

#### SpecificToolChoice

| Field        | Type               | Default             | Description                  |
| ------------ | ------------------ | ------------------- | ---------------------------- |
| `choiceType` | `ToolType`         | `ToolType.FUNCTION` | Choice type (tool type)      |
| `function`   | `SpecificFunction` | —                   | Function (specific function) |

---

#### StreamChoice

| Field          | Type                     | Default | Description                   |
| -------------- | ------------------------ | ------- | ----------------------------- |
| `index`        | `int`                    | —       | Index                         |
| `delta`        | `StreamDelta`            | —       | Delta (stream delta)          |
| `finishReason` | `Optional<FinishReason>` | `null`  | Finish reason (finish reason) |

---

#### StreamDelta

| Field          | Type                             | Default                   | Description                                                            |
| -------------- | -------------------------------- | ------------------------- | ---------------------------------------------------------------------- |
| `role`         | `Optional<String>`               | `null`                    | Role                                                                   |
| `content`      | `Optional<String>`               | `null`                    | The extracted text content                                             |
| `toolCalls`    | `Optional<List<StreamToolCall>>` | `Collections.emptyList()` | Tool calls                                                             |
| `functionCall` | `Optional<StreamFunctionCall>`   | `null`                    | Deprecated legacy function_call delta; retained for API compatibility. |
| `refusal`      | `Optional<String>`               | `null`                    | Refusal                                                                |

---

#### StreamFunctionCall

| Field       | Type               | Default | Description |
| ----------- | ------------------ | ------- | ----------- |
| `name`      | `Optional<String>` | `null`  | The name    |
| `arguments` | `Optional<String>` | `null`  | Arguments   |

---

#### StreamOptions

| Field          | Type                | Default | Description   |
| -------------- | ------------------- | ------- | ------------- |
| `includeUsage` | `Optional<Boolean>` | `null`  | Include usage |

---

#### StreamToolCall

| Field      | Type                           | Default | Description                     |
| ---------- | ------------------------------ | ------- | ------------------------------- |
| `index`    | `int`                          | —       | Index                           |
| `id`       | `Optional<String>`             | `null`  | Unique identifier               |
| `callType` | `Optional<ToolType>`           | `null`  | Call type (tool type)           |
| `function` | `Optional<StreamFunctionCall>` | `null`  | Function (stream function call) |

---

#### SystemMessage

| Field     | Type               | Default | Description                |
| --------- | ------------------ | ------- | -------------------------- |
| `content` | `String`           | —       | The extracted text content |
| `name`    | `Optional<String>` | `null`  | The name                   |

---

#### ToolCall

| Field      | Type           | Default | Description              |
| ---------- | -------------- | ------- | ------------------------ |
| `id`       | `String`       | —       | Unique identifier        |
| `callType` | `ToolType`     | —       | Call type (tool type)    |
| `function` | `FunctionCall` | —       | Function (function call) |

---

#### ToolMessage

| Field        | Type               | Default | Description                |
| ------------ | ------------------ | ------- | -------------------------- |
| `content`    | `String`           | —       | The extracted text content |
| `toolCallId` | `String`           | —       | Tool call id               |
| `name`       | `Optional<String>` | `null`  | The name                   |

---

#### TranscriptionResponse

Response from a transcription request.

| Field      | Type                                   | Default                   | Description |
| ---------- | -------------------------------------- | ------------------------- | ----------- |
| `text`     | `String`                               | —                         | Text        |
| `language` | `Optional<String>`                     | `null`                    | Language    |
| `duration` | `Optional<Double>`                     | `null`                    | Duration    |
| `segments` | `Optional<List<TranscriptionSegment>>` | `Collections.emptyList()` | Segments    |

---

#### TranscriptionSegment

A segment of transcribed audio with timing information.

| Field   | Type     | Default | Description       |
| ------- | -------- | ------- | ----------------- |
| `id`    | `int`    | —       | Unique identifier |
| `start` | `double` | —       | Start             |
| `end`   | `double` | —       | End               |
| `text`  | `String` | —       | Text              |

---

#### Usage

| Field                 | Type                            | Default | Description                                                                                                                                                                         |
| --------------------- | ------------------------------- | ------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `promptTokens`        | `long`                          | —       | Prompt tokens used. Defaults to 0 when absent (some providers omit this).                                                                                                           |
| `completionTokens`    | `long`                          | —       | Completion tokens used. Defaults to 0 when absent (e.g. embedding responses).                                                                                                       |
| `totalTokens`         | `long`                          | —       | Total tokens used. Defaults to 0 when absent (some providers omit this).                                                                                                            |
| `promptTokensDetails` | `Optional<PromptTokensDetails>` | `null`  | Breakdown of tokens used in the prompt, including cached tokens served at the provider's discounted cache-read rate. Absent when the provider does not return prompt-token details. |

---

#### UserMessage

| Field     | Type               | Default            | Description                |
| --------- | ------------------ | ------------------ | -------------------------- |
| `content` | `UserContent`      | `UserContent.TEXT` | The extracted text content |
| `name`    | `Optional<String>` | `null`             | The name                   |

---

### Enums

#### Message

A chat message in a conversation.

| Value       | Description                                                                                               |
| ----------- | --------------------------------------------------------------------------------------------------------- |
| `SYSTEM`    | System — Fields: `0`: `SystemMessage`                                                                     |
| `USER`      | User — Fields: `0`: `UserMessage`                                                                         |
| `ASSISTANT` | Assistant — Fields: `0`: `AssistantMessage`                                                               |
| `TOOL`      | Tool — Fields: `0`: `ToolMessage`                                                                         |
| `DEVELOPER` | Developer — Fields: `0`: `DeveloperMessage`                                                               |
| `FUNCTION`  | Deprecated legacy function-role message; retained for API compatibility. — Fields: `0`: `FunctionMessage` |

---

#### UserContent

| Value   | Description                              |
| ------- | ---------------------------------------- |
| `TEXT`  | Text format — Fields: `0`: `String`      |
| `PARTS` | Parts — Fields: `0`: `List<ContentPart>` |

---

#### ContentPart

| Value         | Description                                        |
| ------------- | -------------------------------------------------- |
| `TEXT`        | Text format — Fields: `text`: `String`             |
| `IMAGE_URL`   | Image url — Fields: `imageUrl`: `ImageUrl`         |
| `DOCUMENT`    | Document — Fields: `document`: `DocumentContent`   |
| `INPUT_AUDIO` | Input audio — Fields: `inputAudio`: `AudioContent` |

---

#### ImageDetail

| Value  | Description |
| ------ | ----------- |
| `LOW`  | Low         |
| `HIGH` | High        |
| `AUTO` | Auto        |

---

#### ToolType

The type discriminator for tool/tool-call objects.

Per the OpenAI spec this is always `"function"`. Using an enum enforces
that constraint at the type level and rejects any other value on
deserialization.

| Value      | Description |
| ---------- | ----------- |
| `FUNCTION` | Function    |

---

#### ToolChoice

| Value      | Description                                  |
| ---------- | -------------------------------------------- |
| `MODE`     | Mode — Fields: `0`: `ToolChoiceMode`         |
| `SPECIFIC` | Specific — Fields: `0`: `SpecificToolChoice` |

---

#### ToolChoiceMode

| Value      | Description |
| ---------- | ----------- |
| `AUTO`     | Auto        |
| `REQUIRED` | Required    |
| `NONE`     | None        |

---

#### ResponseFormat

| Value         | Description                                            |
| ------------- | ------------------------------------------------------ |
| `TEXT`        | Text format                                            |
| `JSON_OBJECT` | Json object                                            |
| `JSON_SCHEMA` | Json schema — Fields: `jsonSchema`: `JsonSchemaFormat` |

---

#### StopSequence

| Value      | Description                            |
| ---------- | -------------------------------------- |
| `SINGLE`   | Single — Fields: `0`: `String`         |
| `MULTIPLE` | Multiple — Fields: `0`: `List<String>` |

---

#### FinishReason

Why a choice stopped generating tokens.

| Value            | Description                                                                                                                                                                                                                                                                                                                                                                              |
| ---------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `STOP`           | Stop                                                                                                                                                                                                                                                                                                                                                                                     |
| `LENGTH`         | Length                                                                                                                                                                                                                                                                                                                                                                                   |
| `TOOL_CALLS`     | Tool calls                                                                                                                                                                                                                                                                                                                                                                               |
| `CONTENT_FILTER` | Content filter                                                                                                                                                                                                                                                                                                                                                                           |
| `FUNCTION_CALL`  | Deprecated legacy finish reason; retained for API compatibility.                                                                                                                                                                                                                                                                                                                         |
| `OTHER`          | Catch-all for unknown finish reasons returned by non-OpenAI providers. Note: this intentionally does **not** carry the original string (e.g. `Other(String)`). Using `#[serde(other)]` requires a unit variant, and switching to `#[serde(untagged)]` would change deserialization semantics for all variants. The original value can be recovered by inspecting the raw JSON if needed. |

---

#### ReasoningEffort

Controls how much reasoning effort the model should use.

| Value    | Description |
| -------- | ----------- |
| `LOW`    | Low         |
| `MEDIUM` | Medium      |
| `HIGH`   | High        |

---

#### EmbeddingFormat

The format in which the embedding vectors are returned.

| Value    | Description                                         |
| -------- | --------------------------------------------------- |
| `FLOAT`  | 32-bit floating-point numbers (default).            |
| `BASE64` | Base64-encoded string representation of the floats. |

---

#### EmbeddingInput

| Value      | Description                            |
| ---------- | -------------------------------------- |
| `SINGLE`   | Single — Fields: `0`: `String`         |
| `MULTIPLE` | Multiple — Fields: `0`: `List<String>` |

---

#### ModerationInput

Input to the moderation endpoint — a single string or multiple strings.

| Value      | Description                            |
| ---------- | -------------------------------------- |
| `SINGLE`   | Single — Fields: `0`: `String`         |
| `MULTIPLE` | Multiple — Fields: `0`: `List<String>` |

---

#### RerankDocument

A document to be reranked — either a plain string or an object with a text field.

| Value    | Description                         |
| -------- | ----------------------------------- |
| `TEXT`   | Text format — Fields: `0`: `String` |
| `OBJECT` | Object — Fields: `text`: `String`   |

---

#### OcrDocument

Document input for OCR — either a URL or inline base64 data.

| Value    | Description                                                                            |
| -------- | -------------------------------------------------------------------------------------- |
| `URL`    | A publicly accessible document URL. — Fields: `url`: `String`                          |
| `BASE64` | Inline base64-encoded document data. — Fields: `data`: `String`, `mediaType`: `String` |

---

#### AuthHeaderFormat

How the API key is sent in the HTTP request.

| Value     | Description                                                     |
| --------- | --------------------------------------------------------------- |
| `BEARER`  | Bearer token: `Authorization: Bearer <key>`                     |
| `API_KEY` | Custom header: e.g., `X-Api-Key: <key>` — Fields: `0`: `String` |
| `NONE`    | No authentication required.                                     |

---

### Errors

#### LiterLlmError

All errors that can occur when using `liter-llm`.

| Variant                   | Description                                                                                                                                                                                                                                                                                                                                                      |
| ------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `AUTHENTICATION`          | authentication failed: {message}                                                                                                                                                                                                                                                                                                                                 |
| `RATE_LIMITED`            | rate limited: {message}                                                                                                                                                                                                                                                                                                                                          |
| `BAD_REQUEST`             | bad request: {message}                                                                                                                                                                                                                                                                                                                                           |
| `CONTEXT_WINDOW_EXCEEDED` | context window exceeded: {message}                                                                                                                                                                                                                                                                                                                               |
| `CONTENT_POLICY`          | content policy violation: {message}                                                                                                                                                                                                                                                                                                                              |
| `NOT_FOUND`               | not found: {message}                                                                                                                                                                                                                                                                                                                                             |
| `SERVER_ERROR`            | server error: {message}                                                                                                                                                                                                                                                                                                                                          |
| `SERVICE_UNAVAILABLE`     | service unavailable: {message}                                                                                                                                                                                                                                                                                                                                   |
| `TIMEOUT`                 | request timeout                                                                                                                                                                                                                                                                                                                                                  |
| `STREAMING`               | A catch-all for errors that occur during streaming response processing. This variant covers multiple sub-conditions including UTF-8 decoding failures, CRC/checksum mismatches (AWS EventStream), JSON parse errors in individual SSE chunks, and buffer overflow conditions. The `message` field contains a human-readable description of the specific failure. |
| `ENDPOINT_NOT_SUPPORTED`  | provider {provider} does not support {endpoint}                                                                                                                                                                                                                                                                                                                  |
| `INVALID_HEADER`          | invalid header {name:?}: {reason}                                                                                                                                                                                                                                                                                                                                |
| `SERIALIZATION`           | serialization error: {0}                                                                                                                                                                                                                                                                                                                                         |
| `BUDGET_EXCEEDED`         | budget exceeded: {message}                                                                                                                                                                                                                                                                                                                                       |
| `HOOK_REJECTED`           | hook rejected: {message}                                                                                                                                                                                                                                                                                                                                         |
| `INTERNAL_ERROR`          | An internal logic error (e.g. unexpected Tower response variant). This should never surface in normal operation — if it does, it indicates a bug in the library.                                                                                                                                                                                                 |

---
