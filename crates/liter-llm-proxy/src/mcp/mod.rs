pub mod errors;
pub mod params;

use std::sync::Arc;

use rmcp::handler::server::router::tool::ToolRouter;
use rmcp::handler::server::wrapper::Parameters;
use rmcp::model::*;
use rmcp::{ServerHandler, tool, tool_handler, tool_router};
use tower::Service;

use liter_llm::client::{BatchClient, FileClient, ResponseClient};
use liter_llm::tower::types::{LlmRequest, LlmResponse};
use liter_llm::types::audio::{CreateSpeechRequest, CreateTranscriptionRequest};
use liter_llm::types::batch::{BatchListQuery, CreateBatchRequest};
use liter_llm::types::files::{CreateFileRequest, FileListQuery, FilePurpose};
use liter_llm::types::image::CreateImageRequest;
use liter_llm::types::moderation::ModerationRequest;
use liter_llm::types::ocr::{OcrDocument, OcrRequest};
use liter_llm::types::rerank::{RerankDocument, RerankRequest};
use liter_llm::types::responses::CreateResponseRequest;
use liter_llm::types::search::SearchRequest;
use liter_llm::types::{ChatCompletionRequest, EmbeddingRequest};

use crate::file_store::FileStore;
use crate::service_pool::ServicePool;

use self::errors::to_error_data;

/// MCP server exposing the liter-llm proxy as a set of callable tools.
///
/// Each tool corresponds to an LLM API endpoint (chat, embed, image generation,
/// etc.) or a management operation (files, batches, responses).
#[derive(Clone)]
pub struct LiterLlmMcp {
    tool_router: ToolRouter<Self>,
    service_pool: Arc<ServicePool>,
    #[allow(dead_code)]
    file_store: Arc<FileStore>,
}

impl LiterLlmMcp {
    /// Create a new MCP server backed by the given service pool and file store.
    pub fn new(service_pool: Arc<ServicePool>, file_store: Arc<FileStore>) -> Self {
        Self {
            tool_router: Self::tool_router(),
            service_pool,
            file_store,
        }
    }
}

// ─── Helper ──────────────────────────────────────────────────────────────────

/// Serialize a value to pretty JSON and wrap it in a successful `CallToolResult`.
fn json_success<T: serde::Serialize>(value: &T) -> Result<CallToolResult, rmcp::ErrorData> {
    let json = serde_json::to_string_pretty(value).map_err(|e| rmcp::ErrorData::internal_error(e.to_string(), None))?;
    Ok(CallToolResult::success(vec![Content::text(json)]))
}

// ─── Tool implementations ────────────────────────────────────────────────────

#[tool_router]
impl LiterLlmMcp {
    // ── Chat & Embeddings ────────────────────────────────────────────────

    #[tool(description = "Send a chat completion request to an LLM")]
    async fn chat(
        &self,
        Parameters(params): Parameters<params::ChatParams>,
    ) -> Result<CallToolResult, rmcp::ErrorData> {
        let req: ChatCompletionRequest = serde_json::from_value(serde_json::json!({
            "model": params.model,
            "messages": params.messages,
            "temperature": params.temperature,
            "max_tokens": params.max_tokens,
        }))
        .map_err(|e| rmcp::ErrorData::invalid_params(e.to_string(), None))?;

        let mut svc = self
            .service_pool
            .get_service(&params.model)
            .map_err(|e| rmcp::ErrorData::internal_error(e.to_string(), None))?;

        let resp = svc.call(LlmRequest::Chat(req)).await.map_err(to_error_data)?;

        match resp {
            LlmResponse::Chat(r) => json_success(&r),
            other => Err(rmcp::ErrorData::internal_error(
                format!("unexpected response variant: {other:?}"),
                None,
            )),
        }
    }

    #[tool(description = "Generate text embeddings for the given input")]
    async fn embed(
        &self,
        Parameters(params): Parameters<params::EmbedParams>,
    ) -> Result<CallToolResult, rmcp::ErrorData> {
        let req: EmbeddingRequest = serde_json::from_value(serde_json::json!({
            "model": params.model,
            "input": params.input,
        }))
        .map_err(|e| rmcp::ErrorData::invalid_params(e.to_string(), None))?;

        let mut svc = self
            .service_pool
            .get_service(&params.model)
            .map_err(|e| rmcp::ErrorData::internal_error(e.to_string(), None))?;

        let resp = svc.call(LlmRequest::Embed(req)).await.map_err(to_error_data)?;

        match resp {
            LlmResponse::Embed(r) => json_success(&r),
            other => Err(rmcp::ErrorData::internal_error(
                format!("unexpected response variant: {other:?}"),
                None,
            )),
        }
    }

