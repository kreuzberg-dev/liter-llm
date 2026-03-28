---
name: liter-llm
description: >-
  Universal LLM API client for 142+ providers with native bindings for
  11 languages. Use when writing code that calls LLM APIs via liter-llm
  in Python, TypeScript, Rust, Go, Java, C#, Ruby, PHP, Elixir, WASM, or C.
  Covers chat, streaming, embeddings, image generation, speech, transcription,
  moderation, reranking, search, OCR, tool calling, and configuration.
license: MIT
metadata:
  author: kreuzberg-dev
  version: "1.0"
  repository: https://github.com/kreuzberg-dev/liter-llm
---

# Liter-LLM Universal LLM Client

Liter-LLM is a universal LLM API client with a Rust core and native bindings for Python, TypeScript/Node.js, Go, Java, C#, Ruby, PHP, Elixir, WebAssembly, and C (FFI). It provides a unified interface to 142+ LLM providers (OpenAI, Anthropic, Google Gemini, Groq, Mistral, Cohere, AWS Bedrock, Azure, and many more) with built-in caching, budgets, rate limiting, hooks, streaming, cost tracking, health checks, and tracing.

Use this skill when writing code that:

- Calls LLM APIs (chat completions, streaming, embeddings) via liter-llm
- Configures liter-llm clients (API keys, timeouts, retries, cache, budget, hooks)
- Uses tool calling / function calling with LLM providers
- Implements streaming responses from LLMs
- Uses search, OCR, image generation, speech, transcription, moderation, or reranking APIs
- Routes requests to specific providers using model prefixes
- Handles LLM API errors across any of the 11 supported languages

## Installation

### Python

```bash
pip install liter-llm
```

### TypeScript / Node.js

```bash
pnpm add @kreuzberg/liter-llm
```

### Rust

```toml
# Cargo.toml
[dependencies]
liter-llm = "0.1"
```

### Go

```bash
go get github.com/kreuzberg-dev/liter-llm/packages/go
```

### Java

```xml
<!-- pom.xml -->
<dependency>
    <groupId>dev.kreuzberg</groupId>
    <artifactId>liter-llm</artifactId>
    <version>0.1.0</version>
</dependency>
```

### C# (.NET)

```bash
dotnet add package Kreuzberg.LiterLlm
```

### Ruby

```bash
gem install liter_llm
```

### PHP

```bash
composer require kreuzberg/liter-llm
```

### Elixir

```elixir
# mix.exs
{:liter_llm, "~> 0.1"}
```

### WebAssembly

```bash
pnpm add @kreuzberg/liter-llm-wasm
```

### C / FFI

```bash
cargo build --release -p liter-llm-ffi
# Produces libliter_llm_ffi.so / .dylib / .dll
```

## Quick Start

### Python (Async)

```python
import asyncio
import os
from liter_llm import LlmClient

async def main() -> None:
    client = LlmClient(api_key=os.environ["OPENAI_API_KEY"])
    response = await client.chat(
        model="openai/gpt-4o",
        messages=[{"role": "user", "content": "Hello!"}],
    )
    print(response.choices[0].message.content)

asyncio.run(main())
```

### TypeScript

```typescript
import { LlmClient } from "@kreuzberg/liter-llm";

const client = new LlmClient({ apiKey: process.env.OPENAI_API_KEY! });
const response = await client.chat({
  model: "openai/gpt-4o",
  messages: [{ role: "user", content: "Hello!" }],
});
console.log(response.choices[0].message.content);
```

### Rust

```rust
use liter_llm::{
    ChatCompletionRequest, ClientConfigBuilder, DefaultClient, LlmClient,
    Message, UserContent, UserMessage,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = ClientConfigBuilder::new(std::env::var("OPENAI_API_KEY")?)
        .build();
    let client = DefaultClient::new(config, Some("openai/gpt-4o"))?;

    let request = ChatCompletionRequest {
        model: "openai/gpt-4o".into(),
        messages: vec![Message::User(UserMessage {
            content: UserContent::Text("Hello!".into()),
            name: None,
        })],
        ..Default::default()
    };

    let response = client.chat(request).await?;
    if let Some(choice) = response.choices.first() {
        println!("{}", choice.message.content.as_deref().unwrap_or(""));
    }
    Ok(())
}
```

### Go

