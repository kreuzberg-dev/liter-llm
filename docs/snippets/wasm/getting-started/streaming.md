```typescript
import init, { LlmClient } from "liter-llm-wasm";

await init();

// Note: chatStream is not yet supported in the WASM binding.
// Use the non-streaming chat method instead.
const client = new LlmClient({ apiKey: "sk-..." });
const response = await client.chat({
  model: "openai/gpt-4o",
  messages: [{ role: "user", content: "Tell me a story" }],
});

console.log(response.choices[0].message.content);
```
