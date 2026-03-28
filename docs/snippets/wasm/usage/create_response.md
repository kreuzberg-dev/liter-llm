<!-- snippet:compile-only -->

```typescript
import init, { LlmClient } from "@kreuzberg/liter-llm-wasm";

await init();

const client = new LlmClient({ apiKey: "sk-..." });
const response = await client.createResponse({
  model: "openai/gpt-4o",
  input: "Explain quantum computing in one sentence.",
});

console.log(response);
```
