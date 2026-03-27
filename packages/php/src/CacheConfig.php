<?php

declare(strict_types=1);

namespace LiterLlm;

/**
 * Configuration for response caching.
 */
final class CacheConfig
{
    /**
     * @param int $maxEntries Maximum number of cached responses to retain.
     * @param int $ttlSeconds Time-to-live for each cache entry in seconds.
     */
    public function __construct(
        public readonly int $maxEntries,
        public readonly int $ttlSeconds = 300,
    ) {
    }
}
