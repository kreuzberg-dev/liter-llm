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
 client := llm.NewClient(llm.WithAPIKey(os.Getenv("BRAVE_API_KEY")))
 resp, err := client.Search(context.Background(), &llm.SearchRequest{
  Model:      "brave/web-search",
  Query:      "What is Rust programming language?",
  MaxResults: 5,
 })
 if err != nil {
  panic(err)
 }
 for _, result := range resp.Results {
  fmt.Printf("%s: %s\n", result.Title, result.URL)
 }
}
```
