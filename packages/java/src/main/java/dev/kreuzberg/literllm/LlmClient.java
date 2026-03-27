package dev.kreuzberg.literllm;

import static dev.kreuzberg.literllm.Types.*;

import com.fasterxml.jackson.databind.DeserializationFeature;
import com.fasterxml.jackson.databind.ObjectMapper;
import com.fasterxml.jackson.databind.SerializationFeature;
import java.net.URI;
import java.net.http.HttpClient;
import java.net.http.HttpRequest;
import java.net.http.HttpResponse;
import java.time.Duration;

/**
 * HTTP client for the liter-llm unified LLM API.
 *
 * <p>
 * Speaks the OpenAI-compatible wire protocol directly — no FFI, no native
 * libraries. The model-name prefix selects the provider and endpoint (e.g.
 * {@code "groq/llama3-70b"} routes to Groq). Implements {@link AutoCloseable};
 * close after use to release the underlying {@link HttpClient} executor.
 *
 * <h2>Example</h2>
 *
 * <pre>{@code
 * try (var client = LlmClient.builder().apiKey(System.getenv("OPENAI_API_KEY")).build()) {
 * 	var request = ChatCompletionRequest.builder("gpt-4o-mini", List.of(new Types.UserMessage("Hello!")))
 * 			.maxTokens(256L).build();
 * 	var response = client.chat(request);
 * 	System.out.println(response.choices().getFirst().message().content());
 * }
 * }</pre>
 */
public final class LlmClient implements AutoCloseable {

	static final String DEFAULT_BASE_URL = "https://api.openai.com/v1";
	static final int DEFAULT_MAX_RETRIES = 2;
	static final Duration DEFAULT_TIMEOUT = Duration.ofSeconds(60);

	private static final int HTTP_OK_MIN = 200;
	private static final int HTTP_OK_MAX = 300;
	private static final int HTTP_BAD_REQUEST = 400;
	private static final int HTTP_UNAUTHORIZED = 401;
	private static final int HTTP_FORBIDDEN = 403;
	private static final int HTTP_NOT_FOUND = 404;
	private static final int HTTP_UNPROCESSABLE = 422;
	private static final int HTTP_RATE_LIMIT = 429;
	private static final int HTTP_SERVER_ERROR_MIN = 500;
	private static final int ERROR_BODY_MAX_LENGTH = 200;

	private final String apiKey;
	private final String baseUrl;
	private final int maxRetries;
	private final HttpClient httpClient;
	private final ObjectMapper objectMapper;
	private final CacheConfig cacheConfig;
	private final BudgetConfig budgetConfig;
	private final java.util.List<LlmHook> hooks = new java.util.concurrent.CopyOnWriteArrayList<>();
	private final java.util.List<ProviderConfig> customProviders = new java.util.concurrent.CopyOnWriteArrayList<>();

	private LlmClient(Builder builder) {
		this.apiKey = builder.apiKey;
		this.baseUrl = builder.baseUrl.endsWith("/")
				? builder.baseUrl.substring(0, builder.baseUrl.length() - 1)
				: builder.baseUrl;
		this.maxRetries = builder.maxRetries;
		this.httpClient = HttpClient.newBuilder().connectTimeout(builder.timeout).build();
		this.objectMapper = createObjectMapper();
		this.cacheConfig = builder.cacheConfig;
		this.budgetConfig = builder.budgetConfig;
	}

	// ─── Public API ───────────────────────────────────────────────────────────

	/**
	 * Sends a chat completion request and returns the full response.
	 *
	 * @param request
	 *            the chat completion request
	 * @return the provider's chat completion response
	 * @throws LlmException
	 *             if the request fails for any reason
	 */
	public ChatCompletionResponse chat(ChatCompletionRequest request) throws LlmException {
		runOnRequest(request);
		try {
			String body = serialize(request);
			String responseBody = post("/chat/completions", body);
			ChatCompletionResponse response = deserialize(responseBody, ChatCompletionResponse.class);
			runOnResponse(request, response);
			return response;
		} catch (LlmException e) {
			runOnError(request, e);
			throw e;
		}
	}

