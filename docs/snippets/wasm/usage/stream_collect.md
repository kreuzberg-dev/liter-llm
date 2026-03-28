```typescript
import init, { LlmClient } from "@kreuzberg/liter-llm-wasm";

await init();

const client = new LlmClient({ apiKey: process.env.OPENAI_API_KEY! });
const chunks = await client.chatStream({
  model: "openai/gpt-4o",
  messages: [{ role: "user", content: "Explain quantum computing briefly" }],
});

let fullText = "";
for (const chunk of chunks) {
  const delta = chunk.choices?.[0]?.delta?.content;
  if (delta) {
    fullText += delta;
    process.stdout.write(delta);
  }
}
console.log();
console.log(`\nFull response length: ${fullText.length} characters`);
```
