---
description: "liter-llm -- Universal LLM API client with native bindings for 11 languages"
---

# liter-llm

**Universal LLM API client -- one Rust core, 11 native language bindings, 142 providers.**

liter-llm gives you a single, unified interface to 100+ LLM providers -- OpenAI, Anthropic, Google, AWS Bedrock, Groq, Mistral, and many more -- with native bindings for Python, TypeScript, Go, Java, Ruby, PHP, C#, Elixir, WebAssembly, and C/FFI.

Built in Rust for performance, safety, and reliability.

<div class="grid cards" markdown>

- :material-rocket-launch:{ .lg .middle } **Getting Started**

    ---

    Install liter-llm in your language of choice and make your first API call in minutes.

    [:octicons-arrow-right-24: Installation](getting-started/installation.md)

- :material-server-network:{ .lg .middle } **142 Providers**

    ---

    Access OpenAI, Anthropic, Google, AWS Bedrock, Groq, Mistral, and 130+ more through one interface.

    [:octicons-arrow-right-24: Providers](providers.md)

- :material-language-rust:{ .lg .middle } **Architecture**

    ---

    Understand the Rust core, Tower middleware stack, and how language bindings work.

    [:octicons-arrow-right-24: Architecture](concepts/architecture.md)

- :material-api:{ .lg .middle } **API Reference**

    ---

    Complete API documentation for all 11 supported languages.

    [:octicons-arrow-right-24: Python](api/python.md) · [:octicons-arrow-right-24: TypeScript](api/typescript.md) · [:octicons-arrow-right-24: Go](api/go.md)

</div>

## Key Features

- **Polyglot** -- Native bindings for 11 languages from a single Rust core
- **142 Providers** -- OpenAI, Anthropic, Google, Bedrock, Groq, Mistral, and more
- **Streaming** -- First-class SSE and AWS EventStream support
- **Observability** -- Built-in OpenTelemetry with GenAI semantic conventions
- **Type Safe** -- Compile-time checked types across all bindings
- **Secure** -- API keys wrapped in `secrecy::SecretString`, never logged or exposed
- **Middleware** -- Composable Tower stack: rate limiting, caching, cost tracking, health checks, fallback
- **Tool Calling** -- Parallel tools, structured outputs, JSON schema validation
