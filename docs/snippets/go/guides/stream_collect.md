```go
package main

import (
 "context"
 "fmt"
 "os"
 "strings"

 llm "github.com/kreuzberg-dev/liter-llm/go"
)

func main() {
 client := llm.NewClient(llm.WithAPIKey(os.Getenv("OPENAI_API_KEY")))
 var sb strings.Builder
 err := client.ChatStream(context.Background(), &llm.ChatCompletionRequest{
  Model: "openai/gpt-4o",
  Messages: []llm.Message{
   llm.NewTextMessage(llm.RoleUser, "Explain quantum computing briefly"),
  },
 }, func(chunk *llm.ChatCompletionChunk) error {
  if len(chunk.Choices) > 0 && chunk.Choices[0].Delta.Content != nil {
   delta := *chunk.Choices[0].Delta.Content
   sb.WriteString(delta)
   fmt.Print(delta)
  }
  return nil
 })
 if err != nil {
  panic(err)
 }
 fmt.Println()
 fmt.Printf("\nFull response length: %d characters\n", sb.Len())
}
```
