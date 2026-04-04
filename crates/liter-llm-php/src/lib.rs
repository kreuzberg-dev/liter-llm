#![allow(unused_imports)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::missing_errors_doc)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(clippy::should_implement_trait)]

use ext_php_rs::prelude::*;
use serde_json;
use std::collections::HashMap;

#[derive(Clone, serde::Deserialize, serde::Serialize)]
#[php_class]
pub struct ModelPricing {
    pub input_cost_per_token: f64,
    pub output_cost_per_token: f64,
}

#[php_impl]
impl ModelPricing {
    pub fn __construct(input_cost_per_token: f64, output_cost_per_token: f64) -> Self {
        Self {
            input_cost_per_token,
            output_cost_per_token,
        }
    }
}

#[derive(Clone, serde::Deserialize, serde::Serialize)]
#[php_class]
pub struct CreateSpeechRequest {
    pub model: String,
    pub input: String,
    pub voice: String,
    pub response_format: Option<String>,
    pub speed: Option<f64>,
}

#[php_impl]
impl CreateSpeechRequest {
    pub fn __construct(
        model: String,
        input: String,
        voice: String,
        response_format: Option<String>,
        speed: Option<f64>,
    ) -> Self {
        Self {
            model,
            input,
            voice,
            response_format,
            speed,
        }
    }
}

#[derive(Clone, serde::Deserialize, serde::Serialize)]
#[php_class]
pub struct TranscriptionResponse {
    pub text: String,
    pub language: Option<String>,
    pub duration: Option<f64>,
    pub segments: Option<Vec<TranscriptionSegment>>,
}

#[php_impl]
impl TranscriptionResponse {
    pub fn from_json(json: String) -> PhpResult<Self> {
        serde_json::from_str(&json).map_err(|e| PhpException::default(e.to_string()).into())
    }
}

#[derive(Clone, serde::Deserialize, serde::Serialize)]
#[php_class]
pub struct TranscriptionSegment {
    pub id: u32,
    pub start: f64,
    pub end: f64,
    pub text: String,
}

#[php_impl]
impl TranscriptionSegment {
    pub fn __construct(id: u32, start: f64, end: f64, text: String) -> Self {
        Self { id, start, end, text }
    }
}

#[derive(Clone, serde::Deserialize, serde::Serialize)]
#[php_class]
pub struct ChatCompletionRequest {
    pub model: String,
    pub messages: Vec<String>,
    pub temperature: Option<f64>,
    pub top_p: Option<f64>,
    pub n: Option<u32>,
    pub stop: Option<String>,
    pub max_tokens: Option<i64>,
    pub presence_penalty: Option<f64>,
    pub frequency_penalty: Option<f64>,
    pub logit_bias: Option<HashMap<String, f64>>,
    pub user: Option<String>,
    pub tools: Option<Vec<ChatCompletionTool>>,
    pub tool_choice: Option<String>,
    pub parallel_tool_calls: Option<bool>,
    pub response_format: Option<String>,
    pub stream_options: Option<String>,
    pub seed: Option<i64>,
    pub reasoning_effort: Option<String>,
    pub extra_body: Option<String>,
}

#[php_impl]
impl ChatCompletionRequest {
    pub fn from_json(json: String) -> PhpResult<Self> {
        serde_json::from_str(&json).map_err(|e| PhpException::default(e.to_string()).into())
    }
}

#[derive(Clone, serde::Deserialize, serde::Serialize)]
#[php_class]
pub struct ChatCompletionResponse {
    pub id: String,
    pub object: String,
    pub created: i64,
    pub model: String,
    pub choices: Vec<Choice>,
    pub usage: Option<Usage>,
    pub system_fingerprint: Option<String>,
    pub service_tier: Option<String>,
}

#[php_impl]
impl ChatCompletionResponse {
    pub fn from_json(json: String) -> PhpResult<Self> {
        serde_json::from_str(&json).map_err(|e| PhpException::default(e.to_string()).into())
    }

    pub fn estimated_cost(&self) -> Option<f64> {
        None
    }
}

#[derive(Clone, serde::Deserialize, serde::Serialize)]
#[php_class]
pub struct Choice {
    pub index: u32,
    pub message: AssistantMessage,
    pub finish_reason: Option<String>,
}

#[php_impl]
impl Choice {
    pub fn from_json(json: String) -> PhpResult<Self> {
        serde_json::from_str(&json).map_err(|e| PhpException::default(e.to_string()).into())
    }
}

#[derive(Clone, serde::Deserialize, serde::Serialize)]
#[php_class]
pub struct ChatCompletionChunk {
    pub id: String,
    pub object: String,
    pub created: i64,
    pub model: String,
    pub choices: Vec<StreamChoice>,
    pub usage: Option<Usage>,
    pub system_fingerprint: Option<String>,
    pub service_tier: Option<String>,
}

#[php_impl]
impl ChatCompletionChunk {
    pub fn from_json(json: String) -> PhpResult<Self> {
        serde_json::from_str(&json).map_err(|e| PhpException::default(e.to_string()).into())
    }
}

