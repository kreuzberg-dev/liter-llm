---
title: "C# API Reference"
---

## C# API Reference <span class="version-badge">v1.3.0</span>

### Functions

#### CreateClient()

Create a new LLM client with simple scalar configuration.

This is the primary binding entry-point. All parameters except `api_key`
are optional — omitting them uses the same defaults as
`ClientConfigBuilder`.

**Errors:**

Returns `LiterLlmError` if the underlying HTTP client cannot be
constructed, or if the resolved provider configuration is invalid.

**Signature:**

```csharp
public static DefaultClient CreateClient(string apiKey, string? baseUrl = null, ulong? timeoutSecs = null, uint? maxRetries = null, string? modelHint = null)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `ApiKey` | `string` | Yes | The api key |
| `BaseUrl` | `string?` | No | The base url |
| `TimeoutSecs` | `ulong?` | No | The timeout secs |
| `MaxRetries` | `uint?` | No | The max retries |
| `ModelHint` | `string?` | No | The model hint |

**Returns:** `DefaultClient`

**Errors:** Throws `Error`.


---

#### CreateClientFromJson()

Create a new LLM client from a JSON string.

The JSON object accepts the same fields as `liter-llm.toml` (snake_case).

**Errors:**

Returns `LiterLlmError.BadRequest` if `json` is not valid JSON or
contains unknown fields.

**Signature:**

```csharp
public static DefaultClient CreateClientFromJson(string json)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `Json` | `string` | Yes | The json |

**Returns:** `DefaultClient`

**Errors:** Throws `Error`.


---

#### RegisterCustomProvider()

Register a custom provider in the global runtime registry.

The provider will be checked **before** all built-in providers during model
detection. If a provider with the same `name` already exists it is replaced.

**Errors:**

Returns an error if the config is invalid (empty name, empty base_url, or
no model prefixes).

**Signature:**

```csharp
public static void RegisterCustomProvider(CustomProviderConfig config)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `Config` | `CustomProviderConfig` | Yes | The configuration options |

**Returns:** `void`

**Errors:** Throws `Error`.


---

#### UnregisterCustomProvider()

Remove a previously registered custom provider by name.

Returns `true` if a provider with the given name was found and removed,
`false` if no such provider existed.

**Errors:**

Returns an error only if the internal lock is poisoned.

**Signature:**

```csharp
public static bool UnregisterCustomProvider(string name)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `Name` | `string` | Yes | The name |

**Returns:** `bool`

**Errors:** Throws `Error`.


---

### Types

#### AssistantMessage

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Content` | `string?` | `null` | The extracted text content |
| `Name` | `string?` | `null` | The name |
| `ToolCalls` | `List<ToolCall>?` | `new List<ToolCall>()` | Tool calls |
| `Refusal` | `string?` | `null` | Refusal |
| `FunctionCall` | `FunctionCall?` | `null` | Deprecated legacy function_call field; retained for API compatibility. |


---

#### AudioContent

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Data` | `string` | — | Base64-encoded audio data. |
| `Format` | `string` | — | Audio format (e.g., "wav", "mp3", "ogg"). |


---

#### ChatCompletionChunk

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Id` | `string` | — | Unique identifier |
| `Object` | `string` | — | Always `"chat.completion.chunk"` from OpenAI-compatible APIs.  Stored as a plain `String` so non-standard provider values do not fail parsing. |
| `Created` | `ulong` | — | Created |
| `Model` | `string` | — | Model |
| `Choices` | `List<StreamChoice>` | `new List<StreamChoice>()` | Choices |
| `Usage` | `Usage?` | `null` | Usage (usage) |
| `SystemFingerprint` | `string?` | `null` | System fingerprint |
| `ServiceTier` | `string?` | `null` | Service tier |


---

#### ChatCompletionRequest

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Model` | `string` | — | Model |
| `Messages` | `List<Message>` | `new List<Message>()` | Messages |
| `Temperature` | `double?` | `null` | Temperature |
| `TopP` | `double?` | `null` | Top p |
| `N` | `uint?` | `null` | N |
| `Stream` | `bool?` | `null` | Whether to stream the response. Managed by the client layer — do not set directly. |
| `Stop` | `StopSequence?` | `null` | Stop (stop sequence) |
| `MaxTokens` | `ulong?` | `null` | Maximum tokens |
| `PresencePenalty` | `double?` | `null` | Presence penalty |
| `FrequencyPenalty` | `double?` | `null` | Frequency penalty |
| `LogitBias` | `Dictionary<string, double>?` | `new Dictionary<string, double>()` | Token bias map.  Uses `BTreeMap` (sorted keys) for deterministic serialization order — important when hashing or signing requests. |
| `User` | `string?` | `null` | User |
| `Tools` | `List<ChatCompletionTool>?` | `new List<ChatCompletionTool>()` | Tools |
| `ToolChoice` | `ToolChoice?` | `null` | Tool choice (tool choice) |
| `ParallelToolCalls` | `bool?` | `null` | Parallel tool calls |
| `ResponseFormat` | `ResponseFormat?` | `null` | Response format (response format) |
| `StreamOptions` | `StreamOptions?` | `null` | Stream options (stream options) |
| `Seed` | `long?` | `null` | Seed |
| `ReasoningEffort` | `ReasoningEffort?` | `null` | Reasoning effort (reasoning effort) |
| `ExtraBody` | `object?` | `null` | Provider-specific extra parameters merged into the request body. Use for guardrails, safety settings, grounding config, etc. |


