# liter-lm (TypeScript/Node.js)

High-performance LLM client library for TypeScript and Node.js. Unified interface for streaming completions, tool calling, and provider routing across OpenAI, Anthropic, and 50+ LLM providers. Powered by Rust core.

## Installation

```bash
npm install liter-lm
```

Or with `pnpm`:

```bash
pnpm add liter-lm
```

## Quick Start

```typescript
import { LlmClient } from "liter-lm";

const client = new LlmClient();
const response = await client.chat({
  model: "openai/gpt-4",
  messages: [{ role: "user", content: "Hello!" }],
});
console.log(response.content);
```

## Full Documentation

For comprehensive documentation, examples, and API reference, see the [main repository](https://github.com/kreuzberg-dev/liter-lm).
