```typescript
import { LlmClient } from "liter-llm";

const client = new LlmClient({ apiKey: process.env.OPENAI_API_KEY! });
const response = await client.chat({
  model: "openai/gpt-4o",
  messages: [{ role: "user", content: "Hello!" }],
});
console.log(response.choices[0].message.content);
```
