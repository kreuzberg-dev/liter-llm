use std::borrow::Cow;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::time::{SystemTime, UNIX_EPOCH};

use crate::provider::Provider;

/// Default base URL for the GitHub Copilot API.
pub const DEFAULT_API_BASE: &str = "https://api.githubcopilot.com";

/// Environment variable used to override the Copilot API base URL.
const API_BASE_ENV_VAR: &str = "GITHUB_COPILOT_API_BASE";

/// Model name prefix used for routing.
const MODEL_PREFIX: &str = "github_copilot/";

// ── Version constants ────────────────────────────────────────────────────────

const COPILOT_VERSION: &str = "0.26.7";
const EDITOR_VERSION: &str = "vscode/1.95.0";
const API_VERSION: &str = "2025-04-01";

// ── Static headers ───────────────────────────────────────────────────────────

/// Static headers required by the GitHub Copilot API on every request.
///
/// These identify the client as VS Code Chat to Copilot's backend and must
/// appear on all requests regardless of the model or payload.
static COPILOT_EXTRA_HEADERS: &[(&str, &str)] = &[
    ("copilot-integration-id", "vscode-chat"),
    ("editor-version", EDITOR_VERSION),
    ("editor-plugin-version", "copilot-chat/0.26.7"),
    ("user-agent", "GitHubCopilotChat/0.26.7"),
    ("openai-intent", "conversation-panel"),
    ("x-github-api-version", API_VERSION),
    ("x-vscode-user-agent-library-version", "electron-fetch"),
];

// Suppress the unused-constant warning: the values are embedded via string
// interpolation in the static slice above; Rust does not track that reference.
const _: () = {
    let _ = COPILOT_VERSION;
};

// ── UUID generation (no external deps) ──────────────────────────────────────

/// Generate a pseudo-random UUID v4 string without requiring the `uuid` crate.
///
/// Uses the current nanosecond timestamp hashed with the thread ID to produce
/// 128 bits of pseudo-entropy, then formats them in standard UUID v4 layout
/// with the version (`4xxx`) and variant (`8xxx`–`Bxxx`) bits set correctly.
///
/// This is not cryptographically random, but it is sufficiently unique for
/// per-request tracing headers where only collision avoidance matters.
fn generate_request_id() -> String {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_nanos())
        .unwrap_or(0);

    let thread_id = std::thread::current().id();

    let mut hasher = DefaultHasher::new();
    nanos.hash(&mut hasher);
    thread_id.hash(&mut hasher);
    let h1 = hasher.finish();

    // A second hash pass adds entropy independent of the first 64-bit word.
    h1.hash(&mut hasher);
    let h2 = hasher.finish();

    let b1 = h1.to_le_bytes();
    let b2 = h2.to_le_bytes();

    // UUID v4 layout:
    //   time_low          (32 bits) : b1[0..4]
    //   time_mid          (16 bits) : b1[4..6]
    //   version + hi      (16 bits) : "4" + b1[6..8] masked to 12 bits
    //   variant + clock   (16 bits) : 0x8000 | (b2[0..2] & 0x3FFF)
    //   node              (48 bits) : b2[2..8]
    let time_low = u32::from_le_bytes([b1[0], b1[1], b1[2], b1[3]]);
    let time_mid = u16::from_le_bytes([b1[4], b1[5]]);
    let version_hi = u16::from_le_bytes([b1[6], b1[7]]) & 0x0FFF;
    let variant_clock = (u16::from_le_bytes([b2[0], b2[1]]) & 0x3FFF) | 0x8000;
    let node = u64::from_le_bytes([b2[2], b2[3], b2[4], b2[5], b2[6], b2[7], 0, 0]) & 0x0000_FFFF_FFFF_FFFF;

    format!("{time_low:08x}-{time_mid:04x}-4{version_hi:03x}-{variant_clock:04x}-{node:012x}")
}

