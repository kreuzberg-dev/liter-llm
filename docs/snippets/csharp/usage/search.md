<!-- snippet:compile-only -->

```csharp
using LiterLlm;

await using var client = new LlmClient(
    apiKey: Environment.GetEnvironmentVariable("BRAVE_API_KEY")!);

var response = await client.SearchAsync(new SearchRequest(
    Model: "brave/web-search",
    Query: "What is Rust programming language?",
    MaxResults: 5
));

foreach (var result in response.Results)
{
    Console.WriteLine($"{result.Title}: {result.Url}");
}
```
