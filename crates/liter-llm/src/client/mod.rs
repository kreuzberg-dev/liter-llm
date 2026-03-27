pub mod config;
#[cfg(all(feature = "native-http", feature = "tower"))]
pub mod managed;

use std::future::Future;
use std::pin::Pin;
#[cfg(feature = "native-http")]
use std::sync::Arc;

use futures_core::Stream;

use crate::error::Result;
use crate::types::audio::{CreateSpeechRequest, CreateTranscriptionRequest, TranscriptionResponse};
use crate::types::batch::{BatchListQuery, BatchListResponse, BatchObject, CreateBatchRequest};
use crate::types::files::{CreateFileRequest, DeleteResponse, FileListQuery, FileListResponse, FileObject};
use crate::types::image::{CreateImageRequest, ImagesResponse};
use crate::types::moderation::{ModerationRequest, ModerationResponse};
use crate::types::rerank::{RerankRequest, RerankResponse};
use crate::types::responses::{CreateResponseRequest, ResponseObject};
use crate::types::{
    ChatCompletionChunk, ChatCompletionRequest, ChatCompletionResponse, EmbeddingRequest, EmbeddingResponse,
    ModelsListResponse,
};

// DefaultClient and its LlmClient impl require reqwest + tokio.
#[cfg(feature = "native-http")]
use crate::auth::Credential;
#[cfg(feature = "native-http")]
use crate::error::LiterLlmError;
#[cfg(feature = "native-http")]
use crate::http;
#[cfg(feature = "native-http")]
use crate::provider::{self, OpenAiCompatibleProvider, OpenAiProvider, Provider};
#[cfg(feature = "native-http")]
use secrecy::ExposeSecret;

pub use config::{ClientConfig, ClientConfigBuilder};

/// A boxed future returning `Result<T>`.
pub type BoxFuture<'a, T> = Pin<Box<dyn Future<Output = Result<T>> + Send + 'a>>;

/// A boxed stream of `Result<T>`.
pub type BoxStream<'a, T> = Pin<Box<dyn Stream<Item = Result<T>> + Send + 'a>>;

/// Result of [`DefaultClient::prepare_request`]:
/// `(url, optional_auth_header, body_json, body_bytes)`.
///
/// The body is pre-serialized into `bytes::Bytes` so it is serialized exactly
/// once — the same bytes are used for signing headers and for the HTTP request
/// body.  On retry, cloning `Bytes` is a zero-copy ref-count bump.
///
/// `body_json` is the pre-serialization JSON value, retained so that
/// [`Provider::dynamic_headers`] can inspect request fields without
/// re-parsing.
///
/// The auth header is `None` when the provider requires no authentication
/// (e.g. local models or providers with `auth: none`).
/// Extra headers are accessed directly from the provider via `extra_headers()`.
#[cfg(feature = "native-http")]
type PreparedRequest = (String, Option<(String, String)>, serde_json::Value, bytes::Bytes);

/// Convert an owned `(String, String)` auth header pair to `(&str, &str)` borrows.
///
/// Centralises the four identical `map(|(n, v)| (n.as_str(), v.as_str()))` expressions
/// that appear wherever we hand headers to the HTTP layer.
#[cfg(feature = "native-http")]
fn str_pair(pair: &(String, String)) -> (&str, &str) {
    (pair.0.as_str(), pair.1.as_str())
}

/// Core LLM client trait.
pub trait LlmClient: Send + Sync {
    /// Send a chat completion request.
    fn chat(&self, req: ChatCompletionRequest) -> BoxFuture<'_, ChatCompletionResponse>;

    /// Send a streaming chat completion request.
    fn chat_stream(&self, req: ChatCompletionRequest) -> BoxFuture<'_, BoxStream<'_, ChatCompletionChunk>>;

    /// Send an embedding request.
    fn embed(&self, req: EmbeddingRequest) -> BoxFuture<'_, EmbeddingResponse>;

    /// List available models.
    fn list_models(&self) -> BoxFuture<'_, ModelsListResponse>;

    /// Generate an image.
    fn image_generate(&self, req: CreateImageRequest) -> BoxFuture<'_, ImagesResponse>;

    /// Generate speech audio from text.
    fn speech(&self, req: CreateSpeechRequest) -> BoxFuture<'_, bytes::Bytes>;

    /// Transcribe audio to text.
    fn transcribe(&self, req: CreateTranscriptionRequest) -> BoxFuture<'_, TranscriptionResponse>;

    /// Check content against moderation policies.
    fn moderate(&self, req: ModerationRequest) -> BoxFuture<'_, ModerationResponse>;