	/**
	 * Sends an embedding request and returns the embedding response.
	 *
	 * @param request
	 *            the embedding request
	 * @return the provider's embedding response
	 * @throws LlmException
	 *             if the request fails for any reason
	 */
	public EmbeddingResponse embed(EmbeddingRequest request) throws LlmException {
		String body = serialize(request);
		String responseBody = post("/embeddings", body);
		return deserialize(responseBody, EmbeddingResponse.class);
	}

	/**
	 * Lists available models for the configured provider.
	 *
	 * @return the list of available models
	 * @throws LlmException
	 *             if the request fails for any reason
	 */
	public ModelsListResponse listModels() throws LlmException {
		String responseBody = get("/models");
		return deserialize(responseBody, ModelsListResponse.class);
	}

	// ─── Inference API ────────────────────────────────────────────────────────

	/**
	 * Generates an image from a text prompt.
	 *
	 * @param request
	 *            the image generation request
	 * @return the provider's images response
	 * @throws LlmException
	 *             if the request fails for any reason
	 */
	public ImagesResponse imageGenerate(CreateImageRequest request) throws LlmException {
		String body = serialize(request);
		String responseBody = post("/images/generations", body);
		return deserialize(responseBody, ImagesResponse.class);
	}

	/**
	 * Generates audio speech from text, returning raw audio bytes.
	 *
	 * @param request
	 *            the speech request
	 * @return raw audio bytes
	 * @throws LlmException
	 *             if the request fails for any reason
	 */
	public byte[] speech(CreateSpeechRequest request) throws LlmException {
		String body = serialize(request);
		return postForBytes("/audio/speech", body);
	}

	/**
	 * Transcribes audio to text.
	 *
	 * @param request
	 *            the transcription request
	 * @return the transcription response
	 * @throws LlmException
	 *             if the request fails for any reason
	 */
	public TranscriptionResponse transcribe(CreateTranscriptionRequest request) throws LlmException {
		String body = serialize(request);
		String responseBody = post("/audio/transcriptions", body);
		return deserialize(responseBody, TranscriptionResponse.class);
	}

	/**
	 * Checks content against moderation policies.
	 *
	 * @param request
	 *            the moderation request
	 * @return the moderation response
	 * @throws LlmException
	 *             if the request fails for any reason
	 */
	public ModerationResponse moderate(ModerationRequest request) throws LlmException {
		String body = serialize(request);
		String responseBody = post("/moderations", body);
		return deserialize(responseBody, ModerationResponse.class);
	}

	/**
	 * Reranks documents by relevance to a query.
	 *
	 * @param request
	 *            the rerank request
	 * @return the rerank response
	 * @throws LlmException
	 *             if the request fails for any reason
	 */
	public RerankResponse rerank(RerankRequest request) throws LlmException {
		String body = serialize(request);
		String responseBody = post("/rerank", body);
		return deserialize(responseBody, RerankResponse.class);
	}

	// ─── File Management ──────────────────────────────────────────────────────

	/**
	 * Uploads a file.
	 *
	 * @param request
	 *            the file upload request
	 * @return the created file object
	 * @throws LlmException
	 *             if the request fails for any reason
	 */
	public FileObject createFile(CreateFileRequest request) throws LlmException {
		String body = serialize(request);
		String responseBody = post("/files", body);
		return deserialize(responseBody, FileObject.class);
	}

	/**
	 * Retrieves metadata for a file by ID.
	 *
	 * @param fileId
	 *            the file identifier
	 * @return the file object
	 * @throws LlmException
	 *             if the request fails for any reason
	 */
	public FileObject retrieveFile(String fileId) throws LlmException {
		String responseBody = get("/files/" + fileId);
		return deserialize(responseBody, FileObject.class);
	}

