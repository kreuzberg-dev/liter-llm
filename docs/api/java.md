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

### Methods

All methods throw `LlmException` on failure.

#### `chat(request)`

```java
public ChatCompletionResponse chat(ChatCompletionRequest request) throws LlmException
```

#### `embed(request)`

```java
public EmbeddingResponse embed(EmbeddingRequest request) throws LlmException
```

#### `listModels()`

```java
public ModelsListResponse listModels() throws LlmException
```

#### `imageGenerate(request)`

```java
public ImagesResponse imageGenerate(CreateImageRequest request) throws LlmException
```

#### `speech(request)`

Returns raw audio bytes.

```java
public byte[] speech(CreateSpeechRequest request) throws LlmException
```

#### `transcribe(request)`

```java
public TranscriptionResponse transcribe(CreateTranscriptionRequest request) throws LlmException
```

#### `moderate(request)`

```java
public ModerationResponse moderate(ModerationRequest request) throws LlmException
```

#### `rerank(request)`

```java
public RerankResponse rerank(RerankRequest request) throws LlmException
```

#### `createFile(request)`

```java
public FileObject createFile(CreateFileRequest request) throws LlmException
```

#### `retrieveFile(fileId)`

```java
public FileObject retrieveFile(String fileId) throws LlmException
```

#### `deleteFile(fileId)`

```java
public DeleteResponse deleteFile(String fileId) throws LlmException
```

#### `listFiles(query)`

```java
public FileListResponse listFiles(FileListQuery query) throws LlmException
```

Pass `null` to list all files.

#### `fileContent(fileId)`

```java
public byte[] fileContent(String fileId) throws LlmException
```

#### `createBatch(request)`

```java
public BatchObject createBatch(CreateBatchRequest request) throws LlmException
```

#### `retrieveBatch(batchId)`

```java
public BatchObject retrieveBatch(String batchId) throws LlmException
```

#### `listBatches(query)`

```java
public BatchListResponse listBatches(BatchListQuery query) throws LlmException
```

#### `cancelBatch(batchId)`

```java
public BatchObject cancelBatch(String batchId) throws LlmException
```

#### `createResponse(request)`

```java
public ResponseObject createResponse(CreateResponseRequest request) throws LlmException
```

#### `retrieveResponse(responseId)`

```java
public ResponseObject retrieveResponse(String responseId) throws LlmException
```

#### `cancelResponse(responseId)`

```java
public ResponseObject cancelResponse(String responseId) throws LlmException
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

    var request = ChatCompletionRequest.builder(
        "gpt-4o-mini",
        List.of(new UserMessage("Hello!"))
    ).maxTokens(256L).build();

    var response = client.chat(request);
    System.out.println(response.choices().getFirst().message().content());
}
```
