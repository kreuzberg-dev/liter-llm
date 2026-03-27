---
description: "liter-llm TypeScript / Node.js API reference"
---

# TypeScript / Node.js API Reference

## Installation

```bash
pnpm add liter-llm
# or
npm install liter-llm
```

## Client

### Constructor

```typescript
import { LlmClient } from 'liter-llm';

const client = new LlmClient({
  apiKey: string,
  baseUrl?: string,
  modelHint?: string,
  maxRetries?: number,     // default: 3
  timeoutSecs?: number,    // default: 60
});
```

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `apiKey` | `string` | *required* | API key for authentication |
| `baseUrl` | `string \| undefined` | `undefined` | Override provider base URL |
| `modelHint` | `string \| undefined` | `undefined` | Hint for provider auto-detection (e.g. `"groq/llama3-70b"`) |
| `maxRetries` | `number \| undefined` | `3` | Retries on 429 / 5xx responses |
| `timeoutSecs` | `number \| undefined` | `60` | Request timeout in seconds |

### Methods

All methods are async and return Promises. Request and response objects use camelCase keys (converted automatically from the snake_case wire format).

#### `chat(request)`

Send a chat completion request.

```typescript
async chat(request: object): Promise<object>
```

```typescript
const resp = await client.chat({
  model: "gpt-4",
  messages: [{ role: "user", content: "Hi" }],
});
console.log(resp.choices[0].message.content);
```

#### `chatStream(request)`

Collect all streaming chat completion chunks into an array. The full SSE stream is consumed on the Rust side before the Promise resolves.

```typescript
async chatStream(request: object): Promise<object[]>
```

```typescript
const chunks = await client.chatStream({
  model: "gpt-4",
  messages: [{ role: "user", content: "Hi" }],
});
for (const chunk of chunks) {
  process.stdout.write(chunk.choices[0]?.delta?.content ?? "");
}
```

#### `embed(request)`

Send an embedding request.

```typescript
async embed(request: object): Promise<object>
```

#### `listModels()`

List available models from the provider.

```typescript
async listModels(): Promise<object>
```

#### `imageGenerate(request)`

Generate an image from a text prompt.

```typescript
async imageGenerate(request: object): Promise<object>
```

#### `speech(request)`

Generate speech audio from text. Returns a `Buffer` of raw audio bytes.

```typescript
async speech(request: object): Promise<Buffer>
```

#### `transcribe(request)`

Transcribe audio to text.

```typescript
async transcribe(request: object): Promise<object>
```

#### `moderate(request)`

Check content against moderation policies.

```typescript
async moderate(request: object): Promise<object>
```

#### `rerank(request)`

Rerank documents by relevance to a query.

```typescript
async rerank(request: object): Promise<object>
```

#### `createFile(request)`

Upload a file.

```typescript
async createFile(request: object): Promise<object>
```

#### `retrieveFile(fileId)`

Retrieve metadata for a file by ID.

```typescript
async retrieveFile(fileId: string): Promise<object>
```

#### `deleteFile(fileId)`

Delete a file by ID.

```typescript
async deleteFile(fileId: string): Promise<object>
```

#### `listFiles(query?)`

List files, optionally filtered.

```typescript
async listFiles(query?: object | null): Promise<object>
```

#### `fileContent(fileId)`

Retrieve the raw content of a file. Returns a `Buffer`.

```typescript
async fileContent(fileId: string): Promise<Buffer>
```

#### `createBatch(request)`

Create a new batch job.

```typescript
async createBatch(request: object): Promise<object>
```

#### `retrieveBatch(batchId)`

Retrieve a batch by ID.

```typescript
async retrieveBatch(batchId: string): Promise<object>
```

#### `listBatches(query?)`

List batches, optionally filtered.

```typescript
async listBatches(query?: object | null): Promise<object>
```

#### `cancelBatch(batchId)`

Cancel an in-progress batch.

```typescript
async cancelBatch(batchId: string): Promise<object>
```

#### `createResponse(request)`

Create a new response via the Responses API.

```typescript
async createResponse(request: object): Promise<object>
```

#### `retrieveResponse(id)`

Retrieve a response by ID.

```typescript
async retrieveResponse(id: string): Promise<object>
```

#### `cancelResponse(id)`

Cancel an in-progress response.

```typescript
async cancelResponse(id: string): Promise<object>
```

### Module Functions

#### `version()`

Returns the library version string.

```typescript
import { version } from 'liter-llm';
console.log(version());
```

## Types

Response objects are plain JavaScript objects with camelCase keys.

### ChatCompletionResponse

| Field | Type | Description |
|-------|------|-------------|
| `id` | `string` | Response ID |
| `model` | `string` | Model used |
| `choices` | `Choice[]` | Completion choices |
| `usage` | `Usage \| undefined` | Token usage |
| `created` | `number` | Unix timestamp |

### ChatCompletionChunk

| Field | Type | Description |
|-------|------|-------------|
| `id` | `string` | Response ID |
| `model` | `string` | Model used |
| `choices` | `StreamChoice[]` | Stream choices with deltas |
| `usage` | `Usage \| undefined` | Token usage (final chunk only) |

## Error Handling

Errors are thrown as JavaScript `Error` objects. The message includes a bracketed label for the error category:

```typescript
try {
  await client.chat({ model: "gpt-4", messages: [] });
} catch (err) {
  // "[Authentication] Invalid API key"
  // "[RateLimited] Too many requests"
  // "[BadRequest] Messages must not be empty"
  console.error(err.message);
}
```

Error categories: `Authentication`, `RateLimited`, `BadRequest`, `ContextWindowExceeded`, `ContentPolicy`, `NotFound`, `ServerError`, `ServiceUnavailable`, `Timeout`, `Network`, `Streaming`, `EndpointNotSupported`, `InvalidHeader`, `Serialization`.

## Example

```typescript
import { LlmClient } from 'liter-llm';

const client = new LlmClient({
  apiKey: process.env.OPENAI_API_KEY!,
});

const resp = await client.chat({
  model: "gpt-4",
  messages: [{ role: "user", content: "Hello!" }],
  maxTokens: 256,
});
console.log(resp.choices[0].message.content);
```
