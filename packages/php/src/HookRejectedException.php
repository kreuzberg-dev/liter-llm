<?php

declare(strict_types=1);

namespace LiterLlm;

/**
 * Thrown when an {@see LlmHook::onRequest()} rejects a request.
 */
final class HookRejectedException extends \RuntimeException
{
    public function __construct(string $message, ?\Throwable $previous = null)
    {
        parent::__construct('liter-llm: hook rejected request: ' . $message, 1801, $previous);
    }
}
