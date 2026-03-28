---
description: "liter-llm Elixir API reference"
---

# Elixir API Reference

The Elixir package is a pure-Elixir HTTP client using `Req`. No NIFs or native libraries required.

## Installation

```elixir
# mix.exs
defp deps do
  [{:liter_llm, "~> 1.0"}]
end
```

## Client

### Constructor

```elixir
client = LiterLlm.Client.new(
  api_key: System.fetch_env!("OPENAI_API_KEY"),
  base_url: "https://api.openai.com/v1",  # default
  max_retries: 2,                           # default
  receive_timeout: 60_000,                  # default, in milliseconds
  cache: [max_entries: 256, ttl_seconds: 300],
  budget: [global_limit: 10.0, model_limits: %{}, enforcement: "hard"]
)
```

| Option | Type | Default | Description |
|--------|------|---------|-------------|
| `:api_key` | `String.t()` | `""` | API key for `Authorization: Bearer` |
| `:base_url` | `String.t()` | `"https://api.openai.com/v1"` | Provider base URL |
| `:max_retries` | `non_neg_integer()` | `2` | Retry count for 429/5xx |
| `:receive_timeout` | `pos_integer()` | `60_000` | Timeout in milliseconds |
| `:cache` | `keyword()` | `nil` | Cache config (`max_entries`, `ttl_seconds`) |
| `:budget` | `keyword()` | `nil` | Budget config (`global_limit`, `model_limits`, `enforcement`) |
| `:cooldown_secs` | `non_neg_integer()` | `nil` | Cooldown period in seconds after transient errors |
| `:rate_limit` | `keyword()` | `nil` | Rate limit config (`rpm`, `tpm`) |
| `:health_check_secs` | `non_neg_integer()` | `nil` | Health check interval in seconds |
| `:cost_tracking` | `boolean()` | `false` | Enable per-request cost tracking |
| `:tracing` | `boolean()` | `false` | Enable OpenTelemetry tracing spans |

The client struct is immutable and safe to share across processes.

### Methods

All methods return `{:ok, result}` or `{:error, %LiterLlm.Error{}}` tuples. Convenience functions are also available on the `LiterLlm` module directly.

#### `chat(client, request, opts \\ [])`

Send a chat completion request.

```elixir
{:ok, response} = LiterLlm.Client.chat(client, %{
  model: "gpt-4o-mini",
  messages: [%{role: "user", content: "Hello!"}],
  max_tokens: 256
})

content = get_in(response, ["choices", Access.at(0), "message", "content"])
IO.puts(content)
```

Returns `{:ok, map()}` where the map matches the OpenAI chat completion response format.

#### `chat_stream(client, request, opts \\ [])`

Start a streaming chat completion. Returns `{:ok, chunks}` where `chunks` is a list of chunk maps.

```elixir
{:ok, chunks} = LiterLlm.Client.chat_stream(client, %{
  model: "gpt-4",
  messages: [%{role: "user", content: "Tell me a joke"}]
})

for chunk <- chunks do
  case get_in(chunk, ["choices", Access.at(0), "delta", "content"]) do
    nil -> :skip
    content -> IO.write(content)
  end
end

IO.puts("")
```

#### `embed(client, request, opts \\ [])`

Send an embedding request.

```elixir
{:ok, response} = LiterLlm.Client.embed(client, %{
  model: "text-embedding-3-small",
  input: "Hello"
})

vector = get_in(response, ["data", Access.at(0), "embedding"])
```

Accepts `model`, `input`, `encoding_format`, `dimensions`, `user`.

#### `list_models(client, opts \\ [])`

List available models.

```elixir
{:ok, response} = LiterLlm.Client.list_models(client)

for model <- response["data"] do
  IO.puts(model["id"])
end
```

#### `image_generate(client, request, opts \\ [])`

Generate images from a text prompt.

```elixir
{:ok, response} = LiterLlm.Client.image_generate(client, %{
  prompt: "A sunset over mountains",
  model: "dall-e-3",
  size: "1024x1024"
})

url = get_in(response, ["data", Access.at(0), "url"])
```

