<!-- snippet:compile-only -->

```typescript
import init, { LlmClient } from "@kreuzberg/liter-llm-wasm";

await init();

const client = new LlmClient({ apiKey: process.env.OPENAI_API_KEY! });
const response = await client.imageGenerate({
  model: "openai/dall-e-3",
  prompt: "A sunset over mountains",
  n: 1,
  size: "1024x1024",
});
console.log(response.data[0].url);
```
