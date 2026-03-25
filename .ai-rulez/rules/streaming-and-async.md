---
priority: high
---

# Streaming and Async Patterns

- All client methods are async-first using `tokio` 1.x runtime.
- Streaming responses use `BoxStream` from `futures-core` for composability.
- FFI bindings must bridge async Rust to each language's concurrency model (Python asyncio, Node.js promises, Go goroutines, etc.).
- Never block the async runtime in binding code — use `spawn_blocking` for CPU-bound FFI work.
- HTTP layer uses `reqwest` 0.13 with configurable timeouts and retry logic.
