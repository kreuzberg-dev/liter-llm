# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [1.2.1] - 2026-04-17

### Added

- `LlmClientRaw` trait with `_raw` variants of all `LlmClient` methods, returning `RawExchange<T>` that exposes the final request body and raw provider response before normalization ([#13](https://github.com/kreuzberg-dev/liter-llm/issues/13))
- `RawExchange<T>` and `RawStreamExchange<S>` types for wire-level debugging and custom parsing
- MCP & IDE integration documentation with setup guides for VS Code, GitHub Copilot, Claude Desktop, Cursor ([#12](https://github.com/kreuzberg-dev/liter-llm/issues/12))

### Fixed

- Docker image now published to `ghcr.io/kreuzberg-dev/liter-llm` ([#11](https://github.com/kreuzberg-dev/liter-llm/issues/11))
- Docker publish workflow timeout increased from 60 to 360 minutes (multi-arch Rust builds via QEMU were timing out)
- Bedrock `build_url` tests no longer flake due to `BEDROCK_CROSS_REGION` env var race condition

## [1.2.0] - 2026-04-07

### Added

- Local LLM provider support: Ollama, LM Studio, vLLM, llama.cpp, LocalAI, llamafile -- use any local inference engine via OpenAI-compatible API
- Docker Compose setup for local LLM integration testing with Ollama
- Integration test suite for local LLM providers

### Fixed

- PHP `onError` hook now passes a proper `\Exception` object instead of a plain string (PHP strict types requires `\Throwable`)
- README templates fixed for rumdl compliance (MD040 code fence language, MD031 blank lines, MD032 list spacing, MD020 closed headings)
- Added 404 to all POST endpoint OpenAPI specs (model not found on default model names)
- Homebrew badge added to all READMEs

## [1.1.1] - 2026-03-29

### Fixed

- Java Maven plugins downgraded to 3.x stable (was 4.0.0-beta, incompatible with Maven 3.9.x CI)
- PHP hook isolation (per-client instead of global), budget per-model enforcement, onError hook invocation, shutdown segfault
- PHP e2e tests set `max_retries=0` to prevent retry delays on mock 500s
- OpenAPI spec: added 400/415/422/503 status codes to all endpoints for schemathesis compliance
- `first_client()` returns 503 Service Unavailable instead of 500 for "no models configured"
- Schemathesis CI checks aligned (removed `content_type_conformance`, `not_a_server_error`)
- Docker cache: per-platform `TARGETARCH` cache IDs prevent multi-arch build races

### Added

- Homebrew formula: `brew tap kreuzberg-dev/tap && brew install liter-llm`
- Homebrew bottle builds (arm64_sequoia) in publish workflow
- `liter-llm-proxy` and `liter-llm-cli` added to crates.io publish pipeline
- Installation docs: CLI/Docker/Homebrew tabs
- `scripts/publish/upload-homebrew-bottles.sh` and `ensure-github-release-exists.sh`

## [1.1.0] - 2026-03-29

OpenAI-compatible LLM proxy server with CLI, MCP tool server, and Docker support.

### Proxy Server (`liter-llm-proxy`)

- **22 REST endpoints** — full OpenAI-compatible API surface: chat completions (streaming + non-streaming), embeddings, models, images, audio (speech + transcription), moderations, rerank, search, OCR, files CRUD, batches CRUD, responses CRUD, health
- **Tower middleware stack** — reuses core middleware: cache, rate limit, budget, cost tracking, cooldown, health check, tracing
- **Virtual API keys** — in-memory key store with per-key model restrictions, RPM/TPM limits, budget limits
- **Model routing** — name-based routing to provider deployments, wildcard aliases, deterministic default client
- **OpenDAL file storage** — configurable backend (memory, S3, GCS, filesystem) for file operations
- **SSE streaming** — chat completion chunks proxied as Server-Sent Events with `[DONE]` sentinel
- **OpenAPI 3.1** — utoipa-generated spec served at `/openapi.json` with bearer auth security scheme
- **TOML configuration** — `liter-llm-proxy.toml` with env var interpolation (`${VAR}`), auto-discovery, `deny_unknown_fields`
- **CORS** — configurable origins from config (default: allow all)
- **Graceful shutdown** — SIGINT/SIGTERM handling via `tokio::signal`

### MCP Server (`rmcp`)

- **22 tools** — full parity with REST API: chat, embed, list_models, generate_image, speech, transcribe, moderate, rerank, search, ocr, file CRUD (5), batch CRUD (4), response CRUD (3)
- **Transports** — stdio (default) and HTTP/SSE via `StreamableHttpService`
- **Parameter schemas** — `schemars::JsonSchema` derives for MCP tool discovery

### CLI (`liter-llm`)

- `liter-llm api` — start proxy server with config, host/port overrides, debug logging
- `liter-llm mcp` — start MCP server with stdio or HTTP transport
- 3-tier config precedence: CLI flags > env vars > config file > defaults

### Docker

- Multi-stage build: `rust:1.91-bookworm` builder, `cgr.dev/chainguard/glibc-dynamic` runtime (35MB)
- Non-root execution, OCI labels, port 4000 exposed
- `ENTRYPOINT ["liter-llm"]`, `CMD ["api", "--host", "0.0.0.0", "--port", "4000"]`

### Testing

- **74 unit tests** — config parsing, error mapping, auth key store, service pool, file store, streaming
- **32 integration tests** — auth middleware, chat/embedding/models routes, error propagation, CORS, health, OpenAPI
- **12 proxy e2e fixtures** — chat (basic + streaming), embeddings, models, auth errors, upstream errors, health, images, moderation, reranking
- **Schemathesis** — contract testing against OpenAPI spec via Docker (`task proxy:schemathesis`)

### CI/CD

- `.github/workflows/ci-docker.yaml` — build + health test + schemathesis contract tests
- `.github/workflows/publish-docker.yaml` — multi-arch (amd64/arm64) publish to `ghcr.io/kreuzberg-dev/liter-llm`
- Taskfile: `proxy:test`, `proxy:schemathesis`

## [1.0.0] - 2026-03-28

Initial stable release. Universal LLM API client with native bindings for 11 languages and 142+ providers.

### Core

- `LlmClient` trait with chat, chat_stream, embed, list_models, image_generate, speech, transcribe, moderate, rerank, search, ocr
- `FileClient`, `BatchClient`, `ResponseClient` traits for file/batch/response operations
- `DefaultClient` with reqwest + tokio, SSE streaming, retry with exponential backoff
- `ManagedClient` with composable Tower middleware stack
- 142 LLM providers embedded at compile time from `schemas/providers.json`
- Per-request provider routing from model name prefix (e.g. `anthropic/claude-sonnet-4-20250514`)
- `secrecy::SecretString` for API keys (zeroized on drop, never logged)
- TOML configuration file loading with auto-discovery (`liter-llm.toml`)
- Custom provider registration at runtime

### Middleware (Tower)

- **CacheLayer** — in-memory LRU + pluggable backends via `CacheStore` trait
- **OpenDAL cache** — 40+ storage backends (Redis, S3, GCS, filesystem, etc.) via Apache OpenDAL
- **BudgetLayer** — global + per-model spending limits with hard/soft enforcement
- **HooksLayer** — request/response/error lifecycle callbacks with guardrail pattern
- **CooldownLayer** — circuit breaker after transient errors
- **ModelRateLimitLayer** — per-model RPM/TPM rate limiting
- **HealthCheckLayer** — background health probing
- **CostTrackingLayer** — per-request cost calculation from embedded pricing registry
- **TracingLayer** — OpenTelemetry GenAI semantic convention spans
- **FallbackLayer** — automatic failover to backup provider
- **RouterLayer** — multi-deployment load balancing (round-robin, latency, cost, weighted)

### Language Bindings

All bindings expose the full API surface with language-idiomatic conventions:

- **Python** (PyO3) — async/await, typed kwargs, full .pyi stubs
- **TypeScript / Node.js** (NAPI-RS) — camelCase, .d.ts types, Promise-based
- **Rust** — native, zero-cost
- **Go** (cgo) — FFI wrapper with build tags, `context.Context` support
- **Java** (Panama FFM) — JDK 25+, `AutoCloseable`, builder pattern
- **C# / .NET** (P/Invoke) — async/await, `IAsyncEnumerable` streaming, `IDisposable`
- **Ruby** (Magnus) — RBS type signatures, Enumerator streaming
- **Elixir** (Rustler NIF) — `{:ok, result}` tuples, OTP-compatible
- **PHP** (ext-php-rs) — PHP 8.2+, JSON in/out, PIE packages
- **WebAssembly** (wasm-bindgen) — browser + Node.js, Fetch API
- **C / FFI** (cbindgen) — `extern "C"` with opaque handles

### Authentication

- Static API keys (Bearer, x-api-key)
- Azure AD OAuth2 client credentials
- Vertex AI service account JWT
- AWS STS Web Identity (EKS/IRSA)
- AWS SigV4 signing for Bedrock

### Provider Transforms

- Anthropic: message format, tool use v1, thinking blocks, max_tokens default
- AWS Bedrock: Converse API, EventStream binary framing, cross-region routing
- Vertex AI: Gemini format, embedding `:predict` endpoint
- Google AI: embedding/list_models response transforms
- Cohere: citation handling
- Mistral: API compatibility
- `param_mappings` for config-driven field renaming (8 providers)

### Documentation

- MkDocs Material site at docs.liter-llm.kreuzberg.dev
- 170+ code snippets across 10 languages
- 11 API reference docs with full method coverage
- Usage pages: Chat & Streaming, Embeddings & Rerank, Media, Search & OCR, Files & Batches, Configuration
- TOML configuration reference
- llms.txt (218 lines) with capabilities, examples, provider list
- Skills directory (4,072 lines) for Claude Code integration
- README generation from Jinja templates via `scripts/generate_readme.py`

### Testing

- 500+ unit and integration tests
- Middleware stack composition tests (cache + budget + hooks + rate limit + cooldown)
- Per-request provider routing tests
- File/batch/response CRUD operation tests
- Concurrency tests (budget atomicity, cache contention, rate limit fairness)
- Redis cache backend integration tests (Docker Compose)
- Live provider tests for 7 providers (OpenAI, Anthropic, Google AI, Vertex AI, Mistral, Azure, Bedrock)
- Smoke test apps for all 10 languages against real APIs
- E2E test generation from JSON fixtures across all languages
- Contract test fixtures for binding API parity

### CI/CD

- Multi-platform publish pipeline: crates.io, PyPI, npm, RubyGems, Hex.pm, Maven Central, NuGet, Packagist, Go FFI, PHP PIE
- Pre-commit hooks: 43 linters across all languages
- Post-generation formatting in e2e-generator
- Version sync script across 27+ manifests with README regeneration

### Previous RC Releases

<details>
<summary>Release candidate history (rc.1 through rc.9)</summary>

- **rc.1** (2026-03-27): Initial release — core crate, 11 bindings, e2e generator
- **rc.2** (2026-03-27): Packaging fixes for crates.io, RubyGems, Elixir NIF, Node NAPI, publish workflow
- **rc.3** (2026-03-27): Cache, budget, hooks middleware; custom providers; TDD e2e fixtures
- **rc.4** (2026-03-28): Shared bindings-core crate; camelCase conversion; real streaming across all bindings
- **rc.5** (2026-03-28): OpenDAL cache; search/OCR endpoints; full middleware wiring; Go/Java/C# FFI rewrites; serde deny_unknown_fields; documentation overhaul
- **rc.6** (2026-03-28): Full API documentation coverage; Rust crate README; version sync improvements
- **rc.7** (2026-03-28): Binding parity (5 middleware params + search/ocr in all 10); contract test fixtures; skills directory; PHP PIE packages
- **rc.8** (2026-03-28): CI fixes (PHP publish, crate order, Maven GPG, Ruby deps, Bedrock test)
- **rc.9** (2026-03-28): Live provider tests; Anthropic/Bedrock/Google streaming fixes; TOML config loading; per-request provider routing; integration test suite

</details>

[Unreleased]: https://github.com/kreuzberg-dev/liter-llm/compare/v1.2.0...HEAD
[1.2.0]: https://github.com/kreuzberg-dev/liter-llm/compare/v1.1.1...v1.2.0
[1.1.1]: https://github.com/kreuzberg-dev/liter-llm/compare/v1.1.0...v1.1.1
[1.1.0]: https://github.com/kreuzberg-dev/liter-llm/compare/v1.0.0...v1.1.0
[1.0.0]: https://github.com/kreuzberg-dev/liter-llm/releases/tag/v1.0.0
