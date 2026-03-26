//! Tower middleware integration for [`crate::client::LlmClient`].
//!
//! This module is only compiled when the `tower` feature is enabled.  It
//! provides:
//!
//! - [`types::LlmRequest`] / [`types::LlmResponse`] — the request/response
//!   enums that cross the tower `Service` boundary.
//! - [`service::LlmService`] — a thin `tower::Service` wrapper around any
//!   [`crate::client::LlmClient`].
//! - [`tracing::TracingLayer`] / [`tracing::TracingService`] — OTEL-compatible
//!   tracing middleware.
//! - [`fallback::FallbackLayer`] / [`fallback::FallbackService`] — route to a
//!   backup service on transient errors.
//! - [`cost::CostTrackingLayer`] / [`cost::CostTrackingService`] — emit
//!   `gen_ai.usage.cost` tracing span attribute from embedded pricing data.
//! - [`rate_limit::ModelRateLimitLayer`] / [`rate_limit::ModelRateLimitService`]
//!   — per-model RPM / TPM rate limiting.
//! - [`cache::CacheLayer`] / [`cache::CacheService`] — in-memory response
//!   caching for non-streaming requests.
//! - [`cooldown::CooldownLayer`] / [`cooldown::CooldownService`] — deployment
//!   cooldowns after transient errors.
//! - [`health::HealthCheckLayer`] / [`health::HealthCheckService`] — periodic
//!   health probes with automatic request rejection on failure.
//!
//! # Example
//!
//! ```rust,ignore
//! use liter_llm::tower::{CostTrackingLayer, LlmService, TracingLayer};
//! use tower::ServiceBuilder;
//!
//! let client = liter_llm::DefaultClient::new(config, None)?;
//! let service = ServiceBuilder::new()
//!     .layer(TracingLayer)
//!     .layer(CostTrackingLayer)
//!     .service(LlmService::new(client));
//! ```

pub mod cache;
pub mod cooldown;
pub mod cost;
pub mod fallback;
pub mod health;
pub mod rate_limit;
pub mod router;
pub mod service;
#[cfg(test)]
mod tests;
#[cfg(test)]
pub(crate) mod tests_common;
pub mod tracing;
pub mod types;

// Re-export tower core types for convenient access
pub use tower::ServiceExt;

pub use cache::{CacheConfig, CacheLayer, CacheService};
pub use cooldown::{CooldownLayer, CooldownService};
pub use cost::{CostTrackingLayer, CostTrackingService};
pub use fallback::{FallbackLayer, FallbackService};
pub use health::{HealthCheckLayer, HealthCheckService};
pub use rate_limit::{ModelRateLimitLayer, ModelRateLimitService, RateLimitConfig};
pub use router::{Router, RoutingStrategy};
pub use service::LlmService;
pub use tracing::{TracingLayer, TracingService};
pub use types::{LlmRequest, LlmResponse};