	/**
	 * Deletes a file by ID.
	 *
	 * @param fileId
	 *            the file identifier
	 * @return the delete confirmation response
	 * @throws LlmException
	 *             if the request fails for any reason
	 */
	public DeleteResponse deleteFile(String fileId) throws LlmException {
		String responseBody = delete("/files/" + fileId);
		return deserialize(responseBody, DeleteResponse.class);
	}

	/**
	 * Lists files, optionally filtered by query parameters.
	 *
	 * @param query
	 *            optional query parameters, may be {@code null}
	 * @return the file list response
	 * @throws LlmException
	 *             if the request fails for any reason
	 */
	public FileListResponse listFiles(FileListQuery query) throws LlmException {
		String path = "/files";
		if (query != null) {
			var params = new java.util.ArrayList<String>();
			if (query.purpose() != null) {
				params.add("purpose=" + query.purpose());
			}
			if (query.limit() != null) {
				params.add("limit=" + query.limit());
			}
			if (query.after() != null) {
				params.add("after=" + query.after());
			}
			if (!params.isEmpty()) {
				path += "?" + String.join("&", params);
			}
		}
		String responseBody = get(path);
		return deserialize(responseBody, FileListResponse.class);
	}

	/**
	 * Retrieves the raw content of a file.
	 *
	 * @param fileId
	 *            the file identifier
	 * @return raw file content as bytes
	 * @throws LlmException
	 *             if the request fails for any reason
	 */
	public byte[] fileContent(String fileId) throws LlmException {
		return getForBytes("/files/" + fileId + "/content");
	}

	// ─── Batch Management ─────────────────────────────────────────────────────

	/**
	 * Creates a new batch job.
	 *
	 * @param request
	 *            the batch creation request
	 * @return the created batch object
	 * @throws LlmException
	 *             if the request fails for any reason
	 */
	public BatchObject createBatch(CreateBatchRequest request) throws LlmException {
		String body = serialize(request);
		String responseBody = post("/batches", body);
		return deserialize(responseBody, BatchObject.class);
	}

	/**
	 * Retrieves a batch by ID.
	 *
	 * @param batchId
	 *            the batch identifier
	 * @return the batch object
	 * @throws LlmException
	 *             if the request fails for any reason
	 */
	public BatchObject retrieveBatch(String batchId) throws LlmException {
		String responseBody = get("/batches/" + batchId);
		return deserialize(responseBody, BatchObject.class);
	}

	/**
	 * Lists batches, optionally filtered by query parameters.
	 *
	 * @param query
	 *            optional query parameters, may be {@code null}
	 * @return the batch list response
	 * @throws LlmException
	 *             if the request fails for any reason
	 */
	public BatchListResponse listBatches(BatchListQuery query) throws LlmException {
		String path = "/batches";
		if (query != null) {
			var params = new java.util.ArrayList<String>();
			if (query.limit() != null) {
				params.add("limit=" + query.limit());
			}
			if (query.after() != null) {
				params.add("after=" + query.after());
			}
			if (!params.isEmpty()) {
				path += "?" + String.join("&", params);
			}
		}
		String responseBody = get(path);
		return deserialize(responseBody, BatchListResponse.class);
	}

	/**
	 * Cancels an in-progress batch.
	 *
	 * @param batchId
	 *            the batch identifier
	 * @return the updated batch object
	 * @throws LlmException
	 *             if the request fails for any reason
	 */
	public BatchObject cancelBatch(String batchId) throws LlmException {
		String responseBody = post("/batches/" + batchId + "/cancel", "");
		return deserialize(responseBody, BatchObject.class);
	}

	// ─── Responses API ────────────────────────────────────────────────────────

	/**
	 * Creates a new response via the Responses API.
	 *
	 * @param request
	 *            the response creation request
	 * @return the created response object
	 * @throws LlmException
	 *             if the request fails for any reason
	 */
	public ResponseObject createResponse(CreateResponseRequest request) throws LlmException {
		String body = serialize(request);
		String responseBody = post("/responses", body);
		return deserialize(responseBody, ResponseObject.class);
	}

