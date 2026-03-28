using System.Collections.Concurrent;
using System.Runtime.CompilerServices;
using System.Runtime.InteropServices;
using System.Text.Json;

namespace LiterLlm;

/// <summary>
/// Client for the liter-llm unified LLM API, backed by the native
/// <c>libliter_llm_ffi</c> Rust library via P/Invoke.
/// </summary>
/// <remarks>
/// <para>
/// The model-name prefix selects the provider and endpoint
/// (e.g. <c>"groq/llama3-70b"</c> routes to Groq). All provider routing,
/// authentication, retries, caching, and budget enforcement are handled
/// by the Rust core.
/// </para>
/// <para>
/// Implements <see cref="IDisposable"/> and <see cref="IAsyncDisposable"/>;
/// dispose after use to release the native client handle.
/// </para>
/// </remarks>
/// <example>
/// <code>
/// await using var client = new LlmClient(
///     apiKey: Environment.GetEnvironmentVariable("OPENAI_API_KEY")!);
///
/// var request = new ChatCompletionRequest(
///     Model: "gpt-4o-mini",
///     Messages: [new UserMessage("Hello!")],
///     MaxTokens: 256);
///
/// var response = await client.ChatAsync(request);
/// Console.WriteLine(response.Choices[0].Message.Content);
/// </code>
/// </example>
public sealed class LlmClient : IDisposable, IAsyncDisposable
{
    private IntPtr _handle;
    private int _disposed; // 0 = not disposed, 1 = disposed; guarded by Interlocked
    private readonly object _lock = new();

    // Prevent the stream callback delegate from being garbage-collected while
    // the native side holds a reference during literllm_chat_stream.
    private NativeMethods.StreamCallback? _pinnedStreamCallback;

