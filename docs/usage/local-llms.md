---
description: "Run LLMs locally with Ollama, LM Studio, vLLM, llama.cpp, LocalAI, and llamafile -- no API key required."
---

# Local LLMs

liter-llm supports local inference engines that expose an OpenAI-compatible API. Run models on your own hardware with zero cloud dependencies and no API key.

## Supported Providers

| Provider                                               | Default URL                 | Prefix                      | Notes                        |
| ------------------------------------------------------ | --------------------------- | --------------------------- | ---------------------------- |
| [Ollama](https://ollama.ai)                            | `http://localhost:11434/v1` | `ollama/`                   | Most popular, easy setup     |
| [LM Studio](https://lmstudio.ai)                       | `http://localhost:1234/v1`  | `lm_studio/` or `lmstudio/` | GUI-based, beginner-friendly |
| [vLLM](https://docs.vllm.ai)                           | `http://localhost:8000/v1`  | `vllm/`                     | High-throughput serving      |
| [llama.cpp](https://github.com/ggerganov/llama.cpp)    | `http://localhost:8080/v1`  | `llamacpp/`                 | Lightweight C++ inference    |
| [LocalAI](https://localai.io)                          | `http://localhost:8080/v1`  | `localai/`                  | Drop-in OpenAI replacement   |
| [llamafile](https://github.com/Mozilla-Ocho/llamafile) | `http://localhost:8080/v1`  | `llamafile/`                | Single-file executable       |

All of these providers are registered in the [provider registry](../providers.md) with their default base URLs. liter-llm routes requests automatically based on the model prefix.

## Supported Capabilities

| Provider  |        Chat        |    Completions     |     Embeddings     |       Rerank       |       Audio        |       Images       |
| --------- | :----------------: | :----------------: | :----------------: | :----------------: | :----------------: | :----------------: |
| Ollama    | :white_check_mark: | :white_check_mark: | :white_check_mark: |         —          |         —          |         —          |
| LM Studio | :white_check_mark: | :white_check_mark: | :white_check_mark: |         —          |         —          |         —          |
| vLLM      | :white_check_mark: | :white_check_mark: | :white_check_mark: | :white_check_mark: |         —          |         —          |
| llama.cpp | :white_check_mark: | :white_check_mark: | :white_check_mark: | :white_check_mark: |         —          |         —          |
| LocalAI   | :white_check_mark: | :white_check_mark: | :white_check_mark: | :white_check_mark: | :white_check_mark: | :white_check_mark: |
| llamafile | :white_check_mark: | :white_check_mark: | :white_check_mark: | :white_check_mark: |         —          |         —          |

All providers also support streaming via SSE and model listing via `/v1/models`.
Tool calling and vision/multimodal are supported through the chat endpoint where the underlying model supports it.

## Quick Start with Ollama

### 1. Install Ollama

```bash
# macOS / Linux
curl -fsSL https://ollama.ai/install.sh | sh

# Or via Homebrew
brew install ollama
```

### 2. Pull a Model

```bash
ollama pull qwen2:0.5b
```

### 3. Use with liter-llm

=== "Python"

    --8<-- "snippets/python/usage/local_llm.md"

=== "TypeScript"

    --8<-- "snippets/typescript/usage/local_llm.md"

=== "Rust"

    --8<-- "snippets/rust/usage/local_llm.md"

=== "Go"

    --8<-- "snippets/go/usage/local_llm.md"

!!! tip "No API key required"
Local providers do not require an API key. Pass an empty string (`""`) as the `api_key` parameter.

## Model Naming Convention

liter-llm uses the standard `provider/model-name` prefix convention for local providers, just like cloud providers:

```text
ollama/llama3.2          -> Ollama running Llama 3.2
ollama/qwen2:0.5b        -> Ollama running Qwen2 0.5B
lm_studio/my-model       -> LM Studio
vllm/meta-llama/Llama-3  -> vLLM
llamacpp/my-model         -> llama.cpp server
localai/gpt-3.5-turbo    -> LocalAI
llamafile/my-model        -> llamafile
```

The prefix determines which base URL and configuration to use. The model name after the `/` is forwarded to the local server as-is.

## Streaming

All local providers support streaming responses via Server-Sent Events (SSE), identical to the cloud provider streaming interface:

=== "Python"

    ```python
    async for chunk in client.chat_stream(
        model="ollama/qwen2:0.5b",
        messages=[{"role": "user", "content": "Hello!"}],
    ):
        print(chunk.choices[0].delta.content, end="")
    ```

=== "Rust"

    ```rust
    use futures_util::StreamExt;

    let mut stream = client.chat_stream(request).await?;
    while let Some(chunk) = stream.next().await {
        let chunk = chunk?;
        if let Some(content) = &chunk.choices[0].delta.content {
            print!("{content}");
        }
    }
    ```

## Embeddings

Several local providers support embedding models. Use the standard embeddings API:

=== "Python"

    ```python
    response = await client.embed(
        model="ollama/all-minilm",
        input="The quick brown fox",
    )
    print(f"Dimensions: {len(response.data[0].embedding)}")
    ```

=== "Rust"

    ```rust
    let response = client.embed(EmbeddingRequest {
        model: "ollama/all-minilm".into(),
        input: EmbeddingInput::Single("The quick brown fox".into()),
        ..Default::default()
    }).await?;
    ```

Popular local embedding models include `all-minilm` (384 dims), `nomic-embed-text` (768 dims), and `mxbai-embed-large` (1024 dims) on Ollama.

## Provider Configuration

### Ollama

Ollama runs on port 11434 by default. No additional configuration is needed:

```toml
# liter-llm.toml
api_key = ""

[[providers]]
name = "ollama"
base_url = "http://localhost:11434/v1"
model_prefixes = ["ollama/"]
```

!!! note "Ollama model names"
Ollama uses its own model naming (e.g., `llama3.2`, `qwen2:0.5b`, `codellama:13b`). Use `ollama list` to see installed models.

### LM Studio

LM Studio runs on port 1234 by default. Load a model in the LM Studio GUI, then use it:

```toml
# liter-llm.toml
api_key = ""

[[providers]]
name = "lm_studio"
base_url = "http://localhost:1234/v1"
model_prefixes = ["lm_studio/", "lmstudio/"]
```

### vLLM

Start vLLM with the OpenAI-compatible server:

```bash
python -m vllm.entrypoints.openai.api_server \
    --model meta-llama/Llama-3-8B \
    --port 8000
```

```toml
# liter-llm.toml
api_key = ""

[[providers]]
name = "vllm"
base_url = "http://localhost:8000/v1"
model_prefixes = ["vllm/"]
```

### llama.cpp

Start the llama.cpp server:

```bash
./llama-server -m model.gguf --port 8080
```

```toml
# liter-llm.toml
api_key = ""

[[providers]]
name = "llamacpp"
base_url = "http://localhost:8080/v1"
model_prefixes = ["llamacpp/"]
```

### LocalAI

```bash
docker run -p 8080:8080 localai/localai:latest
```

```toml
# liter-llm.toml
api_key = ""

[[providers]]
name = "localai"
base_url = "http://localhost:8080/v1"
model_prefixes = ["localai/"]
```

### llamafile

Download and run a llamafile:

```bash
chmod +x llava-v1.5-7b-q4.llamafile
./llava-v1.5-7b-q4.llamafile --server --port 8080
```

```toml
# liter-llm.toml
api_key = ""

[[providers]]
name = "llamafile"
base_url = "http://localhost:8080/v1"
model_prefixes = ["llamafile/"]
```

## Custom Base URL

If your local provider runs on a non-default port or remote host, override the base URL:

=== "Python"

    ```python
    client = LlmClient(api_key="", base_url="http://192.168.1.100:9000/v1")
    ```

=== "TypeScript"

    ```typescript
    const client = new LlmClient({
      apiKey: "",
      baseUrl: "http://192.168.1.100:9000/v1",
    });
    ```

=== "Rust"

    ```rust
    let config = ClientConfigBuilder::new("")
        .base_url("http://192.168.1.100:9000/v1")
        .build();
    ```

=== "Go"

    ```go
    client := llm.NewClient(
        llm.WithAPIKey(""),
        llm.WithBaseURL("http://192.168.1.100:9000/v1"),
    )
    ```

Or in `liter-llm.toml`:

```toml
api_key = ""
base_url = "http://192.168.1.100:9000/v1"
```

## Docker Compose

Run Ollama alongside the liter-llm proxy for a self-contained local setup:

```yaml
# docker-compose.local.yaml
services:
  ollama:
    image: ollama/ollama:latest
    ports:
      - "11434:11434"
    volumes:
      - ollama_data:/root/.ollama

  liter-llm:
    image: ghcr.io/kreuzberg-dev/liter-llm:latest
    ports:
      - "4000:4000"
    environment:
      - LITER_LLM_API_KEY=""
    volumes:
      - ./liter-llm-proxy.toml:/etc/liter-llm/liter-llm-proxy.toml
    depends_on:
      - ollama

volumes:
  ollama_data:
```

Example proxy config for local use:

```toml
# liter-llm-proxy.toml
[server]
host = "0.0.0.0"
port = 4000

[[providers]]
name = "ollama"
base_url = "http://ollama:11434/v1"
model_prefixes = ["ollama/"]
```

Start the stack:

```bash
docker compose -f docker-compose.local.yaml up -d

# Pull a model into Ollama
docker exec -it $(docker compose -f docker-compose.local.yaml ps -q ollama) \
    ollama pull qwen2:0.5b

# Chat via the proxy
curl http://localhost:4000/v1/chat/completions \
    -H "Content-Type: application/json" \
    -d '{"model": "ollama/qwen2:0.5b", "messages": [{"role": "user", "content": "Hello!"}]}'
```

## Troubleshooting

### Connection Refused

```text
Error: connection refused (os error 111)
```

The local server is not running or is on a different port. Verify:

```bash
# Check if Ollama is running
curl http://localhost:11434/v1/models

# Check if the port is in use
lsof -i :11434
```

!!! tip
Make sure the server is started **before** making requests. Ollama starts automatically on macOS but may need `ollama serve` on Linux.

### Model Not Found

```text
Error: model "llama3.2" not found
```

The model is not downloaded. Pull it first:

```bash
# Ollama
ollama pull llama3.2

# Check installed models
ollama list
```

### Timeout Errors

Local models can be slow to load on first request (especially large models). Increase the timeout:

```toml
# liter-llm.toml
timeout_secs = 300  # 5 minutes for initial model load
```

### Docker Networking

When running liter-llm in Docker and a local provider on the host:

- **Linux**: Use `http://host.docker.internal:11434/v1` or `http://172.17.0.1:11434/v1`
- **macOS/Windows**: Use `http://host.docker.internal:11434/v1`

```toml
# liter-llm-proxy.toml (inside Docker)
[[providers]]
name = "ollama"
base_url = "http://host.docker.internal:11434/v1"
model_prefixes = ["ollama/"]
```

### GPU / Performance

- **Ollama**: Automatically uses GPU if available. Check with `ollama ps`.
- **vLLM**: Pass `--tensor-parallel-size N` for multi-GPU.
- **llama.cpp**: Use `-ngl N` to offload N layers to GPU.
- **LocalAI**: Set `GPU_LAYERS` environment variable.