    /// Rerank documents by relevance to a query.
    fn rerank(&self, req: RerankRequest) -> BoxFuture<'_, RerankResponse>;
}

/// File management operations (upload, list, retrieve, delete).
pub trait FileClient: Send + Sync {
    /// Upload a file.
    fn create_file(&self, req: CreateFileRequest) -> BoxFuture<'_, FileObject>;

    /// Retrieve metadata for a file.
    fn retrieve_file(&self, file_id: &str) -> BoxFuture<'_, FileObject>;

    /// Delete a file.
    fn delete_file(&self, file_id: &str) -> BoxFuture<'_, DeleteResponse>;

    /// List files, optionally filtered by query parameters.
    fn list_files(&self, query: Option<FileListQuery>) -> BoxFuture<'_, FileListResponse>;

    /// Retrieve the raw content of a file.
    fn file_content(&self, file_id: &str) -> BoxFuture<'_, bytes::Bytes>;
}

/// Batch processing operations (create, list, retrieve, cancel).
pub trait BatchClient: Send + Sync {
    /// Create a new batch job.
    fn create_batch(&self, req: CreateBatchRequest) -> BoxFuture<'_, BatchObject>;

    /// Retrieve a batch by ID.
    fn retrieve_batch(&self, batch_id: &str) -> BoxFuture<'_, BatchObject>;

    /// List batches, optionally filtered by query parameters.
    fn list_batches(&self, query: Option<BatchListQuery>) -> BoxFuture<'_, BatchListResponse>;

    /// Cancel an in-progress batch.
    fn cancel_batch(&self, batch_id: &str) -> BoxFuture<'_, BatchObject>;
}

/// Responses API operations (create, retrieve, cancel).
pub trait ResponseClient: Send + Sync {
    /// Create a new response.
    fn create_response(&self, req: CreateResponseRequest) -> BoxFuture<'_, ResponseObject>;

    /// Retrieve a response by ID.
    fn retrieve_response(&self, id: &str) -> BoxFuture<'_, ResponseObject>;

    /// Cancel an in-progress response.
    fn cancel_response(&self, id: &str) -> BoxFuture<'_, ResponseObject>;
}

/// Default client implementation backed by `reqwest`.
///
/// The provider is resolved **once** at construction time.  For most
/// use-cases a single client talks to a single provider, so detecting the
/// provider per-request is unnecessary overhead and creates subtle bugs (e.g.
/// the old `list_models` hardcoded `"gpt-4"` as the detection key).
///
/// If you need to talk to multiple providers, create one `DefaultClient` per
/// provider.
///
/// The provider is stored behind an [`Arc`] so it can be shared cheaply into
/// async closures and streaming tasks that must be `'static`.
#[cfg(feature = "native-http")]
pub struct DefaultClient {
    config: ClientConfig,
    http: reqwest::Client,
    /// Provider resolved at construction; shared via Arc so streaming closures
    /// can capture an owned reference without requiring `unsafe`.
    provider: Arc<dyn Provider>,
    /// Pre-computed auth header `(name, value)` — avoids `format!("Bearer {key}")`
    /// on every request.  `None` when the provider requires no authentication.
    cached_auth_header: Option<(String, String)>,
    /// Pre-computed static extra headers — avoids converting `&'static str` pairs
    /// to `(String, String)` on every request.
    cached_extra_headers: Vec<(String, String)>,
}

