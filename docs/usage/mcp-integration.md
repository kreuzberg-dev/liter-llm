---
description: "MCP & IDE Integration — set up the liter-llm MCP server with VS Code, GitHub Copilot, Claude, Cursor, and other compatible tools."
---

# MCP & IDE Integration

The liter-llm MCP server exposes 22 tools for unified access to 142+ LLM providers, embeddings, files, batches, and more. Integrate it with VS Code, GitHub Copilot, Claude Desktop, Cursor, and other MCP-compatible IDEs and applications.

## What is MCP?

The [Model Context Protocol (MCP)](https://modelcontextprotocol.io/) is an open standard for connecting AI applications to context sources. The liter-llm MCP server provides tools that allow AI assistants to call LLM APIs, generate embeddings, search documents, and manage files — all through a unified, provider-agnostic interface.

Instead of hard-coding integrations for each provider (OpenAI, Anthropic, Google, Groq, etc.), MCP lets your IDE or application call any of 142+ providers via a single interface.

## Available MCP Tools

The liter-llm MCP server exposes 22 tools for core LLM operations:

| Tool | Description |
| --- | --- |
| `chat` | Send a chat completion request to any LLM provider |
| `embed` | Generate text embeddings from configured embedding models |
| `list_models` | List available models from all configured providers |
| `generate_image` | Generate images from a text prompt (DALL-E, Flux, etc.) |
| `speech` | Generate speech audio from text (text-to-speech) |
| `transcribe` | Transcribe audio to text (speech-to-text) |
| `moderate` | Check content against moderation policies |
| `rerank` | Rerank documents by relevance to a query |
| `search` | Perform web or document search |
| `ocr` | Extract text from an image or document via OCR |
| `create_file` | Upload a file to the LLM provider |
| `list_files` | List uploaded files from the provider |
| `retrieve_file` | Retrieve metadata for an uploaded file |
| `delete_file` | Delete an uploaded file |
| `file_content` | Retrieve the raw content of an uploaded file |
| `create_batch` | Create a new batch processing job |
| `list_batches` | List batch processing jobs |
| `retrieve_batch` | Retrieve a batch processing job by ID |
| `cancel_batch` | Cancel an in-progress batch job |
| `create_response` | Create a new response (Responses API) |
| `retrieve_response` | Retrieve a response by ID (Responses API) |
| `cancel_response` | Cancel an in-progress response |

## Prerequisites

- **liter-llm CLI** installed (see [Installation](../getting-started/installation.md))
- **API keys** set up for the providers you want to use (e.g., `OPENAI_API_KEY`, `ANTHROPIC_API_KEY`)
- **Optional:** Custom `liter-llm.toml` config file

## Starting the MCP Server

### Stdio Transport (IDE Integration)

For IDE integration (VS Code, GitHub Copilot, Claude Desktop, Cursor), use the `stdio` transport:

```bash
liter-llm mcp --transport stdio
```

This is the standard way to integrate MCP servers into IDEs. The server communicates via stdin/stdout and is configured in the IDE's MCP settings.

### HTTP Transport (Remote/Custom)

For remote connections or custom integrations:

```bash
liter-llm mcp --transport http --host 127.0.0.1 --port 3001
```

The HTTP server will be available at `http://127.0.0.1:3001`.

### Custom Configuration

Use a custom config file to override provider settings, API keys, or model lists:

```bash
liter-llm mcp --transport stdio --config /path/to/liter-llm.toml
```

Configuration is identical to the proxy server — see [Configuration](./configuration.md) for full details.

## IDE Setup

=== "VS Code & GitHub Copilot"

    The VS Code MCP extension allows you to connect MCP servers directly to VS Code and GitHub Copilot. Follow the [VS Code MCP setup guide](https://marketplace.visualstudio.com/items?itemName=ms-vscode.prompt-expansion).

    **Manual Installation (without extension):**

    Edit your VS Code settings (`.vscode/settings.json` or global settings) to register the liter-llm MCP server:

    ```json
    {
      "mcpServers": [
        {
          "name": "liter-llm",
          "command": "liter-llm",
          "args": ["mcp", "--transport", "stdio"],
          "env": {
            "OPENAI_API_KEY": "sk-...",
            "ANTHROPIC_API_KEY": "sk-ant-...",
            "GROQ_API_KEY": "gsk_..."
          }
        }
      ]
    }
    ```

    Then use the tools in GitHub Copilot's chat by typing `@llm-tools` or explicitly referencing tools like `@chat` or `@embed`.

=== "Claude Desktop / Claude Code"

    Claude Desktop and Claude Code use the MCP client to load external tools. Configure liter-llm in `~/Library/Application Support/Claude/claude_desktop_config.json` (macOS) or `%APPDATA%\Claude\claude_desktop_config.json` (Windows):

    ```json
    {
      "mcpServers": {
        "liter-llm": {
          "command": "liter-llm",
          "args": ["mcp", "--transport", "stdio"],
          "env": {
            "OPENAI_API_KEY": "sk-...",
            "ANTHROPIC_API_KEY": "sk-ant-...",
            "GROQ_API_KEY": "gsk_..."
          }
        }
      }
    }
    ```

    After saving, restart Claude Desktop. The liter-llm tools will be available in the Tools panel and can be invoked directly in Claude conversations.

=== "Cursor"

    Cursor supports MCP servers through its `.cursor/settings.json` or global Cursor settings.

    **Manual Configuration:**

    Edit your Cursor settings (⌘, → "Cursor Settings" → "MCP Servers") or directly edit the config file:

    ```json
    {
      "mcpServers": [
        {
          "name": "liter-llm",
          "command": "liter-llm",
          "args": ["mcp", "--transport", "stdio"],
          "env": {
            "OPENAI_API_KEY": "sk-...",
            "ANTHROPIC_API_KEY": "sk-ant-...",
            "GROQ_API_KEY": "gsk_..."
          }
        }
      ]
    }
    ```

    Use tools via Cursor's AI chat or code generation features.

=== "Generic HTTP Client"

    For custom applications or tools that support HTTP-based MCP, use the HTTP transport:

    ```bash
    liter-llm mcp --transport http --host 127.0.0.1 --port 3001
    ```

    Then register the server in your application:

    ```json
    {
      "mcpServers": [
        {
          "name": "liter-llm",
          "url": "http://127.0.0.1:3001"
        }
      ]
    }
    ```

    Refer to your application's MCP client documentation for configuration details.

## Configuration

### Environment Variables

The MCP server reads API keys and provider configuration from environment variables, identical to the [main liter-llm client](./configuration.md#environment-variables):

```bash
export OPENAI_API_KEY="sk-..."
export ANTHROPIC_API_KEY="sk-ant-..."
export GOOGLE_API_KEY="..."
export GROQ_API_KEY="gsk_..."
export MISTRAL_API_KEY="..."
export REPLICATE_API_KEY="..."
```

!!! tip "Provider-specific keys"
    You only need to set keys for providers you plan to use. If you only use OpenAI, set `OPENAI_API_KEY` and liter-llm will automatically route requests to OpenAI based on the model prefix (e.g., `openai/gpt-4o`).

### Custom Config File

To override defaults, base URLs, or provider configurations, use a custom TOML config file:

```bash
liter-llm mcp --transport stdio --config /etc/liter-llm/liter-llm.toml
```

Example `liter-llm.toml`:

```toml
# Global timeout (seconds)
timeout_secs = 30

# Max retries for failed requests
max_retries = 3

# API key (or read from env var)
api_key = "${OPENAI_API_KEY}"

# Custom provider base URL
[[providers]]
name = "openai"
base_url = "https://api.openai.com/v1"
model_prefixes = ["openai/"]

# Local Ollama provider
[[providers]]
name = "ollama"
base_url = "http://localhost:11434/v1"
model_prefixes = ["ollama/"]
```

See [Configuration](./configuration.md) for all available options.

### Master Key (Optional)

For remote HTTP transport, set a master key to authenticate requests:

```bash
export LITER_LLM_MASTER_KEY="your-secure-key"
liter-llm mcp --transport http --host 0.0.0.0 --port 3001
```

Clients must include the key in the `Authorization` header:

```bash
curl http://localhost:3001/chat \
  -H "Authorization: Bearer your-secure-key"
```

## Tool Examples

### Using the `chat` Tool

=== "Claude"

    In Claude, ask a question and it will use the `chat` tool:

    ```
    Call the liter-llm chat tool with model "openai/gpt-4o" to summarize this document: [paste text]
    ```

=== "cURL (HTTP)"

    ```bash
    curl -X POST http://127.0.0.1:3001/tools/chat \
      -H "Content-Type: application/json" \
      -d '{
        "model": "openai/gpt-4o",
        "messages": [{"role": "user", "content": "What is 2+2?"}]
      }'
    ```

### Using the `embed` Tool

Generate embeddings for semantic search:

```json
{
  "model": "openai/text-embedding-3-small",
  "input": "The quick brown fox jumps over the lazy dog"
}
```

### Using the `list_models` Tool

List all available models from configured providers:

```json
{}
```

Returns a list of models with metadata (e.g., `openai/gpt-4o`, `anthropic/claude-opus`, `groq/llama3-70b`).

## Troubleshooting

### MCP Server Not Starting

**Error:** `command not found: liter-llm`

Ensure the CLI is installed and in your `PATH`:

```bash
liter-llm --version
which liter-llm
```

If not found, reinstall:

```bash
brew tap kreuzberg-dev/tap
brew install liter-llm
# or
cargo install liter-llm-cli
```

### IDE Not Seeing Tools

**VS Code / GitHub Copilot:**

1. Restart VS Code completely (close and reopen)
2. Check the "Extension" → "MCP" debug output for errors
3. Verify the command path in `settings.json` is correct

**Claude Desktop:**

1. Restart Claude Desktop
2. Check `~/Library/Logs/Claude/` for error logs
3. Verify the config file is valid JSON

**Cursor:**

1. Use ⌘+K → "MCP Servers" to reload
2. Check the Cursor settings panel for errors

### API Key Not Found

**Error:** `authentication failed: api key not set`

Ensure API keys are set as environment variables before starting the MCP server:

```bash
export OPENAI_API_KEY="sk-..."
liter-llm mcp --transport stdio
```

Or hardcode them in the IDE config (less secure):

```json
"env": {
  "OPENAI_API_KEY": "sk-..."
}
```

!!! warning "Never commit API keys"
    Use environment variables or a `.env` file (not committed to git). See [Secrets and API Key Handling](../getting-started/installation.md#api-key-setup) for best practices.

### Timeout Errors

**Error:** `request timeout after 30s`

Increase the timeout in a config file:

```toml
timeout_secs = 120
```

Then start with the config:

```bash
liter-llm mcp --transport stdio --config liter-llm.toml
```

### Port Already in Use (HTTP)

**Error:** `bind failed: address already in use`

Use a different port:

```bash
liter-llm mcp --transport http --host 127.0.0.1 --port 3002
```

Or kill the existing process:

```bash
lsof -i :3001
kill -9 <PID>
```

### Remote Connection Refused

**Error:** `connection refused: 127.0.0.1:3001`

Ensure the MCP server is running and listening on the correct host/port:

```bash
# Start the server
liter-llm mcp --transport http --host 0.0.0.0 --port 3001 &

# Test the connection
curl http://127.0.0.1:3001/health
```

!!! note
    Use `0.0.0.0` to allow remote connections. For local development, `127.0.0.1` is safer.

## Next Steps

- [Chat & Streaming](./chat.md) — Learn the chat API and streaming patterns
- [Provider Registry](../providers.md) — Browse all 142+ supported providers
- [Configuration](./configuration.md) — Timeouts, retries, and custom endpoints
