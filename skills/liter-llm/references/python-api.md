# Python API Reference

## Installation

```bash
pip install liter-llm
```

## Client

### Constructor

```python
from liter_llm import LlmClient

client = LlmClient(
    *,
    api_key: str,
    base_url: str | None = None,
    model_hint: str | None = None,
    max_retries: int = 3,
    timeout: int = 60,
    cache: dict | None = None,
    budget: dict | None = None,
    extra_headers: dict | None = None,
    cooldown: int | None = None,
    rate_limit: dict | None = None,
    health_check: int | None = None,
    cost_tracking: bool = False,
    tracing: bool = False,
)
```

All parameters are keyword-only. The client is immutable after construction and safe to share across tasks.

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `api_key` | `str` | *required* | API key for authentication (wrapped in `SecretString` internally) |
| `base_url` | `str \| None` | `None` | Override provider base URL |
| `model_hint` | `str \| None` | `None` | Hint for provider auto-detection (e.g. `"groq/llama3-70b"`) |
| `max_retries` | `int` | `3` | Retries on 429 / 5xx responses |
| `timeout` | `int` | `60` | Request timeout in seconds |
| `cache` | `dict \| None` | `None` | Cache config: `{"max_entries": 256, "ttl_seconds": 300}` |
| `budget` | `dict \| None` | `None` | Budget config: `{"global_limit": 10.0, "model_limits": {}, "enforcement": "hard"}` |
| `extra_headers` | `dict \| None` | `None` | Additional HTTP headers sent with every request |
| `cooldown` | `int \| None` | `None` | Cooldown period in seconds after transient errors |
| `rate_limit` | `dict \| None` | `None` | Rate limit config: `{"rpm": 60, "tpm": 100000}` |
| `health_check` | `int \| None` | `None` | Health check interval in seconds |
| `cost_tracking` | `bool` | `False` | Enable per-request cost tracking |
| `tracing` | `bool` | `False` | Enable OpenTelemetry tracing spans |

### Configuration Details

**Cache config fields:**

| Field | Type | Description |
|-------|------|-------------|
| `max_entries` | `int` | Maximum cached responses (default: 256) |
| `ttl_seconds` | `int` | Time-to-live for cached entries (default: 300) |

**Budget config fields:**

| Field | Type | Description |
|-------|------|-------------|
| `global_limit` | `float` | Maximum USD spend across all models |
| `model_limits` | `dict[str, float]` | Per-model USD spend limits |
| `enforcement` | `str` | `"hard"` (reject) or `"soft"` (warn) |

**Rate limit config fields:**

| Field | Type | Description |
|-------|------|-------------|
| `rpm` | `int` | Requests per minute |
| `tpm` | `int` | Tokens per minute |

---

## Methods

All methods are async and must be awaited.

### Core Completion Methods

#### `chat(**kwargs) -> ChatCompletionResponse`

Send a chat completion request.

```python
response = await client.chat(
    model="gpt-4",
    messages=[{"role": "user", "content": "Hello!"}],
    temperature=0.7,
    max_tokens=256,
)
print(response.choices[0].message.content)
```

Accepts the same keyword arguments as the OpenAI Chat Completions API: `model`, `messages`, `temperature`, `top_p`, `max_tokens`, `tools`, `tool_choice`, `response_format`, `stream` (ignored), `n`, `stop`, `presence_penalty`, `frequency_penalty`, `user`.

#### `chat_stream(**kwargs) -> ChatStreamIterator`

Start a streaming chat completion. Returns an async iterator that yields `ChatCompletionChunk` objects. The HTTP request is issued immediately when the method is called.

```python
iterator = await client.chat_stream(model="gpt-4", messages=[...])
async for chunk in iterator:
    print(chunk.choices[0].delta.content, end="")
```

Supports `async with` for deterministic resource cleanup:

```python
async with await client.chat_stream(model="gpt-4", messages=[...]) as stream:
    async for chunk in stream:
        print(chunk.choices[0].delta.content, end="")
```

Call `iterator.cancel()` to signal the background task to stop early.

### Embedding

#### `embed(**kwargs) -> EmbeddingResponse`

Send an embedding request.

```python
response = await client.embed(
    model="text-embedding-3-small",
    input="The quick brown fox",
)
vector = response.data[0].embedding  # list[float]
```

