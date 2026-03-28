<!-- snippet:compile-only -->

```typescript
import { LlmClient } from "@kreuzberg/liter-llm";
import { writeFileSync } from "node:fs";

const client = new LlmClient({ apiKey: process.env.OPENAI_API_KEY! });
const audioBuffer = await client.speech({
  model: "openai/tts-1",
  input: "Hello, world!",
  voice: "alloy",
});
writeFileSync("output.mp3", audioBuffer);
console.log(`Wrote ${audioBuffer.byteLength} bytes to output.mp3`);
```
