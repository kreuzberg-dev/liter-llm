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
 resp, err := client.CreateBatch(context.Background(), &llm.CreateBatchRequest{
  InputFileID:      "file-abc123",
  Endpoint:         "/v1/chat/completions",
  CompletionWindow: "24h",
 })
 if err != nil {
  panic(err)
 }
 fmt.Printf("Batch ID: %s\n", resp.ID)
 fmt.Printf("Status: %s\n", resp.Status)
}
```
