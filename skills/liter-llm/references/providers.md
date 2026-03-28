# Providers Reference

Liter-LLM supports 142 providers out of the box via an embedded provider registry (`schemas/providers.json`).

## Routing Convention

Route requests using the `provider/model` prefix convention. The prefix before the first `/` determines the provider:

```text
openai/gpt-4o          -> OpenAI
anthropic/claude-3-opus -> Anthropic
groq/llama3-70b        -> Groq
mistral/mistral-large   -> Mistral
bedrock/anthropic.claude-3 -> AWS Bedrock
vertex_ai/gemini-pro   -> Google Vertex AI
```

No extra configuration needed beyond setting the provider's API key.

## Major Providers

### OpenAI (`openai/`)

Full API support: chat, embeddings, images, audio, moderation, files, batches, responses.

```python
client = LlmClient(api_key=os.environ["OPENAI_API_KEY"])
response = await client.chat(model="openai/gpt-4o", messages=[...])
```

### Anthropic (`anthropic/`)

Chat completions. Note: Anthropic requires `max_tokens` -- the client sets a default if omitted.

```python
client = LlmClient(api_key=os.environ["ANTHROPIC_API_KEY"])
response = await client.chat(model="anthropic/claude-3-opus", messages=[...])
```

### Google Gemini (`gemini/`)

Chat completions via Google AI Studio.

```python
client = LlmClient(api_key=os.environ["GEMINI_API_KEY"])
response = await client.chat(model="gemini/gemini-pro", messages=[...])
```

### Google Vertex AI (`vertex_ai/`)

Chat, embeddings, images, audio. Requires OAuth2 credentials (not API key). Use the `VertexOAuth2Provider` credential provider.

```python
client = LlmClient(api_key="", base_url="https://us-central1-aiplatform.googleapis.com/v1")
response = await client.chat(model="vertex_ai/gemini-pro", messages=[...])
```

### AWS Bedrock (`bedrock/`)

Chat and embeddings. Uses SigV4 request signing. Requires `AWS_ACCESS_KEY_ID` + `AWS_SECRET_ACCESS_KEY`.

```python
client = LlmClient(api_key=os.environ["AWS_ACCESS_KEY_ID"])
response = await client.chat(model="bedrock/anthropic.claude-3-sonnet", messages=[...])
```

### Azure (`azure/`)

Full API support. Uses `api-key` header (not Bearer token). Base URL format: `https://{resource}.openai.azure.com/openai/deployments/{deployment}`.

```python
client = LlmClient(
    api_key=os.environ["AZURE_OPENAI_API_KEY"],
    base_url="https://myresource.openai.azure.com/openai/deployments/gpt-4"
)
response = await client.chat(model="azure/gpt-4", messages=[...])
```

### Groq (`groq/`)

Fast inference. Chat completions.

```python
client = LlmClient(api_key=os.environ["GROQ_API_KEY"])
response = await client.chat(model="groq/llama3-70b", messages=[...])
```

### Mistral (`mistral/`)

Chat and embeddings. Also supports OCR via `mistral/mistral-ocr-latest`.

```python
client = LlmClient(api_key=os.environ["MISTRAL_API_KEY"])
response = await client.chat(model="mistral/mistral-large-latest", messages=[...])
```

### Cohere (`cohere/`)

Chat and embeddings. Reranking support.

```python
client = LlmClient(api_key=os.environ["CO_API_KEY"])
response = await client.chat(model="cohere/command-r-plus", messages=[...])
```

### DeepSeek (`deepseek/`)

Chat completions.

```python
client = LlmClient(api_key=os.environ["DEEPSEEK_API_KEY"])
response = await client.chat(model="deepseek/deepseek-chat", messages=[...])
```

### OpenRouter (`openrouter/`)

Aggregator -- access many models through one API key. Chat and embeddings.

```python
client = LlmClient(api_key=os.environ["OPENROUTER_API_KEY"])
response = await client.chat(model="openrouter/anthropic/claude-3-opus", messages=[...])
```

### Ollama (`ollama/`)

Local inference. Chat and embeddings. No API key required.

