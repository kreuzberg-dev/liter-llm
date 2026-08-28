---
name: running-the-proxy
description: Use when running the `liter-llm api` OpenAI-compatible gateway — virtual keys, per-key rate limits, budgets, cost tracking, and model routing. Covers the TOML config and the 22 REST endpoints.
---

<!--
AI-RULEZ :: GENERATED FILE — DO NOT EDIT
Content-Hash: blake3:3f5a5101eedafb1369ac00a325e19a411310e607700c789e0fbdb49e3694fb60
Source-Hash: blake3:303551e5dcfae75158588635b2ef5d8742d0251f7c3bba85e65a85abf1b3beb2
Schema-Version: v1
-->

# Running the Proxy

`liter-llm api` is a drop-in OpenAI-compatible gateway: 22 REST endpoints that
route to 165 providers, with multi-tenant virtual keys, rate limits, budgets,
and cost tracking.

## Start it

```bash
liter-llm api --config liter-llm-proxy.toml
```

The proxy auto-discovers `liter-llm-proxy.toml` in the current directory.

## Configuration

```toml
[server]
host = "0.0.0.0"
port = 4000

[general]
master_key = "${LITER_LLM_MASTER_KEY}"

# Each [[models]] entry maps a routable name to a provider/model and its key.
[[models]]
name = "gpt-4o"
provider_model = "openai/gpt-4o"
api_key = "${OPENAI_API_KEY}"

[[models]]
name = "claude-sonnet"
provider_model = "anthropic/claude-sonnet-4-20250514"
api_key = "${ANTHROPIC_API_KEY}"

# Virtual keys scope which configured model names a caller may use.
[[keys]]
key = "sk-team-frontend"
models = ["gpt-4o", "claude-sonnet"]
rpm = 60
tpm = 100000
budget_limit = 50.0
```

`${ENV_VAR}` interpolation keeps secrets out of the file.

## Call it like OpenAI

```bash
curl http://localhost:4000/v1/chat/completions \
  -H "Authorization: Bearer sk-team-frontend" \
  -d '{"model": "gpt-4o", "messages": [{"role": "user", "content": "Hello"}]}'
```

## Notes

- Virtual keys scope which models a caller may use and carry their own RPM/TPM
  and budget limits; the `master_key` administers them.
- The OpenAPI 3.1 spec is served at `/openapi.json`.
- Endpoints cover chat, embeddings, images, audio, moderations, files, batches,
  responses, and model listing.
- Docker: `docker run -p 4000:4000 -e LITER_LLM_MASTER_KEY=sk-key ghcr.io/xberg-io/liter-llm`.
