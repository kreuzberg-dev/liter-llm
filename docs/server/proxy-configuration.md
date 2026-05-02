---
description: "Full TOML reference for liter-llm-proxy.toml: server, models, aliases, virtual keys, rate limits, budgets, cache, files, health, cooldown."
---

# Proxy Configuration

The proxy loads a single TOML file named `liter-llm-proxy.toml`. Pass `--config <path>` to the `liter-llm api` command or place the file in the current working directory (or any parent) and the server will discover it automatically.

Every string value supports `${VAR_NAME}` environment variable interpolation. Unknown fields are rejected at parse time, so typos fail fast instead of silently being ignored.

A minimal file that exposes one model:

--8<-- "snippets/toml/server/minimal.md"

## Discovery and overrides

The proxy resolves config in this order, later values winning:

1. Defaults from each struct's `Default` impl.
2. **Either** auto-discovery of `liter-llm-proxy.toml` in the working directory walked upward to the filesystem root, **or** an explicit `--config <path>` file (mutually exclusive — `--config` disables auto-discovery).
3. CLI flags (`--host`, `--port`, `--master-key`).
4. Environment variables read during `${VAR}` interpolation.

See [Proxy Server > Command-line flags](proxy-server.md#command-line-flags) for the flag list.

## `[server]`

HTTP listener settings.

| Field                  | Type         | Default             | Description                                                                              |
| ---------------------- | ------------ | ------------------- | ---------------------------------------------------------------------------------------- |
| `host`                 | string       | `"0.0.0.0"`         | Bind address.                                                                            |
| `port`                 | u16          | `4000`              | Bind port.                                                                               |
| `request_timeout_secs` | u64          | `600`               | Upper bound on request duration before the proxy returns 504.                            |
| `body_limit_bytes`     | usize        | `10485760` (10 MiB) | Maximum request body size. Requests larger than this return 413.                         |
| `cors_origins`         | list<string> | `["*"]`             | Allowed CORS origins. `["*"]` is wide open; replace with an explicit list in production. |

--8<-- "snippets/toml/server/server.md"

## `[general]`

Proxy-wide behaviour.

| Field                  | Type    | Default | Description                                                                                                                     |
| ---------------------- | ------- | ------- | ------------------------------------------------------------------------------------------------------------------------------- |
| `master_key`           | string? | none    | Superuser API key. Requests with this Bearer token bypass virtual-key restrictions. Can also be set via `LITER_LLM_MASTER_KEY`. |
| `default_timeout_secs` | u64     | `120`   | Default per-request timeout used when a model does not set its own.                                                             |
| `max_retries`          | u32     | `3`     | Retry attempts per upstream request. Only 429 and 5xx trigger a retry.                                                          |
| `enable_cost_tracking` | bool    | `false` | Record `gen_ai.usage.cost` on every response using embedded pricing data.                                                       |
| `enable_tracing`       | bool    | `false` | Emit OpenTelemetry spans with GenAI semantic conventions. Set to `true` when exporting to an OTEL collector.                    |

--8<-- "snippets/toml/server/general.md"

## `[[models]]`

Named model entries. A single model name may appear multiple times to define an active-active load-balanced pool.

| Field            | Type         | Default                        | Description                                                                          |
| ---------------- | ------------ | ------------------------------ | ------------------------------------------------------------------------------------ |
| `name`           | string       | required                       | Alias clients send in the `model` field.                                             |
| `provider_model` | string       | required                       | Fully-qualified provider model, like `openai/gpt-4o`.                                |
| `api_key`        | string?      | none                           | Provider API key. Falls back to the environment variable the provider crate expects. |
| `base_url`       | string?      | none                           | Override the provider endpoint.                                                      |
| `timeout_secs`   | u64?         | `general.default_timeout_secs` | Per-model timeout.                                                                   |
| `fallbacks`      | list<string> | `[]`                           | Named models to try in order when the primary returns a transient error.             |

--8<-- "snippets/toml/server/models.md"

## `[[aliases]]`

Glob-pattern credential overrides. Aliases apply to models that match the pattern and are not already defined as a `[[models]]` entry. Useful when you want a single Anthropic key to cover every Anthropic model without listing them individually.

| Field      | Type    | Default  | Description                                            |
| ---------- | ------- | -------- | ------------------------------------------------------ |
| `pattern`  | string  | required | Glob pattern such as `anthropic/*` or `openai/gpt-4*`. |
| `api_key`  | string? | none     | Credential override for matching models.               |
| `base_url` | string? | none     | Endpoint override for matching models.                 |

--8<-- "snippets/toml/server/aliases.md"

## `[[keys]]`

Virtual API keys. Each key is a Bearer token with its own model allowlist, rate limit, and spend cap. The master key bypasses all of these.

| Field          | Type         | Default  | Description                                                               |
| -------------- | ------------ | -------- | ------------------------------------------------------------------------- |
| `key`          | string       | required | The Bearer token clients present.                                         |
| `description`  | string?      | none     | Free-text label surfaced in logs and the admin API.                       |
| `models`       | list<string> | `[]`     | Allowed model names. Empty means all models are allowed.                  |
| `rpm`          | u32?         | none     | Per-key request-per-minute cap.                                           |
| `tpm`          | u64?         | none     | Per-key token-per-minute cap.                                             |
| `budget_limit` | f64?         | none     | Lifetime spend cap in USD. Requests that would exceed the cap return 402. |

--8<-- "snippets/toml/server/keys.md"

## `[rate_limit]`

Global request-per-minute and token-per-minute caps applied on top of per-key limits. Omit the table to disable global limiting.

| Field | Type | Default | Description                                 |
| ----- | ---- | ------- | ------------------------------------------- |
| `rpm` | u32? | none    | Global requests-per-minute across all keys. |
| `tpm` | u64? | none    | Global tokens-per-minute across all keys.   |

--8<-- "snippets/toml/server/rate_limit.md"

## `[budget]`

Aggregate spend enforcement. When `enforcement = "hard"`, requests that would cross the limit are rejected with 402. Under `"soft"`, they are logged and passed through.

| Field          | Type                 | Default  | Description                                     |
| -------------- | -------------------- | -------- | ----------------------------------------------- |
| `global_limit` | f64?                 | none     | Total lifetime spend cap in USD.                |
| `model_limits` | map<string, f64>     | `{}`     | Per-model spend caps keyed by `provider/model`. |
| `enforcement`  | `"hard"` or `"soft"` | `"hard"` | Whether to reject or log over-budget requests.  |

--8<-- "snippets/toml/server/budget.md"

## `[cache]`

Response cache for non-streaming completions and embeddings. Keys include the model name, request body, and any relevant headers.

| Field            | Type                | Default    | Description                                                                                           |
| ---------------- | ------------------- | ---------- | ----------------------------------------------------------------------------------------------------- |
| `max_entries`    | usize?              | none       | In-memory LRU capacity. Required for the `memory` backend.                                            |
| `ttl_seconds`    | u64?                | none       | Entry lifetime. Entries are evicted after this many seconds.                                          |
| `backend`        | string              | `"memory"` | Backend identifier: `memory`, or any OpenDAL scheme (`redis`, `s3`, `fs`, `gcs`, `azblob`, and more). |
| `backend_config` | map<string, string> | `{}`       | Backend-specific key/value options. See the OpenDAL docs for each scheme.                             |

--8<-- "snippets/toml/server/cache.md"

## `[files]`

Storage backend for the `/v1/files` endpoints.

| Field            | Type                | Default              | Description                                                                                   |
| ---------------- | ------------------- | -------------------- | --------------------------------------------------------------------------------------------- |
| `backend`        | string              | `"memory"`           | Backend identifier. `memory` is volatile; use `s3`, `gcs`, `azblob`, or `fs` for persistence. |
| `prefix`         | string              | `"liter-llm-files/"` | Object key prefix under the backend's root.                                                   |
| `backend_config` | map<string, string> | `{}`                 | Backend-specific options (bucket, region, credentials).                                       |

--8<-- "snippets/toml/server/files.md"

## `[health]`

Periodic upstream probes. When configured, the proxy sends a small request to `probe_model` every `interval_secs` seconds and marks failing providers unhealthy so the fallback layer can skip them.

| Field           | Type    | Default | Description                                                |
| --------------- | ------- | ------- | ---------------------------------------------------------- |
| `interval_secs` | u64?    | none    | Probe interval. Disabled when omitted.                     |
| `probe_model`   | string? | none    | Model name used for the probe. Usually a cheap chat model. |

--8<-- "snippets/toml/server/health.md"

## `[cooldown]`

Circuit-breaker duration. After a provider returns a transient error, the proxy refuses to send it traffic for `duration_secs` seconds and routes to fallbacks instead.

| Field           | Type | Default  | Description                 |
| --------------- | ---- | -------- | --------------------------- |
| `duration_secs` | u64  | required | Cooldown window in seconds. |

--8<-- "snippets/toml/server/cooldown.md"

## Environment variable interpolation

Any `${VAR_NAME}` pattern inside a string value is replaced with the environment variable's value before parsing. Unknown variables expand to an empty string, which is usually what you want for `Option<String>` fields. The interpolation runs on the raw TOML source, so nested tables and array values are expanded uniformly.

--8<-- "snippets/toml/server/env_interpolation.md"

!!! note "Unclosed braces are treated as literals"
If a `${` is missing its closing `}`, the proxy leaves the text as-is rather than silently truncating. That makes typos easy to spot in logs.

## Validation

The parser sets `deny_unknown_fields` on every struct. Any typo or unsupported field raises a `invalid TOML config` error with the line and column. Fix the typo and restart; the proxy does not hot-reload.
