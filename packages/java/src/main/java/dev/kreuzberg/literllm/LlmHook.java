package dev.kreuzberg.literllm;

/**
 * Lifecycle hook interface for intercepting LLM client request/response events.
 *
 * <p>
 * Implement this interface and register it via
 * {@link LlmClient#addHook(LlmHook)}. All methods have default no-op
 * implementations; override only the callbacks you need.
 *
 * <p>
 * Hooks are invoked in registration order, synchronously on the calling thread.
 */
public interface LlmHook {

	/**
	 * Called before the HTTP request is sent.
	 *
	 * <p>
	 * Throw {@link HookRejectedException} to abort the request.
	 *
	 * @param request
	 *            the request object about to be sent
	 * @throws HookRejectedException
	 *             to reject the request
	 */
	default void onRequest(Object request) throws HookRejectedException {
		// no-op by default
	}

	/**
	 * Called after a successful response is received.
	 *
	 * @param request
	 *            the original request
	 * @param response
	 *            the response received from the provider
	 */
	default void onResponse(Object request, Object response) {
		// no-op by default
	}

	/**
	 * Called after a request fails.
	 *
	 * @param request
	 *            the original request
	 * @param error
	 *            the exception that caused the failure
	 */
	default void onError(Object request, Exception error) {
		// no-op by default
	}
}