| Parameter | Type | Description |
|-----------|------|-------------|
| `model` | `str` | Embedding model name |
| `input` | `str \| list[str]` | Text(s) to embed |
| `encoding_format` | `str \| None` | `"float"` or `"base64"` |
| `dimensions` | `int \| None` | Output dimensions (model-dependent) |
| `user` | `str \| None` | End-user identifier |

### Model Discovery

#### `list_models() -> ModelsListResponse`

List available models from the provider.

```python
models = await client.list_models()
for m in models.data:
    print(m.id)
```

### Image Generation

#### `image_generate(**kwargs) -> ImagesResponse`

Generate images from a text prompt.

```python
response = await client.image_generate(
    model="dall-e-3",
    prompt="A sunset over mountains",
    n=1,
    size="1024x1024",
)
```

| Parameter | Type | Description |
|-----------|------|-------------|
| `prompt` | `str` | Text description of the image |
| `model` | `str` | Image generation model |
| `n` | `int \| None` | Number of images to generate |
| `size` | `str \| None` | Image size (e.g. `"1024x1024"`) |
| `quality` | `str \| None` | Quality level (`"standard"`, `"hd"`) |
| `response_format` | `str \| None` | `"url"` or `"b64_json"` |
| `style` | `str \| None` | Style (`"vivid"`, `"natural"`) |
| `user` | `str \| None` | End-user identifier |

### Audio

#### `speech(**kwargs) -> bytes`

Generate speech audio from text. Returns raw audio bytes.

```python
audio = await client.speech(
    model="tts-1",
    input="Hello world",
    voice="alloy",
)
with open("output.mp3", "wb") as f:
    f.write(audio)
```

| Parameter | Type | Description |
|-----------|------|-------------|
| `model` | `str` | TTS model |
| `input` | `str` | Text to speak |
| `voice` | `str` | Voice name |
| `response_format` | `str \| None` | Audio format (`"mp3"`, `"opus"`, `"aac"`, `"flac"`) |
| `speed` | `float \| None` | Speed multiplier (0.25 to 4.0) |

#### `transcribe(**kwargs) -> TranscriptionResponse`

Transcribe audio into text.

```python
response = await client.transcribe(
    model="whisper-1",
    file=audio_bytes,
)
print(response.text)
```

| Parameter | Type | Description |
|-----------|------|-------------|
| `model` | `str` | Transcription model |
| `file` | `bytes` | Audio file bytes |
| `language` | `str \| None` | ISO-639-1 language code |
| `prompt` | `str \| None` | Optional context prompt |
| `response_format` | `str \| None` | Output format |
| `temperature` | `float \| None` | Sampling temperature |

### Content Safety

#### `moderate(**kwargs) -> ModerationResponse`

Classify content for policy violations.

```python
response = await client.moderate(
    input="Some text to check",
    model="text-moderation-latest",
)
flagged = response.results[0].flagged
```

| Parameter | Type | Description |
|-----------|------|-------------|
| `input` | `str` | Content to classify |
| `model` | `str \| None` | Moderation model |

### Search and Retrieval

#### `rerank(**kwargs) -> RerankResponse`

Rerank documents by relevance to a query.

```python
response = await client.rerank(
    model="cohere/rerank-v3.5",
    query="What is machine learning?",
    documents=["ML is a subset of AI...", "Cooking recipes..."],
    top_n=5,
)
```

| Parameter | Type | Description |
|-----------|------|-------------|
| `model` | `str` | Reranking model |
| `query` | `str` | Query to rank against |
| `documents` | `list[str]` | Documents to rerank |
| `top_n` | `int \| None` | Number of top results |

#### `search(**kwargs) -> SearchResponse`

Perform a web or document search across supported providers.

```python
response = await client.search(
    model="brave/search",
    query="latest AI news",
    max_results=10,
)
```

| Parameter | Type | Description |
|-----------|------|-------------|
| `model` | `str` | Search provider/model |
| `query` | `str` | Search query |
| `max_results` | `int \| None` | Maximum results |
| `search_type` | `str \| None` | Search type (provider-specific) |

### OCR

#### `ocr(**kwargs) -> OcrResponse`

