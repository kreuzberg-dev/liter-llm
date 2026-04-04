package literllm

/*
#include "liter_llm.h"
*/
import "C"

import (
	"encoding/json"
	"fmt"
	"unsafe"
)

// lastError retrieves the last error from the FFI layer.
func lastError() error {
	code := int32(C.liter_llm_last_error_code())
	if code == 0 {
		return nil
	}
	ctx := C.liter_llm_last_error_context()
	message := C.GoString(ctx)
	C.liter_llm_free_string(ctx)
	return fmt.Errorf("[%d] %s", code, message)
}

// Why a choice stopped generating tokens.
type FinishReason string

const (
	FinishReasonStop          FinishReason = "stop"
	FinishReasonLength        FinishReason = "length"
	FinishReasonToolCalls     FinishReason = "tool_calls"
	FinishReasonContentFilter FinishReason = "content_filter"
	// Deprecated legacy finish reason; retained for API compatibility.
	FinishReasonFunctionCall FinishReason = "function_call"
	// Catch-all for unknown finish reasons returned by non-OpenAI providers.
	//
	// Note: this intentionally does **not** carry the original string (e.g.
	// `Other(String)`).  Using `#[serde(other)]` requires a unit variant, and
	// switching to `#[serde(untagged)]` would change deserialization semantics
	// for all variants.  The original value can be recovered by inspecting the
	// raw JSON if needed.
	FinishReasonOther FinishReason = "other"
)

// Controls how much reasoning effort the model should use.
type ReasoningEffort string

const (
	ReasoningEffortLow    ReasoningEffort = "low"
	ReasoningEffortMedium ReasoningEffort = "medium"
	ReasoningEffortHigh   ReasoningEffort = "high"
)

// A chat message in a conversation.
// Variants: System, User, Assistant, Tool, Developer, Function
type Message struct {
}

// The type discriminator for tool/tool-call objects. Per the OpenAI spec this
// is always `"function"`. Using an enum enforces that constraint at the type
// level and rejects any other value on deserialization.
type ToolType string

const (
	ToolTypeFunction ToolType = "function"
)

// ToolChoice is a tagged union type (discriminated by JSON tag).
// Variants: Mode, Specific
type ToolChoice struct {
}

// ResponseFormat is a tagged union type (discriminated by JSON tag).
// Variants: Text, JsonObject, JsonSchema
type ResponseFormat struct {
	JsonSchema *JsonSchemaFormat `json:"json_schema,omitempty"`
}

// StopSequence is a tagged union type (discriminated by JSON tag).
// Variants: Single, Multiple
type StopSequence struct {
}

// The format in which the embedding vectors are returned.
type EmbeddingFormat string

const (
	// 32-bit floating-point numbers (default).
	EmbeddingFormatFloat EmbeddingFormat = "float"
	// Base64-encoded string representation of the floats.
	EmbeddingFormatBase64 EmbeddingFormat = "base64"
)

// EmbeddingInput is a tagged union type (discriminated by JSON tag).
// Variants: Single, Multiple
type EmbeddingInput struct {
}

// Input to the moderation endpoint — a single string or multiple strings.
// Variants: Single, Multiple
type ModerationInput struct {
}

// A document to be reranked — either a plain string or an object with a text field.
// Variants: Text, Object
type RerankDocument struct {
	Text *string `json:"text,omitempty"`
}

// Per-token pricing for a single model (USD per token).
type ModelPricing struct {
	// Cost in USD per input (prompt) token.
	InputCostPerToken float64 `json:"input_cost_per_token"`
	// Cost in USD per output (completion) token.  Zero for embedding models.
	OutputCostPerToken float64 `json:"output_cost_per_token"`
}

// Request to generate speech audio from text.
type CreateSpeechRequest struct {
	Model          string   `json:"model"`
	Input          string   `json:"input"`
	Voice          string   `json:"voice"`
	ResponseFormat *string  `json:"response_format,omitempty"`
	Speed          *float64 `json:"speed,omitempty"`
}

// Response from a transcription request.
type TranscriptionResponse struct {
	Text     string                  `json:"text"`
	Language *string                 `json:"language,omitempty"`
	Duration *float64                `json:"duration,omitempty"`
	Segments *[]TranscriptionSegment `json:"segments,omitempty"`
}

// A segment of transcribed audio with timing information.
type TranscriptionSegment struct {
	Id    uint32  `json:"id"`
	Start float64 `json:"start"`
	End   float64 `json:"end"`
	Text  string  `json:"text"`
}

