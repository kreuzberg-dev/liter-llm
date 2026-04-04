//! Per-model rate limiting middleware.
//!
//! [`ModelRateLimitLayer`] wraps any [`Service<LlmRequest>`] and enforces
//! per-model request-per-minute (RPM) and token-per-minute (TPM) limits using
//! a fixed window.  When a model exceeds its configured limit the middleware
//! returns [`LiterLlmError::RateLimited`] without forwarding the request to the
//! inner service.  After a successful response, token usage is extracted and
//! added to the running count.
//!
//! Rate state is tracked per model name in a [`DashMap`] so that independent
//! models do not interfere with each other.

use std::sync::Arc;
use std::task::{Context, Poll};
use std::time::{Duration, Instant};

use dashmap::DashMap;
use tower::{Layer, Service};

use super::types::{LlmRequest, LlmResponse};
use crate::client::BoxFuture;
use crate::error::{LiterLlmError, Result};

// ---- Config ----------------------------------------------------------------

/// Configuration for per-model rate limits.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RateLimitConfig {
    /// Maximum requests per window.  `None` means unlimited.
    pub rpm: Option<u32>,
    /// Maximum tokens per window.  `None` means unlimited.
    pub tpm: Option<u64>,
    /// Fixed window duration (defaults to 60 s).
    pub window: Duration,
}

impl Default for RateLimitConfig {
    fn default() -> Self {
        Self {
            rpm: None,
            tpm: None,
            window: Duration::from_secs(60),
        }
    }
}

// ---- State -----------------------------------------------------------------

/// Per-model counters for the current window.
struct ModelRateState {
    request_count: u64,
    token_count: u64,
    window_start: Instant,
}

impl ModelRateState {
    fn new() -> Self {
        Self {
            request_count: 0,
            token_count: 0,
            window_start: Instant::now(),
        }
    }

    /// Reset counters if the current window has elapsed.
    fn maybe_reset(&mut self, window: Duration) {
        if self.window_start.elapsed() >= window {
            self.request_count = 0;
            self.token_count = 0;
            self.window_start = Instant::now();
        }
    }
}

// ---- Layer -----------------------------------------------------------------

/// Tower [`Layer`] that enforces per-model rate limits.
pub struct ModelRateLimitLayer {
    config: RateLimitConfig,
    state: Arc<DashMap<String, ModelRateState>>,
}

impl ModelRateLimitLayer {
    /// Create a new rate-limit layer with the given configuration.
    #[must_use]
    pub fn new(config: RateLimitConfig) -> Self {
        Self {
            config,
            state: Arc::new(DashMap::new()),
        }
    }
}

impl<S> Layer<S> for ModelRateLimitLayer {
    type Service = ModelRateLimitService<S>;

    fn layer(&self, inner: S) -> Self::Service {
        ModelRateLimitService {
            inner,
            config: self.config.clone(),
            state: Arc::clone(&self.state),
        }
    }
}

// ---- Service ---------------------------------------------------------------

/// Tower service produced by [`ModelRateLimitLayer`].
pub struct ModelRateLimitService<S> {
    inner: S,
    config: RateLimitConfig,
    state: Arc<DashMap<String, ModelRateState>>,
}

impl<S: Clone> Clone for ModelRateLimitService<S> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
            config: self.config.clone(),
            state: Arc::clone(&self.state),
        }
    }
}

impl<S> Service<LlmRequest> for ModelRateLimitService<S>
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
        let model = req.model().unwrap_or("unknown").to_owned();
        let config = self.config.clone();
        let state = Arc::clone(&self.state);

        // --- Pre-flight: check RPM limit ---
        {
            let mut entry = state.entry(model.clone()).or_insert_with(ModelRateState::new);
            entry.maybe_reset(config.window);

            if let Some(rpm) = config.rpm
                && entry.request_count >= u64::from(rpm)
            {
                return Box::pin(async move {
                    Err(LiterLlmError::RateLimited {
                        message: format!(
                            "model {model} exceeded {rpm} requests per {:.0}s window",
                            config.window.as_secs_f64()
                        ),
                        retry_after: Some(config.window),
                    })
                });
            }

            if let Some(tpm) = config.tpm
                && entry.token_count >= tpm
            {
                return Box::pin(async move {
                    Err(LiterLlmError::RateLimited {
                        message: format!(
                            "model {model} exceeded {tpm} tokens per {:.0}s window",
                            config.window.as_secs_f64()
                        ),
                        retry_after: Some(config.window),
                    })
                });
            }

            // Increment request count optimistically.
            entry.request_count += 1;
        }

        let fut = self.inner.call(req);

        Box::pin(async move {
            let resp = fut.await?;

            // --- Post-flight: update token count ---
            if let Some(usage) = resp.usage() {
                let total_tokens = usage.prompt_tokens + usage.completion_tokens;
                if let Some(mut entry) = state.get_mut(&model) {
                    entry.maybe_reset(config.window);
                    entry.token_count += total_tokens;
                }
            }

            Ok(resp)
        })
    }
}

