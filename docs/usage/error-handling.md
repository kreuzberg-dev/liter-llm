---
description: "The 17 LiterLlmError variants, HTTP status mapping, transient classification, retry behaviour, and links to per-language exception types."
---

# Error Handling

Every liter-llm client and the proxy return the same error taxonomy, defined by the `LiterLlmError` enum in `crates/liter-llm/src/error.rs`. Seventeen variants cover authentication, rate limits, payload problems, transport failures, and internal bugs. Language bindings map each variant to an idiomatic exception type but preserve the original semantics.

This page is the canonical reference. See [API Reference](../reference/api-python.md) for the per-language exception names.

## Variants

The 17 variants, their typical cause, and whether the Tower middleware treats them as transient:

| Variant                 | Typical trigger                                                                                                   | Transient? |
| ----------------------- | ----------------------------------------------------------------------------------------------------------------- | ---------- |
| `Authentication`        | Provider rejected the API key or the token is missing.                                                            | no         |
| `RateLimited`           | Provider returned 429. Carries an optional `retry_after` parsed from the header.                                  | yes        |
| `BadRequest`            | Malformed request, unsupported parameter, or a 4xx the proxy could not classify further.                          | no         |
| `ContextWindowExceeded` | Prompt plus `max_tokens` exceeds the model context window. Subclass of `BadRequest` in most bindings.             | no         |
| `ContentPolicy`         | Provider safety filter rejected the request or response. Subclass of `BadRequest`.                                | no         |
| `NotFound`              | Model name is unknown to the provider, or the file/batch/response ID does not exist.                              | no         |
| `ServerError`           | Provider returned 500 with an unexpected body.                                                                    | yes        |
| `ServiceUnavailable`    | Provider returned 502, 503, or 504, or a health probe marked the upstream unhealthy.                              | yes        |
| `Timeout`               | Request exceeded `default_timeout_secs` or the per-model `timeout_secs`.                                          | yes        |
| `Network`               | Transport-level failure from `reqwest` (connection reset, DNS, TLS). Only present with the `native-http` feature. | yes        |
| `Streaming`             | UTF-8 decode, CRC mismatch (AWS EventStream), malformed SSE chunk, or buffer overflow during streaming.           | no         |
| `EndpointNotSupported`  | Provider crate does not implement the requested endpoint (e.g. embeddings on an audio-only provider).             | no         |
| `InvalidHeader`         | A custom header name or value failed HTTP validation.                                                             | no         |
| `Serialization`         | `serde_json` failed to encode the request or decode the response.                                                 | no         |
| `BudgetExceeded`        | A `[budget]` or virtual-key `budget_limit` cap was hit. Returns 402 through the proxy.                            | no         |
| `HookRejected`          | A registered hook explicitly rejected the request.                                                                | no         |
| `InternalError`         | Library bug. Should never surface in normal operation.                                                            | no         |

Transient variants trigger fallbacks and retries. The [Fallback & Routing](fallback-routing.md) layer calls `LiterLlmError::is_transient()` to decide whether to try the next endpoint or return the error to the caller.

## HTTP status mapping

`LiterLlmError::from_status` turns an HTTP status code and response body into the right variant. The mapping, from `error.rs:146`:

| Status              | Variant                                                                                             |
| ------------------- | --------------------------------------------------------------------------------------------------- |
| `401`, `403`        | `Authentication`                                                                                    |
| `429`               | `RateLimited` (with `Retry-After` parsed)                                                           |
| `400`, `422`        | `ContextWindowExceeded` / `ContentPolicy` / `BadRequest` (selected by `code` or message heuristics) |
| `404`               | `NotFound`                                                                                          |
| `405`, `413`        | `BadRequest`                                                                                        |
| `408`               | `Timeout`                                                                                           |
| `500`               | `ServerError`                                                                                       |
| `502`, `503`, `504` | `ServiceUnavailable`                                                                                |
| Other `4xx`         | `BadRequest`                                                                                        |
| Anything else       | `ServerError`                                                                                       |

The classification for 400 and 422 prefers the structured `code` field (`context_length_exceeded`, `content_policy_violation`, `content_filter`) and falls back to substring matching on the message for providers that do not populate `code`.

## Retry behaviour

The built-in HTTP client retries only on transient status codes. From `crates/liter-llm/src/http/retry.rs`:

- Retries only on `429`, `500`, `502`, `503`, `504`. Everything else fails fast.
- `max_retries` defaults to `3` and is set globally via `[general]` `max_retries` in the proxy config.
- Backoff is exponential: `1s`, `2s`, `4s`, `8s`, capped at `30s`.
- Jitter scales each delay to a random value in `[0.5x, 1.0x]` of the capped base to avoid thundering herds.
- For `429`, the `Retry-After` header takes precedence, capped at `60s`. Integer seconds are parsed; HTTP-date format is logged and falls back to exponential backoff.
- The loop honours the overall request timeout. A retry that would exceed the timeout is not attempted.

Retries apply to single-endpoint calls. Cross-endpoint failover between models is handled by the separate [Fallback & Routing](fallback-routing.md) layer.

## Language bindings

Each binding exposes the Rust error taxonomy in whatever shape is idiomatic for the host language. Coverage is not uniform: some bindings mint one exception class per variant, others collapse related variants into broader categories. The table below shows how each binding surfaces errors today.

