```typescript
import { LlmClient, type Tool } from "liter-llm";

const client = new LlmClient();

const tools: Tool[] = [
  {
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
];

const response = await client.chat({
  model: "openai/gpt-4o",
  messages: [{ role: "user", content: "What is the weather in Berlin?" }],
  tools,
});

for (const call of response.toolCalls ?? []) {
  console.log(`Tool: ${call.name}, Args:`, call.arguments);
}
```
