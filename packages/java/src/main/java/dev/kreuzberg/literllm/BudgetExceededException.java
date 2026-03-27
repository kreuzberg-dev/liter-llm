package dev.kreuzberg.literllm;

/**
 * Thrown when a request would exceed the configured cost budget.
 *
 * @see BudgetConfig
 */
public final class BudgetExceededException extends LlmException {

	/** Error code for budget exceeded errors. */
	public static final int CODE_BUDGET_EXCEEDED = 1800;

	/**
	 * Creates a budget-exceeded exception.
	 *
	 * @param message
	 *            human-readable description of the budget violation
	 */
	public BudgetExceededException(String message) {
		super(CODE_BUDGET_EXCEEDED, "liter-llm: budget exceeded: " + message);
	}
}
