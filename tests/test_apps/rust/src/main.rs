//! Smoke tests for the liter-llm published crate.
//!
//! Validates the published crate works against real LLM APIs.
//! Requires API keys in environment variables or .env file at repo root.

use futures::StreamExt;
use liter_llm::{
    ChatCompletionRequest, ClientConfigBuilder, EmbeddingRequest, ManagedClient, Message,
    UserMessage,
};
use std::path::PathBuf;

// ─── .env loader ────────────────────────────────────────────────────────────

fn load_dotenv() {
    let mut dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    // Walk up to repo root (three levels: src -> rust -> test_apps -> tests -> repo)
    for _ in 0..4 {
        dir = match dir.parent() {
            Some(p) => p.to_path_buf(),
            None => break,
        };
        let env_file = dir.join(".env");
        if env_file.exists() {
            if let Ok(contents) = std::fs::read_to_string(&env_file) {
                for line in contents.lines() {
                    let trimmed = line.trim();
                    if trimmed.is_empty() || trimmed.starts_with('#') {
                        continue;
                    }
                    if let Some((key, value)) = trimmed.split_once('=') {
                        let key = key.trim();
                        let value = value.trim();
                        if std::env::var(key).is_err() {
                            std::env::set_var(key, value);
                        }
                    }
                }
            }
            break;
        }
    }
}

fn env_key(name: &str) -> Option<String> {
    std::env::var(name).ok().filter(|v| !v.is_empty())
}

// ─── Test runner ────────────────────────────────────────────────────────────

struct SmokeTest {
    passed: u32,
    failed: u32,
    skipped: u32,
}

impl SmokeTest {
    fn new() -> Self {
        Self {
            passed: 0,
            failed: 0,
            skipped: 0,
        }
    }

    async fn run<F, Fut>(&mut self, name: &str, test_fn: F)
    where
        F: FnOnce() -> Fut,
        Fut: std::future::Future<Output = Result<Option<String>, Box<dyn std::error::Error>>>,
    {
        print!("  {name}... ");
        match test_fn().await {
            Ok(None) => {
                println!("SKIP");
                self.skipped += 1;
            }
            Ok(Some(_)) => {
                println!("PASS");
                self.passed += 1;
            }
            Err(e) => {
                println!("FAIL: {e}");
                self.failed += 1;
            }
        }
    }

    fn summary(&self) -> i32 {
        let total = self.passed + self.failed + self.skipped;
        println!();
        println!("{}", "=".repeat(60));
        println!(
            "Results: {} passed, {} failed, {} skipped ({} total)",
            self.passed, self.failed, self.skipped, total
        );
        if self.failed > 0 { 1 } else { 0 }
    }
}

// ─── Test cases ─────────────────────────────────────────────────────────────

async fn test_chat_openai() -> Result<Option<String>, Box<dyn std::error::Error>> {
    let key = match env_key("OPENAI_API_KEY") {
        Some(k) => k,
        None => return Ok(None),
    };
    let config = ClientConfigBuilder::new().api_key(key).build();
    let client = ManagedClient::new(config, Some("openai/gpt-4o-mini"))?;
    let req = ChatCompletionRequest {
        model: "openai/gpt-4o-mini".into(),
        messages: vec![Message::User(UserMessage {
            content: "Say hello in one word.".into(),
            name: None,
        })],
        max_tokens: Some(10),
        ..Default::default()
    };
    let r = client.chat(req).await?;
    assert!(!r.choices.is_empty(), "no choices in response");
    assert!(
        r.choices[0].message.content.is_some(),
        "empty content"
    );
    assert!(r.usage.is_some(), "no usage data");
    assert!(
        r.usage.as_ref().is_some_and(|u| u.total_tokens > 0),
        "zero tokens"
    );
    Ok(Some("ok".into()))
}

async fn test_chat_anthropic() -> Result<Option<String>, Box<dyn std::error::Error>> {
    let key = match env_key("ANTHROPIC_API_KEY") {
        Some(k) => k,
        None => return Ok(None),
    };
    let config = ClientConfigBuilder::new().api_key(key).build();
    let client = ManagedClient::new(config, Some("anthropic/claude-3-5-haiku-20241022"))?;
    let req = ChatCompletionRequest {
        model: "anthropic/claude-3-5-haiku-20241022".into(),
        messages: vec![Message::User(UserMessage {
            content: "Say hello in one word.".into(),
            name: None,
        })],
        max_tokens: Some(10),
        ..Default::default()
    };
    let r = client.chat(req).await?;
    assert!(!r.choices.is_empty(), "no choices");
    assert!(r.choices[0].message.content.is_some(), "empty content");
    Ok(Some("ok".into()))
}

async fn test_chat_gemini() -> Result<Option<String>, Box<dyn std::error::Error>> {
    let key = match env_key("GEMINI_API_KEY") {
        Some(k) => k,
        None => return Ok(None),
    };
    let config = ClientConfigBuilder::new().api_key(key).build();
    let client = ManagedClient::new(config, Some("google/gemini-2.5-flash-lite"))?;
    let req = ChatCompletionRequest {
        model: "google/gemini-2.5-flash-lite".into(),
        messages: vec![Message::User(UserMessage {
            content: "Say hello in one word.".into(),
            name: None,
        })],
        max_tokens: Some(10),
        ..Default::default()
    };
    let r = client.chat(req).await?;
    assert!(!r.choices.is_empty(), "no choices");
    assert!(r.choices[0].message.content.is_some(), "empty content");
    Ok(Some("ok".into()))
}

