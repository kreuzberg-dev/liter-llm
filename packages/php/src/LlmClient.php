<?php

declare(strict_types=1);

namespace LiterLlm;

/**
 * PHPStan type aliases for LiterLlm JSON shapes.
 *
 * These aliases document the exact array structure that each JSON-string
 * argument / return value encodes so that static analysis can verify callers
 * without requiring PHP class instances.
 *
 * @phpstan-type ImageUrlParam array{url: string, detail?: 'low'|'high'|'auto'}
 * @phpstan-type ContentPartParam array{type: 'text', text: string}|array{type: 'image_url', image_url: ImageUrlParam}
 * @phpstan-type MessageParam array{role: 'system'|'user'|'assistant'|'tool'|'developer'|'function', content: string|list<ContentPartParam>, name?: string, tool_call_id?: string}
 * @phpstan-type FunctionDefinition array{name: string, description?: string, parameters?: array<string, mixed>, strict?: bool}
 * @phpstan-type ToolParam array{type: 'function', function: FunctionDefinition}
 * @phpstan-type SpecificToolChoice array{type: 'function', function: array{name: string}}
 * @phpstan-type ToolChoiceParam 'auto'|'required'|'none'|SpecificToolChoice
 * @phpstan-type ResponseFormatParam array{type: 'text'}|array{type: 'json_object'}|array{type: 'json_schema', json_schema: array{name: string, description?: string, schema: array<string, mixed>, strict?: bool}}
 * @phpstan-type StreamOptions array{include_usage?: bool}
 * @phpstan-type ChatCompletionRequest array{model: string, messages: list<MessageParam>, temperature?: float, top_p?: float, n?: int, stream?: bool, stop?: string|list<string>, max_tokens?: int, presence_penalty?: float, frequency_penalty?: float, logit_bias?: array<string, float>, user?: string, tools?: list<ToolParam>, tool_choice?: ToolChoiceParam, parallel_tool_calls?: bool, response_format?: ResponseFormatParam, stream_options?: StreamOptions, seed?: int}
 * @phpstan-type FunctionCall array{name: string, arguments: string}
 * @phpstan-type ToolCall array{id: string, type: 'function', function: FunctionCall}
 * @phpstan-type AssistantMessage array{content?: string|null, name?: string, tool_calls?: list<ToolCall>, refusal?: string, function_call?: FunctionCall}
 * @phpstan-type ChoiceResponse array{index: int, message: AssistantMessage, finish_reason: 'stop'|'length'|'tool_calls'|'content_filter'|'function_call'|string|null}
 * @phpstan-type UsageResponse array{prompt_tokens: int, completion_tokens: int, total_tokens: int}
 * @phpstan-type ChatCompletionResponse array{id: string, object: string, created: int, model: string, choices: list<ChoiceResponse>, usage?: UsageResponse, system_fingerprint?: string, service_tier?: string}
 * @phpstan-type StreamFunctionCall array{name?: string, arguments?: string}
 * @phpstan-type StreamToolCall array{index: int, id?: string, type?: 'function', function?: StreamFunctionCall}
 * @phpstan-type StreamDelta array{role?: string, content?: string|null, tool_calls?: list<StreamToolCall>, function_call?: StreamFunctionCall, refusal?: string}
 * @phpstan-type StreamChoice array{index: int, delta: StreamDelta, finish_reason: string|null}
 * @phpstan-type ChatCompletionChunk array{id: string, object: string, created: int, model: string, choices: list<StreamChoice>, usage?: UsageResponse, service_tier?: string}
 * @phpstan-type EmbeddingRequest array{model: string, input: string|list<string>, encoding_format?: string, dimensions?: int, user?: string}
 * @phpstan-type EmbeddingObject array{object: string, embedding: list<float>, index: int}
 * @phpstan-type EmbeddingResponse array{object: string, data: list<EmbeddingObject>, model: string, usage: UsageResponse}
 * @phpstan-type ModelObject array{id: string, object: string, created: int, owned_by: string}
 * @phpstan-type ModelsListResponse array{object: string, data: list<ModelObject>}
 */

/**
 * Unified LLM client backed by the liter-llm Rust core.
 *
 * All I/O methods accept a JSON-encoded request string and return a
 * JSON-encoded response string.  Use {@see json_encode} / {@see json_decode}
 * to convert between PHP arrays and the wire format.
 *
 * @example
 * ```php
 * $client = new \LiterLlm\LlmClient('sk-...', 'https://api.openai.com/v1');
 *
 * $response = json_decode($client->chat(json_encode([
 *     'model'    => 'gpt-4',
 *     'messages' => [['role' => 'user', 'content' => 'Hello']],
 * ])), true);
 *
 * echo $response['choices'][0]['message']['content'];
 * ```
 */