Extract text from documents or images using OCR with Markdown output.

```python
response = await client.ocr(
    model="mistral/pixtral",
    file=pdf_bytes,
    mime_type="application/pdf",
)
print(response.text)
```

| Parameter | Type | Description |
|-----------|------|-------------|
| `model` | `str` | OCR model |
| `file` | `bytes` | File content |
| `mime_type` | `str \| None` | MIME type of the file |
| `pages` | `str \| None` | Page range (e.g. `"1-5"`) |

---

## File Operations

### `create_file(**kwargs) -> dict`

Upload a file.

```python
result = await client.create_file(
    file=open("data.jsonl", "rb").read(),
    purpose="batch",
    filename="data.jsonl",
)
file_id = result["id"]
```

| Parameter | Type | Description |
|-----------|------|-------------|
| `file` | `bytes` | File content |
| `purpose` | `str` | File purpose (e.g. `"batch"`, `"fine-tune"`) |
| `filename` | `str \| None` | Original filename |

#### `retrieve_file(file_id: str) -> dict`

Retrieve metadata about an uploaded file.

```python
meta = await client.retrieve_file("file-abc123")
```

#### `delete_file(file_id: str) -> dict`

Delete an uploaded file.

```python
result = await client.delete_file("file-abc123")
```

#### `list_files(**kwargs) -> dict`

List uploaded files.

```python
files = await client.list_files(purpose="batch", limit=10)
```

| Parameter | Type | Description |
|-----------|------|-------------|
| `purpose` | `str \| None` | Filter by purpose |
| `limit` | `int \| None` | Max results |
| `after` | `str \| None` | Cursor for pagination |

#### `file_content(file_id: str) -> bytes`

Download the content of an uploaded file.

```python
content = await client.file_content("file-abc123")
```

---

## Batch Operations

### `create_batch(**kwargs) -> dict`

Create a new batch.

```python
batch = await client.create_batch(
    input_file_id="file-abc123",
    endpoint="/v1/chat/completions",
    completion_window="24h",
)
```

| Parameter | Type | Description |
|-----------|------|-------------|
| `input_file_id` | `str` | ID of the uploaded JSONL file |
| `endpoint` | `str` | API endpoint for batch requests |
| `completion_window` | `str` | Time window (e.g. `"24h"`) |
| `metadata` | `dict \| None` | Optional metadata |

#### `retrieve_batch(batch_id: str) -> dict`

Retrieve a batch by ID.

```python
batch = await client.retrieve_batch("batch-abc123")
print(batch["status"])
```

#### `list_batches(**kwargs) -> dict`

List batches.

```python
batches = await client.list_batches(limit=10)
```

| Parameter | Type | Description |
|-----------|------|-------------|
| `limit` | `int \| None` | Max results |
| `after` | `str \| None` | Cursor for pagination |

#### `cancel_batch(batch_id: str) -> dict`

Cancel a batch.

```python
result = await client.cancel_batch("batch-abc123")
```

---

## Response Operations

### `create_response(**kwargs) -> dict`

Create a new response via the Responses API.

```python
resp = await client.create_response(
    model="gpt-4",
    input="Explain quantum computing",
    max_output_tokens=1024,
)
```

| Parameter | Type | Description |
|-----------|------|-------------|
| `model` | `str` | Model name |
| `input` | `str` | Input text |
| `instructions` | `str \| None` | System instructions |
| `max_output_tokens` | `int \| None` | Max output tokens |
| `temperature` | `float \| None` | Sampling temperature |
| `top_p` | `float \| None` | Nucleus sampling |

#### `retrieve_response(response_id: str) -> dict`

Retrieve a response by ID.

```python
resp = await client.retrieve_response("resp-abc123")
```

#### `cancel_response(response_id: str) -> dict`

Cancel a response.

```python
result = await client.cancel_response("resp-abc123")
```

---

## Provider Management

### `register_provider(config: dict)`

Register a custom provider for self-hosted or unsupported LLM endpoints.

```python
client.register_provider({
    "name": "my-provider",
    "base_url": "https://my-llm.example.com/v1",
    "auth_header": "Authorization",
    "model_prefixes": ["my-provider/"],
})
```

After registration, models prefixed with `"my-provider/"` route to the custom endpoint.

