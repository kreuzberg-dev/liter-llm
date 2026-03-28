<!-- snippet:compile-only -->

```typescript
import { LlmClient } from "@kreuzberg/liter-llm";
import { readFileSync } from "node:fs";

const client = new LlmClient({ apiKey: process.env.OPENAI_API_KEY! });
const audioBuffer = readFileSync("audio.mp3");
const response = await client.transcribe({
  model: "openai/whisper-1",
  file: audioBuffer,
  filename: "audio.mp3",
});
console.log(response.text);
```
