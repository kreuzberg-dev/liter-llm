pub mod config;

use std::future::Future;
use std::pin::Pin;

use futures_core::Stream;

use crate::error::Result;
use crate::types::{
    ChatCompletionChunk, ChatCompletionRequest, ChatCompletionResponse, EmbeddingRequest, EmbeddingResponse,
    ModelsListResponse,
};

// DefaultClient and its LlmClient impl require reqwest + tokio.
#[cfg(feature = "native-http")]
use crate::error::LiterLmError;
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
/// `(url, optional_auth_header, extra_headers, body)`.
///
/// The auth header is `None` when the provider requires no authentication
/// (e.g. local models or providers with `auth: none`).
/// Extra headers carry provider-specific mandatory headers (e.g. `anthropic-version`).
#[cfg(feature = "native-http")]
type PreparedRequest = (
    String,
    Option<(String, String)>,
    Vec<(String, String)>,
    serde_json::Value,
);

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
#[cfg(feature = "native-http")]
pub struct DefaultClient {
    config: ClientConfig,
    http: reqwest::Client,
    /// Provider resolved at construction; used for all requests.
    provider: Box<dyn Provider>,
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

        // Build the header map from pre-validated headers stored in the config.
        // The builder already validated each header name/value, so these
        // conversions are expected to succeed; return a proper error if they
        // somehow fail rather than panicking.
        let mut header_map = reqwest::header::HeaderMap::new();
        for (k, v) in config.headers() {
            let name =
                reqwest::header::HeaderName::from_bytes(k.as_bytes()).map_err(|_| LiterLmError::InvalidHeader {
                    name: k.clone(),
                    reason: "pre-validated header name became invalid".into(),
                })?;
            let val = reqwest::header::HeaderValue::from_str(v).map_err(|_| LiterLmError::InvalidHeader {
                name: k.clone(),
                reason: "pre-validated header value became invalid".into(),
            })?;
            header_map.insert(name, val);
        }

        let http = reqwest::Client::builder()
            .timeout(config.timeout)
            .default_headers(header_map)
            .build()
            .map_err(LiterLmError::from)?;

        Ok(Self { config, http, provider })
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
    fn prepare_request(
        &self,
        serializable: &impl serde::Serialize,
        endpoint_path: &str,
        model: &str,
        stream: Option<bool>,
    ) -> Result<PreparedRequest> {
        if model.is_empty() {
            return Err(LiterLmError::BadRequest {
                message: "model must not be empty".into(),
            });
        }

        let url = format!("{}{}", self.provider.base_url(), endpoint_path);
        let auth_header = self
            .provider
            .auth_header(self.config.api_key.expose_secret())
            .map(|(name, value)| (name.into_owned(), value.into_owned()));

        let extra_headers: Vec<(String, String)> = self
            .provider
            .extra_headers()
            .into_iter()
            .map(|(k, v)| (k.into_owned(), v.into_owned()))
            .collect();

        let bare_model = self.provider.strip_model_prefix(model).to_owned();

        let mut body = serde_json::to_value(serializable)?;
        if let Some(obj) = body.as_object_mut() {
            obj.insert("model".into(), serde_json::Value::String(bare_model));
            if let Some(s) = stream {
                obj.insert("stream".into(), serde_json::Value::Bool(s));
            }
        }
        self.provider.transform_request(&mut body)?;

        Ok((url, auth_header, extra_headers, body))
    }
}

#[cfg(feature = "native-http")]
/// Resolve the provider to use for all requests on this client.
///
/// Priority:
/// 1. Explicit `base_url` in config → custom OpenAI-compatible provider.
/// 2. `model_hint` → auto-detect by model name prefix.
/// 3. Default → OpenAI.
fn build_provider(config: &ClientConfig, model_hint: Option<&str>) -> Box<dyn Provider> {
    if let Some(ref base_url) = config.base_url {
        return Box::new(OpenAiCompatibleProvider {
            name: "custom".into(),
            base_url: base_url.clone(),
            env_var: String::new(),
            model_prefixes: vec![],
        });
    }

    if let Some(model) = model_hint
        && let Some(p) = provider::detect_provider(model)
    {
        return p;
    }

    Box::new(OpenAiProvider)
}

#[cfg(feature = "native-http")]
impl LlmClient for DefaultClient {
    fn chat(&self, req: ChatCompletionRequest) -> BoxFuture<'_, ChatCompletionResponse> {
        Box::pin(async move {
            // Pass stream=false so providers can inspect the flag in transform_request.
            let (url, auth_header, extra_headers, body) =
                self.prepare_request(&req, self.provider.chat_completions_path(), &req.model, Some(false))?;

            let auth = auth_header.as_ref().map(|(n, v)| (n.as_str(), v.as_str()));
            let extra: Vec<(&str, &str)> = extra_headers.iter().map(|(k, v)| (k.as_str(), v.as_str())).collect();
            http::request::post_json(&self.http, &url, auth, &extra, body, self.config.max_retries).await
        })
    }

    fn chat_stream(&self, req: ChatCompletionRequest) -> BoxFuture<'_, BoxStream<'_, ChatCompletionChunk>> {
        Box::pin(async move {
            // Pass stream=true so providers can inspect the flag in transform_request.
            let (url, auth_header, extra_headers, body) =
                self.prepare_request(&req, self.provider.chat_completions_path(), &req.model, Some(true))?;

            let auth = auth_header.as_ref().map(|(n, v)| (n.as_str(), v.as_str()));
            let extra: Vec<(&str, &str)> = extra_headers.iter().map(|(k, v)| (k.as_str(), v.as_str())).collect();
            let stream =
                http::streaming::post_stream(&self.http, &url, auth, &extra, body, self.config.max_retries).await?;
            Ok(stream)
        })
    }

    fn embed(&self, req: EmbeddingRequest) -> BoxFuture<'_, EmbeddingResponse> {
        Box::pin(async move {
            // Embeddings have no stream flag; pass None so it is not inserted.
            let (url, auth_header, extra_headers, body) =
                self.prepare_request(&req, self.provider.embeddings_path(), &req.model, None)?;

            let auth = auth_header.as_ref().map(|(n, v)| (n.as_str(), v.as_str()));
            let extra: Vec<(&str, &str)> = extra_headers.iter().map(|(k, v)| (k.as_str(), v.as_str())).collect();
            http::request::post_json(&self.http, &url, auth, &extra, body, self.config.max_retries).await
        })
    }

    fn list_models(&self) -> BoxFuture<'_, ModelsListResponse> {
        Box::pin(async move {
            // Use the stored provider — no more hardcoded "gpt-4" fallback.
            let url = format!("{}{}", self.provider.base_url(), self.provider.models_path());
            let auth_header = self
                .provider
                .auth_header(self.config.api_key.expose_secret())
                .map(|(name, value)| (name.into_owned(), value.into_owned()));
            let auth = auth_header.as_ref().map(|(n, v)| (n.as_str(), v.as_str()));
            let extra_headers: Vec<(String, String)> = self
                .provider
                .extra_headers()
                .into_iter()
                .map(|(k, v)| (k.into_owned(), v.into_owned()))
                .collect();
            let extra: Vec<(&str, &str)> = extra_headers.iter().map(|(k, v)| (k.as_str(), v.as_str())).collect();

            http::request::get_json(&self.http, &url, auth, &extra, self.config.max_retries).await
        })
    }
}