#### `unregister_provider(name: str) -> bool`

Remove a previously registered custom provider. Returns `True` if found and removed.

```python
removed = client.unregister_provider("my-provider")
```

---

## Hooks

### `add_hook(hook)`

Register a lifecycle hook for request/response/error events. All callbacks are optional and fire-and-forget.

```python
class LoggingHook:
    def on_request(self, request):
        print(f"Sending request to {request['model']}")

    def on_response(self, request, response):
        print(f"Got response: {response.usage.total_tokens} tokens")

    def on_error(self, request, error):
        print(f"Error: {error}")

client.add_hook(LoggingHook())
```

| Callback | Arguments | Description |
|----------|-----------|-------------|
| `on_request(request)` | `dict` | Called before each request |
| `on_response(request, response)` | `dict`, response object | Called after successful response |
| `on_error(request, error)` | `dict`, exception | Called on error |

---

## Budget Tracking

### `budget_used` (property)

Returns the total spend in USD so far (requires `cost_tracking=True` or `budget` config).

```python
print(f"Budget used: ${client.budget_used:.2f}")
```

---

## Types

### `ChatCompletionResponse`

| Field | Type | Description |
|-------|------|-------------|
| `id` | `str` | Response ID |
| `model` | `str` | Model used |
| `choices` | `list[Choice]` | Completion choices |
| `usage` | `Usage \| None` | Token usage |
| `created` | `int` | Unix timestamp |

### `Choice`

| Field | Type | Description |
|-------|------|-------------|
| `index` | `int` | Choice index |
| `message` | `AssistantMessage` | The assistant's message |
| `finish_reason` | `str \| None` | Why generation stopped (`stop`, `length`, `tool_calls`) |

### `AssistantMessage`

| Field | Type | Description |
|-------|------|-------------|
| `content` | `str \| None` | Text content |
| `tool_calls` | `list[ToolCall] \| None` | Tool calls made by the assistant |
| `refusal` | `str \| None` | Refusal message |

### `ToolCall`

| Field | Type | Description |
|-------|------|-------------|
| `id` | `str` | Tool call ID |
| `type` | `str` | Always `"function"` |
| `function` | `FunctionCall` | Function name and arguments |

### `FunctionCall`

| Field | Type | Description |
|-------|------|-------------|
| `name` | `str` | Function name |
| `arguments` | `str` | JSON-encoded arguments |

### `ChatCompletionChunk`

Yielded by `chat_stream()`.

| Field | Type | Description |
|-------|------|-------------|
| `id` | `str` | Response ID |
| `model` | `str` | Model used |
| `choices` | `list[StreamChoice]` | Stream choices with deltas |
| `usage` | `Usage \| None` | Token usage (final chunk only) |

### `StreamChoice`

| Field | Type | Description |
|-------|------|-------------|
| `index` | `int` | Choice index |
| `delta` | `Delta` | Incremental content |
| `finish_reason` | `str \| None` | Set on final chunk |

### `Delta`

| Field | Type | Description |
|-------|------|-------------|
| `content` | `str \| None` | Incremental text |
| `tool_calls` | `list[ToolCall] \| None` | Incremental tool calls |

### `Usage`

| Field | Type | Description |
|-------|------|-------------|
| `prompt_tokens` | `int` | Tokens consumed by the prompt |
| `completion_tokens` | `int` | Tokens consumed by the completion |
| `total_tokens` | `int` | Total tokens |

### `EmbeddingResponse`

| Field | Type | Description |
|-------|------|-------------|
| `data` | `list[EmbeddingObject]` | Embedding vectors |
| `model` | `str` | Model used |
| `usage` | `Usage` | Token usage |

### `EmbeddingObject`

| Field | Type | Description |
|-------|------|-------------|
| `index` | `int` | Index in the input list |
| `embedding` | `list[float]` | Embedding vector |

### `ModelsListResponse`

| Field | Type | Description |
|-------|------|-------------|
| `data` | `list[ModelObject]` | Available models |

### `ModelObject`

| Field | Type | Description |
|-------|------|-------------|
| `id` | `str` | Model identifier |
| `owned_by` | `str` | Model owner |

---

## Error Handling

