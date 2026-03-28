<!-- snippet:compile-only -->

```typescript
import init, { LlmClient } from "@kreuzberg/liter-llm-wasm";

await init();

const client = new LlmClient({ apiKey: process.env.OPENAI_API_KEY! });
const response = await client.moderate({
  model: "openai/omni-moderation-latest",
  input: "This is a test message.",
});

const result = response.results[0];
console.log(`Flagged: ${result.flagged}`);
for (const [category, flagged] of Object.entries(result.categories)) {
  if (flagged) {
    console.log(`  ${category}: ${result.categoryScores[category].toFixed(4)}`);
  }
}
```
