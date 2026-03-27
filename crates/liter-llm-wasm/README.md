# WebAssembly

<div align="center" style="display: flex; flex-wrap: wrap; gap: 8px; justify-content: center; margin: 20px 0;">
  <!-- Language Bindings -->
  <a href="https://crates.io/crates/liter-llm">
    <img src="https://img.shields.io/crates/v/liter-llm?label=Rust&color=007ec6" alt="Rust">
  </a>
  <a href="https://pypi.org/project/liter-llm/">
    <img src="https://img.shields.io/pypi/v/liter-llm?label=Python&color=007ec6" alt="Python">
  </a>
  <a href="https://www.npmjs.com/package/@kreuzberg/liter-llm">
    <img src="https://img.shields.io/npm/v/@kreuzberg/liter-llm?label=Node.js&color=007ec6" alt="Node.js">
  </a>
  <a href="https://www.npmjs.com/package/@kreuzberg/liter-llm-wasm">
    <img src="https://img.shields.io/npm/v/@kreuzberg/liter-llm-wasm?label=WASM&color=007ec6" alt="WASM">
  </a>
  <a href="https://central.sonatype.com/artifact/dev.kreuzberg/liter-llm">
    <img src="https://img.shields.io/maven-central/v/dev.kreuzberg/liter-llm?label=Java&color=007ec6" alt="Java">
  </a>
  <a href="https://github.com/kreuzberg-dev/liter-llm/tree/main/packages/go">
    <img src="https://img.shields.io/github/v/tag/kreuzberg-dev/liter-llm?label=Go&color=007ec6" alt="Go">
  </a>
  <a href="https://www.nuget.org/packages/LiterLlm">
    <img src="https://img.shields.io/nuget/v/LiterLlm?label=C%23&color=007ec6" alt="C#">
  </a>
  <a href="https://packagist.org/packages/kreuzberg/liter-llm">
    <img src="https://img.shields.io/packagist/v/kreuzberg/liter-llm?label=PHP&color=007ec6" alt="PHP">
  </a>
  <a href="https://rubygems.org/gems/liter_llm">
    <img src="https://img.shields.io/gem/v/liter_llm?label=Ruby&color=007ec6" alt="Ruby">
  </a>
  <a href="https://hex.pm/packages/liter_llm">
    <img src="https://img.shields.io/hexpm/v/liter_llm?label=Elixir&color=007ec6" alt="Elixir">
  </a>
  <a href="https://github.com/kreuzberg-dev/liter-llm/pkgs/container/liter-llm">
    <img src="https://img.shields.io/badge/Docker-007ec6?logo=docker&logoColor=white" alt="Docker">
  </a>
  <a href="https://github.com/kreuzberg-dev/liter-llm/tree/main/crates/liter-llm-ffi">
    <img src="https://img.shields.io/badge/C-FFI-007ec6" alt="C FFI">
  </a>

  <!-- Project Info -->
  <a href="https://github.com/kreuzberg-dev/liter-llm/blob/main/LICENSE">
    <img src="https://img.shields.io/badge/License-MIT-007ec6" alt="License">
  </a>
  <a href="https://docs.liter-llm.kreuzberg.dev">
    <img src="https://img.shields.io/badge/docs-kreuzberg.dev-007ec6" alt="Docs">
  </a>
</div>

<div align="center" style="margin: 20px 0;">
  <picture>
    <img width="100%" alt="kreuzberg.dev" src="https://github.com/user-attachments/assets/1b6c6ad7-3b6d-4171-b1c9-f2026cc9deb8" />
  </picture>
</div>

<div align="center" style="margin-bottom: 20px;">
  <a href="https://discord.gg/xt9WY3GnKR">
    <img height="22" src="https://img.shields.io/badge/Discord-Join%20our%20community-7289da?logo=discord&logoColor=white" alt="Discord">
  </a>
</div>

Universal LLM API client for browsers and WebAssembly runtimes. Access 100+ LLM providers with portable deployment across browsers, Deno, and Cloudflare Workers.

## Installation

### Package Installation

Install via one of the supported package managers:

**npm:**

```bash
npm install @kreuzberg/liter-llm-wasm
```

**pnpm:**

```bash
pnpm add @kreuzberg/liter-llm-wasm
```

**yarn:**

```bash
yarn add @kreuzberg/liter-llm-wasm
```

### System Requirements