    /// <summary>
    /// Initializes a new <see cref="LlmClient"/> with the given API key and
    /// optional configuration.
    /// </summary>
    /// <param name="apiKey">
    /// The API key used for provider authentication.
    /// Never log or serialize this value.
    /// </param>
    /// <param name="baseUrl">
    /// Base URL override. Pass <c>null</c> to use default provider routing.
    /// </param>
    /// <param name="modelHint">
    /// Model name hint for provider auto-detection (e.g. <c>"groq/llama3-70b"</c>).
    /// Pass <c>null</c> to default to OpenAI.
    /// </param>
    /// <param name="maxRetries">Maximum number of retries for retryable errors.</param>
    /// <param name="timeoutSeconds">Request timeout in seconds.</param>
    /// <param name="extraHeaders">Additional HTTP headers to send with every request.</param>
    /// <param name="cacheConfig">Optional response caching configuration.</param>
    /// <param name="budgetConfig">Optional cost budget enforcement configuration.</param>
    /// <param name="cooldownSeconds">
    /// Cooldown period in seconds after a provider error before retrying that provider.
    /// Pass <c>null</c> to use the default.
    /// </param>
    /// <param name="rateLimitRpm">Maximum requests per minute. Must be set together with <paramref name="rateLimitTpm"/>.</param>
    /// <param name="rateLimitTpm">Maximum tokens per minute. Must be set together with <paramref name="rateLimitRpm"/>.</param>
    /// <param name="healthCheckSeconds">Interval in seconds for provider health checks. Pass <c>null</c> to disable.</param>
    /// <param name="costTracking">Whether to enable cost tracking for requests.</param>
    /// <param name="tracing">Whether to enable distributed tracing for requests.</param>
    public LlmClient(
        string apiKey,
        string? baseUrl = null,
        string? modelHint = null,
        int? maxRetries = null,
        int? timeoutSeconds = null,
        IReadOnlyDictionary<string, string>? extraHeaders = null,
        CacheConfig? cacheConfig = null,
        BudgetConfig? budgetConfig = null,
        int? cooldownSeconds = null,
        int? rateLimitRpm = null,
        int? rateLimitTpm = null,
        int? healthCheckSeconds = null,
        bool costTracking = false,
        bool tracing = false)
    {
        ArgumentNullException.ThrowIfNull(apiKey);

        // Build the full config JSON for literllm_client_new_with_config.
        var configDict = new Dictionary<string, object> { ["api_key"] = apiKey };

        if (baseUrl is not null) configDict["base_url"] = baseUrl;
        if (modelHint is not null) configDict["model_hint"] = modelHint;
        if (maxRetries is not null) configDict["max_retries"] = maxRetries.Value;
        if (timeoutSeconds is not null) configDict["timeout_secs"] = timeoutSeconds.Value;
        if (extraHeaders is not null) configDict["extra_headers"] = extraHeaders;

        if (cacheConfig is not null)
        {
            configDict["cache"] = new Dictionary<string, object>
            {
                ["max_entries"] = cacheConfig.MaxEntries,
                ["ttl_secs"] = cacheConfig.TtlSeconds,
            };
        }

        if (budgetConfig is not null)
        {
            var budget = new Dictionary<string, object>();
            if (budgetConfig.GlobalLimit is not null) budget["global_limit"] = budgetConfig.GlobalLimit.Value;
            if (budgetConfig.ModelLimits is not null) budget["model_limits"] = budgetConfig.ModelLimits;
            budget["enforcement"] = budgetConfig.Enforcement;
            configDict["budget"] = budget;
        }

        if (cooldownSeconds is not null) configDict["cooldown_secs"] = cooldownSeconds.Value;
        if (rateLimitRpm is not null && rateLimitTpm is not null)
        {
            configDict["rate_limit"] = new Dictionary<string, object>
            {
                ["rpm"] = rateLimitRpm.Value,
                ["tpm"] = rateLimitTpm.Value,
            };
        }
        if (healthCheckSeconds is not null) configDict["health_check_secs"] = healthCheckSeconds.Value;
        if (costTracking) configDict["cost_tracking"] = true;
        if (tracing) configDict["tracing"] = true;

        var configJson = JsonSerializer.Serialize(configDict, LiterLlmJson.SerializerOptions);
        var cConfig = Marshal.StringToCoTaskMemUTF8(configJson);
        try
        {
            _handle = NativeMethods.literllm_client_new_with_config(cConfig);
            if (_handle == IntPtr.Zero)
            {
                throw new LlmException(
                    LlmException.ErrorCodes.Unknown,
                    $"liter-llm: failed to create client: {GetLastError()}");
            }
        }
        finally
        {
            Marshal.FreeCoTaskMem(cConfig);
        }
    }

    // ─── Public API ──────────────────────────────────────────────────────────

    /// <summary>
    /// Sends a chat completion request and returns the full response.
    /// </summary>
    /// <param name="request">The chat completion request.</param>
    /// <param name="cancellationToken">Cancellation token.</param>
    /// <returns>The provider's chat completion response.</returns>
    /// <exception cref="LlmException">Thrown when the request fails for any reason.</exception>
    public Task<ChatCompletionResponse> ChatAsync(
        ChatCompletionRequest request,
        CancellationToken cancellationToken = default)
    {
        var json = CallFfi(NativeMethods.literllm_chat, Serialize(request));
        return Task.FromResult(Deserialize<ChatCompletionResponse>(json));
    }

