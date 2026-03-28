```csharp
using LiterLlm;

await using var client = new LlmClient(
    apiKey: "sk-...",                               // or Environment.GetEnvironmentVariable("OPENAI_API_KEY")!
    baseUrl: "https://api.openai.com/v1",           // override provider base URL
    modelHint: "openai",                            // pre-resolve provider at construction
    maxRetries: 3,                                  // retry on transient failures
    timeoutSecs: 60                                 // request timeout in seconds
);

var response = await client.ChatAsync(new ChatCompletionRequest(
    Model: "openai/gpt-4o",
    Messages: [new UserMessage("Hello!")]
));
Console.WriteLine(response.Choices[0].Message.Content);
```
