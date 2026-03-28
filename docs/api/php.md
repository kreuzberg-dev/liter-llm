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
    apiKey: 'sk-...',
    baseUrl: 'https://api.openai.com/v1',  // optional, default: null
    modelHint: 'groq/llama3-70b',          // optional, default: null
    maxRetries: 3,                          // default: 3
    timeoutSecs: 60,                        // default: 60
    cacheConfig: ['max_entries' => 256, 'ttl_seconds' => 300],  // optional
    budgetConfig: ['global_limit' => 10.0, 'enforcement' => 'hard'],  // optional
);
```

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `$apiKey` | `string` | *required* | API key for authentication |
| `$baseUrl` | `?string` | `null` | Override provider base URL |
| `$modelHint` | `?string` | `null` | Hint for provider auto-detection (e.g. `"groq/llama3-70b"`) |
| `$maxRetries` | `?int` | `3` | Retries on 429/5xx |
| `$timeoutSecs` | `?int` | `60` | Request timeout in seconds |
| `$cacheConfig` | `?array` | `null` | Response cache settings (see below) |
| `$budgetConfig` | `?array` | `null` | Spend budget settings (see below) |

#### Cache Config

| Key | Type | Description |
|-----|------|-------------|
| `max_entries` | `int` | Maximum cached responses |
| `ttl_seconds` | `int` | Time-to-live for cache entries |

#### Budget Config

| Key | Type | Description |
|-----|------|-------------|
| `global_limit` | `float` | Maximum spend in USD |
| `enforcement` | `string` | `"hard"` (reject over-budget) or `"soft"` (warn only) |

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

// $response['data'][0]['embedding'] contains the vector
```

#### `listModels(): string`

List available models. Takes no arguments.

```php
$response = json_decode($client->listModels(), true);

foreach ($response['data'] as $model) {
    echo $model['id'] . "\n";
}
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

echo $response['text'];
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

#### `createFile(string $requestJson): string`

Upload a file.

```php
$response = json_decode($client->createFile(json_encode([
    'file'    => base64_encode(file_get_contents('data.jsonl')),
    'purpose' => 'fine-tune',
])), true);
```

#### `retrieveFile(string $fileId): string`

Retrieve file metadata.

```php
$response = json_decode($client->retrieveFile('file-abc123'), true);
```

#### `deleteFile(string $fileId): string`

Delete a file.

```php
$response = json_decode($client->deleteFile('file-abc123'), true);
```

#### `listFiles(?string $queryJson): string`

List files. Pass `null` or a JSON query string.

```php
$response = json_decode($client->listFiles(json_encode([
    'purpose' => 'fine-tune',
])), true);
```

#### `fileContent(string $fileId): string`

Download file content. Returns raw bytes as a binary string.

```php
$content = $client->fileContent('file-abc123');
file_put_contents('downloaded.jsonl', $content);
```

#### `createBatch(string $requestJson): string`

Create a new batch job.

```php
$response = json_decode($client->createBatch(json_encode([
    'input_file_id'     => 'file-abc123',
    'endpoint'          => '/v1/chat/completions',
    'completion_window' => '24h',
])), true);
```

#### `retrieveBatch(string $batchId): string`

Retrieve a batch by ID.

```php
$response = json_decode($client->retrieveBatch('batch-abc123'), true);
```

#### `listBatches(?string $queryJson): string`

List batches. Pass `null` or a JSON query string.

```php
$response = json_decode($client->listBatches(null), true);
```

#### `cancelBatch(string $batchId): string`

Cancel an in-progress batch.

```php
$response = json_decode($client->cancelBatch('batch-abc123'), true);
```

#### `createResponse(string $requestJson): string`

Create a new response via the Responses API.

```php
$response = json_decode($client->createResponse(json_encode([
    'model' => 'gpt-4',
    'input' => 'Summarize this document...',
])), true);
```

#### `retrieveResponse(string $responseId): string`

Retrieve a response by ID.

```php
$response = json_decode($client->retrieveResponse('resp-abc123'), true);
```

#### `cancelResponse(string $responseId): string`

Cancel a response.

```php
$response = json_decode($client->cancelResponse('resp-abc123'), true);
```

### Hooks

Register a hook object to observe requests, responses, and errors. The hook is an object implementing any combination of `onRequest`, `onResponse`, and `onError` methods.

```php
$client->addHook(new class {
    public function onRequest(string $requestJson): void {
        error_log('Request: ' . $requestJson);
    }

    public function onResponse(string $requestJson, string $responseJson): void {
        error_log('Response received');
    }

    public function onError(string $requestJson, string $errorMessage): void {
        error_log('Error: ' . $errorMessage);
    }
});
```

All three methods are optional -- implement only the ones you need.

### Provider Management

#### `registerProvider(string $providerJson): void`

Register a custom provider at runtime.

```php
$client->registerProvider(json_encode([
    'name'           => 'my-provider',
    'base_url'       => 'https://api.my-provider.com/v1',
    'auth_header'    => 'Authorization',
    'model_prefixes' => ['my-provider/'],
]));
```

#### `unregisterProvider(string $name): void`

Remove a previously registered provider by name.

```php
$client->unregisterProvider('my-provider');
```

### Budget

#### `getBudgetUsed(): float`

Returns the total spend tracked by the budget system (in USD). Returns `0.0` if no budget is configured.

```php
$used = $client->getBudgetUsed();
echo "Budget used: \${$used}\n";
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