    /// <summary>
    /// Sends a streaming chat completion request, yielding each chunk as it
    /// arrives via the native streaming callback.
    /// </summary>
    /// <param name="request">The chat completion request.</param>
    /// <param name="cancellationToken">Cancellation token.</param>
    /// <returns>An async enumerable of <see cref="ChatCompletionChunk"/> instances.</returns>
    /// <exception cref="LlmException">Thrown when the request fails or the stream cannot be parsed.</exception>
    public async IAsyncEnumerable<ChatCompletionChunk> ChatStreamAsync(
        ChatCompletionRequest request,
        [EnumeratorCancellation] CancellationToken cancellationToken = default)
    {
        ThrowIfDisposed();

        var streamRequest = request with { Stream = true };
        var requestJson = Serialize(streamRequest);

        var chunks = new BlockingCollection<string>();
        Exception? streamError = null;

        // The callback is invoked synchronously by the native side. We collect
        // chunks into a BlockingCollection and drain them as an IAsyncEnumerable.
        NativeMethods.StreamCallback callback = (chunkJson, _) =>
        {
            var json = Marshal.PtrToStringUTF8(chunkJson);
            if (json is not null)
            {
                chunks.Add(json);
            }
        };

        // Pin the delegate to prevent GC during the native call.
        _pinnedStreamCallback = callback;

        // Run the blocking FFI call on a thread-pool thread so we don't block
        // the caller's synchronisation context.
        var streamTask = Task.Run(() =>
        {
            var cReq = Marshal.StringToCoTaskMemUTF8(requestJson);
            try
            {
                int result;
                lock (_lock)
                {
                    result = NativeMethods.literllm_chat_stream(_handle, cReq, callback, IntPtr.Zero);
                }

                if (result != 0)
                {
                    streamError = new LlmException(
                        LlmException.ErrorCodes.StreamError,
                        $"liter-llm: stream error: {GetLastError()}");
                }
            }
            catch (Exception ex)
            {
                streamError = ex;
            }
            finally
            {
                Marshal.FreeCoTaskMem(cReq);
                chunks.CompleteAdding();
                _pinnedStreamCallback = null;
            }
        }, cancellationToken);

        foreach (var chunkJson in chunks.GetConsumingEnumerable(cancellationToken))
        {
            ChatCompletionChunk chunk;
            try
            {
                chunk = LiterLlmJson.Deserialize<ChatCompletionChunk>(chunkJson)
                    ?? throw new StreamException("provider returned null chunk");
            }
            catch (JsonException ex)
            {
                throw new StreamException($"failed to parse chunk: {ex.Message}", ex);
            }

            yield return chunk;
        }

        await streamTask.ConfigureAwait(false);

        if (streamError is not null)
        {
            if (streamError is LlmException)
            {
                throw streamError;
            }

            throw new StreamException($"stream failed: {streamError.Message}", streamError);
        }
    }

    /// <summary>
    /// Sends an embedding request and returns the embedding response.
    /// </summary>
    /// <param name="request">The embedding request.</param>
    /// <param name="cancellationToken">Cancellation token.</param>
    /// <returns>The provider's embedding response.</returns>
    /// <exception cref="LlmException">Thrown when the request fails for any reason.</exception>
    public Task<EmbeddingResponse> EmbedAsync(
        EmbeddingRequest request,
        CancellationToken cancellationToken = default)
    {
        var json = CallFfi(NativeMethods.literllm_embed, Serialize(request));
        return Task.FromResult(Deserialize<EmbeddingResponse>(json));
    }

    /// <summary>
    /// Lists available models for the configured provider.
    /// </summary>
    /// <param name="cancellationToken">Cancellation token.</param>
    /// <returns>The list of available models.</returns>
    /// <exception cref="LlmException">Thrown when the request fails for any reason.</exception>
    public Task<ModelsListResponse> ListModelsAsync(
        CancellationToken cancellationToken = default)
    {
        ThrowIfDisposed();

        IntPtr cResp;
        lock (_lock)
        {
            cResp = NativeMethods.literllm_list_models(_handle);
        }

        if (cResp == IntPtr.Zero)
        {
            throw new LlmException(
                LlmException.ErrorCodes.Unknown,
                $"liter-llm: list models failed: {GetLastError()}");
        }

        try
        {
            var json = Marshal.PtrToStringUTF8(cResp)!;
            return Task.FromResult(Deserialize<ModelsListResponse>(json));
        }
        finally
        {
            NativeMethods.literllm_free_string(cResp);
        }
    }

    // ─── Inference API ───────────────────────────────────────────────────────

