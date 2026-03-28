use futures_util::StreamExt;
use liter_llm::{ClientConfigBuilder, DefaultClient, LiterLlmError, LlmClient};

use super::{google_ai_client, require_env, simple_chat_request, simple_embed_request};

#[tokio::test]
async fn chat_basic() {
    let key = require_env!("GEMINI_API_KEY");
    let client = google_ai_client(&key);

    let resp = client.chat(simple_chat_request("gemini-2.5-flash-lite")).await.unwrap();

    // Gemini doesn't include model name in responses — skip model field check.
    assert!(!resp.choices.is_empty(), "google_ai: choices should not be empty");
    assert!(
        resp.choices[0].message.content.as_ref().is_some_and(|c| !c.is_empty()),
        "google_ai: first choice content should be non-empty"
    );
    assert!(
        resp.choices[0].finish_reason.is_some(),
        "google_ai: finish_reason should be present"
    );
}

#[tokio::test]
async fn chat_stream() {
    let key = require_env!("GEMINI_API_KEY");
    let client = google_ai_client(&key);

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
    let key = require_env!("GEMINI_API_KEY");
    let client = google_ai_client(&key);

    let resp = client
        .embed(simple_embed_request("gemini/gemini-embedding-001"))
        .await
        .unwrap();

    assert!(!resp.data.is_empty(), "embedding data should not be empty");
    assert!(
        !resp.data[0].embedding.is_empty(),
        "embedding vector should not be empty"
    );
}

#[tokio::test]
async fn list_models() {
    let key = require_env!("GEMINI_API_KEY");
    let client = google_ai_client(&key);

    let resp = client.list_models().await.unwrap();

    assert!(!resp.data.is_empty(), "models list should not be empty");
    assert!(!resp.data[0].id.is_empty(), "first model id should be non-empty");
}

#[tokio::test]
async fn error_invalid_key() {
    let _key = require_env!("GEMINI_API_KEY");

    let config = ClientConfigBuilder::new("invalid-gemini-key-for-testing").build();
    let client = DefaultClient::new(config, Some("gemini/gemini-2.5-flash-lite")).unwrap();

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
