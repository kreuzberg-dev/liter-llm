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
  timeout_secs: 60,                         # default: 60
  cache: { max_entries: 256, ttl_seconds: 300 },
  budget: { global_limit: 10.0, model_limits: {}, enforcement: 'hard' }
)
```

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `api_key` | `String` | *required* | API key (positional) |
| `base_url:` | `String?` | `nil` | Override provider base URL |
| `model_hint:` | `String?` | `nil` | Provider auto-detection hint |
| `max_retries:` | `Integer` | `3` | Retries on 429/5xx |
| `timeout_secs:` | `Integer` | `60` | Request timeout in seconds |
| `cache:` | `Hash?` | `nil` | Cache config (`max_entries`, `ttl_seconds`) |
| `budget:` | `Hash?` | `nil` | Budget config (`global_limit`, `model_limits`, `enforcement`) |
| `cooldown_secs:` | `Integer?` | `nil` | Cooldown period in seconds after transient errors |
| `rate_limit:` | `Hash?` | `nil` | Rate limit config (`rpm`, `tpm`) |
| `health_check_secs:` | `Integer?` | `nil` | Health check interval in seconds |
| `cost_tracking:` | `Boolean` | `false` | Enable per-request cost tracking |
| `tracing:` | `Boolean` | `false` | Enable OpenTelemetry tracing spans |

The client is immutable after construction and safe to share across threads.

### Methods

All methods are synchronous (they block on the Tokio runtime internally). Methods that accept requests take a JSON string and return a JSON string.

#### `chat(request_json)`

Send a chat completion request.

```ruby
response_json = client.chat(JSON.generate(
  model: 'gpt-4',
  messages: [{ role: 'user', content: 'Hello' }],
  max_tokens: 256
))
response = JSON.parse(response_json)
puts response.dig('choices', 0, 'message', 'content')
```

#### `chat_stream(request_json)`

Start a streaming chat completion. Returns a JSON string encoding an array of chunk objects. Each chunk follows the same structure as `ChatCompletionChunk`.

```ruby
chunks_json = client.chat_stream(JSON.generate(
  model: 'gpt-4',
  messages: [{ role: 'user', content: 'Tell me a joke' }]
))
chunks = JSON.parse(chunks_json)
chunks.each do |chunk|
  content = chunk.dig('choices', 0, 'delta', 'content')
  print content if content