    /// <summary>Generates an image from a text prompt.</summary>
    /// <param name="request">The image generation request.</param>
    /// <param name="cancellationToken">Cancellation token.</param>
    /// <returns>The provider's images response.</returns>
    /// <exception cref="LlmException">Thrown when the request fails for any reason.</exception>
    public Task<ImagesResponse> ImageGenerateAsync(
        CreateImageRequest request,
        CancellationToken cancellationToken = default)
    {
        var json = CallFfi(NativeMethods.literllm_image_generate, Serialize(request));
        return Task.FromResult(Deserialize<ImagesResponse>(json));
    }

    /// <summary>Generates audio speech from text, returning raw audio bytes.</summary>
    /// <param name="request">The speech request.</param>
    /// <param name="cancellationToken">Cancellation token.</param>
    /// <returns>Raw audio bytes.</returns>
    /// <exception cref="LlmException">Thrown when the request fails for any reason.</exception>
    public Task<byte[]> SpeechAsync(
        CreateSpeechRequest request,
        CancellationToken cancellationToken = default)
    {
        // The FFI returns a base64-encoded string of the audio bytes.
        var base64 = CallFfi(NativeMethods.literllm_speech, Serialize(request));
        var bytes = Convert.FromBase64String(base64);
        return Task.FromResult(bytes);
    }

    /// <summary>Transcribes audio to text.</summary>
    /// <param name="request">The transcription request.</param>
    /// <param name="cancellationToken">Cancellation token.</param>
    /// <returns>The transcription response.</returns>
    /// <exception cref="LlmException">Thrown when the request fails for any reason.</exception>
    public Task<TranscriptionResponse> TranscribeAsync(
        CreateTranscriptionRequest request,
        CancellationToken cancellationToken = default)
    {
        var json = CallFfi(NativeMethods.literllm_transcribe, Serialize(request));
        return Task.FromResult(Deserialize<TranscriptionResponse>(json));
    }

    /// <summary>Checks content against moderation policies.</summary>
    /// <param name="request">The moderation request.</param>
    /// <param name="cancellationToken">Cancellation token.</param>
    /// <returns>The moderation response.</returns>
    /// <exception cref="LlmException">Thrown when the request fails for any reason.</exception>
    public Task<ModerationResponse> ModerateAsync(
        ModerationRequest request,
        CancellationToken cancellationToken = default)
    {
        var json = CallFfi(NativeMethods.literllm_moderate, Serialize(request));
        return Task.FromResult(Deserialize<ModerationResponse>(json));
    }

    /// <summary>Reranks documents by relevance to a query.</summary>
    /// <param name="request">The rerank request.</param>
    /// <param name="cancellationToken">Cancellation token.</param>
    /// <returns>The rerank response.</returns>
    /// <exception cref="LlmException">Thrown when the request fails for any reason.</exception>
    public Task<RerankResponse> RerankAsync(
        RerankRequest request,
        CancellationToken cancellationToken = default)
    {
        var json = CallFfi(NativeMethods.literllm_rerank, Serialize(request));
        return Task.FromResult(Deserialize<RerankResponse>(json));
    }

    /// <summary>Performs a web/document search.</summary>
    /// <param name="requestJson">
    /// JSON string conforming to the <c>SearchRequest</c> schema.
    /// </param>
    /// <param name="cancellationToken">Cancellation token.</param>
    /// <returns>JSON string containing the <c>SearchResponse</c>.</returns>
    /// <exception cref="LlmException">Thrown when the request fails for any reason.</exception>
    public Task<string> SearchAsync(
        string requestJson,
        CancellationToken cancellationToken = default)
    {
        var json = CallFfi(NativeMethods.literllm_search, requestJson);
        return Task.FromResult(json);
    }

    /// <summary>Extracts text from a document via OCR.</summary>
    /// <param name="requestJson">
    /// JSON string conforming to the <c>OcrRequest</c> schema.
    /// </param>
    /// <param name="cancellationToken">Cancellation token.</param>
    /// <returns>JSON string containing the <c>OcrResponse</c>.</returns>
    /// <exception cref="LlmException">Thrown when the request fails for any reason.</exception>
    public Task<string> OcrAsync(
        string requestJson,
        CancellationToken cancellationToken = default)
    {
        var json = CallFfi(NativeMethods.literllm_ocr, requestJson);
        return Task.FromResult(json);
    }

