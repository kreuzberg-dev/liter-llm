# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [1.0.0-rc.6] - 2026-03-28

### Added

- **Documentation: full API coverage** — search/OCR usage page, 20 new snippets, 6 new configuration sections (cache backends, cooldown, rate limiting, health checks, cost tracking, tracing) with all 10 language tabs
- All 11 API reference docs updated with `search()`, `ocr()` methods and 5 new constructor params
- Rust crate README generated from Jinja templates — fixes missing README on crates.io
- Version sync script now updates `readme_config.yaml` and auto-regenerates all READMEs
- `readme_config.yaml` Rust language entry with full feature set

### Changed

- `llms.txt` updated with search, OCR, OpenDAL cache, and middleware capabilities

## [1.0.0-rc.5] - 2026-03-28

### Added

- **OpenDAL cache backend** (`opendal-cache` feature): pluggable `CacheStore` implementation supporting 40+ storage backends (S3, Redis, GCS, local filesystem, Memcached, etc.) via Apache OpenDAL. Configure with `cache: { backend: "redis", backend_config: { connection_string: "..." } }`
- **`search()` endpoint**: web/document search across 12 providers (Brave, Tavily, Google PSE, etc.) with `SearchRequest`/`SearchResponse` types
- **`ocr()` endpoint**: document OCR across 4 providers (Mistral, Azure Doc Intelligence, etc.) with `OcrRequest`/`OcrResponse` types
- **Full Tower middleware wiring** in `ManagedClient`: all 8 layers now configurable via `ClientConfig`:
  - `cooldown_duration` — circuit breaker after transient errors
  - `rate_limit_config` — per-model RPM/TPM rate limiting
  - `health_check_interval` — background health probing
  - `enable_cost_tracking` — per-request cost calculation
  - `enable_tracing` — OpenTelemetry GenAI semantic convention spans
  - (previously only cache, budget, hooks were wired)
- `#[serde(deny_unknown_fields)]` on all request/config types for strict input validation
- `param_mappings` wired into `ConfigDrivenProvider` — 8 providers now apply field renaming (e.g. `max_completion_tokens` → `max_tokens`)
- Post-generation formatting in e2e-generator: runs language-native formatters (cargo fmt, ruff, biome, gofmt, rubocop, etc.) after code generation
- Go/Java/C# bindings rewritten as FFI wrappers around `libliter_llm_ffi` (replacing pure HTTP reimplementations)
- Go bindings use cgo with build tags matching kreuzberg pattern (`liter_llm_dev` for monorepo, `go:generate` for production)

### Fixed

- Python/PHP hook serialization: Debug format replaced with proper JSON via `serde_json`
- PHP hook registry memory leak: cleanup on drop
- `managed.rs` expect() replaced with proper error propagation
- Version sync script: regex updated to match `liter-llm-bindings-core` deps, all binding Cargo.toml files added to sync targets
- FFI Cargo.toml: added missing `version` on `liter-llm-bindings-core` dep (crates.io publish fix)
- All path deps now include `version` field for crates.io publishing
- PHP e2e composer.json: added `minimum-stability: dev` for RC versions
- Java javadoc: fixed heading level error
- Node pnpm lockfile updated
- Stale WASM doc comment removed (networking is complete)

### Changed

- Documentation overhaul: simple nav (Installation → Usage → API Reference), 150 snippets across 10 languages, all 11 API docs expanded to full parity, comprehensive llms.txt (218 lines)
- `CacheConfig` extended with `CacheBackend` enum (`Memory` | `OpenDal`)
- `ClientConfig` extended with 5 new middleware fields
- Removed unused workspace dependencies (`tokio-stream`, `url`)

### Removed

- Pure HTTP reimplementations in Go (3,419 LOC), Java (3,027 LOC), C# (2,881 LOC) — replaced with thin FFI wrappers

## [1.0.0-rc.4] - 2026-03-28

### Added

- Shared `liter-llm-bindings-core` crate: case conversion, config parsing, error formatting, runtime management, JSON helpers
- Bidirectional snake_case ↔ camelCase key conversion for Node.js and WASM bindings (inputs and outputs)
- Node.js: accept camelCase request keys (e.g. `maxTokens`) — automatically normalized to snake_case
- WASM: full camelCase output (e.g. `promptTokens`, `finishReason`) and camelCase input support
- Real `chat_stream` across all bindings: Ruby (Enumerator), Elixir (NIF), Java (SSE callback), C# (IAsyncEnumerable), WASM (ReadableStream)
- Real hooks in Node.js via `NapiHookBridge` + `ThreadsafeFunction` (replaces stub)
- PHP: full ManagedClient integration — hooks, budget, cache, custom providers
- `budget_used` / `budgetUsed` getter across all bindings (Python, Node, WASM, PHP, Ruby, C FFI)
- `unregister_provider` across all bindings (Python, Node, WASM, PHP, Ruby, C FFI)
- Documentation: hooks, budget, cache, custom provider sections added to all package READMEs
- Documentation: streaming examples for Java, C#, Ruby, Elixir, WASM (replaced "not yet supported" notes)

### Changed

- All fixture skip conditions removed — every binding runs every E2E test (zero skips)
- E2E generators rewritten for correct error assertions using `fixture.assertions.error_type`
- Node/WASM custom provider tests use static `registerProvider` with `authHeader` field
- PHP binding switched from `DefaultClient` to `ManagedClient` with Tower middleware
- Replaced `markdownlint-cli` with `rumdl-fmt` in pre-commit config
- Java Maven plugins aligned with kreuzberg (3.x stable, not 4.x beta)

### Fixed

- WASM budget/cache config: accept both snake_case and camelCase via serde aliases
- Node budget tests: fixture `global_limit` set to `0.0` for immediate pre-flight rejection
- Error type fixtures: `forbidden_403` → `Authentication`, `service_unavailable_502`/`gateway_timeout_504` → `ServiceUnavailable`
- Rust Bedrock test: updated stream URL expectation for cross-region `us.` prefix
- Elixir: extracted `do_cached_call` helper to fix Credo nesting depth warning
- Ruby: added `futures-core` dependency for `chat_stream`
- PHP E2E generator: use wrapper classes with Composer path dependency for autoloading

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

[Unreleased]: https://github.com/kreuzberg-dev/liter-llm/compare/v1.0.0-rc.4...HEAD
[1.0.0-rc.4]: https://github.com/kreuzberg-dev/liter-llm/compare/v1.0.0-rc.3...v1.0.0-rc.4
[1.0.0-rc.3]: https://github.com/kreuzberg-dev/liter-llm/compare/v1.0.0-rc.2...v1.0.0-rc.3
[1.0.0-rc.2]: https://github.com/kreuzberg-dev/liter-llm/compare/v1.0.0-rc.1...v1.0.0-rc.2
[1.0.0-rc.1]: https://github.com/kreuzberg-dev/liter-llm/releases/tag/v1.0.0-rc.1
