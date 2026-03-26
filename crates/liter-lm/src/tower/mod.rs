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
//!
//! # Example
//!
//! ```rust,ignore
//! use liter_lm::tower::{CostTrackingLayer, LlmService, TracingLayer};
//! use tower::ServiceBuilder;
//!
//! let client = liter_lm::DefaultClient::new(config, None)?;
//! let service = ServiceBuilder::new()
//!     .layer(TracingLayer)
//!     .layer(CostTrackingLayer)
//!     .service(LlmService::new(client));
//! ```

pub mod cost;
pub mod fallback;
pub mod router;
pub mod service;
#[cfg(test)]
mod tests;
pub mod tracing;
pub mod types;

pub use cost::{CostTrackingLayer, CostTrackingService};
pub use fallback::{FallbackLayer, FallbackService};
pub use router::{Router, RoutingStrategy};
pub use service::LlmService;
pub use tracing::{TracingLayer, TracingService};
pub use types::{LlmRequest, LlmResponse};
