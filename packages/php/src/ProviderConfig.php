<?php

declare(strict_types=1);

namespace LiterLlm;

/**
 * Configuration for a custom LLM provider not in the built-in registry.
 */
final class ProviderConfig
{
    /**
     * @param string        $name          Unique provider name used for model routing.
     * @param string        $baseUrl       The provider's API base URL.
     * @param string        $authHeader    The header name used for authentication.
     * @param list<string>  $modelPrefixes Model name prefixes that route to this provider.
     */
    public function __construct(
        public readonly string $name,
        public readonly string $baseUrl,
        public readonly string $authHeader,
        public readonly array $modelPrefixes = [],
    ) {
    }
}
