<!-- snippet:compile-only -->

```csharp
using LiterLlm;

await using var client = new LlmClient(
    apiKey: Environment.GetEnvironmentVariable("OPENAI_API_KEY")!);

var response = await client.CreateBatchAsync(new CreateBatchRequest(
    InputFileId: "file-abc123",
    Endpoint: "/v1/chat/completions",
    CompletionWindow: "24h"
));
Console.WriteLine($"Batch ID: {response.Id}");
Console.WriteLine($"Status: {response.Status}");
```