#[cfg(feature = "native-http")]
impl DefaultClient {
    /// Build a client.
    ///
    /// `model_hint` guides provider auto-detection when no explicit
    /// `base_url` override is present in the config.  For example, passing
    /// `Some("groq/llama3-70b")` selects the Groq provider.  Pass `None` to
    /// default to OpenAI.
    ///
    /// # Errors
    ///
    /// Returns a wrapped [`reqwest::Error`] if the underlying HTTP client
    /// cannot be constructed.  Header names and values are pre-validated by
    /// [`ClientConfigBuilder::header`], so they are inserted directly here.
    pub fn new(config: ClientConfig, model_hint: Option<&str>) -> Result<Self> {
        let provider = build_provider(&config, model_hint);
        // Validate configuration eagerly so callers get a clear error at
        // construction time rather than on the first request.
        provider.validate()?;

        // Build the header map from pre-validated headers stored in the config.
        // The builder already validated each header name/value, so these
        // conversions are expected to succeed; return a proper error if they
        // somehow fail rather than panicking.
        let mut header_map = reqwest::header::HeaderMap::new();
        for (k, v) in config.headers() {
            let name =
                reqwest::header::HeaderName::from_bytes(k.as_bytes()).map_err(|_| LiterLlmError::InvalidHeader {
                    name: k.clone(),
                    reason: "pre-validated header name became invalid".into(),
                })?;
            let val = reqwest::header::HeaderValue::from_str(v).map_err(|_| LiterLlmError::InvalidHeader {
                name: k.clone(),
                reason: "pre-validated header value became invalid".into(),
            })?;
            header_map.insert(name, val);
        }

        let http = reqwest::Client::builder()
            .timeout(config.timeout)
            .default_headers(header_map)
            .build()
            .map_err(LiterLlmError::from)?;

        // Pre-compute the auth header once at construction time to avoid
        // `format!("Bearer {key}")` on every request.
        let cached_auth_header = provider
            .auth_header(config.api_key.expose_secret())
            .map(|(name, value)| (name.into_owned(), value.into_owned()));

        // Pre-compute static extra headers once to avoid `&'static str` ->
        // `String` conversion on every request.
        let cached_extra_headers = provider
            .extra_headers()
            .iter()
            .map(|&(name, value)| (name.to_owned(), value.to_owned()))
            .collect();

        Ok(Self {
            config,
            http,
            provider,
            cached_auth_header,
            cached_extra_headers,
        })
    }

    /// Shared helper: build the URL, resolve auth header strings, strip model
    /// prefix from the request body, set the `stream` flag, apply provider
    /// transform, and return everything needed to fire a request.
    ///
    /// `stream` is inserted into the body **before** `transform_request` runs,
    /// so providers can inspect the final body state in one pass.
    ///
    /// Returns `(url, optional_auth_header, body_value)` where the auth header
    /// is `None` when the provider requires no authentication.
    /// Extra headers are accessed directly from `self.cached_extra_headers`.
    fn prepare_request(
        &self,
        serializable: &impl serde::Serialize,
        endpoint_path: &str,
        model: &str,
        stream: Option<bool>,
    ) -> Result<PreparedRequest> {
        if model.is_empty() {
            return Err(LiterLlmError::BadRequest {
                message: "model must not be empty".into(),
            });
        }

        let bare_model = self.provider.strip_model_prefix(model).to_owned();
        // Use build_url so providers like Azure and Bedrock can embed the model
        // name or deployment identifier into the URL.
        let url = self.provider.build_url(endpoint_path, &bare_model);
        let auth_header = self.cached_auth_header.clone();

        let mut body = serde_json::to_value(serializable)?;
        if let Some(obj) = body.as_object_mut() {
            obj.insert("model".into(), serde_json::Value::String(bare_model));
            if let Some(s) = stream {
                obj.insert("stream".into(), serde_json::Value::Bool(s));
            }
        }
        self.provider.transform_request(&mut body)?;

        // Serialize exactly once — the same bytes are used for signing and for
        // the HTTP request body.  `Bytes` is reference-counted, so cloning on
        // retry is a zero-copy bump.
        let body_bytes = bytes::Bytes::from(serde_json::to_vec(&body)?);

        Ok((url, auth_header, body, body_bytes))
    }

    /// Resolve the auth header for a request.
    ///
    /// When a [`CredentialProvider`] is configured, it is called to obtain a
    /// fresh credential which overrides the pre-computed `cached_auth_header`.
    /// Otherwise the cached header (built at construction from the static
    /// `api_key`) is returned as-is.
    async fn resolve_auth_header(&self) -> Result<Option<(String, String)>> {
        if let Some(ref cp) = self.config.credential_provider {
            let credential = cp.resolve().await?;
            match credential {
                Credential::BearerToken(token) => Ok(Some((
                    "Authorization".to_owned(),
                    format!("Bearer {}", token.expose_secret()),
                ))),
                Credential::AwsCredentials { .. } => {
                    // AWS credentials are handled via signing_headers, not the auth header.
                    // Return None so the normal auth header is skipped.
                    Ok(None)
                }
            }
        } else {
            Ok(self.cached_auth_header.clone())
        }
    }

    /// Build the combined header list for a request.
    ///
    /// Merges the provider's pre-computed static [`Provider::extra_headers`], the
    /// dynamic signing headers returned by [`Provider::signing_headers`],
    /// and the per-request [`Provider::dynamic_headers`] computed from the
    /// JSON body.  Returns an owned vec of `(name, value)` pairs; callers
    /// borrow these for the HTTP layer.
    fn all_headers(
        &self,
        method: &str,
        url: &str,
        body_json: &serde_json::Value,
        body_bytes: &[u8],
    ) -> Vec<(String, String)> {
        // Start with dynamic signing headers (e.g. SigV4 Authorization + x-amz-date).
        let mut headers = self.provider.signing_headers(method, url, body_bytes);
        // Append pre-computed static provider extra headers (e.g. anthropic-version).
        headers.extend(self.cached_extra_headers.iter().cloned());
        // Append per-request dynamic headers (e.g. anthropic-beta).
        headers.extend(self.provider.dynamic_headers(body_json));
        headers
    }
}