// ── Provider ─────────────────────────────────────────────────────────────────

/// GitHub Copilot provider.
///
/// GitHub Copilot exposes an OpenAI-compatible chat completions API at
/// `https://api.githubcopilot.com` (or a dynamic URL returned during the
/// OAuth token exchange).  The request/response format is identical to
/// OpenAI's, so no `transform_request` or `transform_response` overrides are
/// needed.
///
/// Key differences from a vanilla OpenAI provider:
/// - Several static headers identify the caller as VS Code Chat.
/// - Each request carries a unique `x-request-id` UUID.
/// - An `X-Initiator` header signals whether the turn was started by the user
///   or by an agent (tool/assistant message in the context).
/// - The base URL can be overridden at construction time (Copilot's OAuth flow
///   returns a dynamic endpoint URL in the token response).
///
/// # Construction
///
/// ```rust,ignore
/// // Use the default base URL.
/// let provider = GithubCopilotProvider::new();
///
/// // Override the base URL (e.g. from the OAuth token exchange).
/// let provider = GithubCopilotProvider::with_api_base("https://proxy.example.com".to_owned());
///
/// // Read the base URL from the GITHUB_COPILOT_API_BASE environment variable.
/// let provider = GithubCopilotProvider::from_env();
/// ```
pub struct GithubCopilotProvider {
    /// Base URL for the Copilot API. Defaults to [`DEFAULT_API_BASE`].
    api_base: String,
}

impl GithubCopilotProvider {
    /// Create a provider using the default Copilot API base URL.
    #[must_use]
    pub fn new() -> Self {
        Self {
            api_base: DEFAULT_API_BASE.to_owned(),
        }
    }

    /// Create a provider with a custom base URL.
    ///
    /// Use this when the Copilot OAuth token exchange returns a dynamic
    /// endpoint URL that differs from the default.
    #[must_use]
    #[allow(dead_code)] // public API — used by callers that obtain a dynamic URL from the OAuth flow
    pub fn with_api_base(base: String) -> Self {
        Self { api_base: base }
    }

    /// Create a provider by reading the base URL from the
    /// `GITHUB_COPILOT_API_BASE` environment variable.
    ///
    /// Falls back to [`DEFAULT_API_BASE`] when the variable is unset or empty.
    #[must_use]
    pub fn from_env() -> Self {
        let api_base = std::env::var(API_BASE_ENV_VAR)
            .ok()
            .filter(|s| !s.is_empty())
            .unwrap_or_else(|| DEFAULT_API_BASE.to_owned());

        Self { api_base }
    }
}

impl Default for GithubCopilotProvider {
    fn default() -> Self {
        Self::new()
    }
}

impl Provider for GithubCopilotProvider {
    fn name(&self) -> &str {
        "github_copilot"
    }

    fn base_url(&self) -> &str {
        &self.api_base
    }

    fn auth_header<'a>(&'a self, api_key: &'a str) -> Option<(Cow<'static, str>, Cow<'a, str>)> {
        Some((Cow::Borrowed("Authorization"), Cow::Owned(format!("Bearer {api_key}"))))
    }

    /// Static headers required by the GitHub Copilot API on every request.
    ///
    /// These identify the VS Code Chat client to Copilot's backend.
    fn extra_headers(&self) -> &'static [(&'static str, &'static str)] {
        COPILOT_EXTRA_HEADERS
    }

    /// Compute per-request dynamic headers.
    ///
    /// Two headers are generated here:
    ///
    /// - `x-request-id`: A fresh pseudo-random UUID v4 generated for every
    ///   call, used by Copilot's backend for distributed tracing.
    ///
    /// - `X-Initiator`: Either `"agent"` (when the conversation context
    ///   contains any message with role `"tool"` or `"assistant"`, indicating
    ///   an ongoing agentic turn) or `"user"` (for a fresh human-initiated
    ///   turn).  Copilot uses this to apply different rate-limiting and
    ///   routing policies.
    fn dynamic_headers(&self, body: &serde_json::Value) -> Vec<(String, String)> {
        let request_id = generate_request_id();

        let initiator = determine_initiator(body);

        vec![
            ("x-request-id".to_owned(), request_id),
            ("X-Initiator".to_owned(), initiator.to_owned()),
        ]
    }

    fn matches_model(&self, model: &str) -> bool {
        model.starts_with(MODEL_PREFIX)
    }

    fn strip_model_prefix<'m>(&self, model: &'m str) -> &'m str {
        model.strip_prefix(MODEL_PREFIX).unwrap_or(model)
    }

    // All endpoint paths use OpenAI-compatible defaults; no overrides needed.
}

