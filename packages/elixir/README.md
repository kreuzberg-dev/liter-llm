# liter-lm (Elixir)

High-performance LLM client library for Elixir. Unified interface for streaming completions, tool calling, and provider routing across OpenAI, Anthropic, and 50+ LLM providers. Powered by Rust core via Rustler NIF.

## Installation

Add to your `mix.exs` dependencies:

```elixir
def deps do
  [
    {:liter_lm, "~> 1.0.0-rc.1"}
  ]
end
```

## Quick Start

```elixir
defmodule MyApp do
  def chat do
    {:ok, response} = LiterLm.chat(
      model: "openai/gpt-4",
      messages: [
        %{"role" => "user", "content" => "Hello!"}
      ]
    )
    IO.puts(response["content"])
  end
end
```

## Full Documentation

For comprehensive documentation, examples, and API reference, see the [main repository](https://github.com/kreuzberg-dev/liter-lm).
