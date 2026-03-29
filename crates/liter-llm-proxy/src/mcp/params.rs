use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Parameters for the `chat` tool.
#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub struct ChatParams {
    /// Model name (e.g. "openai/gpt-4o", "anthropic/claude-sonnet-4-20250514").
    pub model: String,
    /// Array of message objects with `role` and `content` fields.
    pub messages: serde_json::Value,
    /// Sampling temperature (0.0 to 2.0).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f64>,
    /// Maximum number of tokens to generate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<u32>,
}

/// Parameters for the `embed` tool.
#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub struct EmbedParams {
    /// Embedding model name.
    pub model: String,
    /// Input text or array of texts to embed.
    pub input: serde_json::Value,
}

/// Empty parameters (no arguments required).
#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub struct EmptyParams {}

/// Parameters for the `generate_image` tool.
#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub struct ImageParams {
    /// Text description of the desired image.
    pub prompt: String,
    /// Image generation model name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    /// Number of images to generate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub n: Option<u32>,
    /// Image size (e.g. "1024x1024").
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<String>,
}

/// Parameters for the `speech` tool.
#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub struct SpeechParams {
    /// Text-to-speech model name.
    pub model: String,
    /// Text to convert to speech.
    pub input: String,
    /// Voice identifier (e.g. "alloy", "echo", "nova").
    pub voice: String,
}

/// Parameters for the `transcribe` tool.
#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub struct TranscribeParams {
    /// Transcription model name.
    pub model: String,
    /// Base64-encoded audio file data.
    pub file_base64: String,
}

/// Parameters for the `moderate` tool.
#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub struct ModerateParams {
    /// Text or array of texts to moderate.
    pub input: serde_json::Value,
    /// Moderation model name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
}

/// Parameters for the `rerank` tool.
#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub struct RerankParams {
    /// Reranking model name.
    pub model: String,
    /// Query to rank documents against.
    pub query: String,
    /// List of documents to rerank.
    pub documents: Vec<String>,
}

/// Parameters for the `search` tool.
#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub struct SearchParams {
    /// Search model name.
    pub model: String,
    /// Search query.
    pub query: String,
}

/// Parameters for the `ocr` tool.
#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub struct OcrParams {
    /// OCR model name.
    pub model: String,
    /// URL of the image to process.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_url: Option<String>,
    /// Base64-encoded image data.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_base64: Option<String>,
    /// MIME type of the base64-encoded image (e.g. "image/png", "image/jpeg").
    /// Defaults to "image/png" when omitted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_type: Option<String>,
}

/// Parameters for tools that take a file ID.
#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub struct FileIdParams {
    /// The file identifier.
    pub file_id: String,
}

/// Parameters for the `list_files` tool.
#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub struct ListFilesParams {
    /// Filter by purpose.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub purpose: Option<String>,
    /// Maximum number of files to return.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
}

/// Parameters for the `create_file` tool.
#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub struct CreateFileParams {
    /// Base64-encoded file data.
    pub file_base64: String,
    /// Filename for the uploaded file.
    pub filename: String,
    /// Purpose of the file (e.g. "assistants", "batch", "fine-tune", "vision").
    pub purpose: String,
}

/// Parameters for tools that take a batch ID.
#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub struct BatchIdParams {
    /// The batch job identifier.
    pub batch_id: String,
}

/// Parameters for the `list_batches` tool.
#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub struct ListBatchesParams {
    /// Maximum number of batches to return.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    /// Cursor for pagination (return batches after this ID).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
}

/// Parameters for the `create_batch` tool.
#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub struct CreateBatchParams {
    /// File ID of the JSONL input file.
    pub input_file_id: String,
    /// API endpoint for batch processing (e.g. "/v1/chat/completions").
    pub endpoint: String,
    /// Time window for batch completion (e.g. "24h").
    pub completion_window: String,
}

/// Parameters for tools that take a response ID.
#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub struct ResponseIdParams {
    /// The response identifier.
    pub response_id: String,
}

/// Parameters for the `create_response` tool.
#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub struct CreateResponseParams {
    /// Model name for the response.
    pub model: String,
    /// Input content for the response (string or structured input).
    pub input: serde_json::Value,
}
