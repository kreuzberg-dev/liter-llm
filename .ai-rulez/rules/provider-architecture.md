---
priority: critical
---

# Provider Architecture

- Providers are resolved once at client construction from the embedded `schemas/providers.json` registry — no per-request overhead.
- Model routing uses name prefix convention: `"groq/llama3-70b"` routes to the Groq provider.
- The `LlmClient` trait defines the unified API: `chat`, `chat_stream`, `embeddings`, `list_models`.
- `DefaultClient` implements provider selection, auth header injection, and endpoint mapping.
- Never expose provider-specific types in the public API — all bindings use the unified `types/` definitions.
- API types are schema-driven from `/schemas/api/` JSON schemas; use `task generate:types` to regenerate.
