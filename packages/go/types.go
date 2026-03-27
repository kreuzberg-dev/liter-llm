// Package literllm provides a Go client for the liter-llm universal LLM API.
//
// It speaks the OpenAI-compatible HTTP API directly — no cgo, no CGO, no
// shared libraries required.  The same wire format is used for all
// 142+ providers supported by liter-llm; the model-name prefix
// (e.g. "groq/llama3-70b") selects the provider and endpoint.
package literllm

import (
	"context"
	"encoding/json"
)

// ─── Cache / Budget / Hook / Provider Config ─────────────────────────────────

// CacheConfig configures response caching behavior for the client.
type CacheConfig struct {
	// MaxEntries is the maximum number of cached responses to retain.
	MaxEntries int `json:"max_entries"`
	// TTLSeconds is the time-to-live for each cache entry in seconds.
	TTLSeconds int `json:"ttl_seconds"`
}

// BudgetConfig configures cost budget enforcement for the client.
type BudgetConfig struct {
	// GlobalLimit is the maximum total cost allowed across all models.
	// A nil value means no global limit.
	GlobalLimit *float64 `json:"global_limit,omitempty"`
	// ModelLimits maps model names to per-model cost limits.
	ModelLimits map[string]float64 `json:"model_limits,omitempty"`
	// Enforcement controls budget enforcement behavior: "strict" (reject
	// requests that would exceed budget) or "warn" (log a warning).
	Enforcement string `json:"enforcement"`
}

// ProviderConfig defines a custom LLM provider that is not in the built-in
// registry.  Register it with [Client.RegisterProvider].
type ProviderConfig struct {
	// Name is the unique provider name used for model routing.
	Name string `json:"name"`
	// BaseURL is the provider's API base URL.
	BaseURL string `json:"base_url"`
	// AuthHeader is the header name used for authentication (e.g. "Authorization").
	AuthHeader string `json:"auth_header"`
	// ModelPrefixes lists the model name prefixes that route to this provider.
	ModelPrefixes []string `json:"model_prefixes"`
}

// Hook is the interface for intercepting client request/response lifecycle
// events.  Implement any subset of methods; all are called synchronously in
// the order hooks were added.
type Hook interface {
	// OnRequest is called before the HTTP request is sent.  Return a non-nil
	// error to abort the request (the error is returned to the caller).
	OnRequest(ctx context.Context, req interface{}) error
	// OnResponse is called after a successful response is received.
	OnResponse(ctx context.Context, req, resp interface{})
	// OnError is called after a request fails.
	OnError(ctx context.Context, req interface{}, err error)
}

// mustMarshal is a helper for json.Marshal calls that cannot fail
// (e.g. marshaling a string or a slice of known-good types).
func mustMarshal(v any) json.RawMessage {
	raw, err := json.Marshal(v)
	if err != nil {
		panic("literllm: unexpected marshal error: " + err.Error())
	}
	return raw
}

// ─── Messages ─────────────────────────────────────────────────────────────────

// Role enumerates the valid values for Message.Role.
type Role string

const (
	RoleSystem    Role = "system"
	RoleUser      Role = "user"
	RoleAssistant Role = "assistant"
	RoleTool      Role = "tool"
	RoleDeveloper Role = "developer"
	// RoleFunction is the deprecated legacy function role, retained for API
	// compatibility with older OpenAI SDK responses.
	RoleFunction Role = "function"
)

// Message is a single turn in a chat conversation.
//
// The Content field is intentionally typed as json.RawMessage so that callers
// can supply either a plain string or an array of ContentPart objects without
// losing type safety.  Use [NewTextMessage], [NewToolResultMessage], and
// [NewAssistantMessage] for the common cases.
type Message struct {
	Role       Role            `json:"role"`
	Content    json.RawMessage `json:"content,omitempty"`
	Name       *string         `json:"name,omitempty"`
	ToolCallID *string         `json:"tool_call_id,omitempty"`
	ToolCalls  []ToolCall      `json:"tool_calls,omitempty"`
	// Refusal is set by the assistant when it declines to respond.
	Refusal *string `json:"refusal,omitempty"`
	// FunctionCall is the deprecated legacy field; retained for compatibility.
	FunctionCall *FunctionCall `json:"function_call,omitempty"`
}