#[cfg(feature = "native-http")]
/// Resolve the provider to use for all requests on this client.
///
/// Priority:
/// 1. Explicit `base_url` in config -> custom OpenAI-compatible provider.
/// 2. `model_hint` -> auto-detect by model name prefix.
/// 3. Default -> OpenAI.
fn build_provider(config: &ClientConfig, model_hint: Option<&str>) -> Arc<dyn Provider> {
    if let Some(ref base_url) = config.base_url {
        return Arc::new(OpenAiCompatibleProvider {
            name: "custom".into(),
            base_url: base_url.clone(),
            env_var: None,
            model_prefixes: vec![],
        });
    }

    if let Some(model) = model_hint
        && let Some(p) = provider::detect_provider(model)
    {
        // detect_provider returns Box<dyn Provider>; convert to Arc.
        return Arc::from(p);
    }

    Arc::new(OpenAiProvider)
}

#[cfg(feature = "native-http")]
impl LlmClient for DefaultClient {
    fn chat(&self, req: ChatCompletionRequest) -> BoxFuture<'_, ChatCompletionResponse> {
        Box::pin(async move {
            // Pass stream=false so providers can inspect the flag in transform_request.
            let (url, _cached_auth, body_json, body_bytes) =
                self.prepare_request(&req, self.provider.chat_completions_path(), &req.model, Some(false))?;

            let auth_header = self.resolve_auth_header().await?;
            let all_headers = self.all_headers("POST", &url, &body_json, &body_bytes);
            let extra: Vec<(&str, &str)> = all_headers.iter().map(|(n, v)| (n.as_str(), v.as_str())).collect();

            let auth = auth_header.as_ref().map(str_pair);
            let mut raw =
                http::request::post_json_raw(&self.http, &url, auth, &extra, body_bytes, self.config.max_retries)
                    .await?;
            self.provider.transform_response(&mut raw)?;
            serde_json::from_value::<ChatCompletionResponse>(raw).map_err(LiterLlmError::from)
        })
    }

    fn chat_stream(&self, req: ChatCompletionRequest) -> BoxFuture<'_, BoxStream<'_, ChatCompletionChunk>> {
        Box::pin(async move {
            // Use prepare_request for validation, model-prefix stripping, and
            // transform_request — then override the URL via build_stream_url.
            let (_base_url, _cached_auth, body_json, body_bytes) =
                self.prepare_request(&req, self.provider.chat_completions_path(), &req.model, Some(true))?;

            // Always use build_stream_url for the streaming endpoint.
            // The default implementation delegates to build_url, so this is safe
            // for all providers.  Providers with a distinct streaming endpoint
            // (e.g. Bedrock /converse-stream) override build_stream_url.
            let bare_model = self.provider.strip_model_prefix(&req.model);
            let url = self
                .provider
                .build_stream_url(self.provider.chat_completions_path(), bare_model);

            let auth_header = self.resolve_auth_header().await?;
            let all_headers = self.all_headers("POST", &url, &body_json, &body_bytes);
            let extra: Vec<(&str, &str)> = all_headers.iter().map(|(n, v)| (n.as_str(), v.as_str())).collect();
            let auth = auth_header.as_ref().map(str_pair);

            match self.provider.stream_format() {
                provider::StreamFormat::Sse => {
                    let provider = Arc::clone(&self.provider);
                    let parse_event = move |data: &str| provider.parse_stream_event(data);
                    let stream = http::streaming::post_stream(
                        &self.http,
                        &url,
                        auth,
                        &extra,
                        body_bytes,
                        self.config.max_retries,
                        parse_event,
                    )
                    .await?;
                    Ok(stream)
                }
                provider::StreamFormat::AwsEventStream => {
                    let stream = http::eventstream::post_eventstream(
                        &self.http,
                        &url,
                        auth,
                        &extra,
                        body_bytes,
                        self.config.max_retries,
                        provider::bedrock::parse_bedrock_stream_event,
                    )
                    .await?;
                    Ok(stream)
                }
            }
        })
    }

    fn embed(&self, req: EmbeddingRequest) -> BoxFuture<'_, EmbeddingResponse> {
        Box::pin(async move {
            // Embeddings have no stream flag; pass None so it is not inserted.
            let (url, _cached_auth, body_json, body_bytes) =
                self.prepare_request(&req, self.provider.embeddings_path(), &req.model, None)?;

            let auth_header = self.resolve_auth_header().await?;
            let all_headers = self.all_headers("POST", &url, &body_json, &body_bytes);
            let extra: Vec<(&str, &str)> = all_headers.iter().map(|(n, v)| (n.as_str(), v.as_str())).collect();

            let auth = auth_header.as_ref().map(str_pair);
            let mut raw =
                http::request::post_json_raw(&self.http, &url, auth, &extra, body_bytes, self.config.max_retries)
                    .await?;
            self.provider.transform_response(&mut raw)?;
            serde_json::from_value::<EmbeddingResponse>(raw).map_err(LiterLlmError::from)
        })
    }

    fn list_models(&self) -> BoxFuture<'_, ModelsListResponse> {
        Box::pin(async move {
            // Use build_url so providers like Azure/Bedrock can customise the URL.
            let url = self.provider.build_url(self.provider.models_path(), "");
            let auth_header = self.resolve_auth_header().await?;
            let auth = auth_header.as_ref().map(str_pair);
            // list_models is a GET request; signing headers use an empty body,
            // and dynamic_headers receives a null JSON value.
            let all_headers = self.all_headers("GET", &url, &serde_json::Value::Null, &[]);
            let extra: Vec<(&str, &str)> = all_headers.iter().map(|(n, v)| (n.as_str(), v.as_str())).collect();

            http::request::get_json(&self.http, &url, auth, &extra, self.config.max_retries).await
        })
    }

    fn image_generate(&self, req: CreateImageRequest) -> BoxFuture<'_, ImagesResponse> {
        Box::pin(async move {
            let model = req.model.as_deref().unwrap_or_default();
            let (url, _cached_auth, body_json, body_bytes) =
                self.prepare_request(&req, self.provider.image_generations_path(), model, None)?;

            let auth_header = self.resolve_auth_header().await?;
            let all_headers = self.all_headers("POST", &url, &body_json, &body_bytes);
            let extra: Vec<(&str, &str)> = all_headers.iter().map(|(n, v)| (n.as_str(), v.as_str())).collect();

            let auth = auth_header.as_ref().map(str_pair);
            let mut raw =
                http::request::post_json_raw(&self.http, &url, auth, &extra, body_bytes, self.config.max_retries)
                    .await?;
            self.provider.transform_response(&mut raw)?;
            serde_json::from_value::<ImagesResponse>(raw).map_err(LiterLlmError::from)
        })
    }

    fn speech(&self, req: CreateSpeechRequest) -> BoxFuture<'_, bytes::Bytes> {
        Box::pin(async move {
            let (url, _cached_auth, body_json, body_bytes) =
                self.prepare_request(&req, self.provider.audio_speech_path(), &req.model, None)?;

            let auth_header = self.resolve_auth_header().await?;
            let all_headers = self.all_headers("POST", &url, &body_json, &body_bytes);
            let extra: Vec<(&str, &str)> = all_headers.iter().map(|(n, v)| (n.as_str(), v.as_str())).collect();

            let auth = auth_header.as_ref().map(str_pair);
            http::request::post_binary(&self.http, &url, auth, &extra, body_bytes, self.config.max_retries).await
        })
    }

    fn transcribe(&self, req: CreateTranscriptionRequest) -> BoxFuture<'_, TranscriptionResponse> {
        Box::pin(async move {
            let (url, _cached_auth, body_json, body_bytes) =
                self.prepare_request(&req, self.provider.audio_transcriptions_path(), &req.model, None)?;

            let auth_header = self.resolve_auth_header().await?;
            let all_headers = self.all_headers("POST", &url, &body_json, &body_bytes);
            let extra: Vec<(&str, &str)> = all_headers.iter().map(|(n, v)| (n.as_str(), v.as_str())).collect();

            let auth = auth_header.as_ref().map(str_pair);
            let mut raw =
                http::request::post_json_raw(&self.http, &url, auth, &extra, body_bytes, self.config.max_retries)
                    .await?;
            self.provider.transform_response(&mut raw)?;
            serde_json::from_value::<TranscriptionResponse>(raw).map_err(LiterLlmError::from)
        })
    }

    fn moderate(&self, req: ModerationRequest) -> BoxFuture<'_, ModerationResponse> {
        Box::pin(async move {
            let model = req.model.as_deref().unwrap_or_default();
            let (url, _cached_auth, body_json, body_bytes) =
                self.prepare_request(&req, self.provider.moderations_path(), model, None)?;

            let auth_header = self.resolve_auth_header().await?;
            let all_headers = self.all_headers("POST", &url, &body_json, &body_bytes);
            let extra: Vec<(&str, &str)> = all_headers.iter().map(|(n, v)| (n.as_str(), v.as_str())).collect();

            let auth = auth_header.as_ref().map(str_pair);
            let mut raw =
                http::request::post_json_raw(&self.http, &url, auth, &extra, body_bytes, self.config.max_retries)
                    .await?;
            self.provider.transform_response(&mut raw)?;
            serde_json::from_value::<ModerationResponse>(raw).map_err(LiterLlmError::from)
        })
    }

    fn rerank(&self, req: RerankRequest) -> BoxFuture<'_, RerankResponse> {
        Box::pin(async move {
            let (url, _cached_auth, body_json, body_bytes) =
                self.prepare_request(&req, self.provider.rerank_path(), &req.model, None)?;

            let auth_header = self.resolve_auth_header().await?;
            let all_headers = self.all_headers("POST", &url, &body_json, &body_bytes);
            let extra: Vec<(&str, &str)> = all_headers.iter().map(|(n, v)| (n.as_str(), v.as_str())).collect();

            let auth = auth_header.as_ref().map(str_pair);
            let mut raw =
                http::request::post_json_raw(&self.http, &url, auth, &extra, body_bytes, self.config.max_retries)
                    .await?;
            self.provider.transform_response(&mut raw)?;
            serde_json::from_value::<RerankResponse>(raw).map_err(LiterLlmError::from)
        })
    }
}

