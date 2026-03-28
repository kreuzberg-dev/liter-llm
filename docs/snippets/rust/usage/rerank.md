<!-- snippet:compile-only -->

```rust
use liter_llm::{ClientConfigBuilder, DefaultClient, LlmClient, RerankRequest};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = ClientConfigBuilder::new(std::env::var("OPENAI_API_KEY")?)
        .build();
    let client = DefaultClient::new(config, Some("cohere/rerank-v3.5"))?;

    let response = client
        .rerank(RerankRequest {
            model: "cohere/rerank-v3.5".into(),
            query: "What is the capital of France?".into(),
            documents: vec![
                "Paris is the capital of France.".into(),
                "Berlin is the capital of Germany.".into(),
                "London is the capital of England.".into(),
            ],
            ..Default::default()
        })
        .await?;

    for result in &response.results {
        println!("Index: {}, Score: {:.4}", result.index, result.relevance_score);
    }
    Ok(())
}
```
