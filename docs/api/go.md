---
description: "liter-llm Go API reference"
---

# Go API Reference

The Go package is a pure-Go HTTP client that speaks the OpenAI-compatible wire protocol directly. No cgo or shared libraries required.

## Installation

```bash
go get github.com/kreuzberg-dev/liter-llm/packages/go
```

## Client

### Constructor

```go
import literllm "github.com/kreuzberg-dev/liter-llm/packages/go"

client := literllm.NewClient(
    literllm.WithAPIKey(os.Getenv("OPENAI_API_KEY")),
    literllm.WithBaseURL("https://api.groq.com/openai/v1"),
    literllm.WithTimeout(120 * time.Second),
    literllm.WithHTTPClient(customHTTPClient),
    literllm.WithCache(literllm.CacheConfig{MaxEntries: 256, TTLSeconds: 300}),
    literllm.WithBudget(literllm.BudgetConfig{GlobalLimit: 10.0, Enforcement: "hard"}),
    literllm.WithHook(&myHook{}),
)
```

| Option | Description |
|--------|-------------|
| `WithAPIKey(key)` | API key sent as `Authorization: Bearer` header |
| `WithBaseURL(url)` | Override base URL (default: `https://api.openai.com/v1`) |
| `WithTimeout(d)` | Timeout on the default HTTP client (default: 120s) |
| `WithHTTPClient(hc)` | Replace the default `*http.Client` |
| `WithCache(cfg)` | Enable response caching with `CacheConfig{MaxEntries, TTLSeconds}` |
| `WithBudget(cfg)` | Enable cost budgeting with `BudgetConfig{GlobalLimit, ModelLimits, Enforcement}` |
| `WithHook(h)` | Register a lifecycle hook (implements `LlmHook` interface) |

The `Client` is safe for concurrent use.

### Hook Interface

Hooks let you observe or modify requests and responses at each stage:

```go
type LlmHook interface {
    OnRequest(ctx context.Context, req *HookRequest) error
    OnResponse(ctx context.Context, resp *HookResponse) error
    OnError(ctx context.Context, err error) error
}
```

```go
type myHook struct{}

func (h *myHook) OnRequest(ctx context.Context, req *literllm.HookRequest) error {
    log.Printf("Request to model: %s", req.Model)
    return nil
}
func (h *myHook) OnResponse(ctx context.Context, resp *literllm.HookResponse) error {
    log.Printf("Response tokens: %d", resp.TotalTokens)
    return nil
}
func (h *myHook) OnError(ctx context.Context, err error) error {
    log.Printf("Error: %v", err)
    return nil
}
```

### Interface

All methods on `Client` satisfy the `LlmClient` interface:

```go
type LlmClient interface {
    Chat(ctx context.Context, req *ChatCompletionRequest) (*ChatCompletionResponse, error)
    ChatStream(ctx context.Context, req *ChatCompletionRequest, handler func(*ChatCompletionChunk) error) error
    Embed(ctx context.Context, req *EmbeddingRequest) (*EmbeddingResponse, error)
    ListModels(ctx context.Context) (*ModelsListResponse, error)
    ImageGenerate(ctx context.Context, req *CreateImageRequest) (*ImagesResponse, error)
    Speech(ctx context.Context, req *CreateSpeechRequest) ([]byte, error)
    Transcribe(ctx context.Context, req *CreateTranscriptionRequest) (*TranscriptionResponse, error)
    Moderate(ctx context.Context, req *ModerationRequest) (*ModerationResponse, error)
    Rerank(ctx context.Context, req *RerankRequest) (*RerankResponse, error)
    CreateFile(ctx context.Context, req *CreateFileRequest) (*FileObject, error)
    RetrieveFile(ctx context.Context, fileID string) (*FileObject, error)
    DeleteFile(ctx context.Context, fileID string) (*DeleteResponse, error)
    ListFiles(ctx context.Context, query *FileListQuery) (*FileListResponse, error)
    FileContent(ctx context.Context, fileID string) ([]byte, error)
    CreateBatch(ctx context.Context, req *CreateBatchRequest) (*BatchObject, error)
    RetrieveBatch(ctx context.Context, batchID string) (*BatchObject, error)
    ListBatches(ctx context.Context, query *BatchListQuery) (*BatchListResponse, error)
    CancelBatch(ctx context.Context, batchID string) (*BatchObject, error)
    CreateResponse(ctx context.Context, req *CreateResponseRequest) (*ResponseObject, error)
    RetrieveResponse(ctx context.Context, responseID string) (*ResponseObject, error)
    CancelResponse(ctx context.Context, responseID string) (*ResponseObject, error)
    RegisterProvider(config *ProviderConfig) error
    UnregisterProvider(prefix string) error
    BudgetUsed() float64
}
```

### Methods

#### `Chat(ctx, req)`

Send a non-streaming chat completion request.

