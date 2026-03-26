```go
package main

import (
 "context"
 "fmt"

 llm "github.com/kreuzberg-dev/liter-llm/go"
)

func main() {
 client := llm.NewClient()
 resp, err := client.Chat(context.Background(), &llm.ChatRequest{
  Model: "openai/gpt-4o",
  Messages: []llm.Message{
   {Role: "user", Content: "Hello!"},
  },
 })
 if err != nil {
  panic(err)
 }
 fmt.Println(resp.Content)
}
```
