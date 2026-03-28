<!-- snippet:compile-only -->

```rust
use liter_llm::{ClientConfigBuilder, DefaultClient, LlmClient, OcrRequest, DocumentInput};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = ClientConfigBuilder::new(std::env::var("MISTRAL_API_KEY")?)
        .build();
    let client = DefaultClient::new(config, Some("mistral/mistral-ocr-latest"))?;

    let response = client
        .ocr(OcrRequest {
            model: "mistral/mistral-ocr-latest".into(),
            document: DocumentInput::Url {
                url: "https://example.com/invoice.pdf".into(),
            },
            ..Default::default()
        })
        .await?;

    for page in &response.pages {
        println!("Page {}: {}...", page.index, &page.markdown[..100]);
    }
    Ok(())
}
```