    #[tool(description = "List available models from configured providers")]
    async fn list_models(
        &self,
        #[allow(unused_variables)] Parameters(_params): Parameters<params::EmptyParams>,
    ) -> Result<CallToolResult, rmcp::ErrorData> {
        // Try to list models from the first available service.
        let model_names = self.service_pool.model_names();
        if model_names.is_empty() {
            return Err(rmcp::ErrorData::internal_error("no models configured", None));
        }

        let first_model = model_names[0];
        let mut svc = self
            .service_pool
            .get_service(first_model)
            .map_err(|e| rmcp::ErrorData::internal_error(e.to_string(), None))?;

        let resp = svc.call(LlmRequest::ListModels).await.map_err(to_error_data)?;

        match resp {
            LlmResponse::ListModels(r) => json_success(&r),
            other => Err(rmcp::ErrorData::internal_error(
                format!("unexpected response variant: {other:?}"),
                None,
            )),
        }
    }

    // ── Image generation ─────────────────────────────────────────────────

    #[tool(description = "Generate images from a text prompt")]
    async fn generate_image(
        &self,
        Parameters(params): Parameters<params::ImageParams>,
    ) -> Result<CallToolResult, rmcp::ErrorData> {
        let req = CreateImageRequest {
            prompt: params.prompt,
            model: params.model.clone(),
            n: params.n,
            size: params.size,
            quality: None,
            style: None,
            response_format: None,
            user: None,
        };

        // Use the model name if provided, otherwise fall back to first service.
        let mut svc = if let Some(ref model) = params.model {
            self.service_pool
                .get_service(model)
                .map_err(|e| rmcp::ErrorData::internal_error(e.to_string(), None))?
        } else {
            let names = self.service_pool.model_names();
            let first = names
                .first()
                .ok_or_else(|| rmcp::ErrorData::internal_error("no models configured", None))?;
            self.service_pool
                .get_service(first)
                .map_err(|e| rmcp::ErrorData::internal_error(e.to_string(), None))?
        };

        let resp = svc.call(LlmRequest::ImageGenerate(req)).await.map_err(to_error_data)?;

        match resp {
            LlmResponse::ImageGenerate(r) => json_success(&r),
            other => Err(rmcp::ErrorData::internal_error(
                format!("unexpected response variant: {other:?}"),
                None,
            )),
        }
    }

    // ── Audio ────────────────────────────────────────────────────────────

    #[tool(description = "Generate speech audio from text (text-to-speech)")]
    async fn speech(
        &self,
        Parameters(params): Parameters<params::SpeechParams>,
    ) -> Result<CallToolResult, rmcp::ErrorData> {
        let req = CreateSpeechRequest {
            model: params.model.clone(),
            input: params.input,
            voice: params.voice,
            response_format: None,
            speed: None,
        };

        let mut svc = self
            .service_pool
            .get_service(&params.model)
            .map_err(|e| rmcp::ErrorData::internal_error(e.to_string(), None))?;

        let resp = svc.call(LlmRequest::Speech(req)).await.map_err(to_error_data)?;

        match resp {
            LlmResponse::Speech(bytes) => {
                use base64::Engine;
                let b64 = base64::engine::general_purpose::STANDARD.encode(&bytes);
                Ok(CallToolResult::success(vec![Content::text(format!(
                    "Audio generated ({} bytes). Base64: {}",
                    bytes.len(),
                    b64
                ))]))
            }
            other => Err(rmcp::ErrorData::internal_error(
                format!("unexpected response variant: {other:?}"),
                None,
            )),
        }
    }

    #[tool(description = "Transcribe audio to text (speech-to-text)")]
    async fn transcribe(
        &self,
        Parameters(params): Parameters<params::TranscribeParams>,
    ) -> Result<CallToolResult, rmcp::ErrorData> {
        let req = CreateTranscriptionRequest {
            model: params.model.clone(),
            file: params.file_base64,
            language: None,
            prompt: None,
            response_format: None,
            temperature: None,
        };

        let mut svc = self
            .service_pool
            .get_service(&params.model)
            .map_err(|e| rmcp::ErrorData::internal_error(e.to_string(), None))?;

        let resp = svc.call(LlmRequest::Transcribe(req)).await.map_err(to_error_data)?;

        match resp {
            LlmResponse::Transcribe(r) => json_success(&r),
            other => Err(rmcp::ErrorData::internal_error(
                format!("unexpected response variant: {other:?}"),
                None,
            )),
        }
    }