Accepts `prompt`, `model`, `n`, `size`, `quality`, `response_format`, `style`, `user`.

#### `speech(client, request, opts \\ [])`

Generate speech audio from text. Returns `{:ok, binary()}` with raw audio bytes.

```elixir
{:ok, audio_bytes} = LiterLlm.Client.speech(client, %{
  model: "tts-1",
  input: "Hello, world!",
  voice: "alloy"
})

File.write!("output.mp3", audio_bytes)
```

Accepts `model`, `input`, `voice`, `response_format`, `speed`.

#### `transcribe(client, request, opts \\ [])`

Transcribe audio into text.

```elixir
{:ok, response} = LiterLlm.Client.transcribe(client, %{
  model: "whisper-1",
  file: audio_bytes
})

IO.puts(response["text"])
```

Accepts `model`, `file`, `language`, `prompt`, `response_format`, `temperature`.

#### `moderate(client, request, opts \\ [])`

Classify content for policy violations.

```elixir
{:ok, response} = LiterLlm.Client.moderate(client, %{input: "some text"})

flagged = get_in(response, ["results", Access.at(0), "flagged"])
IO.puts("Flagged: #{flagged}")
```

Accepts `input`, `model`.

#### `rerank(client, request, opts \\ [])`

Rerank documents by relevance to a query.

```elixir
{:ok, response} = LiterLlm.Client.rerank(client, %{
  model: "rerank-v1",
  query: "What is Elixir?",
  documents: ["Elixir is a language", "Python is a language"],
  top_n: 1
})

top_doc = get_in(response, ["results", Access.at(0), "document", "text"])
```

Accepts `model`, `query`, `documents`, `top_n`.

#### `search(client, request, opts \\ [])`

Perform a web or document search across supported providers.

```elixir
{:ok, response} = LiterLlm.Client.search(client, %{
  model: "brave/search",
  query: "latest AI news",
  max_results: 10
})

for result <- response["results"] do
  IO.puts(result["title"])
end
```

Accepts `model`, `query`, `max_results`, `search_type`.

#### `ocr(client, request, opts \\ [])`

Extract text from documents or images using OCR with Markdown output.

```elixir
{:ok, response} = LiterLlm.Client.ocr(client, %{
  model: "mistral/pixtral",
  file: File.read!("document.pdf"),
  mime_type: "application/pdf"
})

IO.puts(response["content"])
```

Accepts `model`, `file`, `mime_type`, `pages`.

#### `create_file(client, request, opts \\ [])`

Upload a file.

```elixir
{:ok, file_obj} = LiterLlm.Client.create_file(client, %{
  file: File.read!("data.jsonl"),
  purpose: "batch",
  filename: "data.jsonl"
})

IO.puts(file_obj["id"])
```

Accepts `file`, `purpose`, `filename`.

#### `retrieve_file(client, file_id, opts \\ [])`

Retrieve metadata about an uploaded file.

```elixir
{:ok, file_obj} = LiterLlm.Client.retrieve_file(client, "file-abc123")
IO.puts(file_obj["filename"])
```

#### `delete_file(client, file_id, opts \\ [])`

Delete an uploaded file.

```elixir
{:ok, result} = LiterLlm.Client.delete_file(client, "file-abc123")
IO.puts(result["deleted"])
```

#### `list_files(client, query \\ nil, opts \\ [])`

List files, optionally filtered by query parameters.

```elixir
{:ok, response} = LiterLlm.Client.list_files(client, %{purpose: "batch"})

for file <- response["data"] do
  IO.puts(file["id"])
end
```

#### `file_content(client, file_id, opts \\ [])`

Download the content of an uploaded file. Returns `{:ok, binary()}`.

```elixir
{:ok, content} = LiterLlm.Client.file_content(client, "file-abc123")
File.write!("downloaded.jsonl", content)
```

#### `create_batch(client, request, opts \\ [])`

Create a new batch.

