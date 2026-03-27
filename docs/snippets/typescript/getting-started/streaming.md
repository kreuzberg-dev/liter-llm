```typescript
import { LlmClient } from "liter-llm";

const client = new LlmClient({ apiKey: process.env.OPENAI_API_KEY! });
const chunks = await client.chatStream({
  model: "openai/gpt-4o",
  messages: [{ role: "user", content: "Tell me a story" }],
});

for (const chunk of chunks) {
  process.stdout.write(chunk.choices[0]?.delta?.content ?? "");
}
console.log();
```
