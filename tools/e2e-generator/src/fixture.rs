use anyhow::{Context, Result, bail};
use camino::{Utf8Path, Utf8PathBuf};
use itertools::Itertools;
use serde::Deserialize;
use walkdir::WalkDir;

/// Parsed fixture definition for LLM API e2e test generation.
#[derive(Debug, Clone, Deserialize)]
pub struct Fixture {
    pub id: String,
    pub category: String,
    pub description: String,
    pub api: ApiSpec,
    #[serde(default)]
    pub assertions: Assertions,
    #[serde(default)]
    #[allow(dead_code)] // reserved for future filtering by tag
    pub tags: Vec<String>,
    #[serde(default)]
    pub skip: Skip,
    /// Source file path (populated after load, not from JSON).
    #[serde(skip)]
    #[allow(dead_code)] // used for error messages in future tooling
    pub source: Utf8PathBuf,
}

/// Specification of which API method is being tested and how to call it.
#[derive(Debug, Clone, Deserialize)]
pub struct ApiSpec {
    /// The API method under test: chat, chat_stream, embed, list_models.
    pub method: String,
    /// The request payload passed to the method (JSON object).
    #[serde(default)]
    pub request: serde_json::Value,
    /// The mock HTTP response the test server returns.
    pub mock_response: MockResponse,
}

/// Mock HTTP response configuration for the test server.
#[derive(Debug, Clone, Deserialize)]
pub struct MockResponse {
    /// HTTP status code to return (e.g., 200, 400, 500).
    pub status: u16,
    /// Response body (for non-streaming responses).
    #[serde(default)]
    pub body: serde_json::Value,
    /// Sequence of SSE chunks (for streaming responses).
    #[serde(default)]
    pub stream_chunks: Vec<serde_json::Value>,
}

/// Assertions to verify on the API response.
///
/// Field names match the JSON fixture files exactly.  All fields are optional
/// so that fixtures only need to declare the assertions they care about.
#[derive(Debug, Clone, Default, Deserialize)]
pub struct Assertions {
    // ── General ─────────────────────────────────────────────────────────────
    /// Whether the call is expected to succeed (default: true).
    /// Generators use this alongside `status >= 400` to detect error scenarios.
    #[serde(default = "default_true")]
    pub expect_success: bool,

    /// Whether the response object is non-null (always true for successful calls).
    #[serde(default)]
    #[allow(dead_code)] // reserved for future generator use
    pub response_not_null: Option<bool>,

    // ── Chat completion ──────────────────────────────────────────────────────
    /// Expected exact number of choices in the response.
    #[serde(default)]
    #[allow(dead_code)] // reserved for future generator use
    pub choices_count: Option<usize>,
    /// Expected content of the first choice's message.
    #[serde(default)]
    #[allow(dead_code)] // reserved for future generator use
    pub first_choice_content: Option<String>,
    /// Expected `finish_reason` of the first choice.
    #[serde(default)]
    #[allow(dead_code)] // reserved for future generator use
    pub first_choice_finish_reason: Option<String>,
    /// Expected total token count from the usage object.
    #[serde(default)]
    #[allow(dead_code)] // reserved for future generator use
    pub usage_total_tokens: Option<u64>,
    /// Expected model identifier in the response.
    #[serde(default)]
    #[allow(dead_code)] // reserved for future generator use
    pub model: Option<String>,

    // ── Embeddings ───────────────────────────────────────────────────────────
    /// Expected number of embedding vectors in the response.
    #[serde(default)]
    pub embedding_count: Option<usize>,
    /// Expected number of dimensions in each embedding vector.
    #[serde(default)]
    #[allow(dead_code)] // reserved for future generator use
    pub embedding_dimensions: Option<usize>,

    // ── Models list ──────────────────────────────────────────────────────────
    /// Minimum number of models returned by `list_models`.
    #[serde(default)]
    pub models_count_min: Option<usize>,