class LlmClient
{
    /** @var list<LlmHook> */
    private array $hooks = [];

    /** @var list<ProviderConfig> */
    private array $customProviders = [];

    /**
     * Create a new LlmClient.
     *
     * @param string            $apiKey       API key for authentication.  Pass an empty string
     *                                        for providers that do not require authentication.
     * @param string|null       $baseUrl      Override the provider base URL.  Pass `null` to use
     *                                        the default routing based on the model-name prefix.
     * @param int               $maxRetries   Number of retries on 429 / 5xx responses.
     * @param int               $timeoutSecs  Request timeout in seconds.
     * @param CacheConfig|null  $cacheConfig  Response caching configuration, or null to disable.
     * @param BudgetConfig|null $budgetConfig Cost budget enforcement configuration, or null to disable.
     */
    public function __construct(
        private readonly string $apiKey,
        private readonly ?string $baseUrl = null,
        private readonly int $maxRetries = 3,
        private readonly int $timeoutSecs = 60,
        private readonly ?CacheConfig $cacheConfig = null,
        private readonly ?BudgetConfig $budgetConfig = null,
    ) {
    }

    /**
     * Register a lifecycle hook.
     *
     * Hooks are invoked in registration order, synchronously.
     *
     * @param LlmHook $hook The hook to register.
     */
    public function addHook(LlmHook $hook): void
    {
        $this->hooks[] = $hook;
    }

    /**
     * Register a custom provider configuration.
     *
     * Requests whose model name starts with one of the provider's prefixes
     * are routed to its base URL.
     *
     * @param ProviderConfig $config The provider configuration to register.
     */
    public function registerProvider(ProviderConfig $config): void
    {
        $this->customProviders[] = $config;
    }

    /**
     * Returns the registered hooks.
     *
     * @return list<LlmHook>
     */
    public function getHooks(): array
    {
        return $this->hooks;
    }

    /**
     * Returns the registered custom providers.
     *
     * @return list<ProviderConfig>
     */
    public function getCustomProviders(): array
    {
        return $this->customProviders;
    }

    /**
     * Returns the configured cache settings, or null if caching is disabled.
     */
    public function getCacheConfig(): ?CacheConfig
    {
        return $this->cacheConfig;
    }

    /**
     * Returns the configured budget settings, or null if budget enforcement is disabled.
     */
    public function getBudgetConfig(): ?BudgetConfig
    {
        return $this->budgetConfig;
    }

    /**
     * Send a chat completion request.
     *
     * @param string $requestJson JSON-encoded {@see ChatCompletionRequest} object.
     *
     * @return string JSON-encoded {@see ChatCompletionResponse}.
     *
     * @throws \RuntimeException When the request is malformed, the network fails,
     *                           or the API returns an error.
     *
     * @phpstan-param string $requestJson
     * @phpstan-return string
     */
    public function chat(string $requestJson): string
    {
        throw new \RuntimeException('Native extension not loaded');
    }

    /**
     * Send a streaming chat completion request and collect all chunks.
     *
     * PHP's synchronous execution model does not support true incremental
     * streaming.  This method drives the full SSE stream to completion on
     * the Rust side and returns every chunk as a JSON array.  For real-time
     * token-by-token output consider the Node.js or Python bindings.
     *
     * The `"stream"` field in the request is forced to `true`; callers do
     * not need to set it explicitly.
     *
     * @param string $requestJson JSON-encoded {@see ChatCompletionRequest} object.
     *
     * @return string JSON-encoded `list<ChatCompletionChunk>`.
     *
     * @throws \RuntimeException On network or API errors.
     *
     * @phpstan-param string $requestJson
     * @phpstan-return string
     */
    public function chatStream(string $requestJson): string
    {
        throw new \RuntimeException('Native extension not loaded');
    }

    /**
     * Send an embedding request.
     *
     * @param string $requestJson JSON-encoded {@see EmbeddingRequest} object.
     *
     * @return string JSON-encoded {@see EmbeddingResponse}.
     *
     * @throws \RuntimeException On network or API errors.
     *
     * @phpstan-param string $requestJson
     * @phpstan-return string
     */
    public function embed(string $requestJson): string
    {
        throw new \RuntimeException('Native extension not loaded');
    }

    /**
     * List models available from the configured provider.
     *
     * @return string JSON-encoded {@see ModelsListResponse}.
     *
     * @throws \RuntimeException On network or API errors.
     *
     * @phpstan-return string
     */
    public function listModels(): string
    {
        throw new \RuntimeException('Native extension not loaded');
    }
}