	/**
	 * Retrieves a response by ID.
	 *
	 * @param responseId
	 *            the response identifier
	 * @return the response object
	 * @throws LlmException
	 *             if the request fails for any reason
	 */
	public ResponseObject retrieveResponse(String responseId) throws LlmException {
		String responseBody = get("/responses/" + responseId);
		return deserialize(responseBody, ResponseObject.class);
	}

	/**
	 * Cancels an in-progress response.
	 *
	 * @param responseId
	 *            the response identifier
	 * @return the updated response object
	 * @throws LlmException
	 *             if the request fails for any reason
	 */
	public ResponseObject cancelResponse(String responseId) throws LlmException {
		String responseBody = post("/responses/" + responseId + "/cancel", "");
		return deserialize(responseBody, ResponseObject.class);
	}

	// ─── Hooks & Custom Providers ─────────────────────────────────────────────

	/**
	 * Registers a lifecycle hook. Hooks are invoked in registration order.
	 *
	 * @param hook
	 *            the hook to register
	 */
	public void addHook(LlmHook hook) {
		hooks.add(hook);
	}

	/**
	 * Registers a custom provider configuration. Requests whose model name starts
	 * with one of the provider's prefixes are routed to its base URL.
	 *
	 * @param config
	 *            the provider configuration to register
	 */
	public void registerProvider(ProviderConfig config) {
		customProviders.add(config);
	}

	/**
	 * Returns the configured cache settings, or {@code null} if caching is
	 * disabled.
	 *
	 * @return the cache configuration
	 */
	public CacheConfig getCacheConfig() {
		return cacheConfig;
	}

	/**
	 * Returns the configured budget settings, or {@code null} if budget enforcement
	 * is disabled.
	 *
	 * @return the budget configuration
	 */
	public BudgetConfig getBudgetConfig() {
		return budgetConfig;
	}

	private void runOnRequest(Object request) throws HookRejectedException {
		for (var hook : hooks) {
			hook.onRequest(request);
		}
	}

	private void runOnResponse(Object request, Object response) {
		for (var hook : hooks) {
			hook.onResponse(request, response);
		}
	}

	private void runOnError(Object request, Exception error) {
		for (var hook : hooks) {
			hook.onError(request, error);
		}
	}

	/**
	 * Closes the underlying HTTP client, releasing resources.
	 *
	 * <p>
	 * After this method returns, the client must not be used.
	 */
	@Override
	public void close() {
		httpClient.close();
	}

	// ─── HTTP Internals ───────────────────────────────────────────────────────

	private String post(String path, String body) throws LlmException {
		var request = HttpRequest.newBuilder().uri(URI.create(baseUrl + path))
				.header("Authorization", "Bearer " + apiKey).header("Content-Type", "application/json")
				.header("Accept", "application/json").POST(HttpRequest.BodyPublishers.ofString(body)).build();
		return executeWithRetry(request);
	}

	private String get(String path) throws LlmException {
		var request = HttpRequest.newBuilder().uri(URI.create(baseUrl + path))
				.header("Authorization", "Bearer " + apiKey).header("Accept", "application/json").GET().build();
		return executeWithRetry(request);
	}

	private String delete(String path) throws LlmException {
		var request = HttpRequest.newBuilder().uri(URI.create(baseUrl + path))
				.header("Authorization", "Bearer " + apiKey).header("Accept", "application/json").DELETE().build();
		return executeWithRetry(request);
	}

	private byte[] postForBytes(String path, String body) throws LlmException {
		var request = HttpRequest.newBuilder().uri(URI.create(baseUrl + path))
				.header("Authorization", "Bearer " + apiKey).header("Content-Type", "application/json")
				.POST(HttpRequest.BodyPublishers.ofString(body)).build();
		return executeWithRetryBytes(request);
	}

