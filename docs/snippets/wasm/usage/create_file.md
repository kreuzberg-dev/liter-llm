<!-- snippet:compile-only -->

```typescript
import init, { LlmClient } from "@kreuzberg/liter-llm-wasm";

await init();

const client = new LlmClient({ apiKey: process.env.OPENAI_API_KEY! });
const fileBuffer = new Uint8Array(/* file bytes */);
const response = await client.createFile({
  file: fileBuffer,
  filename: "data.jsonl",
  purpose: "batch",
});
console.log(`File ID: ${response.id}`);
console.log(`Size: ${response.bytes} bytes`);
```
