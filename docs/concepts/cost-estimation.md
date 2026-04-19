---
description: "Per-request cost estimation using the embedded pricing registry in liter-llm."
---

# Cost Estimation

liter-llm embeds a pricing registry at compile time (`crates/liter-llm/schemas/pricing.json`) and exposes two functions for estimating USD cost from token counts. No network access or external service is required.

Pricing data is derived from the [litellm](https://github.com/BerriAI/litellm) project (MIT License, Copyright 2023 Berri AI) and covers the most widely used models across major providers.

## API

### `cost::completion_cost`

Calculate estimated USD cost given a model name and token counts.

```rust
use liter_llm::cost;

// Returns None for unknown models.
let unknown = cost::completion_cost("my-custom-model", 1000, 500);
assert!(unknown.is_none());

// Returns Some(usd) for known models.
// gpt-4o: input $2.50/1M tokens = 0.0000025/token
//         output $10/1M tokens  = 0.00001/token
let usd = cost::completion_cost("gpt-4o", 1_000, 500).unwrap();
// 1000 * 0.0000025 + 500 * 0.00001 = 0.0025 + 0.005 = 0.0075
assert!((usd - 0.0075).abs() < 1e-9);
```

### `cost::model_pricing`

Retrieve the per-token pricing struct directly.

```rust
use liter_llm::cost;

let p = cost::model_pricing("gpt-4o").unwrap();
println!("input:  ${:.10}/token", p.input_cost_per_token);
println!("output: ${:.10}/token", p.output_cost_per_token);
```

```rust
pub struct ModelPricing {
    pub input_cost_per_token: f64,   // USD per prompt token
    pub output_cost_per_token: f64,  // USD per completion token; 0.0 for embedding models
}
```

## Prefix fallback

When an exact model name is not found, the registry strips from the last `-` or `.` separator and retries. This means versioned model names like `gpt-4-0613` resolve to the `gpt-4` entry automatically.

```text
gpt-4-0613  →  try "gpt-4-0613"  →  try "gpt-4"  →  found
claude-3-opus-20240229  →  try exact  →  try "claude-3-opus"  →  found
```

## Response-level cost

Successful chat and embedding responses expose `estimated_cost()` directly on the response object, using the same registry lookup:

```rust
let resp = client.chat(req).await?;
if let Some(usd) = resp.estimated_cost() {
    println!("cost: ${:.6}", usd);
}

let resp = client.embed(req).await?;
if let Some(usd) = resp.estimated_cost() {
    println!("cost: ${:.6}", usd);
}
```

## Tracing integration

`CostTrackingLayer` records the estimated cost as `gen_ai.usage.cost` on the active OpenTelemetry span after each successful response. This requires the `tower` feature flag. See [Observability](../usage/observability.md) for setup details.

## Proxy budget enforcement

The proxy server's budget system (`[budget]` in the config file) uses the same pricing registry to track cumulative spend per virtual key and globally. Hard-budget mode rejects requests that would exceed the cap; soft-budget mode logs a warning but allows them through.

```toml
[budget]
global_limit = 50.0       # USD; rejects requests once exceeded in hard mode
enforcement = "hard"      # "hard" or "soft"

[[keys]]
key = "vk-..."
budget_limit = 5.0        # per-key cap
```

See [Proxy Configuration](../server/proxy-configuration.md) for the full `[budget]` and `[[keys]]` field reference.

## Supported models

The pricing registry covers all major OpenAI, Anthropic, Google, Mistral, Cohere, Meta, and Bedrock models. Run the following to inspect entries from the embedded registry at runtime:

```rust
// Check if a model has pricing before making a call.
if liter_llm::cost::model_pricing("anthropic/claude-3-5-sonnet-20241022").is_none() {
    eprintln!("no pricing data for this model; cost tracking will be skipped");
}
```

Models not in the registry return `None` from both `completion_cost` and `model_pricing`. Cost tracking is silently skipped for those models. No error is raised.
