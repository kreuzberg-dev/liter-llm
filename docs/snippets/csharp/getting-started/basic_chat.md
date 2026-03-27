```csharp
using LiterLlm;

await using var client = new LlmClient(
    apiKey: Environment.GetEnvironmentVariable("OPENAI_API_KEY")!);

var response = await client.ChatAsync(new ChatCompletionRequest(
    Model: "openai/gpt-4o",
    Messages: [new UserMessage("Hello!")]
));
Console.WriteLine(response.Choices[0].Message.Content);
```
