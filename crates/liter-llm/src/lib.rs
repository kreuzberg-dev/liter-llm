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
pub use error::{LiterLlmError, Result};
// Re-export the public provider helper functions that are part of the crate's
// public API even though the `provider` module itself is pub(crate).
pub use provider::{ProviderConfig, all_providers, complex_provider_names};
pub use types::*;
