```rust
use liter_llm::{
    ChatCompletionRequest, ClientConfigBuilder, DefaultClient, LlmClient,
    Message, UserContent, UserMessage,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = ClientConfigBuilder::new(std::env::var("OPENAI_API_KEY")?)
        .build();
    let client = DefaultClient::new(config, Some("openai/gpt-4o"))?;

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
