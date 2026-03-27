```go
package main

import (
 "context"
 "fmt"
 "time"

 llm "github.com/kreuzberg-dev/liter-llm/go"
)

func main() {
 client := llm.NewClient(
  llm.WithAPIKey("sk-..."),                  // or set OPENAI_API_KEY env var
  llm.WithBaseURL("https://api.openai.com/v1"), // override provider base URL
  llm.WithTimeout(60*time.Second),            // request timeout
 )
 resp, err := client.Chat(context.Background(), &llm.ChatCompletionRequest{
  Model: "openai/gpt-4o",
  Messages: []llm.Message{
   llm.NewTextMessage(llm.RoleUser, "Hello!"),
  },
 })
 if err != nil {
  panic(err)
 }
 if len(resp.Choices) > 0 && resp.Choices[0].Message.Content != nil {
  fmt.Println(*resp.Choices[0].Message.Content)
 }
}
```
