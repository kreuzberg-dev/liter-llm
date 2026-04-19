```toml
# Any ${VAR} pattern in a string value is replaced with the env var at load time.
# Unknown variables expand to an empty string.
[general]
master_key = "${LITER_LLM_MASTER_KEY}"

[[models]]
name = "gpt-4o"
provider_model = "openai/gpt-4o"
api_key = "${OPENAI_API_KEY}"
base_url = "${OPENAI_BASE_URL}"  # empty if unset
```
