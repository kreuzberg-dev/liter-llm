```csharp
using System.Text;
using LiterLlm;

await using var client = new LlmClient(
    apiKey: Environment.GetEnvironmentVariable("OPENAI_API_KEY")!);

var request = new ChatCompletionRequest(
    Model: "openai/gpt-4o",
    Messages: [new UserMessage("Explain quantum computing briefly")]
);

var sb = new StringBuilder();
await foreach (var chunk in client.ChatStreamAsync(request))
{
    var delta = chunk.Choices?[0]?.Delta?.Content;
    if (delta is not null)
    {
        sb.Append(delta);
        Console.Write(delta);
    }
}
Console.WriteLine();
Console.WriteLine($"\nFull response length: {sb.Length} characters");
```
