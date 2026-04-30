// Provider, HTTP, and retry infrastructure are only active with native-http.
// Suppress dead_code lints on the wasm / no-native-http target so that the
// type-only surface compiles cleanly.
#![cfg_attr(
    not(any(feature = "native-http", feature = "wasm-http")),
    allow(dead_code, unused_imports)
)]

pub mod auth;
#[cfg(any(feature = "native-http", feature = "wasm-http"))]
pub mod bindings;
pub mod client;
pub mod cost;
pub mod error;
pub(crate) mod http;
pub mod provider;
#[cfg(test)]
mod tests;
#[cfg(feature = "tokenizer")]
pub mod tokenizer;
#[cfg(feature = "tower")]
pub mod tower;
pub mod types;

// Re-export key types at crate root.
pub use client::{
    BatchClient, BoxFuture, BoxStream, ClientConfig, ClientConfigBuilder, FileClient, FileConfig, LlmClient,
    LlmClientRaw, ResponseClient,
};
// DefaultClient requires the native HTTP stack (reqwest on native or WASM fetch API).
#[cfg(any(feature = "native-http", feature = "wasm-http"))]
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

/// Install the `ring` crypto provider as the rustls process default, idempotently.
///
/// rustls 0.23+ removed the implicit default provider. This function installs
/// `ring` once per process. Subsequent calls are no-ops. Calling it from a
/// downstream Rust app that has already installed `aws-lc-rs` is safe — the
/// `Err` from `install_default()` is silently ignored.
///
/// Called automatically by every internal `reqwest::Client` constructor
/// (auth providers, default HTTP client). Bindings and downstream consumers
/// reach those constructors transitively, so no manual init is required.
///
/// WASM builds are exempt — the WASM target uses the browser/Node.js fetch
/// API instead of rustls, so no crypto provider is needed.
#[cfg(feature = "native-http")]
pub fn ensure_crypto_provider() {
    use std::sync::Once;
    static INIT: Once = Once::new();
    INIT.call_once(|| {
        // `install_default` returns Err if another provider is already installed.
        // That is fine — the caller may have installed `aws-lc-rs` deliberately;
        // we do not want to override their choice.
        let _ = rustls::crypto::ring::default_provider().install_default();
    });
}
