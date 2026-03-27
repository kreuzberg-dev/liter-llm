---
description: "liter-llm C# / .NET API reference"
---

# C# / .NET API Reference

The C# package is a pure .NET HTTP client targeting .NET 8+. No FFI or native libraries required.

## Installation

```bash
dotnet add package LiterLlm
```

## Client

### Constructor

```csharp
using LiterLlm;

var client = new LlmClient(
    apiKey: Environment.GetEnvironmentVariable("OPENAI_API_KEY")!,
    baseUrl: "https://api.openai.com/v1",  // default
    maxRetries: 2,                          // default
    timeout: TimeSpan.FromSeconds(60)       // default
);
```

`LlmClient` implements `IDisposable` and `IAsyncDisposable`:

```csharp
await using var client = new LlmClient(apiKey: "sk-...");
```

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `apiKey` | `string` | *required* | API key for `Authorization: Bearer` |
| `baseUrl` | `string` | `https://api.openai.com/v1` | Provider base URL |
| `maxRetries` | `int` | `2` | Retry count for 429/5xx |
| `timeout` | `TimeSpan?` | 60s | Request timeout |

### Methods

All methods are async and accept an optional `CancellationToken`.

#### `ChatAsync(request, ct)`

```csharp
Task<ChatCompletionResponse> ChatAsync(ChatCompletionRequest request, CancellationToken ct = default)
```

#### `EmbedAsync(request, ct)`

```csharp
Task<EmbeddingResponse> EmbedAsync(EmbeddingRequest request, CancellationToken ct = default)
```

#### `ListModelsAsync(ct)`

```csharp
Task<ModelsListResponse> ListModelsAsync(CancellationToken ct = default)
```

#### `ImageGenerateAsync(request, ct)`

```csharp
Task<ImagesResponse> ImageGenerateAsync(CreateImageRequest request, CancellationToken ct = default)
```

#### `SpeechAsync(request, ct)`

Returns raw audio bytes.

```csharp
Task<byte[]> SpeechAsync(CreateSpeechRequest request, CancellationToken ct = default)
```

#### `TranscribeAsync(request, ct)`

```csharp
Task<TranscriptionResponse> TranscribeAsync(CreateTranscriptionRequest request, CancellationToken ct = default)
```

#### `ModerateAsync(request, ct)`

```csharp
Task<ModerationResponse> ModerateAsync(ModerationRequest request, CancellationToken ct = default)
```

#### `RerankAsync(request, ct)`

```csharp
Task<RerankResponse> RerankAsync(RerankRequest request, CancellationToken ct = default)
```

#### `CreateFileAsync(request, ct)`

```csharp
Task<FileObject> CreateFileAsync(CreateFileRequest request, CancellationToken ct = default)
```

#### `RetrieveFileAsync(fileId, ct)`

```csharp
Task<FileObject> RetrieveFileAsync(string fileId, CancellationToken ct = default)
```

#### `DeleteFileAsync(fileId, ct)`

```csharp
Task<DeleteResponse> DeleteFileAsync(string fileId, CancellationToken ct = default)
```

#### `ListFilesAsync(query?, ct)`

```csharp
Task<FileListResponse> ListFilesAsync(FileListQuery? query = null, CancellationToken ct = default)
```

#### `FileContentAsync(fileId, ct)`

```csharp
Task<byte[]> FileContentAsync(string fileId, CancellationToken ct = default)
```

#### `CreateBatchAsync(request, ct)`

```csharp
Task<BatchObject> CreateBatchAsync(CreateBatchRequest request, CancellationToken ct = default)
```

#### `RetrieveBatchAsync(batchId, ct)`

```csharp
Task<BatchObject> RetrieveBatchAsync(string batchId, CancellationToken ct = default)
```

#### `ListBatchesAsync(query?, ct)`

```csharp
Task<BatchListResponse> ListBatchesAsync(BatchListQuery? query = null, CancellationToken ct = default)
```

#### `CancelBatchAsync(batchId, ct)`

```csharp
Task<BatchObject> CancelBatchAsync(string batchId, CancellationToken ct = default)
```

#### `CreateResponseAsync(request, ct)`

```csharp
Task<ResponseObject> CreateResponseAsync(CreateResponseRequest request, CancellationToken ct = default)
```

#### `RetrieveResponseAsync(responseId, ct)`

```csharp
Task<ResponseObject> RetrieveResponseAsync(string responseId, CancellationToken ct = default)
```

#### `CancelResponseAsync(responseId, ct)`

```csharp
Task<ResponseObject> CancelResponseAsync(string responseId, CancellationToken ct = default)
```

## Types

Types are C# records defined in the `LiterLlm` namespace, serialized with `System.Text.Json` using snake_case naming policy.

### `ChatCompletionRequest`

```csharp
var request = new ChatCompletionRequest(
    Model: "gpt-4o-mini",
    Messages: [new UserMessage("Hello!")],
    MaxTokens: 256
);
```

### `ChatCompletionResponse`

| Property | Type | Description |
|----------|------|-------------|
| `Id` | `string` | Response ID |
| `Model` | `string` | Model used |
| `Choices` | `Choice[]` | Completion choices |
| `Usage` | `Usage?` | Token usage |

## Error Handling

All errors derive from `LlmException` with numeric error codes:

| Exception | Code | HTTP Status |
|-----------|------|-------------|
| `InvalidRequestException` | 1400 | 400, 422 |
| `AuthenticationException` | 1401 | 401, 403 |
| `NotFoundException` | 1404 | 404 |
| `RateLimitException` | 1429 | 429 |
| `ProviderException` | 1500 | 5xx |
| `StreamException` | 1600 | -- |
| `SerializationException` | 1700 | -- |

```csharp
try
{
    var response = await client.ChatAsync(request);
}
catch (RateLimitException ex)
{
    Console.Error.WriteLine($"Rate limited: {ex.Message}");
}
catch (LlmException ex)
{
    Console.Error.WriteLine($"Error {ex.ErrorCode}: {ex.Message}");
}
```

## Example

```csharp
using LiterLlm;

await using var client = new LlmClient(
    apiKey: Environment.GetEnvironmentVariable("OPENAI_API_KEY")!);

var request = new ChatCompletionRequest(
    Model: "gpt-4o-mini",
    Messages: [new UserMessage("Hello!")],
    MaxTokens: 256);

var response = await client.ChatAsync(request);
Console.WriteLine(response.Choices[0].Message.Content);
```
