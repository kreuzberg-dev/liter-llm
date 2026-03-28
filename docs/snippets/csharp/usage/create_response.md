<!-- snippet:compile-only -->

```csharp
using LiterLlm;

await using var client = new LlmClient(
    apiKey: Environment.GetEnvironmentVariable("OPENAI_API_KEY")!);

var response = await client.CreateResponseAsync(new CreateResponseRequest(
    Model: "openai/gpt-4o",
    Input: "Explain quantum computing in one sentence."
));
Console.WriteLine(response);
```
