---
description: "liter-llm WebAssembly API reference"
---

# WebAssembly API Reference

The WASM package exposes a JavaScript-friendly `LlmClient` class via `wasm-bindgen`. It works in both browser and Node.js environments, using the native `fetch` API for HTTP.

## Installation

```bash
npm install liter-llm-wasm
```

## Setup

```javascript
import init, { LlmClient } from 'liter-llm-wasm';

await init(); // Initialize the WASM module
```

## Client

### Constructor

```typescript
const client = new LlmClient({
  apiKey: string,
  baseUrl?: string,
  maxRetries?: number,     // default: 3
  timeoutSecs?: number,    // default: 60
  authHeader?: string,     // override full Authorization header value
});
```

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `apiKey` | `string` | *required* | API key (empty string for no-auth providers) |
| `baseUrl` | `string?` | `undefined` | Override provider base URL |
| `maxRetries` | `number?` | `3` | Retries on 429/5xx |
| `timeoutSecs` | `number?` | `60` | Request timeout in seconds |
| `authHeader` | `string?` | `undefined` | Override `Authorization` header value |

### Methods

All methods are async and return Promises.

#### `chat(request)`

Send a chat completion request.

```typescript
async chat(request: ChatCompletionRequest): Promise<ChatCompletionResponse>
```

#### `chatStream(request)`

Collect all streaming chat completion chunks.

```typescript
async chatStream(request: ChatCompletionRequest): Promise<ChatCompletionChunk[]>
```

#### `embed(request)`

Send an embedding request.

```typescript
async embed(request: EmbeddingRequest): Promise<EmbeddingResponse>
```

#### `listModels()`

List available models.

```typescript
async listModels(): Promise<ModelsListResponse>
```

#### `imageGenerate(request)`

Generate an image from a text prompt.

```typescript
async imageGenerate(request: CreateImageRequest): Promise<ImagesResponse>
```

#### `speech(request)`

Generate speech audio from text. Returns a `Uint8Array`.

```typescript
async speech(request: CreateSpeechRequest): Promise<Uint8Array>
```

#### `transcribe(request)`

Transcribe audio to text.

```typescript
async transcribe(request: CreateTranscriptionRequest): Promise<TranscriptionResponse>
```

#### `moderate(request)`

Check content against moderation policies.

```typescript
async moderate(request: ModerationRequest): Promise<ModerationResponse>
```

#### `rerank(request)`

Rerank documents by relevance.

```typescript
async rerank(request: RerankRequest): Promise<RerankResponse>
```

#### File, Batch, and Response Management

```typescript
async createFile(request: CreateFileRequest): Promise<FileObject>
async retrieveFile(fileId: string): Promise<FileObject>
async deleteFile(fileId: string): Promise<DeleteResponse>
async listFiles(query?: FileListQuery): Promise<FileListResponse>
async fileContent(fileId: string): Promise<Uint8Array>
async createBatch(request: CreateBatchRequest): Promise<BatchObject>
async retrieveBatch(batchId: string): Promise<BatchObject>
async listBatches(query?: BatchListQuery): Promise<BatchListResponse>
async cancelBatch(batchId: string): Promise<BatchObject>
async createResponse(request: CreateResponseRequest): Promise<ResponseObject>
async retrieveResponse(responseId: string): Promise<ResponseObject>
async cancelResponse(responseId: string): Promise<ResponseObject>
```

## Types

The WASM package ships with full TypeScript type definitions (`.d.ts`). All types use snake_case field names matching the wire format. Key interfaces:

### `ChatCompletionRequest`

```typescript
interface ChatCompletionRequest {
  model: string;
  messages: MessageParam[];
  temperature?: number;
  top_p?: number;
  max_tokens?: number;
  tools?: ToolParam[];
  tool_choice?: ToolChoiceParam;
  response_format?: ResponseFormatParam;
  // ...
}
```

### `ChatCompletionResponse`

```typescript
interface ChatCompletionResponse {
  id: string;
  object: string;
  created: number;
  model: string;
  choices: Choice[];
  usage?: UsageResponse;
}
```

### `ChatCompletionChunk`

```typescript
interface ChatCompletionChunk {
  id: string;
  object: string;
  created: number;
  model: string;
  choices: StreamChoice[];
  usage?: UsageResponse;
}
```

## Error Handling

Errors are thrown as JavaScript `Error` objects with descriptive messages from the Rust core.

```javascript
try {
  const resp = await client.chat({ model: "gpt-4", messages: [...] });
} catch (err) {
  console.error(err.message);
}
```

## Example

```javascript
import init, { LlmClient } from 'liter-llm-wasm';

await init();

const client = new LlmClient({ apiKey: 'sk-...' });

const response = await client.chat({
  model: 'gpt-4',
  messages: [{ role: 'user', content: 'Hello!' }],
  max_tokens: 256,
});

console.log(response.choices[0].message.content);
```
