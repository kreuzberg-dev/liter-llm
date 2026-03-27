---
description: "liter-llm PHP API reference"
---

# PHP API Reference

The PHP extension wraps the Rust core via `ext-php-rs`. All request/response data is exchanged as JSON strings.

## Installation

Install the native PHP extension, then:

```php
// php.ini
extension=liter_llm
```

Or install the pure-PHP fallback via Composer:

```bash
composer require kreuzberg/liter-llm
```

## Client

### Constructor

```php
<?php

declare(strict_types=1);

use LiterLlm\LlmClient;

$client = new LlmClient(
    api_key: 'sk-...',
    base_url: 'https://api.openai.com/v1',  // optional, default: null
    model_hint: 'groq/llama3-70b',          // optional, default: null
    max_retries: 3,                          // default: 3
    timeout_secs: 60,                        // default: 60
);
```

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `$api_key` | `string` | *required* | API key for authentication |
| `$base_url` | `?string` | `null` | Override provider base URL |
| `$model_hint` | `?string` | `null` | Hint for provider auto-detection (e.g. `"groq/llama3-70b"`) |
| `$max_retries` | `?int` | `3` | Retries on 429/5xx |
| `$timeout_secs` | `?int` | `60` | Request timeout in seconds |

### Methods

All methods accept a JSON-encoded request string and return a JSON-encoded response string. Use `json_encode()` / `json_decode()` for conversion.

#### `chat(string $requestJson): string`

Send a chat completion request.

```php
$response = json_decode($client->chat(json_encode([
    'model'    => 'gpt-4',
    'messages' => [['role' => 'user', 'content' => 'Hello']],
])), true);

echo $response['choices'][0]['message']['content'];
```

#### `chatStream(string $requestJson): string`

Send a streaming chat completion and collect all chunks. Returns a JSON-encoded array of `ChatCompletionChunk` objects.

PHP's synchronous execution model does not support true incremental streaming. The full SSE stream is consumed on the Rust side and returned as a JSON array.

```php
$chunks = json_decode($client->chatStream(json_encode([
    'model'    => 'gpt-4',
    'messages' => [['role' => 'user', 'content' => 'Hello']],
])), true);

foreach ($chunks as $chunk) {
    echo $chunk['choices'][0]['delta']['content'] ?? '';
}
```

#### `embed(string $requestJson): string`

Send an embedding request.

```php
$response = json_decode($client->embed(json_encode([
    'model' => 'text-embedding-3-small',
    'input' => 'Hello',
])), true);
```

#### `listModels(): string`

List available models. Takes no arguments.

```php
$response = json_decode($client->listModels(), true);
```

#### `imageGenerate(string $requestJson): string`

Generate an image from a text prompt.

```php
$response = json_decode($client->imageGenerate(json_encode([
    'prompt' => 'A sunset over mountains',
    'model'  => 'dall-e-3',
])), true);
```

#### `speech(string $requestJson): string`

Generate speech audio from text. Returns raw audio bytes as a binary string.

```php
$audio = $client->speech(json_encode([
    'model' => 'tts-1',
    'input' => 'Hello',
    'voice' => 'alloy',
]));
file_put_contents('output.mp3', $audio);
```

#### `transcribe(string $requestJson): string`

Transcribe audio to text.

```php
$response = json_decode($client->transcribe(json_encode([
    'model' => 'whisper-1',
    'file'  => $base64Audio,
])), true);
```

#### `moderate(string $requestJson): string`

Check content against moderation policies.

```php
$response = json_decode($client->moderate(json_encode([
    'input' => 'some text',
])), true);
```

#### `rerank(string $requestJson): string`

Rerank documents by relevance to a query.

```php
$response = json_decode($client->rerank(json_encode([
    'model'     => 'rerank-v1',
    'query'     => 'search query',
    'documents' => ['doc a', 'doc b'],
])), true);
```

#### File Management

```php
// Upload a file
string createFile(string $requestJson): string

// Retrieve file metadata
string retrieveFile(string $fileId): string

// Delete a file
string deleteFile(string $fileId): string

// List files (pass null or JSON query string)
string listFiles(?string $queryJson): string

// Download file content (returns raw bytes as binary string)
string fileContent(string $fileId): string
```

#### Batch Management

```php
string createBatch(string $requestJson): string
string retrieveBatch(string $batchId): string
string listBatches(?string $queryJson): string
string cancelBatch(string $batchId): string
```

#### Responses API

```php
string createResponse(string $requestJson): string
string retrieveResponse(string $responseId): string
string cancelResponse(string $responseId): string
```

## Types

All types are documented as PHPStan type aliases in the `LlmClient` class. Key shapes:

### ChatCompletionResponse

```php
array{
    id: string,
    object: string,
    created: int,
    model: string,
    choices: list<array{
        index: int,
        message: array{content?: string|null, tool_calls?: list<...>},
        finish_reason: string|null
    }>,
    usage?: array{prompt_tokens: int, completion_tokens: int, total_tokens: int}
}
```

### EmbeddingResponse

```php
array{
    object: string,
    data: list<array{object: string, embedding: list<float>, index: int}>,
    model: string,
    usage: array{prompt_tokens: int, completion_tokens: int, total_tokens: int}
}
```

## Error Handling

All methods throw `\RuntimeException` on failure. The exception message contains details about the error (network, auth, rate limit, provider error, or invalid request).

```php
try {
    $response = json_decode($client->chat(json_encode($request)), true);
} catch (\RuntimeException $e) {
    echo "Error: " . $e->getMessage() . "\n";
}
```

## Example

```php
<?php

declare(strict_types=1);

use LiterLlm\LlmClient;

$client = new LlmClient(
    api_key: getenv('OPENAI_API_KEY') ?: '',
);

$response = json_decode($client->chat(json_encode([
    'model'    => 'gpt-4',
    'messages' => [['role' => 'user', 'content' => 'Hello!']],
])), true);

echo $response['choices'][0]['message']['content'] . "\n";
```
