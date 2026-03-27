# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [1.0.0-rc.3] - 2026-03-27

### Added

- Pluggable cache backends via `CacheStore` trait with `InMemoryStore` default
- Budget enforcement middleware (`BudgetLayer`) with hard/soft modes, per-model limits
- Callback hooks middleware (`HooksLayer`) with on_request/on_response/on_error + guardrails
- Custom provider registration at runtime (`register_custom_provider`)
- `HookRejected` and `BudgetExceeded` error types
- Extended `ClientConfigBuilder` with `.cache()`, `.budget()`, `.hook()` methods
- Cache, budget, hooks, and custom provider support across all 11 language bindings
- TDD e2e test fixtures for cache, budget, hooks, custom providers, and API surface parity
- E2e test generators updated for all 11 languages with new feature tests
- Hooks wired into ALL 21 API methods in Go, Java, C#, Elixir, WASM, C FFI
- All 17 missing Python method stubs (image, speech, transcribe, moderate, rerank, files, batches, responses)

### Fixed

- Hook panic isolation via `catch_unwind` (panicking hooks don't crash the service)
- Custom provider validation (whitespace-only names, empty-string prefixes rejected)
- Node NAPI: missing `#[napi]` attributes on 6 methods
- WASM e2e: skip empty streaming test file, use pnpm for workspace link protocol
- Java: Maven 3.x-compatible plugin versions
- TypeScript: fix ESM/CJS exports map for tsup dual output
- Generators: stop overwriting package.json (preserve workspace-managed deps)

## [1.0.0-rc.2] - 2026-03-27

### Fixed

- Embed `schemas/` inside liter-llm crate for crates.io packaging
- Ruby vendoring: copy schemas into vendor tree for cross-compilation
- Elixir NIF tarball naming (lib prefix, .so extension, tag in download URL)
- Elixir publish: generate NIF checksums from release assets before compile
- Node NAPI: add platform `npm/<target>/package.json` packages for publish
- Node TypeScript wrapper: add tsup entry config for build
- Publish workflow: inline Ruby gem build and FFI build (fix Windows path mangling)
- Publish workflow: fix `gh release upload` without git repo context
- Publish workflow: artifact action versions (v8 does not exist, use v7)
- Publish workflow: add `republish` input for retag + full republish
- CI: fix `snake_to_camel` trailing underscore handling in NAPI bindings
- CI: fix WASM e2e tests (skip unimplemented `chat_stream`, build nodejs target)
- CI: fix Go golangci-lint install (build from source for Go 1.26 compat)
- CI: fix Python e2e generator unused imports
- CI: fix Windows auth test Duration underflow panic
- Metadata: update all READMEs and package descriptions to 142+ providers

## [1.0.0-rc.1] - 2026-03-27

### Added

- Core Rust crate with `LlmClient` trait, `DefaultClient`, SSE streaming, retry with backoff
- 142 LLM providers embedded at compile time from `schemas/providers.json`
- Strong OpenAI-compatible types with `FinishReason` enum, `SecretString`, `memchr` SSE parsing
- Provider auto-detection from model name prefix
- Python binding (PyO3) with async/streaming, TypedDict+Unpack typed kwargs, full .pyi stubs
- TypeScript binding (NAPI-RS) with full interface types
- C FFI binding with cbindgen header, callback-based streaming
- Go binding with typed structs, pure HTTP client, 22 tests
- Java binding with records and sealed interfaces, 13 tests
- C# binding with records and async HttpClient, 18 tests
- Elixir binding with @type/@spec, Req client, 31 tests
- Ruby binding (Magnus) with RBS type signatures and steep validation
- PHP binding (ext-php-rs) with PHPDoc stubs
- WASM binding (wasm-bindgen) with TypeScript custom section types
- E2E test generator for all 11 languages from JSON fixtures
- 11 fixtures across smoke, streaming, error-handling, tool-calling, types

[Unreleased]: https://github.com/kreuzberg-dev/liter-llm/compare/v1.0.0-rc.3...HEAD
[1.0.0-rc.3]: https://github.com/kreuzberg-dev/liter-llm/compare/v1.0.0-rc.2...v1.0.0-rc.3
[1.0.0-rc.2]: https://github.com/kreuzberg-dev/liter-llm/compare/v1.0.0-rc.1...v1.0.0-rc.2
[1.0.0-rc.1]: https://github.com/kreuzberg-dev/liter-llm/releases/tag/v1.0.0-rc.1
