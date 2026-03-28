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
    timeout: TimeSpan.FromSeconds(60),      // default
    cacheConfig: new CacheConfig(256, 300),
    budgetConfig: new BudgetConfig(10.0, null, "hard")
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
| `cacheConfig` | `CacheConfig?` | `null` | Enable response caching with `CacheConfig(maxEntries, ttlSeconds)` |
| `budgetConfig` | `BudgetConfig?` | `null` | Enable cost budgeting with `BudgetConfig(globalLimit, modelLimits, enforcement)` |

### Hook Interface

Hooks let you observe or modify requests and responses:

```csharp
public interface ILlmHook
{
    Task OnRequestAsync(HookRequest request, CancellationToken ct = default);
    Task OnResponseAsync(HookResponse response, CancellationToken ct = default);
    Task OnErrorAsync(LlmException error, CancellationToken ct = default);
}
```

```csharp
client.AddHook(new MyHook());

class MyHook : ILlmHook
{
    public Task OnRequestAsync(HookRequest request, CancellationToken ct)
    {
        Console.WriteLine($"Request to model: {request.Model}");
        return Task.CompletedTask;
    }

    public Task OnResponseAsync(HookResponse response, CancellationToken ct)
    {
        Console.WriteLine($"Tokens used: {response.TotalTokens}");
        return Task.CompletedTask;
    }

    public Task OnErrorAsync(LlmException error, CancellationToken ct)
    {
        Console.Error.WriteLine($"Error: {error.Message}");
        return Task.CompletedTask;
    }
}
```

### Methods

All methods are async and accept an optional `CancellationToken`.

#### `ChatAsync(request, ct)`

Send a chat completion request.

```csharp
var request = new ChatCompletionRequest(
    Model: "gpt-4o-mini",
    Messages: [new UserMessage("Hello!")],
    MaxTokens: 256);

var response = await client.ChatAsync(request);
Console.WriteLine(response.Choices[0].Message.Content);
```

#### `ChatStreamAsync(request, ct)`

Start a streaming chat completion. Returns an `IAsyncEnumerable<ChatCompletionChunk>`.

```csharp
await foreach (var chunk in client.ChatStreamAsync(request))
{
    if (chunk.Choices[0].Delta.Content is { } content)
    {
        Console.Write(content);
    }
}
```

Supports cancellation via `CancellationToken`:

```csharp
var cts = new CancellationTokenSource(TimeSpan.FromSeconds(30));
await foreach (var chunk in client.ChatStreamAsync(request, cts.Token))
{
    Console.Write(chunk.Choices[0].Delta.Content);
}
```

#### `EmbedAsync(request, ct)`

Send an embedding request.

```csharp
var response = await client.EmbedAsync(new EmbeddingRequest(
    Model: "text-embedding-3-small",
    Input: ["Hello, world!"]));
double[] vector = response.Data[0].Embedding;
```

#### `ListModelsAsync(ct)`

List available models.

```csharp
var response = await client.ListModelsAsync();
foreach (var model in response.Data)
{
    Console.WriteLine(model.Id);
}
```

#### `ImageGenerateAsync(request, ct)`

Generate images from a text prompt.

```csharp
var response = await client.ImageGenerateAsync(new CreateImageRequest(
    Prompt: "A sunset over mountains",
    Model: "dall-e-3",
    N: 1,
    Size: "1024x1024"));
```

#### `SpeechAsync(request, ct)`

Generate speech audio from text. Returns raw audio bytes.

```csharp
byte[] audio = await client.SpeechAsync(new CreateSpeechRequest(
    Model: "tts-1", Input: "Hello, world!", Voice: "alloy"));
await File.WriteAllBytesAsync("output.mp3", audio);
```

#### `TranscribeAsync(request, ct)`

Transcribe audio into text.

```csharp
var response = await client.TranscribeAsync(new CreateTranscriptionRequest(
    Model: "whisper-1", File: audioBytes));
Console.WriteLine(response.Text);
```

#### `ModerateAsync(request, ct)`

Classify content for policy violations.

```csharp
var response = await client.ModerateAsync(new ModerationRequest(
    Input: "Some text to check", Model: "text-moderation-latest"));
```

#### `RerankAsync(request, ct)`

Rerank documents by relevance to a query.

```csharp
var response = await client.RerankAsync(new RerankRequest(
    Model: "rerank-english-v3.0",
    Query: "What is the capital of France?",
    Documents: ["Paris is the capital of France.", "Berlin is in Germany."],
    TopN: 2));
```

#### `CreateFileAsync(request, ct)`

