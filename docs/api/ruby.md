---
description: "liter-llm Ruby API reference"
---

# Ruby API Reference

The Ruby gem wraps the Rust core via Magnus. All request/response data is passed as JSON strings -- use `JSON.parse` and `JSON.generate` for conversion.

## Installation

```bash
gem install liter_llm
```

Or in your Gemfile:

```ruby
gem 'liter_llm'
```

## Client

### Constructor

```ruby
require 'liter_llm'

client = LiterLlm::LlmClient.new('sk-...',
  base_url: 'https://api.openai.com/v1',  # optional
  model_hint: 'groq/llama3-70b',           # optional
  max_retries: 3,                           # default: 3
  timeout_secs: 60                          # default: 60
)
```

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `api_key` | `String` | *required* | API key (positional) |
| `base_url:` | `String?` | `nil` | Override provider base URL |
| `model_hint:` | `String?` | `nil` | Provider auto-detection hint |
| `max_retries:` | `Integer` | `3` | Retries on 429/5xx |
| `timeout_secs:` | `Integer` | `60` | Request timeout in seconds |

### Methods

All methods are synchronous (they block on the Tokio runtime internally). Methods that accept requests take a JSON string and return a JSON string.

#### `chat(request_json)`

Send a chat completion request.

```ruby
response_json = client.chat(JSON.generate(
  model: 'gpt-4',
  messages: [{ role: 'user', content: 'Hello' }]
))
response = JSON.parse(response_json)
```

#### `embed(request_json)`

Send an embedding request.

```ruby
response_json = client.embed(JSON.generate(
  model: 'text-embedding-3-small',
  input: 'Hello'
))
```

#### `list_models`

List available models. Takes no arguments.

```ruby
response_json = client.list_models
```

#### `image_generate(request_json)`

Generate an image from a text prompt.

```ruby
response_json = client.image_generate(JSON.generate(prompt: 'A sunset'))
```

#### `speech(request_json)`

Generate speech audio from text. Returns a base64-encoded string of the audio bytes.

```ruby
base64_audio = client.speech(JSON.generate(
  model: 'tts-1', input: 'Hello', voice: 'alloy'
))
```

#### `transcribe(request_json)`

Transcribe audio to text.

```ruby
response_json = client.transcribe(JSON.generate(model: 'whisper-1', file: base64_audio))
```

#### `moderate(request_json)`

Check content against moderation policies.

```ruby
response_json = client.moderate(JSON.generate(input: 'some text'))
```

#### `rerank(request_json)`

Rerank documents by relevance to a query.

```ruby
response_json = client.rerank(JSON.generate(
  model: 'rerank-v1', query: 'q', documents: ['a', 'b']
))
```

#### `create_file(request_json)`

Upload a file.

#### `retrieve_file(file_id)`

Retrieve metadata for a file by ID.

#### `delete_file(file_id)`

Delete a file by ID.

#### `list_files(query_json)`

List files. Pass `nil` or a JSON string with query parameters.

#### `file_content(file_id)`

Retrieve raw file content as a base64-encoded string.

#### `create_batch(request_json)` / `retrieve_batch(batch_id)` / `list_batches(query_json)` / `cancel_batch(batch_id)`

Batch management operations.

#### `create_response(request_json)` / `retrieve_response(response_id)` / `cancel_response(response_id)`

Responses API operations.

## Error Handling

Errors are raised as Ruby exceptions:

- `ArgumentError` -- invalid request JSON
- `RuntimeError` -- network, auth, provider, or serialization errors

```ruby
begin
  response = JSON.parse(client.chat(JSON.generate(model: 'gpt-4', messages: [])))
rescue ArgumentError => e
  puts "Bad request: #{e.message}"
rescue RuntimeError => e
  puts "Error: #{e.message}"
end
```

## Example

```ruby
require 'liter_llm'
require 'json'

client = LiterLlm::LlmClient.new(ENV.fetch('OPENAI_API_KEY'))

response = JSON.parse(client.chat(JSON.generate(
  model: 'gpt-4',
  messages: [{ role: 'user', content: 'Hello!' }]
)))

puts response.dig('choices', 0, 'message', 'content')
```
