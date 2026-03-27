---
description: "Make your first LLM API call with liter-llm in Python, TypeScript, Rust, Go, and more"
---

# Quick Start

This guide assumes you have [installed liter-llm](installation.md) and set your API key via environment variable.

## Basic Chat Completion

Send a message and get a response:

=== "Python"

    --8<-- "snippets/python/getting-started/basic_chat.md"

=== "TypeScript"

    --8<-- "snippets/typescript/getting-started/basic_chat.md"

=== "Rust"

    --8<-- "snippets/rust/getting-started/basic_chat.md"

=== "Go"

    --8<-- "snippets/go/getting-started/basic_chat.md"

=== "Java"

    --8<-- "snippets/java/getting-started/basic_chat.md"

=== "Ruby"

    --8<-- "snippets/ruby/getting-started/basic_chat.md"

=== "PHP"

    --8<-- "snippets/php/getting-started/basic_chat.md"

=== "C#"

    --8<-- "snippets/csharp/getting-started/basic_chat.md"

=== "Elixir"

    --8<-- "snippets/elixir/getting-started/basic_chat.md"

=== "WASM"

    --8<-- "snippets/wasm/getting-started/basic_chat.md"

## Streaming Responses

Stream tokens as they arrive instead of waiting for the full response:

=== "Python"

    --8<-- "snippets/python/getting-started/streaming.md"

=== "TypeScript"

    --8<-- "snippets/typescript/getting-started/streaming.md"

=== "Rust"

    --8<-- "snippets/rust/getting-started/streaming.md"

=== "Go"

    --8<-- "snippets/go/getting-started/streaming.md"

=== "Java"

    --8<-- "snippets/java/getting-started/streaming.md"

=== "Ruby"

    --8<-- "snippets/ruby/getting-started/streaming.md"

=== "PHP"

    --8<-- "snippets/php/getting-started/streaming.md"

=== "C#"

    --8<-- "snippets/csharp/getting-started/streaming.md"

=== "Elixir"

    --8<-- "snippets/elixir/getting-started/streaming.md"

=== "WASM"

    --8<-- "snippets/wasm/getting-started/streaming.md"

## Provider Routing

liter-llm uses a `provider/model` prefix convention to route requests to the correct provider. The provider prefix determines which API endpoint, auth header, and parameter mappings to use.

```
openai/gpt-4o          -> OpenAI
anthropic/claude-sonnet-4-20250514  -> Anthropic
groq/llama3-70b        -> Groq
google/gemini-2.0-flash   -> Google AI
mistral/mistral-large  -> Mistral
bedrock/anthropic.claude-v2 -> AWS Bedrock
```

Switch providers by changing the model string -- no other code changes needed:

=== "Python"

    ```python
    import asyncio
    import os
    from liter_llm import LlmClient

    async def main() -> None:
        messages = [{"role": "user", "content": "What is the capital of France?"}]

        # OpenAI
        client_openai = LlmClient(api_key=os.environ["OPENAI_API_KEY"])
        r1 = await client_openai.chat(model="openai/gpt-4o", messages=messages)
        print(f"OpenAI: {r1.choices[0].message.content}")

        # Anthropic
        client_anthropic = LlmClient(api_key=os.environ["ANTHROPIC_API_KEY"])
        r2 = await client_anthropic.chat(model="anthropic/claude-sonnet-4-20250514", messages=messages)
        print(f"Anthropic: {r2.choices[0].message.content}")

        # Groq
        client_groq = LlmClient(api_key=os.environ["GROQ_API_KEY"])
        r3 = await client_groq.chat(model="groq/llama3-70b", messages=messages)
        print(f"Groq: {r3.choices[0].message.content}")

    asyncio.run(main())
    ```

=== "TypeScript"

    ```typescript
    import { LlmClient } from "liter-llm";

    const messages = [{ role: "user", content: "What is the capital of France?" }];

    // OpenAI
    const openai = new LlmClient({ apiKey: process.env.OPENAI_API_KEY! });
    const r1 = await openai.chat({ model: "openai/gpt-4o", messages });
    console.log(`OpenAI: ${r1.choices[0].message.content}`);

    // Anthropic
    const anthropic = new LlmClient({ apiKey: process.env.ANTHROPIC_API_KEY! });
    const r2 = await anthropic.chat({ model: "anthropic/claude-sonnet-4-20250514", messages });
    console.log(`Anthropic: ${r2.choices[0].message.content}`);

    // Groq
    const groq = new LlmClient({ apiKey: process.env.GROQ_API_KEY! });
    const r3 = await groq.chat({ model: "groq/llama3-70b", messages });
    console.log(`Groq: ${r3.choices[0].message.content}`);
    ```

!!! note "API keys must be set for each provider you call"
    The examples above require `OPENAI_API_KEY`, `ANTHROPIC_API_KEY`, and `GROQ_API_KEY` to be set. See [Installation](installation.md#api-key-setup) for details.

## Next Steps

- [Chat Guide](../guides/chat.md) -- Multi-turn conversations, system prompts, parameters
- [Streaming Guide](../guides/streaming.md) -- Backpressure, error handling, cancellation
- [Tool Calling](../guides/chat.md) -- Function calling with JSON schema validation
- [Configuration](../guides/configuration.md) -- Timeouts, retries, base URL overrides
- [Provider Registry](../providers.md) -- Browse all 142 supported providers
- [API Reference](../api/python.md) -- Full API documentation for all languages
