---
name: liter-llm
description: >-
  Universal LLM API client for 165 providers with native bindings for 14
  languages. Use when writing code that calls LLM APIs via liter-llm in Python,
  TypeScript, Rust, Go, Java, C#, Ruby, PHP, Elixir, WASM, or C, when running
  the OpenAI-compatible proxy, or when calling LLMs through the MCP server.
  Covers chat, streaming, tool calling, embeddings, image generation, speech,
  transcription, moderation, web search, OCR, reranking, provider routing,
  middleware, and configuration.
license: MIT
metadata:
  author: xberg-io
  version: "0.1.0"
  repository: https://github.com/xberg-io/liter-llm
---

<!--
AI-RULEZ :: GENERATED FILE — DO NOT EDIT
Content-Hash: blake3:a9ebf7409cdd2e0aa6c1374b018a5d8066efbecbb8a501f9e901e288221e6e3e
Source-Hash: blake3:5982f9e920534d61a2cd6166e32a8ee98938c731fc6e892800aaecbeaf8c1221
Schema-Version: v1
-->

# Liter-LLM Universal LLM Client

Liter-LLM is a universal LLM API client with a Rust core and native bindings for
Python, TypeScript/Node.js, Go, Java, C#, Ruby, PHP, Elixir, WebAssembly, and C
(FFI). One unified interface reaches 165 providers (OpenAI, Anthropic, Google
Gemini, Groq, Mistral, Cohere, AWS Bedrock, Azure, and many more).

## Capability map

- **Modalities** — chat completions, streaming, tool/function calling,
  structured outputs, embeddings, image generation, audio (speech,
  transcription), moderation, web search, OCR, reranking, batch,
  and file operations.
- **Provider routing** — `provider/model` prefix selects the backend
  (`openai/gpt-4o`, `anthropic/claude-sonnet-4-20250514`,
  `gemini/gemini-2.0-flash`). Set `model_hint` to skip the prefix.
- **Middleware** — response caching, rate limiting (RPM/TPM), cost tracking,
  budget enforcement (`hard` / `soft`), fallback chains, circuit-breaker
  cooldown, and background health checks.
- **Search providers (12)** and **OCR providers (4)** — first-class `search`
  and `ocr` methods routed by the same prefix convention.
- **Proxy server** — `liter-llm api` exposes an OpenAI-compatible gateway with
  22 REST endpoints, virtual API keys, budgets, and SSE streaming.
- **MCP server** — `liter-llm mcp` exposes 22 tools mirroring the proxy
  endpoints, for MCP-compatible clients (Claude Code, Claude Desktop).
