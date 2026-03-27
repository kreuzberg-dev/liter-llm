---
description: "How to configure liter-llm clients: API keys, timeouts, retries, custom base URLs, and extra headers."
---

# Configuration

The `LlmClient` constructor accepts configuration for authentication, timeouts, retries, and endpoint overrides. All options have sensible defaults.

## Client Construction

=== "Python"

    --8<-- "snippets/python/guides/configuration.md"

=== "TypeScript"

    --8<-- "snippets/typescript/guides/configuration.md"

=== "Go"

    --8<-- "snippets/go/guides/configuration.md"

## Configuration Options

| Option | Type | Default | Description |
| --- | --- | --- | --- |
| `api_key` | string | **required** | Provider API key. Wrapped in `SecretString` internally. Read from env vars yourself (e.g. `os.environ["OPENAI_API_KEY"]`). |
| `base_url` | string | from registry | Override the provider's base URL. |
| `model_hint` | string | none | Pre-resolve a provider at construction (e.g. `"openai"`). |
| `timeout` | duration | 60s | Request timeout. |
| `max_retries` | int | 3 | Number of retries on transient failures. |

## API Key Management

The recommended approach is environment variables. Read the standard variable for your provider and pass it to the constructor:

| Provider | Environment variable |
| --- | --- |
| OpenAI | `OPENAI_API_KEY` |
| Anthropic | `ANTHROPIC_API_KEY` |
| Google (Gemini) | `GEMINI_API_KEY` |
| Groq | `GROQ_API_KEY` |
| Mistral | `MISTRAL_API_KEY` |
| Cohere | `CO_API_KEY` |
| AWS Bedrock | `AWS_ACCESS_KEY_ID` + `AWS_SECRET_ACCESS_KEY` |

!!! warning "Never hardcode API keys"
    API keys passed to the constructor are wrapped in `secrecy::SecretString`. They are never logged, serialized, or included in error messages. Always prefer environment variables over hardcoded strings.

If you pass `api_key` explicitly, it takes precedence over the environment variable.

## Timeouts and Retries

Retries use exponential backoff with jitter. Only transient failures (HTTP 429, 500, 502, 503, 504) are retried.

```python
# Long timeout for large generations
client = LlmClient(api_key="sk-...", timeout=300, max_retries=5)
```

## Custom Base URLs

Override `base_url` to point at a local inference server, corporate proxy, or alternative endpoint:

```python
# Ollama running locally (no API key needed for local providers)
client = LlmClient(api_key="unused", base_url="http://localhost:11434/v1")

# Corporate proxy
client = LlmClient(api_key="sk-...", base_url="https://llm-proxy.internal.company.com/v1")
```

The `base_url` replaces the provider's default URL from the registry. The model prefix still controls which provider's auth and parameter logic is used.

## Extra Headers

In Rust, the `ClientConfigBuilder` supports adding custom headers:

```rust
use liter_llm::ClientConfigBuilder;

let config = ClientConfigBuilder::new("sk-...")
    .header("X-Custom-Header", "value")?
    .timeout(std::time::Duration::from_secs(120))
    .max_retries(5)
    .build();
```

## Credential Providers

For dynamic credentials (e.g. rotating tokens, AWS IAM roles), pass a `credential_provider` that implements the `CredentialProvider` trait. This is called before each request to fetch the current credential:

```rust
use liter_llm::ClientConfigBuilder;
use std::sync::Arc;

let config = ClientConfigBuilder::new("placeholder")
    .credential_provider(Arc::new(my_credential_provider))
    .build();
```

## Model Hints

The `model_hint` parameter pre-resolves a provider at construction time. This is useful when you know you will only use one provider and want to avoid the prefix lookup on each request:

```python
# All requests will use OpenAI, no prefix lookup needed
client = LlmClient(api_key="sk-...", model_hint="openai")
response = await client.chat(
    model="gpt-4o",  # no "openai/" prefix required
    messages=[{"role": "user", "content": "Hello!"}],
)
```