	private byte[] getForBytes(String path) throws LlmException {
		var request = HttpRequest.newBuilder().uri(URI.create(baseUrl + path))
				.header("Authorization", "Bearer " + apiKey).GET().build();
		return executeWithRetryBytes(request);
	}

	private byte[] executeWithRetryBytes(HttpRequest request) throws LlmException {
		return executeWithRetry(request, HttpResponse.BodyHandlers.ofByteArray(),
				body -> new String(body, java.nio.charset.StandardCharsets.UTF_8));
	}

	private String executeWithRetry(HttpRequest request) throws LlmException {
		return executeWithRetry(request, HttpResponse.BodyHandlers.ofString(), body -> body);
	}

	/**
	 * Generic retry loop for HTTP requests. Retries on 429 (rate limit) and 5xx
	 * (server error) status codes; all other errors are thrown immediately.
	 *
	 * @param <T>
	 *            the response body type
	 * @param request
	 *            the HTTP request to execute
	 * @param handler
	 *            the body handler that determines response type
	 * @param bodyToString
	 *            converts the typed body to a String for error messages
	 * @return the response body on success
	 * @throws LlmException
	 *             on non-retryable errors or when retries are exhausted
	 */
	private <T> T executeWithRetry(HttpRequest request, HttpResponse.BodyHandler<T> handler,
			java.util.function.Function<T, String> bodyToString) throws LlmException {
		LlmException lastException = null;
		for (int attempt = 0; attempt <= maxRetries; attempt++) {
			try {
				var response = httpClient.send(request, handler);
				int status = response.statusCode();
				if (status >= HTTP_OK_MIN && status < HTTP_OK_MAX) {
					return response.body();
				}
				lastException = classifyHttpError(status, bodyToString.apply(response.body()));
				// Only retry on 429 and 5xx
				if (status != HTTP_RATE_LIMIT && status < HTTP_SERVER_ERROR_MIN) {
					throw lastException;
				}
			} catch (LlmException e) {
				throw e;
			} catch (InterruptedException e) {
				Thread.currentThread().interrupt();
				lastException = new LlmException(LlmException.CODE_PROVIDER_ERROR,
						"liter-llm: HTTP request failed: " + e.getMessage(), e);
			} catch (java.io.IOException e) {
				lastException = new LlmException(LlmException.CODE_PROVIDER_ERROR,
						"liter-llm: HTTP request failed: " + e.getMessage(), e);
			}
		}
		throw lastException != null
				? lastException
				: new LlmException(LlmException.CODE_UNKNOWN, "liter-llm: unknown error");
	}

	private static LlmException classifyHttpError(int status, String body) {
		String message = extractErrorMessage(body);
		return switch (status) {
			case HTTP_BAD_REQUEST, HTTP_UNPROCESSABLE -> new LlmException.InvalidRequestException(message);
			case HTTP_UNAUTHORIZED, HTTP_FORBIDDEN -> new LlmException.AuthenticationException(message);
			case HTTP_NOT_FOUND -> new LlmException.NotFoundException(message);
			case HTTP_RATE_LIMIT -> new LlmException.RateLimitException(message);
			default -> new LlmException.ProviderException(status, message);
		};
	}

	private static String extractErrorMessage(String body) {
		if (body == null || body.isBlank()) {
			return "empty response body";
		}
		// Best-effort: extract {"error":{"message":"..."}} without full parse
		int msgIdx = body.indexOf("\"message\"");
		if (msgIdx >= 0) {
			int colon = body.indexOf(':', msgIdx);
			int quote1 = body.indexOf('"', colon + 1);
			int quote2 = body.indexOf('"', quote1 + 1);
			if (quote1 >= 0 && quote2 > quote1) {
				return body.substring(quote1 + 1, quote2);
			}
		}
		// Truncate long bodies for readability
		return body.length() > ERROR_BODY_MAX_LENGTH ? body.substring(0, ERROR_BODY_MAX_LENGTH) + "…" : body;
	}

	// ─── Serialization helpers ────────────────────────────────────────────────