/// Determine the `X-Initiator` value from the request body.
///
/// Returns `"agent"` when any message in `body["messages"]` has a role of
/// `"tool"` or `"assistant"`, which indicates that the turn is part of an
/// ongoing multi-step agentic interaction.  Returns `"user"` otherwise.
fn determine_initiator(body: &serde_json::Value) -> &'static str {
    let messages = match body.get("messages").and_then(|m| m.as_array()) {
        Some(msgs) => msgs,
        None => return "user",
    };

    for message in messages {
        let role = message.get("role").and_then(|r| r.as_str()).unwrap_or("");
        if role == "tool" || role == "assistant" {
            return "agent";
        }
    }

    "user"
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use serde_json::json;
    use serial_test::serial;

    use super::*;

    // ── Basic provider metadata ──────────────────────────────────────────────

    #[test]
    fn test_name() {
        let provider = GithubCopilotProvider::new();
        assert_eq!(provider.name(), "github_copilot");
    }

    #[test]
    fn test_base_url_default() {
        let provider = GithubCopilotProvider::new();
        assert_eq!(provider.base_url(), "https://api.githubcopilot.com");
    }

    #[test]
    fn test_base_url_custom() {
        let provider = GithubCopilotProvider::with_api_base("https://proxy.example.com".to_owned());
        assert_eq!(provider.base_url(), "https://proxy.example.com");
    }

    // ── from_env ─────────────────────────────────────────────────────────────

    #[test]
    #[serial]
    fn test_from_env_uses_default_when_unset() {
        // Ensure the variable is not set for this test.
        // SAFETY: tests in this group are serialised via #[serial]; no concurrent env access.
        unsafe { std::env::remove_var("GITHUB_COPILOT_API_BASE") };
        let provider = GithubCopilotProvider::from_env();
        assert_eq!(provider.base_url(), DEFAULT_API_BASE);
    }

    #[test]
    #[serial]
    fn test_from_env_reads_custom_base_url() {
        // SAFETY: tests in this group are serialised via #[serial]; no concurrent env access.
        unsafe { std::env::set_var("GITHUB_COPILOT_API_BASE", "https://custom.copilot.test") };
        let provider = GithubCopilotProvider::from_env();
        // SAFETY: tests in this group are serialised via #[serial]; no concurrent env access.
        unsafe { std::env::remove_var("GITHUB_COPILOT_API_BASE") };
        assert_eq!(provider.base_url(), "https://custom.copilot.test");
    }

    #[test]
    #[serial]
    fn test_from_env_falls_back_on_empty_value() {
        // SAFETY: tests in this group are serialised via #[serial]; no concurrent env access.
        unsafe { std::env::set_var("GITHUB_COPILOT_API_BASE", "") };
        let provider = GithubCopilotProvider::from_env();
        // SAFETY: tests in this group are serialised via #[serial]; no concurrent env access.
        unsafe { std::env::remove_var("GITHUB_COPILOT_API_BASE") };
        assert_eq!(provider.base_url(), DEFAULT_API_BASE);
    }

    // ── Model routing ────────────────────────────────────────────────────────

    #[test]
    fn test_matches_model() {
        let provider = GithubCopilotProvider::new();
        assert!(provider.matches_model("github_copilot/gpt-4o"));
        assert!(provider.matches_model("github_copilot/claude-3.5-sonnet"));
        assert!(provider.matches_model("github_copilot/o3-mini"));
    }

    #[test]
    fn test_does_not_match_other_providers() {
        let provider = GithubCopilotProvider::new();
        assert!(!provider.matches_model("openai/gpt-4o"));
        assert!(!provider.matches_model("gpt-4o"));
        assert!(!provider.matches_model("claude-3.5-sonnet"));
        assert!(!provider.matches_model("anthropic/claude-3.5-sonnet"));
    }

    #[test]
    fn test_strip_model_prefix() {
        let provider = GithubCopilotProvider::new();
        assert_eq!(provider.strip_model_prefix("github_copilot/gpt-4o"), "gpt-4o");
    }

    #[test]
    fn test_strip_model_prefix_no_prefix() {
        let provider = GithubCopilotProvider::new();
        assert_eq!(provider.strip_model_prefix("gpt-4o"), "gpt-4o");
    }

    // ── Auth header ──────────────────────────────────────────────────────────

    #[test]
    fn test_auth_header() {
        let provider = GithubCopilotProvider::new();
        let (name, value) = provider
            .auth_header("ghs_test_token_123")
            .expect("should return an auth header");
        assert_eq!(name, "Authorization");
        assert_eq!(value, "Bearer ghs_test_token_123");
    }

    // ── Static extra headers ─────────────────────────────────────────────────

    #[test]
    fn test_extra_headers() {
        let provider = GithubCopilotProvider::new();
        let headers = provider.extra_headers();

        // All 7 required static headers must be present.
        let find = |key: &str| headers.iter().find(|(k, _)| *k == key).map(|(_, v)| *v);

        assert_eq!(find("copilot-integration-id"), Some("vscode-chat"));
        assert_eq!(find("editor-version"), Some("vscode/1.95.0"));
        assert_eq!(find("editor-plugin-version"), Some("copilot-chat/0.26.7"));
        assert_eq!(find("user-agent"), Some("GitHubCopilotChat/0.26.7"));
        assert_eq!(find("openai-intent"), Some("conversation-panel"));
        assert_eq!(find("x-github-api-version"), Some("2025-04-01"));
        assert_eq!(find("x-vscode-user-agent-library-version"), Some("electron-fetch"));

        assert_eq!(headers.len(), 7, "expected exactly 7 static headers");
    }

    // ── Dynamic headers ──────────────────────────────────────────────────────

    #[test]
    fn test_dynamic_headers_user() {
        let provider = GithubCopilotProvider::new();
        let body = json!({
            "model": "github_copilot/gpt-4o",
            "messages": [
                {"role": "user", "content": "Hello!"}
            ]
        });

        let headers = provider.dynamic_headers(&body);
        let initiator = headers
            .iter()
            .find(|(k, _)| k == "X-Initiator")
            .map(|(_, v)| v.as_str());

        assert_eq!(initiator, Some("user"));
    }

    #[test]
    fn test_dynamic_headers_agent_with_tool_role() {
        let provider = GithubCopilotProvider::new();
        let body = json!({
            "model": "github_copilot/gpt-4o",
            "messages": [
                {"role": "user", "content": "Run the tool."},
                {"role": "assistant", "content": null, "tool_calls": []},
                {"role": "tool", "content": "tool result", "tool_call_id": "abc"}
            ]
        });

        let headers = provider.dynamic_headers(&body);
        let initiator = headers
            .iter()
            .find(|(k, _)| k == "X-Initiator")
            .map(|(_, v)| v.as_str());

        assert_eq!(initiator, Some("agent"));
    }

    #[test]
    fn test_dynamic_headers_agent_with_assistant_role() {
        let provider = GithubCopilotProvider::new();
        let body = json!({
            "messages": [
                {"role": "user", "content": "Hi"},
                {"role": "assistant", "content": "Hello"}
            ]
        });

        let headers = provider.dynamic_headers(&body);
        let initiator = headers
            .iter()
            .find(|(k, _)| k == "X-Initiator")
            .map(|(_, v)| v.as_str());

        assert_eq!(initiator, Some("agent"));
    }

    #[test]
    fn test_dynamic_headers_user_when_no_messages() {
        let provider = GithubCopilotProvider::new();
        let body = json!({ "model": "github_copilot/gpt-4o" });

        let headers = provider.dynamic_headers(&body);
        let initiator = headers
            .iter()
            .find(|(k, _)| k == "X-Initiator")
            .map(|(_, v)| v.as_str());

        assert_eq!(initiator, Some("user"));
    }

    #[test]
    fn test_dynamic_headers_request_id_present_and_valid_uuid() {
        let provider = GithubCopilotProvider::new();
        let body = json!({ "messages": [{"role": "user", "content": "hi"}] });

        let headers = provider.dynamic_headers(&body);
        let request_id = headers
            .iter()
            .find(|(k, _)| k == "x-request-id")
            .map(|(_, v)| v.as_str())
            .expect("x-request-id header must be present");

        // UUID v4 canonical form: xxxxxxxx-xxxx-4xxx-[89ab]xxx-xxxxxxxxxxxx
        // Length check: 8-4-4-4-12 = 32 hex + 4 dashes = 36 chars.
        assert_eq!(request_id.len(), 36, "request id must be 36 characters");

        let parts: Vec<&str> = request_id.split('-').collect();
        assert_eq!(parts.len(), 5, "UUID must have 5 dash-separated groups");
        assert_eq!(parts[0].len(), 8);
        assert_eq!(parts[1].len(), 4);
        assert_eq!(parts[2].len(), 4);
        assert_eq!(parts[3].len(), 4);
        assert_eq!(parts[4].len(), 12);

        // Version nibble must be '4'.
        assert_eq!(&parts[2][0..1], "4", "third group must start with '4' (UUID version 4)");

        // Variant bits: first nibble of fourth group must be 8, 9, a, or b.
        let variant_nibble = parts[3].chars().next().expect("fourth group is non-empty");
        assert!(
            matches!(variant_nibble, '8' | '9' | 'a' | 'b'),
            "fourth group must start with 8, 9, a or b (RFC 4122 variant); got '{variant_nibble}'"
        );

        // All characters must be valid lowercase hex or dashes.
        for ch in request_id.chars() {
            assert!(
                ch.is_ascii_hexdigit() || ch == '-',
                "unexpected character '{ch}' in request id"
            );
        }
    }

    #[test]
    fn test_dynamic_headers_request_id_unique_per_call() {
        let provider = GithubCopilotProvider::new();
        let body = json!({ "messages": [{"role": "user", "content": "hi"}] });

        let id1 = provider
            .dynamic_headers(&body)
            .into_iter()
            .find(|(k, _)| k == "x-request-id")
            .map(|(_, v)| v)
            .expect("x-request-id must be present");

        // Inject a small delay so the nanosecond timestamp differs.
        std::thread::sleep(std::time::Duration::from_nanos(100));

        let id2 = provider
            .dynamic_headers(&body)
            .into_iter()
            .find(|(k, _)| k == "x-request-id")
            .map(|(_, v)| v)
            .expect("x-request-id must be present");

        assert_ne!(id1, id2, "consecutive request IDs must differ");
    }

    // ── Endpoint paths (OpenAI defaults) ─────────────────────────────────────

    #[test]
    fn test_endpoint_paths_are_openai_compatible() {
        let provider = GithubCopilotProvider::new();
        assert_eq!(provider.chat_completions_path(), "/chat/completions");
        assert_eq!(provider.embeddings_path(), "/embeddings");
        assert_eq!(provider.models_path(), "/models");
    }
}