#[derive(Clone, serde::Deserialize, serde::Serialize)]
#[php_class]
pub struct StreamChoice {
    pub index: u32,
    pub delta: StreamDelta,
    pub finish_reason: Option<String>,
}

#[php_impl]
impl StreamChoice {
    pub fn from_json(json: String) -> PhpResult<Self> {
        serde_json::from_str(&json).map_err(|e| PhpException::default(e.to_string()).into())
    }
}

#[derive(Clone, serde::Deserialize, serde::Serialize)]
#[php_class]
pub struct StreamDelta {
    pub role: Option<String>,
    pub content: Option<String>,
    pub tool_calls: Option<Vec<StreamToolCall>>,
    pub function_call: Option<StreamFunctionCall>,
    pub refusal: Option<String>,
}

#[php_impl]
impl StreamDelta {
    pub fn from_json(json: String) -> PhpResult<Self> {
        serde_json::from_str(&json).map_err(|e| PhpException::default(e.to_string()).into())
    }
}

#[derive(Clone, serde::Deserialize, serde::Serialize)]
#[php_class]
pub struct StreamToolCall {
    pub index: u32,
    pub id: Option<String>,
    pub call_type: Option<String>,
    pub function: Option<StreamFunctionCall>,
}

#[php_impl]
impl StreamToolCall {
    pub fn from_json(json: String) -> PhpResult<Self> {
        serde_json::from_str(&json).map_err(|e| PhpException::default(e.to_string()).into())
    }
}

#[derive(Clone, serde::Deserialize, serde::Serialize)]
#[php_class]
pub struct StreamFunctionCall {
    pub name: Option<String>,
    pub arguments: Option<String>,
}

#[php_impl]
impl StreamFunctionCall {
    pub fn __construct(name: Option<String>, arguments: Option<String>) -> Self {
        Self { name, arguments }
    }
}

#[derive(Clone, serde::Deserialize, serde::Serialize)]
#[php_class]
pub struct AssistantMessage {
    pub content: Option<String>,
    pub name: Option<String>,
    pub tool_calls: Option<Vec<ToolCall>>,
    pub refusal: Option<String>,
    pub function_call: Option<FunctionCall>,
}

#[php_impl]
impl AssistantMessage {
    pub fn from_json(json: String) -> PhpResult<Self> {
        serde_json::from_str(&json).map_err(|e| PhpException::default(e.to_string()).into())
    }
}

#[derive(Clone, serde::Deserialize, serde::Serialize)]
#[php_class]
pub struct ChatCompletionTool {
    pub tool_type: String,
    pub function: FunctionDefinition,
}

#[php_impl]
impl ChatCompletionTool {
    pub fn from_json(json: String) -> PhpResult<Self> {
        serde_json::from_str(&json).map_err(|e| PhpException::default(e.to_string()).into())
    }
}

#[derive(Clone, serde::Deserialize, serde::Serialize)]
#[php_class]
pub struct FunctionDefinition {
    pub name: String,
    pub description: Option<String>,
    pub parameters: Option<String>,
    pub strict: Option<bool>,
}

#[php_impl]
impl FunctionDefinition {
    pub fn __construct(
        name: String,
        description: Option<String>,
        parameters: Option<String>,
        strict: Option<bool>,
    ) -> Self {
        Self {
            name,
            description,
            parameters,
            strict,
        }
    }
}

#[derive(Clone, serde::Deserialize, serde::Serialize)]
#[php_class]
pub struct ToolCall {
    pub id: String,
    pub call_type: String,
    pub function: FunctionCall,
}

#[php_impl]
impl ToolCall {
    pub fn from_json(json: String) -> PhpResult<Self> {
        serde_json::from_str(&json).map_err(|e| PhpException::default(e.to_string()).into())
    }
}

#[derive(Clone, serde::Deserialize, serde::Serialize)]
#[php_class]
pub struct FunctionCall {
    pub name: String,
    pub arguments: String,
}

#[php_impl]
impl FunctionCall {
    pub fn __construct(name: String, arguments: String) -> Self {
        Self { name, arguments }
    }
}

#[derive(Clone, serde::Deserialize, serde::Serialize)]
#[php_class]
pub struct Usage {
    pub prompt_tokens: i64,
    pub completion_tokens: i64,
    pub total_tokens: i64,
}

#[php_impl]
impl Usage {
    pub fn __construct(prompt_tokens: i64, completion_tokens: i64, total_tokens: i64) -> Self {
        Self {
            prompt_tokens,
            completion_tokens,
            total_tokens,
        }
    }
}

#[derive(Clone, serde::Deserialize, serde::Serialize)]
#[php_class]
pub struct EmbeddingRequest {
    pub model: String,
    pub input: String,
    pub encoding_format: Option<String>,
    pub dimensions: Option<u32>,
    pub user: Option<String>,
}

#[php_impl]
impl EmbeddingRequest {
    pub fn from_json(json: String) -> PhpResult<Self> {
        serde_json::from_str(&json).map_err(|e| PhpException::default(e.to_string()).into())
    }
}

