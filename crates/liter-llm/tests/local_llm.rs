//! Integration tests against local LLM providers (Ollama).
//!
//! These tests require a running Ollama instance with models pulled.
//! Start with: `task local:up`
//! Run with: `cargo test -p liter-llm --test local_llm -- --ignored`

mod common;

use futures_util::StreamExt;
use liter_llm::{
    ChatCompletionRequest, ClientConfigBuilder, DefaultClient, EmbeddingInput, EmbeddingRequest, LlmClient,
};

const OLLAMA_CHAT_MODEL: &str = "ollama/qwen2:0.5b";
const OLLAMA_EMBED_MODEL: &str = "ollama/all-minilm";

/// Check whether an Ollama instance is reachable.
async fn is_ollama_available() -> bool {
    let base = std::env::var("OLLAMA_BASE_URL").unwrap_or_else(|_| "http://localhost:11434".into());
    reqwest::get(format!("{base}/v1/models")).await.is_ok()
}

fn ollama_client(model_hint: &str) -> DefaultClient {
    let config = ClientConfigBuilder::new("").max_retries(2).build();
    DefaultClient::new(config, Some(model_hint)).expect("failed to build Ollama client")
}

fn simple_chat_request(model: &str) -> ChatCompletionRequest {
    serde_json::from_value(serde_json::json!({
        "model": model,
        "messages": [{"role": "user", "content": "Say hello in one word."}],
        "max_tokens": 16,
    }))
    .expect("failed to build chat request from JSON")
}

fn simple_embed_request(model: &str) -> EmbeddingRequest {
    EmbeddingRequest {
        model: model.into(),
        input: EmbeddingInput::Single("hello world".into()),
        encoding_format: None,
        dimensions: None,
        user: None,
    }
}

#[tokio::test]
#[ignore]
async fn local_chat_ollama() {
    if !is_ollama_available().await {
        eprintln!("SKIP: Ollama not available, skipping");
        return;
    }

    let client = ollama_client(OLLAMA_CHAT_MODEL);
    let resp = client.chat(simple_chat_request(OLLAMA_CHAT_MODEL)).await.unwrap();

    assert!(!resp.choices.is_empty(), "should have at least one choice");
    let choice = &resp.choices[0];
    assert!(
        choice.message.content.as_ref().is_some_and(|c| !c.is_empty()),
        "first choice content should be non-empty"
    );
    assert!(choice.finish_reason.is_some(), "finish_reason should be present");
    assert!(!resp.model.is_empty(), "model field should be non-empty");
}

#[tokio::test]
#[ignore]
async fn local_stream_ollama() {
    if !is_ollama_available().await {
        eprintln!("SKIP: Ollama not available, skipping");
        return;
    }

    let client = ollama_client(OLLAMA_CHAT_MODEL);
    let mut stream = client
        .chat_stream(simple_chat_request(OLLAMA_CHAT_MODEL))
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
#[ignore]
async fn local_embed_ollama() {
    if !is_ollama_available().await {
        eprintln!("SKIP: Ollama not available, skipping");
        return;
    }

    let client = ollama_client(OLLAMA_EMBED_MODEL);
    let resp = client.embed(simple_embed_request(OLLAMA_EMBED_MODEL)).await.unwrap();

    assert!(!resp.data.is_empty(), "should have embedding data");
    assert!(!resp.data[0].embedding.is_empty(), "embedding should have dimensions");
    assert!(!resp.model.is_empty(), "model field should be non-empty");
}

#[tokio::test]
#[ignore]
async fn local_list_models_ollama() {
    if !is_ollama_available().await {
        eprintln!("SKIP: Ollama not available, skipping");
        return;
    }

    let client = ollama_client(OLLAMA_CHAT_MODEL);
    let resp = client.list_models().await.unwrap();

    assert!(!resp.data.is_empty(), "should list at least one model");
    assert!(!resp.data[0].id.is_empty(), "first model id should be non-empty");
}
