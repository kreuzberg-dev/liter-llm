<!-- snippet:compile-only -->

```typescript
import { LlmClient } from "@kreuzberg/liter-llm";

const client = new LlmClient({ apiKey: process.env.OPENAI_API_KEY! });
const response = await client.createResponse({
  model: "openai/gpt-4o",
  input: "Explain quantum computing in one sentence.",
});
console.log(response);
```
