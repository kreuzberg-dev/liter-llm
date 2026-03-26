```csharp
using LiterLlm;

var client = new LlmClient();
await foreach (var chunk in client.ChatStreamAsync(new ChatRequest
{
    Model = "openai/gpt-4o",
    Messages = [new Message { Role = "user", Content = "Tell me a story" }],
}))
{
    Console.Write(chunk.Delta);
}
Console.WriteLine();
```
