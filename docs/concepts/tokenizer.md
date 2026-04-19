---
description: "Token counting via HuggingFace tokenizers in liter-llm , covering API, model mapping, and caching."
---

# Tokenizer

The `tokenizer` feature flag adds two public functions for counting tokens before a request is sent: `count_tokens` for raw text and `count_request_tokens` for a full chat completion request.

Tokenizers are loaded from HuggingFace Hub on first use and cached in a process-global `RwLock<HashMap>` for the process lifetime. Subsequent calls to the same model family reuse the cached tokenizer with only a read-lock.

## Enabling

```toml
[dependencies]
liter-llm = { version = "...", features = ["tokenizer"] }
```

## API

### `count_tokens`

Count tokens in a plain string using the tokenizer for a given model name.

```rust
use liter_llm::tokenizer;

let n = tokenizer::count_tokens("gpt-4o", "Hello, world!")?;
// n ≈ 4
```

### `count_request_tokens`

Count tokens across all messages in a `ChatCompletionRequest`. Adds 4 tokens per message as overhead for role tag, separators, and formatting metadata. This matches the OpenAI tokenization overhead estimate. Multimodal content parts (images, audio, documents) are not counted; only text content contributes.

```rust
use liter_llm::{tokenizer, types::{ChatCompletionRequest, Message, SystemMessage, UserMessage, UserContent}};

let req = ChatCompletionRequest {
    model: "gpt-4o".to_owned(),
    messages: vec![
        Message::System(SystemMessage { content: "You are helpful.".into(), name: None }),
        Message::User(UserMessage {
            content: UserContent::Text("What is 2+2?".into()),
            name: None,
        }),
    ],
    ..Default::default()
};

let n = tokenizer::count_request_tokens("gpt-4o", &req)?;
// n = text_tokens + (2 messages × 4 overhead tokens)
```

## Model-to-tokenizer mapping

liter-llm maps model name prefixes to HuggingFace tokenizer repository IDs. When no prefix matches, the GPT-4o tokenizer is used as a reasonable approximation for modern LLMs.

| Model prefix | HuggingFace tokenizer | Notes |
|---|---|---|
| `gpt-4`, `gpt-3.5`, `chatgpt`, `o1`, `o3`, `o4` | `Xenova/gpt-4o` | Covers all OpenAI chat and reasoning models |
| `claude`, `anthropic` | `Xenova/claude-tokenizer` | Anthropic models |
| `gemini`, `vertex_ai` | `google/gemma-2b` | Google / Vertex AI models |
| `mistral`, `codestral` | `mistralai/Mistral-7B-v0.1` | Mistral models |
| `command`, `cohere` | `Cohere/command-r-plus-tokenizer` | Cohere models |
| `llama`, `meta-llama` | `meta-llama/Meta-Llama-3-8B` | Meta Llama models |
| (all others) | `Xenova/gpt-4o` | Fallback; approximation only |

These counts are estimates. Every provider tokenizes slightly differently; the numbers are useful for pre-flight checks (avoiding obvious context-window overflows) but not for exact billing.

## Caching behavior

The first call for a model family downloads the tokenizer from HuggingFace Hub and stores it in a process-global cache. This requires network access and takes roughly 100-500 ms depending on model size and connection speed. All subsequent calls use the cached tokenizer.

The cache uses a two-phase locking strategy to avoid redundant downloads under concurrent access:

1. Read lock: check if the tokenizer is already cached.
2. On miss, write lock: double-check (another task may have raced), then download and insert.

Poisoned locks return `LiterLlmError::BadRequest`. Tokenizer download failures (network errors, missing repository) also return `LiterLlmError::BadRequest` with the HuggingFace error message.

## Use cases

Token counting is useful in two scenarios:

1. **Pre-flight context-window checks.** Count tokens before sending to avoid a `ContextWindowExceeded` error and trim the conversation history if needed.
2. **Budget estimation.** Combine with [Cost Estimation](./cost-estimation.md) to project cost before committing to a request.