#[cfg(feature = "native-http")]
impl FileClient for DefaultClient {
    fn create_file(&self, req: CreateFileRequest) -> BoxFuture<'_, FileObject> {
        Box::pin(async move {
            let url = self.provider.build_url(self.provider.files_path(), "");
            let auth_header = self.resolve_auth_header().await?;
            let auth = auth_header.as_ref().map(str_pair);
            let all_headers = self.all_headers("POST", &url, &serde_json::Value::Null, &[]);
            let extra: Vec<(&str, &str)> = all_headers.iter().map(|(n, v)| (n.as_str(), v.as_str())).collect();

            // Decode the base64-encoded file data into raw bytes for the multipart upload.
            use base64::Engine;
            let file_bytes = base64::engine::general_purpose::STANDARD
                .decode(&req.file)
                .map_err(|e| LiterLlmError::BadRequest {
                    message: format!("invalid base64 file data: {e}"),
                })?;

            let filename = req.filename.unwrap_or_else(|| "upload".to_owned());
            let file_part = reqwest::multipart::Part::bytes(file_bytes).file_name(filename);
            let purpose_str = serde_json::to_value(&req.purpose)?
                .as_str()
                .unwrap_or_default()
                .to_owned();
            let form = reqwest::multipart::Form::new()
                .part("file", file_part)
                .text("purpose", purpose_str);

            let raw = http::request::post_multipart(&self.http, &url, auth, &extra, form).await?;
            serde_json::from_value::<FileObject>(raw).map_err(LiterLlmError::from)
        })
    }

    fn retrieve_file(&self, file_id: &str) -> BoxFuture<'_, FileObject> {
        let file_id = file_id.to_owned();
        Box::pin(async move {
            let url = format!(
                "{}/{}",
                self.provider.build_url(self.provider.files_path(), ""),
                file_id
            );
            let auth_header = self.resolve_auth_header().await?;
            let auth = auth_header.as_ref().map(str_pair);
            let all_headers = self.all_headers("GET", &url, &serde_json::Value::Null, &[]);
            let extra: Vec<(&str, &str)> = all_headers.iter().map(|(n, v)| (n.as_str(), v.as_str())).collect();

            let raw = http::request::get_json_raw(&self.http, &url, auth, &extra, self.config.max_retries).await?;
            serde_json::from_value::<FileObject>(raw).map_err(LiterLlmError::from)
        })
    }

    fn delete_file(&self, file_id: &str) -> BoxFuture<'_, DeleteResponse> {
        let file_id = file_id.to_owned();
        Box::pin(async move {
            let url = format!(
                "{}/{}",
                self.provider.build_url(self.provider.files_path(), ""),
                file_id
            );
            let auth_header = self.resolve_auth_header().await?;
            let auth = auth_header.as_ref().map(str_pair);
            let all_headers = self.all_headers("DELETE", &url, &serde_json::Value::Null, &[]);
            let extra: Vec<(&str, &str)> = all_headers.iter().map(|(n, v)| (n.as_str(), v.as_str())).collect();

            let raw = http::request::delete_json(&self.http, &url, auth, &extra, self.config.max_retries).await?;
            serde_json::from_value::<DeleteResponse>(raw).map_err(LiterLlmError::from)
        })
    }

    fn list_files(&self, query: Option<FileListQuery>) -> BoxFuture<'_, FileListResponse> {
        Box::pin(async move {
            let base_url = self.provider.build_url(self.provider.files_path(), "");
            let url = if let Some(ref q) = query {
                let mut params = Vec::new();
                if let Some(ref purpose) = q.purpose {
                    params.push(format!("purpose={purpose}"));
                }
                if let Some(limit) = q.limit {
                    params.push(format!("limit={limit}"));
                }
                if let Some(ref after) = q.after {
                    params.push(format!("after={after}"));
                }
                if params.is_empty() {
                    base_url
                } else {
                    format!("{base_url}?{}", params.join("&"))
                }
            } else {
                base_url
            };
            let auth_header = self.resolve_auth_header().await?;
            let auth = auth_header.as_ref().map(str_pair);
            let all_headers = self.all_headers("GET", &url, &serde_json::Value::Null, &[]);
            let extra: Vec<(&str, &str)> = all_headers.iter().map(|(n, v)| (n.as_str(), v.as_str())).collect();

            let raw = http::request::get_json_raw(&self.http, &url, auth, &extra, self.config.max_retries).await?;
            serde_json::from_value::<FileListResponse>(raw).map_err(LiterLlmError::from)
        })
    }

    fn file_content(&self, file_id: &str) -> BoxFuture<'_, bytes::Bytes> {
        let file_id = file_id.to_owned();
        Box::pin(async move {
            let url = format!(
                "{}/{}/content",
                self.provider.build_url(self.provider.files_path(), ""),
                file_id
            );
            let auth_header = self.resolve_auth_header().await?;
            let auth = auth_header.as_ref().map(str_pair);
            let all_headers = self.all_headers("GET", &url, &serde_json::Value::Null, &[]);
            let extra: Vec<(&str, &str)> = all_headers.iter().map(|(n, v)| (n.as_str(), v.as_str())).collect();

            http::request::get_binary(&self.http, &url, auth, &extra, self.config.max_retries).await
        })
    }
}

