use anyhow::{Context, Result, bail};
use camino::{Utf8Path, Utf8PathBuf};
use itertools::Itertools;
use serde::Deserialize;
use walkdir::WalkDir;

/// Parsed fixture definition for LLM API e2e test generation.
#[allow(dead_code)]
#[derive(Debug, Clone, Deserialize)]
pub struct Fixture {
    pub id: String,
    pub category: String,
    pub description: String,
    pub api: ApiSpec,
    #[serde(default)]
    pub assertions: Assertions,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(default)]
    pub skip: Skip,
    /// Source file path (populated after load, not from JSON).
    #[serde(skip)]
    pub source: Utf8PathBuf,
}

/// Specification of which API method is being tested and how to call it.
#[allow(dead_code)]
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
#[allow(dead_code)]
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
#[allow(dead_code)]
#[derive(Debug, Clone, Default, Deserialize)]
pub struct Assertions {
    /// Whether the call is expected to succeed (default: true).
    #[serde(default = "default_true")]
    pub expect_success: bool,
    /// Expected error message substring (when expect_success is false).
    #[serde(default)]
    pub expect_error_contains: Option<String>,
    /// Minimum number of choices/completions in response.
    #[serde(default)]
    pub min_choices: Option<usize>,
    /// Expected content substring in the first choice.
    #[serde(default)]
    pub content_contains: Option<String>,
    /// Whether the response should include a usage object.
    #[serde(default)]
    pub has_usage: Option<bool>,
    /// Expected model identifier in the response.
    #[serde(default)]
    pub model_equals: Option<String>,
    /// Minimum number of embedding vectors returned.
    #[serde(default)]
    pub min_embeddings: Option<usize>,
    /// Whether the embeddings should be non-empty vectors.
    #[serde(default)]
    pub embeddings_non_empty: Option<bool>,
    /// Minimum number of models in list_models response.
    #[serde(default)]
    pub min_models: Option<usize>,
    /// Whether streaming chunks were received (for chat_stream).
    #[serde(default)]
    pub received_stream_chunks: Option<bool>,
}

fn default_true() -> bool {
    true
}

/// Conditions under which the generated test should be skipped.
#[allow(dead_code)]
#[derive(Debug, Clone, Default, Deserialize)]
pub struct Skip {
    /// Skip on specific platforms (e.g., "windows", "linux").
    #[serde(default)]
    pub platform: Vec<String>,
    /// Skip for specific language bindings (e.g., ["wasm"]).
    #[serde(default)]
    pub languages: Vec<String>,
    /// Human-readable reason for skipping.
    #[serde(default)]
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
#[allow(dead_code)]
pub fn group_by_category(fixtures: &[Fixture]) -> Vec<(String, Vec<&Fixture>)> {
    let mut grouped = fixtures
        .iter()
        .into_group_map_by(|f| f.category.clone())
        .into_iter()
        .collect::<Vec<_>>();
    grouped.sort_by(|a, b| a.0.cmp(&b.0));
    grouped
}