- **14 language bindings** — same surface, language-native naming (snake_case
  for Python/Rust/Ruby/Go/Elixir/PHP, camelCase for TS/Node/WASM/C#/Java).

## When to use the MCP server vs the SDK vs the proxy

| Use the … | When you want to … | Entry point |
|-----------|--------------------|-------------|
| **MCP server** | Let the agent call LLM APIs directly as tools, with no glue code | `liter-llm mcp --transport stdio` (this plugin auto-registers it) |
| **SDK / binding** | Write application code that calls LLMs in a specific language | `pip install liter-llm`, `cargo add liter-llm`, etc. |
| **Proxy** | Give many apps/teams a shared OpenAI-compatible endpoint with keys, budgets, and rate limits | `liter-llm api --config liter-llm-proxy.toml` |

Rule of thumb: reach for the MCP server inside an agent session; the SDK when
building software; the proxy when centralizing access for multiple consumers.

## Installation

### CLI (proxy + MCP server)

```bash
# Homebrew (macOS / Linux)
brew install xberg-io/tap/liter-llm

# or run it without a persistent install (the CLI proxy package self-installs the binary)
npx @xberg-io/liter-llm-cli --help
uvx --from liter-llm-cli liter-llm --help

# or download a prebuilt binary from the latest GitHub release:
#   https://github.com/xberg-io/liter-llm/releases/latest

# or build from source
cargo install liter-llm-cli

# or Docker (35MB image)
docker pull ghcr.io/xberg-io/liter-llm
```

### Language bindings

| Language | Install |
|----------|---------|
| Python | `pip install liter-llm` |
| Node.js | `pnpm add @xberg-io/liter-llm` |
| Rust | `cargo add liter-llm` |
| Go | `go get github.com/xberg-io/liter-llm/packages/go` |
| Ruby | `gem install liter_llm` |
| PHP | `composer require xberg-io/liter-llm` |
| C# | `dotnet add package XbergIo.LiterLlm` |
| WASM | `pnpm add @xberg-io/liter-llm-wasm` |

## Quick start (Python, async)

```python
import asyncio
import os
from liter_llm import create_client
from liter_llm._internal_bindings import ChatCompletionRequest

async def main() -> None:
    client = create_client(api_key=os.environ["OPENAI_API_KEY"])
    request = ChatCompletionRequest.from_json(
        '{"model":"openai/gpt-4o","messages":[{"role":"user","content":"Hello!"}]}'
    )
    response = await client.chat(request)
    print(response.choices[0].message.content)

asyncio.run(main())
```

## Provider routing

The prefix before `/` in the request's `model` selects the provider:

```python
ChatCompletionRequest.from_json('{"model":"openai/gpt-4o","messages":[...]}')
ChatCompletionRequest.from_json('{"model":"anthropic/claude-sonnet-4-20250514","messages":[...]}')
ChatCompletionRequest.from_json('{"model":"gemini/gemini-2.0-flash","messages":[...]}')
ChatCompletionRequest.from_json('{"model":"groq/llama3-70b","messages":[...]}')
```

API keys come from environment variables: `OPENAI_API_KEY`, `ANTHROPIC_API_KEY`,
`GEMINI_API_KEY`, `GROQ_API_KEY`, `MISTRAL_API_KEY`, `CO_API_KEY`, and AWS
credentials for Bedrock.

## Configuration

Pass basic options to `create_client(...)` (`api_key`, `base_url`,
`timeout_secs`, `max_retries`, `model_hint`), or set the full surface in a
`liter-llm.toml` (SDK) / `liter-llm-proxy.toml` (proxy) — both auto-discover
from the cwd upward.

```toml
# liter-llm.toml
api_key = "${OPENAI_API_KEY}"
model_hint = "openai"
timeout_secs = 120
max_retries = 5

[cache]
max_entries = 512
ttl_seconds = 600

[budget]
global_limit = 50.0
enforcement = "hard"

[rate_limit]
rpm = 60
tpm = 100000
```

| Option | Default | Description |
|--------|---------|-------------|
| `api_key` | required | Provider key, wrapped in `SecretString` (never logged). |
| `base_url` | from registry | Override the provider base URL. |
| `model_hint` | none | Pre-resolve a provider, skipping the prefix lookup. |
| `timeout_secs` | 60 | Request timeout in seconds. |
| `max_retries` | 3 | Retries on 429/5xx with exponential backoff. |
| `cache` | none | `max_entries`, `ttl_seconds`. |
| `budget` | none | `global_limit`, `model_limits`, `enforcement`. |
| `rate_limit` | none | `rpm`, `tpm`. |
| `cost_tracking` | false | Per-request cost tracking. |
| `tracing` | false | OpenTelemetry spans. |

## Proxy server

```bash
liter-llm api --config liter-llm-proxy.toml
```

22 OpenAI-compatible endpoints, model routing by prefix, virtual API keys,
per-key RPM/TPM limits, cost tracking, budget enforcement, response caching, SSE
streaming, and an OpenAPI 3.1 spec at `/openapi.json`.

## MCP server

```bash
liter-llm mcp --transport stdio   # for Claude Code / Claude Desktop
liter-llm mcp --transport http --port 3001
```

This plugin auto-registers the stdio server via `scripts/mcp-launch.sh`. The 22
tools mirror the proxy endpoints: `chat`, `embed`, `generate_image`, `speech`,
`transcribe`, `moderate`, `rerank`, `search`, `ocr`, `list_models`; file ops
(`create_file`, `list_files`, `retrieve_file`, `delete_file`, `file_content`);
batch ops (`create_batch`, `list_batches`, `retrieve_batch`, `cancel_batch`);
and Responses API (`create_response`, `retrieve_response`, `cancel_response`).

## Common pitfalls

1. **Provider prefix is required** — use `"provider/model"` unless `model_hint`
   is set, or routing fails.
2. **API keys are SecretString** — never logged or serialized. Read from env
   vars; never hardcode.
3. **Python methods are async** — `await` inside an async context.
4. **Naming conventions differ** — camelCase for TS/Node/WASM/C#/Java,
   snake_case elsewhere.
5. **Streaming chunks may have null content** — null-check
   `chunk.choices[0].delta.content` before use.
6. **Budget modes** — `hard` rejects over-budget requests; `soft` logs and
   allows.

## Additional resources

- Upstream docs: <https://docs.liter-llm.xberg.io>
- GitHub: <https://github.com/xberg-io/liter-llm>
