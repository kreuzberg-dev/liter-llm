```toml
[budget]
global_limit = 1000.0
enforcement = "hard"  # or "soft" to log but allow through

[budget.model_limits]
"openai/gpt-4o" = 500.0
"anthropic/claude-opus-4-20250514" = 200.0
```
