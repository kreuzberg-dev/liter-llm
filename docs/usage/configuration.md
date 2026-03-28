---
description: "Client configuration: API keys, timeouts, retries, cache, budget, hooks, and custom providers."
---

# Configuration

## Client Construction

=== "Python"

    --8<-- "snippets/python/guides/configuration.md"

=== "TypeScript"

    --8<-- "snippets/typescript/guides/configuration.md"

=== "Rust"

    --8<-- "snippets/rust/usage/configuration.md"

=== "Go"

    --8<-- "snippets/go/guides/configuration.md"

=== "Java"

    --8<-- "snippets/java/usage/configuration.md"

=== "C#"

    --8<-- "snippets/csharp/usage/configuration.md"

=== "Ruby"

    --8<-- "snippets/ruby/usage/configuration.md"

=== "PHP"

    --8<-- "snippets/php/usage/configuration.md"

=== "Elixir"

    --8<-- "snippets/elixir/usage/configuration.md"

=== "WASM"

    --8<-- "snippets/wasm/usage/configuration.md"

## Options

| Option | Type | Default | Description |
| --- | --- | --- | --- |
| `api_key` | string | **required** | Provider API key. Wrapped in `SecretString` internally. |
| `base_url` | string | from registry | Override the provider's base URL. |
| `model_hint` | string | none | Pre-resolve a provider at construction (e.g. `"openai"`). |
| `timeout` | duration | 60s | Request timeout. |
| `max_retries` | int | 3 | Retries on 429/5xx responses with exponential backoff. |

## API Key Management

Read the standard environment variable for your provider:

| Provider | Environment Variable |
| --- | --- |
| OpenAI | `OPENAI_API_KEY` |
| Anthropic | `ANTHROPIC_API_KEY` |
| Google (Gemini) | `GEMINI_API_KEY` |
| Groq | `GROQ_API_KEY` |
| Mistral | `MISTRAL_API_KEY` |
| Cohere | `CO_API_KEY` |
| AWS Bedrock | `AWS_ACCESS_KEY_ID` + `AWS_SECRET_ACCESS_KEY` |

API keys passed to the constructor are wrapped in `secrecy::SecretString`. They are never logged, serialized, or included in error messages.

## Model Hints

The `model_hint` parameter pre-resolves a provider at construction time. All requests use that provider without prefix lookup:

```python
# All requests use OpenAI -- no "openai/" prefix needed
client = LlmClient(api_key="sk-...", model_hint="openai")
response = await client.chat(model="gpt-4o", messages=[...])
```

## Custom Base URLs

Override `base_url` to point at a local inference server or proxy:

```python
# Ollama running locally
client = LlmClient(api_key="unused", base_url="http://localhost:11434/v1")

# Corporate proxy
client = LlmClient(api_key="sk-...", base_url="https://llm-proxy.internal.company.com/v1")
```

## Cache

Enable response caching to avoid repeated identical requests:

=== "Python"

    ```python
    from liter_llm import LlmClient

    client = LlmClient(
        api_key="sk-...",
        cache={"max_entries": 256, "ttl_seconds": 300},
    )
    ```

=== "TypeScript"

    ```typescript
    import { LlmClient } from "@kreuzberg/liter-llm";

    const client = new LlmClient({
      apiKey: process.env.OPENAI_API_KEY!,
      cache: { maxEntries: 256, ttlSeconds: 300 },
    });
    ```

=== "Rust"

    ```rust
    use liter_llm::{ClientConfigBuilder, CacheConfig};

    let config = ClientConfigBuilder::new("sk-...")
        .cache(CacheConfig { max_entries: 256, ttl_seconds: 300 })
        .build();
    ```

=== "Go"

    ```go
    client := llm.NewClient(
        llm.WithAPIKey(os.Getenv("OPENAI_API_KEY")),
        llm.WithCache(llm.CacheConfig{MaxEntries: 256, TTLSeconds: 300}),
    )
    ```

=== "Java"

    ```java
    var client = LlmClient.builder()
            .apiKey(System.getenv("OPENAI_API_KEY"))
            .cacheConfig(new CacheConfig(256, 300))
            .build();
    ```

=== "C#"

    ```csharp
    var client = new LlmClient(
        apiKey: Environment.GetEnvironmentVariable("OPENAI_API_KEY")!,
        cacheConfig: new CacheConfig(MaxEntries: 256, TtlSeconds: 300));
    ```

