use futures_util::StreamExt;
use liter_llm::{ClientConfigBuilder, DefaultClient, LiterLlmError, LlmClient};

use super::{require_env, require_vertex, simple_chat_request, simple_embed_request, vertex_ai_client};

#[tokio::test]
async fn chat_basic() {
    let token = require_vertex!();
    let client = vertex_ai_client(&token);

    let resp = client.chat(simple_chat_request("gemini-2.5-flash-lite")).await.unwrap();

    // Vertex AI (like Google AI) doesn't include model name in responses.
    assert!(!resp.choices.is_empty(), "vertex_ai: choices should not be empty");
    assert!(
        resp.choices[0].message.content.as_ref().is_some_and(|c| !c.is_empty()),
        "vertex_ai: first choice content should be non-empty"
    );
    assert!(
        resp.choices[0].finish_reason.is_some(),
        "vertex_ai: finish_reason should be present"
    );
}

#[tokio::test]
async fn chat_stream() {
    let token = require_vertex!();
    let client = vertex_ai_client(&token);

    let mut stream = client
        .chat_stream(simple_chat_request("gemini-2.5-flash-lite"))
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

#[tokio::test]
async fn embed() {
    let token = require_vertex!();
    let client = vertex_ai_client(&token);

    let resp = client
        .embed(simple_embed_request("vertex_ai/gemini-embedding-001"))
        .await
        .unwrap();

    assert!(!resp.data.is_empty(), "embedding data should not be empty");
    assert!(
        !resp.data[0].embedding.is_empty(),
        "embedding vector should not be empty"
    );
}

#[tokio::test]
async fn error_invalid_token() {
    let _token = require_vertex!();

    let config = ClientConfigBuilder::new("invalid-token-for-testing").build();
    let client = DefaultClient::new(config, Some("vertex_ai/gemini-2.5-flash-lite")).unwrap();

    let err = client
        .chat(simple_chat_request("gemini-2.5-flash-lite"))
        .await
        .unwrap_err();

    assert!(
        matches!(
            err,
            LiterLlmError::Authentication { .. } | LiterLlmError::BadRequest { .. }
        ),
        "expected Authentication or BadRequest error, got: {err:?}"
    );
}
