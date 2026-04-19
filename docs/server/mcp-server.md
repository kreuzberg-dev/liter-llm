---
description: "Run liter-llm as a Model Context Protocol server that exposes 22 LLM tools over stdio or HTTP to Claude Desktop, Cursor, and other MCP clients."
---

# MCP Server

The `liter-llm` binary can run as a Model Context Protocol (MCP) server. It exposes 22 tools backed by the same `ProxyConfig` used by the HTTP proxy, so every provider, virtual key, fallback, and cache layer that works for the REST API works for MCP clients too.

Launch it with `liter-llm mcp`. The server supports two transports: `stdio` for local clients like Claude Desktop and Cursor, and `http` for network-attached clients using the [Streamable HTTP](https://modelcontextprotocol.io/specification/2025-03-26/basic/transports) transport.

## Quick start

Run the server over stdio against an auto-discovered `liter-llm-proxy.toml`:

```bash
liter-llm mcp
```

Run over HTTP on the default port `3001`:

```bash
liter-llm mcp --transport http --host 127.0.0.1 --port 3001
```

The HTTP transport exposes a single endpoint: `POST /mcp`. Point any MCP HTTP client at `http://127.0.0.1:3001/mcp`.

## Command-line flags

| Flag | Default | Description |
|------|---------|-------------|
| `--config` | auto-discover | Path to the TOML config. Same format as the proxy. |
| `--transport` | `stdio` | Transport mode. One of `stdio` or `http`. |
| `--host` | `127.0.0.1` | Bind address for the HTTP transport. Ignored for stdio. |
| `--port` | `3001` | Bind port for the HTTP transport. Ignored for stdio. |

The MCP server loads the same `liter-llm-proxy.toml` as the HTTP proxy. See [Proxy Configuration](proxy-configuration.md) for the full schema. Any `[[models]]`, `[[aliases]]`, `[[keys]]`, `[cache]`, `[files]`, or `[health]` table defined there applies to MCP requests as well.

## Tools

Every tool returns a JSON payload as a single `text` content part. Errors are propagated as MCP error objects with the liter-llm error type embedded in the message.

### LLM operations

| Tool | Description | Key parameters |
|------|-------------|----------------|
| `chat` | Send a chat completion request to an LLM. | `model`, `messages`, `temperature?`, `max_tokens?` |
| `embed` | Generate text embeddings for the given input. | `model`, `input` |
| `list_models` | List available models from configured providers. | none |

### Media

| Tool | Description | Key parameters |
|------|-------------|----------------|
| `generate_image` | Generate images from a text prompt. | `prompt`, `model?`, `n?`, `size?` |
| `speech` | Generate speech audio from text (TTS). Returns base64 audio. | `model`, `input`, `voice` |
| `transcribe` | Transcribe audio to text (STT). | `model`, `file_base64` |

### Classification and retrieval

| Tool | Description | Key parameters |
|------|-------------|----------------|
| `moderate` | Check content against moderation policies. | `input`, `model?` |
| `rerank` | Rerank documents by relevance to a query. | `model`, `query`, `documents` |
| `search` | Perform a web or document search. | `model`, `query` |
| `ocr` | Extract text from an image or document via OCR. | `model`, `image_url?`, `image_base64?`, `media_type?` |

### Files

| Tool | Description | Key parameters |
|------|-------------|----------------|
| `create_file` | Upload a file to the LLM provider. | `filename`, `content_base64`, `purpose` |
| `list_files` | List uploaded files. | `purpose?`, `limit?` |
| `retrieve_file` | Retrieve metadata for an uploaded file. | `file_id` |
| `delete_file` | Delete an uploaded file. | `file_id` |
| `file_content` | Retrieve the raw content of an uploaded file. | `file_id` |

### Batches

| Tool | Description | Key parameters |
|------|-------------|----------------|
| `create_batch` | Create a new batch processing job. | `input_file_id`, `endpoint`, `completion_window` |
| `list_batches` | List batch processing jobs. | `limit?`, `after?` |
| `retrieve_batch` | Retrieve a batch processing job by ID. | `batch_id` |
| `cancel_batch` | Cancel an in-progress batch processing job. | `batch_id` |

### Responses API

| Tool | Description | Key parameters |
|------|-------------|----------------|
| `create_response` | Create a new response (Responses API). | `model`, `input` |
| `retrieve_response` | Retrieve a response by ID. | `response_id` |
| `cancel_response` | Cancel an in-progress response. | `response_id` |

The full parameter schema for every tool is defined in `crates/liter-llm-proxy/src/mcp/params.rs` and surfaced to MCP clients as JSON Schema through `rmcp`.

## Claude Desktop

Add an entry to your `claude_desktop_config.json`:

```json
{
  "mcpServers": {
    "liter-llm": {
      "command": "liter-llm",
      "args": ["mcp"],
      "env": {
        "OPENAI_API_KEY": "sk-...",
        "ANTHROPIC_API_KEY": "sk-ant-..."
      }
    }
  }
}
```

Restart Claude Desktop. The 22 tools appear under the `liter-llm` server. Point `liter-llm` at a config file with `--config /absolute/path/to/liter-llm-proxy.toml` if you want virtual keys or a custom model list.

## Cursor

Cursor reads MCP servers from `~/.cursor/mcp.json` (or the workspace equivalent). Use the same shape as Claude Desktop:

```json
{
  "mcpServers": {
    "liter-llm": {
      "command": "liter-llm",
      "args": ["mcp", "--config", "/absolute/path/to/liter-llm-proxy.toml"]
    }
  }
}
```

--8<-- "snippets/toml/mcp/stdio.md"

## HTTP transport

Run the server in HTTP mode when the client is on a different machine or when you want to share one MCP server across several users. Pair it with a reverse proxy for TLS.

```bash
liter-llm mcp --transport http --host 0.0.0.0 --port 3001
```

--8<-- "snippets/toml/mcp/http.md"

The HTTP endpoint is `POST /mcp`. Each request opens a short-lived session managed by `rmcp`'s `LocalSessionManager`. There is no authentication on the MCP HTTP transport itself, so bind to loopback or put it behind an authenticated reverse proxy.

!!! warning "HTTP transport has no built-in auth"
    Unlike the REST proxy, `liter-llm mcp --transport http` does not check Bearer tokens. Do not expose it to the public internet without a reverse proxy that handles authentication.

## Shared configuration

The MCP server and the HTTP proxy use the same `ProxyConfig` loader. That means:

- Models defined in `[[models]]` are callable as `chat`, `embed`, `generate_image`, and so on.
- Glob overrides in `[[aliases]]` apply to MCP requests.
- `[cache]` caches non-streaming responses across both surfaces.
- `[files]` persists files uploaded via the `create_file` tool.
- `[[keys]]` virtual keys are loaded but not enforced on MCP calls, since the transports are assumed trusted.

The master key is also loaded, but the MCP surface does not send Bearer tokens, so virtual-key RPM, TPM, and budget caps do not apply to MCP invocations today. Use `[rate_limit]` and `[budget]` for global caps that cover both surfaces.

## Troubleshooting

- **"tool call failed: model 'foo' not found"**: the `model` parameter passed to the tool does not match any `name` in `[[models]]`. Check `liter-llm-proxy.toml` and restart.
- **stdio transport hangs on startup**: the client expects a JSON-RPC handshake on stdin. Make sure you are launching `liter-llm mcp` from an MCP client, not an interactive shell.
- **HTTP transport returns 404**: the endpoint is `/mcp`, not `/`. Every request is `POST /mcp`.
- **Image or audio tools return empty content**: the underlying provider may not support the feature. Check [Providers](../providers.md) for per-provider capability.