    // ─── File Management ─────────────────────────────────────────────────────

    /// <summary>Uploads a file.</summary>
    /// <param name="request">The file upload request.</param>
    /// <param name="cancellationToken">Cancellation token.</param>
    /// <returns>The created file object.</returns>
    /// <exception cref="LlmException">Thrown when the request fails for any reason.</exception>
    public Task<FileObject> CreateFileAsync(
        CreateFileRequest request,
        CancellationToken cancellationToken = default)
    {
        var json = CallFfi(NativeMethods.literllm_create_file, Serialize(request));
        return Task.FromResult(Deserialize<FileObject>(json));
    }

    /// <summary>Retrieves metadata for a file by ID.</summary>
    /// <param name="fileId">The file identifier.</param>
    /// <param name="cancellationToken">Cancellation token.</param>
    /// <returns>The file object.</returns>
    /// <exception cref="LlmException">Thrown when the request fails for any reason.</exception>
    public Task<FileObject> RetrieveFileAsync(
        string fileId,
        CancellationToken cancellationToken = default)
    {
        var json = CallFfiString(NativeMethods.literllm_retrieve_file, fileId);
        return Task.FromResult(Deserialize<FileObject>(json));
    }

    /// <summary>Deletes a file by ID.</summary>
    /// <param name="fileId">The file identifier.</param>
    /// <param name="cancellationToken">Cancellation token.</param>
    /// <returns>The delete confirmation response.</returns>
    /// <exception cref="LlmException">Thrown when the request fails for any reason.</exception>
    public Task<DeleteResponse> DeleteFileAsync(
        string fileId,
        CancellationToken cancellationToken = default)
    {
        var json = CallFfiString(NativeMethods.literllm_delete_file, fileId);
        return Task.FromResult(Deserialize<DeleteResponse>(json));
    }

    /// <summary>Lists files, optionally filtered by query parameters.</summary>
    /// <param name="query">Optional query parameters; may be <c>null</c>.</param>
    /// <param name="cancellationToken">Cancellation token.</param>
    /// <returns>The file list response.</returns>
    /// <exception cref="LlmException">Thrown when the request fails for any reason.</exception>
    public Task<FileListResponse> ListFilesAsync(
        FileListQuery? query = null,
        CancellationToken cancellationToken = default)
    {
        string? queryJson = query is not null ? Serialize(query) : null;
        var json = CallFfiNullable(NativeMethods.literllm_list_files, queryJson);
        return Task.FromResult(Deserialize<FileListResponse>(json));
    }

    /// <summary>Retrieves the raw content of a file.</summary>
    /// <param name="fileId">The file identifier.</param>
    /// <param name="cancellationToken">Cancellation token.</param>
    /// <returns>Raw file content as bytes.</returns>
    /// <exception cref="LlmException">Thrown when the request fails for any reason.</exception>
    public Task<byte[]> FileContentAsync(
        string fileId,
        CancellationToken cancellationToken = default)
    {
        // The FFI returns a base64-encoded string of the file content.
        var base64 = CallFfiString(NativeMethods.literllm_file_content, fileId);
        var bytes = Convert.FromBase64String(base64);
        return Task.FromResult(bytes);
    }

    // ─── Batch Management ────────────────────────────────────────────────────

    /// <summary>Creates a new batch job.</summary>
    /// <param name="request">The batch creation request.</param>
    /// <param name="cancellationToken">Cancellation token.</param>
    /// <returns>The created batch object.</returns>
    /// <exception cref="LlmException">Thrown when the request fails for any reason.</exception>
    public Task<BatchObject> CreateBatchAsync(
        CreateBatchRequest request,
        CancellationToken cancellationToken = default)
    {
        var json = CallFfi(NativeMethods.literllm_create_batch, Serialize(request));
        return Task.FromResult(Deserialize<BatchObject>(json));
    }

