---
description: "Cargo feature flags for liter-llm , covering what each flag enables and its binary-size impact."
---

# Feature Flags

liter-llm uses Cargo feature flags to keep the default binary small and avoid pulling in dependencies that are unused in most deployments. The default build includes only the native HTTP stack.

## Reference

| Flag | Default | Enables | Pulls in |
|------|---------|---------|----------|
| `native-http` | yes | reqwest HTTP client, tokio runtime, SSE parser, base64 codec | `reqwest`, `tokio`, `memchr`, `base64` |
| `tracing` | no | `TracingLayer`, `#[instrument]` spans on HTTP and SSE functions | `tracing` |
| `tower` | no | Full Tower middleware stack: `LlmService`, `TracingLayer`, `FallbackLayer`, `Router`, and all other layers | `tower`, `tower-http`, `dashmap`, `futures-util`, and `tracing` |
| `otel` | no | Re-exports `tracing_opentelemetry` and `opentelemetry` at `liter_llm::tower::tracing::otel` so callers can build an OTEL pipeline without a direct dependency | `tracing-opentelemetry`, `opentelemetry` |
| `bedrock` | no | AWS SigV4 request signing in `BedrockProvider`. Without this flag, the provider still routes `bedrock/` model names but sends requests unsigned (suitable for mock servers). | `aws-sigv4`, `aws-credential-types` |
| `azure-auth` | no | `AzureAdCredentialProvider` (client-credentials OAuth2 flow for Azure OpenAI) | none beyond `native-http` |
| `vertex-auth` | no | `VertexOAuthCredentialProvider` (service-account JWT flow for Vertex AI) | `jsonwebtoken` |
| `bedrock-auth` | no | `WebIdentityCredentialProvider` (STS web identity / IRSA for Bedrock) | none beyond `native-http` |
| `copilot-auth` | no | GitHub Copilot OAuth Device Flow credential provider | `native-http` |
| `tokenizer` | no | `count_tokens()` and `count_request_tokens()` via HuggingFace tokenizers. Downloads tokenizer files from HuggingFace Hub on first use and caches them in-process. | `tokenizers` |
| `opendal-cache` | no | Distributed cache backend via OpenDAL (S3, GCS, Azure Blob, Redis, filesystem). Requires `tower`. | `opendal` |
| `full` | no | All of the above: `tower`, `tracing`, `otel`, `bedrock`, `bedrock-auth`, `azure-auth`, `vertex-auth`, `copilot-auth`, `tokenizer`, `opendal-cache` | All optional dependencies |

## Usage

Add only the flags your deployment needs:

```toml
# Minimal: just the HTTP client, no Tower, no tracing
liter-llm = "..."

# Production server: Tower stack + OTEL + Bedrock
liter-llm = { version = "...", features = ["tower", "otel", "bedrock", "bedrock-auth"] }

# Azure OpenAI + tracing only
liter-llm = { version = "...", features = ["tower", "tracing", "azure-auth"] }

# Everything
liter-llm = { version = "...", features = ["full"] }
```

## Flag dependencies

Some flags imply others:

```text
tower   → tracing
otel    → tracing
bedrock → native-http
azure-auth  → native-http
vertex-auth → native-http
bedrock-auth → native-http
copilot-auth → native-http
opendal-cache → tower
full → native-http, tower, tracing, otel, bedrock, tokenizer, azure-auth, vertex-auth, bedrock-auth, copilot-auth, opendal-cache
```

`native-http` is in `default`. Disable it with `default-features = false` when compiling for WebAssembly. The WASM binding uses the browser `fetch` API instead.

## Binary-size considerations

The `tokenizers` crate (pulled in by `tokenizer`) adds roughly 10-15 MB to a release binary due to the HuggingFace tokenizer runtime. Avoid this flag in size-constrained deployments. Token counts are only required when callers need to pre-flight prompt sizes before sending to a provider.

The `opendal` crate (pulled in by `opendal-cache`) varies in size depending on which storage backends are compiled in. Refer to the [OpenDAL documentation](https://opendal.apache.org) for feature-level granularity.

All other flags add less than 1 MB in a typical release build.
