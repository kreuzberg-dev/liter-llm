use crate::client::BoxStream;
use crate::types::audio::{CreateSpeechRequest, CreateTranscriptionRequest, TranscriptionResponse};
use crate::types::image::{CreateImageRequest, ImagesResponse};
use crate::types::moderation::{ModerationRequest, ModerationResponse};
use crate::types::rerank::{RerankRequest, RerankResponse};
use crate::types::{
    ChatCompletionChunk, ChatCompletionRequest, ChatCompletionResponse, EmbeddingRequest, EmbeddingResponse,
    ModelsListResponse, Usage,
};

/// The request variant passed through the tower `Service` stack.
///
/// Each variant corresponds to one method on [`crate::client::LlmClient`].
#[derive(Debug, Clone)]
pub enum LlmRequest {
    /// Non-streaming chat completion.
    Chat(ChatCompletionRequest),
    /// Streaming chat completion — yields a stream of chunks.
    ChatStream(ChatCompletionRequest),
    /// Text embedding.
    Embed(EmbeddingRequest),
    /// List available models from the provider.
    ListModels,
    /// Image generation.
    ImageGenerate(CreateImageRequest),
    /// Text-to-speech audio generation.
    Speech(CreateSpeechRequest),
    /// Audio transcription.
    Transcribe(CreateTranscriptionRequest),
    /// Content moderation.
    Moderate(ModerationRequest),
    /// Document reranking.
    Rerank(RerankRequest),
}

impl LlmRequest {
    /// OpenTelemetry GenAI `gen_ai.operation.name` value for this request.
    ///
    /// Maps each variant to one of the canonical GenAI semantic convention
    /// operation names: `"chat"`, `"embeddings"`, or `"list_models"`.
    /// Both streaming and non-streaming chat map to `"chat"`.
    #[must_use]
    pub fn operation_name(&self) -> &'static str {
        match self {
            Self::Chat(_) | Self::ChatStream(_) => "chat",
            Self::Embed(_) => "embeddings",
            Self::ListModels => "list_models",
            Self::ImageGenerate(_) => "image_generate",
            Self::Speech(_) => "speech",
            Self::Transcribe(_) => "transcribe",
            Self::Moderate(_) => "moderate",
            Self::Rerank(_) => "rerank",
        }
    }

    /// Human-readable name of the request type; used as a span / metric label.
    #[must_use]
    pub fn request_type(&self) -> &'static str {
        match self {
            Self::Chat(_) => "chat",
            Self::ChatStream(_) => "chat_stream",
            Self::Embed(_) => "embeddings",
            Self::ListModels => "list_models",
            Self::ImageGenerate(_) => "image_generate",
            Self::Speech(_) => "speech",
            Self::Transcribe(_) => "transcribe",
            Self::Moderate(_) => "moderate",
            Self::Rerank(_) => "rerank",
        }
    }

    /// Return the model name embedded in the request, if any.
    #[must_use]
    pub fn model(&self) -> Option<&str> {
        match self {
            Self::Chat(r) | Self::ChatStream(r) => Some(r.model.as_str()),
            Self::Embed(r) => Some(r.model.as_str()),
            Self::ImageGenerate(r) => r.model.as_deref(),
            Self::Speech(r) => Some(r.model.as_str()),
            Self::Transcribe(r) => Some(r.model.as_str()),
            Self::Moderate(r) => r.model.as_deref(),
            Self::Rerank(r) => Some(r.model.as_str()),
            Self::ListModels => None,
        }
    }
}

/// The response variant returned through the tower `Service` stack.
pub enum LlmResponse {
    /// Non-streaming chat completion.
    Chat(ChatCompletionResponse),
    /// Streaming chat completion.
    ChatStream(BoxStream<'static, ChatCompletionChunk>),
    /// Text embedding.
    Embed(EmbeddingResponse),
    /// Model list.
    ListModels(ModelsListResponse),
    /// Image generation.
    ImageGenerate(ImagesResponse),
    /// Text-to-speech audio (raw bytes).
    Speech(bytes::Bytes),
    /// Audio transcription.
    Transcribe(TranscriptionResponse),
    /// Content moderation.
    Moderate(ModerationResponse),
    /// Document reranking.
    Rerank(RerankResponse),
}

impl LlmResponse {
    /// Return the usage data from the response, if present.
    ///
    /// Streaming, model-list, and non-chat responses do not carry aggregated
    /// usage data and always return `None`.
    #[must_use]
    pub fn usage(&self) -> Option<&Usage> {
        match self {
            Self::Chat(r) => r.usage.as_ref(),
            Self::Embed(r) => r.usage.as_ref(),
            Self::ChatStream(_)
            | Self::ListModels(_)
            | Self::ImageGenerate(_)
            | Self::Speech(_)
            | Self::Transcribe(_)
            | Self::Moderate(_)
            | Self::Rerank(_) => None,
        }
    }
}

impl std::fmt::Debug for LlmResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Chat(r) => f.debug_tuple("Chat").field(r).finish(),
            Self::ChatStream(_) => f.write_str("ChatStream(<stream>)"),
            Self::Embed(r) => f.debug_tuple("Embed").field(r).finish(),
            Self::ListModels(r) => f.debug_tuple("ListModels").field(r).finish(),
            Self::ImageGenerate(r) => f.debug_tuple("ImageGenerate").field(r).finish(),
            Self::Speech(b) => f
                .debug_tuple("Speech")
                .field(&format_args!("<{} bytes>", b.len()))
                .finish(),
            Self::Transcribe(r) => f.debug_tuple("Transcribe").field(r).finish(),
            Self::Moderate(r) => f.debug_tuple("Moderate").field(r).finish(),
            Self::Rerank(r) => f.debug_tuple("Rerank").field(r).finish(),
        }
    }
}
