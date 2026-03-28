<?php

declare(strict_types=1);

namespace LiterLlm;

/**
 * Unified LLM client backed by the liter-llm Rust core.
 *
 * When the native extension is loaded, this class is replaced by the Rust
 * implementation which handles caching, budget enforcement, middleware, and
 * all HTTP communication.  The PHP stubs below exist only for static analysis,
 * IDE autocompletion, and error messages when the extension is missing.
 *
 * All I/O methods accept a JSON-encoded request string and return a
 * JSON-encoded response string.
 *
 * @phpstan-type ChatCompletionRequest array{model: string, messages: list<array<string, mixed>>, temperature?: float, top_p?: float, max_tokens?: int}
 * @phpstan-type ChatCompletionResponse array{id: string, object: string, created: int, model: string, choices: list<array<string, mixed>>, usage?: array<string, int>}
 */
class LlmClient
{
    /**
     * Create a new LlmClient.
     *
     * When the native extension is loaded, the Rust constructor handles all
     * parameters.  Cache and budget configs are passed as JSON strings.
     *
     * @param string      $apiKey        API key for authentication.
     * @param string|null $baseUrl       Override the provider base URL.
     * @param string|null $modelHint     Model hint for provider detection.
     * @param int|null    $maxRetries    Retries on 429 / 5xx (default: 3).
     * @param int|null    $timeoutSecs   Request timeout in seconds (default: 60).
     * @param string|null $cacheJson     Cache config as JSON, e.g. '{"max_entries":256,"ttl_seconds":300}'.
     * @param string|null $budgetJson    Budget config as JSON, e.g. '{"global_limit":10.0,"enforcement":"hard"}'.
     * @param int|null    $cooldownSecs  Cooldown after transient errors.
     * @param string|null $rateLimitJson Rate limit config as JSON.
     * @param int|null    $healthCheckSecs Health check interval in seconds.
     * @param bool|null   $costTracking  Enable cost tracking.
     * @param bool|null   $tracing       Enable OpenTelemetry tracing.
     */
    public function __construct(
        string $apiKey,
        ?string $baseUrl = null,
        ?string $modelHint = null,
        ?int $maxRetries = null,
        ?int $timeoutSecs = null,
        ?string $cacheJson = null,
        ?string $budgetJson = null,
        ?int $cooldownSecs = null,
        ?string $rateLimitJson = null,
        ?int $healthCheckSecs = null,
        ?bool $costTracking = null,
        ?bool $tracing = null,
    ) {
        throw new \RuntimeException(
            'Native extension not loaded. Install the liter-llm PHP extension to use this class.',
        );
    }

    /**
     * Register a lifecycle hook.
     *
     * @param object $hook An object implementing onRequest/onResponse/onError methods.
     */
    public function addHook(object $hook): void
    {
        throw new \RuntimeException('Native extension not loaded');
    }

    /**
     * Register a custom provider at runtime.
     *
     * @param string $configJson JSON-encoded provider config with name, base_url, model_prefixes.
     */
    public function registerProvider(string $configJson): void
    {
        throw new \RuntimeException('Native extension not loaded');
    }

    /**
     * Return the total budget spend in USD.
     *
     * @return float Total spend tracked by the budget middleware.
     */
    public function budgetUsed(): float
    {
        throw new \RuntimeException('Native extension not loaded');
    }

    /**
     * Send a chat completion request.
     *
     * @param string $requestJson JSON-encoded chat request.
     * @return string JSON-encoded chat completion response.
     */
    public function chat(string $requestJson): string
    {
        throw new \RuntimeException('Native extension not loaded');
    }

    /**
     * Send a streaming chat completion request and collect all chunks.
     *
     * @param string $requestJson JSON-encoded chat request.
     * @return string JSON-encoded array of ChatCompletionChunk objects.
     */
    public function chatStream(string $requestJson): string
    {
        throw new \RuntimeException('Native extension not loaded');
    }

    /**
     * Send an embedding request.
     *
     * @param string $requestJson JSON-encoded embedding request.
     * @return string JSON-encoded embedding response.
     */
    public function embed(string $requestJson): string
    {
        throw new \RuntimeException('Native extension not loaded');
    }

    /**
     * List models available from the configured provider.
     *
     * @return string JSON-encoded models list response.
     */
    public function listModels(): string
    {
        throw new \RuntimeException('Native extension not loaded');
    }

    /**
     * Generate an image from a text prompt.
     *
     * @param string $requestJson JSON-encoded image generation request.
     * @return string JSON-encoded images response.
     */
    public function imageGenerate(string $requestJson): string
    {
        throw new \RuntimeException('Native extension not loaded');
    }

    /**
     * Generate speech audio from text.
     *
     * @param string $requestJson JSON-encoded speech request.
     * @return string Raw audio bytes.
     */
    public function speech(string $requestJson): string
    {
        throw new \RuntimeException('Native extension not loaded');
    }

    /**
     * Transcribe audio to text.
     *
     * @param string $requestJson JSON-encoded transcription request.
     * @return string JSON-encoded transcription response.
     */
    public function transcribe(string $requestJson): string
    {
        throw new \RuntimeException('Native extension not loaded');
    }

    /**
     * Check content against moderation policies.
     *
     * @param string $requestJson JSON-encoded moderation request.
     * @return string JSON-encoded moderation response.
     */
    public function moderate(string $requestJson): string
    {
        throw new \RuntimeException('Native extension not loaded');
    }

    /**
     * Rerank documents by relevance to a query.
     *
     * @param string $requestJson JSON-encoded rerank request.
     * @return string JSON-encoded rerank response.
     */
    public function rerank(string $requestJson): string
    {
        throw new \RuntimeException('Native extension not loaded');
    }

    /**
     * Perform a web/document search.
     *
     * @param string $requestJson JSON-encoded search request.
     * @return string JSON-encoded search response.
     */
    public function search(string $requestJson): string
    {
        throw new \RuntimeException('Native extension not loaded');
    }

    /**
     * Extract text from a document via OCR.
     *
     * @param string $requestJson JSON-encoded OCR request.
     * @return string JSON-encoded OCR response.
     */
    public function ocr(string $requestJson): string
    {
        throw new \RuntimeException('Native extension not loaded');
    }
}