### ChatCompletionChunk

```php
array{
    id: string,
    object: string,
    created: int,
    model: string,
    choices: list<array{
        index: int,
        delta: array{content?: string|null, tool_calls?: list<...>},
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

### ModelsListResponse

```php
array{
    data: list<array{id: string, object: string, created: int, owned_by: string}>
}
```

## Error Handling

All methods throw `\LiterLlm\LlmException` (which extends `\RuntimeException`) on failure. Specific subclasses allow fine-grained catch blocks.

| Exception | Trigger |
|-----------|---------|
| `LlmException` | Base class for all liter-llm errors |
| `AuthenticationException` | API key rejected (HTTP 401/403) |
| `RateLimitedException` | Rate limit exceeded (HTTP 429) |
| `BadRequestException` | Malformed request (HTTP 400) |
| `ContextWindowExceededException` | Prompt exceeds context window (subclass of `BadRequestException`) |
| `ContentPolicyException` | Content policy violation (subclass of `BadRequestException`) |
| `NotFoundException` | Model/resource not found (HTTP 404) |
| `ServerException` | Provider 5xx error |
| `ServiceUnavailableException` | Provider temporarily unavailable (HTTP 502/503) |
| `TimeoutException` | Request timed out |
| `NetworkException` | Network-level failure |
| `StreamingException` | Error reading streaming response |
| `EndpointNotSupportedException` | Provider does not support the endpoint |
| `SerializationException` | JSON serialization/deserialization failure |

```php
use LiterLlm\LlmException;
use LiterLlm\RateLimitedException;
use LiterLlm\AuthenticationException;

try {
    $response = json_decode($client->chat(json_encode($request)), true);
} catch (RateLimitedException $e) {
    echo "Rate limited: " . $e->getMessage() . "\n";
} catch (AuthenticationException $e) {
    echo "Auth failed: " . $e->getMessage() . "\n";
} catch (LlmException $e) {
    echo "Error: " . $e->getMessage() . "\n";
}
```

## Example

```php
<?php

declare(strict_types=1);

use LiterLlm\LlmClient;

$client = new LlmClient(
    apiKey: getenv('OPENAI_API_KEY') ?: '',
);

// Non-streaming
$response = json_decode($client->chat(json_encode([
    'model'      => 'gpt-4',
    'messages'   => [['role' => 'user', 'content' => 'Hello!']],
    'max_tokens' => 256,
])), true);

echo $response['choices'][0]['message']['content'] . "\n";

// Streaming
$chunks = json_decode($client->chatStream(json_encode([
    'model'    => 'gpt-4',
    'messages' => [['role' => 'user', 'content' => 'Tell me a joke']],
])), true);

foreach ($chunks as $chunk) {
    echo $chunk['choices'][0]['delta']['content'] ?? '';
}
echo "\n";
```
