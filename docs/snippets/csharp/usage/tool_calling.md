```csharp
using LiterLlm;

await using var client = new LlmClient(
    apiKey: Environment.GetEnvironmentVariable("OPENAI_API_KEY")!);

var tools = new[]
{
    new Tool(
        Type: "function",
        Function: new FunctionDefinition(
            Name: "get_weather",
            Description: "Get the current weather for a location",
            Parameters: new
            {
                type = "object",
                properties = new
                {
                    location = new { type = "string", description = "City name" }
                },
                required = new[] { "location" }
            }
        )
    )
};

var response = await client.ChatAsync(new ChatCompletionRequest(
    Model: "openai/gpt-4o",
    Messages: [new UserMessage("What is the weather in Berlin?")],
    Tools: tools
));

foreach (var call in response.Choices[0].Message.ToolCalls ?? [])
{
    Console.WriteLine($"Tool: {call.Function.Name}, Args: {call.Function.Arguments}");
}
```
