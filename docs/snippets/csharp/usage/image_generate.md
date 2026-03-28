<!-- snippet:compile-only -->

```csharp
using LiterLlm;

await using var client = new LlmClient(
    apiKey: Environment.GetEnvironmentVariable("OPENAI_API_KEY")!);

var response = await client.ImageGenerateAsync(new CreateImageRequest(
    Model: "openai/dall-e-3",
    Prompt: "A sunset over mountains",
    N: 1,
    Size: "1024x1024"
));
Console.WriteLine(response.Data[0].Url);
```
