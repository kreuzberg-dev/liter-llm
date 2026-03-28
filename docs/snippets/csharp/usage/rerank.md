<!-- snippet:compile-only -->

```csharp
using LiterLlm;

await using var client = new LlmClient(
    apiKey: Environment.GetEnvironmentVariable("OPENAI_API_KEY")!);

var response = await client.RerankAsync(new RerankRequest(
    Model: "cohere/rerank-v3.5",
    Query: "What is the capital of France?",
    Documents: [
        "Paris is the capital of France.",
        "Berlin is the capital of Germany.",
        "London is the capital of England.",
    ]
));

foreach (var result in response.Results)
{
    Console.WriteLine($"Index: {result.Index}, Score: {result.RelevanceScore:F4}");
}
```
