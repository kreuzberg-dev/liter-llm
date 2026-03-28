<!-- snippet:compile-only -->

```csharp
using LiterLlm;

await using var client = new LlmClient(
    apiKey: Environment.GetEnvironmentVariable("OPENAI_API_KEY")!);

var fileBytes = await File.ReadAllBytesAsync("data.jsonl");
var response = await client.CreateFileAsync(new CreateFileRequest(
    File: fileBytes,
    Filename: "data.jsonl",
    Purpose: "batch"
));
Console.WriteLine($"File ID: {response.Id}");
Console.WriteLine($"Size: {response.Bytes} bytes");
```