#[derive(Clone, serde::Deserialize, serde::Serialize)]
#[php_class]
pub struct EmbeddingResponse {
    pub object: String,
    pub data: Vec<EmbeddingObject>,
    pub model: String,
    pub usage: Option<Usage>,
}

#[php_impl]
impl EmbeddingResponse {
    pub fn from_json(json: String) -> PhpResult<Self> {
        serde_json::from_str(&json).map_err(|e| PhpException::default(e.to_string()).into())
    }

    pub fn estimated_cost(&self) -> Option<f64> {
        None
    }
}

#[derive(Clone, serde::Deserialize, serde::Serialize)]
#[php_class]
pub struct EmbeddingObject {
    pub object: String,
    pub embedding: Vec<f64>,
    pub index: u32,
}

#[php_impl]
impl EmbeddingObject {
    pub fn __construct(object: String, embedding: Vec<f64>, index: u32) -> Self {
        Self {
            object,
            embedding,
            index,
        }
    }
}

#[derive(Clone, serde::Deserialize, serde::Serialize)]
#[php_class]
pub struct CreateImageRequest {
    pub prompt: String,
    pub model: Option<String>,
    pub n: Option<u32>,
    pub size: Option<String>,
    pub quality: Option<String>,
    pub style: Option<String>,
    pub response_format: Option<String>,
    pub user: Option<String>,
}

#[php_impl]
impl CreateImageRequest {
    pub fn __construct(
        prompt: String,
        model: Option<String>,
        n: Option<u32>,
        size: Option<String>,
        quality: Option<String>,
        style: Option<String>,
        response_format: Option<String>,
        user: Option<String>,
    ) -> Self {
        Self {
            prompt,
            model,
            n,
            size,
            quality,
            style,
            response_format,
            user,
        }
    }
}

#[derive(Clone, serde::Deserialize, serde::Serialize)]
#[php_class]
pub struct ImagesResponse {
    pub created: i64,
    pub data: Vec<Image>,
}

#[php_impl]
impl ImagesResponse {
    pub fn from_json(json: String) -> PhpResult<Self> {
        serde_json::from_str(&json).map_err(|e| PhpException::default(e.to_string()).into())
    }
}

#[derive(Clone, serde::Deserialize, serde::Serialize)]
#[php_class]
pub struct Image {
    pub url: Option<String>,
    pub b64_json: Option<String>,
    pub revised_prompt: Option<String>,
}

#[php_impl]
impl Image {
    pub fn __construct(url: Option<String>, b64_json: Option<String>, revised_prompt: Option<String>) -> Self {
        Self {
            url,
            b64_json,
            revised_prompt,
        }
    }
}

#[derive(Clone, serde::Deserialize, serde::Serialize)]
#[php_class]
pub struct ModelsListResponse {
    pub object: String,
    pub data: Vec<ModelObject>,
}

#[php_impl]
impl ModelsListResponse {
    pub fn from_json(json: String) -> PhpResult<Self> {
        serde_json::from_str(&json).map_err(|e| PhpException::default(e.to_string()).into())
    }
}

#[derive(Clone, serde::Deserialize, serde::Serialize)]
#[php_class]
pub struct ModelObject {
    pub id: String,
    pub object: String,
    pub created: i64,
    pub owned_by: String,
}

#[php_impl]
impl ModelObject {
    pub fn __construct(id: String, object: String, created: i64, owned_by: String) -> Self {
        Self {
            id,
            object,
            created,
            owned_by,
        }
    }
}

#[derive(Clone, serde::Deserialize, serde::Serialize)]
#[php_class]
pub struct ModerationRequest {
    pub input: String,
    pub model: Option<String>,
}

#[php_impl]
impl ModerationRequest {
    pub fn from_json(json: String) -> PhpResult<Self> {
        serde_json::from_str(&json).map_err(|e| PhpException::default(e.to_string()).into())
    }
}

#[derive(Clone, serde::Deserialize, serde::Serialize)]
#[php_class]
pub struct ModerationResponse {
    pub id: String,
    pub model: String,
    pub results: Vec<ModerationResult>,
}

#[php_impl]
impl ModerationResponse {
    pub fn from_json(json: String) -> PhpResult<Self> {
        serde_json::from_str(&json).map_err(|e| PhpException::default(e.to_string()).into())
    }
}

#[derive(Clone, serde::Deserialize, serde::Serialize)]
#[php_class]
pub struct ModerationResult {
    pub flagged: bool,
    pub categories: ModerationCategories,
    pub category_scores: ModerationCategoryScores,
}

#[php_impl]
impl ModerationResult {
    pub fn from_json(json: String) -> PhpResult<Self> {
        serde_json::from_str(&json).map_err(|e| PhpException::default(e.to_string()).into())
    }
}

#[derive(Clone, serde::Deserialize, serde::Serialize)]
#[php_class]
pub struct ModerationCategories {
    pub sexual: bool,
    pub hate: bool,
    pub harassment: bool,
    pub self_harm: bool,
    pub sexual_minors: bool,
    pub hate_threatening: bool,
    pub violence_graphic: bool,
    pub self_harm_intent: bool,
    pub self_harm_instructions: bool,
    pub harassment_threatening: bool,
    pub violence: bool,
}