```go
resp, err := client.Chat(ctx, &literllm.ChatCompletionRequest{
    Model:    "gpt-4",
    Messages: []literllm.Message{literllm.NewTextMessage(literllm.RoleUser, "Hello!")},
})
```

#### `ChatStream(ctx, req, handler)`

Send a streaming chat completion request. The handler is invoked once per SSE chunk. Cancel `ctx` to abort early.

```go
err := client.ChatStream(ctx, &literllm.ChatCompletionRequest{
    Model:    "gpt-4",
    Messages: []literllm.Message{literllm.NewTextMessage(literllm.RoleUser, "Hello!")},
}, func(chunk *literllm.ChatCompletionChunk) error {
    if len(chunk.Choices) > 0 && chunk.Choices[0].Delta.Content != nil {
        fmt.Print(*chunk.Choices[0].Delta.Content)
    }
    return nil
})
```

#### `Embed(ctx, req)`

Send an embedding request.

```go
resp, err := client.Embed(ctx, &literllm.EmbeddingRequest{
    Model: "text-embedding-3-small",
    Input: literllm.NewEmbeddingInputSingle("Hello"),
})
```

#### `ListModels(ctx)`

List available models.

```go
resp, err := client.ListModels(ctx)
for _, m := range resp.Data {
    fmt.Println(m.ID)
}
```

#### `ImageGenerate(ctx, req)`

Generate images from a text prompt.

```go
resp, err := client.ImageGenerate(ctx, &literllm.CreateImageRequest{
    Prompt: "A sunset over mountains",
    Model:  "dall-e-3",
    N:      1,
    Size:   "1024x1024",
})
```

#### `Speech(ctx, req)`

Generate speech audio from text. Returns raw audio bytes.

```go
audio, err := client.Speech(ctx, &literllm.CreateSpeechRequest{
    Model: "tts-1",
    Input: "Hello, world!",
    Voice: "alloy",
})
os.WriteFile("output.mp3", audio, 0644)
```

#### `Transcribe(ctx, req)`

Transcribe audio into text.

```go
resp, err := client.Transcribe(ctx, &literllm.CreateTranscriptionRequest{
    Model: "whisper-1",
    File:  audioBytes,
})
fmt.Println(resp.Text)
```

#### `Moderate(ctx, req)`

Classify content for policy violations.

```go
resp, err := client.Moderate(ctx, &literllm.ModerationRequest{
    Input: "Some text to check",
    Model: "text-moderation-latest",
})
```

#### `Rerank(ctx, req)`

Rerank documents by relevance to a query.

```go
resp, err := client.Rerank(ctx, &literllm.RerankRequest{
    Model:     "rerank-english-v3.0",
    Query:     "What is the capital of France?",
    Documents: []string{"Paris is the capital of France.", "Berlin is in Germany."},
    TopN:      2,
})
```

#### `CreateFile(ctx, req)`

Upload a file.

```go
file, err := client.CreateFile(ctx, &literllm.CreateFileRequest{
    File:     fileBytes,
    Purpose:  "batch",
    Filename: "input.jsonl",
})
```

#### `RetrieveFile(ctx, fileID)`

Retrieve metadata about an uploaded file.

```go
file, err := client.RetrieveFile(ctx, "file-abc123")
```

#### `DeleteFile(ctx, fileID)`

Delete an uploaded file.

```go
resp, err := client.DeleteFile(ctx, "file-abc123")
```

#### `ListFiles(ctx, query)`

List uploaded files. Pass `nil` to list all.

```go
resp, err := client.ListFiles(ctx, &literllm.FileListQuery{Purpose: "batch"})
```

#### `FileContent(ctx, fileID)`

Download the content of an uploaded file.

```go
content, err := client.FileContent(ctx, "file-abc123")
```

#### `CreateBatch(ctx, req)`

Create a new batch.

```go
batch, err := client.CreateBatch(ctx, &literllm.CreateBatchRequest{
    InputFileID:      "file-abc123",
    Endpoint:         "/v1/chat/completions",
    CompletionWindow: "24h",
})
```

#### `RetrieveBatch(ctx, batchID)`

Retrieve a batch by ID.

```go
batch, err := client.RetrieveBatch(ctx, "batch-abc123")
```

#### `ListBatches(ctx, query)`

List batches. Pass `nil` to list all.

```go
resp, err := client.ListBatches(ctx, nil)
```

#### `CancelBatch(ctx, batchID)`

Cancel a batch.

```go
batch, err := client.CancelBatch(ctx, "batch-abc123")
```

#### `CreateResponse(ctx, req)`

Create a new response via the Responses API.

```go
resp, err := client.CreateResponse(ctx, &literllm.CreateResponseRequest{
    Model: "gpt-4",
    Input: "Summarize this text...",
})
```

#### `RetrieveResponse(ctx, responseID)`

Retrieve a response by ID.

```go
resp, err := client.RetrieveResponse(ctx, "resp-abc123")
```

#### `CancelResponse(ctx, responseID)`

Cancel a response.

