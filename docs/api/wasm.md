---
description: "liter-llm WebAssembly API reference"
---

# WebAssembly API Reference

The WASM package exposes a JavaScript-friendly `LlmClient` class via `wasm-bindgen`. It works in both browser and Node.js environments, using the native `fetch` API for HTTP.

## Installation

```bash
npm install @kreuzberg/liter-llm-wasm
```

## Setup

The WASM module must be initialized before use:

```javascript
import init, { LlmClient } from '@kreuzberg/liter-llm-wasm';

await init(); // Initialize the WASM module
```

## Client

### Constructor

```typescript
const client = new LlmClient({
  apiKey: string,
  baseUrl?: string,
  modelHint?: string,
  maxRetries?: number,     // default: 3
  timeoutSecs?: number,    // default: 60
  authHeader?: string,     // override full Authorization header value
  cache?: {
    maxEntries?: number,   // default: 256
    ttlSeconds?: number,   // default: 300
  },
  budget?: {
    globalLimit?: number,  // max spend in USD
    enforcement?: string,  // "hard" | "soft"
  },
});
```

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `apiKey` | `string` | *required* | API key (empty string for no-auth providers) |
| `baseUrl` | `string?` | `undefined` | Override provider base URL |
| `modelHint` | `string?` | `undefined` | Hint for provider auto-detection (e.g. `"groq/llama3-70b"`) |
| `maxRetries` | `number?` | `3` | Retries on 429/5xx |
| `timeoutSecs` | `number?` | `60` | Request timeout in seconds |
| `authHeader` | `string?` | `undefined` | Override `Authorization` header value |
| `cache` | `object?` | `undefined` | Response cache settings (see below) |
| `budget` | `object?` | `undefined` | Spend budget settings (see below) |

#### Cache Options

| Key | Type | Description |
|-----|------|-------------|
| `maxEntries` | `number` | Maximum cached responses |
| `ttlSeconds` | `number` | Time-to-live for cache entries |

#### Budget Options

| Key | Type | Description |
|-----|------|-------------|
| `globalLimit` | `number` | Maximum spend in USD |
| `enforcement` | `string` | `"hard"` (reject over-budget) or `"soft"` (warn only) |

### Methods

All methods are async and return Promises. Request and response objects use camelCase keys (converted automatically from the snake_case wire format).

#### `chat(request)`

Send a chat completion request.

```typescript
async chat(request: ChatCompletionRequest): Promise<ChatCompletionResponse>
```

```javascript
const resp = await client.chat({
  model: "gpt-4",
  messages: [{ role: "user", content: "Hello!" }],
  maxTokens: 256,
});
console.log(resp.choices[0].message.content);
```

#### `chatStream(request)`

Collect all streaming chat completion chunks. The full SSE stream is consumed on the Rust/WASM side before the Promise resolves.

```typescript
async chatStream(request: ChatCompletionRequest): Promise<ChatCompletionChunk[]>
```

```javascript
const chunks = await client.chatStream({
  model: "gpt-4",
  messages: [{ role: "user", content: "Tell me a joke" }],
});
for (const chunk of chunks) {
  process.stdout.write(chunk.choices[0]?.delta?.content ?? "");
}
```

#### `embed(request)`

Send an embedding request.

```typescript
async embed(request: EmbeddingRequest): Promise<EmbeddingResponse>
```

```javascript
const resp = await client.embed({
  model: "text-embedding-3-small",
  input: "Hello",
});
// resp.data[0].embedding contains the vector
```

#### `listModels()`

List available models.

```typescript
async listModels(): Promise<ModelsListResponse>
```

```javascript
const resp = await client.listModels();
resp.data.forEach(m => console.log(m.id));
```

#### `imageGenerate(request)`

Generate an image from a text prompt.

```typescript
async imageGenerate(request: CreateImageRequest): Promise<ImagesResponse>
```

```javascript
const resp = await client.imageGenerate({
  prompt: "A sunset over mountains",
  model: "dall-e-3",
});
```

#### `speech(request)`

Generate speech audio from text. Returns a `Uint8Array`.