// ---- Tests -----------------------------------------------------------------

#[cfg(test)]
mod tests {
    use tower::{Layer as _, Service as _};

    use super::*;
    use crate::tower::tests_common::{MockClient, chat_req};

    use crate::tower::service::LlmService;
    use crate::tower::types::LlmRequest;

    #[tokio::test]
    async fn allows_requests_under_rpm_limit() {
        let config = RateLimitConfig {
            rpm: Some(5),
            tpm: None,
            window: Duration::from_secs(60),
        };
        let layer = ModelRateLimitLayer::new(config);
        let inner = LlmService::new(MockClient::ok());
        let mut svc = layer.layer(inner);

        for _ in 0..5 {
            let resp = svc.call(LlmRequest::Chat(chat_req("gpt-4"))).await;
            assert!(resp.is_ok(), "requests under limit should succeed");
        }
    }

    #[tokio::test]
    async fn rejects_requests_over_rpm_limit() {
        let config = RateLimitConfig {
            rpm: Some(2),
            tpm: None,
            window: Duration::from_secs(60),
        };
        let layer = ModelRateLimitLayer::new(config);
        let inner = LlmService::new(MockClient::ok());
        let mut svc = layer.layer(inner);

        // First two succeed.
        svc.call(LlmRequest::Chat(chat_req("gpt-4"))).await.unwrap();
        svc.call(LlmRequest::Chat(chat_req("gpt-4"))).await.unwrap();

        // Third should be rate limited.
        let err = svc
            .call(LlmRequest::Chat(chat_req("gpt-4")))
            .await
            .expect_err("should be rate limited");
        assert!(matches!(err, LiterLlmError::RateLimited { .. }));
    }

    #[tokio::test]
    async fn independent_models_have_separate_limits() {
        let config = RateLimitConfig {
            rpm: Some(1),
            tpm: None,
            window: Duration::from_secs(60),
        };
        let layer = ModelRateLimitLayer::new(config);
        let inner = LlmService::new(MockClient::ok());
        let mut svc = layer.layer(inner);

        svc.call(LlmRequest::Chat(chat_req("gpt-4"))).await.unwrap();
        // Different model should still work.
        svc.call(LlmRequest::Chat(chat_req("gpt-3.5-turbo"))).await.unwrap();
    }

    #[tokio::test]
    async fn tpm_limit_rejects_after_threshold() {
        let config = RateLimitConfig {
            rpm: None,
            tpm: Some(10), // Very low threshold — the mock returns 15 total tokens.
            window: Duration::from_secs(60),
        };
        let layer = ModelRateLimitLayer::new(config);
        let inner = LlmService::new(MockClient::ok());
        let mut svc = layer.layer(inner);

        // First call succeeds and records 15 tokens (over the 10 limit).
        svc.call(LlmRequest::Chat(chat_req("gpt-4"))).await.unwrap();

        // Second call should be rejected because token count >= tpm.
        let err = svc
            .call(LlmRequest::Chat(chat_req("gpt-4")))
            .await
            .expect_err("should be rate limited by TPM");
        assert!(matches!(err, LiterLlmError::RateLimited { .. }));
    }

    #[tokio::test]
    async fn unlimited_config_allows_all_requests() {
        let config = RateLimitConfig::default();
        let layer = ModelRateLimitLayer::new(config);
        let inner = LlmService::new(MockClient::ok());
        let mut svc = layer.layer(inner);

        for _ in 0..100 {
            assert!(svc.call(LlmRequest::Chat(chat_req("gpt-4"))).await.is_ok());
        }
    }
}
