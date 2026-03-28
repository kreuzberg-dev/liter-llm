```rust
use futures::StreamExt;
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
            content: UserContent::Text("Explain quantum computing briefly".into()),
            name: None,
        })],
        ..Default::default()
    };

    let mut stream = client.chat_stream(request).await?;
    let mut full_text = String::new();
    while let Some(chunk) = stream.next().await {
        let chunk = chunk?;
        if let Some(choice) = chunk.choices.first() {
            if let Some(content) = &choice.delta.content {
                full_text.push_str(content);
                print!("{content}");
            }
        }
    }
    println!();
    println!("\nFull response length: {} characters", full_text.len());
    Ok(())
}
```