---

#### ChatCompletionResponse

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Id` | `string` | — | Unique identifier |
| `Object` | `string` | — | Always `"chat.completion"` from OpenAI-compatible APIs.  Stored as a plain `String` so non-standard provider values do not break deserialization. |
| `Created` | `ulong` | — | Created |
| `Model` | `string` | — | Model |
| `Choices` | `List<Choice>` | `new List<Choice>()` | Choices |
| `Usage` | `Usage?` | `null` | Usage (usage) |
| `SystemFingerprint` | `string?` | `null` | System fingerprint |
| `ServiceTier` | `string?` | `null` | Service tier |


---

#### ChatCompletionTool

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `ToolType` | `ToolType` | — | Tool type (tool type) |
| `Function` | `FunctionDefinition` | — | Function (function definition) |


---

#### Choice

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Index` | `uint` | — | Index |
| `Message` | `AssistantMessage` | — | Message (assistant message) |
| `FinishReason` | `FinishReason?` | `null` | Finish reason (finish reason) |


---

#### CreateImageRequest

Request to create images from a text prompt.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Prompt` | `string` | — | Prompt |
| `Model` | `string?` | `null` | Model |
| `N` | `uint?` | `null` | N |
| `Size` | `string?` | `null` | Size in bytes |
| `Quality` | `string?` | `null` | Quality |
| `Style` | `string?` | `null` | Style |
| `ResponseFormat` | `string?` | `null` | Response format |
| `User` | `string?` | `null` | User |


---

#### CreateSpeechRequest

Request to generate speech audio from text.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Model` | `string` | — | Model |
| `Input` | `string` | — | Input |
| `Voice` | `string` | — | Voice |
| `ResponseFormat` | `string?` | `null` | Response format |
| `Speed` | `double?` | `null` | Speed |


---

#### CreateTranscriptionRequest

Request to transcribe audio into text.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Model` | `string` | — | Model |
| `File` | `string` | — | Base64-encoded audio file data. |
| `Language` | `string?` | `null` | Language |
| `Prompt` | `string?` | `null` | Prompt |
| `ResponseFormat` | `string?` | `null` | Response format |
| `Temperature` | `double?` | `null` | Temperature |


---

#### CustomProviderConfig

Configuration for registering a custom LLM provider at runtime.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Name` | `string` | — | Unique name for this provider (e.g., "my-provider"). |
| `BaseUrl` | `string` | — | Base URL for the provider's API (e.g., "<https://api.my-provider.com/v1">). |
| `AuthHeader` | `AuthHeaderFormat` | — | Authentication header format. |
| `ModelPrefixes` | `List<string>` | — | Model name prefixes that route to this provider (e.g., ["my-"]). |


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

###### Chat()

**Signature:**

```csharp
public async Task<ChatCompletionResponse> ChatAsync(ChatCompletionRequest req)
```

###### ChatStream()

**Signature:**

```csharp
public async Task<string> ChatStreamAsync(ChatCompletionRequest req)
```

###### Embed()

**Signature:**

```csharp
public async Task<EmbeddingResponse> EmbedAsync(EmbeddingRequest req)
```

###### ListModels()

**Signature:**

```csharp
public async Task<ModelsListResponse> ListModelsAsync()
```

###### ImageGenerate()

**Signature:**

```csharp
public async Task<ImagesResponse> ImageGenerateAsync(CreateImageRequest req)
```

###### Transcribe()

**Signature:**

```csharp
public async Task<TranscriptionResponse> TranscribeAsync(CreateTranscriptionRequest req)
```

###### Moderate()

**Signature:**

```csharp
public async Task<ModerationResponse> ModerateAsync(ModerationRequest req)
```

###### Rerank()

**Signature:**

```csharp
public async Task<RerankResponse> RerankAsync(RerankRequest req)
```

