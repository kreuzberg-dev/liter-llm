<!-- snippet:compile-only -->

```typescript
import init, { LlmClient } from "@kreuzberg/liter-llm-wasm";

await init();

const client = new LlmClient({ apiKey: process.env.OPENAI_API_KEY! });
const audioBuffer = await client.speech({
  model: "openai/tts-1",
  input: "Hello, world!",
  voice: "alloy",
});
console.log(`Generated ${audioBuffer.byteLength} bytes of audio`);
```
