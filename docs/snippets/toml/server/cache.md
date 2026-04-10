```toml
# In-memory cache (default).
[cache]
max_entries = 4096
ttl_seconds = 900
backend = "memory"
```

```toml
# Redis cache via OpenDAL.
[cache]
max_entries = 10_000
ttl_seconds = 3600
backend = "redis"

[cache.backend_config]
endpoint = "redis://cache.internal:6379"
```
