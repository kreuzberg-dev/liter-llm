# liter-lm (PHP)

High-performance LLM client library for PHP. Unified interface for streaming completions, tool calling, and provider routing across OpenAI, Anthropic, and 50+ LLM providers. Powered by Rust core.

## Installation

```bash
composer require kreuzberg/liter-lm
```

## Quick Start

```php
<?php
use LiterLm\LlmClient;
use LiterLm\ChatRequest;
use LiterLm\Message;

$client = new LlmClient();
$response = $client->chat(new ChatRequest(
    model: "openai/gpt-4",
    messages: [
        new Message(role: "user", content: "Hello!")
    ]
));
echo $response->getContent();
```

## Full Documentation

For comprehensive documentation, examples, and API reference, see the [main repository](https://github.com/kreuzberg-dev/liter-lm).
