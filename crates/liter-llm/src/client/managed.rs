//! A managed LLM client that optionally routes requests through a Tower
//! middleware stack (cache, budget, hooks) when the corresponding
//! [`ClientConfig`] fields are set.
//!
//! When no middleware is configured the client delegates directly to the
//! underlying [`DefaultClient`], adding zero overhead.  When middleware *is*
//! configured, each [`LlmClient`] method converts its typed request into an
//! [`LlmRequest`], sends it through a cloned Tower service stack, and extracts
//! the typed response from the resulting [`LlmResponse`].
//!
//! # Tower `Service::call` takes `&mut self`
//!
//! The [`LlmClient`] trait requires `&self` receivers but Tower's
//! `Service::call` takes `&mut self`.  All our middleware services are `Clone`
//! (state is behind `Arc`) so we clone the service per call — this is a cheap
//! series of `Arc` reference-count bumps.
//!
//! Tower's [`BoxCloneService`](tower::util::BoxCloneService) is `Send` but not
//! `Sync` (its inner trait object is `dyn ... + Send`).  Since [`LlmClient`]
//! requires `Sync`, we wrap the service in a [`std::sync::Mutex`] that is held
//! only for the brief duration of `Clone::clone` (a few `Arc` ref-count bumps).
//! This makes `ManagedClient` `Sync` with negligible contention.

use std::sync::{Arc, Mutex};

use tower::{Layer, Service};

use super::config::ClientConfig;
use super::{BoxFuture, BoxStream, DefaultClient, LlmClient};
use crate::error::{LiterLlmError, Result};
use crate::tower::types::{LlmRequest, LlmResponse};
use crate::tower::{BudgetLayer, BudgetState, CacheLayer, HooksLayer, LlmService};
use crate::types::audio::{CreateSpeechRequest, CreateTranscriptionRequest, TranscriptionResponse};
use crate::types::image::{CreateImageRequest, ImagesResponse};
use crate::types::moderation::{ModerationRequest, ModerationResponse};
use crate::types::rerank::{RerankRequest, RerankResponse};
use crate::types::{
    ChatCompletionChunk, ChatCompletionRequest, ChatCompletionResponse, EmbeddingRequest, EmbeddingResponse,
    ModelsListResponse,
};

// ---------------------------------------------------------------------------
// Type-erased Tower service wrapper
// ---------------------------------------------------------------------------

/// A `Send + Sync` wrapper around [`tower::util::BoxCloneService`].
///
/// `BoxCloneService` is `Send` but not `Sync` because its inner trait object
/// only requires `Send`.  All our concrete middleware services *are* `Sync`
/// (they store shared state behind `Arc`), so wrapping in a `Mutex` is safe
/// and incurs negligible overhead — the lock is held only for the duration of
/// `Clone::clone` (a handful of `Arc` ref-count bumps).
struct SyncService {
    inner: Mutex<tower::util::BoxCloneService<LlmRequest, LlmResponse, LiterLlmError>>,
}

impl SyncService {
    /// Clone the inner service out of the mutex, returning an owned mutable
    /// service that can be `.call()`-ed.
    fn clone_service(&self) -> tower::util::BoxCloneService<LlmRequest, LlmResponse, LiterLlmError> {
        self.inner.lock().expect("ManagedClient service mutex poisoned").clone()
    }
}

// ---------------------------------------------------------------------------
// ManagedClient
// ---------------------------------------------------------------------------

/// A managed LLM client that wraps [`DefaultClient`] with optional Tower
/// middleware (cache, budget, hooks).
///
/// Construct via [`ManagedClient::new`].  If the provided [`ClientConfig`]
/// contains cache, budget, or hook configuration the corresponding Tower
/// layers are composed into a service stack.  Otherwise requests pass
/// straight through to the inner [`DefaultClient`].
///
/// `ManagedClient` implements [`LlmClient`] and can be used everywhere a
/// `DefaultClient` is expected.
pub struct ManagedClient {
    /// The raw client — used directly when no middleware is configured, and
    /// also wrapped by the Tower service when middleware *is* configured.
    inner: Arc<DefaultClient>,

    /// When `Some`, requests are routed through this Tower service stack
    /// instead of going directly to `inner`.
    service: Option<SyncService>,

    /// Budget state handle, exposed so callers can query accumulated spend.
    /// `None` when no budget middleware is configured.
    budget_state: Option<Arc<BudgetState>>,
}

// SAFETY: `SyncService` wraps a `Mutex<BoxCloneService>` which is `Send + Sync`.
// `Arc<DefaultClient>` and `Arc<BudgetState>` are both `Send + Sync`.
// The compiler can verify Send + Sync on `ManagedClient` automatically now
// that `SyncService` is `Send + Sync` (Mutex<T: Send> is Sync).

