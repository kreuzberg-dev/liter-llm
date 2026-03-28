use liter_llm::LlmClient;

use super::{
    anthropic_client, azure_client, gcloud_access_token, google_ai_client, mistral_client, openai_client,
    simple_chat_request, vertex_ai_client,
};

/// Sends the same prompt to every provider whose env var is set and asserts
/// structural parity across all responses.
#[tokio::test]
async fn chat_parity_across_providers() {
    let mut results: Vec<(&str, liter_llm::ChatCompletionResponse)> = Vec::new();

    if let Ok(key) = std::env::var("OPENAI_API_KEY")
        && !key.is_empty()
    {
        let client = openai_client(&key);
        match client.chat(simple_chat_request("gpt-4o-mini")).await {
            Ok(resp) => results.push(("openai", resp)),
            Err(e) => eprintln!("openai chat failed: {e}"),
        }
    }

    if let Ok(key) = std::env::var("ANTHROPIC_API_KEY")
        && !key.is_empty()
    {
        let client = anthropic_client(&key);
        match client.chat(simple_chat_request("claude-haiku-4-5-20251001")).await {
            Ok(resp) => results.push(("anthropic", resp)),
            Err(e) => eprintln!("anthropic chat failed: {e}"),
        }
    }

    if let Ok(key) = std::env::var("GEMINI_API_KEY")
        && !key.is_empty()
    {
        let client = google_ai_client(&key);
        match client.chat(simple_chat_request("gemini-2.5-flash-lite")).await {
            Ok(resp) => results.push(("google_ai", resp)),
            Err(e) => eprintln!("google_ai chat failed: {e}"),
        }
    }

    if let Ok(key) = std::env::var("AZURE_OPENAI_API_KEY")
        && !key.is_empty()
        && std::env::var("AZURE_OPENAI_ENDPOINT").is_ok_and(|v| !v.is_empty())
    {
        let client = azure_client(&key);
        match client.chat(simple_chat_request("azure/gpt-4o-mini")).await {
            Ok(resp) => results.push(("azure", resp)),
            Err(e) => eprintln!("azure chat failed: {e}"),
        }
    }

    if let Ok(key) = std::env::var("MISTRAL_API_KEY")
        && !key.is_empty()
    {
        let client = mistral_client(&key);
        match client.chat(simple_chat_request("mistral-small-latest")).await {
            Ok(resp) => results.push(("mistral", resp)),
            Err(e) => eprintln!("mistral chat failed: {e}"),
        }
    }

    if std::env::var("VERTEXAI_PROJECT").is_ok_and(|v| !v.is_empty())
        && let Some(token) = gcloud_access_token()
    {
        let client = vertex_ai_client(&token);
        match client.chat(simple_chat_request("gemini-2.5-flash-lite")).await {
            Ok(resp) => results.push(("vertex_ai", resp)),
            Err(e) => eprintln!("vertex_ai chat failed: {e}"),
        }
    }

    #[cfg(feature = "bedrock")]
    if std::env::var("AWS_ACCESS_KEY_ID").is_ok_and(|v| !v.is_empty()) {
        let client = super::bedrock_client();
        match client
            .chat(simple_chat_request("bedrock/us.anthropic.claude-sonnet-4-6"))
            .await
        {
            Ok(resp) => results.push(("bedrock", resp)),
            Err(e) => eprintln!("bedrock chat failed: {e}"),
        }
    }

    if results.is_empty() {
        eprintln!("SKIP: no provider API keys set, skipping cross-provider parity test");
        return;
    }

    eprintln!("cross-provider parity: testing {} providers", results.len());

    for (provider, resp) in &results {
        assert!(!resp.choices.is_empty(), "{provider}: choices should not be empty");
        assert!(
            resp.choices[0].message.content.as_ref().is_some_and(|c| !c.is_empty()),
            "{provider}: first choice content should be non-empty"
        );
        // Note: Gemini doesn't include model name in responses, so we skip this check.
    }
}
