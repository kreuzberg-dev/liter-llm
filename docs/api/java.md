---
description: "liter-llm Java API reference"
---

# Java API Reference

The Java package is a pure-Java HTTP client using `java.net.http.HttpClient` (Java 17+). No FFI or native libraries required.

## Installation

```xml
<dependency>
    <groupId>dev.kreuzberg</groupId>
    <artifactId>liter-llm</artifactId>
    <version>1.0.0-rc.1</version>
</dependency>
```

## Client

### Builder

```java
import dev.kreuzberg.literllm.LlmClient;

var client = LlmClient.builder()
    .apiKey(System.getenv("OPENAI_API_KEY"))
    .baseUrl("https://api.openai.com/v1")  // default
    .maxRetries(2)                          // default
    .timeout(Duration.ofSeconds(60))        // default
    .cacheConfig(new CacheConfig(256, 300))
    .budgetConfig(new BudgetConfig(10.0, Map.of(), "hard"))
    .build();
```

`LlmClient` implements `AutoCloseable`. Use try-with-resources:

```java
try (var client = LlmClient.builder().apiKey("sk-...").build()) {
    // ...
}
```

| Builder Method | Default | Description |
|----------------|---------|-------------|
| `apiKey(key)` | `""` | API key for `Authorization: Bearer` |
| `baseUrl(url)` | `https://api.openai.com/v1` | Provider base URL |
| `maxRetries(n)` | `2` | Retry count for 429/5xx |
| `timeout(d)` | 60s | Connection timeout |
| `cacheConfig(cfg)` | `null` | Enable response caching with `CacheConfig(maxEntries, ttlSeconds)` |
| `budgetConfig(cfg)` | `null` | Enable cost budgeting with `BudgetConfig(globalLimit, modelLimits, enforcement)` |
| `cooldownSecs(n)` | `0` | Cooldown period in seconds after transient errors |
| `rateLimitConfig(cfg)` | `null` | Rate limiting with `RateLimitConfig(rpm, tpm)` |
| `healthCheckSecs(n)` | `0` | Health check interval in seconds |
| `costTracking(b)` | `false` | Enable per-request cost tracking |
| `tracing(b)` | `false` | Enable OpenTelemetry tracing spans |

### Hook Interface

Hooks let you observe or modify requests and responses:

```java
client.addHook(new LlmHook() {
    @Override
    public void onRequest(HookRequest request) {
        System.out.println("Request to model: " + request.model());
    }

    @Override
    public void onResponse(HookResponse response) {
        System.out.println("Tokens used: " + response.totalTokens());
    }

    @Override
    public void onError(LlmException error) {
        System.err.println("Error: " + error.getMessage());
    }
});
```

### Methods

All methods throw `LlmException` on failure.

#### `chat(request)`

Send a chat completion request.

```java
var request = Types.ChatCompletionRequest.builder(
    "gpt-4o-mini",
    List.of(new Types.UserMessage("Hello!"))
).maxTokens(256L).build();

var response = client.chat(request);
System.out.println(response.choices().getFirst().message().content());
```

#### `chatStream(request, handler)`

Send a streaming chat completion request. The handler is invoked once per chunk.

```java
client.chatStream(request, chunk -> {
    var choices = chunk.choices();
    if (!choices.isEmpty() && choices.getFirst().delta().content() != null) {
        System.out.print(choices.getFirst().delta().content());
    }
});
```

#### `embed(request)`

Send an embedding request.

```java
var response = client.embed(new EmbeddingRequest(
    "text-embedding-3-small",
    List.of("Hello, world!")
));
List<Double> vector = response.data().getFirst().embedding();
```

#### `listModels()`

List available models.

```java
var response = client.listModels();
for (var model : response.data()) {
    System.out.println(model.id());
}
```

#### `imageGenerate(request)`

Generate images from a text prompt.

```java
var response = client.imageGenerate(new CreateImageRequest(
    "A sunset over mountains",
    "dall-e-3",
    1,
    "1024x1024"
));
```

#### `speech(request)`

Generate speech audio from text. Returns raw audio bytes.

```java
byte[] audio = client.speech(new CreateSpeechRequest(
    "tts-1", "Hello, world!", "alloy"
));
Files.write(Path.of("output.mp3"), audio);
```

#### `transcribe(request)`

Transcribe audio into text.

```java
var response = client.transcribe(new CreateTranscriptionRequest(
    "whisper-1", audioBytes
));
System.out.println(response.text());
```

#### `moderate(request)`

Classify content for policy violations.

```java
var response = client.moderate(new ModerationRequest(
    "Some text to check", "text-moderation-latest"
));
```

#### `rerank(request)`

Rerank documents by relevance to a query.

```java
var response = client.rerank(new RerankRequest(
    "rerank-english-v3.0",
    "What is the capital of France?",
    List.of("Paris is the capital of France.", "Berlin is in Germany."),
    2
));
```

#### `search(request)`

Perform a web or document search across supported providers.

```java
var response = client.search(new SearchRequest(
    "brave/search",
    "latest AI news",
    10
));
```

#### `ocr(request)`

Extract text from documents or images using OCR with Markdown output.

```java
var response = client.ocr(new OcrRequest(
    "mistral/pixtral", fileBytes, "application/pdf"
));
```

#### `createFile(request)`

Upload a file.

```java
var file = client.createFile(new CreateFileRequest(
    fileBytes, "batch", "input.jsonl"
));
```

#### `retrieveFile(fileId)`

Retrieve metadata about an uploaded file.

```java
var file = client.retrieveFile("file-abc123");
```

#### `deleteFile(fileId)`

Delete an uploaded file.