#[cfg(feature = "native-http")]
impl BatchClient for DefaultClient {
    fn create_batch(&self, req: CreateBatchRequest) -> BoxFuture<'_, BatchObject> {
        Box::pin(async move {
            let url = self.provider.build_url(self.provider.batches_path(), "");
            let body_bytes = bytes::Bytes::from(serde_json::to_vec(&req)?);
            let body_json = serde_json::to_value(&req)?;

            let auth_header = self.resolve_auth_header().await?;
            let all_headers = self.all_headers("POST", &url, &body_json, &body_bytes);
            let extra: Vec<(&str, &str)> = all_headers.iter().map(|(n, v)| (n.as_str(), v.as_str())).collect();
            let auth = auth_header.as_ref().map(str_pair);

            let raw = http::request::post_json_raw(&self.http, &url, auth, &extra, body_bytes, self.config.max_retries)
                .await?;
            serde_json::from_value::<BatchObject>(raw).map_err(LiterLlmError::from)
        })
    }

    fn retrieve_batch(&self, batch_id: &str) -> BoxFuture<'_, BatchObject> {
        let batch_id = batch_id.to_owned();
        Box::pin(async move {
            let url = format!(
                "{}/{}",
                self.provider.build_url(self.provider.batches_path(), ""),
                batch_id
            );
            let auth_header = self.resolve_auth_header().await?;
            let auth = auth_header.as_ref().map(str_pair);
            let all_headers = self.all_headers("GET", &url, &serde_json::Value::Null, &[]);
            let extra: Vec<(&str, &str)> = all_headers.iter().map(|(n, v)| (n.as_str(), v.as_str())).collect();

            let raw = http::request::get_json_raw(&self.http, &url, auth, &extra, self.config.max_retries).await?;
            serde_json::from_value::<BatchObject>(raw).map_err(LiterLlmError::from)
        })
    }

    fn list_batches(&self, query: Option<BatchListQuery>) -> BoxFuture<'_, BatchListResponse> {
        Box::pin(async move {
            let base_url = self.provider.build_url(self.provider.batches_path(), "");
            let url = if let Some(ref q) = query {
                let mut params = Vec::new();
                if let Some(limit) = q.limit {
                    params.push(format!("limit={limit}"));
                }
                if let Some(ref after) = q.after {
                    params.push(format!("after={after}"));
                }
                if params.is_empty() {
                    base_url
                } else {
                    format!("{base_url}?{}", params.join("&"))
                }
            } else {
                base_url
            };
            let auth_header = self.resolve_auth_header().await?;
            let auth = auth_header.as_ref().map(str_pair);
            let all_headers = self.all_headers("GET", &url, &serde_json::Value::Null, &[]);
            let extra: Vec<(&str, &str)> = all_headers.iter().map(|(n, v)| (n.as_str(), v.as_str())).collect();

            let raw = http::request::get_json_raw(&self.http, &url, auth, &extra, self.config.max_retries).await?;
            serde_json::from_value::<BatchListResponse>(raw).map_err(LiterLlmError::from)
        })
    }

    fn cancel_batch(&self, batch_id: &str) -> BoxFuture<'_, BatchObject> {
        let batch_id = batch_id.to_owned();
        Box::pin(async move {
            let url = format!(
                "{}/{}/cancel",
                self.provider.build_url(self.provider.batches_path(), ""),
                batch_id
            );
            let auth_header = self.resolve_auth_header().await?;
            let body_json = serde_json::Value::Null;
            let body_bytes = bytes::Bytes::new();
            let all_headers = self.all_headers("POST", &url, &body_json, &body_bytes);
            let extra: Vec<(&str, &str)> = all_headers.iter().map(|(n, v)| (n.as_str(), v.as_str())).collect();
            let auth = auth_header.as_ref().map(str_pair);

            let raw = http::request::post_json_raw(&self.http, &url, auth, &extra, body_bytes, self.config.max_retries)
                .await?;
            serde_json::from_value::<BatchObject>(raw).map_err(LiterLlmError::from)
        })
    }
}

