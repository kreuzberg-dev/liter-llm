<!-- snippet:compile-only -->

```rust
use liter_llm::{ClientConfigBuilder, CreateSpeechRequest, DefaultClient, LlmClient};
use tokio::fs;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = ClientConfigBuilder::new(std::env::var("OPENAI_API_KEY")?)
        .build();
    let client = DefaultClient::new(config, Some("openai/tts-1"))?;

    let audio_bytes = client
        .speech(CreateSpeechRequest {
            model: "openai/tts-1".into(),
            input: "Hello, world!".into(),
            voice: "alloy".into(),
            ..Default::default()
        })
        .await?;

    fs::write("output.mp3", &audio_bytes).await?;
    println!("Wrote {} bytes to output.mp3", audio_bytes.len());
    Ok(())
}
```
