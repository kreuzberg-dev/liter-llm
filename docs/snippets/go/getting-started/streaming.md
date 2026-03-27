```go
package main

import (
 "context"
 "fmt"
 "os"

 llm "github.com/kreuzberg-dev/liter-llm/go"
)

func main() {
 client := llm.NewClient(llm.WithAPIKey(os.Getenv("OPENAI_API_KEY")))
 err := client.ChatStream(
  context.Background(),
  &llm.ChatCompletionRequest{
   Model: "openai/gpt-4o",
   Messages: []llm.Message{
    llm.NewTextMessage(llm.RoleUser, "Tell me a story"),
   },
  },
  func(chunk *llm.ChatCompletionChunk) error {
   if len(chunk.Choices) > 0 && chunk.Choices[0].Delta.Content != nil {
    fmt.Print(*chunk.Choices[0].Delta.Content)
   }
   return nil
  },
 )
 if err != nil {
  panic(err)
 }
 fmt.Println()
}
```
