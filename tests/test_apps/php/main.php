<?php

declare(strict_types=1);

/**
 * Smoke tests for the liter-llm published PHP package.
 *
 * Validates the published package works against real LLM APIs.
 * Requires API keys in environment variables or .env file at repo root.
 */

require_once __DIR__ . '/vendor/autoload.php';

use LiterLlm\LlmClient;
use LiterLlm\CacheConfig;

// ── .env loader ─────────────────────────────────────────────────────────────

function loadDotenv(): void
{
    $dir = dirname(__DIR__);
    for ($i = 0; $i < 4; $i++) {
        $envFile = $dir . DIRECTORY_SEPARATOR . '.env';
        if (file_exists($envFile)) {
            foreach (file($envFile, FILE_IGNORE_NEW_LINES | FILE_SKIP_EMPTY_LINES) as $line) {
                $line = trim($line);
                if ($line === '' || str_starts_with($line, '#')) {
                    continue;
                }
                $idx = strpos($line, '=');
                if ($idx === false) {
                    continue;
                }
                $key = trim(substr($line, 0, $idx));
                $value = trim(substr($line, $idx + 1));
                if (getenv($key) === false) {
                    putenv("$key=$value");
                }
            }
            break;
        }
        $dir = dirname($dir);
    }
}

loadDotenv();

function envKey(string $name): ?string
{
    $value = getenv($name);
    return ($value !== false && $value !== '') ? $value : null;
}

// ── Test runner ─────────────────────────────────────────────────────────────

final class SmokeTest
{
    public int $passed = 0;
    public int $failed = 0;
    public int $skipped = 0;

    /**
     * @param callable(): ?string $fn
     */
    public function run(string $name, callable $fn): void
    {
        echo "  $name... ";
        try {
            $result = $fn();
            if ($result === null) {
                echo "SKIP\n";
                $this->skipped++;
            } else {
                echo "PASS\n";
                $this->passed++;
            }
        } catch (\Throwable $e) {
            echo "FAIL: {$e->getMessage()}\n";
            $this->failed++;
        }
    }

    public function summary(): int
    {
        $total = $this->passed + $this->failed + $this->skipped;
        echo "\n";
        echo str_repeat('=', 60) . "\n";
        echo "Results: {$this->passed} passed, {$this->failed} failed, {$this->skipped} skipped ({$total} total)\n";
        return $this->failed > 0 ? 1 : 0;
    }
}

// ── Test cases ──────────────────────────────────────────────────────────────

function testChatOpenAI(): ?string
{
    $key = envKey('OPENAI_API_KEY');
    if ($key === null) {
        return null;
    }
    $client = new LlmClient($key);
    /** @var array<string, mixed> $resp */
    $resp = json_decode($client->chat(json_encode([
        'model' => 'openai/gpt-4o-mini',
        'messages' => [['role' => 'user', 'content' => 'Say hello in one word.']],
        'max_tokens' => 10,
    ], JSON_THROW_ON_ERROR)), true, 512, JSON_THROW_ON_ERROR);
    if (empty($resp['choices'])) {
        throw new RuntimeException('no choices in response');
    }
    if (empty($resp['choices'][0]['message']['content'])) {
        throw new RuntimeException('empty content');
    }
    if (empty($resp['usage']) || ($resp['usage']['total_tokens'] ?? 0) <= 0) {
        throw new RuntimeException('no usage data');
    }
    return 'ok';
}

function testChatAnthropic(): ?string
{
    $key = envKey('ANTHROPIC_API_KEY');
    if ($key === null) {
        return null;
    }
    $client = new LlmClient($key);
    /** @var array<string, mixed> $resp */
    $resp = json_decode($client->chat(json_encode([
        'model' => 'anthropic/claude-sonnet-4-20250514',
        'messages' => [['role' => 'user', 'content' => 'Say hello in one word.']],
        'max_tokens' => 10,
    ], JSON_THROW_ON_ERROR)), true, 512, JSON_THROW_ON_ERROR);
    if (empty($resp['choices'])) {
        throw new RuntimeException('no choices');
    }
    if (empty($resp['choices'][0]['message']['content'])) {
        throw new RuntimeException('empty content');
    }
    return 'ok';
}

function testChatGemini(): ?string
{
    $key = envKey('GEMINI_API_KEY');
    if ($key === null) {
        return null;
    }
    $client = new LlmClient($key);
    /** @var array<string, mixed> $resp */
    $resp = json_decode($client->chat(json_encode([
        'model' => 'gemini/gemini-2.5-flash-lite',
        'messages' => [['role' => 'user', 'content' => 'Say hello in one word.']],
        'max_tokens' => 10,
    ], JSON_THROW_ON_ERROR)), true, 512, JSON_THROW_ON_ERROR);
    if (empty($resp['choices'])) {
        throw new RuntimeException('no choices');
    }
    if (empty($resp['choices'][0]['message']['content'])) {
        throw new RuntimeException('empty content');
    }
    return 'ok';
}