###### Search()

**Signature:**

```csharp
public async Task<SearchResponse> SearchAsync(SearchRequest req)
```


---

#### DeveloperMessage

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Content` | `string` | — | The extracted text content |
| `Name` | `string?` | `null` | The name |


---

#### DocumentContent

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Data` | `string` | — | Base64-encoded document data or URL. |
| `MediaType` | `string` | — | MIME type (e.g., "application/pdf", "text/csv"). |


---

#### EmbeddingObject

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Object` | `string` | — | Always `"embedding"` from OpenAI-compatible APIs.  Stored as a plain `String` so non-standard provider values do not break deserialization. |
| `Embedding` | `List<double>` | — | Embedding |
| `Index` | `uint` | — | Index |


---

#### EmbeddingRequest

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Model` | `string` | — | Model |
| `Input` | `EmbeddingInput` | — | Input (embedding input) |
| `EncodingFormat` | `EmbeddingFormat?` | `null` | Encoding format (embedding format) |
| `Dimensions` | `uint?` | `null` | Dimensions |
| `User` | `string?` | `null` | User |


---

#### EmbeddingResponse

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Object` | `string` | — | Always `"list"` from OpenAI-compatible APIs.  Stored as a plain `String` so non-standard provider values do not break deserialization. |
| `Data` | `List<EmbeddingObject>` | — | Data |
| `Model` | `string` | — | Model |
| `Usage` | `Usage?` | `null` | Usage (usage) |


---

#### FunctionCall

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Name` | `string` | — | The name |
| `Arguments` | `string` | — | Arguments |


---

#### FunctionDefinition

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Name` | `string` | — | The name |
| `Description` | `string?` | `null` | Human-readable description |
| `Parameters` | `object?` | `null` | Parameters |
| `Strict` | `bool?` | `null` | Strict |


---

#### FunctionMessage

Deprecated legacy function-role message body.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Content` | `string` | — | The extracted text content |
| `Name` | `string` | — | The name |


---

#### Image

A single generated image, returned as either a URL or base64 data.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Url` | `string?` | `null` | Url |
| `B64Json` | `string?` | `null` | B64 json |
| `RevisedPrompt` | `string?` | `null` | Revised prompt |


---

#### ImageUrl

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Url` | `string` | — | Url |
| `Detail` | `ImageDetail?` | `null` | Detail (image detail) |


---

#### ImagesResponse

Response containing generated images.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Created` | `ulong` | — | Created |
| `Data` | `List<Image>` | `new List<Image>()` | Data |


---

#### JsonSchemaFormat

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Name` | `string` | — | The name |
| `Description` | `string?` | `null` | Human-readable description |
| `Schema` | `object` | — | Schema |
| `Strict` | `bool?` | `null` | Strict |


---

#### LiterLlmError


---

#### ModelObject

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Id` | `string` | — | Unique identifier |
| `Object` | `string` | — | Always `"model"` from OpenAI-compatible APIs.  Stored as a plain `String` so non-standard provider values do not break deserialization. |
| `Created` | `ulong` | — | Created |
| `OwnedBy` | `string` | — | Owned by |


---

#### ModelsListResponse

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Object` | `string` | — | Always `"list"` from OpenAI-compatible APIs.  Stored as a plain `String` so non-standard provider values do not break deserialization. |
| `Data` | `List<ModelObject>` | `new List<ModelObject>()` | Data |


---

#### ModerationCategories

Boolean flags for each moderation category.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Sexual` | `bool` | — | Sexual |
| `Hate` | `bool` | — | Hate |
| `Harassment` | `bool` | — | Harassment |
| `SelfHarm` | `bool` | — | Self harm |
| `SexualMinors` | `bool` | — | Sexual minors |
| `HateThreatening` | `bool` | — | Hate threatening |
| `ViolenceGraphic` | `bool` | — | Violence graphic |
| `SelfHarmIntent` | `bool` | — | Self harm intent |
| `SelfHarmInstructions` | `bool` | — | Self harm instructions |
| `HarassmentThreatening` | `bool` | — | Harassment threatening |
| `Violence` | `bool` | — | Violence |


---

#### ModerationCategoryScores

Confidence scores for each moderation category.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Sexual` | `double` | — | Sexual |
| `Hate` | `double` | — | Hate |
| `Harassment` | `double` | — | Harassment |
| `SelfHarm` | `double` | — | Self harm |
| `SexualMinors` | `double` | — | Sexual minors |
| `HateThreatening` | `double` | — | Hate threatening |
| `ViolenceGraphic` | `double` | — | Violence graphic |
| `SelfHarmIntent` | `double` | — | Self harm intent |
| `SelfHarmInstructions` | `double` | — | Self harm instructions |
| `HarassmentThreatening` | `double` | — | Harassment threatening |
| `Violence` | `double` | — | Violence |


