# liter-lm (Python)

High-performance LLM client library for Python. Unified interface for streaming completions, tool calling, and provider routing across OpenAI, Anthropic, and 50+ LLM providers. Powered by Rust core.

## Installation

```bash
pip install liter-lm
```

Or with `uv`:

```bash
uv add liter-lm
```

## Quick Start

```python
from liter_lm import LlmClient

client = LlmClient()
response = await client.chat(
    model="openai/gpt-4",
    messages=[{"role": "user", "content": "Hello!"}]
)
print(response.content)
```

## Full Documentation

For comprehensive documentation, examples, and API reference, see the [main repository](https://github.com/kreuzberg-dev/liter-lm).
