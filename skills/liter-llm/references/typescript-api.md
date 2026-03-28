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
import { LlmClient } from '@kreuzberg/liter-llm';

const client = new LlmClient({
  apiKey: string,
  baseUrl?: string,
  modelHint?: string,
  maxRetries?: number,
  timeoutSecs?: number,
  cache?: CacheOptions,
  budget?: BudgetOptions,
  extraHeaders?: Record<string, string>,
  cooldown?: number,
  rateLimit?: RateLimitOptions,
  healthCheck?: number,
  costTracking?: boolean,
  tracing?: boolean,
});
```

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `apiKey` | `string` | *required* | API key for authentication (wrapped in `SecretString` internally) |
| `baseUrl` | `string \| undefined` | `undefined` | Override provider base URL |
| `modelHint` | `string \| undefined` | `undefined` | Hint for provider auto-detection (e.g. `"groq/llama3-70b"`) |
| `maxRetries` | `number \| undefined` | `3` | Retries on 429 / 5xx responses |
| `timeoutSecs` | `number \| undefined` | `60` | Request timeout in seconds |
| `cache` | `CacheOptions \| undefined` | `undefined` | Cache config |
| `budget` | `BudgetOptions \| undefined` | `undefined` | Budget config |
| `extraHeaders` | `Record<string, string> \| undefined` | `undefined` | Additional HTTP headers sent with every request |
| `cooldown` | `number \| undefined` | `undefined` | Cooldown period in seconds after transient errors |
| `rateLimit` | `RateLimitOptions \| undefined` | `undefined` | Rate limit config |
| `healthCheck` | `number \| undefined` | `undefined` | Health check interval in seconds |
| `costTracking` | `boolean \| undefined` | `undefined` | Enable per-request cost tracking |
| `tracing` | `boolean \| undefined` | `undefined` | Enable OpenTelemetry tracing spans |

### Configuration Types

**CacheOptions:**

| Field | Type | Description |
|-------|------|-------------|
| `maxEntries` | `number` | Maximum cached responses (default: 256) |
| `ttlSeconds` | `number` | Time-to-live for cached entries (default: 300) |

**BudgetOptions:**

| Field | Type | Description |
|-------|------|-------------|
| `globalLimit` | `number` | Maximum USD spend across all models |
| `modelLimits` | `Record<string, number>` | Per-model USD spend limits |
| `enforcement` | `"hard" \| "soft"` | `"hard"` rejects, `"soft"` warns |

**RateLimitOptions:**

| Field | Type | Description |
|-------|------|-------------|
| `rpm` | `number` | Requests per minute |
| `tpm` | `number` | Tokens per minute |

---

## Methods

All methods are async and return Promises. Request and response objects use **camelCase** keys (converted automatically from the snake_case wire format).

### Core Completion Methods

#### `chat(request): Promise<object>`

Send a chat completion request.

```typescript
const resp = await client.chat({
  model: "gpt-4",
  messages: [{ role: "user", content: "Hello!" }],
  maxTokens: 256,
  temperature: 0.7,
});
console.log(resp.choices[0].message.content);
```

Request fields: `model`, `messages`, `temperature`, `topP`, `maxTokens`, `tools`, `toolChoice`, `responseFormat`, `n`, `stop`, `presencePenalty`, `frequencyPenalty`, `user`.

#### `chatStream(request): Promise<object[]>`

Collect all streaming chat completion chunks into an array. The full SSE stream is consumed on the Rust side before the Promise resolves.

```typescript
const chunks = await client.chatStream({
  model: "gpt-4",
  messages: [{ role: "user", content: "Tell me a joke" }],
});
for (const chunk of chunks) {
  process.stdout.write(chunk.choices[0]?.delta?.content ?? "");
}
```

Note: Unlike Python, the Node.js binding collects all chunks before returning. For true streaming, use the chunks array to reconstruct the response.

### Embedding

#### `embed(request): Promise<object>`

Send an embedding request.

```typescript
const resp = await client.embed({
  model: "text-embedding-3-small",
  input: "The quick brown fox",
});
const vector = resp.data[0].embedding; // number[]
```

| Field | Type | Description |
|-------|------|-------------|
| `model` | `string` | Embedding model name |
| `input` | `string \| string[]` | Text(s) to embed |
| `encodingFormat` | `string \| undefined` | `"float"` or `"base64"` |
| `dimensions` | `number \| undefined` | Output dimensions (model-dependent) |
| `user` | `string \| undefined` | End-user identifier |

### Model Discovery

#### `listModels(): Promise<object>`

List available models from the provider.

```typescript
const models = await client.listModels();
for (const m of models.data) {
  console.log(m.id);
}
```

### Image Generation

#### `imageGenerate(request): Promise<object>`

Generate images from a text prompt.

```typescript
const resp = await client.imageGenerate({
  model: "dall-e-3",
  prompt: "A sunset over mountains",
  n: 1,
  size: "1024x1024",
});
```

| Field | Type | Description |
|-------|------|-------------|
| `prompt` | `string` | Text description |
| `model` | `string` | Image generation model |
| `n` | `number \| undefined` | Number of images |
| `size` | `string \| undefined` | Image size (e.g. `"1024x1024"`) |
| `quality` | `string \| undefined` | Quality level |
| `responseFormat` | `string \| undefined` | `"url"` or `"b64Json"` |
| `style` | `string \| undefined` | `"vivid"` or `"natural"` |
| `user` | `string \| undefined` | End-user identifier |

### Audio

#### `speech(request): Promise<Buffer>`

Generate speech audio from text. Returns a `Buffer` of raw audio bytes.

```typescript
const audio = await client.speech({
  model: "tts-1",
  input: "Hello world",
  voice: "alloy",
});
import { writeFileSync } from "node:fs";
writeFileSync("output.mp3", audio);
```

| Field | Type | Description |
|-------|------|-------------|
| `model` | `string` | TTS model |
| `input` | `string` | Text to speak |
| `voice` | `string` | Voice name |
| `responseFormat` | `string \| undefined` | Audio format |
| `speed` | `number \| undefined` | Speed multiplier (0.25 to 4.0) |

#### `transcribe(request): Promise<object>`

Transcribe audio to text.

```typescript
const resp = await client.transcribe({
  model: "whisper-1",
  file: audioBuffer,
});
console.log(resp.text);
```

| Field | Type | Description |
|-------|------|-------------|
| `model` | `string` | Transcription model |
| `file` | `Buffer` | Audio file bytes |
| `language` | `string \| undefined` | ISO-639-1 language code |
| `prompt` | `string \| undefined` | Optional context prompt |
| `responseFormat` | `string \| undefined` | Output format |
| `temperature` | `number \| undefined` | Sampling temperature |

### Content Safety

#### `moderate(request): Promise<object>`

Check content against moderation policies.

```typescript
const resp = await client.moderate({
  input: "Some text to check",
  model: "text-moderation-latest",
});
console.log(resp.results[0].flagged);
```

### Search and Retrieval

#### `rerank(request): Promise<object>`

Rerank documents by relevance to a query.

```typescript
const resp = await client.rerank({
  model: "cohere/rerank-v3.5",
  query: "What is machine learning?",
  documents: ["ML is a subset of AI...", "Cooking recipes..."],
  topN: 5,
});
```

| Field | Type | Description |
|-------|------|-------------|
| `model` | `string` | Reranking model |
| `query` | `string` | Query to rank against |
| `documents` | `string[]` | Documents to rerank |
| `topN` | `number \| undefined` | Number of top results |

#### `search(request): Promise<object>`

Perform a web or document search across supported providers.

```typescript
const resp = await client.search({
  model: "brave/search",
  query: "latest AI news",
  maxResults: 10,
});
```

| Field | Type | Description |
|-------|------|-------------|
| `model` | `string` | Search provider/model |
| `query` | `string` | Search query |
| `maxResults` | `number \| undefined` | Maximum results |
| `searchType` | `string \| undefined` | Search type (provider-specific) |

### OCR

#### `ocr(request): Promise<object>`

Extract text from documents or images using OCR with Markdown output.

```typescript
const resp = await client.ocr({
  model: "mistral/pixtral",
  file: base64Content,
  mimeType: "application/pdf",
});
console.log(resp.text);
```

| Field | Type | Description |
|-------|------|-------------|
| `model` | `string` | OCR model |
| `file` | `string \| Buffer` | File content (base64 or buffer) |
| `mimeType` | `string \| undefined` | MIME type of the file |
| `pages` | `string \| undefined` | Page range (e.g. `"1-5"`) |

---

## File Operations

### `createFile(request): Promise<object>`

Upload a file.

```typescript
const result = await client.createFile({
  file: fileBuffer,
  purpose: "batch",
  filename: "data.jsonl",
});
const fileId = result.id;
```

| Field | Type | Description |
|-------|------|-------------|
| `file` | `Buffer` | File content |
| `purpose` | `string` | File purpose (e.g. `"batch"`, `"fine-tune"`) |
| `filename` | `string \| undefined` | Original filename |

#### `retrieveFile(fileId: string): Promise<object>`

Retrieve metadata for a file by ID.

```typescript
const meta = await client.retrieveFile("file-abc123");
```

#### `deleteFile(fileId: string): Promise<object>`

Delete a file by ID.

```typescript
const result = await client.deleteFile("file-abc123");
```

#### `listFiles(query?: object | null): Promise<object>`

List files, optionally filtered.

```typescript
const files = await client.listFiles({ purpose: "batch", limit: 10 });
```

| Field | Type | Description |
|-------|------|-------------|
| `purpose` | `string \| undefined` | Filter by purpose |
| `limit` | `number \| undefined` | Max results |
| `after` | `string \| undefined` | Cursor for pagination |

#### `fileContent(fileId: string): Promise<Buffer>`

Retrieve the raw content of a file.

```typescript
const content = await client.fileContent("file-abc123");
```

---

## Batch Operations

### `createBatch(request): Promise<object>`

Create a new batch job.

```typescript
const batch = await client.createBatch({
  inputFileId: "file-abc123",
  endpoint: "/v1/chat/completions",
  completionWindow: "24h",
});
```

| Field | Type | Description |
|-------|------|-------------|
| `inputFileId` | `string` | ID of the uploaded JSONL file |
| `endpoint` | `string` | API endpoint for batch requests |
| `completionWindow` | `string` | Time window (e.g. `"24h"`) |
| `metadata` | `Record<string, string> \| undefined` | Optional metadata |

#### `retrieveBatch(batchId: string): Promise<object>`

Retrieve a batch by ID.

```typescript
const batch = await client.retrieveBatch("batch-abc123");
console.log(batch.status);
```

#### `listBatches(query?: object | null): Promise<object>`

List batches, optionally filtered.

```typescript
const batches = await client.listBatches({ limit: 10 });
```

#### `cancelBatch(batchId: string): Promise<object>`

Cancel an in-progress batch.

```typescript
const result = await client.cancelBatch("batch-abc123");
```

---

## Response Operations

### `createResponse(request): Promise<object>`

Create a new response via the Responses API.

```typescript
const resp = await client.createResponse({
  model: "gpt-4",
  input: "Explain quantum computing",
  maxOutputTokens: 1024,
});
```

| Field | Type | Description |
|-------|------|-------------|
| `model` | `string` | Model name |
| `input` | `string` | Input text |
| `instructions` | `string \| undefined` | System instructions |
| `maxOutputTokens` | `number \| undefined` | Max output tokens |
| `temperature` | `number \| undefined` | Sampling temperature |
| `topP` | `number \| undefined` | Nucleus sampling |

#### `retrieveResponse(id: string): Promise<object>`

Retrieve a response by ID.

```typescript
const resp = await client.retrieveResponse("resp-abc123");
```

#### `cancelResponse(id: string): Promise<object>`

Cancel an in-progress response.

```typescript
const result = await client.cancelResponse("resp-abc123");
```

---

## Provider Management

### `registerProvider(config)`

Register a custom provider for self-hosted or unsupported LLM endpoints.

```typescript
client.registerProvider({
  name: "my-provider",
  baseUrl: "https://my-llm.example.com/v1",
  authHeader: "Authorization",
  modelPrefixes: ["my-provider/"],
});
```

After registration, models prefixed with `"my-provider/"` route to the custom endpoint.

#### `unregisterProvider(name: string): boolean`

Remove a previously registered custom provider. Returns `true` if found and removed.

```typescript
const removed = client.unregisterProvider("my-provider");
```

---

## Hooks

### `addHook(hook)`

Register a lifecycle hook for request/response/error events. All callbacks are optional, fire-and-forget, and can be sync or async.

```typescript
client.addHook({
  onRequest(req) { console.log(`Sending: ${req.model}`); },
  onResponse(req, res) { console.log(`Tokens: ${res.usage?.totalTokens}`); },
  onError(req, err) { console.error(`Error: ${err}`); },
});
```

| Callback | Arguments | Description |
|----------|-----------|-------------|
| `onRequest(req)` | request object | Called before each request |
| `onResponse(req, res)` | request, response objects | Called after successful response |
| `onError(req, err)` | request, Error | Called on error |

---

## Budget Tracking

### `budgetUsed` (getter)

Returns the total spend in USD so far (requires `costTracking: true` or `budget` config).

```typescript
console.log(`Budget used: $${client.budgetUsed.toFixed(2)}`);
```

---

## Module Functions

### `version(): string`

Returns the library version string.

```typescript
import { version } from '@kreuzberg/liter-llm';
console.log(version());
```

---

## Types

Response objects are plain JavaScript objects with **camelCase** keys (automatic conversion from snake_case wire format).

### ChatCompletionResponse

| Field | Type | Description |
|-------|------|-------------|
| `id` | `string` | Response ID |
| `model` | `string` | Model used |
| `choices` | `Choice[]` | Completion choices |
| `usage` | `Usage \| undefined` | Token usage |
| `created` | `number` | Unix timestamp |

### Choice

| Field | Type | Description |
|-------|------|-------------|
| `index` | `number` | Choice index |
| `message` | `AssistantMessage` | The assistant's message |
| `finishReason` | `string \| null` | Why generation stopped (`stop`, `length`, `toolCalls`) |

### AssistantMessage

| Field | Type | Description |
|-------|------|-------------|
| `content` | `string \| null` | Text content |
| `toolCalls` | `ToolCall[] \| undefined` | Tool calls made by the assistant |
| `refusal` | `string \| null` | Refusal message |

### ToolCall

| Field | Type | Description |
|-------|------|-------------|
| `id` | `string` | Tool call ID |
| `type` | `string` | Always `"function"` |
| `function` | `FunctionCall` | Function name and arguments |

### FunctionCall

| Field | Type | Description |
|-------|------|-------------|
| `name` | `string` | Function name |
| `arguments` | `string` | JSON-encoded arguments |

### ChatCompletionChunk

Returned as array elements from `chatStream()`.

| Field | Type | Description |
|-------|------|-------------|
| `id` | `string` | Response ID |
| `model` | `string` | Model used |
| `choices` | `StreamChoice[]` | Stream choices with deltas |
| `usage` | `Usage \| undefined` | Token usage (final chunk only) |

### StreamChoice

| Field | Type | Description |
|-------|------|-------------|
| `index` | `number` | Choice index |
| `delta` | `Delta` | Incremental content |
| `finishReason` | `string \| null` | Set on final chunk |

### Delta

| Field | Type | Description |
|-------|------|-------------|
| `content` | `string \| null` | Incremental text |
| `toolCalls` | `ToolCall[] \| undefined` | Incremental tool calls |

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

### EmbeddingObject

| Field | Type | Description |
|-------|------|-------------|
| `index` | `number` | Index in the input list |
| `embedding` | `number[]` | Embedding vector |

### ModelsListResponse

| Field | Type | Description |
|-------|------|-------------|
| `data` | `ModelObject[]` | Available models |

### ModelObject

| Field | Type | Description |
|-------|------|-------------|
| `id` | `string` | Model identifier |
| `ownedBy` | `string` | Model owner |

---

## Error Handling

Errors are thrown as JavaScript `Error` objects. The message includes a bracketed label for the error category.

```typescript
try {
  await client.chat({ model: "gpt-4", messages: [] });
} catch (err) {
  // err.message examples:
  // "[Authentication] Invalid API key"
  // "[RateLimited] Too many requests"
  // "[BadRequest] Messages must not be empty"
  console.error(err.message);
}
```

### Error categories

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

### Parsing error categories

```typescript
try {
  await client.chat(request);
} catch (err) {
  const msg = (err as Error).message;
  if (msg.startsWith("[RateLimited]")) {
    // back off and retry
  } else if (msg.startsWith("[Authentication]")) {
    // check API key
  } else if (msg.startsWith("[ContextWindowExceeded]")) {
    // truncate input
  }
}
```

---

## Usage Examples

### Basic chat

```typescript
import { LlmClient } from '@kreuzberg/liter-llm';

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

