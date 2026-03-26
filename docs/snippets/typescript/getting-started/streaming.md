```typescript
import { LlmClient } from "liter-llm";

const client = new LlmClient();
const stream = client.chatStream({
  model: "openai/gpt-4o",
  messages: [{ role: "user", content: "Tell me a story" }],
});

for await (const chunk of stream) {
  process.stdout.write(chunk.delta ?? "");
}
console.log();
```
