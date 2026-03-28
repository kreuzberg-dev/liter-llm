<!-- snippet:compile-only -->

```csharp
using LiterLlm;

await using var client = new LlmClient(
    apiKey: Environment.GetEnvironmentVariable("MISTRAL_API_KEY")!);

var response = await client.OcrAsync(new OcrRequest(
    Model: "mistral/mistral-ocr-latest",
    Document: new DocumentInput(Type: "document_url", Url: "https://example.com/invoice.pdf")
));

foreach (var page in response.Pages)
{
    Console.WriteLine($"Page {page.Index}: {page.Markdown[..100]}...");
}
```