#[cfg(feature = "native-http")]
impl ResponseClient for DefaultClient {
    fn create_response(&self, req: CreateResponseRequest) -> BoxFuture<'_, ResponseObject> {
        Box::pin(async move {
            let url = self.provider.build_url(self.provider.responses_path(), "");
            let body_bytes = bytes::Bytes::from(serde_json::to_vec(&req)?);
            let body_json = serde_json::to_value(&req)?;

            let auth_header = self.resolve_auth_header().await?;
            let all_headers = self.all_headers("POST", &url, &body_json, &body_bytes);
            let extra: Vec<(&str, &str)> = all_headers.iter().map(|(n, v)| (n.as_str(), v.as_str())).collect();
            let auth = auth_header.as_ref().map(str_pair);

            let raw = http::request::post_json_raw(&self.http, &url, auth, &extra, body_bytes, self.config.max_retries)
                .await?;
            serde_json::from_value::<ResponseObject>(raw).map_err(LiterLlmError::from)
        })
    }

    fn retrieve_response(&self, id: &str) -> BoxFuture<'_, ResponseObject> {
        let id = id.to_owned();
        Box::pin(async move {
            let url = format!("{}/{}", self.provider.build_url(self.provider.responses_path(), ""), id);
            let auth_header = self.resolve_auth_header().await?;
            let auth = auth_header.as_ref().map(str_pair);
            let all_headers = self.all_headers("GET", &url, &serde_json::Value::Null, &[]);
            let extra: Vec<(&str, &str)> = all_headers.iter().map(|(n, v)| (n.as_str(), v.as_str())).collect();

            let raw = http::request::get_json_raw(&self.http, &url, auth, &extra, self.config.max_retries).await?;
            serde_json::from_value::<ResponseObject>(raw).map_err(LiterLlmError::from)
        })
    }

    fn cancel_response(&self, id: &str) -> BoxFuture<'_, ResponseObject> {
        let id = id.to_owned();
        Box::pin(async move {
            let url = format!(
                "{}/{}/cancel",
                self.provider.build_url(self.provider.responses_path(), ""),
                id
            );
            let auth_header = self.resolve_auth_header().await?;
            let body_json = serde_json::Value::Null;
            let body_bytes = bytes::Bytes::new();
            let all_headers = self.all_headers("POST", &url, &body_json, &body_bytes);
            let extra: Vec<(&str, &str)> = all_headers.iter().map(|(n, v)| (n.as_str(), v.as_str())).collect();
            let auth = auth_header.as_ref().map(str_pair);

            let raw = http::request::post_json_raw(&self.http, &url, auth, &extra, body_bytes, self.config.max_retries)
                .await?;
            serde_json::from_value::<ResponseObject>(raw).map_err(LiterLlmError::from)
        })
    }
}
