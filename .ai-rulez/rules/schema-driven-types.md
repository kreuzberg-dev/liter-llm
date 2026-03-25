---
priority: high
---

# Schema-Driven Types

- API types in `crates/liter-lm/src/types/` are derived from the JSON schemas in `/schemas/api/`.
- Run `task generate:types` after modifying schemas to regenerate Rust types.
- The provider registry (`schemas/providers.json`) is embedded at compile time — update it to add new providers.
- OpenAI-compatible API is the baseline; provider-specific parameter mappings live in the registry.
- Keep type definitions in sync across all language bindings when the core types change.
