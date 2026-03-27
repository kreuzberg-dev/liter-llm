//! Rustler NIF bindings for liter-llm.
//!
//! All NIF functions accept and return JSON strings.  The Elixir
//! `LiterLlm.Native` module encodes Elixir maps to JSON before calling NIFs
//! and decodes the JSON response back to Elixir terms.
//!
//! # Scheduling
//!
//! Every NIF is scheduled as `DirtyIo` because it blocks on network I/O.
//! The NIFs use a global Tokio runtime and `block_on` so they never run on
//! the BEAM scheduler threads.

use std::sync::OnceLock;
use std::time::Duration;

use liter_llm::client::{BatchClient, FileClient, LlmClient, ResponseClient};
use liter_llm::types::audio::{CreateSpeechRequest, CreateTranscriptionRequest};
use liter_llm::types::batch::{BatchListQuery, CreateBatchRequest};
use liter_llm::types::files::{CreateFileRequest, FileListQuery};
use liter_llm::types::image::CreateImageRequest;
use liter_llm::types::moderation::ModerationRequest;
use liter_llm::types::rerank::RerankRequest;
use liter_llm::types::responses::CreateResponseRequest;
use liter_llm::{ChatCompletionRequest, ClientConfig, DefaultClient, EmbeddingRequest};
use rustler::{Error as NifError, NifResult, OwnedBinary};
use serde::Deserialize;
use tokio::runtime::Runtime;

// ─── Tokio runtime ────────────────────────────────────────────────────────────

fn runtime() -> &'static Runtime {
    static RT: OnceLock<Runtime> = OnceLock::new();
    RT.get_or_init(|| {
        tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .expect("failed to create Tokio runtime for liter_llm_rustler")
    })
}

// ─── Client construction helpers ─────────────────────────────────────────────

/// Client options accepted as the first argument to every NIF.
#[derive(Deserialize)]
struct ClientOptions {
    api_key: String,
    #[serde(default)]
    base_url: Option<String>,
    #[serde(default)]
    max_retries: Option<u32>,
    #[serde(default)]
    timeout_secs: Option<u64>,
}

fn build_client(config_json: &str, model_hint: Option<&str>) -> NifResult<DefaultClient> {
    let opts: ClientOptions =
        serde_json::from_str(config_json).map_err(|e| nif_err(format!("invalid client config: {e}")))?;

    let mut config = ClientConfig::new(opts.api_key);
    config.base_url = opts.base_url;
    if let Some(r) = opts.max_retries {
        config.max_retries = r;
    }
    if let Some(t) = opts.timeout_secs {
        config.timeout = Duration::from_secs(t);
    }

    DefaultClient::new(config, model_hint).map_err(|e| nif_err(e.to_string()))
}

fn extract_model(request_json: &str) -> Option<String> {
    serde_json::from_str::<serde_json::Value>(request_json)
        .ok()
        .and_then(|v| v.get("model")?.as_str().map(String::from))
}

fn nif_err(msg: impl Into<String>) -> NifError {
    NifError::Term(Box::new(msg.into()))
}

fn to_json<T: serde::Serialize>(value: &T) -> NifResult<String> {
    serde_json::to_string(value).map_err(|e| nif_err(format!("serialization error: {e}")))
}

fn from_json<T: serde::de::DeserializeOwned>(json: &str, label: &str) -> NifResult<T> {
    serde_json::from_str(json).map_err(|e| nif_err(format!("invalid {label}: {e}")))
}

// ─── Core inference NIFs ──────────────────────────────────────────────────────

/// Send a chat completion request.
///
/// `config_json` — JSON string with `{api_key, base_url?, max_retries?, timeout_secs?}`
/// `request_json` — JSON string matching the OpenAI chat completion request shape
#[rustler::nif(schedule = "DirtyIo")]
fn chat(config_json: String, request_json: String) -> NifResult<String> {
    let model = extract_model(&request_json);
    let client = build_client(&config_json, model.as_deref())?;
    let req: ChatCompletionRequest = from_json(&request_json, "chat request")?;
    let resp = runtime()
        .block_on(async move { client.chat(req).await })
        .map_err(|e| nif_err(e.to_string()))?;
    to_json(&resp)
}

/// Send an embedding request.
#[rustler::nif(schedule = "DirtyIo")]
fn embed(config_json: String, request_json: String) -> NifResult<String> {
    let model = extract_model(&request_json);
    let client = build_client(&config_json, model.as_deref())?;
    let req: EmbeddingRequest = from_json(&request_json, "embedding request")?;
    let resp = runtime()
        .block_on(async move { client.embed(req).await })
        .map_err(|e| nif_err(e.to_string()))?;
    to_json(&resp)
}