```typescript
async speech(request: CreateSpeechRequest): Promise<Uint8Array>
```

```javascript
const audio = await client.speech({
  model: "tts-1",
  input: "Hello",
  voice: "alloy",
});
```

#### `transcribe(request)`

Transcribe audio to text.

```typescript
async transcribe(request: CreateTranscriptionRequest): Promise<TranscriptionResponse>
```

```javascript
const resp = await client.transcribe({
  model: "whisper-1",
  file: audioBase64,
});
console.log(resp.text);
```

#### `moderate(request)`

Check content against moderation policies.

```typescript
async moderate(request: ModerationRequest): Promise<ModerationResponse>
```

```javascript
const resp = await client.moderate({ input: "some text" });
```

#### `rerank(request)`

Rerank documents by relevance to a query.

```typescript
async rerank(request: RerankRequest): Promise<RerankResponse>
```

```javascript
const resp = await client.rerank({
  model: "rerank-v1",
  query: "search query",
  documents: ["doc a", "doc b"],
});
```

#### `createFile(request)`

Upload a file.

```typescript
async createFile(request: CreateFileRequest): Promise<FileObject>
```

#### `retrieveFile(fileId)`

Retrieve metadata for a file by ID.

```typescript
async retrieveFile(fileId: string): Promise<FileObject>
```

#### `deleteFile(fileId)`

Delete a file by ID.

```typescript
async deleteFile(fileId: string): Promise<DeleteResponse>
```

#### `listFiles(query?)`

List files, optionally filtered.

```typescript
async listFiles(query?: FileListQuery): Promise<FileListResponse>
```

#### `fileContent(fileId)`

Retrieve the raw content of a file. Returns a `Uint8Array`.

```typescript
async fileContent(fileId: string): Promise<Uint8Array>
```

#### `createBatch(request)`

Create a new batch job.

```typescript
async createBatch(request: CreateBatchRequest): Promise<BatchObject>
```

#### `retrieveBatch(batchId)`

Retrieve a batch by ID.

```typescript
async retrieveBatch(batchId: string): Promise<BatchObject>
```

#### `listBatches(query?)`

List batches, optionally filtered.

```typescript
async listBatches(query?: BatchListQuery): Promise<BatchListResponse>
```

#### `cancelBatch(batchId)`

Cancel an in-progress batch.

```typescript
async cancelBatch(batchId: string): Promise<BatchObject>
```

#### `createResponse(request)`

Create a new response via the Responses API.

```typescript
async createResponse(request: CreateResponseRequest): Promise<ResponseObject>
```

#### `retrieveResponse(responseId)`

Retrieve a response by ID.

```typescript
async retrieveResponse(responseId: string): Promise<ResponseObject>
```

#### `cancelResponse(responseId)`

Cancel an in-progress response.

```typescript
async cancelResponse(responseId: string): Promise<ResponseObject>
```

### Hooks

Register a hook object to observe requests, responses, and errors.

```javascript
client.addHook({
  onRequest(request) {
    console.log("Sending request:", request.model);
  },
  onResponse(request, response) {
    console.log("Tokens used:", response.usage?.totalTokens);
  },
  onError(request, error) {
    console.error("Request failed:", error.message);
  },
});
```

All three callbacks are optional -- provide only the ones you need.

### Provider Management

#### `registerProvider(config)`

Register a custom provider at runtime.

```javascript
client.registerProvider({
  name: "my-provider",
  baseUrl: "https://api.my-provider.com/v1",
  authHeader: "Authorization",
  modelPrefixes: ["my-provider/"],
});
```

#### `unregisterProvider(name)`

Remove a previously registered provider by name.

```javascript
client.unregisterProvider("my-provider");
```

### Budget

#### `budgetUsed`

Read-only property returning the total spend tracked by the budget system (in USD). Returns `0` if no budget is configured.

```javascript
console.log(`Budget used: $${client.budgetUsed}`);
```

## Types

The WASM package ships with full TypeScript type definitions (`.d.ts`). All types use camelCase field names. Key interfaces:

### `ChatCompletionRequest`

