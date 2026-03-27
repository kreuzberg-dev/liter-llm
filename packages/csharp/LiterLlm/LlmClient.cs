using System.Collections.Concurrent;
using System.Net;
using System.Net.Http.Headers;
using System.Security.Cryptography;
using System.Text;
using System.Text.Json;

namespace LiterLlm;

/// <summary>
/// HTTP client for the liter-llm unified LLM API.
/// </summary>
/// <remarks>
/// <para>
/// Speaks the OpenAI-compatible wire protocol directly — no FFI, no native libraries.
/// The model-name prefix selects the provider and endpoint
/// (e.g. <c>"groq/llama3-70b"</c> routes to Groq).
/// </para>
/// <para>
/// Implements <see cref="IDisposable"/>; dispose after use to release the underlying
/// <see cref="HttpClient"/>.
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
    internal const string DefaultBaseUrl = "https://api.openai.com/v1";
    internal const int DefaultMaxRetries = 2;

    private readonly HttpClient _httpClient;
    private readonly int _maxRetries;
    private readonly CacheConfig? _cacheConfig;
    private readonly BudgetConfig? _budgetConfig;
    private readonly List<ILlmHook> _hooks = [];
    private readonly List<ProviderConfig> _customProviders = [];

    // ─── Cache State ──────────────────────────────────────────────────────────

    private sealed class CacheEntry(string response)
    {
        public string Response { get; } = response;
        public long CreatedTicks { get; } = Environment.TickCount64;

        public bool IsExpired(int ttlSeconds) =>
            (Environment.TickCount64 - CreatedTicks) > (long)ttlSeconds * 1000;
    }

    /// <summary>Thread-safe LRU-ish cache backed by ConcurrentDictionary.</summary>
    private readonly ConcurrentDictionary<string, CacheEntry>? _responseCache;

    // ─── Budget State ─────────────────────────────────────────────────────────

    /// <summary>Global spend in microcents (1 USD = 100,000,000 microcents) for lock-free precision.</summary>
    private long _globalSpendMicrocents;

    /// <summary>Per-model spend in microcents.</summary>
    private readonly ConcurrentDictionary<string, long> _modelSpendMicrocents = new();

    private const long MicrocentsPerUsd = 100_000_000L;

    /// <summary>Approximate pricing: [promptPer1M, completionPer1M] in USD.</summary>
    private static readonly Dictionary<string, (double Prompt, double Completion)> ModelPricing = new()
    {
        ["gpt-4o"] = (2.50, 10.00),
        ["gpt-4o-mini"] = (0.15, 0.60),
        ["gpt-4-turbo"] = (10.00, 30.00),
        ["gpt-4"] = (30.00, 60.00),
        ["gpt-3.5-turbo"] = (0.50, 1.50),
        ["claude-3-opus"] = (15.00, 75.00),
        ["claude-3-sonnet"] = (3.00, 15.00),
        ["claude-3-haiku"] = (0.25, 1.25),
    };

    private static readonly (double Prompt, double Completion) FallbackPricing = (1.00, 2.00);

    /// <summary>
    /// Initializes a new <see cref="LlmClient"/> with the given API key.
    /// </summary>
    /// <param name="apiKey">
    /// The API key sent as <c>Authorization: Bearer &lt;key&gt;</c>.
    /// Never log or serialize this value.
    /// </param>
    /// <param name="baseUrl">
    /// Base URL for the API endpoint. Defaults to <c>https://api.openai.com/v1</c>.
    /// </param>
    /// <param name="maxRetries">
    /// Maximum number of retries for retryable errors (429, 5xx). Defaults to 2.
    /// </param>
    /// <param name="timeout">
    /// Request timeout. Defaults to 60 seconds.
    /// </param>
    public LlmClient(
        string apiKey,
        string baseUrl = DefaultBaseUrl,
        int maxRetries = DefaultMaxRetries,
        TimeSpan? timeout = null,
        CacheConfig? cacheConfig = null,
        BudgetConfig? budgetConfig = null)
    {
        ArgumentNullException.ThrowIfNull(apiKey);
        if (maxRetries < 0) throw new ArgumentOutOfRangeException(nameof(maxRetries), "must be >= 0");

        _cacheConfig = cacheConfig;
        _budgetConfig = budgetConfig;
        _responseCache = cacheConfig is not null ? new ConcurrentDictionary<string, CacheEntry>() : null;

        _maxRetries = maxRetries;
        var normalizedBase = baseUrl.TrimEnd('/');

        _httpClient = new HttpClient
        {
            BaseAddress = new Uri(normalizedBase + "/"),
            Timeout = timeout ?? TimeSpan.FromSeconds(60),
        };
        _httpClient.DefaultRequestHeaders.Authorization =
            new AuthenticationHeaderValue("Bearer", apiKey);
        _httpClient.DefaultRequestHeaders.Accept.Add(
            new MediaTypeWithQualityHeaderValue("application/json"));
    }

    // ─── Public API ───────────────────────────────────────────────────────────

    /// <summary>
    /// Sends a chat completion request and returns the full response.
    /// </summary>
    /// <param name="request">The chat completion request.</param>
    /// <param name="cancellationToken">Cancellation token.</param>
    /// <returns>The provider's chat completion response.</returns>
    /// <exception cref="LlmException">Thrown when the request fails for any reason.</exception>
    public async Task<ChatCompletionResponse> ChatAsync(
        ChatCompletionRequest request,
        CancellationToken cancellationToken = default)
    {
        CheckBudget(request.Model);
        await RunOnRequestAsync(request, cancellationToken).ConfigureAwait(false);
        try
        {
            var body = Serialize(request);

            // Check cache before HTTP call.
            var cached = CheckCache(body);
            if (cached is not null)
            {
                var cachedResponse = Deserialize<ChatCompletionResponse>(cached);
                await RunOnResponseAsync(request, cachedResponse, cancellationToken).ConfigureAwait(false);
                return cachedResponse;
            }

            var responseJson = await PostAsync("chat/completions", body, cancellationToken)
                .ConfigureAwait(false);
            var response = Deserialize<ChatCompletionResponse>(responseJson);

            // Store in cache.
            StoreInCache(body, responseJson);

            // Record cost for budget tracking.
            RecordCost(request.Model, response.Usage);

            await RunOnResponseAsync(request, response, cancellationToken).ConfigureAwait(false);
            return response;
        }
        catch (LlmException ex)
        {
            await RunOnErrorAsync(request, ex, cancellationToken).ConfigureAwait(false);
            throw;
        }
    }

    /// <summary>
    /// Sends an embedding request and returns the embedding response.
    /// </summary>
    /// <param name="request">The embedding request.</param>
    /// <param name="cancellationToken">Cancellation token.</param>
    /// <returns>The provider's embedding response.</returns>
    /// <exception cref="LlmException">Thrown when the request fails for any reason.</exception>
    public async Task<EmbeddingResponse> EmbedAsync(
        EmbeddingRequest request,
        CancellationToken cancellationToken = default)
    {
        CheckBudget(request.Model);
        await RunOnRequestAsync(request, cancellationToken).ConfigureAwait(false);
        try
        {
            var body = Serialize(request);

            // Check cache before HTTP call.
            var cached = CheckCache(body);
            if (cached is not null)
            {
                var cachedResponse = Deserialize<EmbeddingResponse>(cached);
                await RunOnResponseAsync(request, cachedResponse, cancellationToken).ConfigureAwait(false);
                return cachedResponse;
            }

            var responseJson = await PostAsync("embeddings", body, cancellationToken)
                .ConfigureAwait(false);
            var response = Deserialize<EmbeddingResponse>(responseJson);

            // Store in cache.
            StoreInCache(body, responseJson);

            // Record cost for budget tracking.
            RecordCost(request.Model, response.Usage);

            await RunOnResponseAsync(request, response, cancellationToken).ConfigureAwait(false);
            return response;
        }
        catch (LlmException ex)
        {
            await RunOnErrorAsync(request, ex, cancellationToken).ConfigureAwait(false);
            throw;
        }
    }

    /// <summary>
    /// Lists available models for the configured provider.
    /// </summary>
    /// <param name="cancellationToken">Cancellation token.</param>
    /// <returns>The list of available models.</returns>
    /// <exception cref="LlmException">Thrown when the request fails for any reason.</exception>
    public async Task<ModelsListResponse> ListModelsAsync(
        CancellationToken cancellationToken = default)
    {
        CheckBudget(null);
        object request = "list_models";
        await RunOnRequestAsync(request, cancellationToken).ConfigureAwait(false);
        try
        {
            var responseJson = await GetAsync("models", cancellationToken).ConfigureAwait(false);
            var response = Deserialize<ModelsListResponse>(responseJson);
            await RunOnResponseAsync(request, response, cancellationToken).ConfigureAwait(false);
            return response;
        }
        catch (LlmException ex)
        {
            await RunOnErrorAsync(request, ex, cancellationToken).ConfigureAwait(false);
            throw;
        }
    }

    // ─── Inference API ────────────────────────────────────────────────────────

    /// <summary>Generates an image from a text prompt.</summary>
    /// <param name="request">The image generation request.</param>
    /// <param name="cancellationToken">Cancellation token.</param>
    /// <returns>The provider's images response.</returns>
    /// <exception cref="LlmException">Thrown when the request fails for any reason.</exception>
    public async Task<ImagesResponse> ImageGenerateAsync(
        CreateImageRequest request,
        CancellationToken cancellationToken = default)
    {
        CheckBudget(request.Model);
        await RunOnRequestAsync(request, cancellationToken).ConfigureAwait(false);
        try
        {
            var body = Serialize(request);
            var responseJson = await PostAsync("images/generations", body, cancellationToken)
                .ConfigureAwait(false);
            var response = Deserialize<ImagesResponse>(responseJson);
            await RunOnResponseAsync(request, response, cancellationToken).ConfigureAwait(false);
            return response;
        }
        catch (LlmException ex)
        {
            await RunOnErrorAsync(request, ex, cancellationToken).ConfigureAwait(false);
            throw;
        }
    }

    /// <summary>Generates audio speech from text, returning raw audio bytes.</summary>
    /// <param name="request">The speech request.</param>
    /// <param name="cancellationToken">Cancellation token.</param>
    /// <returns>Raw audio bytes.</returns>
    /// <exception cref="LlmException">Thrown when the request fails for any reason.</exception>
    public async Task<byte[]> SpeechAsync(
        CreateSpeechRequest request,
        CancellationToken cancellationToken = default)
    {
        CheckBudget(request.Model);
        await RunOnRequestAsync(request, cancellationToken).ConfigureAwait(false);
        try
        {
            var body = Serialize(request);
            var response = await PostForBytesAsync("audio/speech", body, cancellationToken)
                .ConfigureAwait(false);
            await RunOnResponseAsync(request, response, cancellationToken).ConfigureAwait(false);
            return response;
        }
        catch (LlmException ex)
        {
            await RunOnErrorAsync(request, ex, cancellationToken).ConfigureAwait(false);
            throw;
        }
    }

    /// <summary>Transcribes audio to text.</summary>
    /// <param name="request">The transcription request.</param>
    /// <param name="cancellationToken">Cancellation token.</param>
    /// <returns>The transcription response.</returns>
    /// <exception cref="LlmException">Thrown when the request fails for any reason.</exception>
    public async Task<TranscriptionResponse> TranscribeAsync(
        CreateTranscriptionRequest request,
        CancellationToken cancellationToken = default)
    {
        CheckBudget(request.Model);
        await RunOnRequestAsync(request, cancellationToken).ConfigureAwait(false);
        try
        {
            var body = Serialize(request);
            var responseJson = await PostAsync("audio/transcriptions", body, cancellationToken)
                .ConfigureAwait(false);
            var response = Deserialize<TranscriptionResponse>(responseJson);
            await RunOnResponseAsync(request, response, cancellationToken).ConfigureAwait(false);
            return response;
        }
        catch (LlmException ex)
        {
            await RunOnErrorAsync(request, ex, cancellationToken).ConfigureAwait(false);
            throw;
        }
    }

    /// <summary>Checks content against moderation policies.</summary>
    /// <param name="request">The moderation request.</param>
    /// <param name="cancellationToken">Cancellation token.</param>
    /// <returns>The moderation response.</returns>
    /// <exception cref="LlmException">Thrown when the request fails for any reason.</exception>
    public async Task<ModerationResponse> ModerateAsync(
        ModerationRequest request,
        CancellationToken cancellationToken = default)
    {
        CheckBudget(request.Model);
        await RunOnRequestAsync(request, cancellationToken).ConfigureAwait(false);
        try
        {
            var body = Serialize(request);
            var responseJson = await PostAsync("moderations", body, cancellationToken)
                .ConfigureAwait(false);
            var response = Deserialize<ModerationResponse>(responseJson);
            await RunOnResponseAsync(request, response, cancellationToken).ConfigureAwait(false);
            return response;
        }
        catch (LlmException ex)
        {
            await RunOnErrorAsync(request, ex, cancellationToken).ConfigureAwait(false);
            throw;
        }
    }

    /// <summary>Reranks documents by relevance to a query.</summary>
    /// <param name="request">The rerank request.</param>
    /// <param name="cancellationToken">Cancellation token.</param>
    /// <returns>The rerank response.</returns>
    /// <exception cref="LlmException">Thrown when the request fails for any reason.</exception>
    public async Task<RerankResponse> RerankAsync(
        RerankRequest request,
        CancellationToken cancellationToken = default)
    {
        CheckBudget(request.Model);
        await RunOnRequestAsync(request, cancellationToken).ConfigureAwait(false);
        try
        {
            var body = Serialize(request);
            var responseJson = await PostAsync("rerank", body, cancellationToken)
                .ConfigureAwait(false);
            var response = Deserialize<RerankResponse>(responseJson);
            await RunOnResponseAsync(request, response, cancellationToken).ConfigureAwait(false);
            return response;
        }
        catch (LlmException ex)
        {
            await RunOnErrorAsync(request, ex, cancellationToken).ConfigureAwait(false);
            throw;
        }
    }

    // ─── File Management ──────────────────────────────────────────────────────

    /// <summary>Uploads a file.</summary>
    /// <param name="request">The file upload request.</param>
    /// <param name="cancellationToken">Cancellation token.</param>
    /// <returns>The created file object.</returns>
    /// <exception cref="LlmException">Thrown when the request fails for any reason.</exception>
    public async Task<FileObject> CreateFileAsync(
        CreateFileRequest request,
        CancellationToken cancellationToken = default)
    {
        CheckBudget(null);
        await RunOnRequestAsync(request, cancellationToken).ConfigureAwait(false);
        try
        {
            var body = Serialize(request);
            var responseJson = await PostAsync("files", body, cancellationToken)
                .ConfigureAwait(false);
            var response = Deserialize<FileObject>(responseJson);
            await RunOnResponseAsync(request, response, cancellationToken).ConfigureAwait(false);
            return response;
        }
        catch (LlmException ex)
        {
            await RunOnErrorAsync(request, ex, cancellationToken).ConfigureAwait(false);
            throw;
        }
    }

    /// <summary>Retrieves metadata for a file by ID.</summary>
    /// <param name="fileId">The file identifier.</param>
    /// <param name="cancellationToken">Cancellation token.</param>
    /// <returns>The file object.</returns>
    /// <exception cref="LlmException">Thrown when the request fails for any reason.</exception>
    public async Task<FileObject> RetrieveFileAsync(
        string fileId,
        CancellationToken cancellationToken = default)
    {
        CheckBudget(null);
        await RunOnRequestAsync(fileId, cancellationToken).ConfigureAwait(false);
        try
        {
            var responseJson = await GetAsync($"files/{fileId}", cancellationToken)
                .ConfigureAwait(false);
            var response = Deserialize<FileObject>(responseJson);
            await RunOnResponseAsync(fileId, response, cancellationToken).ConfigureAwait(false);
            return response;
        }
        catch (LlmException ex)
        {
            await RunOnErrorAsync(fileId, ex, cancellationToken).ConfigureAwait(false);
            throw;
        }
    }

    /// <summary>Deletes a file by ID.</summary>
    /// <param name="fileId">The file identifier.</param>
    /// <param name="cancellationToken">Cancellation token.</param>
    /// <returns>The delete confirmation response.</returns>
    /// <exception cref="LlmException">Thrown when the request fails for any reason.</exception>
    public async Task<DeleteResponse> DeleteFileAsync(
        string fileId,
        CancellationToken cancellationToken = default)
    {
        CheckBudget(null);
        await RunOnRequestAsync(fileId, cancellationToken).ConfigureAwait(false);
        try
        {
            var responseJson = await DeleteAsync($"files/{fileId}", cancellationToken)
                .ConfigureAwait(false);
            var response = Deserialize<DeleteResponse>(responseJson);
            await RunOnResponseAsync(fileId, response, cancellationToken).ConfigureAwait(false);
            return response;
        }
        catch (LlmException ex)
        {
            await RunOnErrorAsync(fileId, ex, cancellationToken).ConfigureAwait(false);
            throw;
        }
    }

    /// <summary>Lists files, optionally filtered by query parameters.</summary>
    /// <param name="query">Optional query parameters; may be <c>null</c>.</param>
    /// <param name="cancellationToken">Cancellation token.</param>
    /// <returns>The file list response.</returns>
    /// <exception cref="LlmException">Thrown when the request fails for any reason.</exception>
    public async Task<FileListResponse> ListFilesAsync(
        FileListQuery? query = null,
        CancellationToken cancellationToken = default)
    {
        CheckBudget(null);
        object request = query ?? (object)"list_files";
        await RunOnRequestAsync(request, cancellationToken).ConfigureAwait(false);
        try
        {
            var path = "files";
            if (query is not null)
            {
                var parameters = new List<string>();
                if (query.Purpose is not null) parameters.Add($"purpose={Uri.EscapeDataString(query.Purpose)}");
                if (query.Limit is not null) parameters.Add($"limit={query.Limit}");
                if (query.After is not null) parameters.Add($"after={Uri.EscapeDataString(query.After)}");
                if (parameters.Count > 0) path += "?" + string.Join("&", parameters);
            }

            var responseJson = await GetAsync(path, cancellationToken).ConfigureAwait(false);
            var response = Deserialize<FileListResponse>(responseJson);
            await RunOnResponseAsync(request, response, cancellationToken).ConfigureAwait(false);
            return response;
        }
        catch (LlmException ex)
        {
            await RunOnErrorAsync(request, ex, cancellationToken).ConfigureAwait(false);
            throw;
        }
    }

    /// <summary>Retrieves the raw content of a file.</summary>
    /// <param name="fileId">The file identifier.</param>
    /// <param name="cancellationToken">Cancellation token.</param>
    /// <returns>Raw file content as bytes.</returns>
    /// <exception cref="LlmException">Thrown when the request fails for any reason.</exception>
    public async Task<byte[]> FileContentAsync(
        string fileId,
        CancellationToken cancellationToken = default)
    {
        CheckBudget(null);
        await RunOnRequestAsync(fileId, cancellationToken).ConfigureAwait(false);
        try
        {
            var response = await GetForBytesAsync($"files/{fileId}/content", cancellationToken)
                .ConfigureAwait(false);
            await RunOnResponseAsync(fileId, response, cancellationToken).ConfigureAwait(false);
            return response;
        }
        catch (LlmException ex)
        {
            await RunOnErrorAsync(fileId, ex, cancellationToken).ConfigureAwait(false);
            throw;
        }
    }

    // ─── Batch Management ─────────────────────────────────────────────────────

    /// <summary>Creates a new batch job.</summary>
    /// <param name="request">The batch creation request.</param>
    /// <param name="cancellationToken">Cancellation token.</param>
    /// <returns>The created batch object.</returns>
    /// <exception cref="LlmException">Thrown when the request fails for any reason.</exception>
    public async Task<BatchObject> CreateBatchAsync(
        CreateBatchRequest request,
        CancellationToken cancellationToken = default)
    {
        CheckBudget(null);
        await RunOnRequestAsync(request, cancellationToken).ConfigureAwait(false);
        try
        {
            var body = Serialize(request);
            var responseJson = await PostAsync("batches", body, cancellationToken)
                .ConfigureAwait(false);
            var response = Deserialize<BatchObject>(responseJson);
            await RunOnResponseAsync(request, response, cancellationToken).ConfigureAwait(false);
            return response;
        }
        catch (LlmException ex)
        {
            await RunOnErrorAsync(request, ex, cancellationToken).ConfigureAwait(false);
            throw;
        }
    }

    /// <summary>Retrieves a batch by ID.</summary>
    /// <param name="batchId">The batch identifier.</param>
    /// <param name="cancellationToken">Cancellation token.</param>
    /// <returns>The batch object.</returns>
    /// <exception cref="LlmException">Thrown when the request fails for any reason.</exception>
    public async Task<BatchObject> RetrieveBatchAsync(
        string batchId,
        CancellationToken cancellationToken = default)
    {
        CheckBudget(null);
        await RunOnRequestAsync(batchId, cancellationToken).ConfigureAwait(false);
        try
        {
            var responseJson = await GetAsync($"batches/{batchId}", cancellationToken)
                .ConfigureAwait(false);
            var response = Deserialize<BatchObject>(responseJson);
            await RunOnResponseAsync(batchId, response, cancellationToken).ConfigureAwait(false);
            return response;
        }
        catch (LlmException ex)
        {
            await RunOnErrorAsync(batchId, ex, cancellationToken).ConfigureAwait(false);
            throw;
        }
    }

    /// <summary>Lists batches, optionally filtered by query parameters.</summary>
    /// <param name="query">Optional query parameters; may be <c>null</c>.</param>
    /// <param name="cancellationToken">Cancellation token.</param>
    /// <returns>The batch list response.</returns>
    /// <exception cref="LlmException">Thrown when the request fails for any reason.</exception>
    public async Task<BatchListResponse> ListBatchesAsync(
        BatchListQuery? query = null,
        CancellationToken cancellationToken = default)
    {
        CheckBudget(null);
        object request = query ?? (object)"list_batches";
        await RunOnRequestAsync(request, cancellationToken).ConfigureAwait(false);
        try
        {
            var path = "batches";
            if (query is not null)
            {
                var parameters = new List<string>();
                if (query.Limit is not null) parameters.Add($"limit={query.Limit}");
                if (query.After is not null) parameters.Add($"after={Uri.EscapeDataString(query.After)}");
                if (parameters.Count > 0) path += "?" + string.Join("&", parameters);
            }

            var responseJson = await GetAsync(path, cancellationToken).ConfigureAwait(false);
            var response = Deserialize<BatchListResponse>(responseJson);
            await RunOnResponseAsync(request, response, cancellationToken).ConfigureAwait(false);
            return response;
        }
        catch (LlmException ex)
        {
            await RunOnErrorAsync(request, ex, cancellationToken).ConfigureAwait(false);
            throw;
        }
    }

    /// <summary>Cancels an in-progress batch.</summary>
    /// <param name="batchId">The batch identifier.</param>
    /// <param name="cancellationToken">Cancellation token.</param>
    /// <returns>The updated batch object.</returns>
    /// <exception cref="LlmException">Thrown when the request fails for any reason.</exception>
    public async Task<BatchObject> CancelBatchAsync(
        string batchId,
        CancellationToken cancellationToken = default)
    {
        CheckBudget(null);
        await RunOnRequestAsync(batchId, cancellationToken).ConfigureAwait(false);
        try
        {
            var responseJson = await PostAsync($"batches/{batchId}/cancel", "", cancellationToken)
                .ConfigureAwait(false);
            var response = Deserialize<BatchObject>(responseJson);
            await RunOnResponseAsync(batchId, response, cancellationToken).ConfigureAwait(false);
            return response;
        }
        catch (LlmException ex)
        {
            await RunOnErrorAsync(batchId, ex, cancellationToken).ConfigureAwait(false);
            throw;
        }
    }

    // ─── Responses API ────────────────────────────────────────────────────────

    /// <summary>Creates a new response via the Responses API.</summary>
    /// <param name="request">The response creation request.</param>
    /// <param name="cancellationToken">Cancellation token.</param>
    /// <returns>The created response object.</returns>
    /// <exception cref="LlmException">Thrown when the request fails for any reason.</exception>
    public async Task<ResponseObject> CreateResponseAsync(
        CreateResponseRequest request,
        CancellationToken cancellationToken = default)
    {
        CheckBudget(request.Model);
        await RunOnRequestAsync(request, cancellationToken).ConfigureAwait(false);
        try
        {
            var body = Serialize(request);
            var responseJson = await PostAsync("responses", body, cancellationToken)
                .ConfigureAwait(false);
            var response = Deserialize<ResponseObject>(responseJson);
            await RunOnResponseAsync(request, response, cancellationToken).ConfigureAwait(false);
            return response;
        }
        catch (LlmException ex)
        {
            await RunOnErrorAsync(request, ex, cancellationToken).ConfigureAwait(false);
            throw;
        }
    }

    /// <summary>Retrieves a response by ID.</summary>
    /// <param name="responseId">The response identifier.</param>
    /// <param name="cancellationToken">Cancellation token.</param>
    /// <returns>The response object.</returns>
    /// <exception cref="LlmException">Thrown when the request fails for any reason.</exception>
    public async Task<ResponseObject> RetrieveResponseAsync(
        string responseId,
        CancellationToken cancellationToken = default)
    {
        CheckBudget(null);
        await RunOnRequestAsync(responseId, cancellationToken).ConfigureAwait(false);
        try
        {
            var responseJson = await GetAsync($"responses/{responseId}", cancellationToken)
                .ConfigureAwait(false);
            var response = Deserialize<ResponseObject>(responseJson);
            await RunOnResponseAsync(responseId, response, cancellationToken).ConfigureAwait(false);
            return response;
        }
        catch (LlmException ex)
        {
            await RunOnErrorAsync(responseId, ex, cancellationToken).ConfigureAwait(false);
            throw;
        }
    }

    /// <summary>Cancels an in-progress response.</summary>
    /// <param name="responseId">The response identifier.</param>
    /// <param name="cancellationToken">Cancellation token.</param>
    /// <returns>The updated response object.</returns>
    /// <exception cref="LlmException">Thrown when the request fails for any reason.</exception>
    public async Task<ResponseObject> CancelResponseAsync(
        string responseId,
        CancellationToken cancellationToken = default)
    {
        CheckBudget(null);
        await RunOnRequestAsync(responseId, cancellationToken).ConfigureAwait(false);
        try
        {
            var responseJson = await PostAsync($"responses/{responseId}/cancel", "", cancellationToken)
                .ConfigureAwait(false);
            var response = Deserialize<ResponseObject>(responseJson);
            await RunOnResponseAsync(responseId, response, cancellationToken).ConfigureAwait(false);
            return response;
        }
        catch (LlmException ex)
        {
            await RunOnErrorAsync(responseId, ex, cancellationToken).ConfigureAwait(false);
            throw;
        }
    }

    // ─── Hooks & Custom Providers ─────────────────────────────────────────────

    /// <summary>Registers a lifecycle hook. Hooks are invoked in registration order.</summary>
    /// <param name="hook">The hook to register.</param>
    public void AddHook(ILlmHook hook)
    {
        ArgumentNullException.ThrowIfNull(hook);
        _hooks.Add(hook);
    }

    /// <summary>
    /// Registers a custom provider configuration. Requests whose model name starts
    /// with one of the provider's prefixes are routed to its base URL.
    /// </summary>
    /// <param name="config">The provider configuration to register.</param>
    public void RegisterProvider(ProviderConfig config)
    {
        ArgumentNullException.ThrowIfNull(config);
        _customProviders.Add(config);
    }

    /// <summary>Gets the configured cache settings, or <c>null</c> if caching is disabled.</summary>
    public CacheConfig? CacheConfiguration => _cacheConfig;

    /// <summary>Gets the configured budget settings, or <c>null</c> if budget enforcement is disabled.</summary>
    public BudgetConfig? BudgetConfiguration => _budgetConfig;

    private async Task RunOnRequestAsync(object request, CancellationToken ct)
    {
        foreach (var hook in _hooks)
        {
            await hook.OnRequestAsync(request, ct).ConfigureAwait(false);
        }
    }

    private async Task RunOnResponseAsync(object request, object response, CancellationToken ct)
    {
        foreach (var hook in _hooks)
        {
            await hook.OnResponseAsync(request, response, ct).ConfigureAwait(false);
        }
    }

    private async Task RunOnErrorAsync(object request, Exception error, CancellationToken ct)
    {
        foreach (var hook in _hooks)
        {
            await hook.OnErrorAsync(request, error, ct).ConfigureAwait(false);
        }
    }

    // ─── Cache Helpers ─────────────────────────────────────────────────────────

    /// <summary>Returns the cached response for the given request JSON, or null if not found or expired.</summary>
    private string? CheckCache(string requestJson)
    {
        if (_responseCache is null || _cacheConfig is null) return null;

        var key = Sha256Hex(requestJson);
        if (!_responseCache.TryGetValue(key, out var entry)) return null;

        if (entry.IsExpired(_cacheConfig.TtlSeconds))
        {
            _responseCache.TryRemove(key, out _);
            return null;
        }

        return entry.Response;
    }

    /// <summary>Stores a response in the cache keyed by the request JSON hash.</summary>
    private void StoreInCache(string requestJson, string response)
    {
        if (_responseCache is null || _cacheConfig is null) return;

        var key = Sha256Hex(requestJson);
        _responseCache[key] = new CacheEntry(response);

        // Simple eviction: if over capacity, remove an arbitrary entry.
        while (_responseCache.Count > _cacheConfig.MaxEntries)
        {
            // Remove first key (arbitrary but deterministic enough for LRU approximation).
            using var enumerator = _responseCache.GetEnumerator();
            if (enumerator.MoveNext())
            {
                _responseCache.TryRemove(enumerator.Current.Key, out _);
            }
        }
    }

    // ─── Budget Helpers ──────────────────────────────────────────────────────

    /// <summary>Checks if the budget has been exceeded. Throws BudgetExceededException in strict mode.</summary>
    private void CheckBudget(string? model)
    {
        if (_budgetConfig is null || _budgetConfig.Enforcement != "strict") return;

        if (_budgetConfig.GlobalLimit is not null)
        {
            var globalSpendUsd = Interlocked.Read(ref _globalSpendMicrocents) / (double)MicrocentsPerUsd;
            if (globalSpendUsd >= _budgetConfig.GlobalLimit.Value)
            {
                throw new BudgetExceededException(
                    $"global spend ${globalSpendUsd:F6} >= limit ${_budgetConfig.GlobalLimit.Value:F6}");
            }
        }

        if (_budgetConfig.ModelLimits is not null && model is not null
            && _budgetConfig.ModelLimits.TryGetValue(model, out var modelLimit))
        {
            if (_modelSpendMicrocents.TryGetValue(model, out var modelMicrocents))
            {
                var modelSpendUsd = Interlocked.Read(ref modelMicrocents) / (double)MicrocentsPerUsd;
                if (modelSpendUsd >= modelLimit)
                {
                    throw new BudgetExceededException(
                        $"model \"{model}\" spend ${modelSpendUsd:F6} >= limit ${modelLimit:F6}");
                }
            }
        }
    }

    /// <summary>Records cost for the given model based on token usage.</summary>
    private void RecordCost(string? model, Usage? usage)
    {
        if (_budgetConfig is null || usage is null) return;

        var cost = EstimateCost(model, usage);
        if (cost <= 0) return;

        var costMicrocents = (long)Math.Round(cost * MicrocentsPerUsd);
        Interlocked.Add(ref _globalSpendMicrocents, costMicrocents);
        if (model is not null)
        {
            _modelSpendMicrocents.AddOrUpdate(model, costMicrocents, (_, existing) => existing + costMicrocents);
        }
    }

    private static double EstimateCost(string? model, Usage usage)
    {
        var pricing = LookupPricing(model);
        var promptCost = usage.PromptTokens * pricing.Prompt / 1_000_000.0;
        var completionCost = usage.CompletionTokens * pricing.Completion / 1_000_000.0;
        return promptCost + completionCost;
    }

    private static (double Prompt, double Completion) LookupPricing(string? model)
    {
        if (model is null) return FallbackPricing;

        // Exact match.
        if (ModelPricing.TryGetValue(model, out var pricing)) return pricing;

        // Strip provider prefix (e.g. "openai/gpt-4o" -> "gpt-4o").
        var slashIdx = model.IndexOf('/');
        if (slashIdx >= 0)
        {
            var stripped = model[(slashIdx + 1)..];
            if (ModelPricing.TryGetValue(stripped, out pricing)) return pricing;
        }

        // Prefix match for versioned models.
        var name = slashIdx >= 0 ? model[(slashIdx + 1)..] : model;
        foreach (var kvp in ModelPricing)
        {
            if (name.StartsWith(kvp.Key, StringComparison.Ordinal)) return kvp.Value;
        }

        return FallbackPricing;
    }

    private static string Sha256Hex(string input)
    {
        var hash = SHA256.HashData(Encoding.UTF8.GetBytes(input));
        return Convert.ToHexStringLower(hash);
    }

    // ─── HTTP Internals ───────────────────────────────────────────────────────

    private async Task<string> PostAsync(string path, string body, CancellationToken ct)
    {
        LlmException? lastException = null;
        for (int attempt = 0; attempt <= _maxRetries; attempt++)
        {
            using var content = new StringContent(body, Encoding.UTF8, "application/json");
            try
            {
                using var response = await _httpClient
                    .PostAsync(path, content, ct)
                    .ConfigureAwait(false);
                return await HandleResponseAsync(response, ct).ConfigureAwait(false);
            }
            catch (LlmException ex) when (IsRetryable(ex) && attempt < _maxRetries)
            {
                lastException = ex;
            }
            catch (LlmException ex)
            {
                throw;
            }
            catch (TaskCanceledException ex) when (!ct.IsCancellationRequested)
            {
                throw new ProviderException(0, $"request timed out: {ex.Message}");
            }
        }

        throw lastException ?? new LlmException(LlmException.ErrorCodes.Unknown, "liter-llm: unknown error");
    }

    private async Task<string> GetAsync(string path, CancellationToken ct)
    {
        LlmException? lastException = null;
        for (int attempt = 0; attempt <= _maxRetries; attempt++)
        {
            try
            {
                using var response = await _httpClient
                    .GetAsync(path, ct)
                    .ConfigureAwait(false);
                return await HandleResponseAsync(response, ct).ConfigureAwait(false);
            }
            catch (LlmException ex) when (IsRetryable(ex) && attempt < _maxRetries)
            {
                lastException = ex;
            }
            catch (LlmException)
            {
                throw;
            }
            catch (TaskCanceledException ex) when (!ct.IsCancellationRequested)
            {
                throw new ProviderException(0, $"request timed out: {ex.Message}");
            }
        }

        throw lastException ?? new LlmException(LlmException.ErrorCodes.Unknown, "liter-llm: unknown error");
    }

    private async Task<string> DeleteAsync(string path, CancellationToken ct)
    {
        LlmException? lastException = null;
        for (int attempt = 0; attempt <= _maxRetries; attempt++)
        {
            try
            {
                using var response = await _httpClient
                    .DeleteAsync(path, ct)
                    .ConfigureAwait(false);
                return await HandleResponseAsync(response, ct).ConfigureAwait(false);
            }
            catch (LlmException ex) when (IsRetryable(ex) && attempt < _maxRetries)
            {
                lastException = ex;
            }
            catch (LlmException)
            {
                throw;
            }
            catch (TaskCanceledException ex) when (!ct.IsCancellationRequested)
            {
                throw new ProviderException(0, $"request timed out: {ex.Message}");
            }
        }

        throw lastException ?? new LlmException(LlmException.ErrorCodes.Unknown, "liter-llm: unknown error");
    }

    private async Task<byte[]> PostForBytesAsync(string path, string body, CancellationToken ct)
    {
        LlmException? lastException = null;
        for (int attempt = 0; attempt <= _maxRetries; attempt++)
        {
            using var content = new StringContent(body, Encoding.UTF8, "application/json");
            try
            {
                using var response = await _httpClient
                    .PostAsync(path, content, ct)
                    .ConfigureAwait(false);
                return await HandleBytesResponseAsync(response, ct).ConfigureAwait(false);
            }
            catch (LlmException ex) when (IsRetryable(ex) && attempt < _maxRetries)
            {
                lastException = ex;
            }
            catch (LlmException)
            {
                throw;
            }
            catch (TaskCanceledException ex) when (!ct.IsCancellationRequested)
            {
                throw new ProviderException(0, $"request timed out: {ex.Message}");
            }
        }

        throw lastException ?? new LlmException(LlmException.ErrorCodes.Unknown, "liter-llm: unknown error");
    }

    private async Task<byte[]> GetForBytesAsync(string path, CancellationToken ct)
    {
        LlmException? lastException = null;
        for (int attempt = 0; attempt <= _maxRetries; attempt++)
        {
            try
            {
                using var response = await _httpClient
                    .GetAsync(path, ct)
                    .ConfigureAwait(false);
                return await HandleBytesResponseAsync(response, ct).ConfigureAwait(false);
            }
            catch (LlmException ex) when (IsRetryable(ex) && attempt < _maxRetries)
            {
                lastException = ex;
            }
            catch (LlmException)
            {
                throw;
            }
            catch (TaskCanceledException ex) when (!ct.IsCancellationRequested)
            {
                throw new ProviderException(0, $"request timed out: {ex.Message}");
            }
        }

        throw lastException ?? new LlmException(LlmException.ErrorCodes.Unknown, "liter-llm: unknown error");
    }

    private static async Task<string> HandleResponseAsync(HttpResponseMessage response, CancellationToken ct)
    {
        var responseBody = await response.Content.ReadAsStringAsync(ct).ConfigureAwait(false);
        if (response.IsSuccessStatusCode)
        {
            return responseBody;
        }

        throw ClassifyHttpError((int)response.StatusCode, responseBody);
    }

    private static async Task<byte[]> HandleBytesResponseAsync(HttpResponseMessage response, CancellationToken ct)
    {
        if (response.IsSuccessStatusCode)
        {
            return await response.Content.ReadAsByteArrayAsync(ct).ConfigureAwait(false);
        }

        var responseBody = await response.Content.ReadAsStringAsync(ct).ConfigureAwait(false);
        throw ClassifyHttpError((int)response.StatusCode, responseBody);
    }

    private static LlmException ClassifyHttpError(int status, string body)
    {
        var message = ExtractErrorMessage(body);
        return status switch
        {
            400 or 422 => new InvalidRequestException(message),
            401 or 403 => new AuthenticationException(message),
            404 => new NotFoundException(message),
            429 => new RateLimitException(message),
            _ => new ProviderException(status, message),
        };
    }

    private static bool IsRetryable(LlmException ex) =>
        ex is RateLimitException or ProviderException;

    private static string ExtractErrorMessage(string body)
    {
        if (string.IsNullOrWhiteSpace(body))
        {
            return "empty response body";
        }

        // Best-effort: extract {"error":{"message":"..."}} without a full round-trip parse
        var messageIdx = body.IndexOf("\"message\"", StringComparison.Ordinal);
        if (messageIdx >= 0)
        {
            var colon = body.IndexOf(':', messageIdx);
            var quote1 = body.IndexOf('"', colon + 1);
            var quote2 = body.IndexOf('"', quote1 + 1);
            if (quote1 >= 0 && quote2 > quote1)
            {
                return body[(quote1 + 1)..quote2];
            }
        }

        return body.Length > 200 ? body[..200] + "…" : body;
    }

    // ─── Serialization helpers ────────────────────────────────────────────────

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

    // ─── IDisposable ──────────────────────────────────────────────────────────

    /// <summary>Releases the underlying <see cref="HttpClient"/>.</summary>
    public void Dispose() => _httpClient.Dispose();

    /// <summary>Asynchronously releases the underlying <see cref="HttpClient"/>.</summary>
    public ValueTask DisposeAsync()
    {
        _httpClient.Dispose();
        return ValueTask.CompletedTask;
    }
}
