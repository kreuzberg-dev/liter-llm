<!-- snippet:compile-only -->

```typescript
import init, { LlmClient } from "@kreuzberg/liter-llm-wasm";

await init();

const client = new LlmClient({ apiKey: process.env.OPENAI_API_KEY! });
const response = await client.rerank({
  model: "cohere/rerank-v3.5",
  query: "What is the capital of France?",
  documents: [
    "Paris is the capital of France.",
    "Berlin is the capital of Germany.",
    "London is the capital of England.",
  ],
});

for (const result of response.results) {
  console.log(`Index: ${result.index}, Score: ${result.relevanceScore.toFixed(4)}`);
}
```
