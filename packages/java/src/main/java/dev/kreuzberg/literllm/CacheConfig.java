package dev.kreuzberg.literllm;

/**
 * Configuration for response caching.
 *
 * @param maxEntries
 *            maximum number of cached responses to retain
 * @param ttlSeconds
 *            time-to-live for each cache entry in seconds
 */
public record CacheConfig(int maxEntries, int ttlSeconds) {

	/** Default TTL in seconds (5 minutes). */
	public static final int DEFAULT_TTL_SECONDS = 300;

	/**
	 * Creates a cache config with default TTL of 5 minutes.
	 *
	 * @param maxEntries
	 *            maximum number of cached responses
	 */
	public CacheConfig(int maxEntries) {
		this(maxEntries, DEFAULT_TTL_SECONDS);
	}
}
