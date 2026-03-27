package dev.kreuzberg.literllm;

/**
 * Thrown when an {@link LlmHook#onRequest(Object)} rejects a request.
 */
public final class HookRejectedException extends LlmException {

	/** Error code for hook rejection errors. */
	public static final int CODE_HOOK_REJECTED = 1801;

	/**
	 * Creates a hook-rejected exception.
	 *
	 * @param message
	 *            human-readable reason for the rejection
	 */
	public HookRejectedException(String message) {
		super(CODE_HOOK_REJECTED, "liter-llm: hook rejected request: " + message);
	}

	/**
	 * Creates a hook-rejected exception with a cause.
	 *
	 * @param message
	 *            human-readable reason for the rejection
	 * @param cause
	 *            the underlying exception
	 */
	public HookRejectedException(String message, Throwable cause) {
		super(CODE_HOOK_REJECTED, "liter-llm: hook rejected request: " + message, cause);
	}
}
