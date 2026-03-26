```go
package main

import (
 "context"
 "fmt"

 llm "github.com/kreuzberg-dev/liter-llm/go"
)

func main() {
 client := llm.NewClient()
 stream, err := client.ChatStream(context.Background(), &llm.ChatRequest{
  Model: "openai/gpt-4o",
  Messages: []llm.Message{
   {Role: "user", Content: "Tell me a story"},
  },
 })
 if err != nil {
  panic(err)
 }
 defer stream.Close()

 for chunk := range stream.Chunks() {
  fmt.Print(chunk.Delta)
 }
 fmt.Println()
}
```
