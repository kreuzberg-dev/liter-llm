<!-- snippet:compile-only -->

```rust
use liter_llm::{
    ClientConfigBuilder, CreateImageRequest, DefaultClient, LlmClient,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = ClientConfigBuilder::new(std::env::var("OPENAI_API_KEY")?)
        .build();
    let client = DefaultClient::new(config, Some("openai/dall-e-3"))?;

    let response = client
        .image_generate(CreateImageRequest {
            model: "openai/dall-e-3".into(),
            prompt: "A sunset over mountains".into(),
            n: Some(1),
            size: Some("1024x1024".into()),
            ..Default::default()
        })
        .await?;

    println!("{}", response.data[0].url.as_deref().unwrap_or(""));
    Ok(())
}
```
