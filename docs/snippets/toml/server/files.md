```toml
# In-memory (default). Files are lost on restart.
[files]
backend = "memory"

# S3-backed file store.
[files]
backend = "s3"
prefix = "liter-llm-files/"

[files.backend_config]
bucket = "my-llm-files"
region = "us-west-2"
```
