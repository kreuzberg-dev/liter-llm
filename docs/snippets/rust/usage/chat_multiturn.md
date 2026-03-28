```rust
use liter_llm::{
    ChatCompletionRequest, ClientConfigBuilder, DefaultClient, LlmClient,
    Message, UserContent, UserMessage, AssistantMessage, SystemMessage,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = ClientConfigBuilder::new(std::env::var("OPENAI_API_KEY")?)
        .build();
    let client = DefaultClient::new(config, Some("openai/gpt-4o"))?;

    let mut messages = vec![
        Message::System(SystemMessage {
            content: "You are a helpful assistant.".into(),
            name: None,
        }),
        Message::User(UserMessage {
            content: UserContent::Text("What is the capital of France?".into()),
            name: None,
        }),
    ];

    let request = ChatCompletionRequest {
        model: "openai/gpt-4o".into(),
        messages: messages.clone(),
        ..Default::default()
    };
    let response = client.chat(request).await?;
    let content = response.choices[0]
        .message
        .content
        .clone()
        .unwrap_or_default();
    println!("Assistant: {content}");

    // Continue the conversation
    messages.push(Message::Assistant(AssistantMessage {
        content: Some(content),
        ..Default::default()
    }));
    messages.push(Message::User(UserMessage {
        content: UserContent::Text("What about Germany?".into()),
        name: None,
    }));

    let request = ChatCompletionRequest {
        model: "openai/gpt-4o".into(),
        messages,
        ..Default::default()
    };
    let response = client.chat(request).await?;
    if let Some(choice) = response.choices.first() {
        println!("Assistant: {}", choice.message.content.as_deref().unwrap_or(""));
    }

    // Token usage
    if let Some(usage) = &response.usage {
        println!("Tokens: {} in, {} out", usage.prompt_tokens, usage.completion_tokens);
    }
    Ok(())
}
```
