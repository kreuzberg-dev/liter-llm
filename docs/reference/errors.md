---
title: "Error Reference"
---

## Error Reference

All error types thrown by the library across all languages.

### LiterLlmError

All errors that can occur when using `liter-llm`.

| Variant | Message | Description |
|---------|---------|-------------|
| `Authentication` | authentication failed: {message} | Authentication errors |
| `RateLimited` | rate limited: {message} | Rate limited errors |
| `BadRequest` | bad request: {message} | Bad request errors |
| `ContextWindowExceeded` | context window exceeded: {message} | Context window exceeded errors |
| `ContentPolicy` | content policy violation: {message} | Content policy errors |
| `NotFound` | not found: {message} | Not found errors |
| `ServerError` | server error: {message} | Server error errors |
| `ServiceUnavailable` | service unavailable: {message} | Service unavailable errors |
| `Timeout` | request timeout | Timeout errors |
| `Streaming` | streaming error: {message} | A catch-all for errors that occur during streaming response processing. This variant covers multiple sub-conditions including UTF-8 decoding failures, CRC/checksum mismatches (AWS EventStream), JSON parse errors in individual SSE chunks, and buffer overflow conditions.  The `message` field contains a human-readable description of the specific failure. |
| `EndpointNotSupported` | provider {provider} does not support {endpoint} | Endpoint not supported errors |
| `InvalidHeader` | invalid header {name:?}: {reason} | Invalid header errors |
| `Serialization` | serialization error: {0} | Serialization errors |
| `BudgetExceeded` | budget exceeded: {message} | Budget exceeded errors |
| `HookRejected` | hook rejected: {message} | Hook rejected errors |
| `InternalError` | internal error: {message} | An internal logic error (e.g. unexpected Tower response variant). This should never surface in normal operation — if it does, it indicates a bug in the library. |

---