// NewTextMessage creates a simple text message with the given role and content.
func NewTextMessage(role Role, text string) Message {
	raw := mustMarshal(text)
	return Message{Role: role, Content: raw}
}

// NewPartsMessage creates a message whose content is a slice of ContentPart.
func NewPartsMessage(role Role, parts []ContentPart) Message {
	raw := mustMarshal(parts)
	return Message{Role: role, Content: raw}
}

// ContentPartType enumerates allowed content part types.
type ContentPartType string

const (
	ContentPartTypeText     ContentPartType = "text"
	ContentPartTypeImageURL ContentPartType = "image_url"
)

// ContentPart is one element in a multi-modal message content array.
type ContentPart struct {
	Type     ContentPartType `json:"type"`
	Text     string          `json:"text,omitempty"`
	ImageURL *ImageURL       `json:"image_url,omitempty"`
}

// ImageURL specifies an image to include in a message.
type ImageURL struct {
	URL    string       `json:"url"`
	Detail *ImageDetail `json:"detail,omitempty"`
}

// ImageDetail controls the resolution at which the model processes the image.
type ImageDetail string

const (
	ImageDetailLow  ImageDetail = "low"
	ImageDetailHigh ImageDetail = "high"
	ImageDetailAuto ImageDetail = "auto"
)

// ─── Tools ────────────────────────────────────────────────────────────────────

// ToolType is the only valid value for tool and tool-call type fields.
// The OpenAI spec always uses "function"; other values are rejected.
type ToolType string

const ToolTypeFunction ToolType = "function"

// ChatCompletionTool describes a function the model may call.
type ChatCompletionTool struct {
	Type     ToolType           `json:"type"`
	Function FunctionDefinition `json:"function"`
}

// FunctionDefinition holds the schema for a callable function.
type FunctionDefinition struct {
	Name        string  `json:"name"`
	Description *string `json:"description,omitempty"`
	// Parameters is a JSON Schema object.  Use json.RawMessage to avoid
	// double-encoding.
	Parameters json.RawMessage `json:"parameters,omitempty"`
	Strict     *bool           `json:"strict,omitempty"`
}

// ToolCall is a request from the model to invoke a tool.
type ToolCall struct {
	ID       string       `json:"id"`
	Type     ToolType     `json:"type"`
	Function FunctionCall `json:"function"`
}

// FunctionCall carries the name and JSON-encoded arguments for a tool call.
type FunctionCall struct {
	Name      string `json:"name"`
	Arguments string `json:"arguments"`
}

// ─── Tool Choice ─────────────────────────────────────────────────────────────

// ToolChoice controls how (and whether) the model calls tools.
// Use the ToolChoiceAuto, ToolChoiceRequired, and ToolChoiceNone sentinel
// values, or construct a ToolChoice with a specific function via
// [NewSpecificToolChoice].
type ToolChoice struct {
	raw json.RawMessage
}

// MarshalJSON implements json.Marshaler.
func (tc ToolChoice) MarshalJSON() ([]byte, error) {
	if tc.raw == nil {
		return []byte(`"auto"`), nil
	}
	return tc.raw, nil
}

// UnmarshalJSON implements json.Unmarshaler.
func (tc *ToolChoice) UnmarshalJSON(data []byte) error {
	tc.raw = make(json.RawMessage, len(data))
	copy(tc.raw, data)
	return nil
}

var (
	// ToolChoiceAuto lets the model decide whether to call a tool.
	ToolChoiceAuto = ToolChoice{raw: json.RawMessage(`"auto"`)}
	// ToolChoiceRequired forces the model to call a tool.
	ToolChoiceRequired = ToolChoice{raw: json.RawMessage(`"required"`)}
	// ToolChoiceNone prevents the model from calling any tool.
	ToolChoiceNone = ToolChoice{raw: json.RawMessage(`"none"`)}
)

// specificToolChoice is the JSON shape for targeting a named function.
type specificToolChoice struct {
	Type     ToolType         `json:"type"`
	Function specificFunction `json:"function"`
}

