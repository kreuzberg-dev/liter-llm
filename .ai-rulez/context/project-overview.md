---
summary: Universal LM API client architecture, crate layout, and provider registry.
---

# Project Overview

Liter-LM is a universal LLM API client written in Rust with native bindings for 10+ languages.

## Crate Layout

- `crates/liter-lm` — Core library: client, providers, types, HTTP layer, error handling
- `crates/liter-lm-py` — Python bindings (PyO3)
- `crates/liter-lm-node` — Node.js bindings (NAPI-RS)
- `crates/liter-lm-ffi` — C FFI layer for Go, Java, C#
- `crates/liter-lm-php` — PHP bindings (ext-php-rs)
- `crates/liter-lm-wasm` — WebAssembly bindings (wasm-bindgen)

## Language Packages

- `packages/go` — Go module wrapping the C FFI
- `packages/java` — Java wrapper via Panama FFM
- `packages/csharp` — .NET wrapper via P/Invoke
- `packages/ruby` — Ruby gem (Magnus)
- `packages/elixir` — Elixir package (Rustler NIF)

## Provider Registry

`schemas/providers.json` contains 100+ LLM provider configurations compiled into the binary. Each entry defines: base URL, auth header format, model prefixes, and parameter mappings.

## E2E Test Generation

`tools/e2e-generator` generates language-specific E2E tests from fixtures in `/fixtures/`. Tests cover: configuration, error handling, streaming, tool calling, and type validation.

## Key Commands

- `task build` — Build all crates
- `task test` — Run test suites
- `task lint` — Lint via prek
- `task generate:types` — Regenerate types from JSON schemas
- `task e2e:generate:all` — Regenerate E2E tests for all languages
