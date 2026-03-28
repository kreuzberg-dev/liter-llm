---
description: "liter-llm TypeScript / Node.js API reference"
---

# TypeScript / Node.js API Reference

## Installation

```bash
pnpm add @kreuzberg/liter-llm
# or
npm install @kreuzberg/liter-llm
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
| `cache` | `CacheOptions \| undefined` | `undefined` | Cache config: `{ maxEntries: 256, ttlSeconds: 300 }` |
| `budget` | `BudgetOptions \| undefined` | `undefined` | Budget config: `{ globalLimit: 10.0, modelLimits: {}, enforcement: "hard" }` |
| `extraHeaders` | `Record<string, string> \| undefined` | `undefined` | Additional HTTP headers |
| `cooldown` | `number \| undefined` | `undefined` | Cooldown period in seconds after transient errors |
| `rateLimit` | `RateLimitOptions \| undefined` | `undefined` | Rate limit config: `{ rpm: 60, tpm: 100000 }` |
| `healthCheck` | `number \| undefined` | `undefined` | Health check interval in seconds |
| `costTracking` | `boolean \| undefined` | `undefined` | Enable per-request cost tracking |
| `tracing` | `boolean \| undefined` | `undefined` | Enable OpenTelemetry tracing spans |

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

#### `search(request)`

Perform a web or document search across supported providers.

```typescript
async search(request: object): Promise<object>
```

```typescript
const resp = await client.search({
  model: "brave/search",
  query: "latest AI news",
  maxResults: 10,
});
```

#### `ocr(request)`

Extract text from documents or images using OCR with Markdown output.

```typescript
async ocr(request: object): Promise<object>
```

```typescript
const resp = await client.ocr({
  model: "mistral/pixtral",
  file: base64Content,
  mimeType: "application/pdf",
});
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

#### `registerProvider(config)`

Register a custom provider for self-hosted or unsupported LLM endpoints.

```typescript
client.registerProvider({
  name: "my-provider",
  baseUrl: "https://my-llm.example.com/v1",
  authHeader: "Authorization",
  modelPrefixes: ["my-provider/"],
});
```

#### `unregisterProvider(name)`

Remove a previously registered custom provider. Returns `true` if found and removed.

```typescript
const removed = client.unregisterProvider("my-provider");
```

#### `addHook(hook)`

Register a lifecycle hook for request/response/error events.

```typescript
client.addHook({
  onRequest(req) { console.log(`Sending: ${req.model}`); },
  onResponse(req, res) { console.log(`Tokens: ${res.usage?.totalTokens}`); },
  onError(req, err) { console.error(`Error: ${err}`); },
});
```

All callbacks are optional, fire-and-forget, and can be sync or async.

#### `budgetUsed`

Getter returning the total spend in USD so far.

```typescript
console.log(`Budget used: $${client.budgetUsed.toFixed(2)}`);
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

### Choice

| Field | Type | Description |
|-------|------|-------------|
| `index` | `number` | Choice index |
| `message` | `AssistantMessage` | The assistant's message |
| `finishReason` | `string \| null` | Why generation stopped (`stop`, `length`, `tool_calls`) |

### AssistantMessage

| Field | Type | Description |
|-------|------|-------------|
| `content` | `string \| null` | Text content |
| `toolCalls` | `ToolCall[] \| undefined` | Tool calls made by the assistant |
| `refusal` | `string \| null` | Refusal message |

### Usage

| Field | Type | Description |
|-------|------|-------------|
| `promptTokens` | `number` | Tokens consumed by the prompt |
| `completionTokens` | `number` | Tokens consumed by the completion |
| `totalTokens` | `number` | Total tokens |

### EmbeddingResponse

| Field | Type | Description |
|-------|------|-------------|
| `data` | `EmbeddingObject[]` | Embedding vectors |
| `model` | `string` | Model used |
| `usage` | `Usage` | Token usage |

### ModelsListResponse

| Field | Type | Description |
|-------|------|-------------|
| `data` | `ModelObject[]` | Available models |

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

| Category | Trigger |
|----------|---------|
| `Authentication` | API key rejected (HTTP 401/403) |
| `RateLimited` | Rate limit exceeded (HTTP 429) |
| `BadRequest` | Malformed request (HTTP 400) |
| `ContextWindowExceeded` | Prompt exceeds context window |
| `ContentPolicy` | Content policy violation |
| `NotFound` | Model/resource not found (HTTP 404) |
| `ServerError` | Provider 5xx error |
| `ServiceUnavailable` | Provider temporarily unavailable (HTTP 502/503) |
| `Timeout` | Request timed out |
| `Network` | Network-level failure |
| `Streaming` | Error reading streaming response |
| `EndpointNotSupported` | Provider does not support the endpoint |
| `InvalidHeader` | Custom header name or value is invalid |
| `Serialization` | JSON serialization/deserialization failure |

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
