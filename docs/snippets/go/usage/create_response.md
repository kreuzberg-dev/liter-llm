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
	resp, err := client.CreateResponse(context.Background(), &llm.CreateResponseRequest{
		Model: "openai/gpt-4o",
		Input: "Explain quantum computing in one sentence.",
	})
	if err != nil {
		panic(err)
	}
	fmt.Println(resp)
}
```