#[php_impl]
impl ModerationCategories {
    pub fn __construct(
        sexual: bool,
        hate: bool,
        harassment: bool,
        self_harm: bool,
        sexual_minors: bool,
        hate_threatening: bool,
        violence_graphic: bool,
        self_harm_intent: bool,
        self_harm_instructions: bool,
        harassment_threatening: bool,
        violence: bool,
    ) -> Self {
        Self {
            sexual,
            hate,
            harassment,
            self_harm,
            sexual_minors,
            hate_threatening,
            violence_graphic,
            self_harm_intent,
            self_harm_instructions,
            harassment_threatening,
            violence,
        }
    }
}

#[derive(Clone, serde::Deserialize, serde::Serialize)]
#[php_class]
pub struct ModerationCategoryScores {
    pub sexual: f64,
    pub hate: f64,
    pub harassment: f64,
    pub self_harm: f64,
    pub sexual_minors: f64,
    pub hate_threatening: f64,
    pub violence_graphic: f64,
    pub self_harm_intent: f64,
    pub self_harm_instructions: f64,
    pub harassment_threatening: f64,
    pub violence: f64,
}

#[php_impl]
impl ModerationCategoryScores {
    pub fn __construct(
        sexual: f64,
        hate: f64,
        harassment: f64,
        self_harm: f64,
        sexual_minors: f64,
        hate_threatening: f64,
        violence_graphic: f64,
        self_harm_intent: f64,
        self_harm_instructions: f64,
        harassment_threatening: f64,
        violence: f64,
    ) -> Self {
        Self {
            sexual,
            hate,
            harassment,
            self_harm,
            sexual_minors,
            hate_threatening,
            violence_graphic,
            self_harm_intent,
            self_harm_instructions,
            harassment_threatening,
            violence,
        }
    }
}

#[derive(Clone, serde::Deserialize, serde::Serialize)]
#[php_class]
pub struct RerankRequest {
    pub model: String,
    pub query: String,
    pub documents: Vec<String>,
    pub top_n: Option<u32>,
    pub return_documents: Option<bool>,
}

#[php_impl]
impl RerankRequest {
    pub fn from_json(json: String) -> PhpResult<Self> {
        serde_json::from_str(&json).map_err(|e| PhpException::default(e.to_string()).into())
    }
}

#[derive(Clone, serde::Deserialize, serde::Serialize)]
#[php_class]
pub struct RerankResponse {
    pub id: Option<String>,
    pub results: Vec<RerankResult>,
    pub meta: Option<String>,
}

#[php_impl]
impl RerankResponse {
    pub fn from_json(json: String) -> PhpResult<Self> {
        serde_json::from_str(&json).map_err(|e| PhpException::default(e.to_string()).into())
    }
}

#[derive(Clone, serde::Deserialize, serde::Serialize)]
#[php_class]
pub struct RerankResult {
    pub index: u32,
    pub relevance_score: f64,
    pub document: Option<RerankResultDocument>,
}

#[php_impl]
impl RerankResult {
    pub fn from_json(json: String) -> PhpResult<Self> {
        serde_json::from_str(&json).map_err(|e| PhpException::default(e.to_string()).into())
    }
}

#[derive(Clone, serde::Deserialize, serde::Serialize)]
#[php_class]
pub struct RerankResultDocument {
    pub text: String,
}

#[php_impl]
impl RerankResultDocument {
    pub fn __construct(text: String) -> Self {
        Self { text }
    }
}

// FinishReason enum values
pub const FINISHREASON_STOP: &str = "Stop";
pub const FINISHREASON_LENGTH: &str = "Length";
pub const FINISHREASON_TOOLCALLS: &str = "ToolCalls";
pub const FINISHREASON_CONTENTFILTER: &str = "ContentFilter";
pub const FINISHREASON_FUNCTIONCALL: &str = "FunctionCall";
pub const FINISHREASON_OTHER: &str = "Other";

// ReasoningEffort enum values
pub const REASONINGEFFORT_LOW: &str = "Low";
pub const REASONINGEFFORT_MEDIUM: &str = "Medium";
pub const REASONINGEFFORT_HIGH: &str = "High";

// Message enum values
pub const MESSAGE_SYSTEM: &str = "System";
pub const MESSAGE_USER: &str = "User";
pub const MESSAGE_ASSISTANT: &str = "Assistant";
pub const MESSAGE_TOOL: &str = "Tool";
pub const MESSAGE_DEVELOPER: &str = "Developer";
pub const MESSAGE_FUNCTION: &str = "Function";

// ToolType enum values
pub const TOOLTYPE_FUNCTION: &str = "Function";

// ToolChoice enum values
pub const TOOLCHOICE_MODE: &str = "Mode";
pub const TOOLCHOICE_SPECIFIC: &str = "Specific";

// ResponseFormat enum values
pub const RESPONSEFORMAT_TEXT: &str = "Text";
pub const RESPONSEFORMAT_JSONOBJECT: &str = "JsonObject";
pub const RESPONSEFORMAT_JSONSCHEMA: &str = "JsonSchema";

