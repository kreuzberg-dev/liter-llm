```go
package main

import (
    "context"
    "errors"
    "fmt"
    "os"

    literllm "github.com/kreuzberg-dev/liter-llm/packages/go"
)

func main() {
    client := literllm.NewClient(
        literllm.WithAPIKey(os.Getenv("OPENAI_API_KEY")),
    )

    _, err := client.Chat(context.Background(), &literllm.ChatCompletionRequest{
        Model:    "openai/gpt-4o",
        Messages: []literllm.Message{literllm.NewTextMessage(literllm.RoleUser, "Hello")},
    })
    if err == nil {
        return
    }

    switch {
    case errors.Is(err, literllm.ErrAuthentication):
        // 401/403 — rotate the key.
        fmt.Println("auth failed:", err)
    case errors.Is(err, literllm.ErrRateLimit):
        // 429 — transient, back off and retry.
        fmt.Println("rate limited:", err)
    case errors.Is(err, literllm.ErrBudgetExceeded):
        fmt.Println("budget exceeded:", err)
    case errors.Is(err, literllm.ErrProviderError):
        // 5xx — transient on the proxy, terminal from the caller's view.
        fmt.Println("provider error:", err)
    default:
        // Inspect the underlying HTTP status when present.
        var apiErr *literllm.APIError
        if errors.As(err, &apiErr) {
            fmt.Printf("HTTP %d: %s\n", apiErr.StatusCode, apiErr.Message)
            return
        }
        fmt.Println("llm error:", err)
    }
}
```