end
puts
```

#### `embed(request_json)`

Send an embedding request.

```ruby
response_json = client.embed(JSON.generate(
  model: 'text-embedding-3-small',
  input: 'Hello'
))
response = JSON.parse(response_json)
vector = response.dig('data', 0, 'embedding')
```

Accepts `model`, `input`, `encoding_format`, `dimensions`, `user`.

#### `list_models`

List available models. Takes no arguments.

```ruby
response_json = client.list_models
models = JSON.parse(response_json)
models['data'].each { |m| puts m['id'] }
```

#### `image_generate(request_json)`

Generate images from a text prompt.

```ruby
response_json = client.image_generate(JSON.generate(
  prompt: 'A sunset over mountains',
  model: 'dall-e-3',
  size: '1024x1024'
))
response = JSON.parse(response_json)
url = response.dig('data', 0, 'url')
```

Accepts `prompt`, `model`, `n`, `size`, `quality`, `response_format`, `style`, `user`.

#### `speech(request_json)`

Generate speech audio from text. Returns a base64-encoded string of the audio bytes.

```ruby
base64_audio = client.speech(JSON.generate(
  model: 'tts-1', input: 'Hello, world!', voice: 'alloy'
))
File.binwrite('output.mp3', Base64.decode64(base64_audio))
```

Accepts `model`, `input`, `voice`, `response_format`, `speed`.

#### `transcribe(request_json)`

Transcribe audio into text.

```ruby
response_json = client.transcribe(JSON.generate(
  model: 'whisper-1',
  file: base64_audio
))
response = JSON.parse(response_json)
puts response['text']
```

Accepts `model`, `file`, `language`, `prompt`, `response_format`, `temperature`.

#### `moderate(request_json)`

Classify content for policy violations.

```ruby
response_json = client.moderate(JSON.generate(input: 'some text'))
response = JSON.parse(response_json)
puts response.dig('results', 0, 'flagged')
```

Accepts `input`, `model`.

#### `rerank(request_json)`

Rerank documents by relevance to a query.

```ruby
response_json = client.rerank(JSON.generate(
  model: 'rerank-v1',
  query: 'What is Ruby?',
  documents: ['Ruby is a language', 'Python is a language'],
  top_n: 1
))
response = JSON.parse(response_json)
puts response.dig('results', 0, 'document', 'text')
```

Accepts `model`, `query`, `documents`, `top_n`.

#### `search(request_json)`

Perform a web or document search across supported providers.

```ruby
response_json = client.search(JSON.generate(
  model: 'brave/search',
  query: 'latest AI news',
  max_results: 10
))
response = JSON.parse(response_json)
response['results'].each { |r| puts r['title'] }
```

Accepts `model`, `query`, `max_results`, `search_type`.

#### `ocr(request_json)`

Extract text from documents or images using OCR with Markdown output.

```ruby
response_json = client.ocr(JSON.generate(
  model: 'mistral/pixtral',
  file: Base64.encode64(File.binread('document.pdf')),
  mime_type: 'application/pdf'
))
response = JSON.parse(response_json)
puts response['content']
```

Accepts `model`, `file`, `mime_type`, `pages`.

#### `create_file(request_json)`

Upload a file.

```ruby
response_json = client.create_file(JSON.generate(
  file: Base64.encode64(File.binread('data.jsonl')),
  purpose: 'batch',
  filename: 'data.jsonl'
))
file_obj = JSON.parse(response_json)
puts file_obj['id']
```

Accepts `file`, `purpose`, `filename`.

#### `retrieve_file(file_id)`

Retrieve metadata about an uploaded file.

```ruby
response_json = client.retrieve_file('file-abc123')
file_obj = JSON.parse(response_json)
puts file_obj['filename']
```

#### `delete_file(file_id)`

Delete an uploaded file.

```ruby
response_json = client.delete_file('file-abc123')
result = JSON.parse(response_json)
puts result['deleted']
```

#### `list_files(query_json)`

List files. Pass `nil` or a JSON string with query parameters.

```ruby
response_json = client.list_files(JSON.generate(purpose: 'batch'))
files = JSON.parse(response_json)
files['data'].each { |f| puts f['id'] }
```

#### `file_content(file_id)`

Retrieve raw file content as a base64-encoded string.

```ruby
base64_content = client.file_content('file-abc123')
content = Base64.decode64(base64_content)
```

#### `create_batch(request_json)`

Create a new batch.

```ruby
response_json = client.create_batch(JSON.generate(
  input_file_id: 'file-abc123',
  endpoint: '/v1/chat/completions',
  completion_window: '24h'
))
batch = JSON.parse(response_json)
puts batch['id']
```

Accepts `input_file_id`, `endpoint`, `completion_window`, `metadata`.

#### `retrieve_batch(batch_id)`

Retrieve a batch by ID.

```ruby
response_json = client.retrieve_batch('batch-abc123')
batch = JSON.parse(response_json)
puts batch['status']
```

#### `list_batches(query_json)`

List batches. Pass `nil` or a JSON string with query parameters.

```ruby
response_json = client.list_batches(nil)
batches = JSON.parse(response_json)
batches['data'].each { |b| puts "#{b['id']}: #{b['status']}" }
```

#### `cancel_batch(batch_id)`

Cancel a batch.

```ruby
response_json = client.cancel_batch('batch-abc123')
batch = JSON.parse(response_json)
puts batch['status']
```

#### `create_response(request_json)`

Create a new response via the Responses API.

```ruby
response_json = client.create_response(JSON.generate(
  model: 'gpt-4',
  input: 'Summarize this document'
))
response = JSON.parse(response_json)
puts response['output']
```

Accepts `model`, `input`, `instructions`, `max_output_tokens`, `temperature`, `top_p`.

#### `retrieve_response(response_id)`

Retrieve a response by ID.

```ruby
response_json = client.retrieve_response('resp-abc123')
response = JSON.parse(response_json)
puts response['status']
```

#### `cancel_response(response_id)`

Cancel a response.

```ruby
response_json = client.cancel_response('resp-abc123')
response = JSON.parse(response_json)
puts response['status']
```

#### `register_provider(name, config_json)`

Register a custom provider at runtime.

```ruby
client.register_provider('my-provider', JSON.generate(
  base_url: 'https://my-llm.example.com/v1',
  auth_header: 'Authorization',
  auth_prefix: 'Bearer '
))
```

#### `unregister_provider(name)`

Remove a previously registered custom provider.

```ruby
client.unregister_provider('my-provider')
```

#### `add_hook(hook)`

Register a lifecycle hook. The hook is a Hash with lambda callbacks for request, response, and error events.

```ruby
client.add_hook({
  on_request: ->(request_json) {
    puts "Sending request: #{request_json[0..80]}..."
  },
  on_response: ->(request_json, response_json) {
    puts "Got response for model: #{JSON.parse(request_json)['model']}"
  },
  on_error: ->(request_json, error_message) {
    warn "Request failed: #{error_message}"
  }
})
```

All three callbacks are optional -- provide only the ones you need.

#### `budget_used`

Returns the total budget consumed so far (as a `Float`). Only meaningful when a `budget` was configured in the constructor.

```ruby
puts "Budget used: $#{client.budget_used}"
```

## Types

All responses are returned as JSON strings. After parsing with `JSON.parse`, you get Ruby Hashes and Arrays with string keys. The tables below document the structure.

### `ChatCompletionResponse`

| Key | Type | Description |
|-----|------|-------------|
| `"id"` | `String` | Response ID |
| `"model"` | `String` | Model used |
| `"choices"` | `Array<Hash>` | Completion choices |
| `"usage"` | `Hash?` | Token usage |
| `"created"` | `Integer` | Unix timestamp |

### `Choice`

| Key | Type | Description |
|-----|------|-------------|
| `"index"` | `Integer` | Choice index |
| `"message"` | `Hash` | The assistant's message |
| `"finish_reason"` | `String?` | Why generation stopped (`stop`, `length`, `tool_calls`) |

### `AssistantMessage`

| Key | Type | Description |
|-----|------|-------------|
| `"content"` | `String?` | Text content |
| `"tool_calls"` | `Array<Hash>?` | Tool calls made by the assistant |
| `"refusal"` | `String?` | Refusal message |

### `ChatCompletionChunk`

Returned inside the array from `chat_stream`.

| Key | Type | Description |
|-----|------|-------------|
| `"id"` | `String` | Response ID |
| `"model"` | `String` | Model used |
| `"choices"` | `Array<Hash>` | Stream choices with deltas |
| `"usage"` | `Hash?` | Token usage (final chunk only) |

### `Usage`

| Key | Type | Description |
|-----|------|-------------|
| `"prompt_tokens"` | `Integer` | Tokens consumed by the prompt |
| `"completion_tokens"` | `Integer` | Tokens consumed by the completion |
| `"total_tokens"` | `Integer` | Total tokens |

### `EmbeddingResponse`

| Key | Type | Description |
|-----|------|-------------|
| `"data"` | `Array<Hash>` | Embedding objects, each with `"embedding"` (array of floats) and `"index"` |
| `"model"` | `String` | Model used |
| `"usage"` | `Hash` | Token usage |

### `ModelsListResponse`

| Key | Type | Description |
|-----|------|-------------|
| `"data"` | `Array<Hash>` | Available models, each with `"id"`, `"created"`, `"owned_by"` |

## Error Handling

All errors are raised as Ruby exceptions inheriting from `LiterLlm::Error` (which itself inherits from `StandardError`). Invalid arguments raise `ArgumentError`.

| Exception | Trigger |
|-----------|---------|
| `LiterLlm::Error` | Base class for all liter-llm errors |
| `LiterLlm::AuthenticationError` | API key rejected (HTTP 401/403) |
| `LiterLlm::RateLimitError` | Rate limit exceeded (HTTP 429) |
| `LiterLlm::BadRequestError` | Malformed request (HTTP 400) |
| `LiterLlm::ContextWindowExceededError` | Prompt exceeds context window (subclass of `BadRequestError`) |
| `LiterLlm::ContentPolicyError` | Content policy violation (subclass of `BadRequestError`) |
| `LiterLlm::NotFoundError` | Model/resource not found (HTTP 404) |
| `LiterLlm::ServerError` | Provider 5xx error |
| `LiterLlm::ServiceUnavailableError` | Provider temporarily unavailable (HTTP 502/503) |
| `LiterLlm::TimeoutError` | Request timed out |
| `LiterLlm::NetworkError` | Network-level failure |
| `LiterLlm::StreamingError` | Error reading streaming response |
| `LiterLlm::EndpointNotSupportedError` | Provider does not support the endpoint |
| `LiterLlm::SerializationError` | JSON serialization/deserialization failure |
| `LiterLlm::BudgetExceededError` | Budget limit exceeded |

```ruby
require 'liter_llm'

