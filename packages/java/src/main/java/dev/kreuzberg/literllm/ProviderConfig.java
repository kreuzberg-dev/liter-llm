package dev.kreuzberg.literllm;

import java.util.List;

/**
 * Configuration for a custom LLM provider not in the built-in registry.
 *
 * @param name
 *            unique provider name used for model routing
 * @param baseUrl
 *            the provider's API base URL
 * @param authHeader
 *            the header name used for authentication (e.g. "Authorization")
 * @param modelPrefixes
 *            model name prefixes that route to this provider
 */
public record ProviderConfig(String name, String baseUrl, String authHeader, List<String> modelPrefixes) {
}