	private String serialize(Object value) throws LlmException {
		try {
			return objectMapper.writeValueAsString(value);
		} catch (com.fasterxml.jackson.core.JsonProcessingException e) {
			throw new LlmException.SerializationException("failed to serialize request", e);
		}
	}

	private <T> T deserialize(String json, Class<T> type) throws LlmException {
		try {
			return objectMapper.readValue(json, type);
		} catch (com.fasterxml.jackson.core.JsonProcessingException e) {
			throw new LlmException.SerializationException("failed to deserialize " + type.getSimpleName() + " response",
					e);
		}
	}

	private static ObjectMapper createObjectMapper() {
		return new ObjectMapper().configure(DeserializationFeature.FAIL_ON_UNKNOWN_PROPERTIES, false)
				.configure(SerializationFeature.FAIL_ON_EMPTY_BEANS, false);
	}

	// ─── Builder ──────────────────────────────────────────────────────────────

	/**
	 * Returns a new {@link Builder} for constructing an {@link LlmClient}.
	 *
	 * @return a fresh builder
	 */
	public static Builder builder() {
		return new Builder();
	}

	/** Fluent builder for {@link LlmClient}. */
	public static final class Builder {

		private String apiKey = "";
		private String baseUrl = DEFAULT_BASE_URL;
		private int maxRetries = DEFAULT_MAX_RETRIES;
		private Duration timeout = DEFAULT_TIMEOUT;
		private CacheConfig cacheConfig;
		private BudgetConfig budgetConfig;

		private Builder() {
		}

		/**
		 * Sets the API key sent as {@code Authorization: Bearer <key>}.
		 *
		 * <p>
		 * Reads from the environment when not explicitly set. Never log or serialize
		 * this value.
		 *
		 * @param apiKey
		 *            the API key
		 * @return this builder
		 */
		public Builder apiKey(String apiKey) {
			this.apiKey = apiKey;
			return this;
		}

		/**
		 * Sets the base URL for the API endpoint.
		 *
		 * <p>
		 * Defaults to {@value LlmClient#DEFAULT_BASE_URL}. Override to target a
		 * different provider or a local proxy.
		 *
		 * @param baseUrl
		 *            base URL without trailing slash
		 * @return this builder
		 */
		public Builder baseUrl(String baseUrl) {
			this.baseUrl = baseUrl;
			return this;
		}

		/**
		 * Sets the maximum number of retries for retryable errors (429, 5xx).
		 *
		 * <p>
		 * Defaults to {@value LlmClient#DEFAULT_MAX_RETRIES}.
		 *
		 * @param maxRetries
		 *            non-negative retry count
		 * @return this builder
		 */
		public Builder maxRetries(int maxRetries) {
			if (maxRetries < 0) {
				throw new IllegalArgumentException("maxRetries must be >= 0");
			}
			this.maxRetries = maxRetries;
			return this;
		}

		/**
		 * Sets the connection timeout.
		 *
		 * <p>
		 * Defaults to {@value} 60 seconds.
		 *
		 * @param timeout
		 *            positive duration
		 * @return this builder
		 */
		public Builder timeout(Duration timeout) {
			this.timeout = timeout;
			return this;
		}

		/**
		 * Enables response caching with the given configuration.
		 *
		 * @param cacheConfig
		 *            cache settings
		 * @return this builder
		 */
		public Builder cache(CacheConfig cacheConfig) {
			this.cacheConfig = cacheConfig;
			return this;
		}

		/**
		 * Enables cost budget enforcement with the given configuration.
		 *
		 * @param budgetConfig
		 *            budget settings
		 * @return this builder
		 */
		public Builder budget(BudgetConfig budgetConfig) {
			this.budgetConfig = budgetConfig;
			return this;
		}

		/**
		 * Builds the {@link LlmClient}.
		 *
		 * @return a configured client instance
		 */
		public LlmClient build() {
			return new LlmClient(this);
		}
	}
}