type specificFunction struct {
	Name string `json:"name"`
}

// NewSpecificToolChoice returns a ToolChoice that forces the model to call the
// named function.
func NewSpecificToolChoice(functionName string) ToolChoice {
	raw := mustMarshal(specificToolChoice{
		Type:     ToolTypeFunction,
		Function: specificFunction{Name: functionName},
	})
	return ToolChoice{raw: raw}
}

// ─── Response Format ──────────────────────────────────────────────────────────

// ResponseFormatType enumerates the supported response format types.
type ResponseFormatType string

const (
	ResponseFormatTypeText       ResponseFormatType = "text"
	ResponseFormatTypeJSONObject ResponseFormatType = "json_object"
	ResponseFormatTypeJSONSchema ResponseFormatType = "json_schema"
)

// ResponseFormat instructs the model to produce output in a specific format.
type ResponseFormat struct {
	Type       ResponseFormatType `json:"type"`
	JSONSchema *JSONSchemaFormat  `json:"json_schema,omitempty"`
}

// JSONSchemaFormat is the schema descriptor used with ResponseFormatTypeJSONSchema.
type JSONSchemaFormat struct {
	Name        string          `json:"name"`
	Description *string         `json:"description,omitempty"`
	Schema      json.RawMessage `json:"schema"`
	Strict      *bool           `json:"strict,omitempty"`
}

// ─── Usage ────────────────────────────────────────────────────────────────────

// Usage reports token consumption for a request.
type Usage struct {
	PromptTokens     uint64 `json:"prompt_tokens"`
	CompletionTokens uint64 `json:"completion_tokens"`
	TotalTokens      uint64 `json:"total_tokens"`
}

// ─── Stop Sequence ────────────────────────────────────────────────────────────

// StopSequence is either a single stop string or a list of stop strings.
// Use [NewStopString] and [NewStopStrings] for construction.
type StopSequence struct {
	raw json.RawMessage
}

// MarshalJSON implements json.Marshaler.
func (s StopSequence) MarshalJSON() ([]byte, error) {
	return s.raw, nil
}

// UnmarshalJSON implements json.Unmarshaler.
func (s *StopSequence) UnmarshalJSON(data []byte) error {
	s.raw = make(json.RawMessage, len(data))
	copy(s.raw, data)
	return nil
}

// NewStopString creates a StopSequence from a single stop string.
func NewStopString(stop string) StopSequence {
	raw := mustMarshal(stop)
	return StopSequence{raw: raw}
}

// NewStopStrings creates a StopSequence from multiple stop strings.
func NewStopStrings(stops []string) StopSequence {
	raw := mustMarshal(stops)
	return StopSequence{raw: raw}
}

// ─── Chat Request ─────────────────────────────────────────────────────────────

// StreamOptions configures behavior for streaming requests.
type StreamOptions struct {
	// IncludeUsage, when true, requests that the final chunk include usage stats.
	IncludeUsage *bool `json:"include_usage,omitempty"`
}

// ChatCompletionRequest is the body for a chat completion API call.
//
// Only Model and Messages are required.  All other fields are optional and are
// omitted from the JSON payload when nil/zero.
type ChatCompletionRequest struct {
	Model             string               `json:"model"`
	Messages          []Message            `json:"messages"`
	Temperature       *float64             `json:"temperature,omitempty"`
	TopP              *float64             `json:"top_p,omitempty"`
	N                 *uint32              `json:"n,omitempty"`
	Stream            *bool                `json:"stream,omitempty"`
	Stop              *StopSequence        `json:"stop,omitempty"`
	MaxTokens         *uint64              `json:"max_tokens,omitempty"`
	PresencePenalty   *float64             `json:"presence_penalty,omitempty"`
	FrequencyPenalty  *float64             `json:"frequency_penalty,omitempty"`
	LogitBias         map[string]float64   `json:"logit_bias,omitempty"`
	User              *string              `json:"user,omitempty"`
	Tools             []ChatCompletionTool `json:"tools,omitempty"`
	ToolChoice        *ToolChoice          `json:"tool_choice,omitempty"`
	ParallelToolCalls *bool                `json:"parallel_tool_calls,omitempty"`
	ResponseFormat    *ResponseFormat      `json:"response_format,omitempty"`
	StreamOptions     *StreamOptions       `json:"stream_options,omitempty"`
	Seed              *int64               `json:"seed,omitempty"`
}

