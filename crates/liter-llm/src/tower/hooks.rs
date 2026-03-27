//! Tower middleware that invokes user-defined hooks before and after requests.
//!
//! [`HooksLayer`] wraps any [`Service<LlmRequest>`] and calls registered
//! [`LlmHook`] implementations at three lifecycle points:
//!
//! - **`on_request`** — before the request is forwarded to the inner service.
//!   Returning `Err` from any hook short-circuits the chain (guardrail
//!   rejection).
//! - **`on_response`** — after a successful response from the inner service.
//! - **`on_error`** — when the inner service returns an error.
//!
//! Hooks are invoked sequentially in registration order.
//!
//! # Example
//!
//! ```rust,ignore
//! use std::sync::Arc;
//! use liter_llm::tower::{HooksLayer, LlmHook, LlmService, TracingLayer};
//! use tower::ServiceBuilder;
//!
//! let hook: Arc<dyn LlmHook> = Arc::new(MyAuditHook);
//! let service = ServiceBuilder::new()
//!     .layer(HooksLayer::single(hook))
//!     .service(LlmService::new(client));
//! ```

use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll};

use tower::Layer;
use tower::Service;

use super::types::{LlmRequest, LlmResponse};
use crate::client::BoxFuture;
use crate::error::{LiterLlmError, Result};

// ─── Hook Trait ──────────────────────────────────────────────────────────────

/// Callback trait for observing and guarding LLM requests.
///
/// All methods have default no-op implementations, so consumers only need to
/// override the lifecycle points they care about.
pub trait LlmHook: Send + Sync + 'static {
    /// Called before the request is sent to the inner service.
    ///
    /// Return `Err` to short-circuit the entire service chain — this enables
    /// guardrail patterns such as content filtering or budget enforcement.
    fn on_request(&self, _req: &LlmRequest) -> Pin<Box<dyn Future<Output = Result<()>> + Send + '_>> {
        Box::pin(async { Ok(()) })
    }

    /// Called after the inner service returns a successful response.
    fn on_response(&self, _req: &LlmRequest, _resp: &LlmResponse) -> Pin<Box<dyn Future<Output = ()> + Send + '_>> {
        Box::pin(async {})
    }

    /// Called when the inner service returns an error.
    fn on_error(&self, _req: &LlmRequest, _err: &LiterLlmError) -> Pin<Box<dyn Future<Output = ()> + Send + '_>> {
        Box::pin(async {})
    }
}

// ─── Layer ───────────────────────────────────────────────────────────────────

/// Tower [`Layer`] that attaches [`LlmHook`] callbacks to a service.
///
/// Hooks are stored behind `Arc` so that the layer and all services it
/// produces share the same hook instances without cloning them.
#[derive(Clone)]
pub struct HooksLayer {
    hooks: Arc<Vec<Arc<dyn LlmHook>>>,
}

impl HooksLayer {
    /// Create a new layer with the given list of hooks.
    ///
    /// Hooks are invoked sequentially in the order they appear in the vector.
    #[must_use]
    pub fn new(hooks: Vec<Arc<dyn LlmHook>>) -> Self {
        Self { hooks: Arc::new(hooks) }
    }

    /// Convenience constructor for a single hook.
    #[must_use]
    pub fn single(hook: Arc<dyn LlmHook>) -> Self {
        Self::new(vec![hook])
    }
}

impl<S> Layer<S> for HooksLayer {
    type Service = HooksService<S>;

    fn layer(&self, inner: S) -> Self::Service {
        HooksService {
            inner,
            hooks: Arc::clone(&self.hooks),
        }
    }
}

// ─── Service ─────────────────────────────────────────────────────────────────

/// Tower service produced by [`HooksLayer`].
pub struct HooksService<S> {
    inner: S,
    hooks: Arc<Vec<Arc<dyn LlmHook>>>,
}

