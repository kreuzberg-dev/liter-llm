## Proxy, MCP Server & Plugin

<details>
<summary><strong>Run the OpenAI-compatible proxy or the MCP server</strong></summary>

Beyond the SDK, the `liter-llm` CLI ships an OpenAI-compatible proxy and a Model Context Protocol (MCP) server:

```bash
brew install xberg-io/tap/liter-llm   # or: cargo install liter-llm-cli
# Windows: scoop bucket add xberg https://github.com/xberg-io/scoop-bucket && scoop install liter-llm
liter-llm api --config liter-llm-proxy.toml   # OpenAI-compatible proxy
liter-llm mcp --transport stdio               # MCP tool server

# or run the proxy without installing:
docker run -p 4000:4000 -e LITER_LLM_MASTER_KEY=sk-your-key ghcr.io/xberg-io/liter-llm
```

To use the MCP server inside a coding agent, install the **liter-llm plugin** from the [`xberg-io/plugins`](https://github.com/xberg-io/plugins) marketplace — it auto-registers the server. See the [MCP server](https://docs.liter-llm.xberg.io/server/mcp-server/) and [proxy server](https://docs.liter-llm.xberg.io/server/proxy-server/) guides for configuration, CLI usage, and agent integration.

</details>