    // ── Moderation ───────────────────────────────────────────────────────

    #[tool(description = "Check content against moderation policies")]
    async fn moderate(
        &self,
        Parameters(params): Parameters<params::ModerateParams>,
    ) -> Result<CallToolResult, rmcp::ErrorData> {
        let req: ModerationRequest = serde_json::from_value(serde_json::json!({
            "input": params.input,
            "model": params.model,
        }))
        .map_err(|e| rmcp::ErrorData::invalid_params(e.to_string(), None))?;

        // Use the model name if provided, otherwise fall back to first service.
        let mut svc = if let Some(ref model) = params.model {
            self.service_pool
                .get_service(model)
                .map_err(|e| rmcp::ErrorData::internal_error(e.to_string(), None))?
        } else {
            let names = self.service_pool.model_names();
            let first = names
                .first()
                .ok_or_else(|| rmcp::ErrorData::internal_error("no models configured", None))?;
            self.service_pool
                .get_service(first)
                .map_err(|e| rmcp::ErrorData::internal_error(e.to_string(), None))?
        };

        let resp = svc.call(LlmRequest::Moderate(req)).await.map_err(to_error_data)?;

        match resp {
            LlmResponse::Moderate(r) => json_success(&r),
            other => Err(rmcp::ErrorData::internal_error(
                format!("unexpected response variant: {other:?}"),
                None,
            )),
        }
    }

    // ── Rerank ───────────────────────────────────────────────────────────

    #[tool(description = "Rerank documents by relevance to a query")]
    async fn rerank(
        &self,
        Parameters(params): Parameters<params::RerankParams>,
    ) -> Result<CallToolResult, rmcp::ErrorData> {
        let req = RerankRequest {
            model: params.model.clone(),
            query: params.query,
            documents: params.documents.into_iter().map(RerankDocument::Text).collect(),
            top_n: None,
            return_documents: None,
        };

        let mut svc = self
            .service_pool
            .get_service(&params.model)
            .map_err(|e| rmcp::ErrorData::internal_error(e.to_string(), None))?;

        let resp = svc.call(LlmRequest::Rerank(req)).await.map_err(to_error_data)?;

        match resp {
            LlmResponse::Rerank(r) => json_success(&r),
            other => Err(rmcp::ErrorData::internal_error(
                format!("unexpected response variant: {other:?}"),
                None,
            )),
        }
    }

    // ── Search ───────────────────────────────────────────────────────────

    #[tool(description = "Perform a web or document search")]
    async fn search(
        &self,
        Parameters(params): Parameters<params::SearchParams>,
    ) -> Result<CallToolResult, rmcp::ErrorData> {
        let req = SearchRequest {
            model: params.model.clone(),
            query: params.query,
            max_results: None,
            search_domain_filter: None,
            country: None,
        };

        let mut svc = self
            .service_pool
            .get_service(&params.model)
            .map_err(|e| rmcp::ErrorData::internal_error(e.to_string(), None))?;

        let resp = svc.call(LlmRequest::Search(req)).await.map_err(to_error_data)?;

        match resp {
            LlmResponse::Search(r) => json_success(&r),
            other => Err(rmcp::ErrorData::internal_error(
                format!("unexpected response variant: {other:?}"),
                None,
            )),
        }
    }

    // ── OCR ──────────────────────────────────────────────────────────────

    #[tool(description = "Extract text from an image or document via OCR")]
    async fn ocr(&self, Parameters(params): Parameters<params::OcrParams>) -> Result<CallToolResult, rmcp::ErrorData> {
        let document = if let Some(url) = params.image_url {
            OcrDocument::Url { url }
        } else if let Some(data) = params.image_base64 {
            let media_type = params.media_type.unwrap_or_else(|| "image/png".to_string());
            OcrDocument::Base64 { data, media_type }
        } else {
            return Err(rmcp::ErrorData::invalid_params(
                "either image_url or image_base64 must be provided",
                None,
            ));
        };

        let req = OcrRequest {
            model: params.model.clone(),
            document,
            pages: None,
            include_image_base64: None,
        };

        let mut svc = self
            .service_pool
            .get_service(&params.model)
            .map_err(|e| rmcp::ErrorData::internal_error(e.to_string(), None))?;

        let resp = svc.call(LlmRequest::Ocr(req)).await.map_err(to_error_data)?;

        match resp {
            LlmResponse::Ocr(r) => json_success(&r),
            other => Err(rmcp::ErrorData::internal_error(
                format!("unexpected response variant: {other:?}"),
                None,
            )),
        }
    }

