# liter-llm

<div align="center" style="display: flex; flex-wrap: wrap; gap: 8px; justify-content: center; margin: 20px 0;">
  <!-- Language Bindings -->  <a href="https://crates.io/crates/liter-llm">
    <img src="https://img.shields.io/crates/v/liter-llm?label=Rust&color=007ec6" alt="Rust">
  </a>  <a href="https://pypi.org/project/liter-llm/">
    <img src="https://img.shields.io/pypi/v/liter-llm?label=Python&color=007ec6" alt="Python">
  </a>  <a href="https://www.npmjs.com/package/@kreuzberg/liter-llm">
    <img src="https://img.shields.io/npm/v/@kreuzberg/liter-llm?label=Node.js&color=007ec6" alt="Node">
  </a>  <a href="https://www.npmjs.com/package/@kreuzberg/liter-llm-wasm">
    <img src="https://img.shields.io/npm/v/@kreuzberg/liter-llm-wasm?label=WASM&color=007ec6" alt="Wasm">
  </a>  <a href="https://central.sonatype.com/artifact/dev.kreuzberg/liter-llm">
    <img src="https://img.shields.io/maven-central/v/dev.kreuzberg/liter-llm?label=Java&color=007ec6" alt="Java">
  </a>  <a href="https://github.com/kreuzberg-dev/liter-llm/tree/main/packages/go/v1">
    <img src="https://img.shields.io/github/v/tag/kreuzberg-dev/liter-llm?label=Go&color=007ec6" alt="Go">
  </a>  <a href="https://www.nuget.org/packages/LiterLlm">
    <img src="https://img.shields.io/nuget/v/LiterLlm?label=C%23&color=007ec6" alt="Csharp">
  </a>  <a href="https://packagist.org/packages/kreuzberg/liter-llm">
    <img src="https://img.shields.io/packagist/v/kreuzberg/liter-llm?label=PHP&color=007ec6" alt="Php">
  </a>  <a href="https://rubygems.org/gems/liter_llm">
    <img src="https://img.shields.io/gem/v/liter_llm?label=Ruby&color=007ec6" alt="Ruby">
  </a>  <a href="https://hex.pm/packages/liter_llm">
    <img src="https://img.shields.io/hexpm/v/liter_llm?label=Elixir&color=007ec6" alt="Elixir">
  </a>  <a href="https://github.com/kreuzberg-dev/liter-llm/pkgs/container/liter-llm">
    <img src="https://img.shields.io/badge/Docker-007ec6?logo=docker&logoColor=white" alt="Docker">
  </a>  <a href="https://github.com/kreuzberg-dev/liter-llm/tree/main/crates/liter-llm-ffi">
    <img src="https://img.shields.io/badge/C-FFI-007ec6" alt="Ffi">
  </a>
  <!-- Project Info -->
  <a href="https://github.com/kreuzberg-dev/liter-llm/actions">
    <img src="https://img.shields.io/github/actions/workflow/status/kreuzberg-dev/liter-llm/ci-rust.yaml?branch=main&label=CI&color=007ec6" alt="CI">
  </a>
  <a href="https://github.com/kreuzberg-dev/liter-llm/blob/main/LICENSE">
    <img src="https://img.shields.io/badge/License-MIT-007ec6.svg" alt="License">
  </a>
  <a href="https://docs.liter-llm.kreuzberg.dev">
    <img src="https://img.shields.io/badge/docs-kreuzberg.dev-007ec6" alt="Docs">
  </a>
</div>

<div align="center">
  <a href="https://discord.gg/xt9WY3GnKR">
    <img height="22" src="https://img.shields.io/badge/Discord-Join%20our%20community-7289da?logo=discord&logoColor=white" alt="Discord">
  </a>
</div>

Universal LLM API client with polyglot bindings for 100+ providers

## Overview

**liter-llm** provides unified access to 100+ LLM providers with a single API and native bindings for multiple programming languages. Ship LLM-powered features in your application without managing provider-specific SDKs.

## Architecture

```text
liter-llm/
├── crates/
│   ├── liter-llm/           # Rust core library
│   ├── liter-llm-py/        # Python (maturin/PyO3) bindings
│   ├── liter-llm-node/      # Node.js (NAPI-RS) bindings
│   ├── liter-llm-java/      # Java (Panama FFI) bindings
│   ├── liter-llm-ffi/       # C-compatible FFI library
│   ├── liter-llm-php/       # PHP (ext-php-rs) bindings
│   └── liter-llm-wasm/      # WebAssembly (wasm-bindgen) bindings
├── packages/
│   ├── go/                 # Go (cgo) bindings
│   ├── ruby/               # Ruby (Magnus) gem
│   ├── elixir/             # Elixir (Rustler NIF) package
│   ├── csharp/             # .NET (P/Invoke) package
│   └── java/               # Java wrapper
└── schemas/                # Provider registry and API schemas
```

## Quick Start

### Rust

```sh
cargo add liter-llm
```

See [Rust README](crates/liter-llm/README.md) for full documentation.

### Python

