<!-- snippet:compile-only -->

```csharp
using LiterLlm;

await using var client = new LlmClient(
    apiKey: Environment.GetEnvironmentVariable("OPENAI_API_KEY")!);

var response = await client.ModerateAsync(new CreateModerationRequest(
    Model: "openai/omni-moderation-latest",
    Input: "This is a test message."
));

var result = response.Results[0];
Console.WriteLine($"Flagged: {result.Flagged}");
foreach (var (category, flagged) in result.Categories)
{
    if (flagged)
    {
        Console.WriteLine($"  {category}: {result.CategoryScores[category]:F4}");
    }
}
```