/// List available models.
#[rustler::nif(schedule = "DirtyIo")]
fn list_models(config_json: String) -> NifResult<String> {
    let client = build_client(&config_json, None)?;
    let resp = runtime()
        .block_on(async move { client.list_models().await })
        .map_err(|e| nif_err(e.to_string()))?;
    to_json(&resp)
}

/// Generate an image from a text prompt.
#[rustler::nif(schedule = "DirtyIo")]
fn image_generate(config_json: String, request_json: String) -> NifResult<String> {
    let model = extract_model(&request_json);
    let client = build_client(&config_json, model.as_deref())?;
    let req: CreateImageRequest = from_json(&request_json, "image generate request")?;
    let resp = runtime()
        .block_on(async move { client.image_generate(req).await })
        .map_err(|e| nif_err(e.to_string()))?;
    to_json(&resp)
}

/// Generate speech audio from text.
///
/// Returns the raw audio bytes as an Elixir binary.
#[rustler::nif(schedule = "DirtyIo")]
fn speech(config_json: String, request_json: String) -> NifResult<OwnedBinary> {
    let model = extract_model(&request_json);
    let client = build_client(&config_json, model.as_deref())?;
    let req: CreateSpeechRequest = from_json(&request_json, "speech request")?;
    let bytes = runtime()
        .block_on(async move { client.speech(req).await })
        .map_err(|e| nif_err(e.to_string()))?;
    let mut bin = OwnedBinary::new(bytes.len()).ok_or_else(|| nif_err("failed to allocate binary"))?;
    bin.as_mut_slice().copy_from_slice(&bytes);
    Ok(bin)
}

/// Transcribe audio to text.
#[rustler::nif(schedule = "DirtyIo")]
fn transcribe(config_json: String, request_json: String) -> NifResult<String> {
    let model = extract_model(&request_json);
    let client = build_client(&config_json, model.as_deref())?;
    let req: CreateTranscriptionRequest = from_json(&request_json, "transcription request")?;
    let resp = runtime()
        .block_on(async move { client.transcribe(req).await })
        .map_err(|e| nif_err(e.to_string()))?;
    to_json(&resp)
}

/// Check content against moderation policies.
#[rustler::nif(schedule = "DirtyIo")]
fn moderate(config_json: String, request_json: String) -> NifResult<String> {
    let client = build_client(&config_json, None)?;
    let req: ModerationRequest = from_json(&request_json, "moderation request")?;
    let resp = runtime()
        .block_on(async move { client.moderate(req).await })
        .map_err(|e| nif_err(e.to_string()))?;
    to_json(&resp)
}

/// Rerank documents by relevance to a query.
#[rustler::nif(schedule = "DirtyIo")]
fn rerank(config_json: String, request_json: String) -> NifResult<String> {
    let model = extract_model(&request_json);
    let client = build_client(&config_json, model.as_deref())?;
    let req: RerankRequest = from_json(&request_json, "rerank request")?;
    let resp = runtime()
        .block_on(async move { client.rerank(req).await })
        .map_err(|e| nif_err(e.to_string()))?;
    to_json(&resp)
}

// ─── File management NIFs ─────────────────────────────────────────────────────

/// Upload a file.
#[rustler::nif(schedule = "DirtyIo")]
fn create_file(config_json: String, request_json: String) -> NifResult<String> {
    let client = build_client(&config_json, None)?;
    let req: CreateFileRequest = from_json(&request_json, "file request")?;
    let resp = runtime()
        .block_on(async move { client.create_file(req).await })
        .map_err(|e| nif_err(e.to_string()))?;
    to_json(&resp)
}

/// Retrieve metadata for a file.
#[rustler::nif(schedule = "DirtyIo")]
fn retrieve_file(config_json: String, file_id: String) -> NifResult<String> {
    let client = build_client(&config_json, None)?;
    let resp = runtime()
        .block_on(async move { client.retrieve_file(&file_id).await })
        .map_err(|e| nif_err(e.to_string()))?;
    to_json(&resp)
}

/// Delete a file.
#[rustler::nif(schedule = "DirtyIo")]
fn delete_file(config_json: String, file_id: String) -> NifResult<String> {
    let client = build_client(&config_json, None)?;
    let resp = runtime()
        .block_on(async move { client.delete_file(&file_id).await })
        .map_err(|e| nif_err(e.to_string()))?;
    to_json(&resp)
}

