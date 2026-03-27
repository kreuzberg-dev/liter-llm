```csharp
using LiterLlm;

// Note: The C# client does not yet support streaming.
// Use the non-streaming ChatAsync method instead.
await using var client = new LlmClient(
    apiKey: Environment.GetEnvironmentVariable("OPENAI_API_KEY")!);

var response = await client.ChatAsync(new ChatCompletionRequest(
    Model: "openai/gpt-4o",
    Messages: [new UserMessage("Tell me a story")]
));
Console.WriteLine(response.Choices[0].Message.Content);
```