```java
var response = client.deleteFile("file-abc123");
```

#### `listFiles(query)`

List uploaded files. Pass `null` to list all files.

```java
var response = client.listFiles(new FileListQuery("batch", null, null));
```

#### `fileContent(fileId)`

Download the content of an uploaded file.

```java
byte[] content = client.fileContent("file-abc123");
```

#### `createBatch(request)`

Create a new batch.

```java
var batch = client.createBatch(new CreateBatchRequest(
    "file-abc123", "/v1/chat/completions", "24h"
));
```

#### `retrieveBatch(batchId)`

Retrieve a batch by ID.

```java
var batch = client.retrieveBatch("batch-abc123");
```

#### `listBatches(query)`

List batches. Pass `null` to list all.

```java
var response = client.listBatches(null);
```

#### `cancelBatch(batchId)`

Cancel a batch.

```java
var batch = client.cancelBatch("batch-abc123");
```

#### `createResponse(request)`

Create a new response via the Responses API.

```java
var response = client.createResponse(new CreateResponseRequest(
    "gpt-4", "Summarize this text..."
));
```

#### `retrieveResponse(responseId)`

Retrieve a response by ID.

```java
var response = client.retrieveResponse("resp-abc123");
```

#### `cancelResponse(responseId)`

Cancel a response.

```java
var response = client.cancelResponse("resp-abc123");
```

#### `registerProvider(config)`

Register a custom provider at runtime.

```java
client.registerProvider(new ProviderConfig(
    "custom", "https://api.custom-llm.com/v1", "Authorization"
));
```

#### `unregisterProvider(prefix)`

Remove a previously registered provider.

```java
client.unregisterProvider("custom");
```

#### `getBudgetUsed()`

Return the total cost consumed so far (when budget tracking is enabled).

```java
double used = client.getBudgetUsed();
System.out.printf("Budget used: $%.4f%n", used);
```

## Types

Types are defined as Java records in `dev.kreuzberg.literllm.Types`. Messages use a sealed interface hierarchy.

### Message Types

```java
new Types.SystemMessage("You are a helpful assistant")
new Types.UserMessage("Hello!")
new Types.AssistantMessage("Hi there!")
new Types.ToolMessage(toolCallId, content)
```

### `ChatCompletionRequest`

Built with the static builder:

```java
var request = Types.ChatCompletionRequest.builder(
    "gpt-4o-mini",
    List.of(new Types.UserMessage("Hello!"))
).maxTokens(256L).build();
```

### `ChatCompletionResponse`

| Method | Type | Description |
|--------|------|-------------|
| `id()` | `String` | Response ID |
| `model()` | `String` | Model used |
| `choices()` | `List<Choice>` | Completion choices |
| `usage()` | `Usage` | Token usage |

### `Usage`

| Method | Type | Description |
|--------|------|-------------|
| `promptTokens()` | `int` | Tokens consumed by the prompt |
| `completionTokens()` | `int` | Tokens consumed by the completion |
| `totalTokens()` | `int` | Total tokens |

### `EmbeddingResponse`

| Method | Type | Description |
|--------|------|-------------|
| `data()` | `List<EmbeddingObject>` | Embedding vectors |
| `model()` | `String` | Model used |
| `usage()` | `Usage` | Token usage |

### `ModelsListResponse`

| Method | Type | Description |
|--------|------|-------------|
| `data()` | `List<ModelObject>` | Available models |

### `CacheConfig`

| Parameter | Type | Description |
|-----------|------|-------------|
| `maxEntries` | `int` | Maximum number of cached responses |
| `ttlSeconds` | `int` | Time-to-live in seconds for cache entries |

### `BudgetConfig`

| Parameter | Type | Description |
|-----------|------|-------------|
| `globalLimit` | `double` | Maximum total cost in dollars |
| `modelLimits` | `Map<String, Double>` | Per-model cost limits |
| `enforcement` | `String` | `"hard"` (reject) or `"soft"` (warn) |

## Error Handling

All errors extend `LlmException` with numeric error codes (1000+):

| Exception | Code | HTTP Status |
|-----------|------|-------------|
| `InvalidRequestException` | 1400 | 400, 422 |
| `AuthenticationException` | 1401 | 401, 403 |
| `NotFoundException` | 1404 | 404 |
| `RateLimitException` | 1429 | 429 |
| `ProviderException` | 1500 | 5xx |
| `StreamException` | 1600 | -- |
| `SerializationException` | 1700 | -- |

```java
try {
    var response = client.chat(request);
} catch (LlmException.RateLimitException e) {
    System.err.println("Rate limited: " + e.getMessage());
} catch (LlmException.AuthenticationException e) {
    System.err.println("Auth failed: " + e.getMessage());
} catch (LlmException e) {
    System.err.printf("Error %d: %s%n", e.getErrorCode(), e.getMessage());
}
```

## Example

```java
import dev.kreuzberg.literllm.LlmClient;
import dev.kreuzberg.literllm.Types.*;
import java.util.List;

try (var client = LlmClient.builder()
        .apiKey(System.getenv("OPENAI_API_KEY"))
        .build()) {

    // Non-streaming
    var request = ChatCompletionRequest.builder(
        "gpt-4o-mini",
        List.of(new UserMessage("Hello!"))
    ).maxTokens(256L).build();

    var response = client.chat(request);
    System.out.println(response.choices().getFirst().message().content());

    // Streaming
    client.chatStream(request, chunk -> {
        var choices = chunk.choices();
        if (!choices.isEmpty() && choices.getFirst().delta().content() != null) {
            System.out.print(choices.getFirst().delta().content());
        }
    });
}
```
