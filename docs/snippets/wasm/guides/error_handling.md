```typescript
import init, { LlmClient } from "@kreuzberg/liter-llm-wasm";

await init();

const client = new LlmClient({ apiKey: process.env.OPENAI_API_KEY! });

try {
  const response = await client.chat({
    model: "openai/gpt-4o",
    messages: [{ role: "user", content: "Hello" }],
  });
  console.log(response.choices[0].message.content);
} catch (err) {
  // The WASM binding rejects with a plain Error whose message is formatted
  // as "HTTP {status}: {message}". Parse the status to branch on category.
  const message = err instanceof Error ? err.message : String(err);
  const match = message.match(/^HTTP (\d+):/);
  const status = match ? Number(match[1]) : null;

  if (status === 429) {
    console.error("rate limited:", message);
  } else if (status === 401 || status === 403) {
    console.error("auth failed:", message);
  } else if (status === 408 || (status !== null && status >= 500)) {
    console.error("transient error, retry with backoff:", message);
  } else if (message.includes("budget exceeded")) {
    console.error("budget exceeded:", message);
  } else {
    console.error("llm error:", message);
  }
}
```
