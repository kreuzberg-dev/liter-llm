<!-- snippet:compile-only -->

```typescript
import init, { LlmClient } from "@kreuzberg/liter-llm-wasm";

await init();

const client = new LlmClient({ apiKey: process.env.OPENAI_API_KEY! });
const audioBuffer = new Uint8Array(/* audio file bytes */);
const response = await client.transcribe({
  model: "openai/whisper-1",
  file: audioBuffer,
  filename: "audio.mp3",
});
console.log(response.text);
```