Upload a file.

```csharp
var file = await client.CreateFileAsync(new CreateFileRequest(
    File: fileBytes, Purpose: "batch", Filename: "input.jsonl"));
```

#### `RetrieveFileAsync(fileId, ct)`

Retrieve metadata about an uploaded file.

```csharp
var file = await client.RetrieveFileAsync("file-abc123");
```

#### `DeleteFileAsync(fileId, ct)`

Delete an uploaded file.

```csharp
var response = await client.DeleteFileAsync("file-abc123");
```

#### `ListFilesAsync(query?, ct)`

List uploaded files. Pass `null` to list all.

```csharp
var response = await client.ListFilesAsync(new FileListQuery(Purpose: "batch"));
```

#### `FileContentAsync(fileId, ct)`

Download the content of an uploaded file.

```csharp
byte[] content = await client.FileContentAsync("file-abc123");
```

#### `CreateBatchAsync(request, ct)`

Create a new batch.

```csharp
var batch = await client.CreateBatchAsync(new CreateBatchRequest(
    InputFileId: "file-abc123",
    Endpoint: "/v1/chat/completions",
    CompletionWindow: "24h"));
```

#### `RetrieveBatchAsync(batchId, ct)`

Retrieve a batch by ID.

```csharp
var batch = await client.RetrieveBatchAsync("batch-abc123");
```

#### `ListBatchesAsync(query?, ct)`

List batches. Pass `null` to list all.

```csharp
var response = await client.ListBatchesAsync();
```

#### `CancelBatchAsync(batchId, ct)`

Cancel a batch.

```csharp
var batch = await client.CancelBatchAsync("batch-abc123");
```

#### `CreateResponseAsync(request, ct)`

Create a new response via the Responses API.

```csharp
var response = await client.CreateResponseAsync(new CreateResponseRequest(
    Model: "gpt-4", Input: "Summarize this text..."));
```

#### `RetrieveResponseAsync(responseId, ct)`

Retrieve a response by ID.

```csharp
var response = await client.RetrieveResponseAsync("resp-abc123");
```

#### `CancelResponseAsync(responseId, ct)`

Cancel a response.

```csharp
var response = await client.CancelResponseAsync("resp-abc123");
```

#### `RegisterProvider(config)`

Register a custom provider at runtime.

```csharp
client.RegisterProvider(new ProviderConfig(
    Prefix: "custom",
    BaseUrl: "https://api.custom-llm.com/v1",
    AuthHeader: "Authorization"));
```

#### `UnregisterProvider(prefix)`

Remove a previously registered provider.

```csharp
client.UnregisterProvider("custom");
```

#### `BudgetUsed`

Property returning the total cost consumed so far (when budget tracking is enabled).

```csharp
double used = client.BudgetUsed;
Console.WriteLine($"Budget used: ${used:F4}");
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

### `Usage`

| Property | Type | Description |
|----------|------|-------------|
| `PromptTokens` | `int` | Tokens consumed by the prompt |
| `CompletionTokens` | `int` | Tokens consumed by the completion |
| `TotalTokens` | `int` | Total tokens |

### `EmbeddingResponse`

| Property | Type | Description |
|----------|------|-------------|
| `Data` | `EmbeddingObject[]` | Embedding vectors |
| `Model` | `string` | Model used |
| `Usage` | `Usage` | Token usage |

### `ModelsListResponse`

| Property | Type | Description |
|----------|------|-------------|
| `Data` | `ModelObject[]` | Available models |

### `CacheConfig`

| Parameter | Type | Description |
|-----------|------|-------------|
| `MaxEntries` | `int` | Maximum number of cached responses |
| `TtlSeconds` | `int` | Time-to-live in seconds for cache entries |

### `BudgetConfig`

| Parameter | Type | Description |
|-----------|------|-------------|
| `GlobalLimit` | `double` | Maximum total cost in dollars |
| `ModelLimits` | `Dictionary<string, double>?` | Per-model cost limits |
| `Enforcement` | `string` | `"hard"` (reject) or `"soft"` (warn) |

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

// Non-streaming
var request = new ChatCompletionRequest(
    Model: "gpt-4o-mini",
    Messages: [new UserMessage("Hello!")],
    MaxTokens: 256);

var response = await client.ChatAsync(request);
Console.WriteLine(response.Choices[0].Message.Content);

// Streaming
await foreach (var chunk in client.ChatStreamAsync(request))
{
    if (chunk.Choices[0].Delta.Content is { } content)
    {
        Console.Write(content);
    }
}
```
