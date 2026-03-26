# liter-lm (Java)

High-performance LLM client library for Java. Unified interface for streaming completions, tool calling, and provider routing across OpenAI, Anthropic, and 50+ LLM providers. Powered by Rust core.

## Installation

Add to `pom.xml`:

```xml
<dependency>
  <groupId>dev.kreuzberg</groupId>
  <artifactId>liter-lm</artifactId>
  <version>1.0.0-rc.1</version>
</dependency>
```

## Quick Start

```java
import dev.kreuzberg.LiterLm.*;

public class Main {
  public static void main(String[] args) {
    LlmClient client = new LlmClient();
    ChatResponse response = client.chat(
      "openai/gpt-4",
      new Message("user", "Hello!")
    );
    System.out.println(response.getContent());
  }
}
```

## Full Documentation

For comprehensive documentation, examples, and API reference, see the [main repository](https://github.com/kreuzberg-dev/liter-lm).
