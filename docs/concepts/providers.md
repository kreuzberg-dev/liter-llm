---
description: "How liter-llm routes requests to 142 LLM providers using an embedded registry and model prefix convention."
---

# Provider Registry

liter-llm ships with a compiled-in registry of **142 providers**. No runtime configuration files, no network fetches -- the registry is embedded in the binary at build time from `schemas/providers.json`.

## Model Routing

Requests are routed to providers using a `provider/model` prefix convention:

| Model string | Provider | Actual model sent |
| --- | --- | --- |
| `openai/gpt-4o` | OpenAI | `gpt-4o` |
| `anthropic/claude-3-5-sonnet-20241022` | Anthropic | `claude-3-5-sonnet-20241022` |
| `groq/llama-3.1-70b-versatile` | Groq | `llama-3.1-70b-versatile` |
| `bedrock/anthropic.claude-3-sonnet` | AWS Bedrock | `anthropic.claude-3-sonnet` |
| `gemini/gemini-1.5-pro` | Google AI Studio | `gemini-1.5-pro` |
| `mistral/mistral-large-latest` | Mistral AI | `mistral-large-latest` |
| `ollama/llama3` | Ollama (local) | `llama3` |

The prefix is stripped before the request is forwarded to the provider's API.

## How the Registry Works

Each entry in `schemas/providers.json` defines:

- **Base URL** -- the provider's API endpoint
- **Auth format** -- how the API key is sent (Bearer token, custom header, etc.)
- **Model prefixes** -- which prefixes route to this provider
- **Parameter mappings** -- provider-specific field name translations
- **Supported capabilities** -- chat, embeddings, images, audio, moderation

The registry is compiled into the binary with `include_str!` and deserialized once at startup. Provider resolution at client construction is a single hashmap lookup.

## Auth Patterns

Providers use different authentication mechanisms. liter-llm handles all of them transparently:

| Pattern | Providers | Header |
| --- | --- | --- |
| Bearer token | OpenAI, Mistral, Groq, most providers | `Authorization: Bearer <key>` |
| API key header | Anthropic | `x-api-key: <key>` |
| AWS Signature V4 | Bedrock, Sagemaker | SigV4 signing |
| OAuth / ADC | Google Vertex AI | Application Default Credentials |
| None | Ollama, LM Studio, local providers | No auth header |

API keys are wrapped in `secrecy::SecretString` and are never logged, serialized, or exposed in error messages.

## Custom and Local Providers

Any OpenAI-compatible API can be used by overriding the base URL at client construction. This works for local inference servers and proxies.

### Ollama

```python
from liter_llm import LlmClient

client = LlmClient(api_key="unused", base_url="http://localhost:11434/v1")
response = await client.chat(
    model="ollama/llama3",
    messages=[{"role": "user", "content": "Hello!"}],
)
```

### vLLM

```python
client = LlmClient(api_key="unused", base_url="http://localhost:8000/v1")
response = await client.chat(
    model="hosted_vllm/meta-llama/Llama-3-8B-Instruct",
    messages=[{"role": "user", "content": "Hello!"}],
)
```

### LM Studio

```python
client = LlmClient(api_key="unused", base_url="http://localhost:1234/v1")
response = await client.chat(
    model="lm_studio/my-local-model",
    messages=[{"role": "user", "content": "Hello!"}],
)
```

!!! tip "Environment variables"
    Most providers have a standard environment variable for the API key (e.g. `OPENAI_API_KEY`, `ANTHROPIC_API_KEY`). Read these in your code and pass them to the `api_key` constructor parameter: `LlmClient(api_key=os.environ["OPENAI_API_KEY"])`.

## Full Provider List

See the [Providers](../providers.md) page for the complete table of all 142 supported providers with their prefixes and capability matrix.
