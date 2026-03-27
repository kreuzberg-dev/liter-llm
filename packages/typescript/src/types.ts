// ─── Common / Shared ─────────────────────────────────────────────────────────

export interface SystemMessage {
	role: "system";
	content: string;
	name?: string;
}

export interface TextContentPart {
	type: "text";
	text: string;
}

export interface ImageUrl {
	url: string;
	detail?: "low" | "high" | "auto";
}

export interface ImageUrlContentPart {
	type: "image_url";
	imageUrl: ImageUrl;
}

export type ContentPart = TextContentPart | ImageUrlContentPart;

export interface UserMessage {
	role: "user";
	content: string | ContentPart[];
	name?: string;
}

export interface FunctionCall {
	name: string;
	arguments: string;
}

export interface ToolCall {
	id: string;
	type: "function";
	function: FunctionCall;
}

export interface AssistantMessage {
	role: "assistant";
	content?: string;
	name?: string;
	toolCalls?: ToolCall[];
	refusal?: string;
	/** @deprecated Legacy function_call field; use toolCalls instead. */
	functionCall?: FunctionCall;
}

export interface ToolMessage {
	role: "tool";
	content: string;
	toolCallId: string;
	name?: string;
}

export interface DeveloperMessage {
	role: "developer";
	content: string;
	name?: string;
}

/** @deprecated Legacy function-role message; use ToolMessage instead. */
export interface FunctionMessage {
	role: "function";
	content: string;
	name: string;
}

export type Message = SystemMessage | UserMessage | AssistantMessage | ToolMessage | DeveloperMessage | FunctionMessage;

// ─── Tools ───────────────────────────────────────────────────────────────────

export interface FunctionDefinition {
	name: string;
	description?: string;
	/** JSON Schema object describing the function parameters. */
	parameters?: Record<string, unknown>;
	strict?: boolean;
}

export interface ChatCompletionTool {
	type: "function";
	function: FunctionDefinition;
}

export type ToolChoiceMode = "auto" | "required" | "none";

export interface SpecificFunction {
	name: string;
}

export interface SpecificToolChoice {
	type: "function";
	function: SpecificFunction;
}

export type ToolChoice = ToolChoiceMode | SpecificToolChoice;

// ─── Response Format ─────────────────────────────────────────────────────────

export interface ResponseFormatText {
	type: "text";
}

export interface ResponseFormatJsonObject {
	type: "json_object";
}

export interface JsonSchemaFormat {
	name: string;
	description?: string;
	/** JSON Schema object. */
	schema: Record<string, unknown>;
	strict?: boolean;
}

export interface ResponseFormatJsonSchema {
	type: "json_schema";
	jsonSchema: JsonSchemaFormat;
}

export type ResponseFormat = ResponseFormatText | ResponseFormatJsonObject | ResponseFormatJsonSchema;

// ─── Usage ───────────────────────────────────────────────────────────────────

export interface Usage {
	promptTokens: number;
	completionTokens: number;
	totalTokens: number;
}

// ─── Chat Request ─────────────────────────────────────────────────────────────

export interface StreamOptions {
	includeUsage?: boolean;
}

export interface ChatCompletionRequest {
	model: string;
	messages: Message[];
	temperature?: number;
	top_p?: number;
	n?: number;
	stream?: boolean;
	/** Single stop string or array of stop strings. */
	stop?: string | string[];
	max_tokens?: number;
	presence_penalty?: number;
	frequency_penalty?: number;
	logit_bias?: Record<string, number>;
	user?: string;
	tools?: ChatCompletionTool[];
	tool_choice?: ToolChoice;
	parallel_tool_calls?: boolean;
	response_format?: ResponseFormat;
	stream_options?: StreamOptions;
	seed?: number;
}

// ─── Chat Response ────────────────────────────────────────────────────────────

export type FinishReason = "stop" | "length" | "tool_calls" | "content_filter" | "function_call" | "other";

export interface Choice {
	index: number;
	message: AssistantMessage;
	finishReason: FinishReason | null;
}

export interface ChatCompletionResponse {
	id: string;
	/** Always `"chat.completion"` from OpenAI-compatible APIs. */
	object: string;
	created: number;
	model: string;
	choices: Choice[];
	usage?: Usage;
	systemFingerprint?: string;
	serviceTier?: string;
}