```sh
pip install liter-llm
```

```sh
uv add liter-llm
```

See [Python README](crates/liter-llm-py/README.md) for full documentation.

### Node.js

```sh
npm install @kreuzberg/liter-llm
```

```sh
pnpm add @kreuzberg/liter-llm
```

```sh
yarn add @kreuzberg/liter-llm
```

See [Node.js README](crates/liter-llm-node/README.md) for full documentation.

### Go

```sh
go get github.com/kreuzberg-dev/liter-llm/packages/go/v1
```

See [Go README](packages/go/v1/README.md) for full documentation.

### Java

```xml
<dependency>
  <groupId>dev.kreuzberg</groupId>
  <artifactId>liter-llm</artifactId>
  <version>0.1.0</version>
</dependency>
```

```groovy
implementation("dev.kreuzberg:liter-llm:0.1.0")
```

See [Java README](crates/liter-llm-java/README.md) for full documentation.

### Elixir

```elixir
{:liter_llm, "~> 0.1"}
```

See [Elixir README](packages/elixir/README.md) for full documentation.

### Ruby

```sh
gem install liter_llm
```

See [Ruby README](packages/ruby/README.md) for full documentation.

### WebAssembly

```sh
npm install @kreuzberg/liter-llm-wasm
```

```sh
pnpm add @kreuzberg/liter-llm-wasm
```

See [WebAssembly README](crates/liter-llm-wasm/README.md) for full documentation.

### PHP

```sh
composer require kreuzberg/liter-llm
```

See [PHP README](crates/liter-llm-php/README.md) for full documentation.

### .NET (C#)

```sh
dotnet add package LiterLlm
```

See [.NET (C#) README](packages/csharp/LiterLlm/README.md) for full documentation.

### C/C++ (FFI)

Build from source as part of this workspace.

See [C/C++ (FFI) README](crates/liter-llm-ffi/README.md) for full documentation.

## Core API

All bindings expose a unified `chat()` function for sending requests to LLM providers:

| Language | Function |
| -------- | -------- |
| Rust | `liter_llm::DefaultClient::new(config).chat(messages, options)` |
| Python | `LiterLlmClient(api_key=...).chat(messages, config)` |
| Node.js | `new LiterLlmClient({ apiKey }).chat(messages, config)` |
| Go | `client.Chat(ctx, messages, config)` |
| Java | `client.chat(messages, configJson)` |
| Ruby | `LiterLlm::Client.new(api_key: ...).chat(messages, config)` |
| Elixir | `LiterLlm.chat(messages, config)` |
| WASM | `new LiterLlmClient({ apiKey }).chat(messages, config)` |
| C FFI | `liter_llm_chat(client, messages_json, config_json)` |

The `chat()` function returns a structured response including the message content, usage statistics, and provider metadata.

## Features

| Feature | Description |
| --- | --- |
| **100+ Providers** | Unified access to OpenAI, Anthropic, Groq, Mistral, Cohere, and 100+ more |
| **Streaming** | First-class streaming support via `chat_stream()` across all bindings |
| **Embeddings** | Embedding generation with `embeddings()` for supported providers |
| **Tool Calling** | Structured tool/function calling support |
| **Polyglot Bindings** | Native bindings for Rust, Python, Node.js, Go, Java, Elixir, Ruby, PHP, C# |
| **Schema-Driven** | Provider registry compiled from `schemas/providers.json` — no runtime lookups |

## Supported Providers

100+ providers including OpenAI, Anthropic, Groq, Mistral, Cohere, Together AI, Fireworks, Perplexity, DeepSeek, Google Gemini, and many more. See `schemas/providers.json` for the full list.

## Package READMEs

- [Rust](crates/liter-llm/README.md) -- Rust core library with unified LLM client.
- [Python](crates/liter-llm-py/README.md) -- Python bindings for liter-llm via PyO3.
- [Node.js](crates/liter-llm-node/README.md) -- Node.js NAPI bindings for liter-llm.
- [Go](packages/go/v1/README.md) -- Go bindings for liter-llm via cgo.
- [Java](crates/liter-llm-java/README.md) -- Java bindings for liter-llm via Panama FFM (JDK 21+).
- [Elixir](packages/elixir/README.md) -- Elixir bindings for liter-llm via Rustler NIF.
- [Ruby](packages/ruby/README.md) -- Ruby bindings for liter-llm via Magnus.
- [WebAssembly](crates/liter-llm-wasm/README.md) -- WebAssembly bindings for liter-llm.
- [PHP](crates/liter-llm-php/README.md) -- PHP extension for liter-llm via ext-php-rs.
- [.NET (C#)](packages/csharp/LiterLlm/README.md) -- .NET P/Invoke bindings for liter-llm.
- [C/C++ (FFI)](crates/liter-llm-ffi/README.md) -- C-compatible FFI bindings for liter-llm.

## Contributing

Contributions are welcome! See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

Join our [Discord community](https://discord.gg/xt9WY3GnKR) for questions and discussion.

## License

MIT -- see [LICENSE](LICENSE) for details.