// ChatCompletionRequest is a type.
type ChatCompletionRequest struct {
	Model            string        `json:"model"`
	Messages         []Message     `json:"messages"`
	Temperature      *float64      `json:"temperature,omitempty"`
	TopP             *float64      `json:"top_p,omitempty"`
	N                *uint32       `json:"n,omitempty"`
	Stop             *StopSequence `json:"stop,omitempty"`
	MaxTokens        *uint64       `json:"max_tokens,omitempty"`
	PresencePenalty  *float64      `json:"presence_penalty,omitempty"`
	FrequencyPenalty *float64      `json:"frequency_penalty,omitempty"`
	// Token bias map.  Uses `BTreeMap` (sorted keys) for deterministic
	// serialization order — important when hashing or signing requests.
	LogitBias         *map[string]float64   `json:"logit_bias,omitempty"`
	User              *string               `json:"user,omitempty"`
	Tools             *[]ChatCompletionTool `json:"tools,omitempty"`
	ToolChoice        *ToolChoice           `json:"tool_choice,omitempty"`
	ParallelToolCalls *bool                 `json:"parallel_tool_calls,omitempty"`
	ResponseFormat    *ResponseFormat       `json:"response_format,omitempty"`
	StreamOptions     *string               `json:"stream_options,omitempty"`
	Seed              *int64                `json:"seed,omitempty"`
	ReasoningEffort   *ReasoningEffort      `json:"reasoning_effort,omitempty"`
	// Provider-specific extra parameters merged into the request body.
	// Use for guardrails, safety settings, grounding config, etc.
	ExtraBody *map[string]interface{} `json:"extra_body,omitempty"`
}

// ChatCompletionResponse is a type.
type ChatCompletionResponse struct {
	Id string `json:"id"`
	// Always `"chat.completion"` from OpenAI-compatible APIs.  Stored as a
	// plain `String` so non-standard provider values do not break deserialization.
	Object            string   `json:"object"`
	Created           uint64   `json:"created"`
	Model             string   `json:"model"`
	Choices           []Choice `json:"choices"`
	Usage             *Usage   `json:"usage,omitempty"`
	SystemFingerprint *string  `json:"system_fingerprint,omitempty"`
	ServiceTier       *string  `json:"service_tier,omitempty"`
}

// Choice is a type.
type Choice struct {
	Index        uint32           `json:"index"`
	Message      AssistantMessage `json:"message"`
	FinishReason *FinishReason    `json:"finish_reason,omitempty"`
}

// ChatCompletionChunk is a type.
type ChatCompletionChunk struct {
	Id string `json:"id"`
	// Always `"chat.completion.chunk"` from OpenAI-compatible APIs.  Stored
	// as a plain `String` so non-standard provider values do not fail parsing.
	Object            string         `json:"object"`
	Created           uint64         `json:"created"`
	Model             string         `json:"model"`
	Choices           []StreamChoice `json:"choices"`
	Usage             *Usage         `json:"usage,omitempty"`
	SystemFingerprint *string        `json:"system_fingerprint,omitempty"`
	ServiceTier       *string        `json:"service_tier,omitempty"`
}

// StreamChoice is a type.
type StreamChoice struct {
	Index        uint32        `json:"index"`
	Delta        StreamDelta   `json:"delta"`
	FinishReason *FinishReason `json:"finish_reason,omitempty"`
}

// StreamDelta is a type.
type StreamDelta struct {
	Role      *string           `json:"role,omitempty"`
	Content   *string           `json:"content,omitempty"`
	ToolCalls *[]StreamToolCall `json:"tool_calls,omitempty"`
	// Deprecated legacy function_call delta; retained for API compatibility.
	FunctionCall *StreamFunctionCall `json:"function_call,omitempty"`
	Refusal      *string             `json:"refusal,omitempty"`
}

// StreamToolCall is a type.
type StreamToolCall struct {
	Index    uint32              `json:"index"`
	Id       *string             `json:"id,omitempty"`
	CallType *ToolType           `json:"call_type,omitempty"`
	Function *StreamFunctionCall `json:"function,omitempty"`
}

// StreamFunctionCall is a type.
type StreamFunctionCall struct {
	Name      *string `json:"name,omitempty"`
	Arguments *string `json:"arguments,omitempty"`
}

// AssistantMessage is a type.
type AssistantMessage struct {
	Content   *string     `json:"content,omitempty"`
	Name      *string     `json:"name,omitempty"`
	ToolCalls *[]ToolCall `json:"tool_calls,omitempty"`
	Refusal   *string     `json:"refusal,omitempty"`
	// Deprecated legacy function_call field; retained for API compatibility.
	FunctionCall *FunctionCall `json:"function_call,omitempty"`
}

