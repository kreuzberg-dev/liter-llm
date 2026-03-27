namespace LiterLlm;

/// <summary>
/// Configuration for response caching.
/// </summary>
/// <param name="MaxEntries">Maximum number of cached responses to retain.</param>
/// <param name="TtlSeconds">Time-to-live for each cache entry in seconds.</param>
public record CacheConfig(int MaxEntries, int TtlSeconds = 300);