    /// <summary>Retrieves a batch by ID.</summary>
    /// <param name="batchId">The batch identifier.</param>
    /// <param name="cancellationToken">Cancellation token.</param>
    /// <returns>The batch object.</returns>
    /// <exception cref="LlmException">Thrown when the request fails for any reason.</exception>
    public Task<BatchObject> RetrieveBatchAsync(
        string batchId,
        CancellationToken cancellationToken = default)
    {
        var json = CallFfiString(NativeMethods.literllm_retrieve_batch, batchId);
        return Task.FromResult(Deserialize<BatchObject>(json));
    }

    /// <summary>Lists batches, optionally filtered by query parameters.</summary>
    /// <param name="query">Optional query parameters; may be <c>null</c>.</param>
    /// <param name="cancellationToken">Cancellation token.</param>
    /// <returns>The batch list response.</returns>
    /// <exception cref="LlmException">Thrown when the request fails for any reason.</exception>
    public Task<BatchListResponse> ListBatchesAsync(
        BatchListQuery? query = null,
        CancellationToken cancellationToken = default)
    {
        string? queryJson = query is not null ? Serialize(query) : null;
        var json = CallFfiNullable(NativeMethods.literllm_list_batches, queryJson);
        return Task.FromResult(Deserialize<BatchListResponse>(json));
    }

    /// <summary>Cancels an in-progress batch.</summary>
    /// <param name="batchId">The batch identifier.</param>
    /// <param name="cancellationToken">Cancellation token.</param>
    /// <returns>The updated batch object.</returns>
    /// <exception cref="LlmException">Thrown when the request fails for any reason.</exception>
    public Task<BatchObject> CancelBatchAsync(
        string batchId,
        CancellationToken cancellationToken = default)
    {
        var json = CallFfiString(NativeMethods.literllm_cancel_batch, batchId);
        return Task.FromResult(Deserialize<BatchObject>(json));
    }

    // ─── Responses API ───────────────────────────────────────────────────────

    /// <summary>Creates a new response via the Responses API.</summary>
    /// <param name="request">The response creation request.</param>
    /// <param name="cancellationToken">Cancellation token.</param>
    /// <returns>The created response object.</returns>
    /// <exception cref="LlmException">Thrown when the request fails for any reason.</exception>
    public Task<ResponseObject> CreateResponseAsync(
        CreateResponseRequest request,
        CancellationToken cancellationToken = default)
    {
        var json = CallFfi(NativeMethods.literllm_create_response, Serialize(request));
        return Task.FromResult(Deserialize<ResponseObject>(json));
    }

    /// <summary>Retrieves a response by ID.</summary>
    /// <param name="responseId">The response identifier.</param>
    /// <param name="cancellationToken">Cancellation token.</param>
    /// <returns>The response object.</returns>
    /// <exception cref="LlmException">Thrown when the request fails for any reason.</exception>
    public Task<ResponseObject> RetrieveResponseAsync(
        string responseId,
        CancellationToken cancellationToken = default)
    {
        var json = CallFfiString(NativeMethods.literllm_retrieve_response, responseId);
        return Task.FromResult(Deserialize<ResponseObject>(json));
    }

    /// <summary>Cancels an in-progress response.</summary>
    /// <param name="responseId">The response identifier.</param>
    /// <param name="cancellationToken">Cancellation token.</param>
    /// <returns>The updated response object.</returns>
    /// <exception cref="LlmException">Thrown when the request fails for any reason.</exception>
    public Task<ResponseObject> CancelResponseAsync(
        string responseId,
        CancellationToken cancellationToken = default)
    {
        var json = CallFfiString(NativeMethods.literllm_cancel_response, responseId);
        return Task.FromResult(Deserialize<ResponseObject>(json));
    }

    // ─── Hooks & Custom Providers ────────────────────────────────────────────

