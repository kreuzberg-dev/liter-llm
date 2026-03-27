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
 resp, err := client.Embed(context.Background(), &llm.EmbeddingRequest{
  Model: "openai/text-embedding-3-small",
  Input: llm.NewEmbeddingInputMultiple([]string{"The quick brown fox jumps over the lazy dog"}),
 })
 if err != nil {
  panic(err)
 }
 fmt.Printf("Dimensions: %d\n", len(resp.Data[0].Embedding))
 fmt.Printf("First 5 values: %v\n", resp.Data[0].Embedding[:5])
}
```
