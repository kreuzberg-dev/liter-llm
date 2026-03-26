use serde::{Deserialize, Serialize};

// ─── Responses ────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ModelsListResponse {
    /// Always `"list"` from OpenAI-compatible APIs.  Stored as a plain
    /// `String` so non-standard provider values do not break deserialization.
    pub object: String,
    pub data: Vec<ModelObject>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ModelObject {
    pub id: String,
    /// Always `"model"` from OpenAI-compatible APIs.  Stored as a plain
    /// `String` so non-standard provider values do not break deserialization.
    pub object: String,
    pub created: u64,
    pub owned_by: String,
}
