```csharp
using LiterLlm;

await using var client = new LlmClient(
    apiKey: Environment.GetEnvironmentVariable("OPENAI_API_KEY")!);

var response = await client.EmbedAsync(new EmbeddingRequest(
    Model: "openai/text-embedding-3-small",
    Input: ["The quick brown fox jumps over the lazy dog"]
));

var embedding = response.Data[0].Embedding;
Console.WriteLine($"Dimensions: {embedding.Length}");
Console.WriteLine($"First 5 values: [{string.Join(", ", embedding[..5])}]");
```
