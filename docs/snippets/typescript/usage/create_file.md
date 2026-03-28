<!-- snippet:compile-only -->

```typescript
import { LlmClient } from "@kreuzberg/liter-llm";
import { readFileSync } from "node:fs";

const client = new LlmClient({ apiKey: process.env.OPENAI_API_KEY! });
const fileBuffer = readFileSync("data.jsonl");
const response = await client.createFile({
  file: fileBuffer,
  filename: "data.jsonl",
  purpose: "batch",
});
console.log(`File ID: ${response.id}`);
console.log(`Size: ${response.bytes} bytes`);
```
