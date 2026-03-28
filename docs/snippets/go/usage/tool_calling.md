```go
package main

import (
 "context"
 "encoding/json"
 "fmt"
 "os"

 llm "github.com/kreuzberg-dev/liter-llm/packages/go"
)

func main() {
 client := llm.NewClient(llm.WithAPIKey(os.Getenv("OPENAI_API_KEY")))

 tools := []llm.Tool{
  {
   Type: "function",
   Function: llm.FunctionDefinition{
    Name:        "get_weather",
    Description: "Get the current weather for a location",
    Parameters: json.RawMessage(`{
     "type": "object",
     "properties": {
      "location": {"type": "string", "description": "City name"}
     },
     "required": ["location"]
    }`),
   },
  },
 }

 resp, err := client.Chat(context.Background(), &llm.ChatCompletionRequest{
  Model: "openai/gpt-4o",
  Messages: []llm.Message{
   llm.NewTextMessage(llm.RoleUser, "What is the weather in Berlin?"),
  },
  Tools: tools,
 })
 if err != nil {
  panic(err)
 }

 for _, call := range resp.Choices[0].Message.ToolCalls {
  fmt.Printf("Tool: %s, Args: %s\n", call.Function.Name, call.Function.Arguments)
 }
}
```
