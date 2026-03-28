---
description: "liter-llm Python API reference"
---

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
)
```

All parameters are keyword-only.

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `api_key` | `str` | *required* | API key for authentication |
| `base_url` | `str \| None` | `None` | Override provider base URL |
| `model_hint` | `str \| None` | `None` | Hint for provider auto-detection (e.g. `"groq/llama3-70b"`) |
| `max_retries` | `int` | `3` | Retries on 429 / 5xx responses |
| `timeout` | `int` | `60` | Request timeout in seconds |
| `cache` | `dict \| None` | `None` | Cache config: `{"max_entries": 256, "ttl_seconds": 300}` |
| `budget` | `dict \| None` | `None` | Budget config: `{"global_limit": 10.0, "model_limits": {}, "enforcement": "hard"}` |
| `extra_headers` | `dict \| None` | `None` | Additional HTTP headers |

The client is immutable after construction and safe to share across tasks.

### Methods

All methods are async and must be awaited.

#### `chat(**kwargs)`

Send a chat completion request.

```python
async def chat(**kwargs) -> ChatCompletionResponse
```

Accepts the same keyword arguments as the OpenAI Chat Completions API (`model`, `messages`, `temperature`, `max_tokens`, etc.).

#### `chat_stream(**kwargs)`

Start a streaming chat completion. Returns an async iterator that yields `ChatCompletionChunk` objects. The HTTP request is issued immediately when the method is called.

```python
async def chat_stream(**kwargs) -> ChatStreamIterator
```

Use with `async for`:

```python
iterator = await client.chat_stream(model="gpt-4", messages=[...])
async for chunk in iterator:
    print(chunk.choices[0].delta.content, end="")
```

The iterator supports `async with` for deterministic resource cleanup:

```python
async with await client.chat_stream(model="gpt-4", messages=[...]) as stream:
    async for chunk in stream:
        print(chunk.choices[0].delta.content, end="")
```

Call `iterator.cancel()` to signal the background task to stop early.

#### `embed(**kwargs)`

Send an embedding request.

```python
async def embed(**kwargs) -> EmbeddingResponse
```

Accepts `model`, `input`, `encoding_format`, `dimensions`, `user`.

#### `list_models()`

List available models from the provider.

```python
async def list_models() -> ModelsListResponse
```

#### `image_generate(**kwargs)`

Generate images from a text prompt.

```python
async def image_generate(**kwargs) -> ImagesResponse
```

Accepts `prompt`, `model`, `n`, `size`, `quality`, `response_format`, `style`, `user`.

#### `speech(**kwargs)`

Generate speech audio from text.

```python
async def speech(**kwargs) -> bytes
```

Accepts `model`, `input`, `voice`, `response_format`, `speed`. Returns raw audio bytes.

#### `transcribe(**kwargs)`

Transcribe audio into text.

```python
async def transcribe(**kwargs) -> TranscriptionResponse
```

Accepts `model`, `file`, `language`, `prompt`, `response_format`, `temperature`.

#### `moderate(**kwargs)`

Classify content for policy violations.

```python
async def moderate(**kwargs) -> ModerationResponse
```

Accepts `input`, `model`.

#### `rerank(**kwargs)`

Rerank documents by relevance to a query.

```python
async def rerank(**kwargs) -> RerankResponse
```

Accepts `model`, `query`, `documents`, `top_n`.

#### `create_file(**kwargs)`

Upload a file.

```python
async def create_file(**kwargs) -> dict
```

Accepts `file`, `purpose`, `filename`. Returns a dict with file object fields.

#### `retrieve_file(file_id)`

Retrieve metadata about an uploaded file.

```python
async def retrieve_file(file_id: str) -> dict
```

#### `delete_file(file_id)`

Delete an uploaded file.

```python
async def delete_file(file_id: str) -> dict
```

#### `list_files(**kwargs)`

List uploaded files. Optional keyword arguments: `purpose`, `limit`, `after`.

```python
async def list_files(**kwargs) -> dict
```

#### `file_content(file_id)`

Download the content of an uploaded file.

```python
async def file_content(file_id: str) -> bytes
```

#### `create_batch(**kwargs)`

Create a new batch.

```python
async def create_batch(**kwargs) -> dict
```

Accepts `input_file_id`, `endpoint`, `completion_window`, `metadata`.

#### `retrieve_batch(batch_id)`

Retrieve a batch by ID.

```python
async def retrieve_batch(batch_id: str) -> dict
```

#### `list_batches(**kwargs)`

List batches. Optional keyword arguments: `limit`, `after`.

```python
async def list_batches(**kwargs) -> dict
```

#### `cancel_batch(batch_id)`

Cancel a batch.

```python
async def cancel_batch(batch_id: str) -> dict
```

#### `create_response(**kwargs)`

Create a new response via the Responses API.

```python
async def create_response(**kwargs) -> dict
```

Accepts `model`, `input`, `instructions`, `max_output_tokens`, `temperature`, `top_p`.

#### `retrieve_response(response_id)`

Retrieve a response by ID.

```python
async def retrieve_response(response_id: str) -> dict
```

#### `cancel_response(response_id)`

Cancel a response.

```python
async def cancel_response(response_id: str) -> dict
```

#### `register_provider(config)`

Register a custom provider for self-hosted or unsupported LLM endpoints.

```python
client.register_provider({
    "name": "my-provider",
    "base_url": "https://my-llm.example.com/v1",
    "auth_header": "Authorization",
    "model_prefixes": ["my-provider/"],
})
```

#### `unregister_provider(name)`

Remove a previously registered custom provider. Returns `True` if found and removed.

```python
removed = client.unregister_provider("my-provider")
```

#### `add_hook(hook)`

Register a lifecycle hook for request/response/error events.

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

#### `budget_used`

Property returning the total spend in USD so far.

```python
print(f"Budget used: ${client.budget_used:.2f}")
```

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

### `ChatCompletionChunk`

Yielded by `chat_stream()`.

| Field | Type | Description |
|-------|------|-------------|
| `id` | `str` | Response ID |
| `model` | `str` | Model used |
| `choices` | `list[StreamChoice]` | Stream choices with deltas |
| `usage` | `Usage \| None` | Token usage (final chunk only) |

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

### `ModelsListResponse`

| Field | Type | Description |
|-------|------|-------------|
| `data` | `list[ModelObject]` | Available models |

## Error Handling

All errors are raised as Python exceptions inheriting from `liter_llm.LlmError` (which itself inherits from `Exception`). Invalid arguments to the constructor or malformed keyword arguments raise `ValueError`.

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

## Example

```python
import asyncio
from liter_llm import LlmClient

async def main():
    client = LlmClient(api_key="sk-...")

    # Non-streaming
    response = await client.chat(
        model="gpt-4",
        messages=[{"role": "user", "content": "Hello!"}],
        max_tokens=256,
    )
    print(response.choices[0].message.content)

    # Streaming
    async with await client.chat_stream(
        model="gpt-4",
        messages=[{"role": "user", "content": "Tell me a joke"}],
    ) as stream:
        async for chunk in stream:
            delta = chunk.choices[0].delta
            if delta.content:
                print(delta.content, end="", flush=True)

asyncio.run(main())
```
