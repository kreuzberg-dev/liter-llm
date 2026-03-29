use serde::Deserialize;

/// A named model entry with optional provider overrides and fallback chain.
#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ModelEntry {
    pub name: String,
    pub provider_model: String,
    pub api_key: Option<String>,
    pub base_url: Option<String>,
    pub timeout_secs: Option<u64>,
    #[serde(default)]
    pub fallbacks: Vec<String>,
}

/// A pattern-based alias that routes model names matching `pattern` with
/// optional credential overrides.
#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AliasEntry {
    pub pattern: String,
    pub api_key: Option<String>,
    pub base_url: Option<String>,
}