// ─── Streaming Types ─────────────────────────────────────────────────────────

export interface StreamFunctionCall {
	name?: string;
	arguments?: string;
}

export interface StreamToolCall {
	index: number;
	id?: string;
	type?: "function";
	function?: StreamFunctionCall;
}

export interface StreamDelta {
	role?: string;
	content?: string;
	toolCalls?: StreamToolCall[];
	/** @deprecated Legacy function_call delta; use toolCalls instead. */
	functionCall?: StreamFunctionCall;
	refusal?: string;
}

export interface StreamChoice {
	index: number;
	delta: StreamDelta;
	finishReason: FinishReason | null;
}

export interface ChatCompletionChunk {
	id: string;
	/** Always `"chat.completion.chunk"` from OpenAI-compatible APIs. */
	object: string;
	created: number;
	model: string;
	choices: StreamChoice[];
	usage?: Usage;
	serviceTier?: string;
}

// ─── Embedding Types ─────────────────────────────────────────────────────────

export interface EmbeddingRequest {
	model: string;
	/** Single string or array of strings to embed. */
	input: string | string[];
	encodingFormat?: string;
	dimensions?: number;
	user?: string;
}

export interface EmbeddingObject {
	/** Always `"embedding"` from OpenAI-compatible APIs. */
	object: string;
	embedding: number[];
	index: number;
}

export interface EmbeddingResponse {
	/** Always `"list"` from OpenAI-compatible APIs. */
	object: string;
	data: EmbeddingObject[];
	model: string;
	usage: Usage;
}

// ─── Models Types ─────────────────────────────────────────────────────────────

export interface ModelObject {
	id: string;
	/** Always `"model"` from OpenAI-compatible APIs. */
	object: string;
	created: number;
	ownedBy: string;
}

export interface ModelsListResponse {
	/** Always `"list"` from OpenAI-compatible APIs. */
	object: string;
	data: ModelObject[];
}

// ─── Cache / Budget / Hooks / Provider ────────────────────────────────────────

/** Configuration for the response cache. */
export interface CacheOptions {
	/** Maximum number of cached entries (default: 256). */
	maxEntries?: number;
	/** Time-to-live for cached entries in seconds (default: 300). */
	ttlSeconds?: number;
}

/** Configuration for budget enforcement. */
export interface BudgetOptions {
	/** Maximum total spend across all models in USD. */
	globalLimit?: number;
	/** Per-model spending limits in USD, keyed by model name. */
	modelLimits?: Record<string, number>;
	/** Enforcement mode: `"soft"` (warn only) or `"hard"` (reject). Default: `"hard"`. */
	enforcement?: "soft" | "hard";
}

/** Hook object with optional lifecycle callbacks. */
export interface LlmHook {
	/** Called before the request is sent. Throw to reject (guardrail). */
	onRequest?(request: Record<string, unknown>): void | Promise<void>;
	/** Called after a successful response. */
	onResponse?(request: Record<string, unknown>, response: Record<string, unknown>): void | Promise<void>;
	/** Called when the request fails with an error. */
	onError?(request: Record<string, unknown>, error: Error): void | Promise<void>;
}

/** Configuration for registering a custom LLM provider at runtime. */
export interface CustomProviderOptions {
	/** Unique name for this provider. */
	name: string;
	/** Base URL for the provider's API. */
	baseUrl: string;
	/** Authentication style: `"bearer"`, `"none"`, or a custom header name
	 *  (e.g. `"X-Api-Key"` sends the API key via that header). */
	authHeader: string;
	/** Model name prefixes that route to this provider. */
	modelPrefixes: string[];
}

// ─── Client Options ───────────────────────────────────────────────────────────

export interface LlmClientOptions {
	apiKey: string;
	baseUrl?: string;
	/** Model hint for provider auto-detection (e.g. `"groq/llama3-70b"` selects the Groq provider). */
	modelHint?: string;
	maxRetries?: number;
	/** Timeout in seconds. */
	timeoutSecs?: number;
	/** Response cache configuration. */
	cache?: CacheOptions;
	/** Budget enforcement configuration. */
	budget?: BudgetOptions;
	/** Extra headers sent on every request. */
	extraHeaders?: Record<string, string>;
}
