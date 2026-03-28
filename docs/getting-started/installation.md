---
description: "Installing liter-llm for Python, TypeScript, Rust, Go, Java, Ruby, PHP, C#, Elixir, WebAssembly, and C/FFI"
---

# Installation

liter-llm ships prebuilt native packages for all major languages. No Rust toolchain required unless building from source.

## Install

=== "Python"

    Requires Python 3.10+

    ```bash
    pip install liter-llm
    ```

    Or with [uv](https://docs.astral.sh/uv/):

    ```bash
    uv add liter-llm
    ```

=== "TypeScript / Node.js"

    Requires Node.js 18+

    ```bash
    pnpm add @kreuzberg/liter-llm
    ```

    Or with npm / yarn:

    ```bash
    npm install @kreuzberg/liter-llm
    # or
    yarn add @kreuzberg/liter-llm
    ```

=== "Rust"

    Requires Rust 1.75+ (stable)

    ```bash
    cargo add liter-llm
    ```

=== "Go"

    Requires Go 1.23+

    ```bash
    go get github.com/kreuzberg-dev/liter-llm/packages/go
    ```

=== "Java"

    Requires Java 17+ (Panama FFM)

    **Maven:**

    ```xml
    <dependency>
        <groupId>dev.kreuzberg</groupId>
        <artifactId>liter-llm</artifactId>
        <version>0.1.0</version>
    </dependency>
    ```

    **Gradle:**

    ```kotlin
    implementation("dev.kreuzberg:liter-llm:0.1.0")
    ```

=== "Ruby"

    Requires Ruby 3.2+

    ```bash
    gem install liter_llm
    ```

    Or add to your `Gemfile`:

    ```ruby
    gem "liter_llm"
    ```

=== "PHP"

    Requires PHP 8.2+

    ```bash
    composer require kreuzberg/liter-llm
    ```

=== "C# / .NET"

    Requires .NET 8+

    ```bash
    dotnet add package LiterLlm
    ```

=== "Elixir"

    Requires Elixir 1.14+ / OTP 25+

    Add to `mix.exs`:

    ```elixir
    defp deps do
      [
        {:liter_llm, "~> 0.1"}
      ]
    end
    ```

    Then run:

    ```bash
    mix deps.get
    ```

=== "WebAssembly"

    ```bash
    pnpm add @kreuzberg/liter-llm-wasm
    ```

=== "C / FFI"

    Build from source (requires Rust toolchain):

    ```bash
    git clone https://github.com/kreuzberg-dev/liter-llm.git
    cd liter-llm
    cargo build --release -p liter-llm-ffi
    ```

    The shared library and C header are output to `target/release/`.

## API Key Setup

liter-llm reads API keys from environment variables. Set the key for the provider(s) you plan to use:

```bash
export OPENAI_API_KEY="sk-..."
export ANTHROPIC_API_KEY="sk-ant-..."
export GOOGLE_API_KEY="..."
export GROQ_API_KEY="gsk_..."
export MISTRAL_API_KEY="..."
export AWS_ACCESS_KEY_ID="..."
export AWS_SECRET_ACCESS_KEY="..."
```

!!! tip "You only need the key for the provider you are calling"
    If you only use OpenAI models, only `OPENAI_API_KEY` is required. liter-llm resolves the provider from the model prefix (e.g. `openai/gpt-4o`) and injects the matching key automatically.

You can also pass the key directly at client construction:

=== "Python"

    ```python
    from liter_llm import LlmClient

    client = LlmClient(api_key="sk-...")
    ```

=== "TypeScript"

    ```typescript
    import { LlmClient } from "@kreuzberg/liter-llm";

    const client = new LlmClient({ apiKey: "sk-..." });
    ```

=== "Rust"

    ```rust
    use liter_llm::{ClientConfigBuilder, DefaultClient};

    let config = ClientConfigBuilder::new("sk-...").build();
    let client = DefaultClient::new(config, None)?;
    ```

!!! warning "Do not hard-code keys in source files"
    Use environment variables or a secret manager. API keys passed to `LlmClient` are wrapped in `secrecy::SecretString` internally and never logged.

## Verify Installation

=== "Python"

    ```bash
    python -c "from liter_llm import LlmClient; print('ok')"
    ```

=== "TypeScript"

    ```bash
    node -e "import('@kreuzberg/liter-llm').then(m => { new m.LlmClient({ apiKey: 'test' }); console.log('ok') })"
    ```

=== "Rust"

    ```bash
    cargo build
    ```

=== "Go"

    ```bash
    go build ./...
    ```

## Building from Source

If prebuilt binaries are not available for your platform, you can build from source. This requires the Rust toolchain (stable 1.75+):

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
git clone https://github.com/kreuzberg-dev/liter-llm.git
cd liter-llm
task build
```

## Next Steps

- [Chat & Streaming](../usage/chat.md) -- Make your first API call
- [Provider Registry](../providers.md) -- Browse all 142 supported providers
- [Configuration](../usage/configuration.md) -- Timeouts, retries, base URL overrides
