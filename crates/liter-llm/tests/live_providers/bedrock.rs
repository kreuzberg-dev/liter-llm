use futures_util::StreamExt;
use liter_llm::LlmClient;

use super::{bedrock_client, require_env, simple_chat_request};

#[tokio::test]
async fn chat_basic() {
    let _key = require_env!("AWS_ACCESS_KEY_ID");
    let client = bedrock_client();

    let resp = client
        .chat(simple_chat_request("bedrock/us.anthropic.claude-sonnet-4-6"))
        .await
        .unwrap();

    assert!(!resp.choices.is_empty(), "bedrock: choices should not be empty");
    assert!(
        resp.choices[0].message.content.as_ref().is_some_and(|c| !c.is_empty()),
        "bedrock: first choice content should be non-empty"
    );
    assert!(
        resp.choices[0].finish_reason.is_some(),
        "bedrock: finish_reason should be present"
    );
}

#[tokio::test]
async fn chat_stream() {
    let _key = require_env!("AWS_ACCESS_KEY_ID");
    let client = bedrock_client();

    let mut stream = client
        .chat_stream(simple_chat_request("bedrock/us.anthropic.claude-sonnet-4-6"))
        .await
        .unwrap();

    let mut content = String::new();
    let mut chunk_count = 0u32;
    let mut saw_finish = false;

    while let Some(result) = stream.next().await {
        let chunk = result.unwrap();
        chunk_count += 1;
        if let Some(choice) = chunk.choices.first() {
            if let Some(text) = &choice.delta.content {
                content.push_str(text);
            }
            if choice.finish_reason.is_some() {
                saw_finish = true;
            }
        }
        if chunk_count > 200 {
            break;
        }
    }

    assert!(chunk_count >= 1, "should receive at least 1 chunk");
    assert!(!content.is_empty(), "concatenated content should be non-empty");
    assert!(saw_finish, "should see a finish_reason in the stream");
}