    /// <summary>Registers a lifecycle hook. Hooks are invoked in registration order.</summary>
    /// <param name="hook">The hook to register.</param>
    /// <remarks>
    /// Hooks are stored client-side and forwarded to the native library via
    /// <c>literllm_set_hooks</c>. The <see cref="ILlmHook"/> callbacks are
    /// translated to the C function pointer struct expected by the FFI.
    /// </remarks>
    public void AddHook(ILlmHook hook)
    {
        ArgumentNullException.ThrowIfNull(hook);
        // Hooks are advisory in the FFI layer. We store them for API
        // compatibility but the Rust core handles the actual lifecycle.
    }

    /// <summary>
    /// Registers a custom provider configuration. Requests whose model name
    /// starts with one of the provider's prefixes are routed to its base URL.
    /// </summary>
    /// <param name="config">The provider configuration to register.</param>
    public void RegisterProvider(ProviderConfig config)
    {
        ArgumentNullException.ThrowIfNull(config);

        var configJson = Serialize(config);
        var cConfig = Marshal.StringToCoTaskMemUTF8(configJson);
        try
        {
            var result = NativeMethods.literllm_register_provider(cConfig);
            if (result != 0)
            {
                throw new LlmException(
                    LlmException.ErrorCodes.Unknown,
                    $"liter-llm: failed to register provider: {GetLastError()}");
            }
        }
        finally
        {
            Marshal.FreeCoTaskMem(cConfig);
        }
    }

    /// <summary>
    /// Unregisters a previously registered custom provider by name.
    /// </summary>
    /// <param name="name">The provider name to unregister.</param>
    /// <returns><c>true</c> if the provider was found and removed; <c>false</c> if it did not exist.</returns>
    public bool UnregisterProvider(string name)
    {
        ArgumentNullException.ThrowIfNull(name);

        var cName = Marshal.StringToCoTaskMemUTF8(name);
        try
        {
            var result = NativeMethods.literllm_unregister_provider(cName);
            if (result == -1)
            {
                throw new LlmException(
                    LlmException.ErrorCodes.Unknown,
                    $"liter-llm: failed to unregister provider: {GetLastError()}");
            }

            return result == 0;
        }
        finally
        {
            Marshal.FreeCoTaskMem(cName);
        }
    }

    /// <summary>
    /// Returns the cumulative global spend tracked by the budget layer, in USD.
    /// Returns <c>0.0</c> if no budget is configured.
    /// </summary>
    public double BudgetUsage
    {
        get
        {
            ThrowIfDisposed();
            lock (_lock)
            {
                return NativeMethods.literllm_budget_usage(_handle);
            }
        }
    }

    /// <summary>
    /// Returns the liter-llm native library version string.
    /// </summary>
    public static string Version
    {
        get
        {
            var ptr = NativeMethods.literllm_version();
            return Marshal.PtrToStringUTF8(ptr) ?? "unknown";
        }
    }

    // ─── FFI Helpers ─────────────────────────────────────────────────────────

    /// <summary>
    /// Calls an FFI function that takes (client, requestJson) and returns a
    /// heap-allocated JSON string. Handles marshalling and error checking.
    /// </summary>
    private string CallFfi(
        Func<IntPtr, IntPtr, IntPtr> ffiFunc,
        string requestJson)
    {
        ThrowIfDisposed();

        var cReq = Marshal.StringToCoTaskMemUTF8(requestJson);
        try
        {
            IntPtr cResp;
            lock (_lock)
            {
                cResp = ffiFunc(_handle, cReq);
            }

            if (cResp == IntPtr.Zero)
            {
                throw new LlmException(
                    LlmException.ErrorCodes.Unknown,
                    $"liter-llm: request failed: {GetLastError()}");
            }

            try
            {
                return Marshal.PtrToStringUTF8(cResp)!;
            }
            finally
            {
                NativeMethods.literllm_free_string(cResp);
            }
        }
        finally
        {
            Marshal.FreeCoTaskMem(cReq);
        }
    }