// ─── Chat Response ────────────────────────────────────────────────────────────

// FinishReason indicates why a choice stopped generating tokens.
type FinishReason string

const (
	FinishReasonStop          FinishReason = "stop"
	FinishReasonLength        FinishReason = "length"
	FinishReasonToolCalls     FinishReason = "tool_calls"
	FinishReasonContentFilter FinishReason = "content_filter"
	// FinishReasonFunctionCall is the deprecated legacy value retained for
	// API compatibility.
	FinishReasonFunctionCall FinishReason = "function_call"
)

// AssistantMessage holds the content returned by the model in a non-streaming
// response.
type AssistantMessage struct {
	Content      *string       `json:"content,omitempty"`
	Name         *string       `json:"name,omitempty"`
	ToolCalls    []ToolCall    `json:"tool_calls,omitempty"`
	Refusal      *string       `json:"refusal,omitempty"`
	FunctionCall *FunctionCall `json:"function_call,omitempty"`
}

// Choice is one completion alternative in a ChatCompletionResponse.
type Choice struct {
	Index        uint32           `json:"index"`
	Message      AssistantMessage `json:"message"`
	FinishReason *FinishReason    `json:"finish_reason"`
}

// ChatCompletionResponse is the response body for a non-streaming chat
// completion request.
type ChatCompletionResponse struct {
	ID                string   `json:"id"`
	Object            string   `json:"object"`
	Created           uint64   `json:"created"`
	Model             string   `json:"model"`
	Choices           []Choice `json:"choices"`
	Usage             *Usage   `json:"usage,omitempty"`
	SystemFingerprint *string  `json:"system_fingerprint,omitempty"`
	ServiceTier       *string  `json:"service_tier,omitempty"`
}

// ─── Stream Chunk ─────────────────────────────────────────────────────────────

// StreamFunctionCall is the delta shape for a deprecated legacy function call.
type StreamFunctionCall struct {
	Name      *string `json:"name,omitempty"`
	Arguments *string `json:"arguments,omitempty"`
}

// StreamToolCall is an incremental update to a tool call within a chunk.
type StreamToolCall struct {
	Index    uint32              `json:"index"`
	ID       *string             `json:"id,omitempty"`
	Type     *ToolType           `json:"type,omitempty"`
	Function *StreamFunctionCall `json:"function,omitempty"`
}

// StreamDelta contains the incremental content for one stream choice.
type StreamDelta struct {
	Role         *string             `json:"role,omitempty"`
	Content      *string             `json:"content,omitempty"`
	ToolCalls    []StreamToolCall    `json:"tool_calls,omitempty"`
	FunctionCall *StreamFunctionCall `json:"function_call,omitempty"`
	Refusal      *string             `json:"refusal,omitempty"`
}

// StreamChoice is one choice entry within a ChatCompletionChunk.
type StreamChoice struct {
	Index        uint32        `json:"index"`
	Delta        StreamDelta   `json:"delta"`
	FinishReason *FinishReason `json:"finish_reason"`
}

// ChatCompletionChunk is a single server-sent event emitted during a streaming
// chat completion.
type ChatCompletionChunk struct {
	ID          string         `json:"id"`
	Object      string         `json:"object"`
	Created     uint64         `json:"created"`
	Model       string         `json:"model"`
	Choices     []StreamChoice `json:"choices"`
	Usage       *Usage         `json:"usage,omitempty"`
	ServiceTier *string        `json:"service_tier,omitempty"`
}

// ─── Embedding ────────────────────────────────────────────────────────────────

// EmbeddingInput is either a single string or a list of strings.
// Use [NewEmbeddingInputSingle] and [NewEmbeddingInputMultiple].
type EmbeddingInput struct {
	raw json.RawMessage
}

// MarshalJSON implements json.Marshaler.
func (e EmbeddingInput) MarshalJSON() ([]byte, error) {
	return e.raw, nil
}

