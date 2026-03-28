<!-- snippet:compile-only -->

```rust
use liter_llm::{ClientConfigBuilder, CreateTranscriptionRequest, DefaultClient, LlmClient};
use tokio::fs;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = ClientConfigBuilder::new(std::env::var("OPENAI_API_KEY")?)
        .build();
    let client = DefaultClient::new(config, Some("openai/whisper-1"))?;

    let audio_bytes = fs::read("audio.mp3").await?;
    let response = client
        .transcribe(CreateTranscriptionRequest {
            model: "openai/whisper-1".into(),
            file: audio_bytes,
            filename: "audio.mp3".into(),
            ..Default::default()
        })
        .await?;

    println!("{}", response.text);
    Ok(())
}
```
