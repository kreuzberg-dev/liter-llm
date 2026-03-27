package dev.kreuzberg.literllm;

import java.util.Map;

/**
 * Configuration for cost budget enforcement.
 *
 * @param globalLimit
 *            maximum total cost allowed across all models, or {@code null} for
 *            no global limit
 * @param modelLimits
 *            per-model cost limits keyed by model name
 * @param enforcement
 *            enforcement mode: {@code "strict"} (reject requests exceeding
 *            budget) or {@code "warn"} (log a warning)
 */
public record BudgetConfig(Double globalLimit, Map<String, Double> modelLimits, String enforcement) {

	/**
	 * Creates a budget config with strict enforcement and no per-model limits.
	 *
	 * @param globalLimit
	 *            maximum total cost allowed
	 */
	public BudgetConfig(double globalLimit) {
		this(globalLimit, Map.of(), "strict");
	}
}
