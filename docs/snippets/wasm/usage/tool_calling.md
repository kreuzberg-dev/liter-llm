```typescript
import init, { LlmClient } from "@kreuzberg/liter-llm-wasm";

await init();

const client = new LlmClient({ apiKey: process.env.OPENAI_API_KEY! });

const tools = [
  {
    type: "function" as const,
    function: {
      name: "get_weather",
      description: "Get the current weather for a location",
      parameters: {
        type: "object",
        properties: {
          location: { type: "string", description: "City name" },
        },
        required: ["location"],
      },
    },
  },
];

const response = await client.chat({
  model: "openai/gpt-4o",
  messages: [{ role: "user", content: "What is the weather in Berlin?" }],
  tools,
});

for (const call of response.choices[0]?.message?.toolCalls ?? []) {
  console.log(`Tool: ${call.function.name}, Args: ${call.function.arguments}`);
}
```
