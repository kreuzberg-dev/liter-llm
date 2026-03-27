<?php

declare(strict_types=1);

namespace LiterLlm;

/**
 * Configuration for cost budget enforcement.
 */
final class BudgetConfig
{
    /**
     * @param float|null              $globalLimit  Maximum total cost allowed across all models, or null for no limit.
     * @param array<string, float>    $modelLimits  Per-model cost limits keyed by model name.
     * @param string                  $enforcement  Enforcement mode: "strict" or "warn".
     */
    public function __construct(
        public readonly ?float $globalLimit = null,
        public readonly array $modelLimits = [],
        public readonly string $enforcement = 'strict',
    ) {
    }
}
