---
description: "liter-llm -- Universal LLM API client with native bindings for 11 languages and 142 providers"
---

# liter-llm

**Universal LLM API client -- one Rust core, 11 native language bindings, 142 providers.**

liter-llm gives you a single, unified interface to 142 LLM providers -- OpenAI, Anthropic, Google, AWS Bedrock, Groq, Mistral, and many more -- with native bindings for Python, TypeScript, Go, Java, Ruby, PHP, C#, Elixir, WebAssembly, and C/FFI.

Built in Rust for performance, safety, and reliability.

<div style="display: flex; gap: 0.5rem; margin: 1.5rem 0;">

[Get Started](getting-started/installation.md){ .md-button .md-button--primary }
[Usage](usage/chat.md){ .md-button }
[GitHub](https://github.com/kreuzberg-dev/liter-llm){ .md-button }

</div>

<div class="grid cards" markdown>

- :material-rocket-launch:{ .lg .middle } **Getting Started**

---

    Install liter-llm in your language of choice and make your first API call in minutes.

    [:octicons-arrow-right-24: Installation](getting-started/installation.md)

- :material-server-network:{ .lg .middle } **142 Providers**

---

    Access OpenAI, Anthropic, Google, AWS Bedrock, Groq, Mistral, and 130+ more through one interface.

    [:octicons-arrow-right-24: Providers](providers.md)

- :material-chat-processing:{ .lg .middle } **Usage**

---

    Chat, streaming, embeddings, image generation, speech, moderation, and more.

    [:octicons-arrow-right-24: Chat & Streaming](usage/chat.md)

- :material-api:{ .lg .middle } **API Reference**

---

    Complete API documentation for all 11 supported languages.

    [:octicons-arrow-right-24: Python](api/python.md) | [:octicons-arrow-right-24: TypeScript](api/typescript.md) | [:octicons-arrow-right-24: Go](api/go.md)

</div>

## Why liter-llm?

A universal LLM API client, compiled from the ground up in Rust. No interpreter, no transitive dependency tree, no supply chain surface area. The kind of [supply chain attack that hit litellm](https://www.xda-developers.com/popular-python-library-backdoor-machine/) is structurally impossible here.

API keys are wrapped in [`secrecy::SecretString`](https://docs.rs/secrecy/), observability is built in via [OpenTelemetry](https://opentelemetry.io/), and middleware is composable via [Tower](https://docs.rs/tower/). We give credit to [litellm](https://github.com/BerriAI/litellm) for proving the category -- see [ATTRIBUTIONS.md](https://github.com/kreuzberg-dev/liter-llm/blob/main/ATTRIBUTIONS.md).

## Key Features

- **Polyglot** -- Native bindings for 11 languages from a single Rust core
- **142 Providers** -- OpenAI, Anthropic, Google, Bedrock, Groq, Mistral, and more
- **Streaming** -- First-class SSE and AWS EventStream support
- **Observability** -- Built-in OpenTelemetry with GenAI semantic conventions
- **Type Safe** -- Compile-time checked types across all bindings
- **Secure** -- API keys wrapped in `secrecy::SecretString`, never logged or exposed
- **Middleware** -- Composable Tower stack: rate limiting, caching, cost tracking, health checks, fallback
- **Tool Calling** -- Parallel tools, structured outputs, JSON schema validation

## Quick Example

=== "Python"

    --8<-- "snippets/python/getting-started/basic_chat.md"

=== "TypeScript"

    --8<-- "snippets/typescript/getting-started/basic_chat.md"

=== "Rust"

    --8<-- "snippets/rust/getting-started/basic_chat.md"

=== "Go"

    --8<-- "snippets/go/getting-started/basic_chat.md"

## Part of kreuzberg.dev

liter-llm is built by the [kreuzberg.dev](https://kreuzberg.dev) team -- the same people behind [Kreuzberg](https://docs.kreuzberg.dev) (document extraction for 91+ formats), [tree-sitter-language-pack](https://github.com/kreuzberg-dev/tree-sitter-language-pack), and [html-to-markdown](https://github.com/kreuzberg-dev/html-to-markdown). All our libraries share the same Rust-core, polyglot-bindings architecture.

## Community

- [:fontawesome-brands-discord: Discord](https://discord.gg/xt9WY3GnKR) -- Real-time chat with the community
- [:fontawesome-brands-github: GitHub Discussions](https://github.com/kreuzberg-dev/liter-llm/discussions) -- Questions, ideas, show-and-tell
- [:octicons-issue-opened-24: Issue Tracker](https://github.com/kreuzberg-dev/liter-llm/issues) -- Bug reports and feature requests