// UnmarshalJSON implements json.Unmarshaler.
func (e *EmbeddingInput) UnmarshalJSON(data []byte) error {
	e.raw = make(json.RawMessage, len(data))
	copy(e.raw, data)
	return nil
}

// NewEmbeddingInputSingle wraps a single string as an EmbeddingInput.
func NewEmbeddingInputSingle(text string) EmbeddingInput {
	raw := mustMarshal(text)
	return EmbeddingInput{raw: raw}
}

// NewEmbeddingInputMultiple wraps multiple strings as an EmbeddingInput.
func NewEmbeddingInputMultiple(texts []string) EmbeddingInput {
	raw := mustMarshal(texts)
	return EmbeddingInput{raw: raw}
}

// EmbeddingRequest is the body for an embedding API call.
type EmbeddingRequest struct {
	Model          string         `json:"model"`
	Input          EmbeddingInput `json:"input"`
	EncodingFormat *string        `json:"encoding_format,omitempty"`
	Dimensions     *uint32        `json:"dimensions,omitempty"`
	User           *string        `json:"user,omitempty"`
}

// EmbeddingObject holds a single embedding vector.
type EmbeddingObject struct {
	Object    string    `json:"object"`
	Embedding []float64 `json:"embedding"`
	Index     uint32    `json:"index"`
}

// EmbeddingResponse is the response body for an embedding request.
type EmbeddingResponse struct {
	Object string            `json:"object"`
	Data   []EmbeddingObject `json:"data"`
	Model  string            `json:"model"`
	Usage  Usage             `json:"usage"`
}

// ─── Models ───────────────────────────────────────────────────────────────────

// ModelObject describes a single model entry returned by the list-models API.
type ModelObject struct {
	ID      string `json:"id"`
	Object  string `json:"object"`
	Created uint64 `json:"created"`
	OwnedBy string `json:"owned_by"`
}

// ModelsListResponse is the response body for the list-models API.
type ModelsListResponse struct {
	Object string        `json:"object"`
	Data   []ModelObject `json:"data"`
}

// ─── Image Generation ─────────────────────────────────────────────────────────

// CreateImageRequest is the body for an image generation API call.
type CreateImageRequest struct {
	Prompt         string  `json:"prompt"`
	Model          *string `json:"model,omitempty"`
	N              *uint32 `json:"n,omitempty"`
	Quality        *string `json:"quality,omitempty"`
	ResponseFormat *string `json:"response_format,omitempty"`
	Size           *string `json:"size,omitempty"`
	Style          *string `json:"style,omitempty"`
	User           *string `json:"user,omitempty"`
}

// ImageData holds a single image result.
type ImageData struct {
	URL           *string `json:"url,omitempty"`
	B64JSON       *string `json:"b64_json,omitempty"`
	RevisedPrompt *string `json:"revised_prompt,omitempty"`
}

// ImagesResponse is the response body for an image generation request.
type ImagesResponse struct {
	Created uint64      `json:"created"`
	Data    []ImageData `json:"data"`
}

// ─── Speech ───────────────────────────────────────────────────────────────────

// CreateSpeechRequest is the body for a speech generation API call.
type CreateSpeechRequest struct {
	Model          string   `json:"model"`
	Input          string   `json:"input"`
	Voice          string   `json:"voice"`
	ResponseFormat *string  `json:"response_format,omitempty"`
	Speed          *float64 `json:"speed,omitempty"`
}

// ─── Transcription ────────────────────────────────────────────────────────────

// CreateTranscriptionRequest is the body for an audio transcription API call.
// The File field must be base64-encoded audio data.
type CreateTranscriptionRequest struct {
	File           string   `json:"file"`
	Model          string   `json:"model"`
	Language       *string  `json:"language,omitempty"`
	Prompt         *string  `json:"prompt,omitempty"`
	ResponseFormat *string  `json:"response_format,omitempty"`
	Temperature    *float64 `json:"temperature,omitempty"`
}

// TranscriptionResponse is the response body for a transcription request.
type TranscriptionResponse struct {
	Text string `json:"text"`
}

