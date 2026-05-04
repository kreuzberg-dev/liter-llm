use serde::{Deserialize, Serialize};

/// Request to classify content for policy violations.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ModerationRequest {
    pub input: ModerationInput,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
}

/// Input to the moderation endpoint — a single string or multiple strings.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ModerationInput {
    Single(String),
    Multiple(Vec<String>),
}

impl Default for ModerationInput {
    fn default() -> Self {
        Self::Single(String::new())
    }
}

/// Response from the moderation endpoint.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ModerationResponse {
    pub id: String,
    pub model: String,
    pub results: Vec<ModerationResult>,
}

/// A single moderation classification result.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ModerationResult {
    pub flagged: bool,
    pub categories: ModerationCategories,
    pub category_scores: ModerationCategoryScores,
}

/// Boolean flags for each moderation category.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ModerationCategories {
    #[serde(default)]
    pub sexual: bool,
    #[serde(default)]
    pub hate: bool,
    #[serde(default)]
    pub harassment: bool,
    #[serde(default, rename = "self-harm")]
    pub self_harm: bool,
    #[serde(default, rename = "sexual/minors")]
    pub sexual_minors: bool,
    #[serde(default, rename = "hate/threatening")]
    pub hate_threatening: bool,
    #[serde(default, rename = "violence/graphic")]
    pub violence_graphic: bool,
    #[serde(default, rename = "self-harm/intent")]
    pub self_harm_intent: bool,
    #[serde(default, rename = "self-harm/instructions")]
    pub self_harm_instructions: bool,
    #[serde(default, rename = "harassment/threatening")]
    pub harassment_threatening: bool,
    #[serde(default)]
    pub violence: bool,
}

/// Confidence scores for each moderation category.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ModerationCategoryScores {
    #[serde(default)]
    pub sexual: f64,
    #[serde(default)]
    pub hate: f64,
    #[serde(default)]
    pub harassment: f64,
    #[serde(default, rename = "self-harm")]
    pub self_harm: f64,
    #[serde(default, rename = "sexual/minors")]
    pub sexual_minors: f64,
    #[serde(default, rename = "hate/threatening")]
    pub hate_threatening: f64,
    #[serde(default, rename = "violence/graphic")]
    pub violence_graphic: f64,
    #[serde(default, rename = "self-harm/intent")]
    pub self_harm_intent: f64,
    #[serde(default, rename = "self-harm/instructions")]
    pub self_harm_instructions: f64,
    #[serde(default, rename = "harassment/threatening")]
    pub harassment_threatening: f64,
    #[serde(default)]
    pub violence: f64,
}