---

#### ModerationRequest

Request to classify content for policy violations.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Input` | `ModerationInput` | — | Input (moderation input) |
| `Model` | `string?` | `null` | Model |


---

#### ModerationResponse

Response from the moderation endpoint.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Id` | `string` | — | Unique identifier |
| `Model` | `string` | — | Model |
| `Results` | `List<ModerationResult>` | — | Results |


---

#### ModerationResult

A single moderation classification result.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Flagged` | `bool` | — | Flagged |
| `Categories` | `ModerationCategories` | — | Categories (moderation categories) |
| `CategoryScores` | `ModerationCategoryScores` | — | Category scores (moderation category scores) |


---

#### OcrImage

An image extracted from an OCR page.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Id` | `string` | — | Unique image identifier. |
| `ImageBase64` | `string?` | `null` | Base64-encoded image data. |


---

#### OcrPage

A single page of OCR output.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Index` | `uint` | — | Page index (0-based). |
| `Markdown` | `string` | — | Extracted content as Markdown. |
| `Images` | `List<OcrImage>?` | `null` | Extracted images, if `include_image_base64` was set. |
| `Dimensions` | `PageDimensions?` | `null` | Page dimensions in pixels, if available. |


---

#### OcrRequest

An OCR request.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Model` | `string` | — | The model/provider to use (e.g. `"mistral/mistral-ocr-latest"`). |
| `Document` | `OcrDocument` | — | The document to process. |
| `Pages` | `List<uint>?` | `null` | Specific pages to process (1-indexed). `null` means all pages. |
| `IncludeImageBase64` | `bool?` | `null` | Whether to include base64-encoded images of each page. |


---

#### OcrResponse

An OCR response.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Pages` | `List<OcrPage>` | — | Extracted pages. |
| `Model` | `string` | — | The model used. |
| `Usage` | `Usage?` | `null` | Token usage, if reported by the provider. |


---

#### PageDimensions

Page dimensions in pixels.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Width` | `uint` | — | Width in pixels. |
| `Height` | `uint` | — | Height in pixels. |


---

#### RerankRequest

Request to rerank documents by relevance to a query.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Model` | `string` | — | Model |
| `Query` | `string` | — | Query |
| `Documents` | `List<RerankDocument>` | — | Documents |
| `TopN` | `uint?` | `null` | Top n |
| `ReturnDocuments` | `bool?` | `null` | Return documents |


---

#### RerankResponse

Response from the rerank endpoint.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Id` | `string?` | `null` | Unique identifier |
| `Results` | `List<RerankResult>` | — | Results |
| `Meta` | `object?` | `null` | Meta |


---

#### RerankResult

A single reranked document with its relevance score.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Index` | `uint` | — | Index |
| `RelevanceScore` | `double` | — | Relevance score |
| `Document` | `RerankResultDocument?` | `null` | Document (rerank result document) |


---

#### RerankResultDocument

The text content of a reranked document, returned when `return_documents` is true.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Text` | `string` | — | Text |


---

#### SearchRequest

A search request.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Model` | `string` | — | The model/provider to use (e.g. `"brave/web-search"`, `"tavily/search"`). |
| `Query` | `string` | — | The search query. |
| `MaxResults` | `uint?` | `null` | Maximum number of results to return. |
| `SearchDomainFilter` | `List<string>?` | `new List<string>()` | Domain filter — restrict results to specific domains. |
| `Country` | `string?` | `null` | Country code for localized results (ISO 3166-1 alpha-2). |


---

#### SearchResponse

A search response.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Results` | `List<SearchResult>` | — | The search results. |
| `Model` | `string` | — | The model used. |


---

#### SearchResult

An individual search result.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Title` | `string` | — | Title of the result. |
| `Url` | `string` | — | URL of the result. |
| `Snippet` | `string` | — | Text snippet / excerpt. |
| `Date` | `string?` | `null` | Publication or last-updated date, if available. |


---

#### SpecificFunction

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Name` | `string` | — | The name |


---

#### SpecificToolChoice

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `ChoiceType` | `ToolType` | `ToolType.Function` | Choice type (tool type) |
| `Function` | `SpecificFunction` | — | Function (specific function) |


---

