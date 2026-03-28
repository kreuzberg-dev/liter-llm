```rust
use liter_llm::{
    ChatCompletionRequest, ClientConfigBuilder, DefaultClient, FunctionDefinition,
    LlmClient, Message, Tool, UserContent, UserMessage,
};
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = ClientConfigBuilder::new(std::env::var("OPENAI_API_KEY")?)
        .build();
    let client = DefaultClient::new(config, Some("openai/gpt-4o"))?;

    let tools = vec![Tool {
        r#type: "function".into(),
        function: FunctionDefinition {
            name: "get_weather".into(),
            description: Some("Get the current weather for a location".into()),
            parameters: Some(json!({
                "type": "object",
                "properties": {
                    "location": { "type": "string", "description": "City name" }
                },
                "required": ["location"]
            })),
        },
    }];

    let request = ChatCompletionRequest {
        model: "openai/gpt-4o".into(),
        messages: vec![Message::User(UserMessage {
            content: UserContent::Text("What is the weather in Berlin?".into()),
            name: None,
        })],
        tools: Some(tools),
        ..Default::default()
    };

    let response = client.chat(request).await?;
    if let Some(tool_calls) = &response.choices[0].message.tool_calls {
        for call in tool_calls {
            println!("Tool: {}, Args: {}", call.function.name, call.function.arguments);
        }
    }
    Ok(())
}
```
