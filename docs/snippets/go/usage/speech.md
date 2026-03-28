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
 audioBytes, err := client.Speech(context.Background(), &llm.CreateSpeechRequest{
  Model: "openai/tts-1",
  Input: "Hello, world!",
  Voice: "alloy",
 })
 if err != nil {
  panic(err)
 }
 if err := os.WriteFile("output.mp3", audioBytes, 0644); err != nil {
  panic(err)
 }
 fmt.Printf("Wrote %d bytes to output.mp3\n", len(audioBytes))
}
```