```elixir
{:ok, batch} = LiterLlm.Client.create_batch(client, %{
  input_file_id: "file-abc123",
  endpoint: "/v1/chat/completions",
  completion_window: "24h"
})

IO.puts(batch["id"])
```

Accepts `input_file_id`, `endpoint`, `completion_window`, `metadata`.

#### `retrieve_batch(client, batch_id, opts \\ [])`

Retrieve a batch by ID.

```elixir
{:ok, batch} = LiterLlm.Client.retrieve_batch(client, "batch-abc123")
IO.puts(batch["status"])
```

#### `list_batches(client, query \\ nil, opts \\ [])`

List batches, optionally filtered by query parameters.

```elixir
{:ok, response} = LiterLlm.Client.list_batches(client)

for batch <- response["data"] do
  IO.puts("#{batch["id"]}: #{batch["status"]}")
end
```

#### `cancel_batch(client, batch_id, opts \\ [])`

Cancel a batch.

```elixir
{:ok, batch} = LiterLlm.Client.cancel_batch(client, "batch-abc123")
IO.puts(batch["status"])
```

#### `create_response(client, request, opts \\ [])`

Create a new response via the Responses API.

```elixir
{:ok, response} = LiterLlm.Client.create_response(client, %{
  model: "gpt-4",
  input: "Summarize this document"
})

IO.puts(response["output"])
```

Accepts `model`, `input`, `instructions`, `max_output_tokens`, `temperature`, `top_p`.

#### `retrieve_response(client, response_id, opts \\ [])`

Retrieve a response by ID.

```elixir
{:ok, response} = LiterLlm.Client.retrieve_response(client, "resp-abc123")
IO.puts(response["status"])
```

#### `cancel_response(client, response_id, opts \\ [])`

Cancel a response.

```elixir
{:ok, response} = LiterLlm.Client.cancel_response(client, "resp-abc123")
IO.puts(response["status"])
```

#### `register_provider(client, name, config)`

Register a custom provider at runtime.

```elixir
:ok = LiterLlm.Client.register_provider(client, "my-provider", %{
  base_url: "https://my-llm.example.com/v1",
  auth_header: "Authorization",
  auth_prefix: "Bearer "
})
```

#### `unregister_provider(client, name)`

Remove a previously registered custom provider.

```elixir
:ok = LiterLlm.Client.unregister_provider(client, "my-provider")
```

#### `add_hook(client, hook_module)`

Register a lifecycle hook. The hook module must implement the `LiterLlm.Hook` behaviour.

```elixir
defmodule MyHook do
  @behaviour LiterLlm.Hook

  @impl true
  def on_request(request) do
    IO.puts("Sending request to #{request["model"]}")
    :ok
  end

  @impl true
  def on_response(request, response) do
    IO.puts("Got response: #{response["id"]}")
    :ok
  end

  @impl true
  def on_error(request, error) do
    IO.warn("Request failed: #{error.message}")
    :ok
  end
end

LiterLlm.Client.add_hook(client, MyHook)
```

All three callbacks are optional -- implement only the ones you need.

#### `budget_used(client)`

Returns the total budget consumed so far as a float. Only meaningful when `:budget` was configured in the constructor.

```elixir
used = LiterLlm.Client.budget_used(client)
IO.puts("Budget used: $#{used}")
```

## Types

All responses are returned as Elixir maps with string keys. The tables below document the structure.

### `ChatCompletionResponse`

| Key | Type | Description |
|-----|------|-------------|
| `"id"` | `String.t()` | Response ID |
| `"model"` | `String.t()` | Model used |
| `"choices"` | `[map()]` | Completion choices |
| `"usage"` | `map() \| nil` | Token usage |
| `"created"` | `integer()` | Unix timestamp |

### `Choice`

| Key | Type | Description |
|-----|------|-------------|
| `"index"` | `integer()` | Choice index |
| `"message"` | `map()` | The assistant's message |
| `"finish_reason"` | `String.t() \| nil` | Why generation stopped (`stop`, `length`, `tool_calls`) |