function testStreamingOpenAI(): ?string
{
    $key = envKey('OPENAI_API_KEY');
    if ($key === null) {
        return null;
    }
    $client = new LlmClient($key);
    $chunksJson = $client->chatStream(json_encode([
        'model' => 'openai/gpt-4o-mini',
        'messages' => [['role' => 'user', 'content' => 'Count from 1 to 5.']],
        'max_tokens' => 50,
    ], JSON_THROW_ON_ERROR));
    /** @var list<array<string, mixed>> $chunks */
    $chunks = json_decode($chunksJson, true, 512, JSON_THROW_ON_ERROR);
    if (empty($chunks)) {
        throw new RuntimeException('no chunks received');
    }
    return 'ok';
}

function testEmbedOpenAI(): ?string
{
    $key = envKey('OPENAI_API_KEY');
    if ($key === null) {
        return null;
    }
    $client = new LlmClient($key);
    /** @var array<string, mixed> $resp */
    $resp = json_decode($client->embed(json_encode([
        'model' => 'openai/text-embedding-3-small',
        'input' => ['Hello, world!'],
    ], JSON_THROW_ON_ERROR)), true, 512, JSON_THROW_ON_ERROR);
    if (empty($resp['data'])) {
        throw new RuntimeException('no embeddings');
    }
    if (empty($resp['data'][0]['embedding'])) {
        throw new RuntimeException('empty embedding vector');
    }
    return 'ok';
}

function testListModelsOpenAI(): ?string
{
    $key = envKey('OPENAI_API_KEY');
    if ($key === null) {
        return null;
    }
    $client = new LlmClient($key);
    /** @var array<string, mixed> $resp */
    $resp = json_decode($client->listModels(), true, 512, JSON_THROW_ON_ERROR);
    if (empty($resp['data'])) {
        throw new RuntimeException('no models returned');
    }
    return 'ok';
}

function testProviderRouting(): ?string
{
    $openaiKey = envKey('OPENAI_API_KEY');
    $anthropicKey = envKey('ANTHROPIC_API_KEY');
    if ($openaiKey === null || $anthropicKey === null) {
        return null;
    }

    $messages = [['role' => 'user', 'content' => 'Say hi.']];

    $clientOpenAI = new LlmClient($openaiKey);
    /** @var array<string, mixed> $r1 */
    $r1 = json_decode($clientOpenAI->chat(json_encode([
        'model' => 'openai/gpt-4o-mini',
        'messages' => $messages,
        'max_tokens' => 5,
    ], JSON_THROW_ON_ERROR)), true, 512, JSON_THROW_ON_ERROR);
    if (empty($r1['choices'])) {
        throw new RuntimeException('OpenAI failed');
    }

    $clientAnthropic = new LlmClient($anthropicKey);
    /** @var array<string, mixed> $r2 */
    $r2 = json_decode($clientAnthropic->chat(json_encode([
        'model' => 'anthropic/claude-sonnet-4-20250514',
        'messages' => $messages,
        'max_tokens' => 5,
    ], JSON_THROW_ON_ERROR)), true, 512, JSON_THROW_ON_ERROR);
    if (empty($r2['choices'])) {
        throw new RuntimeException('Anthropic failed');
    }
    return 'ok';
}

function testCacheMemory(): ?string
{
    $key = envKey('OPENAI_API_KEY');
    if ($key === null) {
        return null;
    }
    $client = new LlmClient(
        $key,
        cacheConfig: new CacheConfig(maxEntries: 10, ttlSeconds: 60),
    );
    $messages = [['role' => 'user', 'content' => 'What is 2+2? Answer with just the number.']];
    $reqJson = json_encode([
        'model' => 'openai/gpt-4o-mini',
        'messages' => $messages,
        'max_tokens' => 5,
    ], JSON_THROW_ON_ERROR);

    /** @var array<string, mixed> $r1 */
    $r1 = json_decode($client->chat($reqJson), true, 512, JSON_THROW_ON_ERROR);
    /** @var array<string, mixed> $r2 */
    $r2 = json_decode($client->chat($reqJson), true, 512, JSON_THROW_ON_ERROR);

    if (empty($r1['choices'])) {
        throw new RuntimeException('first request failed');
    }
    if (empty($r2['choices'])) {
        throw new RuntimeException('second request failed');
    }
    if (($r1['choices'][0]['message']['content'] ?? '') !== ($r2['choices'][0]['message']['content'] ?? '')) {
        throw new RuntimeException('cache miss - responses differ');
    }
    return 'ok';
}

// ── Main ────────────────────────────────────────────────────────────────────

echo "liter-llm Smoke Tests (PHP)\n";
echo str_repeat('=', 60) . "\n\n";

$suite = new SmokeTest();

echo "Chat Completions:\n";
$suite->run('OpenAI gpt-4o-mini', testChatOpenAI(...));
$suite->run('Anthropic claude-3-5-haiku', testChatAnthropic(...));
$suite->run('Google gemini-2.5-flash-lite', testChatGemini(...));

$suite->run('OpenAI streaming', testStreamingOpenAI(...));
$suite->run('OpenAI text-embedding-3-small', testEmbedOpenAI(...));
$suite->run('OpenAI list models', testListModelsOpenAI(...));
$suite->run('Multi-provider routing', testProviderRouting(...));
$suite->run('In-memory cache hit', testCacheMemory(...));

exit($suite->summary());