// StopSequence enum values
pub const STOPSEQUENCE_SINGLE: &str = "Single";
pub const STOPSEQUENCE_MULTIPLE: &str = "Multiple";

// EmbeddingFormat enum values
pub const EMBEDDINGFORMAT_FLOAT: &str = "Float";
pub const EMBEDDINGFORMAT_BASE64: &str = "Base64";

// EmbeddingInput enum values
pub const EMBEDDINGINPUT_SINGLE: &str = "Single";
pub const EMBEDDINGINPUT_MULTIPLE: &str = "Multiple";

// ModerationInput enum values
pub const MODERATIONINPUT_SINGLE: &str = "Single";
pub const MODERATIONINPUT_MULTIPLE: &str = "Multiple";

// RerankDocument enum values
pub const RERANKDOCUMENT_TEXT: &str = "Text";
pub const RERANKDOCUMENT_OBJECT: &str = "Object";

#[php_function]
pub fn completion_cost() -> Option<f64> {
    None
}

impl From<ModelPricing> for liter_llm::ModelPricing {
    fn from(val: ModelPricing) -> Self {
        Self {
            input_cost_per_token: val.input_cost_per_token,
            output_cost_per_token: val.output_cost_per_token,
        }
    }
}

impl From<liter_llm::ModelPricing> for ModelPricing {
    fn from(val: liter_llm::ModelPricing) -> Self {
        Self {
            input_cost_per_token: val.input_cost_per_token,
            output_cost_per_token: val.output_cost_per_token,
        }
    }
}

impl From<CreateSpeechRequest> for liter_llm::CreateSpeechRequest {
    fn from(val: CreateSpeechRequest) -> Self {
        Self {
            model: val.model,
            input: val.input,
            voice: val.voice,
            response_format: val.response_format,
            speed: val.speed,
        }
    }
}

impl From<liter_llm::CreateSpeechRequest> for CreateSpeechRequest {
    fn from(val: liter_llm::CreateSpeechRequest) -> Self {
        Self {
            model: val.model,
            input: val.input,
            voice: val.voice,
            response_format: val.response_format,
            speed: val.speed,
        }
    }
}

impl From<TranscriptionResponse> for liter_llm::TranscriptionResponse {
    fn from(val: TranscriptionResponse) -> Self {
        Self {
            text: val.text,
            language: val.language,
            duration: val.duration,
            segments: val.segments.map(|v| v.into_iter().map(Into::into).collect()),
        }
    }
}

impl From<liter_llm::TranscriptionResponse> for TranscriptionResponse {
    fn from(val: liter_llm::TranscriptionResponse) -> Self {
        Self {
            text: val.text,
            language: val.language,
            duration: val.duration,
            segments: val.segments.map(|v| v.into_iter().map(Into::into).collect()),
        }
    }
}

impl From<TranscriptionSegment> for liter_llm::TranscriptionSegment {
    fn from(val: TranscriptionSegment) -> Self {
        Self {
            id: val.id,
            start: val.start,
            end: val.end,
            text: val.text,
        }
    }
}

impl From<liter_llm::TranscriptionSegment> for TranscriptionSegment {
    fn from(val: liter_llm::TranscriptionSegment) -> Self {
        Self {
            id: val.id,
            start: val.start,
            end: val.end,
            text: val.text,
        }
    }
}

impl From<ChatCompletionRequest> for liter_llm::ChatCompletionRequest {
    fn from(val: ChatCompletionRequest) -> Self {
        let json = serde_json::to_string(&val).expect("skif: serialize binding type");
        serde_json::from_str(&json).expect("skif: deserialize to core type")
    }
}

impl From<ChatCompletionResponse> for liter_llm::ChatCompletionResponse {
    fn from(val: ChatCompletionResponse) -> Self {
        let json = serde_json::to_string(&val).expect("skif: serialize binding type");
        serde_json::from_str(&json).expect("skif: deserialize to core type")
    }
}

impl From<liter_llm::ChatCompletionResponse> for ChatCompletionResponse {
    fn from(val: liter_llm::ChatCompletionResponse) -> Self {
        Self {
            id: val.id,
            object: val.object,
            created: val.created as i64,
            model: val.model,
            choices: val.choices.into_iter().map(Into::into).collect(),
            usage: val.usage.map(Into::into),
            system_fingerprint: val.system_fingerprint,
            service_tier: val.service_tier,
        }
    }
}

impl From<Choice> for liter_llm::Choice {
    fn from(val: Choice) -> Self {
        let json = serde_json::to_string(&val).expect("skif: serialize binding type");
        serde_json::from_str(&json).expect("skif: deserialize to core type")
    }
}

impl From<liter_llm::Choice> for Choice {
    fn from(val: liter_llm::Choice) -> Self {
        Self {
            index: val.index,
            message: val.message.into(),
            finish_reason: val.finish_reason.as_ref().map(|v| format!("{:?}", v)),
        }
    }
}

impl From<ChatCompletionChunk> for liter_llm::ChatCompletionChunk {
    fn from(val: ChatCompletionChunk) -> Self {
        let json = serde_json::to_string(&val).expect("skif: serialize binding type");
        serde_json::from_str(&json).expect("skif: deserialize to core type")
    }
}