impl<S: Clone> Clone for HooksService<S> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
            hooks: Arc::clone(&self.hooks),
        }
    }
}

impl<S> Service<LlmRequest> for HooksService<S>
where
    S: Service<LlmRequest, Response = LlmResponse, Error = LiterLlmError> + Send + 'static,
    S::Future: Send + 'static,
{
    type Response = LlmResponse;
    type Error = LiterLlmError;
    type Future = BoxFuture<'static, LlmResponse>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<()>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, req: LlmRequest) -> Self::Future {
        let hooks = Arc::clone(&self.hooks);
        // Clone the request so we can pass it to post-hooks after the inner
        // service consumes the original.
        let req_clone = req.clone();
        let fut = self.inner.call(req);

        Box::pin(async move {
            // Pre-hooks: run sequentially; short-circuit on first Err.
            for hook in hooks.iter() {
                hook.on_request(&req_clone).await?;
            }

            match fut.await {
                Ok(resp) => {
                    // Post-hooks (success path).
                    for hook in hooks.iter() {
                        hook.on_response(&req_clone, &resp).await;
                    }
                    Ok(resp)
                }
                Err(err) => {
                    // Post-hooks (error path).
                    for hook in hooks.iter() {
                        hook.on_error(&req_clone, &err).await;
                    }
                    Err(err)
                }
            }
        })
    }
}

