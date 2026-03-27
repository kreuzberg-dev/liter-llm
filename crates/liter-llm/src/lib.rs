// Provider, HTTP, and retry infrastructure are only active with native-http.
// Suppress dead_code lints on the wasm / no-native-http target so that the
// type-only surface compiles cleanly.
#![cfg_attr(not(feature = "native-http"), allow(dead_code, unused_imports))]

pub mod auth;
pub mod client;
pub mod cost;
pub mod error;
pub(crate) mod http;
pub(crate) mod provider;
#[cfg(test)]
mod tests;
#[cfg(feature = "tokenizer")]
pub mod tokenizer;
#[cfg(feature = "tower")]
pub mod tower;
pub mod types;

// Re-export key types at crate root.
pub use client::{
    BatchClient, BoxFuture, BoxStream, ClientConfig, ClientConfigBuilder, FileClient, LlmClient, ResponseClient,
};
// DefaultClient requires the native HTTP stack (reqwest + tokio).
#[cfg(feature = "native-http")]
pub use client::DefaultClient;
// ManagedClient requires both the native HTTP stack and Tower middleware.
#[cfg(all(feature = "native-http", feature = "tower"))]
pub use client::managed::ManagedClient;
pub use error::{LiterLlmError, Result};
// Re-export the public provider helper functions that are part of the crate's
// public API even though the `provider` module itself is pub(crate).
pub use provider::custom::{
    AuthHeaderFormat, CustomProviderConfig, register_custom_provider, unregister_custom_provider,
};
pub use provider::{ProviderConfig, all_providers, complex_provider_names};
pub use types::*;
