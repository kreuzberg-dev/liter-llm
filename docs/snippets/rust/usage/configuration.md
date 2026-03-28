```rust
use liter_llm::{
    ChatCompletionRequest, ClientConfigBuilder, DefaultClient, LlmClient,
    Message, UserContent, UserMessage,
};
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = ClientConfigBuilder::new("sk-...".to_string()) // or std::env::var("OPENAI_API_KEY")?
        .base_url("https://api.openai.com/v1")      // override provider base URL
        .max_retries(3)                               // retry on transient failures
        .timeout(Duration::from_secs(60))             // request timeout
        .build();
    let client = DefaultClient::new(config, Some("openai/gpt-4o"))?; // pre-resolve provider

    let request = ChatCompletionRequest {
        model: "openai/gpt-4o".into(),
        messages: vec![Message::User(UserMessage {
            content: UserContent::Text("Hello!".into()),
            name: None,
        })],
        ..Default::default()
    };

    let response = client.chat(request).await?;
    if let Some(choice) = response.choices.first() {
        println!("{}", choice.message.content.as_deref().unwrap_or(""));
    }
    Ok(())
}
```
