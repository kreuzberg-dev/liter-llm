use anyhow::{Context, Result, bail};
use camino::{Utf8Path, Utf8PathBuf};
use itertools::Itertools;
use serde::Deserialize;
use walkdir::WalkDir;

/// Client configuration overrides for fixtures that test caching, budgets, etc.
#[derive(Debug, Clone, Default, Deserialize)]
pub struct ClientConfig {
    #[serde(default)]
    #[allow(dead_code)] // cache config is hardcoded in emitters for now
    pub cache: Option<serde_json::Value>,
    #[serde(default)]
    pub budget: Option<serde_json::Value>,
    #[serde(default)]
    #[allow(dead_code)] // reserved for future generator use
    pub extra_headers: Option<serde_json::Value>,
    #[serde(default)]
    pub custom_provider: Option<serde_json::Value>,
}

/// Parsed fixture definition for LLM API e2e test generation.
#[derive(Debug, Clone, Deserialize)]
pub struct Fixture {
    pub id: String,
    pub category: String,
    pub description: String,
    pub api: ApiSpec,
    #[serde(default)]
    pub client_config: ClientConfig,
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
    /// `None` for live/smoke test fixtures that hit real APIs — these are
    /// filtered out during loading and never reach generators.
    #[serde(default)]
    pub mock_response: Option<MockResponse>,
}

impl ApiSpec {
    /// Access the mock response, panicking if absent.
    ///
    /// Live/smoke fixtures (without `mock_response`) are filtered out during
    /// loading, so this is safe to call in generators.
    pub fn mock_response(&self) -> &MockResponse {
        self.mock_response
            .as_ref()
            .expect("mock_response is required for generated tests (live fixtures should be filtered)")
    }
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

    // ── Image generation ─────────────────────────────────────────────────────
    /// Expected number of image objects in the response.
    #[serde(default)]
    #[allow(dead_code)] // reserved for future generator use
    pub image_count: Option<usize>,
    /// Assert that the response contains base64-encoded image data.
    #[serde(default)]
    #[allow(dead_code)] // reserved for future generator use
    pub image_has_b64_data: Option<bool>,
    /// Assert that the response contains image URLs.
    #[serde(default)]
    #[allow(dead_code)] // reserved for future generator use
    pub image_has_url: Option<bool>,

    // ── Speech ────────────────────────────────────────────────────────────────
    /// Assert that the audio response body is non-empty.
    #[serde(default)]
    #[allow(dead_code)] // reserved for future generator use
    pub audio_not_empty: Option<bool>,

    // ── Transcription ─────────────────────────────────────────────────────────
    /// Expected transcription text.
    #[serde(default)]
    #[allow(dead_code)] // reserved for future generator use
    pub transcription_text: Option<String>,

    // ── Moderation ────────────────────────────────────────────────────────────
    /// Expected number of moderation result objects.
    #[serde(default)]
    #[allow(dead_code)] // reserved for future generator use
    pub moderation_result_count: Option<usize>,
    /// Assert that at least one result is flagged.
    #[serde(default)]
    #[allow(dead_code)] // reserved for future generator use
    pub moderation_has_flagged: Option<bool>,

    // ── Rerank ────────────────────────────────────────────────────────────────
    /// Expected number of rerank result objects.
    #[serde(default)]
    #[allow(dead_code)] // reserved for future generator use
    pub rerank_result_count: Option<usize>,
    /// Assert that results contain the document text.
    #[serde(default)]
    #[allow(dead_code)] // reserved for future generator use
    pub rerank_has_documents: Option<bool>,
    /// Assert the top result's relevance score is above this threshold.
    #[serde(default)]
    #[allow(dead_code)] // reserved for future generator use
    pub rerank_top_score_min: Option<f64>,

    // ── Files ─────────────────────────────────────────────────────────────────
    /// Expected file ID in the response.
    #[serde(default)]
    #[allow(dead_code)] // reserved for future generator use
    pub file_id: Option<String>,
    /// Expected number of files in a list response.
    #[serde(default)]
    #[allow(dead_code)] // reserved for future generator use
    pub file_count: Option<usize>,
    /// Assert that file content is non-empty.
    #[serde(default)]
    #[allow(dead_code)] // reserved for future generator use
    pub file_content_not_empty: Option<bool>,
    /// Assert that the file was deleted successfully.
    #[serde(default)]
    #[allow(dead_code)] // reserved for future generator use
    pub file_deleted: Option<bool>,

    // ── Batches ───────────────────────────────────────────────────────────────
    /// Expected batch ID in the response.
    #[serde(default)]
    #[allow(dead_code)] // reserved for future generator use
    pub batch_id: Option<String>,
    /// Expected batch status (e.g. "completed", "in_progress", "cancelled").
    #[serde(default)]
    #[allow(dead_code)] // reserved for future generator use
    pub batch_status: Option<String>,
    /// Expected number of batches in a list response.
    #[serde(default)]
    #[allow(dead_code)] // reserved for future generator use
    pub batch_count: Option<usize>,

    // ── Responses API ─────────────────────────────────────────────────────────
    /// Expected response ID from the Responses API.
    #[serde(default)]
    #[allow(dead_code)] // reserved for future generator use
    pub response_id: Option<String>,
    /// Expected response status (e.g. "completed", "cancelled").
    #[serde(default)]
    #[allow(dead_code)] // reserved for future generator use
    pub response_status: Option<String>,
    /// Expected output items count from the Responses API.
    #[serde(default)]
    #[allow(dead_code)] // reserved for future generator use
    pub response_output_count: Option<usize>,
    /// Assert that the response contains tool call output.
    #[serde(default)]
    #[allow(dead_code)] // reserved for future generator use
    pub response_has_tool_calls: Option<bool>,

    // ── Error handling ───────────────────────────────────────────────────────
    /// Expected error variant name (e.g. "Authentication", "RateLimited").
    #[serde(default)]
    pub error_type: Option<String>,
    /// Expected HTTP status code that triggered the error.
    #[serde(default)]
    #[allow(dead_code)] // reserved for future generator use
    pub error_status_code: Option<u16>,

    // ── Cache ─────────────────────────────────────────────────────────────────
    /// Assert that the response was served from cache.
    #[serde(default)]
    pub cache_hit: Option<bool>,
    /// Assert that the cache was bypassed for this request.
    #[serde(default)]
    pub cache_bypassed: Option<bool>,

    // ── Budget ────────────────────────────────────────────────────────────────
    /// Assert that cost was tracked for the request.
    #[serde(default)]
    pub cost_tracked: Option<bool>,

    // ── Hooks ─────────────────────────────────────────────────────────────────
    /// Assert that the `on_request` hook was called.
    #[serde(default)]
    pub hook_on_request_called: Option<bool>,
    /// Assert that the `on_response` hook was called.
    #[serde(default)]
    pub hook_on_response_called: Option<bool>,
    /// Assert that the `on_error` hook was called.
    #[serde(default)]
    pub hook_on_error_called: Option<bool>,
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

        // Skip live/smoke fixtures that don't have mock responses —
        // they are for integration tests, not generated e2e tests.
        if fixture.api.mock_response.is_none() {
            continue;
        }

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