    // ── Streaming ────────────────────────────────────────────────────────────
    /// Minimum number of SSE chunks expected in a streaming response.
    #[serde(default)]
    pub stream_chunk_count_min: Option<usize>,
    /// Expected concatenated content across all streaming chunks.
    #[serde(default)]
    #[allow(dead_code)] // reserved for future generator use
    pub stream_final_content: Option<String>,
    /// Assert that the stream terminates with a `[DONE]` sentinel and no error.
    #[serde(default)]
    #[allow(dead_code)] // reserved for future generator use
    pub stream_completes_cleanly: Option<bool>,
    /// Assert that no chunks are received after the `[DONE]` sentinel.
    #[serde(default)]
    #[allow(dead_code)] // reserved for future generator use
    pub no_chunks_after_done: Option<bool>,

    // ── Tool calling ─────────────────────────────────────────────────────────
    /// Assert that the first choice contains at least one tool call.
    #[serde(default)]
    #[allow(dead_code)] // reserved for future generator use
    pub first_choice_has_tool_calls: Option<bool>,
    /// Expected `function.name` of the first tool call in the first choice.
    #[serde(default)]
    #[allow(dead_code)] // reserved for future generator use
    pub first_tool_call_function_name: Option<String>,

    // ── Error handling ───────────────────────────────────────────────────────
    /// Expected error variant name (e.g. "Authentication", "RateLimited").
    #[serde(default)]
    #[allow(dead_code)] // reserved for future generator use
    pub error_type: Option<String>,
    /// Expected HTTP status code that triggered the error.
    #[serde(default)]
    #[allow(dead_code)] // reserved for future generator use
    pub error_status_code: Option<u16>,
}

fn default_true() -> bool {
    true
}

/// Conditions under which the generated test should be skipped.
#[derive(Debug, Clone, Default, Deserialize)]
pub struct Skip {
    /// Skip on specific platforms (e.g., "windows", "linux").
    #[serde(default)]
    #[allow(dead_code)] // reserved for future platform-conditional skip generation
    pub platform: Vec<String>,
    /// Skip for specific language bindings (e.g., ["wasm"]).
    #[serde(default)]
    pub languages: Vec<String>,
    /// Human-readable reason for skipping.
    #[serde(default)]
    #[allow(dead_code)] // reserved for future skip message generation
    pub reason: Option<String>,
}

/// Load all JSON fixtures from a directory tree.
///
/// Files named `schema.json` or starting with `_` are ignored.
/// Fixtures are sorted by (category, id) and duplicate IDs cause an error.
pub fn load_fixtures(fixtures_dir: &Utf8Path) -> Result<Vec<Fixture>> {
    let mut fixtures = Vec::new();

    for entry in WalkDir::new(fixtures_dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
    {
        let path = Utf8PathBuf::from_path_buf(entry.into_path())
            .map_err(|_| anyhow::anyhow!("Fixture path is not valid UTF-8"))?;

        if path
            .file_name()
            .is_some_and(|name| name == "schema.json" || name.starts_with('_'))
        {
            continue;
        }

        if path.extension() != Some("json") {
            continue;
        }

        let contents = std::fs::read_to_string(&path).with_context(|| format!("Failed to read fixture {path}"))?;
        let mut fixture: Fixture = serde_json::from_str(&contents).with_context(|| format!("Parsing {path}"))?;

        fixture.source = path;
        fixtures.push(fixture);
    }

    fixtures.sort_by_key(|f| (f.category.clone(), f.id.clone()));

    let duplicates = fixtures
        .iter()
        .tuple_windows()
        .filter(|(a, b): &(&Fixture, &Fixture)| a.id == b.id)
        .map(|(a, _)| a.id.clone())
        .collect::<Vec<_>>();

    if !duplicates.is_empty() {
        bail!("Duplicate fixture IDs found: {:?}", duplicates);
    }

    Ok(fixtures)
}

/// Group fixtures by their category field.
pub fn group_by_category(fixtures: &[Fixture]) -> Vec<(String, Vec<&Fixture>)> {
    let mut grouped = fixtures
        .iter()
        .into_group_map_by(|f| f.category.clone())
        .into_iter()
        .collect::<Vec<_>>();
    grouped.sort_by(|a, b| a.0.cmp(&b.0));
    grouped
}
