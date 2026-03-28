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
 resp, err := client.Moderate(context.Background(), &llm.CreateModerationRequest{
  Model: "openai/omni-moderation-latest",
  Input: "This is a test message.",
 })
 if err != nil {
  panic(err)
 }
 result := resp.Results[0]
 fmt.Printf("Flagged: %v\n", result.Flagged)
 for category, flagged := range result.Categories {
  if flagged {
   fmt.Printf("  %s: %.4f\n", category, result.CategoryScores[category])
  }
 }
}
```