All errors inherit from `liter_llm.LlmError` (which inherits from `Exception`). Invalid constructor arguments or malformed keyword arguments raise `ValueError`.

| Exception | Trigger |
|-----------|---------|
| `LlmError` | Base class for all liter-llm errors |
| `AuthenticationError` | API key rejected (HTTP 401/403) |
| `RateLimitedError` | Rate limit exceeded (HTTP 429) |
| `BadRequestError` | Malformed request (HTTP 400) |
| `ContextWindowExceededError` | Prompt exceeds context window (subclass of `BadRequestError`) |
| `ContentPolicyError` | Content policy violation (subclass of `BadRequestError`) |
| `NotFoundError` | Model/resource not found (HTTP 404) |
| `ServerError` | Provider 5xx error |
| `ServiceUnavailableError` | Provider temporarily unavailable (HTTP 502/503) |
| `LlmTimeoutError` | Request timed out |
| `NetworkError` | Network-level failure |
| `StreamingError` | Error reading streaming response |
| `EndpointNotSupportedError` | Provider does not support the endpoint |
| `InvalidHeaderError` | Custom header name or value is invalid |
| `SerializationError` | JSON serialization/deserialization failure |
| `HookRejectedError` | A hook rejected the request before it was sent |

### Error handling pattern

```python
from liter_llm import LlmError, RateLimitedError, AuthenticationError

try:
    response = await client.chat(model="gpt-4", messages=[...])
except ValueError as e:
    # Invalid arguments (malformed keyword args, missing fields)
    print(f"Bad request: {e}")
except RateLimitedError as e:
    print(f"Rate limited: {e}")
except AuthenticationError as e:
    print(f"Auth failed: {e}")
except LlmError as e:
    # Catch-all for other liter-llm errors
    print(f"Error: {e}")
```

### Exception hierarchy

```text
Exception
  ValueError          -- invalid constructor/method arguments
  LlmError            -- base for all API errors
    AuthenticationError
    RateLimitedError
    BadRequestError
      ContextWindowExceededError
      ContentPolicyError
    NotFoundError
    ServerError
    ServiceUnavailableError
    LlmTimeoutError
    NetworkError
    StreamingError
    EndpointNotSupportedError
    InvalidHeaderError
    SerializationError
    HookRejectedError
```

---

## Async Patterns and Best Practices

### Running the client

```python
import asyncio
from liter_llm import LlmClient

async def main():
    client = LlmClient(api_key="sk-...")
    response = await client.chat(
        model="gpt-4",
        messages=[{"role": "user", "content": "Hello!"}],
    )
    print(response.choices[0].message.content)

asyncio.run(main())
```

### Concurrent requests

```python
async def parallel_requests(client):
    tasks = [
        client.chat(model="gpt-4", messages=[{"role": "user", "content": q}])
        for q in ["Question 1", "Question 2", "Question 3"]
    ]
    responses = await asyncio.gather(*tasks)
    return responses
```

### Streaming with context manager

```python
async with await client.chat_stream(
    model="gpt-4",
    messages=[{"role": "user", "content": "Tell me a story"}],
) as stream:
    full_text = ""
    async for chunk in stream:
        delta = chunk.choices[0].delta
        if delta.content:
            full_text += delta.content
            print(delta.content, end="", flush=True)
```

### Client with middleware

```python
client = LlmClient(
    api_key="sk-...",
    cache={"max_entries": 256, "ttl_seconds": 300},
    budget={"global_limit": 10.0, "enforcement": "hard"},
    rate_limit={"rpm": 60, "tpm": 100000},
    cost_tracking=True,
    cooldown=5,
    health_check=30,
)

# Check spending
print(f"Spent: ${client.budget_used:.2f}")
```

### Provider routing

Models are routed by name prefix. No per-request overhead.

```python
# OpenAI (default, no prefix needed)
await client.chat(model="gpt-4", messages=[...])

# Groq
await client.chat(model="groq/llama3-70b", messages=[...])

# Anthropic
await client.chat(model="anthropic/claude-3-opus", messages=[...])

# Custom provider
client.register_provider({
    "name": "local",
    "base_url": "http://localhost:8080/v1",
    "auth_header": "Authorization",
    "model_prefixes": ["local/"],
})
await client.chat(model="local/my-model", messages=[...])
```
