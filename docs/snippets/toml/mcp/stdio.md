```json title="claude_desktop_config.json (stdio)"
{
  "mcpServers": {
    "liter-llm": {
      "command": "liter-llm",
      "args": ["mcp", "--config", "/absolute/path/to/liter-llm-proxy.toml"],
      "env": {
        "OPENAI_API_KEY": "sk-...",
        "ANTHROPIC_API_KEY": "sk-ant-..."
      }
    }
  }
}
```
