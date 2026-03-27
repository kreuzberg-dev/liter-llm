```go
package main

import (
 "context"
 "fmt"
 "os"

 llm "github.com/kreuzberg-dev/liter-llm/go"
)

func main() {
 client := llm.NewClient(llm.WithAPIKey(os.Getenv("OPENAI_API_KEY")))
 messages := []llm.Message{
  llm.NewTextMessage(llm.RoleSystem, "You are a helpful assistant."),
  llm.NewTextMessage(llm.RoleUser, "What is the capital of France?"),
 }

 resp, err := client.Chat(context.Background(), &llm.ChatCompletionRequest{
  Model:    "openai/gpt-4o",
  Messages: messages,
 })
 if err != nil {
  panic(err)
 }
 content := ""
 if len(resp.Choices) > 0 && resp.Choices[0].Message.Content != nil {
  content = *resp.Choices[0].Message.Content
 }
 fmt.Printf("Assistant: %s\n", content)

 // Continue the conversation
 messages = append(messages,
  llm.NewTextMessage(llm.RoleAssistant, content),
  llm.NewTextMessage(llm.RoleUser, "What about Germany?"),
 )

 resp, err = client.Chat(context.Background(), &llm.ChatCompletionRequest{
  Model:    "openai/gpt-4o",
  Messages: messages,
 })
 if err != nil {
  panic(err)
 }
 if len(resp.Choices) > 0 && resp.Choices[0].Message.Content != nil {
  fmt.Printf("Assistant: %s\n", *resp.Choices[0].Message.Content)
 }

 // Token usage
 if resp.Usage != nil {
  fmt.Printf("Tokens: %d in, %d out\n", resp.Usage.PromptTokens, resp.Usage.CompletionTokens)
 }
}
```