impl From<liter_llm::ChatCompletionChunk> for ChatCompletionChunk {
    fn from(val: liter_llm::ChatCompletionChunk) -> Self {
        Self {
            id: val.id,
            object: val.object,
            created: val.created as i64,
            model: val.model,
            choices: val.choices.into_iter().map(Into::into).collect(),
            usage: val.usage.map(Into::into),
            system_fingerprint: val.system_fingerprint,
            service_tier: val.service_tier,
        }
    }
}

impl From<StreamChoice> for liter_llm::StreamChoice {
    fn from(val: StreamChoice) -> Self {
        let json = serde_json::to_string(&val).expect("skif: serialize binding type");
        serde_json::from_str(&json).expect("skif: deserialize to core type")
    }
}

impl From<liter_llm::StreamChoice> for StreamChoice {
    fn from(val: liter_llm::StreamChoice) -> Self {
        Self {
            index: val.index,
            delta: val.delta.into(),
            finish_reason: val.finish_reason.as_ref().map(|v| format!("{:?}", v)),
        }
    }
}

impl From<StreamDelta> for liter_llm::StreamDelta {
    fn from(val: StreamDelta) -> Self {
        let json = serde_json::to_string(&val).expect("skif: serialize binding type");
        serde_json::from_str(&json).expect("skif: deserialize to core type")
    }
}

impl From<liter_llm::StreamDelta> for StreamDelta {
    fn from(val: liter_llm::StreamDelta) -> Self {
        Self {
            role: val.role,
            content: val.content,
            tool_calls: val.tool_calls.map(|v| v.into_iter().map(Into::into).collect()),
            function_call: val.function_call.map(Into::into),
            refusal: val.refusal,
        }
    }
}

impl From<StreamToolCall> for liter_llm::StreamToolCall {
    fn from(val: StreamToolCall) -> Self {
        let json = serde_json::to_string(&val).expect("skif: serialize binding type");
        serde_json::from_str(&json).expect("skif: deserialize to core type")
    }
}

impl From<liter_llm::StreamToolCall> for StreamToolCall {
    fn from(val: liter_llm::StreamToolCall) -> Self {
        Self {
            index: val.index,
            id: val.id,
            call_type: val.call_type.as_ref().map(|v| format!("{:?}", v)),
            function: val.function.map(Into::into),
        }
    }
}

impl From<StreamFunctionCall> for liter_llm::StreamFunctionCall {
    fn from(val: StreamFunctionCall) -> Self {
        Self {
            name: val.name,
            arguments: val.arguments,
        }
    }
}

impl From<liter_llm::StreamFunctionCall> for StreamFunctionCall {
    fn from(val: liter_llm::StreamFunctionCall) -> Self {
        Self {
            name: val.name,
            arguments: val.arguments,
        }
    }
}

impl From<AssistantMessage> for liter_llm::AssistantMessage {
    fn from(val: AssistantMessage) -> Self {
        let json = serde_json::to_string(&val).expect("skif: serialize binding type");
        serde_json::from_str(&json).expect("skif: deserialize to core type")
    }
}

impl From<liter_llm::AssistantMessage> for AssistantMessage {
    fn from(val: liter_llm::AssistantMessage) -> Self {
        Self {
            content: val.content,
            name: val.name,
            tool_calls: val.tool_calls.map(|v| v.into_iter().map(Into::into).collect()),
            refusal: val.refusal,
            function_call: val.function_call.map(Into::into),
        }
    }
}

impl From<ChatCompletionTool> for liter_llm::ChatCompletionTool {
    fn from(val: ChatCompletionTool) -> Self {
        let json = serde_json::to_string(&val).expect("skif: serialize binding type");
        serde_json::from_str(&json).expect("skif: deserialize to core type")
    }
}

impl From<ToolCall> for liter_llm::ToolCall {
    fn from(val: ToolCall) -> Self {
        let json = serde_json::to_string(&val).expect("skif: serialize binding type");
        serde_json::from_str(&json).expect("skif: deserialize to core type")
    }
}

impl From<liter_llm::ToolCall> for ToolCall {
    fn from(val: liter_llm::ToolCall) -> Self {
        Self {
            id: val.id,
            call_type: format!("{:?}", val.call_type),
            function: val.function.into(),
        }
    }
}

impl From<FunctionCall> for liter_llm::FunctionCall {
    fn from(val: FunctionCall) -> Self {
        Self {
            name: val.name,
            arguments: val.arguments,
        }
    }
}

impl From<liter_llm::FunctionCall> for FunctionCall {
    fn from(val: liter_llm::FunctionCall) -> Self {
        Self {
            name: val.name,
            arguments: val.arguments,
        }
    }
}

impl From<Usage> for liter_llm::Usage {
    fn from(val: Usage) -> Self {
        Self {
            prompt_tokens: val.prompt_tokens as u64,
            completion_tokens: val.completion_tokens as u64,
            total_tokens: val.total_tokens as u64,
        }
    }
}