// ─── Moderation ───────────────────────────────────────────────────────────────

// ModerationRequest is the body for a moderation API call.
type ModerationRequest struct {
	Input json.RawMessage `json:"input"`
	Model *string         `json:"model,omitempty"`
}

// ModerationCategoryScores holds per-category confidence scores.
type ModerationCategoryScores struct {
	Sexual           float64 `json:"sexual"`
	Hate             float64 `json:"hate"`
	Harassment       float64 `json:"harassment"`
	SelfHarm         float64 `json:"self-harm"`
	Violence         float64 `json:"violence"`
	SexualMinors     float64 `json:"sexual/minors"`
	HateThreatening  float64 `json:"hate/threatening"`
	ViolenceGraphic  float64 `json:"violence/graphic"`
	SelfHarmIntent   float64 `json:"self-harm/intent"`
	SelfHarmInstr    float64 `json:"self-harm/instructions"`
	HarassmentThreat float64 `json:"harassment/threatening"`
}

// ModerationCategories holds per-category boolean flags.
type ModerationCategories struct {
	Sexual           bool `json:"sexual"`
	Hate             bool `json:"hate"`
	Harassment       bool `json:"harassment"`
	SelfHarm         bool `json:"self-harm"`
	Violence         bool `json:"violence"`
	SexualMinors     bool `json:"sexual/minors"`
	HateThreatening  bool `json:"hate/threatening"`
	ViolenceGraphic  bool `json:"violence/graphic"`
	SelfHarmIntent   bool `json:"self-harm/intent"`
	SelfHarmInstr    bool `json:"self-harm/instructions"`
	HarassmentThreat bool `json:"harassment/threatening"`
}

// ModerationResult holds the result for a single input.
type ModerationResult struct {
	Flagged        bool                     `json:"flagged"`
	Categories     ModerationCategories     `json:"categories"`
	CategoryScores ModerationCategoryScores `json:"category_scores"`
}

// ModerationResponse is the response body for a moderation request.
type ModerationResponse struct {
	ID      string             `json:"id"`
	Model   string             `json:"model"`
	Results []ModerationResult `json:"results"`
}

// ─── Rerank ───────────────────────────────────────────────────────────────────

// RerankRequest is the body for a rerank API call.
type RerankRequest struct {
	Model     string          `json:"model"`
	Query     string          `json:"query"`
	Documents json.RawMessage `json:"documents"`
	TopN      *uint32         `json:"top_n,omitempty"`
}

// RerankResult holds a single reranked document result.
type RerankResult struct {
	Index          uint32  `json:"index"`
	RelevanceScore float64 `json:"relevance_score"`
}

// RerankResponse is the response body for a rerank request.
type RerankResponse struct {
	Results []RerankResult `json:"results"`
	Model   string         `json:"model"`
	Usage   *Usage         `json:"usage,omitempty"`
}

// ─── Files ────────────────────────────────────────────────────────────────────

// FilePurpose enumerates the valid values for file upload purpose.
type FilePurpose string

const (
	FilePurposeAssistants FilePurpose = "assistants"
	FilePurposeFineTune   FilePurpose = "fine-tune"
	FilePurposeBatch      FilePurpose = "batch"
	FilePurposeVision     FilePurpose = "vision"
	FilePurposeUserData   FilePurpose = "user_data"
	FilePurposeEvals      FilePurpose = "evals"
	FilePurposeResponses  FilePurpose = "responses"
)

// CreateFileRequest is the body for a file upload API call.
// The File field must be base64-encoded file data.
type CreateFileRequest struct {
	File     string      `json:"file"`
	Purpose  FilePurpose `json:"purpose"`
	Filename *string     `json:"filename,omitempty"`
}

// FileObject describes a file that has been uploaded.
type FileObject struct {
	ID            string `json:"id"`
	Object        string `json:"object"`
	Bytes         uint64 `json:"bytes"`
	CreatedAt     uint64 `json:"created_at"`
	Filename      string `json:"filename"`
	Purpose       string `json:"purpose"`
	Status        string `json:"status,omitempty"`
	StatusDetails string `json:"status_details,omitempty"`
}

