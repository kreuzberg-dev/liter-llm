# liter-lm (Go)

High-performance LLM client library for Go. Unified interface for streaming completions, tool calling, and provider routing across OpenAI, Anthropic, and 50+ LLM providers. Powered by Rust core.

## Installation

```bash
go get github.com/kreuzberg-dev/liter-lm/go
```

## Quick Start

```go
package main

import (
 "context"
 llm "github.com/kreuzberg-dev/liter-lm/go"
)

func main() {
 client := llm.NewClient()
 resp, _ := client.Chat(context.Background(), &llm.ChatRequest{
  Model: "openai/gpt-4",
  Messages: []llm.Message{
   {Role: "user", Content: "Hello!"},
  },
 })
 println(resp.Content)
}
```

## Full Documentation

For comprehensive documentation, examples, and API reference, see the [main repository](https://github.com/kreuzberg-dev/liter-lm).
