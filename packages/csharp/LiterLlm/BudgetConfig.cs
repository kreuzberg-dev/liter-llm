namespace LiterLlm;

/// <summary>
/// Configuration for cost budget enforcement.
/// </summary>
/// <param name="GlobalLimit">
/// Maximum total cost allowed across all models, or <c>null</c> for no global limit.
/// </param>
/// <param name="ModelLimits">Per-model cost limits keyed by model name.</param>
/// <param name="Enforcement">
/// Enforcement mode: <c>"strict"</c> (reject requests exceeding budget)
/// or <c>"warn"</c> (log a warning).
/// </param>
public record BudgetConfig(
    double? GlobalLimit = null,
    IReadOnlyDictionary<string, double>? ModelLimits = null,
    string Enforcement = "strict");
