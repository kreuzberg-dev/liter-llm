# liter-llm (TypeScript/Node.js)

<div align="center" style="display: flex; flex-wrap: wrap; gap: 8px; justify-content: center; margin: 20px 0;">
  <!-- Language Badges -->
  <a href="https://crates.io/crates/liter-llm">
    <img src="https://img.shields.io/crates/v/liter-llm?label=Rust&color=007ec6" alt="Rust">
  </a>
  <a href="https://pypi.org/project/liter-llm/">
    <img src="https://img.shields.io/pypi/v/liter-llm?label=Python&color=007ec6" alt="Python">
  </a>
  <a href="https://www.npmjs.com/package/liter-llm">
    <img src="https://img.shields.io/npm/v/liter-llm?label=TypeScript&color=007ec6" alt="TypeScript">
  </a>
  <a href="https://pkg.go.dev/github.com/kreuzberg-dev/liter-llm/packages/go">
    <img src="https://img.shields.io/github/v/tag/kreuzberg-dev/liter-llm?label=Go&color=007ec6" alt="Go">
  </a>
  <a href="https://central.sonatype.com/artifact/dev.kreuzberg/liter-llm">
    <img src="https://img.shields.io/maven-central/v/dev.kreuzberg/liter-llm?label=Java&color=007ec6" alt="Java">
  </a>
  <a href="https://rubygems.org/gems/liter_llm">
    <img src="https://img.shields.io/gem/v/liter_llm?label=Ruby&color=007ec6" alt="Ruby">
  </a>
  <a href="https://packagist.org/packages/kreuzberg/liter-llm">
    <img src="https://img.shields.io/packagist/v/kreuzberg/liter-llm?label=PHP&color=007ec6" alt="PHP">
  </a>
  <a href="https://www.nuget.org/packages/LiterLlm/">
    <img src="https://img.shields.io/nuget/v/LiterLlm?label=C%23&color=007ec6" alt="C#">
  </a>
  <a href="https://hex.pm/packages/liter_llm">
    <img src="https://img.shields.io/hexpm/v/liter_llm?label=Elixir&color=007ec6" alt="Elixir">
  </a>

  <!-- Project Info -->
  <a href="https://github.com/kreuzberg-dev/liter-llm/blob/main/LICENSE">
    <img src="https://img.shields.io/badge/License-MIT-007ec6" alt="License">
  </a>
  <a href="https://github.com/kreuzberg-dev/liter-llm">
    <img src="https://img.shields.io/badge/GitHub-liter--lm-007ec6?logo=github" alt="GitHub">
  </a>
</div>

High-performance LLM client library for TypeScript and Node.js. Unified interface for
streaming completions, tool calling, and provider routing across OpenAI, Anthropic, and
142+ LLM providers. Powered by Rust core via NAPI-RS bindings with full TypeScript type
definitions and native async/Promise support.

## Installation

**npm:**

```bash
npm install liter-llm
```

**pnpm:**

```bash
pnpm add liter-llm
```

## Quick Start

```typescript
import { LlmClient } from "liter-llm";

const client = new LlmClient({ apiKey: process.env.OPENAI_API_KEY });
const response = await client.chat({
  model: "openai/gpt-4o",
  messages: [{ role: "user", content: "Hello!" }],
});
console.log(response.choices[0].message.content);
```

## Features

| Feature              | Supported                                          |
| -------------------- | -------------------------------------------------- |
| **Provider Routing** | 142+ providers via `"provider/model"` prefix       |
| **Chat Completions** | OpenAI-compatible unified API                      |
| **Streaming**        | Server-sent events, token-by-token                 |
| **Tool Calling**     | Function definitions, structured outputs           |
| **Async**            | Native async/await                                 |
| **Provider Auth**    | Automatic key injection from environment variables |
| **Retry Logic**      | Configurable retries with exponential backoff      |
| **Timeouts**         | Per-request configurable timeouts                  |

## Streaming

Stream tokens as they are generated for responsive user experiences:

```typescript
import { LlmClient } from "liter-llm";

const client = new LlmClient({ apiKey: process.env.OPENAI_API_KEY });
const stream = client.chatStream({
  model: "openai/gpt-4o",
  messages: [{ role: "user", content: "Tell me a story" }],
});

for await (const chunk of stream) {
  process.stdout.write(chunk.delta ?? "");
}
console.log();
```

## Tool Calling

Define tools for the model to call with structured outputs:

```typescript
import { LlmClient, type Tool } from "liter-llm";

const client = new LlmClient({ apiKey: process.env.OPENAI_API_KEY });

const tools: Tool[] = [
  {
    name: "get_weather",
    description: "Get the current weather for a location",
    parameters: {
      type: "object",
      properties: {
        location: { type: "string", description: "City name" },
      },
      required: ["location"],
    },
  },
];

const response = await client.chat({
  model: "openai/gpt-4o",
  messages: [{ role: "user", content: "What is the weather in Berlin?" }],
  tools,
});

for (const call of response.toolCalls ?? []) {
  console.log(`Tool: ${call.name}, Args:`, call.arguments);
}
```

## Hooks

Register lifecycle hooks for observability or request manipulation:

```typescript
client.addHook({
  onRequest: (req) => console.log(`Sending to ${req.model}`),
  onResponse: (resp) => console.log(`Got ${resp.choices.length} choices`),
  onError: (err) => console.error(`Error: ${err}`),
});
```

## Budget

Set per-model or global cost limits:

```typescript
const client = new LlmClient({
  apiKey: process.env.OPENAI_API_KEY,
  budget: { globalLimit: 10.0, modelLimits: { "openai/gpt-4o": 5.0 } },
});
console.log(client.budgetUsed); // cumulative spend in USD
```

## Cache

Enable in-memory LRU response caching:

```typescript
const client = new LlmClient({
  apiKey: process.env.OPENAI_API_KEY,
  cache: { maxEntries: 256, ttlSeconds: 300 },
});
```

## Custom Providers

Register a custom provider at runtime (static method):

```typescript
LlmClient.registerProvider({
  name: "my-provider",
  baseUrl: "https://api.my-provider.com/v1",
  authHeader: "Authorization",
  authPrefix: "Bearer",
  modelPrefix: "my-provider",
});
```

## Provider Routing

Route requests to any of 142+ providers using a `"provider/model"` prefix:

```text
openai/gpt-4o
anthropic/claude-opus-4-5
groq/llama3-70b-8192
mistral/mistral-large-latest
together/meta-llama/Llama-3-70b-chat-hf
```

Set the provider API key as an environment variable (e.g. `OPENAI_API_KEY`, `ANTHROPIC_API_KEY`).
The client picks up keys automatically — no per-call configuration required.

## Documentation

- **[GitHub Repository](https://github.com/kreuzberg-dev/liter-llm)**
- **[Provider List](https://github.com/kreuzberg-dev/liter-llm/blob/main/schemas/providers.json)**
- **[Contributing Guide](https://github.com/kreuzberg-dev/liter-llm/blob/main/CONTRIBUTING.md)**

## License

MIT License — see [LICENSE](https://github.com/kreuzberg-dev/liter-llm/blob/main/LICENSE) for details.
