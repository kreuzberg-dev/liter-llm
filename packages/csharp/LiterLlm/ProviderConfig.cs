namespace LiterLlm;

/// <summary>
/// Configuration for a custom LLM provider not in the built-in registry.
/// </summary>
/// <param name="Name">Unique provider name used for model routing.</param>
/// <param name="BaseUrl">The provider's API base URL.</param>
/// <param name="AuthHeader">The header name used for authentication (e.g. "Authorization").</param>
/// <param name="ModelPrefixes">Model name prefixes that route to this provider.</param>
public record ProviderConfig(
    string Name,
    string BaseUrl,
    string AuthHeader,
    IReadOnlyList<string> ModelPrefixes);