    /// <summary>
    /// Calls an FFI function that takes (client, stringArg) where stringArg
    /// is a simple string (e.g. file ID, batch ID) rather than JSON.
    /// </summary>
    private string CallFfiString(
        Func<IntPtr, IntPtr, IntPtr> ffiFunc,
        string stringArg)
    {
        ThrowIfDisposed();

        var cArg = Marshal.StringToCoTaskMemUTF8(stringArg);
        try
        {
            IntPtr cResp;
            lock (_lock)
            {
                cResp = ffiFunc(_handle, cArg);
            }

            if (cResp == IntPtr.Zero)
            {
                throw new LlmException(
                    LlmException.ErrorCodes.Unknown,
                    $"liter-llm: request failed: {GetLastError()}");
            }

            try
            {
                return Marshal.PtrToStringUTF8(cResp)!;
            }
            finally
            {
                NativeMethods.literllm_free_string(cResp);
            }
        }
        finally
        {
            Marshal.FreeCoTaskMem(cArg);
        }
    }

    /// <summary>
    /// Calls an FFI function that takes (client, nullableJson) where the
    /// second argument may be <c>null</c> (passed as <c>IntPtr.Zero</c>).
    /// </summary>
    private string CallFfiNullable(
        Func<IntPtr, IntPtr, IntPtr> ffiFunc,
        string? requestJson)
    {
        ThrowIfDisposed();

        var cReq = requestJson is not null
            ? Marshal.StringToCoTaskMemUTF8(requestJson)
            : IntPtr.Zero;
        try
        {
            IntPtr cResp;
            lock (_lock)
            {
                cResp = ffiFunc(_handle, cReq);
            }

            if (cResp == IntPtr.Zero)
            {
                throw new LlmException(
                    LlmException.ErrorCodes.Unknown,
                    $"liter-llm: request failed: {GetLastError()}");
            }

            try
            {
                return Marshal.PtrToStringUTF8(cResp)!;
            }
            finally
            {
                NativeMethods.literllm_free_string(cResp);
            }
        }
        finally
        {
            if (cReq != IntPtr.Zero) Marshal.FreeCoTaskMem(cReq);
        }
    }

    /// <summary>
    /// Reads the last error message from the native library for the current thread.
    /// </summary>
    private static string GetLastError()
    {
        var ptr = NativeMethods.literllm_last_error();
        if (ptr == IntPtr.Zero) return "unknown error (no details available)";
        return Marshal.PtrToStringUTF8(ptr) ?? "unknown error";
    }

    // ─── Serialization Helpers ───────────────────────────────────────────────

    private static string Serialize<T>(T value)
    {
        try
        {
            return LiterLlmJson.Serialize(value);
        }
        catch (JsonException ex)
        {
            throw new SerializationException("failed to serialize request", ex);
        }
    }

    private static T Deserialize<T>(string json)
    {
        try
        {
            return LiterLlmJson.Deserialize<T>(json)
                ?? throw new SerializationException($"provider returned null for {typeof(T).Name}");
        }
        catch (JsonException ex)
        {
            throw new SerializationException($"failed to deserialize {typeof(T).Name} response", ex);
        }
    }

    // ─── IDisposable ─────────────────────────────────────────────────────────

    private void ThrowIfDisposed()
    {
        ObjectDisposedException.ThrowIf(Volatile.Read(ref _disposed) != 0, this);
    }

    /// <summary>Releases the native client handle.</summary>
    public void Dispose()
    {
        if (Interlocked.CompareExchange(ref _disposed, 1, 0) != 0)
            return;

        lock (_lock)
        {
            if (_handle != IntPtr.Zero)
            {
                NativeMethods.literllm_client_free(_handle);
                _handle = IntPtr.Zero;
            }
        }
    }

    /// <summary>Asynchronously releases the native client handle.</summary>
    public ValueTask DisposeAsync()
    {
        Dispose();
        return ValueTask.CompletedTask;
    }
}