impl From<liter_llm::Usage> for Usage {
    fn from(val: liter_llm::Usage) -> Self {
        Self {
            prompt_tokens: val.prompt_tokens as i64,
            completion_tokens: val.completion_tokens as i64,
            total_tokens: val.total_tokens as i64,
        }
    }
}

impl From<EmbeddingRequest> for liter_llm::EmbeddingRequest {
    fn from(val: EmbeddingRequest) -> Self {
        let json = serde_json::to_string(&val).expect("skif: serialize binding type");
        serde_json::from_str(&json).expect("skif: deserialize to core type")
    }
}

impl From<liter_llm::EmbeddingRequest> for EmbeddingRequest {
    fn from(val: liter_llm::EmbeddingRequest) -> Self {
        Self {
            model: val.model,
            input: format!("{:?}", val.input),
            encoding_format: val.encoding_format.as_ref().map(|v| format!("{:?}", v)),
            dimensions: val.dimensions,
            user: val.user,
        }
    }
}

impl From<EmbeddingResponse> for liter_llm::EmbeddingResponse {
    fn from(val: EmbeddingResponse) -> Self {
        Self {
            object: val.object,
            data: val.data.into_iter().map(Into::into).collect(),
            model: val.model,
            usage: val.usage.map(Into::into),
        }
    }
}

impl From<liter_llm::EmbeddingResponse> for EmbeddingResponse {
    fn from(val: liter_llm::EmbeddingResponse) -> Self {
        Self {
            object: val.object,
            data: val.data.into_iter().map(Into::into).collect(),
            model: val.model,
            usage: val.usage.map(Into::into),
        }
    }
}

impl From<EmbeddingObject> for liter_llm::EmbeddingObject {
    fn from(val: EmbeddingObject) -> Self {
        Self {
            object: val.object,
            embedding: val.embedding,
            index: val.index,
        }
    }
}

impl From<liter_llm::EmbeddingObject> for EmbeddingObject {
    fn from(val: liter_llm::EmbeddingObject) -> Self {
        Self {
            object: val.object,
            embedding: val.embedding,
            index: val.index,
        }
    }
}

impl From<CreateImageRequest> for liter_llm::CreateImageRequest {
    fn from(val: CreateImageRequest) -> Self {
        Self {
            prompt: val.prompt,
            model: val.model,
            n: val.n,
            size: val.size,
            quality: val.quality,
            style: val.style,
            response_format: val.response_format,
            user: val.user,
        }
    }
}

impl From<liter_llm::CreateImageRequest> for CreateImageRequest {
    fn from(val: liter_llm::CreateImageRequest) -> Self {
        Self {
            prompt: val.prompt,
            model: val.model,
            n: val.n,
            size: val.size,
            quality: val.quality,
            style: val.style,
            response_format: val.response_format,
            user: val.user,
        }
    }
}

impl From<ImagesResponse> for liter_llm::ImagesResponse {
    fn from(val: ImagesResponse) -> Self {
        Self {
            created: val.created as u64,
            data: val.data.into_iter().map(Into::into).collect(),
        }
    }
}

impl From<liter_llm::ImagesResponse> for ImagesResponse {
    fn from(val: liter_llm::ImagesResponse) -> Self {
        Self {
            created: val.created as i64,
            data: val.data.into_iter().map(Into::into).collect(),
        }
    }
}

impl From<Image> for liter_llm::Image {
    fn from(val: Image) -> Self {
        Self {
            url: val.url,
            b64_json: val.b64_json,
            revised_prompt: val.revised_prompt,
        }
    }
}

impl From<liter_llm::Image> for Image {
    fn from(val: liter_llm::Image) -> Self {
        Self {
            url: val.url,
            b64_json: val.b64_json,
            revised_prompt: val.revised_prompt,
        }
    }
}

impl From<ModelsListResponse> for liter_llm::ModelsListResponse {
    fn from(val: ModelsListResponse) -> Self {
        Self {
            object: val.object,
            data: val.data.into_iter().map(Into::into).collect(),
        }
    }
}

impl From<liter_llm::ModelsListResponse> for ModelsListResponse {
    fn from(val: liter_llm::ModelsListResponse) -> Self {
        Self {
            object: val.object,
            data: val.data.into_iter().map(Into::into).collect(),
        }
    }
}

impl From<ModelObject> for liter_llm::ModelObject {
    fn from(val: ModelObject) -> Self {
        Self {
            id: val.id,
            object: val.object,
            created: val.created as u64,
            owned_by: val.owned_by,
        }
    }
}

impl From<liter_llm::ModelObject> for ModelObject {
    fn from(val: liter_llm::ModelObject) -> Self {
        Self {
            id: val.id,
            object: val.object,
            created: val.created as i64,
            owned_by: val.owned_by,
        }
    }
}

impl From<ModerationRequest> for liter_llm::ModerationRequest {
    fn from(val: ModerationRequest) -> Self {
        let json = serde_json::to_string(&val).expect("skif: serialize binding type");
        serde_json::from_str(&json).expect("skif: deserialize to core type")
    }
}

