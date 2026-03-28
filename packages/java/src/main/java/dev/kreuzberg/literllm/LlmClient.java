package dev.kreuzberg.literllm;

import static dev.kreuzberg.literllm.Types.*;

import com.fasterxml.jackson.databind.DeserializationFeature;
import com.fasterxml.jackson.databind.ObjectMapper;
import com.fasterxml.jackson.databind.SerializationFeature;
import java.lang.foreign.Arena;
import java.lang.foreign.Linker;
import java.lang.foreign.MemorySegment;
import java.time.Duration;
import java.util.Base64;
import java.util.LinkedHashMap;
import java.util.Map;
import java.util.concurrent.atomic.AtomicBoolean;
import java.util.function.Consumer;

/**
 * Native FFI client for the liter-llm unified LLM API.
 *
 * <p>
 * Delegates all API calls to the Rust {@code libliter_llm_ffi} shared library
 * via Java's Panama Foreign Function &amp; Memory API. The model-name prefix
 * selects the provider and endpoint (e.g. {@code "groq/llama3-70b"} routes to
 * Groq). Implements {@link AutoCloseable}; close after use to free the native
 * client handle.
 *
 * <p>
 * <b>Example:</b>
 * </p>
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
@SuppressWarnings({"PMD.AvoidCatchingGenericException", "PMD.EmptyCatchBlock"})
public final class LlmClient implements AutoCloseable {

	static final String DEFAULT_BASE_URL = "https://api.openai.com/v1";
	static final int DEFAULT_MAX_RETRIES = 2;
	static final Duration DEFAULT_TIMEOUT = Duration.ofSeconds(60);

	private static final ObjectMapper OBJECT_MAPPER = new ObjectMapper()
			.configure(DeserializationFeature.FAIL_ON_UNKNOWN_PROPERTIES, false)
			.configure(SerializationFeature.FAIL_ON_EMPTY_BEANS, false);

	private final MemorySegment handle;
	private final Arena arena;
	private final AtomicBoolean closed = new AtomicBoolean(false);

	/**
	 * Creates a client with just an API key, using default configuration.
	 *
	 * @param apiKey
	 *            the API key for authentication
	 */
	public LlmClient(String apiKey) {
		this(builder().apiKey(apiKey));
	}

	private LlmClient(Builder builder) {
		this.arena = Arena.ofShared();
		try {
			var configJson = OBJECT_MAPPER.writeValueAsString(builder.toConfigMap());
			var cConfig = arena.allocateFrom(configJson);
			this.handle = (MemorySegment) NativeMethods.CLIENT_NEW_WITH_CONFIG.invokeExact(cConfig);
		} catch (LlmException e) {
			arena.close();
			throw new RuntimeException("Failed to create LlmClient: " + e.getMessage(), e);
		} catch (Throwable t) {
			arena.close();
			throw new RuntimeException("failed to create native client", t);
		}
		if (handle.equals(MemorySegment.NULL)) {
			String err = lastErrorMessage();
			arena.close();
			throw new RuntimeException(new LlmException(LlmException.CODE_UNKNOWN, "liter-llm: " + err));
		}
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
		String json = callJson(NativeMethods.CHAT, serialize(request));
		return deserialize(json, ChatCompletionResponse.class);
	}

	/**
	 * Sends a streaming chat completion request, invoking the callback for each
	 * chunk received via server-sent events (SSE).
	 *
	 * <p>
	 * The {@code stream} field on the request is forced to {@code true}.
	 *
	 * @param request
	 *            the chat completion request
	 * @param onChunk
	 *            callback invoked for each {@link ChatCompletionChunk} received;
	 *            must not be {@code null}
	 * @throws LlmException
	 *             if the request fails or the stream cannot be parsed
	 * @throws IllegalArgumentException
	 *             if {@code onChunk} is {@code null}
	 */
	public void chatStream(ChatCompletionRequest request, Consumer<ChatCompletionChunk> onChunk) throws LlmException {
		if (onChunk == null) {
			throw new IllegalArgumentException("onChunk callback must not be null");
		}
		// Force stream=true by creating a copy with the stream flag set.
		ChatCompletionRequest streamRequest = new ChatCompletionRequest(request.model(), request.messages(),
				request.temperature(), request.topP(), request.n(), Boolean.TRUE, request.stop(), request.maxTokens(),
				request.presencePenalty(), request.frequencyPenalty(), request.logitBias(), request.user(),
				request.tools(), request.toolChoice(), request.parallelToolCalls(), request.responseFormat(),
				request.streamOptions(), request.seed());
		String requestJson = serialize(streamRequest);

		try (var callArena = Arena.ofConfined()) {
			var cReq = callArena.allocateFrom(requestJson);

			// Create an upcall stub for the stream callback.
			var upcallStub = Linker.nativeLinker()
					.upcallStub(
							java.lang.invoke.MethodHandles.lookup()
									.bind(new StreamCallbackTarget(onChunk, OBJECT_MAPPER), "accept",
											java.lang.invoke.MethodType.methodType(void.class, MemorySegment.class,
													MemorySegment.class)),
							NativeMethods.STREAM_CALLBACK_DESCRIPTOR, callArena);

			int result = (int) NativeMethods.CHAT_STREAM.invokeExact(handle, cReq, upcallStub, MemorySegment.NULL);
			if (result != 0) {
				throw new LlmException.StreamException(lastErrorMessage());
			}
		} catch (LlmException e) {
			throw e;
		} catch (Throwable t) {
			throw new RuntimeException("FFI stream call failed", t);
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
		String json = callJson(NativeMethods.EMBED, serialize(request));
		return deserialize(json, EmbeddingResponse.class);
	}

	/**
	 * Lists available models for the configured provider.
	 *
	 * @return the list of available models
	 * @throws LlmException
	 *             if the request fails for any reason
	 */
	public ModelsListResponse listModels() throws LlmException {
		String json = callJsonNoArg(NativeMethods.LIST_MODELS);
		return deserialize(json, ModelsListResponse.class);
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
		String json = callJson(NativeMethods.IMAGE_GENERATE, serialize(request));
		return deserialize(json, ImagesResponse.class);
	}

	/**
	 * Generates audio speech from text, returning raw audio bytes.
	 *
	 * <p>
	 * The FFI layer returns the audio as a base64-encoded string; this method
	 * decodes it to raw bytes.
	 *
	 * @param request
	 *            the speech request
	 * @return raw audio bytes
	 * @throws LlmException
	 *             if the request fails for any reason
	 */
	public byte[] speech(CreateSpeechRequest request) throws LlmException {
		String base64 = callJson(NativeMethods.SPEECH, serialize(request));
		return Base64.getDecoder().decode(base64);
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
		String json = callJson(NativeMethods.TRANSCRIBE, serialize(request));
		return deserialize(json, TranscriptionResponse.class);
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
		String json = callJson(NativeMethods.MODERATE, serialize(request));
		return deserialize(json, ModerationResponse.class);
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
		String json = callJson(NativeMethods.RERANK, serialize(request));
		return deserialize(json, RerankResponse.class);
	}

	// ─── Search & OCR ────────────────────────────────────────────────────────

	/**
	 * Performs a web search using the configured provider.
	 *
	 * @param request
	 *            the search request
	 * @return the search response
	 * @throws LlmException
	 *             if the request fails for any reason
	 */
	public SearchResponse search(SearchRequest request) throws LlmException {
		String json = callJson(NativeMethods.SEARCH, serialize(request));
		return deserialize(json, SearchResponse.class);
	}

	/**
	 * Performs optical character recognition on images.
	 *
	 * @param request
	 *            the OCR request
	 * @return the OCR response
	 * @throws LlmException
	 *             if the request fails for any reason
	 */
	public OcrResponse ocr(OcrRequest request) throws LlmException {
		String json = callJson(NativeMethods.OCR, serialize(request));
		return deserialize(json, OcrResponse.class);
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
		String json = callJson(NativeMethods.CREATE_FILE, serialize(request));
		return deserialize(json, FileObject.class);
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
		String json = callJsonStringArg(NativeMethods.RETRIEVE_FILE, fileId);
		return deserialize(json, FileObject.class);
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
		String json = callJsonStringArg(NativeMethods.DELETE_FILE, fileId);
		return deserialize(json, DeleteResponse.class);
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
		String queryJson = query != null ? serialize(query) : null;
		String json = callJsonNullable(NativeMethods.LIST_FILES, queryJson);
		return deserialize(json, FileListResponse.class);
	}

	/**
	 * Retrieves the raw content of a file.
	 *
	 * <p>
	 * The FFI layer returns the content as a base64-encoded string; this method
	 * decodes it to raw bytes.
	 *
	 * @param fileId
	 *            the file identifier
	 * @return raw file content as bytes
	 * @throws LlmException
	 *             if the request fails for any reason
	 */
	public byte[] fileContent(String fileId) throws LlmException {
		String base64 = callJsonStringArg(NativeMethods.FILE_CONTENT, fileId);
		return Base64.getDecoder().decode(base64);
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
		String json = callJson(NativeMethods.CREATE_BATCH, serialize(request));
		return deserialize(json, BatchObject.class);
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
		String json = callJsonStringArg(NativeMethods.RETRIEVE_BATCH, batchId);
		return deserialize(json, BatchObject.class);
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
		String queryJson = query != null ? serialize(query) : null;
		String json = callJsonNullable(NativeMethods.LIST_BATCHES, queryJson);
		return deserialize(json, BatchListResponse.class);
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
		String json = callJsonStringArg(NativeMethods.CANCEL_BATCH, batchId);
		return deserialize(json, BatchObject.class);
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
		String json = callJson(NativeMethods.CREATE_RESPONSE, serialize(request));
		return deserialize(json, ResponseObject.class);
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
		String json = callJsonStringArg(NativeMethods.RETRIEVE_RESPONSE, responseId);
		return deserialize(json, ResponseObject.class);
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
		String json = callJsonStringArg(NativeMethods.CANCEL_RESPONSE, responseId);
		return deserialize(json, ResponseObject.class);
	}

	// ─── Hooks & Custom Providers ─────────────────────────────────────────────

	/**
	 * Registers a custom provider configuration at runtime. The provider is
	 * registered globally in the Rust core.
	 *
	 * @param config
	 *            the provider configuration to register
	 * @throws LlmException
	 *             if registration fails
	 */
	public void registerProvider(ProviderConfig config) throws LlmException {
		try (var callArena = Arena.ofConfined()) {
			String configJson = serialize(config);
			var cJson = callArena.allocateFrom(configJson);
			int result = (int) NativeMethods.REGISTER_PROVIDER.invokeExact(cJson);
			if (result != 0) {
				throw new LlmException(LlmException.CODE_INVALID_REQUEST, "liter-llm: " + lastErrorMessage());
			}
		} catch (LlmException e) {
			throw e;
		} catch (Throwable t) {
			throw new RuntimeException("FFI register_provider call failed", t);
		}
	}

	/**
	 * Unregisters a previously registered custom provider by name.
	 *
	 * @param name
	 *            the provider name to unregister
	 * @return {@code true} if a provider with that name was found and removed,
	 *         {@code false} if no provider with that name existed
	 * @throws LlmException
	 *             if the operation fails
	 */
	public boolean unregisterProvider(String name) throws LlmException {
		try (var callArena = Arena.ofConfined()) {
			var cName = callArena.allocateFrom(name);
			int result = (int) NativeMethods.UNREGISTER_PROVIDER.invokeExact(cName);
			if (result == -1) {
				throw new LlmException(LlmException.CODE_UNKNOWN, "liter-llm: " + lastErrorMessage());
			}
			return result == 0;
		} catch (LlmException e) {
			throw e;
		} catch (Throwable t) {
			throw new RuntimeException("FFI unregister_provider call failed", t);
		}
	}

	/**
	 * Returns the cumulative global spend tracked by the budget layer, in USD.
	 *
	 * @return spend in USD, or {@code 0.0} if no budget is configured
	 */
	public double budgetUsage() {
		try {
			return (double) NativeMethods.BUDGET_USAGE.invokeExact(handle);
		} catch (Throwable t) {
			throw new RuntimeException("FFI budget_usage call failed", t);
		}
	}

	/**
	 * Returns the version string of the native liter-llm library.
	 *
	 * @return version string (e.g. "1.0.0-rc.5")
	 */
	public static String version() {
		try {
			var cVersion = (MemorySegment) NativeMethods.VERSION.invokeExact();
			if (cVersion.equals(MemorySegment.NULL)) {
				return "unknown";
			}
			// Version string is static; do not free.
			return cVersion.reinterpret(Long.MAX_VALUE).getString(0);
		} catch (Throwable t) {
			throw new RuntimeException("FFI version call failed", t);
		}
	}

	/**
	 * Closes the native client handle and releases all associated memory.
	 *
	 * <p>
	 * After this method returns, the client must not be used.
	 */
	@Override
	public void close() {
		if (!closed.compareAndSet(false, true)) {
			return;
		}
		try {
			NativeMethods.CLIENT_FREE.invokeExact(handle);
		} catch (Throwable t) {
			// literllm_client_free is best-effort; errors during cleanup are ignored.
		}
		arena.close();
	}

	// ─── FFI Call Helpers ─────────────────────────────────────────────────────

	/**
	 * Invokes a two-argument FFI function: {@code char* fn(client, request_json)}.
	 * Frees the returned string after reading it.
	 */
	private String callJson(java.lang.invoke.MethodHandle mh, String requestJson) throws LlmException {
		try (var callArena = Arena.ofConfined()) {
			var cReq = callArena.allocateFrom(requestJson);
			var cResp = (MemorySegment) mh.invokeExact(handle, cReq);
			if (cResp.equals(MemorySegment.NULL)) {
				throw new LlmException(LlmException.CODE_PROVIDER_ERROR, "liter-llm: " + lastErrorMessage());
			}
			String result = cResp.reinterpret(Long.MAX_VALUE).getString(0);
			NativeMethods.FREE_STRING.invokeExact(cResp);
			return result;
		} catch (LlmException e) {
			throw e;
		} catch (Throwable t) {
			throw new RuntimeException("FFI call failed", t);
		}
	}

	/**
	 * Invokes a one-argument FFI function: {@code char* fn(client)}. Used for
	 * {@code literllm_list_models}.
	 */
	private String callJsonNoArg(java.lang.invoke.MethodHandle mh) throws LlmException {
		try {
			var cResp = (MemorySegment) mh.invokeExact(handle);
			if (cResp.equals(MemorySegment.NULL)) {
				throw new LlmException(LlmException.CODE_PROVIDER_ERROR, "liter-llm: " + lastErrorMessage());
			}
			String result = cResp.reinterpret(Long.MAX_VALUE).getString(0);
			NativeMethods.FREE_STRING.invokeExact(cResp);
			return result;
		} catch (LlmException e) {
			throw e;
		} catch (Throwable t) {
			throw new RuntimeException("FFI call failed", t);
		}
	}

	/**
	 * Invokes a two-argument FFI function where the second argument is a plain
	 * string (not JSON request body): {@code char* fn(client, id_string)}.
	 */
	private String callJsonStringArg(java.lang.invoke.MethodHandle mh, String arg) throws LlmException {
		try (var callArena = Arena.ofConfined()) {
			var cArg = callArena.allocateFrom(arg);
			var cResp = (MemorySegment) mh.invokeExact(handle, cArg);
			if (cResp.equals(MemorySegment.NULL)) {
				throw new LlmException(LlmException.CODE_PROVIDER_ERROR, "liter-llm: " + lastErrorMessage());
			}
			String result = cResp.reinterpret(Long.MAX_VALUE).getString(0);
			NativeMethods.FREE_STRING.invokeExact(cResp);
			return result;
		} catch (LlmException e) {
			throw e;
		} catch (Throwable t) {
			throw new RuntimeException("FFI call failed", t);
		}
	}

	/**
	 * Invokes a two-argument FFI function where the second argument may be
	 * {@code null}: {@code char* fn(client, nullable_json)}. Passes
	 * {@link MemorySegment#NULL} when the argument is {@code null}.
	 */
	private String callJsonNullable(java.lang.invoke.MethodHandle mh, String jsonOrNull) throws LlmException {
		try (var callArena = Arena.ofConfined()) {
			MemorySegment cArg = jsonOrNull != null ? callArena.allocateFrom(jsonOrNull) : MemorySegment.NULL;
			var cResp = (MemorySegment) mh.invokeExact(handle, cArg);
			if (cResp.equals(MemorySegment.NULL)) {
				throw new LlmException(LlmException.CODE_PROVIDER_ERROR, "liter-llm: " + lastErrorMessage());
			}
			String result = cResp.reinterpret(Long.MAX_VALUE).getString(0);
			NativeMethods.FREE_STRING.invokeExact(cResp);
			return result;
		} catch (LlmException e) {
			throw e;
		} catch (Throwable t) {
			throw new RuntimeException("FFI call failed", t);
		}
	}

	/**
	 * Reads the last error message from the native library's thread-local storage.
	 * The returned pointer must not be freed.
	 */
	private static String lastErrorMessage() {
		try {
			var cErr = (MemorySegment) NativeMethods.LAST_ERROR.invokeExact();
			if (cErr.equals(MemorySegment.NULL)) {
				return "unknown error";
			}
			return cErr.reinterpret(Long.MAX_VALUE).getString(0);
		} catch (Throwable t) {
			return "unknown error (failed to read native error: " + t.getMessage() + ")";
		}
	}

	// ─── Serialization Helpers ────────────────────────────────────────────────

	private static String serialize(Object value) throws LlmException {
		try {
			return OBJECT_MAPPER.writeValueAsString(value);
		} catch (com.fasterxml.jackson.core.JsonProcessingException e) {
			throw new LlmException.SerializationException("failed to serialize request", e);
		}
	}

	private static <T> T deserialize(String json, Class<T> type) throws LlmException {
		try {
			return OBJECT_MAPPER.readValue(json, type);
		} catch (com.fasterxml.jackson.core.JsonProcessingException e) {
			throw new LlmException.SerializationException("failed to deserialize " + type.getSimpleName() + " response",
					e);
		}
	}

	// ─── Stream Callback Target ──────────────────────────────────────────────

	/**
	 * Bound target for the native stream callback upcall. Each invocation
	 * deserializes the chunk JSON and forwards it to the user's {@link Consumer}.
	 */
	private static final class StreamCallbackTarget {

		private final Consumer<ChatCompletionChunk> onChunk;
		private final ObjectMapper mapper;

		StreamCallbackTarget(Consumer<ChatCompletionChunk> onChunk, ObjectMapper mapper) {
			this.onChunk = onChunk;
			this.mapper = mapper;
		}

		/** Called by the native upcall stub for each SSE chunk. */
		@SuppressWarnings("unused")
		public void accept(MemorySegment chunkJson, MemorySegment userData) {
			try {
				String json = chunkJson.reinterpret(Long.MAX_VALUE).getString(0);
				ChatCompletionChunk chunk = mapper.readValue(json, ChatCompletionChunk.class);
				onChunk.accept(chunk);
			} catch (Exception e) {
				// Stream callbacks cannot propagate exceptions to the Rust side.
				// Log and continue to avoid crashing the native runtime.
				System.err.println("liter-llm: stream callback error: " + e.getMessage());
			}
		}
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
		private String baseUrl;
		private String modelHint;
		private int maxRetries = DEFAULT_MAX_RETRIES;
		private Duration timeout = DEFAULT_TIMEOUT;
		private CacheConfig cacheConfig;
		private BudgetConfig budgetConfig;
		private Map<String, String> extraHeaders;
		private Integer cooldownSeconds;
		private Integer rateLimitRpm;
		private Integer rateLimitTpm;
		private Integer healthCheckSeconds;
		private boolean costTracking;
		private boolean tracing;

		private Builder() {
		}

		/**
		 * Sets the API key sent for provider authentication. Never log or serialize
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
		 * Sets the base URL for the API endpoint. Override to target a different
		 * provider or a local proxy.
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
		 * Sets a model hint for automatic provider detection (e.g.
		 * {@code "groq/llama3-70b"}).
		 *
		 * @param modelHint
		 *            the model name hint
		 * @return this builder
		 */
		public Builder modelHint(String modelHint) {
			this.modelHint = modelHint;
			return this;
		}

		/**
		 * Sets the maximum number of retries for retryable errors.
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
		 * Sets the request timeout.
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
		 * Sets extra HTTP headers to include in every request.
		 *
		 * @param extraHeaders
		 *            header name-value pairs
		 * @return this builder
		 */
		public Builder extraHeaders(Map<String, String> extraHeaders) {
			this.extraHeaders = extraHeaders;
			return this;
		}

		/**
		 * Sets the cooldown period in seconds after a provider error before retrying
		 * that provider.
		 *
		 * @param seconds
		 *            cooldown duration in seconds
		 * @return this builder
		 */
		public Builder cooldown(int seconds) {
			this.cooldownSeconds = seconds;
			return this;
		}

		/**
		 * Sets the rate limit for requests per minute and tokens per minute.
		 *
		 * @param rpm
		 *            maximum requests per minute
		 * @param tpm
		 *            maximum tokens per minute
		 * @return this builder
		 */
		public Builder rateLimit(int rpm, int tpm) {
			this.rateLimitRpm = rpm;
			this.rateLimitTpm = tpm;
			return this;
		}

		/**
		 * Sets the interval in seconds for provider health checks.
		 *
		 * @param seconds
		 *            health check interval in seconds
		 * @return this builder
		 */
		public Builder healthCheck(int seconds) {
			this.healthCheckSeconds = seconds;
			return this;
		}

		/**
		 * Enables or disables cost tracking for requests.
		 *
		 * @param enabled
		 *            whether cost tracking is enabled
		 * @return this builder
		 */
		public Builder costTracking(boolean enabled) {
			this.costTracking = enabled;
			return this;
		}

		/**
		 * Enables or disables distributed tracing for requests.
		 *
		 * @param enabled
		 *            whether tracing is enabled
		 * @return this builder
		 */
		public Builder tracing(boolean enabled) {
			this.tracing = enabled;
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

		/**
		 * Converts builder state to the JSON configuration map expected by the FFI
		 * {@code literllm_client_new_with_config} function.
		 */
		Map<String, Object> toConfigMap() {
			var config = new LinkedHashMap<String, Object>();
			config.put("api_key", apiKey);
			if (baseUrl != null) {
				config.put("base_url", baseUrl);
			}
			if (modelHint != null) {
				config.put("model_hint", modelHint);
			}
			config.put("max_retries", maxRetries);
			config.put("timeout_secs", timeout.toSeconds());
			if (extraHeaders != null && !extraHeaders.isEmpty()) {
				config.put("extra_headers", extraHeaders);
			}
			if (cacheConfig != null) {
				var cache = new LinkedHashMap<String, Object>();
				cache.put("max_entries", cacheConfig.maxEntries());
				cache.put("ttl_secs", cacheConfig.ttlSeconds());
				config.put("cache", cache);
			}
			if (budgetConfig != null) {
				var budget = new LinkedHashMap<String, Object>();
				if (budgetConfig.globalLimit() != null) {
					budget.put("global_limit", budgetConfig.globalLimit());
				}
				if (budgetConfig.modelLimits() != null && !budgetConfig.modelLimits().isEmpty()) {
					budget.put("model_limits", budgetConfig.modelLimits());
				}
				if (budgetConfig.enforcement() != null) {
					budget.put("enforcement", budgetConfig.enforcement());
				}
				config.put("budget", budget);
			}
			if (cooldownSeconds != null) {
				config.put("cooldown_secs", cooldownSeconds);
			}
			if (rateLimitRpm != null && rateLimitTpm != null) {
				var rateLimit = new LinkedHashMap<String, Object>();
				rateLimit.put("rpm", rateLimitRpm);
				rateLimit.put("tpm", rateLimitTpm);
				config.put("rate_limit", rateLimit);
			}
			if (healthCheckSeconds != null) {
				config.put("health_check_secs", healthCheckSeconds);
			}
			if (costTracking) {
				config.put("cost_tracking", true);
			}
			if (tracing) {
				config.put("tracing", true);
			}
			return config;
		}
	}
}
