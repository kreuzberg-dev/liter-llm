---
description: "Complete list of 143 supported LLM providers"
---

# Supported Providers

Liter-LLM supports **143 providers** out of the box. Route requests to any provider using the `provider/model` prefix convention -- for example, `openai/gpt-4o` routes to OpenAI and `anthropic/claude-3-opus` routes to Anthropic. No extra configuration is needed beyond setting the provider's API key.

| Provider                       | Prefix                       |        Chat        |     Embeddings     |       Image        |       Audio        |     Moderation     |
| ------------------------------ | ---------------------------- | :----------------: | :----------------: | :----------------: | :----------------: | :----------------: |
| A2A                            | `a2a/`                       | :white_check_mark: |         --         |         --         |         --         |         --         |
| Abliteration                   | `abliteration/`              | :white_check_mark: |         --         |         --         |         --         |         --         |
| AI/ML API                      | `aiml/`                      | :white_check_mark: | :white_check_mark: | :white_check_mark: |         --         |         --         |
| AI21                           | `ai21/`                      | :white_check_mark: |         --         |         --         |         --         |         --         |
| AI21 Chat                      | `ai21_chat/`                 | :white_check_mark: |         --         |         --         |         --         |         --         |
| Amazon Nova                    | `amazon_nova/`               | :white_check_mark: |         --         |         --         |         --         |         --         |
| Anthropic                      | `anthropic/`                 | :white_check_mark: |         --         |         --         |         --         |         --         |
| Anthropic Text                 | `anthropic_text/`            | :white_check_mark: |         --         |         --         |         --         |         --         |
| Apertis                        | `apertis/`                   | :white_check_mark: | :white_check_mark: |         --         |         --         |         --         |
| AssemblyAI                     | `assemblyai/`                | :white_check_mark: |         --         |         --         | :white_check_mark: |         --         |
| Auto Router                    | `auto_router/`               | :white_check_mark: |         --         |         --         |         --         |         --         |
| AWS - Bedrock                  | `bedrock/`                   | :white_check_mark: | :white_check_mark: |         --         |         --         |         --         |
| AWS - Polly                    | `aws_polly/`                 |         --         |         --         |         --         | :white_check_mark: |         --         |
| AWS - Sagemaker                | `sagemaker/`                 | :white_check_mark: | :white_check_mark: |         --         |         --         |         --         |
| AWS S3 Vectors                 | `s3_vectors/`                |         --         |         --         |         --         |         --         |         --         |
| Azure                          | `azure/`                     | :white_check_mark: | :white_check_mark: | :white_check_mark: | :white_check_mark: | :white_check_mark: |
| Azure AI                       | `azure_ai/`                  | :white_check_mark: | :white_check_mark: | :white_check_mark: | :white_check_mark: | :white_check_mark: |
| Azure AI Document Intelligence | `azure_ai/doc-intelligence/` |         --         |         --         |         --         |         --         |         --         |
| Azure AI Foundry Agents        | `azure_ai/agents/`           | :white_check_mark: |         --         |         --         |         --         |         --         |
| Azure Text                     | `azure_text/`                | :white_check_mark: |         --         |         --         | :white_check_mark: | :white_check_mark: |
| Baseten                        | `baseten/`                   | :white_check_mark: |         --         |         --         |         --         |         --         |
| Brave Search                   | `brave/`                     |         --         |         --         |         --         |         --         |         --         |
| Bytez                          | `bytez/`                     | :white_check_mark: |         --         |         --         |         --         |         --         |
| Cerebras                       | `cerebras/`                  | :white_check_mark: |         --         |         --         |         --         |         --         |
| ChatGPT Subscription           | `chatgpt/`                   | :white_check_mark: |         --         |         --         |         --         |         --         |
| Chutes                         | `chutes/`                    | :white_check_mark: | :white_check_mark: |         --         |         --         |         --         |
| Clarifai                       | `clarifai/`                  | :white_check_mark: |         --         |         --         |         --         |         --         |
| Cloudflare AI Workers          | `cloudflare/`                | :white_check_mark: |         --         |         --         |         --         |         --         |
| Codestral                      | `codestral/`                 | :white_check_mark: |         --         |         --         |         --         |         --         |
| Cohere                         | `cohere/`                    | :white_check_mark: | :white_check_mark: |         --         |         --         |         --         |
| Cohere Chat                    | `cohere_chat/`               | :white_check_mark: |         --         |         --         |         --         |         --         |
| CometAPI                       | `cometapi/`                  | :white_check_mark: | :white_check_mark: | :white_check_mark: |         --         |         --         |
| CompactifAI                    | `compactifai/`               | :white_check_mark: |         --         |         --         |         --         |         --         |
| Cursor BYOK                    | `cursor/`                    | :white_check_mark: |         --         |         --         |         --         |         --         |
| Custom                         | `custom/`                    | :white_check_mark: |         --         |         --         |         --         |         --         |
| Custom OpenAI                  | `custom_openai/`             | :white_check_mark: |         --         |         --         | :white_check_mark: | :white_check_mark: |
| Dashscope                      | `dashscope/`                 | :white_check_mark: |         --         |         --         |         --         |         --         |
| Databricks                     | `databricks/`                | :white_check_mark: |         --         |         --         |         --         |         --         |
| DataForSEO                     | `dataforseo/`                |         --         |         --         |         --         |         --         |         --         |
| DataRobot                      | `datarobot/`                 | :white_check_mark: |         --         |         --         |         --         |         --         |
| Deepgram                       | `deepgram/`                  | :white_check_mark: |         --         |         --         | :white_check_mark: |         --         |
| DeepInfra                      | `deepinfra/`                 | :white_check_mark: |         --         |         --         |         --         |         --         |
| Deepseek                       | `deepseek/`                  | :white_check_mark: |         --         |         --         |         --         |         --         |
| Docker Model Runner            | `docker_model_runner/`       | :white_check_mark: |         --         |         --         |         --         |         --         |
| DuckDuckGo                     | `duckduckgo/`                |         --         |         --         |         --         |         --         |         --         |
| ElevenLabs                     | `elevenlabs/`                |         --         |         --         |         --         | :white_check_mark: |         --         |
| Empower                        | `empower/`                   | :white_check_mark: |         --         |         --         |         --         |         --         |
| Exa AI                         | `exa_ai/`                    |         --         |         --         |         --         |         --         |         --         |
| Fal AI                         | `fal_ai/`                    | :white_check_mark: |         --         | :white_check_mark: |         --         |         --         |
| Featherless AI                 | `featherless_ai/`            | :white_check_mark: |         --         |         --         |         --         |         --         |
| Firecrawl                      | `firecrawl/`                 |         --         |         --         |         --         |         --         |         --         |
| Fireworks AI                   | `fireworks_ai/`              | :white_check_mark: |         --         |         --         |         --         |         --         |
| FriendliAI                     | `friendliai/`                | :white_check_mark: |         --         |         --         |         --         |         --         |
| Galadriel                      | `galadriel/`                 | :white_check_mark: |         --         |         --         |         --         |         --         |
| GigaChat                       | `gigachat/`                  | :white_check_mark: | :white_check_mark: |         --         |         --         |         --         |
| GitHub Copilot                 | `github_copilot/`            | :white_check_mark: |         --         |         --         |         --         |         --         |
| GitHub Models                  | `github/`                    | :white_check_mark: |         --         |         --         |         --         |         --         |
| GMI Cloud                      | `gmi/`                       | :white_check_mark: |         --         |         --         |         --         |         --         |
| Google - Vertex AI             | `vertex_ai/`                 | :white_check_mark: | :white_check_mark: | :white_check_mark: | :white_check_mark: |         --         |
| Google AI Studio - Gemini      | `gemini/`                    | :white_check_mark: |         --         |         --         |         --         |         --         |
| Google PSE                     | `google_pse/`                |         --         |         --         |         --         |         --         |         --         |
| GradientAI                     | `gradient_ai/`               | :white_check_mark: |         --         |         --         |         --         |         --         |
| Groq AI                        | `groq/`                      | :white_check_mark: |         --         |         --         |         --         |         --         |
| Helicone                       | `helicone/`                  | :white_check_mark: |         --         |         --         |         --         |         --         |
| Heroku                         | `heroku/`                    | :white_check_mark: |         --         |         --         |         --         |         --         |
| Hosted VLLM                    | `hosted_vllm/`               | :white_check_mark: | :white_check_mark: |         --         |         --         |         --         |
| Huggingface                    | `huggingface/`               | :white_check_mark: | :white_check_mark: |         --         |         --         |         --         |
| Hyperbolic                     | `hyperbolic/`                | :white_check_mark: |         --         |         --         |         --         |         --         |
| IBM - Watsonx.ai               | `watsonx/`                   | :white_check_mark: | :white_check_mark: |         --         | :white_check_mark: |         --         |
| Infinity                       | `infinity/`                  |         --         | :white_check_mark: |         --         |         --         |         --         |
| Jina AI                        | `jina_ai/`                   |         --         | :white_check_mark: |         --         |         --         |         --         |
| Lambda AI                      | `lambda_ai/`                 | :white_check_mark: |         --         |         --         |         --         |         --         |
| LangGraph                      | `langgraph/`                 | :white_check_mark: |         --         |         --         |         --         |         --         |
| Lemonade                       | `lemonade/`                  | :white_check_mark: |         --         |         --         |         --         |         --         |
| Linkup                         | `linkup/`                    |         --         |         --         |         --         |         --         |         --         |
| LiteLLM Proxy                  | `litellm_proxy/`             | :white_check_mark: | :white_check_mark: | :white_check_mark: |         --         |         --         |
| llama.cpp                      | `llamacpp/`                  | :white_check_mark: | :white_check_mark: |         --         |         --         |         --         |
| llamafile                      | `llamafile/`                 | :white_check_mark: | :white_check_mark: |         --         |         --         |         --         |
| LlamaGate                      | `llamagate/`                 | :white_check_mark: |         --         |         --         |         --         |         --         |
| LM Studio                      | `lmstudio/`                  | :white_check_mark: | :white_check_mark: |         --         |         --         |         --         |
| LocalAI                        | `localai/`                   | :white_check_mark: | :white_check_mark: | :white_check_mark: | :white_check_mark: |         --         |
| Manus                          | `manus/`                     | :white_check_mark: |         --         |         --         |         --         |         --         |
| Maritalk                       | `maritalk/`                  | :white_check_mark: |         --         |         --         |         --         |         --         |
| Meta - Llama API               | `meta_llama/`                | :white_check_mark: |         --         |         --         |         --         |         --         |
| Milvus                         | `milvus/`                    |         --         |         --         |         --         |         --         |         --         |
| Minimax                        | `minimax/`                   | :white_check_mark: |         --         |         --         |         --         |         --         |
| Mistral AI API                 | `mistral/`                   | :white_check_mark: | :white_check_mark: |         --         | :white_check_mark: |         --         |
| Moonshot                       | `moonshot/`                  | :white_check_mark: |         --         |         --         |         --         |         --         |
| Morph                          | `morph/`                     | :white_check_mark: |         --         |         --         |         --         |         --         |
| NanoGPT                        | `nanogpt/`                   | :white_check_mark: | :white_check_mark: |         --         |         --         |         --         |
| Nebius AI Studio               | `nebius/`                    | :white_check_mark: | :white_check_mark: |         --         |         --         |         --         |
| NLP Cloud                      | `nlp_cloud/`                 | :white_check_mark: |         --         |         --         |         --         |         --         |
| Novita AI                      | `novita/`                    | :white_check_mark: |         --         |         --         |         --         |         --         |
| Nscale                         | `nscale/`                    | :white_check_mark: |         --         |         --         |         --         |         --         |
| Nvidia NIM                     | `nvidia_nim/`                | :white_check_mark: | :white_check_mark: |         --         |         --         |         --         |
| OCI                            | `oci/`                       | :white_check_mark: |         --         |         --         |         --         |         --         |
| Ollama                         | `ollama/`                    | :white_check_mark: | :white_check_mark: |         --         |         --         |         --         |
| Oobabooga                      | `oobabooga/`                 | :white_check_mark: |         --         |         --         | :white_check_mark: | :white_check_mark: |
| OpenAI                         | `openai/`                    | :white_check_mark: | :white_check_mark: | :white_check_mark: | :white_check_mark: | :white_check_mark: |
| OpenAI-like                    | `openai_like/`               |         --         | :white_check_mark: |         --         |         --         |         --         |
| OpenRouter                     | `openrouter/`                | :white_check_mark: | :white_check_mark: |         --         |         --         |         --         |
| OVHCloud AI Endpoints          | `ovhcloud/`                  | :white_check_mark: |         --         |         --         | :white_check_mark: |         --         |
| Parallel AI                    | `parallel_ai/`               |         --         |         --         |         --         |         --         |         --         |
| Perplexity AI                  | `perplexity/`                | :white_check_mark: |         --         |         --         |         --         |         --         |
| Petals                         | `petals/`                    | :white_check_mark: |         --         |         --         |         --         |         --         |
| PG Vector                      | `pg_vector/`                 |         --         |         --         |         --         |         --         |         --         |
| Poe                            | `poe/`                       | :white_check_mark: | :white_check_mark: |         --         |         --         |         --         |
| Predibase                      | `predibase/`                 | :white_check_mark: |         --         |         --         |         --         |         --         |
| PublicAI                       | `publicai/`                  | :white_check_mark: |         --         |         --         |         --         |         --         |
| Pydantic AI Agents             | `pydantic_ai_agents/`        |         --         |         --         |         --         |         --         |         --         |
| RAGFlow                        | `ragflow/`                   | :white_check_mark: |         --         |         --         |         --         |         --         |
| Recraft                        | `recraft/`                   |         --         |         --         | :white_check_mark: |         --         |         --         |
| Replicate                      | `replicate/`                 | :white_check_mark: |         --         |         --         |         --         |         --         |
| RunwayML                       | `runwayml/`                  |         --         |         --         | :white_check_mark: | :white_check_mark: |         --         |
| Sagemaker Chat                 | `sagemaker_chat/`            | :white_check_mark: |         --         |         --         |         --         |         --         |
| Sambanova                      | `sambanova/`                 | :white_check_mark: |         --         |         --         |         --         |         --         |
| SAP Generative AI Hub          | `sap/`                       | :white_check_mark: |         --         |         --         |         --         |         --         |
| Sarvam                         | `sarvam/`                    | :white_check_mark: |         --         |         --         |         --         |         --         |
| Scaleway                       | `scaleway/`                  | :white_check_mark: |         --         |         --         |         --         |         --         |
| SearXNG                        | `searxng/`                   |         --         |         --         |         --         |         --         |         --         |
| Serper                         | `serper/`                    |         --         |         --         |         --         |         --         |         --         |
| Snowflake                      | `snowflake/`                 | :white_check_mark: |         --         |         --         |         --         |         --         |
| Stability AI                   | `stability/`                 |         --         |         --         | :white_check_mark: |         --         |         --         |
| Synthetic                      | `synthetic/`                 | :white_check_mark: | :white_check_mark: |         --         |         --         |         --         |
| Tavily                         | `tavily/`                    |         --         |         --         |         --         |         --         |         --         |
| Text Completion Codestral      | `text-completion-codestral/` | :white_check_mark: |         --         |         --         |         --         |         --         |
| Text Completion OpenAI         | `text-completion-openai/`    | :white_check_mark: |         --         |         --         | :white_check_mark: | :white_check_mark: |
| Together AI                    | `together_ai/`               | :white_check_mark: |         --         |         --         |         --         |         --         |
| Topaz                          | `topaz/`                     |         --         |         --         |         --         |         --         |         --         |
| Triton                         | `triton/`                    | :white_check_mark: |         --         |         --         |         --         |         --         |
| V0                             | `v0/`                        | :white_check_mark: |         --         |         --         |         --         |         --         |
| Venice.ai                      | `venice/`                    | :white_check_mark: |         --         |         --         |         --         |         --         |
| Vercel AI Gateway              | `vercel_ai_gateway/`         | :white_check_mark: |         --         |         --         |         --         |         --         |
| Vertex AI Agent Engine         | `vertex_ai/agent_engine/`    | :white_check_mark: |         --         |         --         |         --         |         --         |
| vLLM                           | `vllm/`                      | :white_check_mark: | :white_check_mark: |         --         |         --         |         --         |
| Volcengine                     | `volcengine/`                | :white_check_mark: |         --         |         --         |         --         |         --         |
| Voyage AI                      | `voyage/`                    |         --         | :white_check_mark: |         --         |         --         |         --         |
| WandB Inference                | `wandb/`                     | :white_check_mark: |         --         |         --         |         --         |         --         |
| Watsonx Text                   | `watsonx_text/`              | :white_check_mark: |         --         |         --         |         --         |         --         |
| xAI                            | `xai/`                       | :white_check_mark: |         --         |         --         |         --         |         --         |
| Xiaomi Mimo                    | `xiaomi_mimo/`               | :white_check_mark: |         --         |         --         |         --         |         --         |
| Xinference                     | `xinference/`                |         --         | :white_check_mark: |         --         |         --         |         --         |
| Z.AI                           | `zai/`                       | :white_check_mark: |         --         |         --         |         --         |         --         |

_143 providers total._

## Usage

Use any provider by prefixing the model name with the provider's routing prefix:

```python
from liter_llm import LiterLLM

client = LiterLLM()

# OpenAI
response = await client.chat("openai/gpt-4o", messages=[
    {"role": "user", "content": "Hello!"}
])

# Anthropic
response = await client.chat("anthropic/claude-3-opus", messages=[
    {"role": "user", "content": "Hello!"}
])

# Groq
response = await client.chat("groq/llama3-70b", messages=[
    {"role": "user", "content": "Hello!"}
])
```

## Custom Providers

Any OpenAI-compatible API can be used as a custom provider by setting the base URL and API key directly:

```python
response = await client.chat("custom/my-model",
    base_url="https://my-api.example.com/v1",
    api_key="my-key",
    messages=[
        {"role": "user", "content": "Hello!"}
    ]
)
```

## Provider Registry

The full provider registry with base URLs, auth configuration, and model mappings is available at [schemas/providers.json](https://github.com/kreuzberg-dev/liter-llm/blob/main/schemas/providers.json).
