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

    // ── Cache state ─────────────────────────────────────────────────────

    /** @var array<string, array{response: string, time: float}> */
    private array $cacheEntries = [];

    /** @var list<string> Insertion-ordered keys for LRU eviction. */
    private array $cacheOrder = [];

    // ── Budget state ────────────────────────────────────────────────────

    /** Cumulative spend across all models. */
    private float $globalSpend = 0.0;

    /** @var array<string, float> Per-model cumulative spend. */
    private array $modelSpend = [];

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

    // ── Hook & provider registration ────────────────────────────────────

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

    // ── Accessors ───────────────────────────────────────────────────────

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
     * Returns the current global spend tracked by the budget system.
     */
    public function getGlobalSpend(): float
    {
        return $this->globalSpend;
    }

    /**
     * Returns per-model spend tracked by the budget system.
     *
     * @return array<string, float>
     */
    public function getModelSpend(): array
    {
        return $this->modelSpend;
    }

    // ── Public API ──────────────────────────────────────────────────────

    /**
     * Send a chat completion request.
     *
     * @param string $requestJson JSON-encoded {@see ChatCompletionRequest} object.
     *
     * @return string JSON-encoded {@see ChatCompletionResponse}.
     *
     * @throws \RuntimeException When the request is malformed, the network fails,
     *                           or the API returns an error.
     * @throws BudgetExceededException When the cost budget is exceeded (strict mode).
     * @throws HookRejectedException When a hook rejects the request.
     *
     * @phpstan-param string $requestJson
     * @phpstan-return string
     */
    public function chat(string $requestJson): string
    {
        $model = $this->extractModel($requestJson);
        $this->runOnRequest($requestJson);
        $this->checkBudget($model);

        $cached = $this->checkCache($requestJson);
        if ($cached !== null) {
            $this->runOnResponse($requestJson, $cached);
            return $cached;
        }

        try {
            $response = $this->nativeChat($requestJson);
        } catch (\Throwable $e) {
            $this->runOnError($requestJson, $e);
            throw $e;
        }

        $this->storeCache($requestJson, $response);
        $this->recordCost($model, $response);
        $this->runOnResponse($requestJson, $response);
        return $response;
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
     * @throws BudgetExceededException When the cost budget is exceeded (strict mode).
     * @throws HookRejectedException When a hook rejects the request.
     *
     * @phpstan-param string $requestJson
     * @phpstan-return string
     */
    public function chatStream(string $requestJson): string
    {
        $model = $this->extractModel($requestJson);
        $this->runOnRequest($requestJson);
        $this->checkBudget($model);

        try {
            $response = $this->nativeChatStream($requestJson);
        } catch (\Throwable $e) {
            $this->runOnError($requestJson, $e);
            throw $e;
        }

        $this->recordCost($model, $response);
        $this->runOnResponse($requestJson, $response);
        return $response;
    }

    /**
     * Send an embedding request.
     *
     * @param string $requestJson JSON-encoded {@see EmbeddingRequest} object.
     *
     * @return string JSON-encoded {@see EmbeddingResponse}.
     *
     * @throws \RuntimeException On network or API errors.
     * @throws BudgetExceededException When the cost budget is exceeded (strict mode).
     * @throws HookRejectedException When a hook rejects the request.
     *
     * @phpstan-param string $requestJson
     * @phpstan-return string
     */
    public function embed(string $requestJson): string
    {
        $model = $this->extractModel($requestJson);
        $this->runOnRequest($requestJson);
        $this->checkBudget($model);

        $cached = $this->checkCache($requestJson);
        if ($cached !== null) {
            $this->runOnResponse($requestJson, $cached);
            return $cached;
        }

        try {
            $response = $this->nativeEmbed($requestJson);
        } catch (\Throwable $e) {
            $this->runOnError($requestJson, $e);
            throw $e;
        }

        $this->storeCache($requestJson, $response);
        $this->recordCost($model, $response);
        $this->runOnResponse($requestJson, $response);
        return $response;
    }

    /**
     * List models available from the configured provider.
     *
     * @return string JSON-encoded {@see ModelsListResponse}.
     *
     * @throws \RuntimeException On network or API errors.
     * @throws HookRejectedException When a hook rejects the request.
     *
     * @phpstan-return string
     */
    public function listModels(): string
    {
        $this->runOnRequest('{"action":"list_models"}');

        try {
            $response = $this->nativeListModels();
        } catch (\Throwable $e) {
            $this->runOnError('{"action":"list_models"}', $e);
            throw $e;
        }

        $this->runOnResponse('{"action":"list_models"}', $response);
        return $response;
    }

    // ── Cache implementation ────────────────────────────────────────────

    /**
     * Look up a cached response for the given request JSON.
     *
     * Returns the cached response string if found and not expired, or null.
     */
    private function checkCache(string $requestJson): ?string
    {
        if ($this->cacheConfig === null) {
            return null;
        }

        $key = $this->cacheKey($requestJson);
        if (!isset($this->cacheEntries[$key])) {
            return null;
        }

        $entry = $this->cacheEntries[$key];
        $elapsed = microtime(true) - $entry['time'];

        if ($elapsed > $this->cacheConfig->ttlSeconds) {
            unset($this->cacheEntries[$key]);
            $this->cacheOrder = array_values(array_filter(
                $this->cacheOrder,
                static fn (string $k): bool => $k !== $key,
            ));
            return null;
        }

        return $entry['response'];
    }

    /**
     * Store a response in the cache, evicting the oldest entry if necessary.
     */
    private function storeCache(string $requestJson, string $response): void
    {
        if ($this->cacheConfig === null) {
            return;
        }

        $key = $this->cacheKey($requestJson);

        // Evict oldest if at capacity
        while (count($this->cacheEntries) >= $this->cacheConfig->maxEntries) {
            $oldest = array_shift($this->cacheOrder);
            if ($oldest !== null) {
                unset($this->cacheEntries[$oldest]);
            }
        }

        $this->cacheEntries[$key] = [
            'response' => $response,
            'time' => microtime(true),
        ];
        $this->cacheOrder[] = $key;
    }

    /**
     * Compute a cache key from the request JSON string.
     */
    private function cacheKey(string $requestJson): string
    {
        return hash('sha256', $requestJson);
    }

    // ── Budget implementation ───────────────────────────────────────────

    /**
     * Check whether the next request for the given model is within budget.
     *
     * @throws BudgetExceededException In strict mode when the budget is exceeded.
     */
    private function checkBudget(?string $model): void
    {
        if ($this->budgetConfig === null) {
            return;
        }

        $globalLimit = $this->budgetConfig->globalLimit;
        if ($globalLimit !== null && $this->globalSpend >= $globalLimit) {
            $msg = sprintf(
                'global budget exceeded: $%.4f >= $%.4f',
                $this->globalSpend,
                $globalLimit,
            );
            $this->handleBudgetExceeded($msg);
        }

        if ($model !== null) {
            $modelLimits = $this->budgetConfig->modelLimits;
            if (isset($modelLimits[$model])) {
                $limit = $modelLimits[$model];
                $spent = $this->modelSpend[$model] ?? 0.0;
                if ($spent >= $limit) {
                    $msg = sprintf(
                        "model '%s' budget exceeded: \$%.4f >= \$%.4f",
                        $model,
                        $spent,
                        $limit,
                    );
                    $this->handleBudgetExceeded($msg);
                }
            }
        }
    }

    /**
     * Record usage-based cost from a response.
     *
     * Uses total_tokens as a rough cost proxy ($0.001 per 1K tokens).
     */
    private function recordCost(?string $model, string $responseJson): void
    {
        if ($this->budgetConfig === null) {
            return;
        }

        $decoded = json_decode($responseJson, true);
        if (!is_array($decoded)) {
            return;
        }

        $totalTokens = $decoded['usage']['total_tokens'] ?? null;
        if (!is_int($totalTokens)) {
            return;
        }

        // Approximate cost: $0.001 per 1K tokens
        $cost = ($totalTokens / 1000.0) * 0.001;

        $this->globalSpend += $cost;
        if ($model !== null) {
            $this->modelSpend[$model] = ($this->modelSpend[$model] ?? 0.0) + $cost;
        }
    }

    /**
     * Handle a budget exceeded condition based on enforcement mode.
     *
     * @throws BudgetExceededException In strict mode.
     */
    private function handleBudgetExceeded(string $message): void
    {
        if ($this->budgetConfig !== null && $this->budgetConfig->enforcement === 'warn') {
            // In warn mode, log via error_log but do not throw.
            error_log('liter-llm: ' . $message);
            return;
        }

        throw new BudgetExceededException($message);
    }

    // ── Hook invocation ─────────────────────────────────────────────────

    /**
     * Invoke all registered hooks' onRequest callback.
     *
     * @param mixed $request The request data about to be sent.
     *
     * @throws HookRejectedException If a hook rejects the request.
     */
    private function runOnRequest(mixed $request): void
    {
        foreach ($this->hooks as $hook) {
            $hook->onRequest($request);
        }
    }

    /**
     * Invoke all registered hooks' onResponse callback.
     *
     * @param mixed $request  The original request.
     * @param mixed $response The response received from the provider.
     */
    private function runOnResponse(mixed $request, mixed $response): void
    {
        foreach ($this->hooks as $hook) {
            try {
                $hook->onResponse($request, $response);
            } catch (\Throwable) {
                // Advisory hooks must not break the response flow.
            }
        }
    }

    /**
     * Invoke all registered hooks' onError callback.
     *
     * @param mixed      $request The original request.
     * @param \Throwable $error   The exception that caused the failure.
     */
    private function runOnError(mixed $request, \Throwable $error): void
    {
        foreach ($this->hooks as $hook) {
            try {
                $hook->onError($request, $error);
            } catch (\Throwable) {
                // Advisory hooks must not mask the original error.
            }
        }
    }

    // ── Helpers ─────────────────────────────────────────────────────────

    /**
     * Extract the model name from a JSON-encoded request, or null if absent.
     */
    private function extractModel(string $requestJson): ?string
    {
        $decoded = json_decode($requestJson, true);
        if (is_array($decoded) && isset($decoded['model']) && is_string($decoded['model'])) {
            return $decoded['model'];
        }
        return null;
    }

    // ── Native extension stubs ──────────────────────────────────────────
    //
    // These methods are overridden by the Rust native extension at load
    // time.  The PHP stubs exist so that static analysis, IDE autocompletion,
    // and unit tests work even without the compiled extension.

    /**
     * Native chat completion call (overridden by extension).
     *
     * @phpstan-param string $requestJson
     * @phpstan-return string
     */
    protected function nativeChat(string $requestJson): string
    {
        throw new \RuntimeException('Native extension not loaded');
    }

    /**
     * Native streaming chat completion call (overridden by extension).
     *
     * @phpstan-param string $requestJson
     * @phpstan-return string
     */
    protected function nativeChatStream(string $requestJson): string
    {
        throw new \RuntimeException('Native extension not loaded');
    }

    /**
     * Native embedding call (overridden by extension).
     *
     * @phpstan-param string $requestJson
     * @phpstan-return string
     */
    protected function nativeEmbed(string $requestJson): string
    {
        throw new \RuntimeException('Native extension not loaded');
    }

    /**
     * Native list models call (overridden by extension).
     *
     * @phpstan-return string
     */
    protected function nativeListModels(): string
    {
        throw new \RuntimeException('Native extension not loaded');
    }
}
