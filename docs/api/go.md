---
description: "liter-llm Go API reference"
---

# Go API Reference

The Go package is a pure-Go HTTP client that speaks the OpenAI-compatible wire protocol directly. No cgo or shared libraries required.

## Installation

```bash
go get github.com/kreuzberg-dev/liter-llm/go
```

## Client

### Constructor

```go
import literllm "github.com/kreuzberg-dev/liter-llm/go"

client := literllm.NewClient(
    literllm.WithAPIKey(os.Getenv("OPENAI_API_KEY")),
    literllm.WithBaseURL("https://api.groq.com/openai/v1"),
    literllm.WithTimeout(120 * time.Second),
    literllm.WithHTTPClient(customHTTPClient),
)
```

| Option | Description |
|--------|-------------|
| `WithAPIKey(key)` | API key sent as `Authorization: Bearer` header |
| `WithBaseURL(url)` | Override base URL (default: `https://api.openai.com/v1`) |
| `WithTimeout(d)` | Timeout on the default HTTP client (default: 120s) |
| `WithHTTPClient(hc)` | Replace the default `*http.Client` |

The `Client` is safe for concurrent use.

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

    literllm "github.com/kreuzberg-dev/liter-llm/go"
)

func main() {
    client := literllm.NewClient(
        literllm.WithAPIKey(os.Getenv("OPENAI_API_KEY")),
    )

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
}
```
