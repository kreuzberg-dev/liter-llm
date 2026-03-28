<!-- snippet:compile-only -->

```rust
use liter_llm::{
    ClientConfigBuilder, CreateResponseRequest, DefaultClient, ResponseClient,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = ClientConfigBuilder::new(std::env::var("OPENAI_API_KEY")?)
        .build();
    let client = DefaultClient::new(config, Some("openai/gpt-4o"))?;

    let request = CreateResponseRequest {
        model: "openai/gpt-4o".into(),
        input: Some("Explain quantum computing in one sentence.".into()),
        ..Default::default()
    };

    let response = client.create_response(request).await?;
    println!("{:?}", response);
    Ok(())
}
```