    // ── File operations ──────────────────────────────────────────────────

    #[tool(description = "Upload a file to the LLM provider")]
    async fn create_file(
        &self,
        Parameters(params): Parameters<params::CreateFileParams>,
    ) -> Result<CallToolResult, rmcp::ErrorData> {
        let purpose: FilePurpose = serde_json::from_value(serde_json::Value::String(params.purpose)).map_err(|e| {
            rmcp::ErrorData::invalid_params(
                format!("invalid purpose (expected assistants, batch, fine-tune, or vision): {e}"),
                None,
            )
        })?;

        let req = CreateFileRequest {
            file: params.file_base64,
            purpose,
            filename: Some(params.filename),
        };

        let client = self
            .service_pool
            .first_client()
            .map_err(|e| rmcp::ErrorData::internal_error(e.to_string(), None))?;

        let result = client.create_file(req).await.map_err(to_error_data)?;
        json_success(&result)
    }

    #[tool(description = "List uploaded files")]
    async fn list_files(
        &self,
        Parameters(params): Parameters<params::ListFilesParams>,
    ) -> Result<CallToolResult, rmcp::ErrorData> {
        let query = if params.purpose.is_some() || params.limit.is_some() {
            Some(FileListQuery {
                purpose: params.purpose,
                limit: params.limit,
                after: None,
            })
        } else {
            None
        };

        let client = self
            .service_pool
            .first_client()
            .map_err(|e| rmcp::ErrorData::internal_error(e.to_string(), None))?;

        let result = client.list_files(query).await.map_err(to_error_data)?;
        json_success(&result)
    }

    #[tool(description = "Retrieve metadata for an uploaded file")]
    async fn retrieve_file(
        &self,
        Parameters(params): Parameters<params::FileIdParams>,
    ) -> Result<CallToolResult, rmcp::ErrorData> {
        let client = self
            .service_pool
            .first_client()
            .map_err(|e| rmcp::ErrorData::internal_error(e.to_string(), None))?;

        let result = client.retrieve_file(&params.file_id).await.map_err(to_error_data)?;
        json_success(&result)
    }

    #[tool(description = "Delete an uploaded file")]
    async fn delete_file(
        &self,
        Parameters(params): Parameters<params::FileIdParams>,
    ) -> Result<CallToolResult, rmcp::ErrorData> {
        let client = self
            .service_pool
            .first_client()
            .map_err(|e| rmcp::ErrorData::internal_error(e.to_string(), None))?;

        let result = client.delete_file(&params.file_id).await.map_err(to_error_data)?;
        json_success(&result)
    }

    #[tool(description = "Retrieve the raw content of an uploaded file")]
    async fn file_content(
        &self,
        Parameters(params): Parameters<params::FileIdParams>,
    ) -> Result<CallToolResult, rmcp::ErrorData> {
        let client = self
            .service_pool
            .first_client()
            .map_err(|e| rmcp::ErrorData::internal_error(e.to_string(), None))?;

        let bytes = client.file_content(&params.file_id).await.map_err(to_error_data)?;

        use base64::Engine;
        let b64 = base64::engine::general_purpose::STANDARD.encode(&bytes);
        Ok(CallToolResult::success(vec![Content::text(format!(
            "File content ({} bytes). Base64: {b64}",
            bytes.len()
        ))]))
    }

    // ── Batch operations ─────────────────────────────────────────────────

    #[tool(description = "Create a new batch processing job")]
    async fn create_batch(
        &self,
        Parameters(params): Parameters<params::CreateBatchParams>,
    ) -> Result<CallToolResult, rmcp::ErrorData> {
        let req = CreateBatchRequest {
            input_file_id: params.input_file_id,
            endpoint: params.endpoint,
            completion_window: params.completion_window,
            metadata: None,
        };

        let client = self
            .service_pool
            .first_client()
            .map_err(|e| rmcp::ErrorData::internal_error(e.to_string(), None))?;

        let result = client.create_batch(req).await.map_err(to_error_data)?;
        json_success(&result)
    }