```go
package main

import (
 "context"
 "fmt"
 "os"

 llm "github.com/kreuzberg-dev/liter-llm/packages/go"
)

func main() {
 client := llm.NewClient(llm.WithAPIKey(os.Getenv("OPENAI_API_KEY")))
 resp, err := client.Chat(context.Background(), &llm.ChatCompletionRequest{
  Model: "openai/gpt-4o",
  Messages: []llm.Message{
   llm.NewTextMessage(llm.RoleUser, "Hello!"),
  },
 })
 if err != nil {
  panic(err)
 }
 if len(resp.Choices) > 0 && resp.Choices[0].Message.Content != nil {
  fmt.Println(*resp.Choices[0].Message.Content)
 }
}
```

## Configuration

All languages use the same configuration structure with language-appropriate naming conventions (snake_case for Python/Rust/Ruby/Go/Elixir/PHP, camelCase for TypeScript/Node.js/WASM/C#/Java).

### Python

```python
from liter_llm import LlmClient

client = LlmClient(
    api_key="sk-...",
    base_url="https://custom-proxy.example.com/v1",  # override provider URL
    model_hint="openai",            # pre-resolve provider (skip prefix lookup)
    max_retries=3,                  # retries on 429/5xx with exponential backoff
    timeout=60,                     # request timeout in seconds
    cache={"max_entries": 256, "ttl_seconds": 300},
    budget={"global_limit": 10.0, "model_limits": {"openai/gpt-4o": 5.0}, "enforcement": "hard"},
    cooldown=30,                    # circuit breaker after transient errors
    rate_limit={"rpm": 60, "tpm": 100000},
    health_check=60,                # background provider health checks
    cost_tracking=True,             # per-request cost tracking
    tracing=True,                   # OpenTelemetry tracing spans
)
client.add_hook(MyLoggingHook())
```

### TypeScript

```typescript
import { LlmClient } from "@kreuzberg/liter-llm";

const client = new LlmClient({
  apiKey: process.env.OPENAI_API_KEY!,
  baseUrl: "https://custom-proxy.example.com/v1",
  modelHint: "openai",
  maxRetries: 3,
  timeout: 60,
  cache: { maxEntries: 256, ttlSeconds: 300 },
  budget: { globalLimit: 10.0, modelLimits: { "openai/gpt-4o": 5.0 }, enforcement: "hard" },
  cooldown: 30,
  rateLimit: { rpm: 60, tpm: 100000 },
  healthCheck: 60,
  costTracking: true,
  tracing: true,
});
```

### Configuration Options

| Option | Type | Default | Description |
| --- | --- | --- | --- |
| `api_key` | string | **required** | Provider API key. Wrapped in `SecretString` internally. |
| `base_url` | string | from registry | Override the provider's base URL. |
| `model_hint` | string | none | Pre-resolve a provider at construction (e.g. `"openai"`). |
| `timeout` | duration | 60s | Request timeout. |
| `max_retries` | int | 3 | Retries on 429/5xx responses with exponential backoff. |
| `cache` | object | none | Response caching config (`max_entries`, `ttl_seconds`). |
| `budget` | object | none | Spending limits (`global_limit`, `model_limits`, `enforcement`). |
| `cooldown` | int | none | Circuit breaker cooldown in seconds after transient errors. |
| `rate_limit` | object | none | Rate limiting (`rpm`, `tpm`). |
| `health_check` | int | none | Background health check interval in seconds. |
| `cost_tracking` | bool | false | Enable per-request cost tracking. |
| `tracing` | bool | false | Enable OpenTelemetry tracing spans. |

### API Key Environment Variables

| Provider | Environment Variable |
| --- | --- |
| OpenAI | `OPENAI_API_KEY` |
| Anthropic | `ANTHROPIC_API_KEY` |
| Google (Gemini) | `GEMINI_API_KEY` |
| Groq | `GROQ_API_KEY` |
| Mistral | `MISTRAL_API_KEY` |
| Cohere | `CO_API_KEY` |
| AWS Bedrock | `AWS_ACCESS_KEY_ID` + `AWS_SECRET_ACCESS_KEY` |

## Provider Routing

Model routing uses a name prefix convention. The prefix before the `/` determines which provider handles the request:

```python
# OpenAI
response = await client.chat(model="openai/gpt-4o", messages=[...])

# Anthropic
response = await client.chat(model="anthropic/claude-sonnet-4-20250514", messages=[...])

# Google Gemini
response = await client.chat(model="google/gemini-2.0-flash", messages=[...])

# Groq
response = await client.chat(model="groq/llama3-70b", messages=[...])

# Mistral
response = await client.chat(model="mistral/mistral-large-latest", messages=[...])

# Azure OpenAI
response = await client.chat(model="azure/gpt-4o", messages=[...])

# AWS Bedrock
response = await client.chat(model="bedrock/anthropic.claude-v2", messages=[...])
```

With `model_hint`, you can skip the prefix:

```python
client = LlmClient(api_key="sk-...", model_hint="openai")
response = await client.chat(model="gpt-4o", messages=[...])  # routes to OpenAI
```

## Streaming

### Python

```python
import asyncio
import os
from liter_llm import LlmClient

async def main() -> None:
    client = LlmClient(api_key=os.environ["OPENAI_API_KEY"])
    async for chunk in await client.chat_stream(
        model="openai/gpt-4o",
        messages=[{"role": "user", "content": "Tell me a story"}],
    ):
        if chunk.choices and chunk.choices[0].delta.content:
            print(chunk.choices[0].delta.content, end="", flush=True)
    print()

asyncio.run(main())
```

### TypeScript

```typescript
import { LlmClient } from "@kreuzberg/liter-llm";

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

## Tool Calling

### Python

```python
import asyncio
import os
from liter_llm import LlmClient

async def main() -> None:
    client = LlmClient(api_key=os.environ["OPENAI_API_KEY"])

    tools = [
        {
            "type": "function",
            "function": {
                "name": "get_weather",
                "description": "Get the current weather for a location",
                "parameters": {
                    "type": "object",
                    "properties": {
                        "location": {"type": "string", "description": "City name"},
                    },
                    "required": ["location"],
                },
            },
        }
    ]

    response = await client.chat(
        model="openai/gpt-4o",
        messages=[{"role": "user", "content": "What is the weather in Berlin?"}],
        tools=tools,
    )

    choice = response.choices[0]
    if choice.message.tool_calls:
        for call in choice.message.tool_calls:
            print(f"Tool: {call.function.name}, Args: {call.function.arguments}")

asyncio.run(main())
```

## Search and OCR

### Search

```python
import asyncio
import os
from liter_llm import LlmClient

async def main() -> None:
    client = LlmClient(api_key=os.environ["BRAVE_API_KEY"])
    response = await client.search(
        model="brave/web-search",
        query="What is Rust programming language?",
        max_results=5,
    )
    for result in response.results:
        print(f"{result.title}: {result.url}")

asyncio.run(main())
```

### OCR

```python
import asyncio
import os
from liter_llm import LlmClient

async def main() -> None:
    client = LlmClient(api_key=os.environ["MISTRAL_API_KEY"])
    response = await client.ocr(
        model="mistral/mistral-ocr-latest",
        document={"type": "document_url", "url": "https://example.com/invoice.pdf"},
    )
    for page in response.pages:
        print(f"Page {page.index}: {page.markdown[:100]}...")

asyncio.run(main())
```

## Error Handling

### Python

```python
from liter_llm import LlmClient, LlmError

try:
    response = await client.chat(model="openai/gpt-4o", messages=[...])
except LlmError as e:
    # All liter-llm errors inherit from LlmError
    # Specific variants: AuthenticationError, RateLimitedError,
    # BadRequestError, ContextWindowExceededError, ContentPolicyError,
    # NotFoundError, ServerError, ServiceUnavailableError, LlmTimeoutError,
    # BudgetExceededError
    print(f"LLM error: {e}")
```

### TypeScript

```typescript
import { LlmClient } from "@kreuzberg/liter-llm";

try {
  const response = await client.chat({ model: "openai/gpt-4o", messages: [...] });
} catch (err) {
  // Errors are plain Error objects with bracketed category in the message:
  // "[Authentication] Invalid API key"
  // "[RateLimited] Too many requests"
  // "[BadRequest] Messages must not be empty"
  const msg = (err as Error).message;
  if (msg.startsWith("[RateLimited]")) {
    // back off and retry
  } else if (msg.startsWith("[Authentication]")) {
    // check API key
  }
  console.error(msg);
}
```

### Rust

```rust
use liter_llm::{LlmClient, LiterLlmError};

match client.chat(request).await {
    Ok(response) => println!("{}", response.choices[0].message.content.as_deref().unwrap_or("")),
    Err(LiterLlmError::Authentication { message }) => eprintln!("Auth failed: {message}"),
    Err(LiterLlmError::RateLimited { message, retry_after }) => {
        eprintln!("Rate limited: {message}, retry after: {retry_after:?}");
    }
    Err(LiterLlmError::BadRequest { message }) => eprintln!("Bad request: {message}"),
    Err(LiterLlmError::ContextWindowExceeded { message }) => eprintln!("Too long: {message}"),
    Err(LiterLlmError::Timeout) => eprintln!("Request timed out"),
    Err(e) => eprintln!("Error: {e}"),
}
```

## Hooks

Register lifecycle hooks for request/response/error events:

### Python

```python
from liter_llm import LlmClient

class LoggingHook:
    def on_request(self, request):
        print(f"Sending request to {request['model']}")

    def on_response(self, request, response):
        print(f"Got response: {response.usage.total_tokens} tokens")

    def on_error(self, request, error):
        print(f"Error: {error}")

client = LlmClient(api_key="sk-...")
client.add_hook(LoggingHook())
```

### TypeScript

```typescript
import { LlmClient } from "@kreuzberg/liter-llm";

const client = new LlmClient({ apiKey: process.env.OPENAI_API_KEY! });
client.addHook({
  onRequest(req) { console.log(`Sending: ${req.model}`); },
  onResponse(req, res) { console.log(`Tokens: ${res.usage?.totalTokens}`); },
  onError(req, err) { console.error(`Error: ${err}`); },
});
```

## Common Pitfalls

1. **Python: all methods are async.** You must use `await` and run inside an async context. Use `asyncio.run(main())` at the top level. There are no synchronous methods.

2. **Naming conventions differ by language.** TypeScript, Node.js, WASM, Java, and C# use camelCase (`chatStream`, `apiKey`, `maxRetries`). Python, Rust, Ruby, Go, Elixir, and PHP use snake_case (`chat_stream`, `api_key`, `max_retries`).

3. **Provider prefix is required.** Always use `"provider/model-name"` format (e.g. `"openai/gpt-4o"`, `"anthropic/claude-sonnet-4-20250514"`). Without the prefix, routing will fail unless `model_hint` is set.

4. **API keys are wrapped in SecretString.** Keys passed to the constructor are never logged, serialized, or included in error messages. Read keys from environment variables, never hardcode them.

5. **Streaming: first/last chunks may have null content.** Always check `chunk.choices[0].delta.content` (Python) or `chunk.choices[0]?.delta?.content` (TypeScript) for null/undefined before using the value.

6. **Rust: `DefaultClient::new` requires `ClientConfigBuilder`.** Build config with `ClientConfigBuilder::new(api_key).build()`, then pass to `DefaultClient::new(config, model_hint)`.

7. **Rust: chat is async.** Use `#[tokio::main]` or call from an async context. The `LlmClient` trait defines `async fn chat(...)`.

8. **Budget enforcement modes.** `"hard"` rejects requests that exceed the budget. `"soft"` logs a warning but allows the request through. Default is no budget enforcement.

9. **Cache is per-client.** Each `LlmClient` instance has its own cache. Cache keys are derived from the full request (model + messages + parameters).

10. **Go: check error returns and nil pointers.** Response fields like `Content` are pointers -- always nil-check before dereferencing.

## Additional Resources

Detailed reference files for specific topics:

- **[Python API Reference](references/python-api.md)** -- All functions, config, types, error hierarchy
- **[TypeScript API Reference](references/typescript-api.md)** -- All functions with camelCase conventions
- **[Rust API Reference](references/rust-api.md)** -- Traits, Tower middleware, feature flags
- **[Other Language Bindings](references/other-bindings.md)** -- Go, Java, C#, Ruby, PHP, Elixir, WASM, C FFI
- **[Configuration Reference](references/configuration.md)** -- All config options, cache backends, middleware
- **[Provider Reference](references/providers.md)** -- Routing, auth types, custom providers
- **[Advanced Features](references/advanced-features.md)** -- Search, OCR, OpenDAL cache, Tower stack, tracing

Full documentation: <https://docs.liter-llm.kreuzberg.dev>
GitHub: <https://github.com/kreuzberg-dev/liter-llm>