// ─── Tests ───────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use std::sync::atomic::{AtomicUsize, Ordering};

    use tower::Layer as _;
    use tower::Service as _;

    use super::*;
    use crate::tower::service::LlmService;
    use crate::tower::tests_common::{MockClient, chat_req};
    use crate::tower::types::{LlmRequest, LlmResponse};

    // ── Test hook implementations ────────────────────────────────────────────

    /// A hook that records how many times each callback was invoked.
    struct CountingHook {
        on_request_count: AtomicUsize,
        on_response_count: AtomicUsize,
        on_error_count: AtomicUsize,
    }

    impl CountingHook {
        fn new() -> Self {
            Self {
                on_request_count: AtomicUsize::new(0),
                on_response_count: AtomicUsize::new(0),
                on_error_count: AtomicUsize::new(0),
            }
        }
    }

    impl LlmHook for CountingHook {
        fn on_request(&self, _req: &LlmRequest) -> Pin<Box<dyn Future<Output = Result<()>> + Send + '_>> {
            self.on_request_count.fetch_add(1, Ordering::SeqCst);
            Box::pin(async { Ok(()) })
        }

        fn on_response(&self, _req: &LlmRequest, _resp: &LlmResponse) -> Pin<Box<dyn Future<Output = ()> + Send + '_>> {
            self.on_response_count.fetch_add(1, Ordering::SeqCst);
            Box::pin(async {})
        }

        fn on_error(&self, _req: &LlmRequest, _err: &LiterLlmError) -> Pin<Box<dyn Future<Output = ()> + Send + '_>> {
            self.on_error_count.fetch_add(1, Ordering::SeqCst);
            Box::pin(async {})
        }
    }

    /// A hook that rejects all requests (guardrail).
    struct RejectAllHook;

    impl LlmHook for RejectAllHook {
        fn on_request(&self, _req: &LlmRequest) -> Pin<Box<dyn Future<Output = Result<()>> + Send + '_>> {
            Box::pin(async {
                Err(LiterLlmError::ContentPolicy {
                    message: "rejected by guardrail".into(),
                })
            })
        }
    }

    /// A hook that records its invocation order into a shared vector.
    struct OrderTrackingHook {
        id: usize,
        order: Arc<std::sync::Mutex<Vec<usize>>>,
    }

    impl LlmHook for OrderTrackingHook {
        fn on_request(&self, _req: &LlmRequest) -> Pin<Box<dyn Future<Output = Result<()>> + Send + '_>> {
            self.order.lock().expect("lock poisoned").push(self.id);
            Box::pin(async { Ok(()) })
        }

        fn on_response(&self, _req: &LlmRequest, _resp: &LlmResponse) -> Pin<Box<dyn Future<Output = ()> + Send + '_>> {
            self.order.lock().expect("lock poisoned").push(self.id + 100);
            Box::pin(async {})
        }
    }

    // ── Tests ────────────────────────────────────────────────────────────────

    #[tokio::test]
    async fn on_request_hook_is_called() {
        let hook = Arc::new(CountingHook::new());
        let inner = LlmService::new(MockClient::ok());
        let mut svc = HooksLayer::single(Arc::clone(&hook) as Arc<dyn LlmHook>).layer(inner);

        let _resp = svc
            .call(LlmRequest::Chat(chat_req("gpt-4")))
            .await
            .expect("should succeed");

        assert_eq!(hook.on_request_count.load(Ordering::SeqCst), 1);
    }

    #[tokio::test]
    async fn on_response_hook_is_called_on_success() {
        let hook = Arc::new(CountingHook::new());
        let inner = LlmService::new(MockClient::ok());
        let mut svc = HooksLayer::single(Arc::clone(&hook) as Arc<dyn LlmHook>).layer(inner);

        let _resp = svc
            .call(LlmRequest::Chat(chat_req("gpt-4")))
            .await
            .expect("should succeed");

        assert_eq!(hook.on_response_count.load(Ordering::SeqCst), 1);
        assert_eq!(hook.on_error_count.load(Ordering::SeqCst), 0);
    }

    #[tokio::test]
    async fn on_error_hook_is_called_on_failure() {
        let hook = Arc::new(CountingHook::new());
        let inner = LlmService::new(MockClient::failing_timeout());
        let mut svc = HooksLayer::single(Arc::clone(&hook) as Arc<dyn LlmHook>).layer(inner);

        let err = svc
            .call(LlmRequest::Chat(chat_req("gpt-4")))
            .await
            .expect_err("should fail");

        assert!(matches!(err, LiterLlmError::Timeout));
        assert_eq!(hook.on_error_count.load(Ordering::SeqCst), 1);
        assert_eq!(hook.on_response_count.load(Ordering::SeqCst), 0);
    }

    #[tokio::test]
    async fn guardrail_rejection_short_circuits_inner_service() {
        let mock = MockClient::ok();
        let call_count = Arc::clone(&mock.call_count);
        let inner = LlmService::new(mock);
        let mut svc = HooksLayer::single(Arc::new(RejectAllHook) as Arc<dyn LlmHook>).layer(inner);

        let err = svc
            .call(LlmRequest::Chat(chat_req("gpt-4")))
            .await
            .expect_err("should be rejected by guardrail");

        assert!(matches!(err, LiterLlmError::ContentPolicy { .. }));
        // The inner service must NOT have been called.
        assert_eq!(call_count.load(Ordering::SeqCst), 0);
    }

    #[tokio::test]
    async fn multiple_hooks_called_in_registration_order() {
        let order = Arc::new(std::sync::Mutex::new(Vec::new()));

        let hooks: Vec<Arc<dyn LlmHook>> = vec![
            Arc::new(OrderTrackingHook {
                id: 1,
                order: Arc::clone(&order),
            }),
            Arc::new(OrderTrackingHook {
                id: 2,
                order: Arc::clone(&order),
            }),
            Arc::new(OrderTrackingHook {
                id: 3,
                order: Arc::clone(&order),
            }),
        ];

        let inner = LlmService::new(MockClient::ok());
        let mut svc = HooksLayer::new(hooks).layer(inner);

        let _resp = svc
            .call(LlmRequest::Chat(chat_req("gpt-4")))
            .await
            .expect("should succeed");

        let recorded = order.lock().expect("lock poisoned").clone();
        // Pre-hooks: 1, 2, 3 then post-hooks: 101, 102, 103
        assert_eq!(recorded, vec![1, 2, 3, 101, 102, 103]);
    }
}