| Binding    | Surface                                                                                                                                     | Categories                                                                                                                                                                                                                                                               |
| ---------- | ------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| Rust       | `LiterLlmError` enum with 17 variants.                                                                                                      | 1:1 with the canonical list. `is_transient()` and `error_type()` available.                                                                                                                                                                                              |
| Python     | Exception hierarchy rooted at `LlmError`.                                                                                                   | 16 classes (every variant except `InternalError`, which surfaces as the base `LlmError`). `ContextWindowExceededError` and `ContentPolicyError` inherit from `BadRequestError`.                                                                                          |
| TypeScript | Thrown JavaScript `Error` objects.                                                                                                          | Single `Error` type. The message starts with a bracketed category label (`[Authentication]`, `[RateLimited]`, …). Match on the label rather than the class.                                                                                                              |
| Go         | Sentinel errors plus `*APIError` and `*StreamError` wrapper types.                                                                          | 8 sentinels: `ErrInvalidRequest`, `ErrAuthentication`, `ErrRateLimit`, `ErrNotFound`, `ErrProviderError`, `ErrStream`, `ErrBudgetExceeded`, `ErrHookRejected`. Use `errors.Is` and `errors.As`. `*APIError` exposes `StatusCode` and `Message`.                          |
| Java       | `LlmException` base plus seven inner subclasses and two standalone subclasses.                                                              | `InvalidRequestException`, `AuthenticationException`, `RateLimitException`, `NotFoundException`, `ProviderException`, `StreamException`, `SerializationException`, `BudgetExceededException`, `HookRejectedException`. Every subclass carries a stable `getErrorCode()`. |
| C#         | `LlmException` base plus nine sealed subclasses.                                                                                            | Mirrors the Java layout. Numeric `ErrorCode` constants cover the same categories.                                                                                                                                                                                        |
| Ruby       | Raises `RuntimeError` with a message.                                                                                                       | No typed hierarchy today. Branch on the string message or the underlying HTTP status exposed by the error.                                                                                                                                                               |
| Elixir     | `{:error, %LiterLlm.Error{kind: atom, code: int, http_status: int}}`.                                                                       | 10 kinds: `:unknown`, `:invalid_request`, `:authentication`, `:not_found`, `:rate_limit`, `:provider_error`, `:stream_error`, `:serialization`, `:budget_exceeded`, `:hook_rejected`. Pattern match on `kind`.                                                           |
| PHP        | Throws `\RuntimeException` for generic failures, plus `BudgetExceededException` and `HookRejectedException` for the two dedicated variants. | Two typed exceptions; everything else is a `RuntimeException` with a provider message.                                                                                                                                                                                   |
| WASM       | Rejects the returned `Promise` with a plain JavaScript `Error`.                                                                             | No typed hierarchy. Message is formatted as `HTTP {status}: {message}`. Parse the status to branch on category.                                                                                                                                                          |
| C FFI      | Returns `NULL` (or `-1` for `int32_t` returns) and stores a thread-local error message.                                                     | Read via `literllm_last_error()`. The message is formatted as `<function>: [<Category>] <details>` using the same bracketed labels as the TypeScript binding.                                                                                                            |

!!! note "Per-language exception trees"
See the `Error Handling` section of each language reference for full class inheritance, retry helpers, and runnable examples: [Python](../reference/api-python.md#error-handling), [TypeScript](../reference/api-typescript.md#error-handling), [Rust](../reference/api-rust.md#error-handling), [Go](../reference/api-go.md#error-handling), [Java](../reference/api-java.md#error-handling), [C#](../reference/api-csharp.md#error-handling), [Ruby](../reference/api-ruby.md#error-handling), [Elixir](../reference/api-elixir.md#error-handling), [PHP](../reference/api-php.md#error-handling), [WASM](../reference/api-wasm.md#error-handling), [C FFI](../reference/api-c.md#error-handling).

Bindings that collapse variants still map the HTTP status code to the same category the Rust core would return. The branching in your code may look coarser, but the wire-level semantics (which response is retried, which is a hard failure) are identical.

## Catching errors

Start by catching the base error type and branch on specific variants only where you need different behaviour.

=== "Python"
--8<-- "snippets/python/guides/error_handling.md"

=== "TypeScript"
--8<-- "snippets/typescript/guides/error_handling.md"

=== "Rust"
--8<-- "snippets/rust/guides/error_handling.md"

=== "Go"
--8<-- "snippets/go/guides/error_handling.md"

=== "Java"
--8<-- "snippets/java/guides/error_handling.md"

=== "C#"
--8<-- "snippets/csharp/guides/error_handling.md"

=== "Ruby"
--8<-- "snippets/ruby/guides/error_handling.md"

=== "PHP"
--8<-- "snippets/php/guides/error_handling.md"

=== "Elixir"
--8<-- "snippets/elixir/guides/error_handling.md"

=== "WASM"
--8<-- "snippets/wasm/guides/error_handling.md"

## Observability

The tracing middleware records an `error.type` span attribute on every failed request, set to the value returned by `LiterLlmError::error_type()`. The set of possible values matches the variant names in the table above. See [Observability](observability.md) for the full span schema.
