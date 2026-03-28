<!-- snippet:compile-only -->

```typescript
import { LlmClient } from "@kreuzberg/liter-llm";

const client = new LlmClient({ apiKey: process.env.BRAVE_API_KEY! });
const response = await client.search({
  model: "brave/web-search",
  query: "What is Rust programming language?",
  maxResults: 5,
});

for (const result of response.results) {
  console.log(`${result.title}: ${result.url}`);
}
```
