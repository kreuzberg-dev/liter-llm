---
name: using-the-mcp-server
description: Use when calling LLM APIs through the liter-llm MCP server's 22 tools, and to decide when MCP beats the CLI or SDK. Covers the tool surface, the auto-installing launcher, and authentication.
---

<!--
AI-RULEZ :: GENERATED FILE — DO NOT EDIT
Content-Hash: blake3:4c8d31c57c57ec1db9bc8fa0064086f1ee33a81418bc129b4bf9fae2b5099bab
Source-Hash: blake3:52b7805950bc172a5fd4adecfa93662d5ea4324acb1e965576e4e1ea95c358a1
Schema-Version: v1
-->

# Using the MCP Server

The `liter-llm` MCP server exposes 22 tools that mirror the proxy's REST
endpoints, so an MCP-compatible client (Claude Code, Claude Desktop) can call
165 LLM providers as tools with no glue code.

## How it runs in this plugin

The plugin auto-registers the server. Its command is the bundled launcher
`scripts/mcp-launch.sh`, which execs `liter-llm mcp --transport stdio`. On first
run the launcher resolves a `liter-llm` binary: it reuses one on `PATH`, then
tries `npx`/`uvx`, then Homebrew, then downloads a checksum-verified prebuilt
from the tool's latest GitHub release. Override with
`LITER_LLM_LAUNCHER=auto|npx|uvx|brew|download`.

To run it manually:

```bash
liter-llm mcp --transport stdio          # for Claude Code / Claude Desktop
liter-llm mcp --transport http --port 3001
```

## The 22 tools

Modality tools: `chat`, `embed`, `generate_image`, `speech`, `transcribe`,
`moderate`, `rerank`, `search`, `ocr`.

Model listing: `list_models`.

File operations: `create_file`, `list_files`, `retrieve_file`, `delete_file`,
`file_content`.

Batch operations: `create_batch`, `list_batches`, `retrieve_batch`,
`cancel_batch`.

Responses API: `create_response`, `retrieve_response`, `cancel_response`.

## Authentication

stdio has no per-request auth headers, so the server binds auth once at startup
via `[mcp]` config: set `mcp.stdio_key_id` to bind a specific virtual key, or
`mcp.stdio_trust_local = true` for fully trusted local environments. Provider
keys still come from env vars.

## When to prefer MCP over the CLI or SDK

- **Prefer MCP** inside an agent session: the agent invokes tools directly, with
  no code to write or process to manage.
- **Prefer the SDK** when building application software in a specific language.
- **Prefer the proxy** when many apps or teams need a shared OpenAI-compatible
  endpoint with keys, budgets, and rate limits.
