use crate::client::BoxStream;
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
        }
    }

    /// Return the model name embedded in the request, if any.
    #[must_use]
    pub fn model(&self) -> Option<&str> {
        match self {
            Self::Chat(r) | Self::ChatStream(r) => Some(r.model.as_str()),
            Self::Embed(r) => Some(r.model.as_str()),
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
}

impl LlmResponse {
    /// Return the usage data from the response, if present.
    ///
    /// Streaming and model-list responses do not carry aggregated usage data
    /// and always return `None`.
    #[must_use]
    pub fn usage(&self) -> Option<&Usage> {
        match self {
            Self::Chat(r) => r.usage.as_ref(),
            Self::Embed(r) => r.usage.as_ref(),
            Self::ChatStream(_) | Self::ListModels(_) => None,
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
        }
    }
}
