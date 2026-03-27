<?php

declare(strict_types=1);

namespace LiterLlm;

/**
 * Thrown when a request would exceed the configured cost budget.
 *
 * @see BudgetConfig
 */
final class BudgetExceededException extends \RuntimeException
{
    public function __construct(string $message, ?\Throwable $previous = null)
    {
        parent::__construct('liter-llm: budget exceeded: ' . $message, 1800, $previous);
    }
}