=== "Ruby"

    ```ruby
    client = LiterLlm::LlmClient.new(ENV.fetch("OPENAI_API_KEY"), {
      cache: { max_entries: 256, ttl_seconds: 300 }
    })
    ```

=== "PHP"

    ```php
    $client = new LlmClient(
        apiKey: getenv('OPENAI_API_KEY') ?: '',
        cacheConfig: ['max_entries' => 256, 'ttl_seconds' => 300],
    );
    ```

=== "Elixir"

    ```elixir
    client = LiterLlm.Client.new(
      api_key: System.fetch_env!("OPENAI_API_KEY"),
      cache: [max_entries: 256, ttl_seconds: 300]
    )
    ```

=== "WASM"

    ```typescript
    import init, { LlmClient } from "@kreuzberg/liter-llm-wasm";
    await init();

    const client = new LlmClient({
      apiKey: "sk-...",
      cache: { maxEntries: 256, ttlSeconds: 300 },
    });
    ```

| Option | Type | Default | Description |
| --- | --- | --- | --- |
| `max_entries` | int | 256 | Maximum cached responses |
| `ttl_seconds` | int | 300 | Time-to-live in seconds |

## Budget

Track and enforce spending limits:

=== "Python"

    ```python
    from liter_llm import LlmClient

    client = LlmClient(
        api_key="sk-...",
        budget={"global_limit": 10.0, "model_limits": {"openai/gpt-4o": 5.0}, "enforcement": "hard"},
    )
    print(f"Budget used: ${client.budget_used:.2f}")
    ```

=== "TypeScript"

    ```typescript
    import { LlmClient } from "@kreuzberg/liter-llm";

    const client = new LlmClient({
      apiKey: process.env.OPENAI_API_KEY!,
      budget: { globalLimit: 10.0, modelLimits: { "openai/gpt-4o": 5.0 }, enforcement: "hard" },
    });
    console.log(`Budget used: $${client.budgetUsed.toFixed(2)}`);
    ```

=== "Rust"

    ```rust
    use liter_llm::{ClientConfigBuilder, BudgetConfig};

    let config = ClientConfigBuilder::new("sk-...")
        .budget(BudgetConfig {
            global_limit: Some(10.0),
            model_limits: Default::default(),
            enforcement: "hard".into(),
        })
        .build();
    ```

=== "Go"

    ```go
    client := llm.NewClient(
        llm.WithAPIKey(os.Getenv("OPENAI_API_KEY")),
        llm.WithBudget(llm.BudgetConfig{
            GlobalLimit: 10.0,
            ModelLimits: map[string]float64{"openai/gpt-4o": 5.0},
            Enforcement: "hard",
        }),
    )
    fmt.Printf("Budget used: $%.2f\n", client.BudgetUsed())
    ```

=== "Java"

    ```java
    var client = LlmClient.builder()
            .apiKey(System.getenv("OPENAI_API_KEY"))
            .budgetConfig(new BudgetConfig(10.0, Map.of("openai/gpt-4o", 5.0), "hard"))
            .build();
    System.out.printf("Budget used: $%.2f%n", client.getBudgetUsed());
    ```

=== "C#"

    ```csharp
    var client = new LlmClient(
        apiKey: Environment.GetEnvironmentVariable("OPENAI_API_KEY")!,
        budgetConfig: new BudgetConfig(
            GlobalLimit: 10.0,
            ModelLimits: new() { ["openai/gpt-4o"] = 5.0 },
            Enforcement: "hard"));
    Console.WriteLine($"Budget used: ${client.BudgetUsed:F2}");
    ```

=== "Ruby"

    ```ruby
    client = LiterLlm::LlmClient.new(ENV.fetch("OPENAI_API_KEY"), {
      budget: { global_limit: 10.0, model_limits: {}, enforcement: "hard" }
    })
    puts "Budget used: $#{client.budget_used}"
    ```

=== "PHP"

    ```php
    $client = new LlmClient(
        apiKey: getenv('OPENAI_API_KEY') ?: '',
        budgetConfig: ['global_limit' => 10.0, 'enforcement' => 'hard'],
    );
    echo "Budget used: $" . $client->getBudgetUsed() . PHP_EOL;
    ```

=== "Elixir"

    ```elixir
    client = LiterLlm.Client.new(
      api_key: System.fetch_env!("OPENAI_API_KEY"),
      budget: [global_limit: 10.0, enforcement: "hard"]
    )
    IO.puts("Budget used: $#{LiterLlm.Client.budget_used(client)}")
    ```