### `AssistantMessage`

| Key | Type | Description |
|-----|------|-------------|
| `"content"` | `String.t() \| nil` | Text content |
| `"tool_calls"` | `[map()] \| nil` | Tool calls made by the assistant |
| `"refusal"` | `String.t() \| nil` | Refusal message |

### `ChatCompletionChunk`

Returned inside the list from `chat_stream/3`.

| Key | Type | Description |
|-----|------|-------------|
| `"id"` | `String.t()` | Response ID |
| `"model"` | `String.t()` | Model used |
| `"choices"` | `[map()]` | Stream choices with deltas |
| `"usage"` | `map() \| nil` | Token usage (final chunk only) |

### `Usage`

| Key | Type | Description |
|-----|------|-------------|
| `"prompt_tokens"` | `integer()` | Tokens consumed by the prompt |
| `"completion_tokens"` | `integer()` | Tokens consumed by the completion |
| `"total_tokens"` | `integer()` | Total tokens |

### `EmbeddingResponse`

| Key | Type | Description |
|-----|------|-------------|
| `"data"` | `[map()]` | Embedding objects, each with `"embedding"` (list of floats) and `"index"` |
| `"model"` | `String.t()` | Model used |
| `"usage"` | `map()` | Token usage |

### `ModelsListResponse`

| Key | Type | Description |
|-----|------|-------------|
| `"data"` | `[map()]` | Available models, each with `"id"`, `"created"`, `"owned_by"` |

## Error Handling

Errors are returned as `{:error, %LiterLlm.Error{}}` structs. Pattern match on `:kind` for programmatic handling.

```elixir
case LiterLlm.Client.chat(client, request) do
  {:ok, response} ->
    process(response)

  {:error, %LiterLlm.Error{kind: :rate_limit}} ->
    retry_after_backoff()

  {:error, %LiterLlm.Error{kind: :authentication, message: msg}} ->
    raise "Auth failed: #{msg}"

  {:error, %LiterLlm.Error{kind: :budget_exceeded}} ->
    IO.puts("Budget limit reached")

  {:error, %LiterLlm.Error{} = err} ->
    Logger.error("LLM error: #{err}")
end
```

| Kind | Code | Description |
|------|------|-------------|
| `:unknown` | 1000 | Unknown error |
| `:invalid_request` | 1400 | Malformed request (400/422) |
| `:authentication` | 1401 | API key rejected (401/403) |
| `:not_found` | 1404 | Model/resource not found (404) |
| `:rate_limit` | 1429 | Rate limit exceeded (429) |
| `:provider_error` | 1500 | Provider 5xx error |
| `:service_unavailable` | 1503 | Provider temporarily unavailable (502/503) |
| `:timeout` | 1504 | Request timed out |
| `:network_error` | 1510 | Network-level failure |
| `:stream_error` | 1600 | Stream parse failure |
| `:serialization` | 1700 | JSON encode/decode failure |
| `:endpoint_not_supported` | 1800 | Provider does not support the endpoint |
| `:budget_exceeded` | 1900 | Budget limit exceeded |
| `:context_window_exceeded` | 1401 | Prompt exceeds context window |
| `:content_policy` | 1402 | Content policy violation |

## Example

```elixir
client = LiterLlm.Client.new(api_key: System.fetch_env!("OPENAI_API_KEY"))

# Non-streaming
{:ok, response} = LiterLlm.Client.chat(client, %{
  model: "gpt-4o-mini",
  messages: [%{role: "user", content: "Hello!"}],
  max_tokens: 256
})

response
|> get_in(["choices", Access.at(0), "message", "content"])
|> IO.puts()

# Streaming
{:ok, chunks} = LiterLlm.Client.chat_stream(client, %{
  model: "gpt-4",
  messages: [%{role: "user", content: "Tell me a joke"}]
})

for chunk <- chunks do
  case get_in(chunk, ["choices", Access.at(0), "delta", "content"]) do
    nil -> :skip
    content -> IO.write(content)
  end
end

IO.puts("")
```