async fn test_streaming_openai() -> Result<Option<String>, Box<dyn std::error::Error>> {
    let key = match env_key("OPENAI_API_KEY") {
        Some(k) => k,
        None => return Ok(None),
    };
    let config = ClientConfigBuilder::new().api_key(key).build();
    let client = ManagedClient::new(config, Some("openai/gpt-4o-mini"))?;
    let req = ChatCompletionRequest {
        model: "openai/gpt-4o-mini".into(),
        messages: vec![Message::User(UserMessage {
            content: "Count from 1 to 5.".into(),
            name: None,
        })],
        max_tokens: Some(50),
        ..Default::default()
    };
    let mut stream = client.chat_stream(req).await?;
    let mut count = 0u32;
    while let Some(chunk) = stream.next().await {
        let _ = chunk;
        count += 1;
    }
    assert!(count > 0, "no chunks received");
    Ok(Some("ok".into()))
}

async fn test_embed_openai() -> Result<Option<String>, Box<dyn std::error::Error>> {
    let key = match env_key("OPENAI_API_KEY") {
        Some(k) => k,
        None => return Ok(None),
    };
    let config = ClientConfigBuilder::new().api_key(key).build();
    let client = ManagedClient::new(config, Some("openai/text-embedding-3-small"))?;
    let req = EmbeddingRequest {
        model: "openai/text-embedding-3-small".into(),
        input: vec!["Hello, world!".into()],
        ..Default::default()
    };
    let r = client.embed(req).await?;
    assert!(!r.data.is_empty(), "no embeddings");
    assert!(!r.data[0].embedding.is_empty(), "empty embedding vector");
    Ok(Some("ok".into()))
}

async fn test_list_models_openai() -> Result<Option<String>, Box<dyn std::error::Error>> {
    let key = match env_key("OPENAI_API_KEY") {
        Some(k) => k,
        None => return Ok(None),
    };
    let config = ClientConfigBuilder::new().api_key(key).build();
    let client = ManagedClient::new(config, Some("openai/gpt-4o-mini"))?;
    let r = client.list_models().await?;
    assert!(!r.data.is_empty(), "no models returned");
    Ok(Some("ok".into()))
}

async fn test_provider_routing() -> Result<Option<String>, Box<dyn std::error::Error>> {
    let openai_key = match env_key("OPENAI_API_KEY") {
        Some(k) => k,
        None => return Ok(None),
    };
    let anthropic_key = match env_key("ANTHROPIC_API_KEY") {
        Some(k) => k,
        None => return Ok(None),
    };

    let messages = vec![Message::User(UserMessage {
        content: "Say hi.".into(),
        name: None,
    })];

    let config_openai = ClientConfigBuilder::new().api_key(openai_key).build();
    let client_openai = ManagedClient::new(config_openai, Some("openai/gpt-4o-mini"))?;
    let r1 = client_openai
        .chat(ChatCompletionRequest {
            model: "openai/gpt-4o-mini".into(),
            messages: messages.clone(),
            max_tokens: Some(5),
            ..Default::default()
        })
        .await?;
    assert!(!r1.choices.is_empty(), "OpenAI failed");

    let config_anthropic = ClientConfigBuilder::new().api_key(anthropic_key).build();
    let client_anthropic =
        ManagedClient::new(config_anthropic, Some("anthropic/claude-3-5-haiku-20241022"))?;
    let r2 = client_anthropic
        .chat(ChatCompletionRequest {
            model: "anthropic/claude-3-5-haiku-20241022".into(),
            messages,
            max_tokens: Some(5),
            ..Default::default()
        })
        .await?;
    assert!(!r2.choices.is_empty(), "Anthropic failed");

    Ok(Some("ok".into()))
}

async fn test_cache_memory() -> Result<Option<String>, Box<dyn std::error::Error>> {
    let key = match env_key("OPENAI_API_KEY") {
        Some(k) => k,
        None => return Ok(None),
    };
    let config = ClientConfigBuilder::new()
        .api_key(key)
        .cache_max_entries(10)
        .cache_ttl_seconds(60)
        .build();
    let client = ManagedClient::new(config, Some("openai/gpt-4o-mini"))?;

    let messages = vec![Message::User(UserMessage {
        content: "What is 2+2? Answer with just the number.".into(),
        name: None,
    })];

    let req1 = ChatCompletionRequest {
        model: "openai/gpt-4o-mini".into(),
        messages: messages.clone(),
        max_tokens: Some(5),
        ..Default::default()
    };
    let r1 = client.chat(req1).await?;

    let req2 = ChatCompletionRequest {
        model: "openai/gpt-4o-mini".into(),
        messages,
        max_tokens: Some(5),
        ..Default::default()
    };
    let r2 = client.chat(req2).await?;

    assert!(!r1.choices.is_empty(), "first request failed");
    assert!(!r2.choices.is_empty(), "second request failed");
    assert_eq!(
        r1.choices[0].message.content, r2.choices[0].message.content,
        "cache miss - responses differ"
    );
    Ok(Some("ok".into()))
}

// ─── Main ───────────────────────────────────────────────────────────────────

#[tokio::main]
async fn main() {
    load_dotenv();

    println!("liter-llm Smoke Tests (Rust)");
    println!("{}", "=".repeat(60));
    println!();

    let mut suite = SmokeTest::new();

    println!("Chat Completions:");
    suite.run("OpenAI gpt-4o-mini", test_chat_openai).await;
    suite.run("Anthropic claude-3-5-haiku", test_chat_anthropic).await;
    suite.run("Google gemini-2.5-flash-lite", test_chat_gemini).await;

    suite.run("OpenAI streaming", test_streaming_openai).await;
    suite.run("OpenAI text-embedding-3-small", test_embed_openai).await;
    suite.run("OpenAI list models", test_list_models_openai).await;
    suite.run("Multi-provider routing", test_provider_routing).await;
    suite.run("In-memory cache hit", test_cache_memory).await;

    std::process::exit(suite.summary());
}
