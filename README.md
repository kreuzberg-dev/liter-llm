# liter-llm

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
    <img src="https://img.shields.io/badge/License-MIT-007ec6.svg" alt="License">
  </a>
  <a href="https://docs.liter-llm.kreuzberg.dev">
    <img src="https://img.shields.io/badge/docs-kreuzberg.dev-007ec6" alt="Docs">
  </a>
</div>

<img width="3384" height="573" alt="kreuzberg.dev" src="https://github.com/user-attachments/assets/1b6c6ad7-3b6d-4171-b1c9-f2026cc9deb8" />

<div align="center" style="margin-top: 20px;">
  <a href="https://discord.gg/xt9WY3GnKR">
    <img height="22" src="https://img.shields.io/badge/Discord-Join%20our%20community-7289da?logo=discord&logoColor=white" alt="Discord">
  </a>
</div>

**A lighter, faster, safer universal LLM API client** -- one Rust core, 11 native language bindings, 142 providers.

## Why liter-llm?

A universal LLM API client, compiled from the ground up in Rust. No interpreter, no transitive dependency tree, no supply chain surface area. One binary, 11 native language bindings, 142 providers.

- **Compiled Rust core.** No `pip install` supply chain. No `.pth` auto-execution hooks. No runtime dependency tree to compromise. The kind of [supply chain attack that hit litellm](https://www.xda-developers.com/popular-python-library-backdoor-machine/) in 2026 is structurally impossible here.
- **Secrets stay secret.** API keys are wrapped in [`secrecy::SecretString`](https://docs.rs/secrecy/) -- zeroed on drop, redacted in logs, never serialized.
- **Polyglot from day one.** Python, TypeScript, Go, Java, Ruby, PHP, C#, Elixir, WebAssembly, C/FFI -- all thin wrappers around the same Rust core. No reimplementation drift.
- **Observability built in.** Production-grade [OpenTelemetry](https://opentelemetry.io/) with GenAI semantic conventions -- not an afterthought callback system.
- **Composable middleware.** Rate limiting, caching, cost tracking, health checks, and fallback as [Tower](https://docs.rs/tower/) layers you stack like building blocks.

We give credit to [litellm](https://github.com/BerriAI/litellm) for proving the category -- our provider registry was bootstrapped from theirs. See [ATTRIBUTIONS.md](ATTRIBUTIONS.md).

## Feature Comparison

An honest look at where things stand. We're newer and leaner -- litellm has breadth we haven't matched yet, and we have depth they can't easily retrofit.

| | liter-llm | litellm |
|---|---|---|
| **Language** | Rust (compiled, memory-safe) | Python |
| **Bindings** | 11 native (Rust, Python, TS, Go, Java, Ruby, PHP, C#, Elixir, WASM, C) | Python (+ OpenAI-compatible proxy) |
| **Providers** | 142 (compiled at build time) | 100+ (runtime resolution) |
| **Streaming** | SSE + AWS EventStream binary protocol | SSE + AWS EventStream |
| **Observability** | Built-in OpenTelemetry (GenAI semconv) | 40+ callback integrations |
| **API key safety** | `secrecy::SecretString` (zeroed, redacted) | Plain strings |
| **Middleware** | Composable Tower stack | Built-in callback system |
| **Proxy / Gateway** | -- | Yes |
| **Guardrails** | -- | 10+ integrations, 4 execution modes (advanced: enterprise) |
| **Semantic caching** | -- | Redis + Qdrant backends |
| **Virtual key mgmt** | -- | Yes (key rotation: enterprise) |
| **Management API** | -- | Multi-tenant (teams, budgets, keys; tiers + reporting: enterprise) |
| **Fine-tuning API** | -- | Enterprise only |
| **Load balancer** | Fallback middleware | Full router with strategies |
| **Cost tracking** | Embedded pricing + OTEL spans | Per-key/team/model budgets |
| **Rate limiting** | Per-model RPM/TPM (Tower layer) | Per-key/user/team/model |
| **Caching** | In-memory LRU + 40+ backends via OpenDAL (S3, Redis, GCS, DynamoDB, disk, ...) | 7 backends (Redis, S3, GCS, disk, Qdrant) |
| **Tool calling** | Parallel tools, structured output, JSON schema | Full support |
| **Embeddings** | Yes | Yes |
| **Batch API** | Yes | Yes |
| **Audio / Speech** | Yes | Yes |
| **Lifecycle hooks** | onRequest/onResponse/onError per-client | Callback integrations |
| **Budget enforcement** | Per-model + global limits, hard/soft modes | Per-key/team budgets |
| **Health checks** | Automatic provider probes + cooldown | -- |
| **Custom providers** | Runtime `register_provider` API | Config + code-based |
| **Image generation** | Yes | Yes |

## Key Features

- **142 providers** -- OpenAI, Anthropic, Google, AWS Bedrock, Groq, Mistral, Together AI, Fireworks, Perplexity, DeepSeek, Cohere, and [130+ more](schemas/providers.json)
- **11 native bindings** -- Rust, Python, TypeScript/Node.js, Go, Java, Ruby, PHP, C#, Elixir, WebAssembly, C/FFI
- **First-class streaming** -- SSE and AWS EventStream binary protocol with zero-copy buffers
- **OpenTelemetry** -- GenAI semantic conventions, cost tracking spans, HTTP-level tracing
- **Tower middleware** -- Rate limiting, LRU caching, cost estimation, health checks, cooldowns, fallback -- all composable
- **Tool calling** -- Parallel tools, structured outputs, JSON schema validation
- **Embeddings** -- Dimension selection, base64 format, multi-provider support
- **Schema-driven** -- Provider registry and API types compiled from JSON schemas, no runtime lookups

## Architecture

```text
liter-llm/
├── crates/
│   ├── liter-llm/           # Rust core library
│   ├── liter-llm-py/        # Python (PyO3) core
│   ├── liter-llm-node/      # Node.js (NAPI-RS) core
│   ├── liter-llm-ffi/       # C-compatible FFI layer
│   ├── liter-llm-php/       # PHP (ext-php-rs) core
│   └── liter-llm-wasm/      # WebAssembly (wasm-bindgen) core
├── packages/
│   ├── python/               # Python package
│   ├── typescript/           # TypeScript/Node.js package
│   ├── go/                   # Go (cgo) module
│   ├── java/                 # Java (Panama FFI) package
│   ├── ruby/                 # Ruby (Magnus) gem
│   ├── elixir/               # Elixir (Rustler NIF) package
│   ├── csharp/               # .NET (P/Invoke) package
│   └── php/                  # PHP (Composer) package
└── schemas/                  # Provider registry and API schemas
```

## Quick Start

Install in your language of choice:

| Language | Install |
|----------|---------|
| Python | `pip install liter-llm` |
| Node.js | `pnpm add @kreuzberg/liter-llm` |
| Rust | `cargo add liter-llm` |
| Go | `go get github.com/kreuzberg-dev/liter-llm/packages/go` |
| Java | `dev.kreuzberg:liter-llm` (Maven/Gradle) |
| Ruby | `gem install liter_llm` |
| PHP | `composer require kreuzberg/liter-llm` |
| C# | `dotnet add package LiterLlm` |
| Elixir | `{:liter_llm, "~> 0.1"}` in mix.exs |
| WASM | `pnpm add @kreuzberg/liter-llm-wasm` |
| C/FFI | Build from source -- see [FFI crate](crates/liter-llm-ffi) |

### Usage

```python
from liter_llm import LlmClient

client = LlmClient()

# Chat with any provider using the provider/model prefix
response = client.chat(
    model="openai/gpt-4o",
    messages=[{"role": "user", "content": "Hello!"}],
)
print(response.choices[0].message.content)

# Stream responses
for chunk in client.chat_stream(
    model="anthropic/claude-3-5-sonnet-20241022",
    messages=[{"role": "user", "content": "Tell me a story"}],
):
    print(chunk.delta, end="", flush=True)
```

The same API is available in all 11 languages -- see the language READMEs below for idiomatic examples.

## Core API

All bindings expose a unified `chat()` function:

| Language | Usage |
| -------- | ----- |
| Rust | `DefaultClient::new(config).chat(messages, options).await` |
| Python | `LlmClient(api_key=...).chat(messages, config)` |
| Node.js | `new LlmClient({ apiKey }).chat(messages, config)` |
| Go | `client.Chat(ctx, messages, config)` |
| Java | `client.chat(messages, configJson)` |
| Ruby | `LiterLlm::LlmClient.new(api_key, config).chat(messages)` |
| Elixir | `LiterLlm.chat(messages, config)` |
| PHP | `LiterLlm\LlmClient::new($apiKey)->chat($messages, $config)` |
| C# | `new LlmClient(apiKey).ChatAsync(messages, config)` |
| WASM | `new LlmClient({ apiKey }).chat(messages, config)` |
| C FFI | `liter_llm_chat(client, messages_json, config_json)` |

## Language READMEs

| Language | README | Binding |
| -------- | ------ | ------- |
| Python | [packages/python](packages/python/README.md) | PyO3 |
| TypeScript / Node.js | [crates/liter-llm-node](crates/liter-llm-node/README.md) | NAPI-RS |
| Go | [packages/go](packages/go/README.md) | cgo |
| Java | [packages/java](packages/java/README.md) | Panama FFI |
| Ruby | [packages/ruby](packages/ruby/README.md) | Magnus |
| Elixir | [packages/elixir](packages/elixir/README.md) | Rustler NIF |
| PHP | [packages/php](packages/php/README.md) | ext-php-rs |
| .NET (C#) | [packages/csharp](packages/csharp/README.md) | P/Invoke |
| WebAssembly | [crates/liter-llm-wasm](crates/liter-llm-wasm/README.md) | wasm-bindgen |
| C/C++ (FFI) | [crates/liter-llm-ffi](crates/liter-llm-ffi) | C ABI |

## Part of kreuzberg.dev

liter-llm is built by the [kreuzberg.dev](https://kreuzberg.dev) team -- the same people behind [Kreuzberg](https://github.com/kreuzberg-dev/kreuzberg) (document extraction for 91+ formats), [tree-sitter-language-pack](https://github.com/kreuzberg-dev/tree-sitter-language-pack) (multilingual parsing), and [html-to-markdown](https://github.com/kreuzberg-dev/html-to-markdown). All our libraries share the same Rust-core, polyglot-bindings architecture. Visit [kreuzberg.dev](https://kreuzberg.dev) or find us on [GitHub](https://github.com/kreuzberg-dev).

## Contributing

Contributions are welcome! See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

Join our [Discord community](https://discord.gg/xt9WY3GnKR) for questions and discussion.

## License

MIT -- see [LICENSE](LICENSE) for details.