```typescript
interface ChatCompletionRequest {
  model: string;
  messages: MessageParam[];
  temperature?: number;
  topP?: number;
  maxTokens?: number;
  tools?: ToolParam[];
  toolChoice?: ToolChoiceParam;
  responseFormat?: ResponseFormatParam;
  // ...
}
```

### `ChatCompletionResponse`

| Field | Type | Description |
|-------|------|-------------|
| `id` | `string` | Response ID |
| `model` | `string` | Model used |
| `choices` | `Choice[]` | Completion choices |
| `usage` | `Usage \| undefined` | Token usage |
| `created` | `number` | Unix timestamp |

### `Choice`

| Field | Type | Description |
|-------|------|-------------|
| `index` | `number` | Choice index |
| `message` | `AssistantMessage` | The assistant's message |
| `finishReason` | `string \| null` | Why generation stopped (`stop`, `length`, `tool_calls`) |

### `AssistantMessage`

| Field | Type | Description |
|-------|------|-------------|
| `content` | `string \| null` | Text content |
| `toolCalls` | `ToolCall[] \| null` | Tool calls made by the assistant |
| `refusal` | `string \| null` | Refusal message |

### `ChatCompletionChunk`

| Field | Type | Description |
|-------|------|-------------|
| `id` | `string` | Response ID |
| `model` | `string` | Model used |
| `choices` | `StreamChoice[]` | Stream choices with deltas |
| `usage` | `Usage \| undefined` | Token usage (final chunk only) |

### `Usage`

| Field | Type | Description |
|-------|------|-------------|
| `promptTokens` | `number` | Tokens consumed by the prompt |
| `completionTokens` | `number` | Tokens consumed by the completion |
| `totalTokens` | `number` | Total tokens |

### `EmbeddingResponse`

| Field | Type | Description |
|-------|------|-------------|
| `data` | `EmbeddingObject[]` | Embedding vectors |
| `model` | `string` | Model used |
| `usage` | `Usage` | Token usage |

### `ModelsListResponse`

| Field | Type | Description |
|-------|------|-------------|
| `data` | `ModelObject[]` | Available models |

## Error Handling

Errors are thrown as JavaScript `Error` objects. The message includes a bracketed label for the error category.

| Error Category | Trigger |
|----------------|---------|
| `[Authentication]` | API key rejected (HTTP 401/403) |
| `[RateLimited]` | Rate limit exceeded (HTTP 429) |
| `[BadRequest]` | Malformed request (HTTP 400) |
| `[ContextWindowExceeded]` | Prompt exceeds context window |
| `[ContentPolicy]` | Content policy violation |
| `[NotFound]` | Model/resource not found (HTTP 404) |
| `[ServerError]` | Provider 5xx error |
| `[ServiceUnavailable]` | Provider temporarily unavailable (HTTP 502/503) |
| `[Timeout]` | Request timed out |
| `[Network]` | Network-level failure |
| `[Streaming]` | Error reading streaming response |
| `[EndpointNotSupported]` | Provider does not support the endpoint |
| `[Serialization]` | JSON serialization/deserialization failure |

```javascript
try {
  const resp = await client.chat({ model: "gpt-4", messages: [...] });
} catch (err) {
  if (err.message.startsWith("[RateLimited]")) {
    // back off and retry
  } else if (err.message.startsWith("[Authentication]")) {
    console.error("Invalid API key");
  } else {
    console.error(err.message);
  }
}
```

## Example

```javascript
import init, { LlmClient } from '@kreuzberg/liter-llm-wasm';

await init();

const client = new LlmClient({ apiKey: 'sk-...' });

// Non-streaming
const response = await client.chat({
  model: 'gpt-4',
  messages: [{ role: 'user', content: 'Hello!' }],
  maxTokens: 256,
});
console.log(response.choices[0].message.content);

// Streaming
const chunks = await client.chatStream({
  model: 'gpt-4',
  messages: [{ role: 'user', content: 'Tell me a joke' }],
});
for (const chunk of chunks) {
  process.stdout.write(chunk.choices[0]?.delta?.content ?? '');
}
```
