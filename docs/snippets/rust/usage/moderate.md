<!-- snippet:compile-only -->

```rust
use liter_llm::{ClientConfigBuilder, CreateModerationRequest, DefaultClient, LlmClient};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = ClientConfigBuilder::new(std::env::var("OPENAI_API_KEY")?)
        .build();
    let client = DefaultClient::new(config, Some("openai/omni-moderation-latest"))?;

    let response = client
        .moderate(CreateModerationRequest {
            model: Some("openai/omni-moderation-latest".into()),
            input: "This is a test message.".into(),
        })
        .await?;

    let result = &response.results[0];
    println!("Flagged: {}", result.flagged);
    for (category, &flagged) in &result.categories {
        if flagged {
            if let Some(&score) = result.category_scores.get(category) {
                println!("  {category}: {score:.4}");
            }
        }
    }
    Ok(())
}
```