begin
  response = JSON.parse(client.chat(JSON.generate(
    model: 'gpt-4', messages: [{ role: 'user', content: 'Hello' }]
  )))
rescue ArgumentError => e
  puts "Bad arguments: #{e.message}"
rescue LiterLlm::RateLimitError => e
  puts "Rate limited: #{e.message}"
rescue LiterLlm::AuthenticationError => e
  puts "Auth failed: #{e.message}"
rescue LiterLlm::BudgetExceededError => e
  puts "Budget exceeded: #{e.message}"
rescue LiterLlm::Error => e
  # Catch-all for other liter-llm errors
  puts "Error: #{e.message}"
end
```

## Example

```ruby
require 'liter_llm'
require 'json'

client = LiterLlm::LlmClient.new(ENV.fetch('OPENAI_API_KEY'))

# Non-streaming
response = JSON.parse(client.chat(JSON.generate(
  model: 'gpt-4',
  messages: [{ role: 'user', content: 'Hello!' }],
  max_tokens: 256
)))
puts response.dig('choices', 0, 'message', 'content')

# Streaming
chunks = JSON.parse(client.chat_stream(JSON.generate(
  model: 'gpt-4',
  messages: [{ role: 'user', content: 'Tell me a joke' }]
)))
chunks.each do |chunk|
  content = chunk.dig('choices', 0, 'delta', 'content')
  print content if content
end
puts
```
