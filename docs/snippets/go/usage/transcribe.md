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
 audioBytes, err := os.ReadFile("audio.mp3")
 if err != nil {
  panic(err)
 }
 resp, err := client.Transcribe(context.Background(), &llm.CreateTranscriptionRequest{
  Model:    "openai/whisper-1",
  File:     audioBytes,
  Filename: "audio.mp3",
 })
 if err != nil {
  panic(err)
 }
 fmt.Println(resp.Text)
}
```
