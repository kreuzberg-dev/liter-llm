---
description: "Installing liter-llm for Python, TypeScript, Rust, Go, Java, Ruby, PHP, C#, Elixir, WebAssembly, and C/FFI"
---

# Installation

liter-llm has prebuilt packages for every supported language. Pick your stack, run one command, and start calling models.

Every package includes prebuilt binaries for Linux (x86_64 / aarch64), macOS (Apple Silicon), and Windows. No Rust toolchain needed unless you're building from source.

## CLI / Docker

The CLI runs the proxy server and MCP tool server. You don't need it if you're only using a language binding.

=== "Homebrew"

    ```bash
    brew tap kreuzberg-dev/tap
    brew install liter-llm
    ```

=== "Cargo"

    ```bash
    cargo install liter-llm-cli
    ```

=== "Docker"

    ```bash
    docker pull ghcr.io/kreuzberg-dev/liter-llm:latest
    docker run -p 4000:4000 -e LITER_LLM_MASTER_KEY=sk-your-key ghcr.io/kreuzberg-dev/liter-llm
    ```

Start the proxy:

```bash
liter-llm api --config liter-llm-proxy.toml
```

Or the MCP server:

```bash
liter-llm mcp --transport stdio
```

[:octicons-arrow-right-24: Proxy Server docs](../server/proxy-server.md) &nbsp; [:octicons-arrow-right-24: MCP Server docs](../server/mcp-server.md)

---

## Choose your language

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
        <version>1.4.0-rc.17</version>
    </dependency>
    ```

    **Gradle:**

    ```kotlin
    implementation("dev.kreuzberg:liter-llm:1.4.0-rc.17")
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
        {:liter_llm, "~> 1.4.0-rc.17"}
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

---

## API Key Setup

Set the environment variable for the provider you're calling:

```bash
export OPENAI_API_KEY="sk-..."
export ANTHROPIC_API_KEY="sk-ant-..."
export GOOGLE_API_KEY="..."
export GROQ_API_KEY="gsk_..."
export MISTRAL_API_KEY="..."
export AWS_ACCESS_KEY_ID="..."
export AWS_SECRET_ACCESS_KEY="..."
```

!!! tip "You only need one key"
If you only call OpenAI models, only `OPENAI_API_KEY` is needed. liter-llm resolves the provider from the model prefix (e.g. `openai/gpt-4o`) and picks the matching key automatically.

You can also pass the key at client construction:

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

!!! warning "Don't hard-code keys in source files"
Use environment variables or a secret manager. Keys passed to `LlmClient` are wrapped in `secrecy::SecretString` and never logged.

---

## Verify it works

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

---

## Building from source

If prebuilt binaries aren't available for your platform, build from source. You'll need the Rust toolchain (stable 1.75+):

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
git clone https://github.com/kreuzberg-dev/liter-llm.git
cd liter-llm
task build
```

---

## Next steps

- [Chat & Streaming](../usage/chat.md) -- Make your first API call
- [MCP & IDE Integration](../usage/mcp-integration.md) -- Integrate with VS Code, GitHub Copilot, Claude, Cursor
- [Provider Registry](../providers.md) -- Browse all 142+ supported providers
- [Configuration](../usage/configuration.md) -- Timeouts, retries, base URL overrides