impl From<liter_llm::ModerationRequest> for ModerationRequest {
    fn from(val: liter_llm::ModerationRequest) -> Self {
        Self {
            input: format!("{:?}", val.input),
            model: val.model,
        }
    }
}

impl From<ModerationResponse> for liter_llm::ModerationResponse {
    fn from(val: ModerationResponse) -> Self {
        Self {
            id: val.id,
            model: val.model,
            results: val.results.into_iter().map(Into::into).collect(),
        }
    }
}

impl From<liter_llm::ModerationResponse> for ModerationResponse {
    fn from(val: liter_llm::ModerationResponse) -> Self {
        Self {
            id: val.id,
            model: val.model,
            results: val.results.into_iter().map(Into::into).collect(),
        }
    }
}

impl From<ModerationResult> for liter_llm::ModerationResult {
    fn from(val: ModerationResult) -> Self {
        Self {
            flagged: val.flagged,
            categories: val.categories.into(),
            category_scores: val.category_scores.into(),
        }
    }
}

impl From<liter_llm::ModerationResult> for ModerationResult {
    fn from(val: liter_llm::ModerationResult) -> Self {
        Self {
            flagged: val.flagged,
            categories: val.categories.into(),
            category_scores: val.category_scores.into(),
        }
    }
}

impl From<ModerationCategories> for liter_llm::ModerationCategories {
    fn from(val: ModerationCategories) -> Self {
        Self {
            sexual: val.sexual,
            hate: val.hate,
            harassment: val.harassment,
            self_harm: val.self_harm,
            sexual_minors: val.sexual_minors,
            hate_threatening: val.hate_threatening,
            violence_graphic: val.violence_graphic,
            self_harm_intent: val.self_harm_intent,
            self_harm_instructions: val.self_harm_instructions,
            harassment_threatening: val.harassment_threatening,
            violence: val.violence,
        }
    }
}

impl From<liter_llm::ModerationCategories> for ModerationCategories {
    fn from(val: liter_llm::ModerationCategories) -> Self {
        Self {
            sexual: val.sexual,
            hate: val.hate,
            harassment: val.harassment,
            self_harm: val.self_harm,
            sexual_minors: val.sexual_minors,
            hate_threatening: val.hate_threatening,
            violence_graphic: val.violence_graphic,
            self_harm_intent: val.self_harm_intent,
            self_harm_instructions: val.self_harm_instructions,
            harassment_threatening: val.harassment_threatening,
            violence: val.violence,
        }
    }
}

impl From<ModerationCategoryScores> for liter_llm::ModerationCategoryScores {
    fn from(val: ModerationCategoryScores) -> Self {
        Self {
            sexual: val.sexual,
            hate: val.hate,
            harassment: val.harassment,
            self_harm: val.self_harm,
            sexual_minors: val.sexual_minors,
            hate_threatening: val.hate_threatening,
            violence_graphic: val.violence_graphic,
            self_harm_intent: val.self_harm_intent,
            self_harm_instructions: val.self_harm_instructions,
            harassment_threatening: val.harassment_threatening,
            violence: val.violence,
        }
    }
}

impl From<liter_llm::ModerationCategoryScores> for ModerationCategoryScores {
    fn from(val: liter_llm::ModerationCategoryScores) -> Self {
        Self {
            sexual: val.sexual,
            hate: val.hate,
            harassment: val.harassment,
            self_harm: val.self_harm,
            sexual_minors: val.sexual_minors,
            hate_threatening: val.hate_threatening,
            violence_graphic: val.violence_graphic,
            self_harm_intent: val.self_harm_intent,
            self_harm_instructions: val.self_harm_instructions,
            harassment_threatening: val.harassment_threatening,
            violence: val.violence,
        }
    }
}

impl From<RerankRequest> for liter_llm::RerankRequest {
    fn from(val: RerankRequest) -> Self {
        let json = serde_json::to_string(&val).expect("skif: serialize binding type");
        serde_json::from_str(&json).expect("skif: deserialize to core type")
    }
}

impl From<liter_llm::RerankRequest> for RerankRequest {
    fn from(val: liter_llm::RerankRequest) -> Self {
        Self {
            model: val.model,
            query: val.query,
            documents: val.documents.iter().map(|v| format!("{:?}", v)).collect(),
            top_n: val.top_n,
            return_documents: val.return_documents,
        }
    }
}

impl From<RerankResult> for liter_llm::RerankResult {
    fn from(val: RerankResult) -> Self {
        Self {
            index: val.index,
            relevance_score: val.relevance_score,
            document: val.document.map(Into::into),
        }
    }
}

impl From<liter_llm::RerankResult> for RerankResult {
    fn from(val: liter_llm::RerankResult) -> Self {
        Self {
            index: val.index,
            relevance_score: val.relevance_score,
            document: val.document.map(Into::into),
        }
    }
}

impl From<RerankResultDocument> for liter_llm::RerankResultDocument {
    fn from(val: RerankResultDocument) -> Self {
        Self { text: val.text }
    }
}

impl From<liter_llm::RerankResultDocument> for RerankResultDocument {
    fn from(val: liter_llm::RerankResultDocument) -> Self {
        Self { text: val.text }
    }
}
