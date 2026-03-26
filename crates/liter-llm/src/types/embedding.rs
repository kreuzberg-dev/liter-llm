use serde::{Deserialize, Serialize};

use super::common::Usage;
use crate::cost;

// ─── Encoding format ──────────────────────────────────────────────────────────

/// The format in which the embedding vectors are returned.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum EmbeddingFormat {
    /// 32-bit floating-point numbers (default).
    Float,
    /// Base64-encoded string representation of the floats.
    Base64,
}

// ─── Request ──────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EmbeddingRequest {
    pub model: String,
    pub input: EmbeddingInput,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub encoding_format: Option<EmbeddingFormat>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dimensions: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum EmbeddingInput {
    Single(String),
    Multiple(Vec<String>),
}

// ─── Response ─────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EmbeddingResponse {
    /// Always `"list"` from OpenAI-compatible APIs.  Stored as a plain
    /// `String` so non-standard provider values do not break deserialization.
    pub object: String,
    pub data: Vec<EmbeddingObject>,
    pub model: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub usage: Option<Usage>,
}

impl EmbeddingResponse {
    /// Estimate the cost of this embedding request based on embedded pricing data.
    ///
    /// Returns `None` if:
    /// - the `model` field is not present in the embedded pricing registry, or
    /// - the `usage` field is absent from the response.
    ///
    /// Embedding models only charge for input tokens; output cost is zero.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// let cost = response.estimated_cost();
    /// if let Some(usd) = cost {
    ///     println!("Embedding cost: ${usd:.8}");
    /// }
    /// ```
    #[must_use]
    pub fn estimated_cost(&self) -> Option<f64> {
        let usage = self.usage.as_ref()?;
        cost::completion_cost(&self.model, usage.prompt_tokens, usage.completion_tokens)
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EmbeddingObject {
    /// Always `"embedding"` from OpenAI-compatible APIs.  Stored as a plain
    /// `String` so non-standard provider values do not break deserialization.
    pub object: String,
    pub embedding: Vec<f64>,
    pub index: u32,
}
