```rust
use liter_llm::{
    ClientConfigBuilder, DefaultClient, EmbeddingInput, EmbeddingRequest, LlmClient,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = ClientConfigBuilder::new(std::env::var("OPENAI_API_KEY")?)
        .build();
    let client = DefaultClient::new(config, Some("openai/text-embedding-3-small"))?;

    let request = EmbeddingRequest {
        model: "openai/text-embedding-3-small".into(),
        input: EmbeddingInput::Multiple(vec![
            "The quick brown fox jumps over the lazy dog".into(),
        ]),
        ..Default::default()
    };

    let response = client.embed(request).await?;
    let embedding = &response.data[0].embedding;
    println!("Dimensions: {}", embedding.len());
    println!("First 5 values: {:?}", &embedding[..5]);
    Ok(())
}
```