    #[tool(description = "List batch processing jobs")]
    async fn list_batches(
        &self,
        Parameters(params): Parameters<params::ListBatchesParams>,
    ) -> Result<CallToolResult, rmcp::ErrorData> {
        let query = if params.limit.is_some() || params.after.is_some() {
            Some(BatchListQuery {
                limit: params.limit,
                after: params.after,
            })
        } else {
            None
        };

        let client = self
            .service_pool
            .first_client()
            .map_err(|e| rmcp::ErrorData::internal_error(e.to_string(), None))?;

        let result = client.list_batches(query).await.map_err(to_error_data)?;
        json_success(&result)
    }

    #[tool(description = "Retrieve a batch processing job by ID")]
    async fn retrieve_batch(
        &self,
        Parameters(params): Parameters<params::BatchIdParams>,
    ) -> Result<CallToolResult, rmcp::ErrorData> {
        let client = self
            .service_pool
            .first_client()
            .map_err(|e| rmcp::ErrorData::internal_error(e.to_string(), None))?;

        let result = client.retrieve_batch(&params.batch_id).await.map_err(to_error_data)?;
        json_success(&result)
    }

    #[tool(description = "Cancel an in-progress batch processing job")]
    async fn cancel_batch(
        &self,
        Parameters(params): Parameters<params::BatchIdParams>,
    ) -> Result<CallToolResult, rmcp::ErrorData> {
        let client = self
            .service_pool
            .first_client()
            .map_err(|e| rmcp::ErrorData::internal_error(e.to_string(), None))?;

        let result = client.cancel_batch(&params.batch_id).await.map_err(to_error_data)?;
        json_success(&result)
    }

    // ── Response operations ──────────────────────────────────────────────

    #[tool(description = "Create a new response (Responses API)")]
    async fn create_response(
        &self,
        Parameters(params): Parameters<params::CreateResponseParams>,
    ) -> Result<CallToolResult, rmcp::ErrorData> {
        let req = CreateResponseRequest {
            model: params.model,
            input: params.input,
            instructions: None,
            tools: None,
            temperature: None,
            max_output_tokens: None,
            metadata: None,
        };

        let client = self
            .service_pool
            .first_client()
            .map_err(|e| rmcp::ErrorData::internal_error(e.to_string(), None))?;

        let result = client.create_response(req).await.map_err(to_error_data)?;
        json_success(&result)
    }

    #[tool(description = "Retrieve a response by ID (Responses API)")]
    async fn retrieve_response(
        &self,
        Parameters(params): Parameters<params::ResponseIdParams>,
    ) -> Result<CallToolResult, rmcp::ErrorData> {
        let client = self
            .service_pool
            .first_client()
            .map_err(|e| rmcp::ErrorData::internal_error(e.to_string(), None))?;

        let result = client
            .retrieve_response(&params.response_id)
            .await
            .map_err(to_error_data)?;
        json_success(&result)
    }

    #[tool(description = "Cancel an in-progress response (Responses API)")]
    async fn cancel_response(
        &self,
        Parameters(params): Parameters<params::ResponseIdParams>,
    ) -> Result<CallToolResult, rmcp::ErrorData> {
        let client = self
            .service_pool
            .first_client()
            .map_err(|e| rmcp::ErrorData::internal_error(e.to_string(), None))?;

        let result = client
            .cancel_response(&params.response_id)
            .await
            .map_err(to_error_data)?;
        json_success(&result)
    }
}

// ─── ServerHandler implementation ────────────────────────────────────────────

#[tool_handler]
impl ServerHandler for LiterLlmMcp {
    fn get_info(&self) -> ServerInfo {
        let mut capabilities = ServerCapabilities::default();
        capabilities.tools = Some(ToolsCapability::default());

        InitializeResult::new(capabilities)
            .with_server_info(Implementation::new("liter-llm", env!("CARGO_PKG_VERSION")))
            .with_instructions(
                "LiterLLM proxy — universal LLM API gateway with 142+ providers. \
                 Use the chat tool to send completion requests, embed for embeddings, \
                 and the file/batch/response tools for management operations.",
            )
    }
}
