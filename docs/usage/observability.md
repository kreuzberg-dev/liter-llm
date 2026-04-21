---
description: "OpenTelemetry tracing and cost tracking for liter-llm requests."
---

# Observability

liter-llm emits OpenTelemetry-compatible tracing spans for every LLM request via two Tower middleware layers: `TracingLayer` and `CostTrackingLayer`. Spans follow the [OpenTelemetry GenAI semantic conventions](https://opentelemetry.io/docs/specs/semconv/gen-ai/).

## Feature flags

| Flag | Purpose |
|------|---------|
| `tracing` | Enables `TracingLayer` and `CostTrackingLayer`. Required for any span emission. |
| `otel` | Re-exports `tracing_opentelemetry` and `opentelemetry` crates so callers can wire a full OTEL pipeline without adding direct dependencies. |

Enable in `Cargo.toml`:

```toml
[dependencies]
liter-llm = { version = "...", features = ["tracing"] }
# Add "otel" to export spans to an OTEL collector:
liter-llm = { version = "...", features = ["tracing", "otel"] }
```

## Span attributes

Each request creates a `gen_ai` span. The following attributes are populated according to the GenAI semantic conventions:

| Attribute | Type | When set |
|-----------|------|----------|
| `gen_ai.operation.name` | string | Always. Values: `"chat"`, `"embeddings"`, `"list_models"`, `"image_generate"`, `"speech"`, `"transcribe"`, `"moderate"`, `"rerank"`, `"search"`, `"ocr"`. |
| `gen_ai.request.model` | string | Always. Empty string for `list_models`. |
| `gen_ai.system` | string | Always. The provider prefix from the model name (e.g. `"openai"` for `"openai/gpt-4"`). Empty when no prefix is present. |
| `gen_ai.response.id` | string | Successful chat responses. |
| `gen_ai.response.model` | string | Successful chat and embedding responses. |
| `gen_ai.response.finish_reasons` | string | Successful chat responses. Space-separated finish reason names (e.g. `"stop"` or `"length tool_calls"`). |
| `gen_ai.usage.input_tokens` | int | Successful chat and embedding responses when usage data is present. |
| `gen_ai.usage.output_tokens` | int | Successful chat responses when usage data is present. |
| `gen_ai.usage.cost` | float | Set by `CostTrackingLayer` when the model appears in the pricing registry. Value is USD. |
| `error.type` | string | On error. Set to the `LiterLlmError` variant name (e.g. `"RateLimited"`, `"Timeout"`). |

## Enabling tracing on the client

Pass `tracing=True` (or the equivalent for your language) when constructing the client. The client then applies `TracingLayer` and `CostTrackingLayer` to the inner Tower service.

=== "Python"

    ```python
    client = LlmClient(api_key="sk-...", tracing=True)
    ```

=== "TypeScript"

    ```typescript
    const client = new LlmClient({ apiKey: "sk-...", tracing: true });
    ```

=== "Rust"

    ```rust
    let config = ClientConfigBuilder::new("sk-...")
        .tracing(true)
        .build();
    ```

=== "Go"

    ```go
    client := llm.NewClient(
        llm.WithAPIKey(os.Getenv("OPENAI_API_KEY")),
        llm.WithTracing(),
    )
    ```

=== "Java"

    ```java
    var client = LlmClient.builder()
            .apiKey(System.getenv("OPENAI_API_KEY"))
            .tracing(true)
            .build();
    ```

=== "C#"

    ```csharp
    var client = new LlmClient(
        apiKey: Environment.GetEnvironmentVariable("OPENAI_API_KEY")!,
        tracing: true);
    ```

=== "Ruby"

    ```ruby
    client = LiterLlm::LlmClient.new(ENV.fetch("OPENAI_API_KEY"),
      tracing: true
    )
    ```

=== "PHP"

    ```php
    $client = new LlmClient(
        apiKey: getenv('OPENAI_API_KEY') ?: '',
        tracing: true,
    );
    ```

=== "Elixir"

    ```elixir
    client = LiterLlm.Client.new(
      api_key: System.fetch_env!("OPENAI_API_KEY"),
      tracing: true
    )
    ```

=== "WASM"

    ```typescript
    const client = new LlmClient({ apiKey: "sk-...", tracing: true });
    ```

## Exporting spans with OpenTelemetry (Rust)

The `otel` feature re-exports `tracing_opentelemetry` and `opentelemetry` at `liter_llm::tower::tracing::otel`. Wire a subscriber that sends spans to an OTEL collector:

```rust
use liter_llm::tower::tracing::otel::{
    tracing_opentelemetry::OpenTelemetryLayer,
    opentelemetry,
};
use opentelemetry_otlp::WithExportConfig;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

// Build an OTLP exporter sending to localhost:4317.
let tracer = opentelemetry_otlp::new_pipeline()
    .tracing()
    .with_exporter(
        opentelemetry_otlp::new_exporter()
            .tonic()
            .with_endpoint("http://localhost:4317"),
    )
    .install_batch(opentelemetry_sdk::runtime::Tokio)?;

// Attach the OTEL layer to the tracing subscriber.
tracing_subscriber::registry()
    .with(OpenTelemetryLayer::new(tracer))
    .with(tracing_subscriber::fmt::layer())
    .init();

// Now construct the client with tracing=true.
let config = ClientConfigBuilder::new("sk-...").tracing(true).build();
let client = DefaultClient::new(config, None)?;
```

Any OTEL-compatible backend accepts these spans: Jaeger, Tempo, Honeycomb, Datadog, etc.

## Cost tracking

`CostTrackingLayer` records estimated USD cost as `gen_ai.usage.cost` on the active tracing span after each successful response. It looks up pricing from the embedded pricing registry (`crates/liter-llm/schemas/pricing.json`). Models not in the registry produce no attribute.

Enable cost tracking independently of tracing:

=== "Python"

    ```python
    client = LlmClient(api_key="sk-...", cost_tracking=True)
    ```

=== "TypeScript"

    ```typescript
    const client = new LlmClient({ apiKey: "sk-...", costTracking: true });
    ```

=== "Rust"

    ```rust
    use liter_llm::tower::{CostTrackingLayer, LlmService};
    use tower::ServiceBuilder;

    let inner = LlmService::new(client);
    let service = ServiceBuilder::new()
        .layer(CostTrackingLayer)
        .service(inner);
    ```

The cost value is also accessible directly on successful response objects via `estimated_cost()`:

```rust
let resp = client.chat(req).await?;
if let Some(cost_usd) = resp.estimated_cost() {
    println!("cost: ${:.6}", cost_usd);
}
```

The pricing registry lives at `crates/liter-llm/schemas/pricing.json`. Models not in the registry produce no `gen_ai.usage.cost` attribute.

## Proxy trace context forwarding

When running behind the proxy server, incoming `traceparent` and `tracestate` headers are forwarded to the upstream provider request. The proxy creates a child span for each routed request, which allows distributed traces to span the client, proxy, and provider in a single trace tree.

Enable tracing on the proxy by setting `tracing = true` in the `[general]` section of the proxy configuration file. See [Proxy Configuration](../server/proxy-configuration.md) for the full field reference.

## Tower layer composition

`TracingLayer` and `CostTrackingLayer` are standard Tower layers and compose with any `Service<LlmRequest>`. The recommended order wraps `CostTrackingLayer` inside `TracingLayer` so the cost attribute is recorded on the same span:

```rust
use liter_llm::tower::{CostTrackingLayer, LlmService, TracingLayer};
use tower::ServiceBuilder;

let service = ServiceBuilder::new()
    .layer(TracingLayer)          // outer: opens the gen_ai span
    .layer(CostTrackingLayer)     // inner: records cost inside the open span
    .service(LlmService::new(client));
```
