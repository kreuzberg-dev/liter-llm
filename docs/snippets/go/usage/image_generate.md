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
 resp, err := client.ImageGenerate(context.Background(), &llm.CreateImageRequest{
  Model:  "openai/dall-e-3",
  Prompt: "A sunset over mountains",
  N:      1,
  Size:   "1024x1024",
 })
 if err != nil {
  panic(err)
 }
 fmt.Println(resp.Data[0].URL)
}
```
