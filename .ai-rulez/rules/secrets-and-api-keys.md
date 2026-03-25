---
priority: critical
---

# Secrets and API Key Handling

- All API keys are wrapped in `secrecy::SecretString` — never log, serialize, or expose them.
- Provider auth headers are injected via the `http/` layer, not in client or binding code.
- Test fixtures must use mock/placeholder keys, never real credentials.
- E2E tests that hit live APIs must read keys from environment variables only.
