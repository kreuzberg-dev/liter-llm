```csharp
using LiterLlm;

await using var client = new LlmClient(
    apiKey: Environment.GetEnvironmentVariable("OPENAI_API_KEY")!);

var messages = new List<IMessage>
{
    new SystemMessage("You are a helpful assistant."),
    new UserMessage("What is the capital of France?"),
};

var response = await client.ChatAsync(new ChatCompletionRequest(
    Model: "openai/gpt-4o", Messages: messages));
var content = response.Choices[0].Message.Content;
Console.WriteLine($"Assistant: {content}");

// Continue the conversation
messages.Add(new AssistantMessage(content!));
messages.Add(new UserMessage("What about Germany?"));

response = await client.ChatAsync(new ChatCompletionRequest(
    Model: "openai/gpt-4o", Messages: messages));
Console.WriteLine($"Assistant: {response.Choices[0].Message.Content}");

// Token usage
if (response.Usage is not null)
{
    Console.WriteLine($"Tokens: {response.Usage.PromptTokens} in, {response.Usage.CompletionTokens} out");
}
```
