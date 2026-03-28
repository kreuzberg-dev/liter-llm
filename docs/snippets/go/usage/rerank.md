<!-- snippet:compile-only -->

```go
package main

import (
 "context"
 "fmt"
 "os"

 llm "github.com/kreuzberg-dev/liter-llm/packages/go"
)

func main() {
 client := llm.NewClient(llm.WithAPIKey(os.Getenv("OPENAI_API_KEY")))
 resp, err := client.Rerank(context.Background(), &llm.RerankRequest{
  Model: "cohere/rerank-v3.5",
  Query: "What is the capital of France?",
  Documents: []string{
   "Paris is the capital of France.",
   "Berlin is the capital of Germany.",
   "London is the capital of England.",
  },
 })
 if err != nil {
  panic(err)
 }
 for _, result := range resp.Results {
  fmt.Printf("Index: %d, Score: %.4f\n", result.Index, result.RelevanceScore)
 }
}
```
