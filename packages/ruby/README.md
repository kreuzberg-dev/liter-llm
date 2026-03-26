# liter-lm (Ruby)

High-performance LLM client library for Ruby. Unified interface for streaming completions, tool calling, and provider routing across OpenAI, Anthropic, and 50+ LLM providers. Powered by Rust core.

## Installation

```bash
gem install liter_lm
```

Or add to your `Gemfile`:

```ruby
gem 'liter_lm', '~> 1.0.0.rc1'
```

## Quick Start

```ruby
require 'liter_lm'

client = LiterLm::Client.new
response = client.chat(
  model: "openai/gpt-4",
  messages: [{ role: "user", content: "Hello!" }]
)
puts response.content
```

## Full Documentation

For comprehensive documentation, examples, and API reference, see the [main repository](https://github.com/kreuzberg-dev/liter-lm).
