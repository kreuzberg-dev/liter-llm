```typescript
import init, { LlmClient } from "@kreuzberg/liter-llm-wasm";

await init();

const client = new LlmClient({ apiKey: process.env.OPENAI_API_KEY! });
const messages: Array<{ role: string; content: string }> = [
  { role: "system", content: "You are a helpful assistant." },
  { role: "user", content: "What is the capital of France?" },
];

let response = await client.chat({ model: "openai/gpt-4o", messages });
console.log(`Assistant: ${response.choices[0].message.content}`);

// Continue the conversation
messages.push({ role: "assistant", content: response.choices[0].message.content! });
messages.push({ role: "user", content: "What about Germany?" });

response = await client.chat({ model: "openai/gpt-4o", messages });
console.log(`Assistant: ${response.choices[0].message.content}`);

// Token usage
console.log(`Tokens: ${response.usage?.promptTokens} in, ${response.usage?.completionTokens} out`);
```
