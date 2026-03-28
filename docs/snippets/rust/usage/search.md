<!-- snippet:compile-only -->

```rust
use liter_llm::{ClientConfigBuilder, DefaultClient, LlmClient, SearchRequest};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = ClientConfigBuilder::new(std::env::var("BRAVE_API_KEY")?)
        .build();
    let client = DefaultClient::new(config, Some("brave/web-search"))?;

    let response = client
        .search(SearchRequest {
            model: "brave/web-search".into(),
            query: "What is Rust programming language?".into(),
            max_results: Some(5),
            ..Default::default()
        })
        .await?;

    for result in &response.results {
        println!("{}: {}", result.title, result.url);
    }
    Ok(())
}
```
