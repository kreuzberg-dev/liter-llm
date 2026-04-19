```toml
[[keys]]
key = "vk-team-frontend"
description = "Frontend team, chat-only, capped spend"
models = ["gpt-4o", "claude-sonnet"]
rpm = 60
tpm = 200_000
budget_limit = 100.0

[[keys]]
key = "vk-batch-worker"
description = "Overnight batch jobs, unrestricted model access"
rpm = 10
budget_limit = 500.0
```
