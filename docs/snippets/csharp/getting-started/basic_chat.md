```csharp
using LiterLlm;

var client = new LlmClient();
var response = await client.ChatAsync(new ChatRequest
{
    Model = "openai/gpt-4o",
    Messages = [new Message { Role = "user", Content = "Hello!" }],
});
Console.WriteLine(response.Content);
```
