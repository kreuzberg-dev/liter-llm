use serde::Deserialize;

/// A virtual API key with optional model restrictions and rate/budget limits.
#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct VirtualKeyConfig {
    pub key: String,
    pub description: Option<String>,
    /// Models this key is allowed to access. Empty means all models.
    #[serde(default)]
    pub models: Vec<String>,
    pub rpm: Option<u32>,
    pub tpm: Option<u64>,
    pub budget_limit: Option<f64>,
}