impl ManagedClient {
    /// Build a managed client.
    ///
    /// `model_hint` guides provider auto-detection — see
    /// [`DefaultClient::new`] for details.
    ///
    /// If the config contains cache, budget, or hook settings the
    /// corresponding Tower layers are composed into a service stack.
    /// Otherwise requests pass straight through to the inner client.
    ///
    /// # Errors
    ///
    /// Returns an error if the underlying [`DefaultClient`] cannot be
    /// constructed (e.g. invalid headers or HTTP client build failure).
    pub fn new(config: ClientConfig, model_hint: Option<&str>) -> Result<Self> {
        let client = DefaultClient::new(config.clone(), model_hint)?;
        let inner = Arc::new(client);

        let (service, budget_state) = build_service_stack(&config, Arc::clone(&inner));

        Ok(Self {
            inner,
            service,
            budget_state,
        })
    }

    /// Return a reference to the underlying [`DefaultClient`].
    #[must_use]
    pub fn inner(&self) -> &DefaultClient {
        &self.inner
    }

    /// Return the budget state handle, if budget middleware is configured.
    ///
    /// Use this to query accumulated spend at runtime.
    #[must_use]
    pub fn budget_state(&self) -> Option<&Arc<BudgetState>> {
        self.budget_state.as_ref()
    }

    /// Return `true` when middleware is active (requests go through the Tower
    /// service stack).
    #[must_use]
    pub fn has_middleware(&self) -> bool {
        self.service.is_some()
    }

    // -- helpers ----------------------------------------------------------

    /// Clone the Tower service and call it with `req`, returning the raw
    /// [`LlmResponse`].
    fn call_service(&self, req: LlmRequest) -> BoxFuture<'static, LlmResponse> {
        let mut svc = self
            .service
            .as_ref()
            .expect("call_service called without middleware")
            .clone_service();
        Box::pin(async move { svc.call(req).await })
    }
}

/// Inspect the config and, when at least one middleware option is set,
/// compose a Tower service stack wrapping the given client.
///
/// Returns `(Some(service), budget_state)` when middleware is configured,
/// or `(None, None)` when the config has no middleware.
fn build_service_stack(
    config: &ClientConfig,
    client: Arc<DefaultClient>,
) -> (Option<SyncService>, Option<Arc<BudgetState>>) {
    let has_cache = config.cache_config.is_some();
    let has_budget = config.budget_config.is_some();
    let has_hooks = !config.hooks.is_empty();

    if !has_cache && !has_budget && !has_hooks {
        return (None, None);
    }

    // Start with the base LlmService wrapping the DefaultClient.
    let base = LlmService::new_from_arc(client);

    // Layer application order: hooks (outermost) -> budget -> cache -> base
    // service (innermost).  This means:
    //   - Hooks see every request first (can reject / audit).
    //   - Budget checks happen next (can reject if over budget).
    //   - Cache is closest to the base (avoids budget charge for cache hits).

    let mut budget_state: Option<Arc<BudgetState>> = None;

    // We cannot use ServiceBuilder generics easily when layers are optional,
    // so we type-erase into BoxCloneService at each step.
    type Bcs = tower::util::BoxCloneService<LlmRequest, LlmResponse, LiterLlmError>;

    // Start by boxing the base service.
    let svc: Bcs = tower::util::BoxCloneService::new(base);

    // Apply cache layer.
    let svc = if let Some(ref cache_cfg) = config.cache_config {
        let layer = if let Some(ref store) = config.cache_store {
            CacheLayer::with_store(Arc::clone(store))
        } else {
            CacheLayer::new(cache_cfg.clone())
        };
        tower::util::BoxCloneService::new(layer.layer(svc))
    } else {
        svc
    };

    // Apply budget layer.
    let svc = if let Some(ref budget_cfg) = config.budget_config {
        let state = Arc::new(BudgetState::new());
        budget_state = Some(Arc::clone(&state));
        let layer = BudgetLayer::new(budget_cfg.clone(), state);
        tower::util::BoxCloneService::new(layer.layer(svc))
    } else {
        svc
    };

    // Apply hooks layer.
    let svc = if has_hooks {
        let layer = HooksLayer::new(config.hooks.clone());
        tower::util::BoxCloneService::new(layer.layer(svc))
    } else {
        svc
    };

    // Wrap in SyncService so ManagedClient is Sync.
    (Some(SyncService { inner: Mutex::new(svc) }), budget_state)
}

// ---------------------------------------------------------------------------
// LlmClient implementation
// ---------------------------------------------------------------------------