#### StreamChoice

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Index` | `uint` | — | Index |
| `Delta` | `StreamDelta` | — | Delta (stream delta) |
| `FinishReason` | `FinishReason?` | `null` | Finish reason (finish reason) |


---

#### StreamDelta

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Role` | `string?` | `null` | Role |
| `Content` | `string?` | `null` | The extracted text content |
| `ToolCalls` | `List<StreamToolCall>?` | `new List<StreamToolCall>()` | Tool calls |
| `FunctionCall` | `StreamFunctionCall?` | `null` | Deprecated legacy function_call delta; retained for API compatibility. |
| `Refusal` | `string?` | `null` | Refusal |


---

#### StreamFunctionCall

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Name` | `string?` | `null` | The name |
| `Arguments` | `string?` | `null` | Arguments |


---

#### StreamOptions

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `IncludeUsage` | `bool?` | `null` | Include usage |


---

#### StreamToolCall

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Index` | `uint` | — | Index |
| `Id` | `string?` | `null` | Unique identifier |
| `CallType` | `ToolType?` | `null` | Call type (tool type) |
| `Function` | `StreamFunctionCall?` | `null` | Function (stream function call) |


---

#### SystemMessage

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Content` | `string` | — | The extracted text content |
| `Name` | `string?` | `null` | The name |


---

#### ToolCall

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Id` | `string` | — | Unique identifier |
| `CallType` | `ToolType` | — | Call type (tool type) |
| `Function` | `FunctionCall` | — | Function (function call) |


---

#### ToolMessage

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Content` | `string` | — | The extracted text content |
| `ToolCallId` | `string` | — | Tool call id |
| `Name` | `string?` | `null` | The name |


---

#### TranscriptionResponse

Response from a transcription request.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Text` | `string` | — | Text |
| `Language` | `string?` | `null` | Language |
| `Duration` | `double?` | `null` | Duration |
| `Segments` | `List<TranscriptionSegment>?` | `new List<TranscriptionSegment>()` | Segments |


---

#### TranscriptionSegment

A segment of transcribed audio with timing information.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Id` | `uint` | — | Unique identifier |
| `Start` | `double` | — | Start |
| `End` | `double` | — | End |
| `Text` | `string` | — | Text |


---

#### Usage

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `PromptTokens` | `ulong` | — | Prompt tokens used. Defaults to 0 when absent (some providers omit this). |
| `CompletionTokens` | `ulong` | — | Completion tokens used. Defaults to 0 when absent (e.g. embedding responses). |
| `TotalTokens` | `ulong` | — | Total tokens used. Defaults to 0 when absent (some providers omit this). |


---

#### UserMessage

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Content` | `UserContent` | `UserContent.Text` | The extracted text content |
| `Name` | `string?` | `null` | The name |


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
| `Text` | Text format — Fields: `0`: `string` |
| `Parts` | Parts — Fields: `0`: `List<ContentPart>` |


---

#### ContentPart

| Value | Description |
|-------|-------------|
| `Text` | Text format — Fields: `Text`: `string` |
| `ImageUrl` | Image url — Fields: `ImageUrl`: `ImageUrl` |
| `Document` | Document — Fields: `Document`: `DocumentContent` |
| `InputAudio` | Input audio — Fields: `InputAudio`: `AudioContent` |


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
| `JsonSchema` | Json schema — Fields: `JsonSchema`: `JsonSchemaFormat` |


---

#### StopSequence

| Value | Description |
|-------|-------------|
| `Single` | Single — Fields: `0`: `string` |
| `Multiple` | Multiple — Fields: `0`: `List<string>` |


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
| `Single` | Single — Fields: `0`: `string` |
| `Multiple` | Multiple — Fields: `0`: `List<string>` |


---

#### ModerationInput

Input to the moderation endpoint — a single string or multiple strings.

| Value | Description |
|-------|-------------|
| `Single` | Single — Fields: `0`: `string` |
| `Multiple` | Multiple — Fields: `0`: `List<string>` |


---

#### RerankDocument

A document to be reranked — either a plain string or an object with a text field.

| Value | Description |
|-------|-------------|
| `Text` | Text format — Fields: `0`: `string` |
| `Object` | Object — Fields: `Text`: `string` |


---

#### OcrDocument

Document input for OCR — either a URL or inline base64 data.

| Value | Description |
|-------|-------------|
| `Url` | A publicly accessible document URL. — Fields: `Url`: `string` |
| `Base64` | Inline base64-encoded document data. — Fields: `Data`: `string`, `MediaType`: `string` |


---

#### AuthHeaderFormat

How the API key is sent in the HTTP request.

| Value | Description |
|-------|-------------|
| `Bearer` | Bearer token: `Authorization: Bearer <key>` |
| `ApiKey` | Custom header: e.g., `X-Api-Key: <key>` — Fields: `0`: `string` |
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