// ChatCompletionTool is a type.
type ChatCompletionTool struct {
	ToolType ToolType           `json:"tool_type"`
	Function FunctionDefinition `json:"function"`
}

// FunctionDefinition is a type.
type FunctionDefinition struct {
	Name        string                  `json:"name"`
	Description *string                 `json:"description,omitempty"`
	Parameters  *map[string]interface{} `json:"parameters,omitempty"`
	Strict      *bool                   `json:"strict,omitempty"`
}

// ToolCall is a type.
type ToolCall struct {
	Id       string       `json:"id"`
	CallType ToolType     `json:"call_type"`
	Function FunctionCall `json:"function"`
}

// FunctionCall is a type.
type FunctionCall struct {
	Name      string `json:"name"`
	Arguments string `json:"arguments"`
}

// Usage is a type.
type Usage struct {
	// Prompt tokens used. Defaults to 0 when absent (some providers omit this).
	PromptTokens uint64 `json:"prompt_tokens"`
	// Completion tokens used. Defaults to 0 when absent (e.g. embedding responses).
	CompletionTokens uint64 `json:"completion_tokens"`
	// Total tokens used. Defaults to 0 when absent (some providers omit this).
	TotalTokens uint64 `json:"total_tokens"`
}

// EmbeddingRequest is a type.
type EmbeddingRequest struct {
	Model          string           `json:"model"`
	Input          EmbeddingInput   `json:"input"`
	EncodingFormat *EmbeddingFormat `json:"encoding_format,omitempty"`
	Dimensions     *uint32          `json:"dimensions,omitempty"`
	User           *string          `json:"user,omitempty"`
}

// EmbeddingResponse is a type.
type EmbeddingResponse struct {
	// Always `"list"` from OpenAI-compatible APIs.  Stored as a plain
	// `String` so non-standard provider values do not break deserialization.
	Object string            `json:"object"`
	Data   []EmbeddingObject `json:"data"`
	Model  string            `json:"model"`
	Usage  *Usage            `json:"usage,omitempty"`
}

// EmbeddingObject is a type.
type EmbeddingObject struct {
	// Always `"embedding"` from OpenAI-compatible APIs.  Stored as a plain
	// `String` so non-standard provider values do not break deserialization.
	Object    string    `json:"object"`
	Embedding []float64 `json:"embedding"`
	Index     uint32    `json:"index"`
}

// Request to create images from a text prompt.
type CreateImageRequest struct {
	Prompt         string  `json:"prompt"`
	Model          *string `json:"model,omitempty"`
	N              *uint32 `json:"n,omitempty"`
	Size           *string `json:"size,omitempty"`
	Quality        *string `json:"quality,omitempty"`
	Style          *string `json:"style,omitempty"`
	ResponseFormat *string `json:"response_format,omitempty"`
	User           *string `json:"user,omitempty"`
}

// Response containing generated images.
type ImagesResponse struct {
	Created uint64  `json:"created"`
	Data    []Image `json:"data"`
}

// A single generated image, returned as either a URL or base64 data.
type Image struct {
	Url           *string `json:"url,omitempty"`
	B64Json       *string `json:"b64_json,omitempty"`
	RevisedPrompt *string `json:"revised_prompt,omitempty"`
}

// ModelsListResponse is a type.
type ModelsListResponse struct {
	// Always `"list"` from OpenAI-compatible APIs.  Stored as a plain
	// `String` so non-standard provider values do not break deserialization.
	Object string        `json:"object"`
	Data   []ModelObject `json:"data"`
}

// ModelObject is a type.
type ModelObject struct {
	Id string `json:"id"`
	// Always `"model"` from OpenAI-compatible APIs.  Stored as a plain
	// `String` so non-standard provider values do not break deserialization.
	Object  string `json:"object"`
	Created uint64 `json:"created"`
	OwnedBy string `json:"owned_by"`
}

// Request to classify content for policy violations.
type ModerationRequest struct {
	Input ModerationInput `json:"input"`
	Model *string         `json:"model,omitempty"`
}

// Response from the moderation endpoint.
type ModerationResponse struct {
	Id      string             `json:"id"`
	Model   string             `json:"model"`
	Results []ModerationResult `json:"results"`
}

// A single moderation classification result.
type ModerationResult struct {
	Flagged        bool                     `json:"flagged"`
	Categories     ModerationCategories     `json:"categories"`
	CategoryScores ModerationCategoryScores `json:"category_scores"`
}

