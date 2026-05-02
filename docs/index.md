---
description: "liter-llm — Universal LLM API client. One Rust core, 11 native language bindings, 143 providers."
---

# liter-llm

liter-llm is an LLM API client written in Rust with native bindings for Python, TypeScript, Go, Java, C#, Ruby, PHP, Elixir, WebAssembly, C, and Rust. One API surface across 143 providers. No Python runtime, no dependency chain surprises. It ships as a compiled binary with Tower middleware, an OpenAI-compatible proxy server, and an MCP server built in.

<div class="hero-badges" markdown>

[:material-lightning-bolt: Quick Start](getting-started/installation.md){ .md-button .md-button--primary }
[:material-package-variant: Installation](getting-started/installation.md){ .md-button }
[:fontawesome-brands-github: GitHub](https://github.com/kreuzberg-dev/liter-llm){ .md-button }
[:fontawesome-brands-discord: Community](https://discord.gg/xt9WY3GnKR){ .md-button }

</div>

---

## Explore the Docs

<!-- markdownlint-disable MD030 MD035 -->
<div class="grid cards" markdown>

- :material-rocket-launch:{ .lg .middle } **Getting Started**

  ***

  Install the package for your language and make your first API call.

  [:octicons-arrow-right-24: Installation](getting-started/installation.md)

- :material-chat:{ .lg .middle } **Chat & Streaming**

  ***

  Single-turn and multi-turn chat, streaming, tool calling, structured outputs.

  [:octicons-arrow-right-24: Chat Guide](usage/chat.md)

- :material-server:{ .lg .middle } **Proxy Server**

  ***

  OpenAI-compatible proxy with virtual keys, budget enforcement, and TOML config.

  [:octicons-arrow-right-24: Proxy Server](server/proxy-server.md)

- :material-routes:{ .lg .middle } **Fallback & Routing**

  ***

  Round-robin, latency-based, cost-based, weighted-random, and ordered-fallback strategies.

  [:octicons-arrow-right-24: Routing Guide](usage/fallback-routing.md)

- :material-key-variant:{ .lg .middle } **Authentication**

  ***

  Azure AD, AWS Bedrock STS/IRSA, Vertex AI OAuth2 with automatic token caching.

  [:octicons-arrow-right-24: Auth Guide](usage/authentication.md)

- :material-code-braces:{ .lg .middle } **API Reference**

  ***

  Full reference for Python, TypeScript, Rust, Go, Java, C#, Ruby, Elixir, PHP, WASM, C FFI.

  [:octicons-arrow-right-24: Python](reference/api-python.md)

</div>
<!-- markdownlint-enable MD030 MD035 -->

---

## Part of kreuzberg.dev

liter-llm is built by the [kreuzberg.dev](https://kreuzberg.dev) team, the same people behind a family of Rust-core, polyglot-bindings libraries.

<div class="home-family" markdown>

<a class="home-family__card" href="https://docs.kreuzberg.dev" target="_blank" markdown>
:material-file-document-multiple:{ .home-family__icon }
**Kreuzberg**
<span>Document extraction for 91+ formats — PDF, Office, images, HTML, and more.</span>
</a>

<a class="home-family__card" href="https://github.com/kreuzberg-dev/tree-sitter-language-pack" target="_blank" markdown>
:material-code-tags:{ .home-family__icon }
**tree-sitter-language-pack**
<span>All Tree-sitter grammars in one package, across every language binding.</span>
</a>

<a class="home-family__card" href="https://github.com/kreuzberg-dev/html-to-markdown" target="_blank" markdown>
:material-language-markdown:{ .home-family__icon }
**html-to-markdown**
<span>Fast, lossless HTML to Markdown conversion with a Rust core.</span>
</a>

</div>

---

## Getting Help

- **Bugs & feature requests** -- [Open an issue on GitHub](https://github.com/kreuzberg-dev/liter-llm/issues)
- **Community chat** -- [Join the Discord](https://discord.gg/xt9WY3GnKR)
- **Contributing** -- [Read the contributor guide](contributing.md)