```python
client = LlmClient(api_key="unused", base_url="http://localhost:11434/v1")
response = await client.chat(model="ollama/llama3", messages=[...])
```

### Together AI (`together_ai/`)

Chat completions.

```python
client = LlmClient(api_key=os.environ["TOGETHER_API_KEY"])
response = await client.chat(model="together_ai/meta-llama/Llama-3-70b-chat-hf", messages=[...])
```

## Auth Types

The provider registry defines three auth header patterns:

| Auth Type | Header Format | Providers |
|-----------|--------------|-----------|
| Bearer | `Authorization: Bearer {key}` | Most providers (OpenAI, Anthropic, Groq, Mistral, etc.) |
| ApiKey | `api-key: {key}` | Azure |
| None | No auth header | Ollama, local servers |

## Capability Matrix (Selected)

| Provider | Chat | Embed | Image | Audio | Moderation | Search | OCR |
|----------|:----:|:-----:|:-----:|:-----:|:----------:|:------:|:---:|
| OpenAI | Y | Y | Y | Y | Y | - | - |
| Anthropic | Y | - | - | - | - | - | - |
| Azure | Y | Y | Y | Y | Y | - | - |
| Vertex AI | Y | Y | Y | Y | - | - | - |
| Bedrock | Y | Y | - | - | - | - | - |
| Groq | Y | - | - | - | - | - | - |
| Mistral | Y | Y | - | - | - | - | Y |
| Cohere | Y | Y | - | - | - | - | - |
| Ollama | Y | Y | - | - | - | - | - |
| Brave | - | - | - | - | - | Y | - |
| Tavily | - | - | - | - | - | Y | - |

## Custom Provider Registration

Register any OpenAI-compatible API at runtime:

```python
# Python
client.register_provider({
    "name": "my-provider",
    "base_url": "https://my-llm.example.com/v1",
    "auth_header": "Authorization",
    "model_prefixes": ["my-provider/"],
})
# Use: model="my-provider/my-model"
client.unregister_provider("my-provider")
```

```typescript
// TypeScript
client.registerProvider({
  name: "my-provider",
  baseUrl: "https://my-llm.example.com/v1",
  authHeader: "Authorization",
  modelPrefixes: ["my-provider/"],
});
client.unregisterProvider("my-provider");
```

```go
// Go
client.RegisterProvider(&literllm.ProviderConfig{
    Prefix:     "custom",
    BaseURL:    "https://api.custom-llm.com/v1",
    AuthHeader: "Authorization",
})
client.UnregisterProvider("custom")
```

## Provider-Specific Notes

- **Anthropic**: Requires `max_tokens` on every request. The client injects a default (4096) if not provided.
- **AWS Bedrock**: Uses SigV4 request signing, not Bearer tokens. Configure with `AWS_ACCESS_KEY_ID` + `AWS_SECRET_ACCESS_KEY` + optional `AWS_SESSION_TOKEN`.
- **Google Vertex AI**: Uses OAuth2 bearer tokens, not API keys. Use `VertexOAuth2Provider` credential provider for automatic token refresh.
- **Azure**: Uses `api-key` header instead of `Authorization: Bearer`. Base URL must include the deployment path.
- **Ollama**: No API key required. Set `api_key=""` and `base_url="http://localhost:11434/v1"`.
- **OpenRouter**: Supports model routing through their aggregation layer. Prefix with `openrouter/` then the underlying model identifier.
- **Search providers** (Brave, Tavily, Serper, etc.): Only support the `search()` endpoint, not chat/embeddings.
- **The provider registry** at `schemas/providers.json` is embedded at compile time -- update it and rebuild to add new providers.

## Provider Registry Schema

Each provider entry in `schemas/providers.json` defines:

- `base_url` -- API endpoint
- `auth_header` -- Header name for the API key
- `auth_prefix` -- Value prefix (e.g. `"Bearer "`)
- `model_prefixes` -- Routing prefixes that map to this provider
- `supported_endpoints` -- Which API endpoints the provider supports
- `parameter_mappings` -- Provider-specific parameter transformations
