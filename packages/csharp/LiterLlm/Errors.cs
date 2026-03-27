namespace LiterLlm;

/// <summary>
/// Base exception for all liter-llm client errors.
/// </summary>
/// <remarks>
/// All exceptions thrown by <see cref="LlmClient"/> derive from this class.
/// Use <see cref="ErrorCode"/> for programmatic error handling.
/// </remarks>
public class LlmException : Exception
{
    /// <summary>Numeric error codes (1000+) identifying error categories.</summary>
    public static class ErrorCodes
    {
        /// <summary>Unknown or unclassified error.</summary>
        public const int Unknown = 1000;

        /// <summary>The request body was malformed or missing required fields.</summary>
        public const int InvalidRequest = 1400;

        /// <summary>The API key was rejected by the provider.</summary>
        public const int Authentication = 1401;

        /// <summary>The provider rate-limited the request.</summary>
        public const int RateLimit = 1429;

        /// <summary>The requested model or resource does not exist.</summary>
        public const int NotFound = 1404;

        /// <summary>The provider returned a 5xx server error.</summary>
        public const int ProviderError = 1500;

        /// <summary>A streaming response could not be parsed.</summary>
        public const int StreamError = 1600;

        /// <summary>JSON serialization or deserialization failed.</summary>
        public const int Serialization = 1700;

        /// <summary>A request would exceed the configured cost budget.</summary>
        public const int BudgetExceeded = 1800;

        /// <summary>A hook rejected the request.</summary>
        public const int HookRejected = 1801;
    }

    /// <summary>Gets the numeric error code identifying the category of this error.</summary>
    public int ErrorCode { get; }

    /// <summary>
    /// Initializes a new instance of <see cref="LlmException"/>.
    /// </summary>
    /// <param name="errorCode">Numeric code (1000+) identifying the error category.</param>
    /// <param name="message">Human-readable error description.</param>
    /// <param name="inner">The underlying exception, if any.</param>
    public LlmException(int errorCode, string message, Exception? inner = null)
        : base(message, inner)
    {
        ErrorCode = errorCode;
    }
}

/// <summary>
/// Thrown when the provider rejects the request due to invalid input.
/// </summary>
/// <remarks>Corresponds to HTTP 400 / 422 from the provider.</remarks>
public sealed class InvalidRequestException : LlmException
{
    /// <summary>
    /// Initializes a new instance of <see cref="InvalidRequestException"/>.
    /// </summary>
    public InvalidRequestException(string message, Exception? inner = null)
        : base(LlmException.ErrorCodes.InvalidRequest,
               $"liter-llm: invalid request: {message}", inner)
    { }
}

/// <summary>
/// Thrown when the provider rejects the API key.
/// </summary>
/// <remarks>Corresponds to HTTP 401 / 403 from the provider.</remarks>
public sealed class AuthenticationException : LlmException
{
    /// <summary>
    /// Initializes a new instance of <see cref="AuthenticationException"/>.
    /// </summary>
    public AuthenticationException(string message)
        : base(LlmException.ErrorCodes.Authentication,
               $"liter-llm: authentication failed: {message}")
    { }
}

/// <summary>
/// Thrown when the provider rate-limits the request.
/// </summary>
/// <remarks>Corresponds to HTTP 429 from the provider.</remarks>
public sealed class RateLimitException : LlmException
{
    /// <summary>
    /// Initializes a new instance of <see cref="RateLimitException"/>.
    /// </summary>
    public RateLimitException(string message)
        : base(LlmException.ErrorCodes.RateLimit,
               $"liter-llm: rate limit exceeded: {message}")
    { }
}

/// <summary>
/// Thrown when the requested model or resource does not exist.
/// </summary>
/// <remarks>Corresponds to HTTP 404 from the provider.</remarks>
public sealed class NotFoundException : LlmException
{
    /// <summary>
    /// Initializes a new instance of <see cref="NotFoundException"/>.
    /// </summary>
    public NotFoundException(string message)
        : base(LlmException.ErrorCodes.NotFound,
               $"liter-llm: not found: {message}")
    { }
}

/// <summary>
/// Thrown when the provider returns a server error.
/// </summary>
/// <remarks>Corresponds to HTTP 5xx from the provider.</remarks>
public sealed class ProviderException : LlmException
{
    /// <summary>Gets the HTTP status code returned by the provider.</summary>
    public int HttpStatus { get; }

    /// <summary>
    /// Initializes a new instance of <see cref="ProviderException"/>.
    /// </summary>
    public ProviderException(int httpStatus, string message)
        : base(LlmException.ErrorCodes.ProviderError,
               $"liter-llm: provider error {httpStatus}: {message}")
    {
        HttpStatus = httpStatus;
    }
}

/// <summary>
/// Thrown when a streaming response cannot be parsed.
/// </summary>
public sealed class StreamException : LlmException
{
    /// <summary>
    /// Initializes a new instance of <see cref="StreamException"/>.
    /// </summary>
    public StreamException(string message, Exception? inner = null)
        : base(LlmException.ErrorCodes.StreamError,
               $"liter-llm: stream error: {message}", inner)
    { }
}

/// <summary>
/// Thrown when JSON serialization or deserialization fails.
/// </summary>
public sealed class SerializationException : LlmException
{
    /// <summary>
    /// Initializes a new instance of <see cref="SerializationException"/>.
    /// </summary>
    public SerializationException(string message, Exception? inner = null)
        : base(LlmException.ErrorCodes.Serialization,
               $"liter-llm: serialization error: {message}", inner)
    { }
}

/// <summary>
/// Thrown when a request would exceed the configured cost budget.
/// </summary>
/// <seealso cref="BudgetConfig"/>
public sealed class BudgetExceededException : LlmException
{
    /// <summary>Error code for budget exceeded errors.</summary>
    public const int Code = 1800;

    /// <summary>
    /// Initializes a new instance of <see cref="BudgetExceededException"/>.
    /// </summary>
    public BudgetExceededException(string message)
        : base(Code, $"liter-llm: budget exceeded: {message}")
    { }
}

/// <summary>
/// Thrown when an <see cref="ILlmHook.OnRequestAsync"/> rejects a request.
/// </summary>
public sealed class HookRejectedException : LlmException
{
    /// <summary>Error code for hook rejection errors.</summary>
    public const int Code = 1801;

    /// <summary>
    /// Initializes a new instance of <see cref="HookRejectedException"/>.
    /// </summary>
    public HookRejectedException(string message, Exception? inner = null)
        : base(Code, $"liter-llm: hook rejected request: {message}", inner)
    { }
}