// Boolean flags for each moderation category.
type ModerationCategories struct {
	Sexual                bool `json:"sexual"`
	Hate                  bool `json:"hate"`
	Harassment            bool `json:"harassment"`
	SelfHarm              bool `json:"self_harm"`
	SexualMinors          bool `json:"sexual_minors"`
	HateThreatening       bool `json:"hate_threatening"`
	ViolenceGraphic       bool `json:"violence_graphic"`
	SelfHarmIntent        bool `json:"self_harm_intent"`
	SelfHarmInstructions  bool `json:"self_harm_instructions"`
	HarassmentThreatening bool `json:"harassment_threatening"`
	Violence              bool `json:"violence"`
}

// Confidence scores for each moderation category.
type ModerationCategoryScores struct {
	Sexual                float64 `json:"sexual"`
	Hate                  float64 `json:"hate"`
	Harassment            float64 `json:"harassment"`
	SelfHarm              float64 `json:"self_harm"`
	SexualMinors          float64 `json:"sexual_minors"`
	HateThreatening       float64 `json:"hate_threatening"`
	ViolenceGraphic       float64 `json:"violence_graphic"`
	SelfHarmIntent        float64 `json:"self_harm_intent"`
	SelfHarmInstructions  float64 `json:"self_harm_instructions"`
	HarassmentThreatening float64 `json:"harassment_threatening"`
	Violence              float64 `json:"violence"`
}

// Request to rerank documents by relevance to a query.
type RerankRequest struct {
	Model           string           `json:"model"`
	Query           string           `json:"query"`
	Documents       []RerankDocument `json:"documents"`
	TopN            *uint32          `json:"top_n,omitempty"`
	ReturnDocuments *bool            `json:"return_documents,omitempty"`
}

// Response from the rerank endpoint.
type RerankResponse struct {
	Id      *string                 `json:"id,omitempty"`
	Results []RerankResult          `json:"results"`
	Meta    *map[string]interface{} `json:"meta,omitempty"`
}

// A single reranked document with its relevance score.
type RerankResult struct {
	Index          uint32                `json:"index"`
	RelevanceScore float64               `json:"relevance_score"`
	Document       *RerankResultDocument `json:"document,omitempty"`
}

// The text content of a reranked document, returned when `return_documents` is true.
type RerankResultDocument struct {
	Text string `json:"text"`
}

// Calculate the estimated cost of a completion given a model name and token
// counts.
//
// Returns `None` if the model is not present in the embedded pricing registry.
// Returns `Some(cost_usd)` otherwise, where the value is in US dollars.
//
// When an exact model name match is not found, progressively shorter prefixes
// are tried by stripping from the last `-` or `.` separator.  For example,
// `gpt-4-0613` will match `gpt-4` if no `gpt-4-0613` entry exists.
//
// # Example
//
// ```rust
// use liter_llm::cost;
//
// let usd = cost::completion_cost("gpt-4o", 1_000, 500).unwrap();
// // 1000 * 0.0000025 + 500 * 0.00001 = 0.0025 + 0.005 = 0.0075
// assert!((usd - 0.0075).abs() < 1e-9);
// ```
func CompletionCost(model string, prompt_tokens uint64, completion_tokens uint64) **float64 {
	cModel := C.CString(model)
	defer C.free(unsafe.Pointer(cModel))

	ptr := C.liter_llm_completion_cost(cModel, cPromptTokens, cCompletionTokens)
	return unmarshalF64(ptr)
}

// Estimate the cost of this response based on embedded pricing data.
//
// Returns `None` if:
// - the `model` field is not present in the embedded pricing registry, or
// - the `usage` field is absent from the response.
//
// # Example
//
// ```rust,ignore
// let cost = response.estimated_cost();
// if let Some(usd) = cost {
// println!("Request cost: ${usd:.6}");
// }
// ```
func (r *ChatCompletionResponse) EstimatedCost() **float64 {
	ptr := C.liter_llm_chat_completion_response_estimated_cost(unsafe.Pointer(r))
	return unmarshalF64(ptr)
}

// Estimate the cost of this embedding request based on embedded pricing data.
//
// Returns `None` if:
// - the `model` field is not present in the embedded pricing registry, or
// - the `usage` field is absent from the response.
//
// Embedding models only charge for input tokens; output cost is zero.
//
// # Example
//
// ```rust,ignore
// let cost = response.estimated_cost();
// if let Some(usd) = cost {
// println!("Embedding cost: ${usd:.8}");
// }
// ```
func (r *EmbeddingResponse) EstimatedCost() **float64 {
	ptr := C.liter_llm_embedding_response_estimated_cost(unsafe.Pointer(r))
	return unmarshalF64(ptr)
}
