<!-- snippet:compile-only -->

```rust
use liter_llm::{ClientConfigBuilder, CreateFileRequest, DefaultClient, LlmClient};
use tokio::fs;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = ClientConfigBuilder::new(std::env::var("OPENAI_API_KEY")?)
        .build();
    let client = DefaultClient::new(config, Some("openai/gpt-4o"))?;

    let file_bytes = fs::read("data.jsonl").await?;
    let response = client
        .create_file(CreateFileRequest {
            file: file_bytes,
            filename: "data.jsonl".into(),
            purpose: "batch".into(),
        })
        .await?;

    println!("File ID: {}", response.id);
    println!("Size: {} bytes", response.bytes);
    Ok(())
}
```