// DeleteResponse is the response body for a delete operation.
type DeleteResponse struct {
	ID      string `json:"id"`
	Object  string `json:"object"`
	Deleted bool   `json:"deleted"`
}

// FileListQuery holds optional query parameters for listing files.
type FileListQuery struct {
	Purpose *string `json:"purpose,omitempty"`
	Limit   *uint32 `json:"limit,omitempty"`
	After   *string `json:"after,omitempty"`
}

// FileListResponse is the response body for listing files.
type FileListResponse struct {
	Object string       `json:"object"`
	Data   []FileObject `json:"data"`
}

// ─── Batches ──────────────────────────────────────────────────────────────────

// CreateBatchRequest is the body for creating a batch job.
type CreateBatchRequest struct {
	InputFileID      string            `json:"input_file_id"`
	Endpoint         string            `json:"endpoint"`
	CompletionWindow string            `json:"completion_window"`
	Metadata         map[string]string `json:"metadata,omitempty"`
}

// BatchRequestCounts holds counts for a batch job.
type BatchRequestCounts struct {
	Total     uint64 `json:"total"`
	Completed uint64 `json:"completed"`
	Failed    uint64 `json:"failed"`
}

// BatchObject describes a batch processing job.
type BatchObject struct {
	ID               string              `json:"id"`
	Object           string              `json:"object"`
	Endpoint         string              `json:"endpoint"`
	InputFileID      string              `json:"input_file_id"`
	CompletionWindow string              `json:"completion_window"`
	Status           string              `json:"status"`
	OutputFileID     *string             `json:"output_file_id,omitempty"`
	ErrorFileID      *string             `json:"error_file_id,omitempty"`
	CreatedAt        uint64              `json:"created_at"`
	InProgressAt     *uint64             `json:"in_progress_at,omitempty"`
	ExpiresAt        *uint64             `json:"expires_at,omitempty"`
	FinalizingAt     *uint64             `json:"finalizing_at,omitempty"`
	CompletedAt      *uint64             `json:"completed_at,omitempty"`
	FailedAt         *uint64             `json:"failed_at,omitempty"`
	ExpiredAt        *uint64             `json:"expired_at,omitempty"`
	CancellingAt     *uint64             `json:"cancelling_at,omitempty"` //nolint:misspell // OpenAI API field name
	CancelledAt      *uint64             `json:"cancelled_at,omitempty"`  //nolint:misspell // OpenAI API field name
	RequestCounts    *BatchRequestCounts `json:"request_counts,omitempty"`
	Metadata         map[string]string   `json:"metadata,omitempty"`
}

// BatchListQuery holds optional query parameters for listing batches.
type BatchListQuery struct {
	Limit *uint32 `json:"limit,omitempty"`
	After *string `json:"after,omitempty"`
}

// BatchListResponse is the response body for listing batches.
type BatchListResponse struct {
	Object string        `json:"object"`
	Data   []BatchObject `json:"data"`
}

// ─── Responses ────────────────────────────────────────────────────────────────

// CreateResponseRequest is the body for creating a response via the
// Responses API.
type CreateResponseRequest struct {
	Model        string            `json:"model"`
	Input        json.RawMessage   `json:"input"`
	Instructions *string           `json:"instructions,omitempty"`
	MaxTokens    *uint64           `json:"max_output_tokens,omitempty"`
	Temperature  *float64          `json:"temperature,omitempty"`
	TopP         *float64          `json:"top_p,omitempty"`
	Stream       *bool             `json:"stream,omitempty"`
	Metadata     map[string]string `json:"metadata,omitempty"`
}

// ResponseObject describes a response from the Responses API.
type ResponseObject struct {
	ID        string            `json:"id"`
	Object    string            `json:"object"`
	CreatedAt uint64            `json:"created_at"`
	Status    string            `json:"status"`
	Model     string            `json:"model"`
	Output    json.RawMessage   `json:"output,omitempty"`
	Usage     *Usage            `json:"usage,omitempty"`
	Metadata  map[string]string `json:"metadata,omitempty"`
	Error     json.RawMessage   `json:"error,omitempty"`
}
