```typescript
import { LlmClient } from "liter-llm";

const client = new LlmClient({ apiKey: process.env.OPENAI_API_KEY! });

try {
  const response = await client.chat({
    model: "openai/gpt-4o",
    messages: [{ role: "user", content: "Hello" }],
  });
  console.log(response.choices[0].message.content);
} catch (err) {
  // All liter-llm errors surface as JavaScript Error objects. The message
  // carries a bracketed category label: "[RateLimited] Too many requests".
  if (err instanceof Error) {
    if (err.message.startsWith("[Authentication]")) {
      // 401/403 – rotate the key.
      console.error("auth failed:", err.message);
    } else if (err.message.startsWith("[RateLimited]")) {
      // 429 – transient, retry or fall back.
      console.error("rate limited:", err.message);
    } else if (err.message.startsWith("[BudgetExceeded]")) {
      console.error("budget exceeded:", err.message);
    } else {
      console.error("llm error:", err.message);
    }
  }
}
```
