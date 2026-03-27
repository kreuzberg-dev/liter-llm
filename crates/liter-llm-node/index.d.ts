/* auto-generated type stubs for @kreuzberg/liter-llm-native */

export interface CacheOptions {
	/** Maximum number of cached entries (default: 256). */
	maxEntries?: number;
	/** Time-to-live for cached entries in seconds (default: 300). */
	ttlSeconds?: number;
}

export interface BudgetOptions {
	/** Maximum total spend across all models in USD. */
	globalLimit?: number;
	/** Per-model spending limits in USD, keyed by model name. */
	modelLimits?: Record<string, number>;
	/** Enforcement mode: "soft" (warn only) or "hard" (reject). Default: "hard". */
	enforcement?: "soft" | "hard";
}

export interface CustomProviderOptions {
	/** Unique name for this provider. */
	name: string;
	/** Base URL for the provider's API. */
	baseUrl: string;
	/** Authentication style: "bearer", "none", or a custom header name (e.g. "X-Api-Key"). */
	authHeader: string;
	/** Model name prefixes that route to this provider. */
	modelPrefixes: string[];
}

export interface LlmClientOptions {
	apiKey: string;
	baseUrl?: string;
	modelHint?: string;
	maxRetries?: number;
	timeoutSecs?: number;
	/** Response cache configuration. */
	cache?: CacheOptions;
	/** Budget enforcement configuration. */
	budget?: BudgetOptions;
	/** Extra headers sent on every request, as key-value pairs. */
	extraHeaders?: Record<string, string>;
}

export class LlmClient {
	constructor(options: LlmClientOptions);
	chat(request: Record<string, unknown>): Promise<Record<string, unknown>>;
	chatStream(request: Record<string, unknown>): Promise<Record<string, unknown>[]>;
	embed(request: Record<string, unknown>): Promise<Record<string, unknown>>;
	listModels(): Promise<Record<string, unknown>>;
	imageGenerate(request: Record<string, unknown>): Promise<Record<string, unknown>>;
	speech(request: Record<string, unknown>): Promise<Buffer>;
	transcribe(request: Record<string, unknown>): Promise<Record<string, unknown>>;
	moderate(request: Record<string, unknown>): Promise<Record<string, unknown>>;
	rerank(request: Record<string, unknown>): Promise<Record<string, unknown>>;
	createFile(request: Record<string, unknown>): Promise<Record<string, unknown>>;
	retrieveFile(fileId: string): Promise<Record<string, unknown>>;
	deleteFile(fileId: string): Promise<Record<string, unknown>>;
	listFiles(query?: Record<string, unknown>): Promise<Record<string, unknown>>;
	fileContent(fileId: string): Promise<Buffer>;
	createBatch(request: Record<string, unknown>): Promise<Record<string, unknown>>;
	retrieveBatch(batchId: string): Promise<Record<string, unknown>>;
	listBatches(query?: Record<string, unknown>): Promise<Record<string, unknown>>;
	cancelBatch(batchId: string): Promise<Record<string, unknown>>;
	createResponse(request: Record<string, unknown>): Promise<Record<string, unknown>>;
	retrieveResponse(id: string): Promise<Record<string, unknown>>;
	cancelResponse(id: string): Promise<Record<string, unknown>>;
	/** Register a custom LLM provider at runtime. */
	static registerProvider(config: CustomProviderOptions): void;
	/** Unregister a previously registered custom provider by name. */
	static unregisterProvider(name: string): boolean;
}

export function version(): string;
