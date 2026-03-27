namespace LiterLlm;

/// <summary>
/// Lifecycle hook interface for intercepting LLM client request/response events.
/// </summary>
/// <remarks>
/// Implement this interface and register it via <see cref="LlmClient.AddHook"/>.
/// Hooks are invoked in registration order, synchronously on the calling thread.
/// </remarks>
public interface ILlmHook
{
    /// <summary>
    /// Called before the HTTP request is sent.
    /// </summary>
    /// <remarks>
    /// Throw <see cref="HookRejectedException"/> to abort the request.
    /// The default implementation does nothing.
    /// </remarks>
    /// <param name="request">The request object about to be sent.</param>
    /// <param name="cancellationToken">Cancellation token.</param>
    /// <returns>A task representing the asynchronous operation.</returns>
    Task OnRequestAsync(object request, CancellationToken cancellationToken = default)
        => Task.CompletedTask;

    /// <summary>
    /// Called after a successful response is received.
    /// </summary>
    /// <param name="request">The original request.</param>
    /// <param name="response">The response received from the provider.</param>
    /// <param name="cancellationToken">Cancellation token.</param>
    /// <returns>A task representing the asynchronous operation.</returns>
    Task OnResponseAsync(object request, object response, CancellationToken cancellationToken = default)
        => Task.CompletedTask;

    /// <summary>
    /// Called after a request fails.
    /// </summary>
    /// <param name="request">The original request.</param>
    /// <param name="error">The exception that caused the failure.</param>
    /// <param name="cancellationToken">Cancellation token.</param>
    /// <returns>A task representing the asynchronous operation.</returns>
    Task OnErrorAsync(object request, Exception error, CancellationToken cancellationToken = default)
        => Task.CompletedTask;
}
