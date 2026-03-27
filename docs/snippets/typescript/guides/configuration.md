```typescript
import { LlmClient } from "liter-llm";

const client = new LlmClient({
  apiKey: "sk-...",           // or set OPENAI_API_KEY env var
  baseUrl: undefined,         // override provider base URL
  modelHint: "openai",        // pre-resolve provider at construction
  maxRetries: 3,              // retry on transient failures
  timeoutSecs: 60,            // request timeout in seconds
});

const response = await client.chat({
  model: "openai/gpt-4o",
  messages: [{ role: "user", content: "Hello!" }],
});
console.log(response.choices[0].message.content);
```
