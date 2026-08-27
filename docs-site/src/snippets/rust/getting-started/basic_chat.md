---
id: readme_rust_basic_chat
language: rust
target: rust
level: syntax
requires: []
side_effect: network
---

Send a message to any provider using the `provider/model` prefix.

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
        let text = choice.message.content.as_ref().and_then(|content| content.as_text());
        println!("{}", text.unwrap_or_default());
    }
    Ok(())
}
```
