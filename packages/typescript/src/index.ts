// liter-llm TypeScript bindings
// Runtime class re-exported from the native NAPI-RS module.
export { LlmClient } from "@kreuzberg/liter-llm-native";

export type {
	// Messages
	SystemMessage,
	UserMessage,
	AssistantMessage,
	ToolMessage,
	DeveloperMessage,
	FunctionMessage,
	Message,
	// Content parts
	TextContentPart,
	ImageUrl,
	ImageUrlContentPart,
	ContentPart,
	// Tools
	FunctionDefinition,
	FunctionCall,
	ToolCall,
	ChatCompletionTool,
	ToolChoiceMode,
	SpecificFunction,
	SpecificToolChoice,
	ToolChoice,
	// Response format
	ResponseFormatText,
	ResponseFormatJsonObject,
	JsonSchemaFormat,
	ResponseFormatJsonSchema,
	ResponseFormat,
	// Usage
	Usage,
	// Chat request / response
	StreamOptions,
	ChatCompletionRequest,
	FinishReason,
	Choice,
	ChatCompletionResponse,
	// Streaming
	StreamFunctionCall,
	StreamToolCall,
	StreamDelta,
	StreamChoice,
	ChatCompletionChunk,
	// Embeddings
	EmbeddingRequest,
	EmbeddingObject,
	EmbeddingResponse,
	// Models
	ModelObject,
	ModelsListResponse,
	// Client
	LlmClientOptions,
} from "./types.js";
