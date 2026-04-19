```bash title="HTTP transport launch"
liter-llm mcp \
  --transport http \
  --host 127.0.0.1 \
  --port 3001 \
  --config ./liter-llm-proxy.toml
```

```bash title="HTTP transport smoke test"
# List the 22 tools exposed by the server.
curl -s http://127.0.0.1:3001/mcp \
  -H 'Content-Type: application/json' \
  -d '{"jsonrpc":"2.0","id":1,"method":"tools/list"}'
```