- Modern browser with WebAssembly support, or Deno 1.0+, or Cloudflare Workers
- API keys via environment variables or runtime configuration

## Quick Start

### Basic Chat

Send a message to any provider using the `provider/model` prefix:

```typescript
import init, { LlmClient } from "@kreuzberg/liter-llm-wasm";

await init();

const client = new LlmClient();
const response = await client.chat({
  model: "openai/gpt-4o",
  messages: [{ role: "user", content: "Hello!" }],
});

console.log(response.content);
```

### Common Use Cases

#### Streaming Responses

Stream tokens in real time:

```typescript
import init, { LlmClient } from "@kreuzberg/liter-llm-wasm";

await init();

const client = new LlmClient();
const stream = client.chatStream({
  model: "openai/gpt-4o",
  messages: [{ role: "user", content: "Tell me a story" }],
});

for await (const chunk of stream) {
  console.log(chunk.delta);
}
```

### Next Steps

- **[Provider Registry](https://github.com/kreuzberg-dev/liter-llm/blob/main/schemas/providers.json)** - Full list of supported providers
- **[GitHub Repository](https://github.com/kreuzberg-dev/liter-llm)** - Source, issues, and discussions

## Features

### Supported Providers (100+)

Route to any provider using the `provider/model` prefix convention:

| Provider | Example Model |
|----------|--------------|
| **OpenAI** | `openai/gpt-4o`, `openai/gpt-4o-mini` |
| **Anthropic** | `anthropic/claude-3-5-sonnet-20241022` |
| **Groq** | `groq/llama-3.1-70b-versatile` |
| **Mistral** | `mistral/mistral-large-latest` |
| **Cohere** | `cohere/command-r-plus` |
| **Together AI** | `together/meta-llama/Meta-Llama-3.1-70B-Instruct-Turbo` |
| **Fireworks** | `fireworks/accounts/fireworks/models/llama-v3p1-70b-instruct` |
| **Google Vertex** | `vertexai/gemini-1.5-pro` |
| **Amazon Bedrock** | `bedrock/anthropic.claude-3-5-sonnet-20241022-v2:0` |

**[Complete Provider List](https://github.com/kreuzberg-dev/liter-llm/blob/main/schemas/providers.json)**

### Key Capabilities

- **Provider Routing** -- Single client for 100+ LLM providers via `provider/model` prefix
- **Unified API** -- Consistent `chat`, `chat_stream`, `embeddings`, `list_models` interface

- **Streaming** -- Real-time token streaming via `chat_stream`

- **Tool Calling** -- Function calling and tool use across all supporting providers

- **Type Safe** -- Schema-driven types compiled from JSON schemas
- **Secure** -- API keys never logged or serialized, managed via environment variables
- **Observability** -- Built-in [OpenTelemetry](https://opentelemetry.io/docs/specs/semconv/gen-ai/) with GenAI semantic conventions
- **Error Handling** -- Structured errors with provider context and retry hints

### Performance

Built on a compiled Rust core for speed and safety:

- **Provider resolution** at client construction -- zero per-request overhead
- **Configurable timeouts** and connection pooling
- **Zero-copy streaming** with SSE and AWS EventStream support
- **API keys** wrapped in secure memory, zeroed on drop

## Provider Routing

Route to 100+ providers using the `provider/model` prefix convention:

```
openai/gpt-4o
anthropic/claude-3-5-sonnet-20241022
groq/llama-3.1-70b-versatile
mistral/mistral-large-latest
```

See the [provider registry](https://github.com/kreuzberg-dev/liter-llm/blob/main/schemas/providers.json) for the full list.

## Documentation

- **[Documentation](https://docs.liter-llm.kreuzberg.dev)** -- Full docs and API reference
- **[GitHub Repository](https://github.com/kreuzberg-dev/liter-llm)** -- Source, issues, and discussions
- **[Provider Registry](https://github.com/kreuzberg-dev/liter-llm/blob/main/schemas/providers.json)** -- 142 supported providers

## Contributing

Contributions are welcome! See [CONTRIBUTING.md](https://github.com/kreuzberg-dev/liter-llm/blob/main/CONTRIBUTING.md) for guidelines.

Join our [Discord community](https://discord.gg/xt9WY3GnKR) for questions and discussion.

## License

MIT -- see [LICENSE](https://github.com/kreuzberg-dev/liter-llm/blob/main/LICENSE) for details.