### Streaming

```typescript
const chunks = await client.chatStream({
  model: "gpt-4",
  messages: [{ role: "user", content: "Tell me a joke" }],
});
let fullText = "";
for (const chunk of chunks) {
  const content = chunk.choices[0]?.delta?.content ?? "";
  fullText += content;
  process.stdout.write(content);
}
```

### Client with middleware

```typescript
const client = new LlmClient({
  apiKey: process.env.API_KEY!,
  cache: { maxEntries: 256, ttlSeconds: 300 },
  budget: { globalLimit: 10.0, enforcement: "hard" },
  rateLimit: { rpm: 60, tpm: 100000 },
  costTracking: true,
  cooldown: 5,
  healthCheck: 30,
});

// Check spending
console.log(`Spent: $${client.budgetUsed.toFixed(2)}`);
```

### Provider routing

Models are routed by name prefix. No per-request overhead.

```typescript
// OpenAI (default, no prefix needed)
await client.chat({ model: "gpt-4", messages: [...] });

// Groq
await client.chat({ model: "groq/llama3-70b", messages: [...] });

// Anthropic
await client.chat({ model: "anthropic/claude-3-opus", messages: [...] });

// Custom provider
client.registerProvider({
  name: "local",
  baseUrl: "http://localhost:8080/v1",
  authHeader: "Authorization",
  modelPrefixes: ["local/"],
});
await client.chat({ model: "local/my-model", messages: [...] });
```

### Tool calling

```typescript
const resp = await client.chat({
  model: "gpt-4",
  messages: [{ role: "user", content: "What's the weather in Paris?" }],
  tools: [{
    type: "function",
    function: {
      name: "getWeather",
      description: "Get weather for a location",
      parameters: {
        type: "object",
        properties: {
          location: { type: "string" }
        },
        required: ["location"],
      },
    },
  }],
});

const toolCall = resp.choices[0].message.toolCalls?.[0];
if (toolCall) {
  const args = JSON.parse(toolCall.function.arguments);
  console.log(`Tool: ${toolCall.function.name}, Args:`, args);
}
```
