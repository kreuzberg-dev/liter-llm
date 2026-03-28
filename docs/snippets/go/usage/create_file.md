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
 fileBytes, err := os.ReadFile("data.jsonl")
 if err != nil {
  panic(err)
 }
 resp, err := client.CreateFile(context.Background(), &llm.CreateFileRequest{
  File:     fileBytes,
  Filename: "data.jsonl",
  Purpose:  "batch",
 })
 if err != nil {
  panic(err)
 }
 fmt.Printf("File ID: %s\n", resp.ID)
 fmt.Printf("Size: %d bytes\n", resp.Bytes)
}
```