=== "WASM"

    ```typescript
    import init, { LlmClient } from "@kreuzberg/liter-llm-wasm";
    await init();

    const client = new LlmClient({
      apiKey: "sk-...",
      budget: { globalLimit: 10.0, enforcement: "hard" },
    });
    console.log(`Budget used: $${client.budgetUsed.toFixed(2)}`);
    ```

| Option | Type | Description |
| --- | --- | --- |
| `global_limit` | float | Maximum total spend in USD |
| `model_limits` | map | Per-model spend limits |
| `enforcement` | string | `"hard"` (reject over-budget) or `"soft"` (warn only) |

## Hooks

Register lifecycle hooks for request/response/error events:

=== "Python"

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

=== "TypeScript"

    ```typescript
    import { LlmClient } from "@kreuzberg/liter-llm";

    const client = new LlmClient({ apiKey: process.env.OPENAI_API_KEY! });
    client.addHook({
      onRequest(req) { console.log(`Sending: ${req.model}`); },
      onResponse(req, res) { console.log(`Tokens: ${res.usage?.totalTokens}`); },
      onError(req, err) { console.error(`Error: ${err}`); },
    });
    ```

=== "Rust"

    ```rust
    use liter_llm::LlmHook;

    struct LoggingHook;
    impl LlmHook for LoggingHook {
        fn on_request(&self, req: &ChatCompletionRequest) -> Result<()> {
            println!("Sending: {}", req.model);
            Ok(())
        }
        fn on_response(&self, _req: &ChatCompletionRequest, resp: &ChatCompletionResponse) {
            if let Some(u) = &resp.usage { println!("Tokens: {}", u.total_tokens); }
        }
        fn on_error(&self, _req: &ChatCompletionRequest, err: &LiterLlmError) {
            eprintln!("Error: {err}");
        }
    }
    ```

=== "Go"

    ```go
    type loggingHook struct{}
    func (h *loggingHook) OnRequest(req *llm.ChatCompletionRequest) error {
        fmt.Printf("Sending: %s\n", req.Model)
        return nil
    }
    func (h *loggingHook) OnResponse(req *llm.ChatCompletionRequest, resp *llm.ChatCompletionResponse) {
        if resp.Usage != nil { fmt.Printf("Tokens: %d\n", resp.Usage.TotalTokens) }
    }
    func (h *loggingHook) OnError(req *llm.ChatCompletionRequest, err error) {
        fmt.Printf("Error: %v\n", err)
    }

    client := llm.NewClient(
        llm.WithAPIKey(os.Getenv("OPENAI_API_KEY")),
        llm.WithHook(&loggingHook{}),
    )
    ```

=== "Java"

    ```java
    client.addHook(new LlmHook() {
        @Override public void onRequest(ChatCompletionRequest req) {
            System.out.println("Sending: " + req.model());
        }
        @Override public void onResponse(ChatCompletionRequest req, ChatCompletionResponse resp) {
            System.out.println("Tokens: " + resp.usage().totalTokens());
        }
        @Override public void onError(ChatCompletionRequest req, LlmException err) {
            System.err.println("Error: " + err.getMessage());
        }
    });
    ```

=== "C#"

    ```csharp
    client.AddHook(new LoggingHook());

    class LoggingHook : ILlmHook
    {
        public Task OnRequestAsync(ChatCompletionRequest req) {
            Console.WriteLine($"Sending: {req.Model}");
            return Task.CompletedTask;
        }
        public Task OnResponseAsync(ChatCompletionRequest req, ChatCompletionResponse resp) {
            Console.WriteLine($"Tokens: {resp.Usage?.TotalTokens}");
            return Task.CompletedTask;
        }
        public Task OnErrorAsync(ChatCompletionRequest req, Exception err) {
            Console.Error.WriteLine($"Error: {err.Message}");
            return Task.CompletedTask;
        }
    }
    ```

=== "Ruby"

    ```ruby
    hook = {
      on_request: ->(req) { puts "Sending: #{JSON.parse(req)['model']}" },
      on_response: ->(req, resp) { puts "Response received" },
      on_error: ->(req, err) { puts "Error: #{err}" }
    }
    client.add_hook(hook)
    ```