```go
resp, err := client.CancelResponse(ctx, "resp-abc123")
```

#### `RegisterProvider(config)`

Register a custom provider at runtime.

```go
err := client.RegisterProvider(&literllm.ProviderConfig{
    Prefix:  "custom",
    BaseURL: "https://api.custom-llm.com/v1",
    AuthHeader: "Authorization",
})
```

#### `UnregisterProvider(prefix)`

Remove a previously registered provider.

```go
err := client.UnregisterProvider("custom")
```

#### `BudgetUsed()`

Return the total cost consumed so far (when budget tracking is enabled).

```go
used := client.BudgetUsed()
fmt.Printf("Budget used: $%.4f\n", used)
```

## Types

### Message Helpers

```go
literllm.NewTextMessage(literllm.RoleUser, "Hello!")
literllm.NewPartsMessage(literllm.RoleUser, []literllm.ContentPart{...})
```

### `ChatCompletionResponse`

| Field | Type | JSON |
|-------|------|------|
| `ID` | `string` | `id` |
| `Model` | `string` | `model` |
| `Choices` | `[]Choice` | `choices` |
| `Usage` | `*Usage` | `usage` |
| `Created` | `uint64` | `created` |

### `ChatCompletionChunk`

| Field | Type | JSON |
|-------|------|------|
| `ID` | `string` | `id` |
| `Model` | `string` | `model` |
| `Choices` | `[]StreamChoice` | `choices` |
| `Usage` | `*Usage` | `usage` |

### `Usage`

| Field | Type | JSON |
|-------|------|------|
| `PromptTokens` | `int` | `prompt_tokens` |
| `CompletionTokens` | `int` | `completion_tokens` |
| `TotalTokens` | `int` | `total_tokens` |

### `EmbeddingResponse`

| Field | Type | JSON |
|-------|------|------|
| `Data` | `[]EmbeddingObject` | `data` |
| `Model` | `string` | `model` |
| `Usage` | `Usage` | `usage` |

### `ModelsListResponse`

| Field | Type | JSON |
|-------|------|------|
| `Data` | `[]ModelObject` | `data` |

### `CacheConfig`

| Field | Type | Description |
|-------|------|-------------|
| `MaxEntries` | `int` | Maximum number of cached responses |
| `TTLSeconds` | `int` | Time-to-live in seconds for cache entries |

### `BudgetConfig`

| Field | Type | Description |
|-------|------|-------------|
| `GlobalLimit` | `float64` | Maximum total cost in dollars |
| `ModelLimits` | `map[string]float64` | Per-model cost limits |
| `Enforcement` | `string` | `"hard"` (reject) or `"soft"` (warn) |

## Error Handling

Errors use Go sentinel errors for `errors.Is` matching:

```go
var (
    ErrInvalidRequest = errors.New("literllm: invalid request")
    ErrAuthentication  = errors.New("literllm: authentication failed")
    ErrRateLimit       = errors.New("literllm: rate limit exceeded")
    ErrNotFound        = errors.New("literllm: not found")
    ErrProviderError   = errors.New("literllm: provider error")
    ErrStream          = errors.New("literllm: stream error")
)
```

Use `errors.Is` for programmatic handling:

```go
resp, err := client.Chat(ctx, req)
if errors.Is(err, literllm.ErrRateLimit) {
    // back off and retry
}
```

The `*APIError` type provides `StatusCode` and `Message` for HTTP errors:

```go
var apiErr *literllm.APIError
if errors.As(err, &apiErr) {
    fmt.Printf("HTTP %d: %s\n", apiErr.StatusCode, apiErr.Message)
}
```

## Example

```go
package main

import (
    "context"
    "fmt"
    "os"

    literllm "github.com/kreuzberg-dev/liter-llm/packages/go"
)

func main() {
    client := literllm.NewClient(
        literllm.WithAPIKey(os.Getenv("OPENAI_API_KEY")),
    )

    // Non-streaming
    resp, err := client.Chat(context.Background(), &literllm.ChatCompletionRequest{
        Model:    "gpt-4",
        Messages: []literllm.Message{literllm.NewTextMessage(literllm.RoleUser, "Hello!")},
    })
    if err != nil {
        panic(err)
    }
    if len(resp.Choices) > 0 && resp.Choices[0].Message.Content != nil {
        fmt.Println(*resp.Choices[0].Message.Content)
    }

    // Streaming
    err = client.ChatStream(context.Background(), &literllm.ChatCompletionRequest{
        Model:    "gpt-4",
        Messages: []literllm.Message{literllm.NewTextMessage(literllm.RoleUser, "Tell me a joke")},
    }, func(chunk *literllm.ChatCompletionChunk) error {
        if len(chunk.Choices) > 0 && chunk.Choices[0].Delta.Content != nil {
            fmt.Print(*chunk.Choices[0].Delta.Content)
        }
        return nil
    })
    if err != nil {
        panic(err)
    }
}
```
