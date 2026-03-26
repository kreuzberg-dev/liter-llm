# PHP

<div align="center" style="display: flex; flex-wrap: wrap; gap: 8px; justify-content: center; margin: 20px 0;">
  <!-- Language Bindings -->
  <a href="https://crates.io/crates/liter-llm">
    <img src="https://img.shields.io/crates/v/liter-llm?label=Rust&color=007ec6" alt="Rust">
  </a>
  <a href="https://hex.pm/packages/liter_llm">
    <img src="https://img.shields.io/hexpm/v/liter_llm?label=Elixir&color=007ec6" alt="Elixir">
  </a>
  <a href="https://pypi.org/project/liter-llm/">
    <img src="https://img.shields.io/pypi/v/liter-llm?label=Python&color=007ec6" alt="Python">
  </a>
  <a href="https://www.npmjs.com/package/liter-llm">
    <img src="https://img.shields.io/npm/v/liter-llm?label=Node.js&color=007ec6" alt="Node.js">
  </a>
  <a href="https://www.npmjs.com/package/liter-llm-wasm">
    <img src="https://img.shields.io/npm/v/liter-llm-wasm?label=WASM&color=007ec6" alt="WASM">
  </a>
  <a href="https://central.sonatype.com/artifact/dev.kreuzberg/liter-llm">
    <img src="https://img.shields.io/maven-central/v/dev.kreuzberg/liter-llm?label=Java&color=007ec6" alt="Java">
  </a>
  <a href="https://github.com/kreuzberg-dev/liter-llm/releases">
    <img src="https://img.shields.io/github/v/tag/kreuzberg-dev/liter-llm?label=Go&color=007ec6" alt="Go">
  </a>
  <a href="https://www.nuget.org/packages/LiterLlm/">
    <img src="https://img.shields.io/nuget/v/LiterLlm?label=C%23&color=007ec6" alt="C#">
  </a>
  <a href="https://packagist.org/packages/kreuzberg-dev/liter-llm">
    <img src="https://img.shields.io/packagist/v/kreuzberg-dev/liter-llm?label=PHP&color=007ec6" alt="PHP">
  </a>
  <a href="https://rubygems.org/gems/liter-llm">
    <img src="https://img.shields.io/gem/v/liter-llm?label=Ruby&color=007ec6" alt="Ruby">
  </a>

  <!-- Project Info -->
  <a href="https://github.com/kreuzberg-dev/liter-llm/blob/main/LICENSE">
    <img src="https://img.shields.io/badge/License-MIT-007ec6" alt="License">
  </a>
  <a href="https://github.com/kreuzberg-dev/liter-llm">
    <img src="https://img.shields.io/badge/docs-GitHub-007ec6" alt="Documentation">
  </a>
</div>

Universal LLM API client for PHP. Access 100+ LLM providers through a single interface with modern PHP 8.2+ support and a type-safe API.

## Installation

### Package Installation

Install via Composer:

```bash
composer require kreuzberg-dev/liter-llm
```

### System Requirements

- **PHP 8.2+** required
- API keys via environment variables (e.g. `OPENAI_API_KEY`, `ANTHROPIC_API_KEY`)

## Quick Start

### Basic Chat

Send a message to any provider using the `provider/model` prefix:

```php
<?php

declare(strict_types=1);

use LiterLlm\LlmClient;
use LiterLlm\ChatRequest;
use LiterLlm\Message;

$client = new LlmClient();
$response = $client->chat(new ChatRequest(
    model: 'openai/gpt-4o',
    messages: [
        new Message(role: 'user', content: 'Hello!'),
    ],
));
echo $response->getContent() . PHP_EOL;
```

### Common Use Cases

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

- **Provider Routing** - Single client for 100+ LLM providers via `provider/model` prefix
- **Unified API** - Consistent `chat`, `chat_stream`, `embeddings`, `list_models` interface

- **Tool Calling** - Function calling and tool use across all supporting providers

- **Type Safe** - Schema-driven types from JSON schemas, language-native definitions
- **Auth Injection** - API keys managed via environment variables, never hardcoded
- **Error Handling** - Structured errors with provider context and retry hints

### Observability (OpenTelemetry GenAI Semantic Conventions)

liter-llm implements the [OpenTelemetry Semantic Conventions for Generative AI](https://opentelemetry.io/docs/specs/semconv/gen-ai/) systems. When the `tracing` feature is enabled, every LLM call emits structured spans with:

| Attribute | Description |
|-----------|-------------|
| `gen_ai.operation.name` | `chat`, `embeddings`, `list_models` |
| `gen_ai.request.model` | Requested model name |
| `gen_ai.system` | Provider name (`openai`, `anthropic`, etc.) |
| `gen_ai.response.id` | Completion ID from the provider |
| `gen_ai.response.model` | Actual model used (may differ from request) |
| `gen_ai.response.finish_reasons` | Why generation stopped |
| `gen_ai.usage.input_tokens` | Prompt token count |
| `gen_ai.usage.output_tokens` | Completion token count |
| `gen_ai.usage.cost` | Estimated USD cost (via `CostTrackingLayer`) |
| `error.type` | Error variant on failure |

Enable `otel` feature for OpenTelemetry export via `tracing-opentelemetry`.

### Performance

| Characteristic | Detail |
|----------------|--------|
| **Provider resolution** | Once at client construction, zero per-request overhead |
| **HTTP layer** | `reqwest` with configurable timeouts and connection pooling |
| **Streaming** | Zero-allocation SSE parser with `memchr` |
| **Memory** | Keys wrapped in `secrecy::SecretString`, never logged |

## Tool Calling

Liter-LM supports tool calling (function calling) across all providers that offer it:

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

- **[GitHub Repository](https://github.com/kreuzberg-dev/liter-llm)**
- **[API Reference](https://github.com/kreuzberg-dev/liter-llm#api-reference)**
- **[Provider Registry](https://github.com/kreuzberg-dev/liter-llm/blob/main/schemas/providers.json)**

## Contributing

Contributions are welcome! See [Contributing Guide](https://github.com/kreuzberg-dev/liter-llm/blob/main/CONTRIBUTING.md).

## License

MIT License - see LICENSE file for details.

## Support

- **GitHub Issues**: [Report bugs](https://github.com/kreuzberg-dev/liter-llm/issues)
- **Discussions**: [Ask questions](https://github.com/kreuzberg-dev/liter-llm/discussions)
