package dev.kreuzberg.literllm;

/**
 * Base exception for all liter-llm client errors.
 *
 * <p>
 * All errors thrown by {@link LlmClient} extend this class. Use the
 * {@link #getErrorCode()} method for programmatic error handling.
 */
public class LlmException extends Exception {

	/** Numeric error codes (1000+) for programmatic error handling. */
	public static final int CODE_UNKNOWN = 1000;

	public static final int CODE_INVALID_REQUEST = 1400;
	public static final int CODE_AUTHENTICATION = 1401;
	public static final int CODE_RATE_LIMIT = 1429;
	public static final int CODE_NOT_FOUND = 1404;
	public static final int CODE_PROVIDER_ERROR = 1500;
	public static final int CODE_STREAM_ERROR = 1600;
	public static final int CODE_SERIALIZATION = 1700;

	private final int errorCode;

	/**
	 * Creates an exception with a message and error code.
	 *
	 * @param errorCode
	 *            numeric code (1000+) identifying the error category
	 * @param message
	 *            human-readable error description
	 */
	public LlmException(int errorCode, String message) {
		super(message);
		this.errorCode = errorCode;
	}

	/**
	 * Creates an exception with a message, error code, and cause.
	 *
	 * @param errorCode
	 *            numeric code (1000+) identifying the error category
	 * @param message
	 *            human-readable error description
	 * @param cause
	 *            the underlying exception that triggered this error
	 */
	public LlmException(int errorCode, String message, Throwable cause) {
		super(message, cause);
		this.errorCode = errorCode;
	}

	/**
	 * Returns the numeric error code identifying the category of this error.
	 *
	 * @return error code (1000+)
	 */
	public int getErrorCode() {
		return errorCode;
	}

	// ─── Subtypes ─────────────────────────────────────────────────────────────

	/**
	 * Thrown when the provider rejects the request due to invalid input.
	 *
	 * <p>
	 * Corresponds to HTTP 400 / 422 from the provider.
	 */
	public static final class InvalidRequestException extends LlmException {

		/** Creates an invalid-request exception. */
		public InvalidRequestException(String message) {
			super(CODE_INVALID_REQUEST, "liter-llm: invalid request: " + message);
		}

		/** Creates an invalid-request exception with a cause. */
		public InvalidRequestException(String message, Throwable cause) {
			super(CODE_INVALID_REQUEST, "liter-llm: invalid request: " + message, cause);
		}
	}

	/**
	 * Thrown when the provider rejects the API key.
	 *
	 * <p>
	 * Corresponds to HTTP 401 / 403 from the provider.
	 */
	public static final class AuthenticationException extends LlmException {

		/** Creates an authentication exception. */
		public AuthenticationException(String message) {
			super(CODE_AUTHENTICATION, "liter-llm: authentication failed: " + message);
		}
	}

	/**
	 * Thrown when the provider rate-limits the request.
	 *
	 * <p>
	 * Corresponds to HTTP 429 from the provider.
	 */
	public static final class RateLimitException extends LlmException {

		/** Creates a rate-limit exception. */
		public RateLimitException(String message) {
			super(CODE_RATE_LIMIT, "liter-llm: rate limit exceeded: " + message);
		}
	}

	/**
	 * Thrown when the requested model or resource does not exist.
	 *
	 * <p>
	 * Corresponds to HTTP 404 from the provider.
	 */
	public static final class NotFoundException extends LlmException {

		/** Creates a not-found exception. */
		public NotFoundException(String message) {
			super(CODE_NOT_FOUND, "liter-llm: not found: " + message);
		}
	}

	/**
	 * Thrown when the provider returns a 5xx server error.
	 *
	 * <p>
	 * Corresponds to HTTP 5xx from the provider.
	 */
	public static final class ProviderException extends LlmException {

		private final int httpStatus;

		/** Creates a provider exception with HTTP status and message. */
		public ProviderException(int httpStatus, String message) {
			super(CODE_PROVIDER_ERROR, "liter-llm: provider error " + httpStatus + ": " + message);
			this.httpStatus = httpStatus;
		}

		/**
		 * Returns the HTTP status code returned by the provider.
		 *
		 * @return HTTP status code
		 */
		public int getHttpStatus() {
			return httpStatus;
		}
	}

	/** Thrown when a streaming response cannot be parsed. */
	public static final class StreamException extends LlmException {

		/** Creates a stream-parse exception. */
		public StreamException(String message) {
			super(CODE_STREAM_ERROR, "liter-llm: stream error: " + message);
		}

		/** Creates a stream-parse exception with a cause. */
		public StreamException(String message, Throwable cause) {
			super(CODE_STREAM_ERROR, "liter-llm: stream error: " + message, cause);
		}
	}

	/** Thrown when JSON serialization or deserialization fails. */
	public static final class SerializationException extends LlmException {

		/** Creates a serialization exception. */
		public SerializationException(String message, Throwable cause) {
			super(CODE_SERIALIZATION, "liter-llm: serialization error: " + message, cause);
		}
	}
}
