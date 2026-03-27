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
  receive_timeout: 60_000                   # default, in milliseconds
)
```

| Option | Type | Default | Description |
|--------|------|---------|-------------|
| `:api_key` | `String.t()` | `""` | API key for `Authorization: Bearer` |
| `:base_url` | `String.t()` | `"https://api.openai.com/v1"` | Provider base URL |
| `:max_retries` | `non_neg_integer()` | `2` | Retry count for 429/5xx |
| `:receive_timeout` | `pos_integer()` | `60_000` | Timeout in milliseconds |

### Methods

All methods return `{:ok, result}` or `{:error, %LiterLlm.Error{}}` tuples.

#### `chat(client, request, opts \\ [])`

Send a chat completion request.

```elixir
{:ok, response} = LiterLlm.Client.chat(client, %{
  model: "gpt-4o-mini",
  messages: [%{role: "user", content: "Hello!"}],
  max_tokens: 256
})
```

Returns `{:ok, map()}` where the map matches the OpenAI chat completion response format.

#### `embed(client, request, opts \\ [])`

Send an embedding request.

```elixir
{:ok, response} = LiterLlm.Client.embed(client, %{
  model: "text-embedding-3-small",
  input: "Hello"
})
```

#### `list_models(client, opts \\ [])`

List available models.

```elixir
{:ok, response} = LiterLlm.Client.list_models(client)
```

#### `image_generate(client, request, opts \\ [])`

Generate an image from a text prompt.

```elixir
{:ok, response} = LiterLlm.Client.image_generate(client, %{
  prompt: "A sunset over mountains",
  model: "dall-e-3"
})
```

#### `speech(client, request, opts \\ [])`

Generate speech audio from text. Returns `{:ok, binary()}` with raw audio bytes.

```elixir
{:ok, audio_bytes} = LiterLlm.Client.speech(client, %{
  model: "tts-1", input: "Hello", voice: "alloy"
})
```

#### `transcribe(client, request, opts \\ [])`

Transcribe audio to text.

#### `moderate(client, request, opts \\ [])`

Check content against moderation policies.

#### `rerank(client, request, opts \\ [])`

Rerank documents by relevance to a query.

#### `create_file(client, request, opts \\ [])`

Upload a file.

#### `retrieve_file(client, file_id, opts \\ [])`

Retrieve metadata for a file by ID.

#### `delete_file(client, file_id, opts \\ [])`

Delete a file by ID.

#### `list_files(client, query \\ nil, opts \\ [])`

List files, optionally filtered by query parameters.

#### `file_content(client, file_id, opts \\ [])`

Retrieve raw file content as `{:ok, binary()}`.

#### `create_batch(client, request, opts \\ [])` / `retrieve_batch(client, batch_id, opts \\ [])` / `list_batches(client, query \\ nil, opts \\ [])` / `cancel_batch(client, batch_id, opts \\ [])`

Batch management operations.

#### `create_response(client, request, opts \\ [])` / `retrieve_response(client, response_id, opts \\ [])` / `cancel_response(client, response_id, opts \\ [])`

Responses API operations.

## Error Handling

Errors are returned as `{:error, %LiterLlm.Error{}}` structs. Pattern match on `:kind` for programmatic handling:

```elixir
case LiterLlm.Client.chat(client, request) do
  {:ok, response} ->
    process(response)

  {:error, %LiterLlm.Error{kind: :rate_limit}} ->
    retry_after_backoff()

  {:error, %LiterLlm.Error{kind: :authentication, message: msg}} ->
    raise "Auth failed: #{msg}"

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
| `:stream_error` | 1600 | Stream parse failure |
| `:serialization` | 1700 | JSON encode/decode failure |

## Example

```elixir
client = LiterLlm.Client.new(api_key: System.fetch_env!("OPENAI_API_KEY"))

{:ok, response} = LiterLlm.Client.chat(client, %{
  model: "gpt-4o-mini",
  messages: [%{role: "user", content: "Hello!"}],
  max_tokens: 256
})

response
|> Map.get("choices")
|> List.first()
|> get_in(["message", "content"])
|> IO.puts()
```