impl LlmClient for ManagedClient {
    fn chat(&self, req: ChatCompletionRequest) -> BoxFuture<'_, ChatCompletionResponse> {
        if self.service.is_none() {
            return self.inner.chat(req);
        }
        let fut = self.call_service(LlmRequest::Chat(req));
        Box::pin(async move {
            match fut.await? {
                LlmResponse::Chat(r) => Ok(r),
                other => Err(LiterLlmError::InternalError {
                    message: format!("expected Chat response, got {other:?}"),
                }),
            }
        })
    }

    fn chat_stream(&self, req: ChatCompletionRequest) -> BoxFuture<'_, BoxStream<'_, ChatCompletionChunk>> {
        if self.service.is_none() {
            return self.inner.chat_stream(req);
        }
        let fut = self.call_service(LlmRequest::ChatStream(req));
        Box::pin(async move {
            match fut.await? {
                LlmResponse::ChatStream(s) => Ok(s),
                other => Err(LiterLlmError::InternalError {
                    message: format!("expected ChatStream response, got {other:?}"),
                }),
            }
        })
    }

    fn embed(&self, req: EmbeddingRequest) -> BoxFuture<'_, EmbeddingResponse> {
        if self.service.is_none() {
            return self.inner.embed(req);
        }
        let fut = self.call_service(LlmRequest::Embed(req));
        Box::pin(async move {
            match fut.await? {
                LlmResponse::Embed(r) => Ok(r),
                other => Err(LiterLlmError::InternalError {
                    message: format!("expected Embed response, got {other:?}"),
                }),
            }
        })
    }

    fn list_models(&self) -> BoxFuture<'_, ModelsListResponse> {
        if self.service.is_none() {
            return self.inner.list_models();
        }
        let fut = self.call_service(LlmRequest::ListModels);
        Box::pin(async move {
            match fut.await? {
                LlmResponse::ListModels(r) => Ok(r),
                other => Err(LiterLlmError::InternalError {
                    message: format!("expected ListModels response, got {other:?}"),
                }),
            }
        })
    }

    fn image_generate(&self, req: CreateImageRequest) -> BoxFuture<'_, ImagesResponse> {
        if self.service.is_none() {
            return self.inner.image_generate(req);
        }
        let fut = self.call_service(LlmRequest::ImageGenerate(req));
        Box::pin(async move {
            match fut.await? {
                LlmResponse::ImageGenerate(r) => Ok(r),
                other => Err(LiterLlmError::InternalError {
                    message: format!("expected ImageGenerate response, got {other:?}"),
                }),
            }
        })
    }

    fn speech(&self, req: CreateSpeechRequest) -> BoxFuture<'_, bytes::Bytes> {
        if self.service.is_none() {
            return self.inner.speech(req);
        }
        let fut = self.call_service(LlmRequest::Speech(req));
        Box::pin(async move {
            match fut.await? {
                LlmResponse::Speech(r) => Ok(r),
                other => Err(LiterLlmError::InternalError {
                    message: format!("expected Speech response, got {other:?}"),
                }),
            }
        })
    }

    fn transcribe(&self, req: CreateTranscriptionRequest) -> BoxFuture<'_, TranscriptionResponse> {
        if self.service.is_none() {
            return self.inner.transcribe(req);
        }
        let fut = self.call_service(LlmRequest::Transcribe(req));
        Box::pin(async move {
            match fut.await? {
                LlmResponse::Transcribe(r) => Ok(r),
                other => Err(LiterLlmError::InternalError {
                    message: format!("expected Transcribe response, got {other:?}"),
                }),
            }
        })
    }

    fn moderate(&self, req: ModerationRequest) -> BoxFuture<'_, ModerationResponse> {
        if self.service.is_none() {
            return self.inner.moderate(req);
        }
        let fut = self.call_service(LlmRequest::Moderate(req));
        Box::pin(async move {
            match fut.await? {
                LlmResponse::Moderate(r) => Ok(r),
                other => Err(LiterLlmError::InternalError {
                    message: format!("expected Moderate response, got {other:?}"),
                }),
            }
        })
    }

    fn rerank(&self, req: RerankRequest) -> BoxFuture<'_, RerankResponse> {
        if self.service.is_none() {
            return self.inner.rerank(req);
        }
        let fut = self.call_service(LlmRequest::Rerank(req));
        Box::pin(async move {
            match fut.await? {
                LlmResponse::Rerank(r) => Ok(r),
                other => Err(LiterLlmError::InternalError {
                    message: format!("expected Rerank response, got {other:?}"),
                }),
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::client::ClientConfigBuilder;

    /// Verify that `ManagedClient` with no middleware config has no service
    /// stack and `has_middleware()` returns false.
    #[test]
    fn no_middleware_when_config_is_plain() {
        let config = ClientConfig::new("test-key");
        let client = ManagedClient::new(config, None).expect("should build");
        assert!(!client.has_middleware());
        assert!(client.budget_state().is_none());
    }

    /// Verify that adding a cache config activates middleware.
    #[test]
    fn middleware_active_with_cache_config() {
        use crate::tower::CacheConfig;
        let config = ClientConfigBuilder::new("test-key")
            .cache(CacheConfig::default())
            .build();
        let client = ManagedClient::new(config, None).expect("should build");
        assert!(client.has_middleware());
    }

    /// Verify that adding a budget config activates middleware and exposes
    /// budget state.
    #[test]
    fn middleware_active_with_budget_config() {
        use crate::tower::BudgetConfig;
        let config = ClientConfigBuilder::new("test-key")
            .budget(BudgetConfig::default())
            .build();
        let client = ManagedClient::new(config, None).expect("should build");
        assert!(client.has_middleware());
        assert!(client.budget_state().is_some());
    }
}
