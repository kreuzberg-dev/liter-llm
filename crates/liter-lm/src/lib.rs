pub mod client;
pub mod error;
pub(crate) mod http;
pub(crate) mod provider;
#[cfg(test)]
mod tests;
pub mod types;

// Re-export key types at crate root.
pub use client::{BoxFuture, BoxStream, ClientConfig, ClientConfigBuilder, LlmClient};
// DefaultClient requires the native HTTP stack (reqwest + tokio).
#[cfg(feature = "native-http")]
pub use client::DefaultClient;
pub use error::{LiterLmError, Result};
// Re-export the public provider helper functions that are part of the crate's
// public API even though the `provider` module itself is pub(crate).
pub use provider::{ProviderConfig, all_providers, complex_provider_names};
pub use types::*;