/// List files, optionally filtered by query parameters.
///
/// `query_json` — JSON string or `"null"` to list all files without filtering.
#[rustler::nif(schedule = "DirtyIo")]
fn list_files(config_json: String, query_json: String) -> NifResult<String> {
    let client = build_client(&config_json, None)?;
    let query: Option<FileListQuery> = if query_json == "null" || query_json.is_empty() {
        None
    } else {
        Some(from_json(&query_json, "file list query")?)
    };
    let resp = runtime()
        .block_on(async move { client.list_files(query).await })
        .map_err(|e| nif_err(e.to_string()))?;
    to_json(&resp)
}

/// Retrieve the raw content of a file.
#[rustler::nif(schedule = "DirtyIo")]
fn file_content(config_json: String, file_id: String) -> NifResult<OwnedBinary> {
    let client = build_client(&config_json, None)?;
    let bytes = runtime()
        .block_on(async move { client.file_content(&file_id).await })
        .map_err(|e| nif_err(e.to_string()))?;
    let mut bin = OwnedBinary::new(bytes.len()).ok_or_else(|| nif_err("failed to allocate binary"))?;
    bin.as_mut_slice().copy_from_slice(&bytes);
    Ok(bin)
}

// ─── Batch management NIFs ────────────────────────────────────────────────────

/// Create a new batch job.
#[rustler::nif(schedule = "DirtyIo")]
fn create_batch(config_json: String, request_json: String) -> NifResult<String> {
    let client = build_client(&config_json, None)?;
    let req: CreateBatchRequest = from_json(&request_json, "batch request")?;
    let resp = runtime()
        .block_on(async move { client.create_batch(req).await })
        .map_err(|e| nif_err(e.to_string()))?;
    to_json(&resp)
}

/// Retrieve a batch by ID.
#[rustler::nif(schedule = "DirtyIo")]
fn retrieve_batch(config_json: String, batch_id: String) -> NifResult<String> {
    let client = build_client(&config_json, None)?;
    let resp = runtime()
        .block_on(async move { client.retrieve_batch(&batch_id).await })
        .map_err(|e| nif_err(e.to_string()))?;
    to_json(&resp)
}

/// List batches, optionally filtered by query parameters.
#[rustler::nif(schedule = "DirtyIo")]
fn list_batches(config_json: String, query_json: String) -> NifResult<String> {
    let client = build_client(&config_json, None)?;
    let query: Option<BatchListQuery> = if query_json == "null" || query_json.is_empty() {
        None
    } else {
        Some(from_json(&query_json, "batch list query")?)
    };
    let resp = runtime()
        .block_on(async move { client.list_batches(query).await })
        .map_err(|e| nif_err(e.to_string()))?;
    to_json(&resp)
}

/// Cancel an in-progress batch.
#[rustler::nif(schedule = "DirtyIo")]
fn cancel_batch(config_json: String, batch_id: String) -> NifResult<String> {
    let client = build_client(&config_json, None)?;
    let resp = runtime()
        .block_on(async move { client.cancel_batch(&batch_id).await })
        .map_err(|e| nif_err(e.to_string()))?;
    to_json(&resp)
}

// ─── Response management NIFs ─────────────────────────────────────────────────

/// Create a new response.
#[rustler::nif(schedule = "DirtyIo")]
fn create_response(config_json: String, request_json: String) -> NifResult<String> {
    let client = build_client(&config_json, None)?;
    let req: CreateResponseRequest = from_json(&request_json, "response request")?;
    let resp = runtime()
        .block_on(async move { client.create_response(req).await })
        .map_err(|e| nif_err(e.to_string()))?;
    to_json(&resp)
}

/// Retrieve a response by ID.
#[rustler::nif(schedule = "DirtyIo")]
fn retrieve_response(config_json: String, response_id: String) -> NifResult<String> {
    let client = build_client(&config_json, None)?;
    let resp = runtime()
        .block_on(async move { client.retrieve_response(&response_id).await })
        .map_err(|e| nif_err(e.to_string()))?;
    to_json(&resp)
}

/// Cancel an in-progress response.
#[rustler::nif(schedule = "DirtyIo")]
fn cancel_response(config_json: String, response_id: String) -> NifResult<String> {
    let client = build_client(&config_json, None)?;
    let resp = runtime()
        .block_on(async move { client.cancel_response(&response_id).await })
        .map_err(|e| nif_err(e.to_string()))?;
    to_json(&resp)
}

// ─── NIF init ─────────────────────────────────────────────────────────────────

rustler::init!(
    "Elixir.LiterLlm.Native",
    [
        // Core inference
        chat,
        embed,
        list_models,
        image_generate,
        speech,
        transcribe,
        moderate,
        rerank,
        // File management
        create_file,
        retrieve_file,
        delete_file,
        list_files,
        file_content,
        // Batch management
        create_batch,
        retrieve_batch,
        list_batches,
        cancel_batch,
        // Response management
        create_response,
        retrieve_response,
        cancel_response,
    ]
);
