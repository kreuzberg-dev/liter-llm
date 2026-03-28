<!-- snippet:compile-only -->

```rust
use liter_llm::{ClientConfigBuilder, CreateBatchRequest, DefaultClient, LlmClient};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = ClientConfigBuilder::new(std::env::var("OPENAI_API_KEY")?)
        .build();
    let client = DefaultClient::new(config, Some("openai/gpt-4o"))?;

    let response = client
        .create_batch(CreateBatchRequest {
            input_file_id: "file-abc123".into(),
            endpoint: "/v1/chat/completions".into(),
            completion_window: "24h".into(),
            ..Default::default()
        })
        .await?;

    println!("Batch ID: {}", response.id);
    println!("Status: {}", response.status);
    Ok(())
}
```
