# liter-lm (C#/.NET)

High-performance LLM client library for C# and .NET. Unified interface for streaming completions, tool calling, and provider routing across OpenAI, Anthropic, and 50+ LLM providers. Powered by Rust core.

## Installation

```bash
dotnet add package LiterLm
```

## Quick Start

```csharp
using LiterLm;

var client = new LlmClient();
var response = await client.ChatAsync(new ChatRequest
{
    Model = "openai/gpt-4",
    Messages = new[] {
        new Message { Role = "user", Content = "Hello!" }
    }
});
Console.WriteLine(response.Content);
```

## Full Documentation

For comprehensive documentation, examples, and API reference, see the [main repository](https://github.com/kreuzberg-dev/liter-lm).