=== "PHP"

    ```php
    $client->addHook(new class {
        public function onRequest(string $requestJson): void {
            $req = json_decode($requestJson, true);
            echo "Sending: {$req['model']}" . PHP_EOL;
        }
        public function onResponse(string $requestJson, string $responseJson): void {
            echo "Response received" . PHP_EOL;
        }
        public function onError(string $requestJson, string $errorMessage): void {
            echo "Error: {$errorMessage}" . PHP_EOL;
        }
    });
    ```

=== "Elixir"

    ```elixir
    defmodule LoggingHook do
      @behaviour LiterLlm.Hook

      def on_request(request), do: IO.puts("Sending: #{request["model"]}")
      def on_response(_request, _response), do: IO.puts("Response received")
      def on_error(_request, error), do: IO.puts("Error: #{inspect(error)}")
    end

    client = LiterLlm.Client.new(api_key: "sk-...") |> LiterLlm.Client.add_hook(LoggingHook)
    ```

=== "WASM"

    ```typescript
    import init, { LlmClient } from "@kreuzberg/liter-llm-wasm";
    await init();

    const client = new LlmClient({ apiKey: "sk-..." });
    client.addHook({
      onRequest(req) { console.log(`Sending: ${req.model}`); },
      onResponse(req, res) { console.log(`Tokens: ${res.usage?.totalTokens}`); },
      onError(req, err) { console.error(`Error: ${err}`); },
    });
    ```

## Custom Providers

Register custom providers for self-hosted or unsupported LLM endpoints:

=== "Python"

    ```python
    from liter_llm import LlmClient

    client = LlmClient(api_key="sk-...")
    client.register_provider({
        "name": "my-provider",
        "base_url": "https://my-llm.example.com/v1",
        "auth_header": "Authorization",
        "model_prefixes": ["my-provider/"],
    })
    ```

=== "TypeScript"

    ```typescript
    import { LlmClient } from "@kreuzberg/liter-llm";

    const client = new LlmClient({ apiKey: process.env.OPENAI_API_KEY! });
    client.registerProvider({
      name: "my-provider",
      baseUrl: "https://my-llm.example.com/v1",
      authHeader: "Authorization",
      modelPrefixes: ["my-provider/"],
    });
    ```

=== "Rust"

    ```rust
    use liter_llm::{register_custom_provider, CustomProviderConfig};

    register_custom_provider(CustomProviderConfig {
        name: "my-provider".into(),
        base_url: "https://my-llm.example.com/v1".into(),
        auth_header: "Authorization".into(),
        model_prefixes: vec!["my-provider/".into()],
    })?;
    ```

=== "Go"

    ```go
    client.RegisterProvider(llm.ProviderConfig{
        Name:          "my-provider",
        BaseURL:       "https://my-llm.example.com/v1",
        AuthHeader:    "Authorization",
        ModelPrefixes: []string{"my-provider/"},
    })
    ```

=== "Java"

    ```java
    client.registerProvider(new ProviderConfig(
        "my-provider",
        "https://my-llm.example.com/v1",
        "Authorization",
        List.of("my-provider/")));
    ```

=== "C#"

    ```csharp
    client.RegisterProvider(new ProviderConfig(
        Name: "my-provider",
        BaseUrl: "https://my-llm.example.com/v1",
        AuthHeader: "Authorization",
        ModelPrefixes: ["my-provider/"]));
    ```

=== "Ruby"

    ```ruby
    client.register_provider(JSON.generate(
      name: "my-provider",
      base_url: "https://my-llm.example.com/v1",
      auth_header: "Authorization",
      model_prefixes: ["my-provider/"]
    ))
    ```

=== "PHP"

    ```php
    $client->registerProvider(json_encode([
        'name' => 'my-provider',
        'base_url' => 'https://my-llm.example.com/v1',
        'auth_header' => 'Authorization',
        'model_prefixes' => ['my-provider/'],
    ]));
    ```

=== "Elixir"

    ```elixir
    LiterLlm.register_provider(%{
      name: "my-provider",
      base_url: "https://my-llm.example.com/v1",
      auth_header: "Authorization",
      model_prefixes: ["my-provider/"]
    })
    ```

=== "WASM"

    ```typescript
    import init, { LlmClient } from "@kreuzberg/liter-llm-wasm";
    await init();

    const client = new LlmClient({ apiKey: "sk-..." });
    client.registerProvider({
      name: "my-provider",
      baseUrl: "https://my-llm.example.com/v1",
      authHeader: "Authorization",
      modelPrefixes: ["my-provider/"],
    });
    ```
