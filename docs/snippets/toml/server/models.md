```toml
[[models]]
name = "gpt-4o"
provider_model = "openai/gpt-4o"
api_key = "${OPENAI_API_KEY}"
timeout_secs = 60
fallbacks = ["claude-sonnet", "llama3-groq"]

[[models]]
name = "claude-sonnet"
provider_model = "anthropic/claude-sonnet-4-20250514"
api_key = "${ANTHROPIC_API_KEY}"

[[models]]
name = "llama3-groq"
provider_model = "groq/llama3-70b-8192"
api_key = "${GROQ_API_KEY}"
```
