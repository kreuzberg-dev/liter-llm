use axum::Json;
use serde::Serialize;
use utoipa::openapi::security::{Http, HttpAuthScheme, SecurityScheme};
use utoipa::{Modify, OpenApi, ToSchema};

/// OpenAI-compatible error response for OpenAPI documentation.
#[derive(Serialize, ToSchema)]
pub struct ProxyErrorBody {
    pub error: ProxyErrorDetail,
}

/// Detail within an OpenAI-compatible error response.
#[derive(Serialize, ToSchema)]
pub struct ProxyErrorDetail {
    pub message: String,
    #[serde(rename = "type")]
    pub error_type: String,
    pub param: Option<String>,
    pub code: Option<String>,
}

struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        if let Some(components) = openapi.components.as_mut() {
            components.add_security_scheme("bearer_auth", SecurityScheme::Http(Http::new(HttpAuthScheme::Bearer)));
        }
    }
}

#[derive(OpenApi)]
#[openapi(
    info(
        title = "liter-llm Proxy",
        version = "1.0.0",
        description = "OpenAI-compatible LLM proxy server — model routing, virtual keys, rate limiting, cost tracking.",
        license(name = "MIT"),
    ),
    servers(
        (url = "/", description = "Default server"),
    ),
    paths(
        crate::routes::chat::chat_completions,
        crate::routes::embeddings::create_embedding,
        crate::routes::models::list_models,
        crate::routes::images::create_image,
        crate::routes::audio::create_speech,
        crate::routes::audio::create_transcription,
        crate::routes::moderations::create_moderation,
        crate::routes::rerank::rerank,
        crate::routes::search::search,
        crate::routes::ocr::ocr,
        crate::routes::files::create_file,
        crate::routes::files::list_files,
        crate::routes::files::retrieve_file,
        crate::routes::files::delete_file,
        crate::routes::files::file_content,
        crate::routes::batches::create_batch,
        crate::routes::batches::list_batches,
        crate::routes::batches::retrieve_batch,
        crate::routes::batches::cancel_batch,
        crate::routes::responses::create_response,
        crate::routes::responses::retrieve_response,
        crate::routes::responses::cancel_response,
        crate::routes::health::health,
        crate::routes::health::liveness,
        crate::routes::health::readiness,
    ),
    components(schemas(ProxyErrorBody, ProxyErrorDetail, crate::routes::health::HealthResponse)),
    tags(
        (name = "chat", description = "Chat completions"),
        (name = "embeddings", description = "Text embeddings"),
        (name = "models", description = "Model listing"),
        (name = "images", description = "Image generation"),
        (name = "audio", description = "Audio speech and transcription"),
        (name = "moderations", description = "Content moderation"),
        (name = "rerank", description = "Document reranking"),
        (name = "search", description = "Web and document search"),
        (name = "ocr", description = "Optical character recognition"),
        (name = "files", description = "File management"),
        (name = "batches", description = "Batch processing"),
        (name = "responses", description = "Response management"),
        (name = "health", description = "Health checks"),
    ),
    security(("bearer_auth" = [])),
    modifiers(&SecurityAddon),
)]
pub struct ApiDoc;

/// GET /openapi.json
///
/// Returns the OpenAPI specification for the proxy API.
pub async fn openapi_schema() -> Json<utoipa::openapi::OpenApi> {
    Json(ApiDoc::openapi())
}
