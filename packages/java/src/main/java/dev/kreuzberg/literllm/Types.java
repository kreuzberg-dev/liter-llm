package dev.kreuzberg.literllm;

import com.fasterxml.jackson.annotation.JsonCreator;
import com.fasterxml.jackson.annotation.JsonIgnoreProperties;
import com.fasterxml.jackson.annotation.JsonInclude;
import com.fasterxml.jackson.annotation.JsonProperty;
import com.fasterxml.jackson.annotation.JsonSubTypes;
import com.fasterxml.jackson.annotation.JsonTypeInfo;
import com.fasterxml.jackson.annotation.JsonValue;
import com.fasterxml.jackson.databind.JsonNode;
import java.util.List;
import java.util.Map;

/**
 * Type definitions for the liter-llm unified LLM API.
 *
 * <p>
 * All types mirror the OpenAI-compatible wire format and match the Rust core
 * types in {@code
 * crates/liter-llm/src/types/}. Jackson annotations handle snake_case
 * serialization.
 */
@SuppressWarnings({"unused", "PMD.MissingStaticMethodInNonInstantiatableClass"})
public final class Types {

	private Types() {
	}

	// ─── Messages ─────────────────────────────────────────────────────────────

	/**
	 * A single turn in a chat conversation.
	 *
	 * <p>
	 * Use the concrete subtypes {@link SystemMessage}, {@link UserMessage},
	 * {@link AssistantMessage}, {@link ToolMessage}, {@link DeveloperMessage}, and
	 * {@link FunctionMessage}.
	 */
	@JsonTypeInfo(use = JsonTypeInfo.Id.NAME, property = "role")
	@JsonSubTypes({@JsonSubTypes.Type(value = SystemMessage.class, name = "system"),
			@JsonSubTypes.Type(value = UserMessage.class, name = "user"),
			@JsonSubTypes.Type(value = AssistantMessage.class, name = "assistant"),
			@JsonSubTypes.Type(value = ToolMessage.class, name = "tool"),
			@JsonSubTypes.Type(value = DeveloperMessage.class, name = "developer"),
			@JsonSubTypes.Type(value = FunctionMessage.class, name = "function")})
	public sealed interface Message
			permits SystemMessage, UserMessage, AssistantMessage, ToolMessage, DeveloperMessage, FunctionMessage {
	}

	/** A system-role message providing context or instructions to the model. */
	@JsonInclude(JsonInclude.Include.NON_NULL)
	@JsonIgnoreProperties(ignoreUnknown = true)
	public record SystemMessage(@JsonProperty("content") String content,
			@JsonProperty("name") String name) implements Message {

		/** Creates a system message with content only (no name). */
		public SystemMessage(String content) {
			this(content, null);
		}
	}

	/**
	 * A user-role message.
	 *
	 * <p>
	 * The {@code content} field may be a plain {@link String} or a
	 * {@code List<ContentPart>} for multi-modal inputs. Deserialize with
	 * {@link com.fasterxml.jackson.databind.ObjectMapper} configured with the
	 * liter-llm module.
	 */
	@JsonInclude(JsonInclude.Include.NON_NULL)
	@JsonIgnoreProperties(ignoreUnknown = true)
	public record UserMessage(@JsonProperty("content") Object content,
			@JsonProperty("name") String name) implements Message {

		/** Creates a text-only user message. */
		public UserMessage(String content) {
			this(content, null);
		}

		/** Creates a multi-part user message. */
		public UserMessage(List<ContentPart> parts) {
			this(parts, null);
		}
	}

	/** An assistant-role message, typically the model's reply. */
	@JsonInclude(JsonInclude.Include.NON_NULL)
	@JsonIgnoreProperties(ignoreUnknown = true)
	public record AssistantMessage(@JsonProperty("content") String content, @JsonProperty("name") String name,
			@JsonProperty("tool_calls") List<ToolCall> toolCalls, @JsonProperty("refusal") String refusal,
			@JsonProperty("function_call") FunctionCall functionCall) implements Message {

		/** Creates an assistant message with content only. */
		public AssistantMessage(String content) {
			this(content, null, null, null, null);
		}

		/** Creates an assistant message with tool calls. */
		public AssistantMessage(List<ToolCall> toolCalls) {
			this(null, null, toolCalls, null, null);
		}
	}

	/** A tool-role message returning the result of a tool call. */
	@JsonInclude(JsonInclude.Include.NON_NULL)
	@JsonIgnoreProperties(ignoreUnknown = true)
	public record ToolMessage(@JsonProperty("content") String content, @JsonProperty("tool_call_id") String toolCallId,
			@JsonProperty("name") String name) implements Message {

		/** Creates a tool result message. */
		public ToolMessage(String content, String toolCallId) {
			this(content, toolCallId, null);
		}
	}

	/**
	 * A developer-role message (used by some providers for system-level guidance).
	 */
	@JsonInclude(JsonInclude.Include.NON_NULL)
	@JsonIgnoreProperties(ignoreUnknown = true)
	public record DeveloperMessage(@JsonProperty("content") String content,
			@JsonProperty("name") String name) implements Message {

		/** Creates a developer message with content only. */
		public DeveloperMessage(String content) {
			this(content, null);
		}
	}

	/**
	 * A function-role message.
	 *
	 * @deprecated Retained for API compatibility with legacy OpenAI function
	 *             calling.
	 */
	@Deprecated
	@JsonInclude(JsonInclude.Include.NON_NULL)
	@JsonIgnoreProperties(ignoreUnknown = true)
	public record FunctionMessage(@JsonProperty("content") String content,
			@JsonProperty("name") String name) implements Message {
	}

	// ─── Content Parts ────────────────────────────────────────────────────────

	/**
	 * A single part of a multi-modal message content array.
	 *
	 * <p>
	 * Use {@link TextPart} for text and {@link ImageUrlPart} for images.
	 */
	@JsonTypeInfo(use = JsonTypeInfo.Id.NAME, property = "type")
	@JsonSubTypes({@JsonSubTypes.Type(value = TextPart.class, name = "text"),
			@JsonSubTypes.Type(value = ImageUrlPart.class, name = "image_url")})
	public sealed interface ContentPart permits TextPart, ImageUrlPart {
	}

	/** A text segment within a multi-part message. */
	@JsonIgnoreProperties(ignoreUnknown = true)
	public record TextPart(@JsonProperty("text") String text) implements ContentPart {
	}

	/** An image segment within a multi-part message, referenced by URL. */
	@JsonIgnoreProperties(ignoreUnknown = true)
	public record ImageUrlPart(@JsonProperty("image_url") ImageUrl imageUrl) implements ContentPart {
	}

	/** An image URL with optional detail level. */
	@JsonInclude(JsonInclude.Include.NON_NULL)
	@JsonIgnoreProperties(ignoreUnknown = true)
	public record ImageUrl(@JsonProperty("url") String url, @JsonProperty("detail") ImageDetail detail) {

		/** Creates an image URL without an explicit detail level. */
		public ImageUrl(String url) {
			this(url, null);
		}
	}

	/** Controls the resolution at which the model processes an image. */
	public enum ImageDetail {
		@JsonProperty("low")
		LOW, @JsonProperty("high")
		HIGH, @JsonProperty("auto")
		AUTO;

		@JsonValue
		public String toJson() {
			return name().toLowerCase(java.util.Locale.ROOT);
		}
	}

	// ─── Tools ────────────────────────────────────────────────────────────────

	/**
	 * The type discriminator for tool and tool-call objects.
	 *
	 * <p>
	 * Per the OpenAI spec this is always {@code "function"}.
	 */
	public enum ToolType {
		@JsonProperty("function")
		FUNCTION;

		@JsonValue
		public String toJson() {
			return "function";
		}
	}

	/** Describes a function the model may call. */
	@JsonInclude(JsonInclude.Include.NON_NULL)
	@JsonIgnoreProperties(ignoreUnknown = true)
	public record ChatCompletionTool(@JsonProperty("type") ToolType type,
			@JsonProperty("function") FunctionDefinition function) {

		/** Creates a tool with type {@code FUNCTION}. */
		public ChatCompletionTool(FunctionDefinition function) {
			this(ToolType.FUNCTION, function);
		}
	}

	/** Schema description of a callable function. */
	@JsonInclude(JsonInclude.Include.NON_NULL)
	@JsonIgnoreProperties(ignoreUnknown = true)
	public record FunctionDefinition(@JsonProperty("name") String name, @JsonProperty("description") String description,
			@JsonProperty("parameters") JsonNode parameters, @JsonProperty("strict") Boolean strict) {

		/** Creates a function definition with a name only. */
		public FunctionDefinition(String name) {
			this(name, null, null, null);
		}

		/** Creates a function definition with a name and description. */
		public FunctionDefinition(String name, String description) {
			this(name, description, null, null);
		}

		/**
		 * Creates a function definition with name, description, and JSON Schema
		 * parameters.
		 */
		public FunctionDefinition(String name, String description, JsonNode parameters) {
			this(name, description, parameters, null);
		}
	}

	/** A request from the model to invoke a named tool. */
	@JsonIgnoreProperties(ignoreUnknown = true)
	public record ToolCall(@JsonProperty("id") String id, @JsonProperty("type") ToolType type,
			@JsonProperty("function") FunctionCall function) {
	}

	/** The name and JSON-encoded arguments for a tool call. */
	@JsonIgnoreProperties(ignoreUnknown = true)
	public record FunctionCall(@JsonProperty("name") String name, @JsonProperty("arguments") String arguments) {
	}

	// ─── Tool Choice ──────────────────────────────────────────────────────────

	/**
	 * Controls whether and how the model calls tools.
	 *
	 * <p>
	 * Use {@link #AUTO}, {@link #REQUIRED}, {@link #NONE}, or
	 * {@link #forFunction(String)} for a specific function.
	 */
	public sealed interface ToolChoice permits ToolChoice.Mode, ToolChoice.Specific {

		/** Let the model decide whether to call a tool. */
		Mode AUTO = new Mode("auto");

		/** Force the model to call at least one tool. */
		Mode REQUIRED = new Mode("required");

		/** Prevent the model from calling any tools. */
		Mode NONE = new Mode("none");

		/**
		 * Returns a {@link Specific} tool choice that forces the model to call the
		 * named function.
		 */
		static Specific forFunction(String name) {
			return new Specific(ToolType.FUNCTION, new SpecificFunction(name));
		}

		/** A string-valued tool choice mode (auto / required / none). */
		@JsonIgnoreProperties(ignoreUnknown = true)
		record Mode(@JsonValue String value) implements ToolChoice {
			@JsonCreator
			public Mode {
			}
		}

		/** A tool-choice that targets a specific named function. */
		@JsonIgnoreProperties(ignoreUnknown = true)
		record Specific(@JsonProperty("type") ToolType type,
				@JsonProperty("function") SpecificFunction function) implements ToolChoice {
		}
	}

	/** Identifies a specific function by name for {@link ToolChoice.Specific}. */
	@JsonIgnoreProperties(ignoreUnknown = true)
	public record SpecificFunction(@JsonProperty("name") String name) {
	}

	// ─── Response Format ──────────────────────────────────────────────────────

	/**
	 * Instructs the model to produce output in a specific format.
	 *
	 * <p>
	 * Use {@link #TEXT}, {@link #JSON_OBJECT}, or
	 * {@link #jsonSchema(JsonSchemaFormat)}.
	 */
	@JsonTypeInfo(use = JsonTypeInfo.Id.NAME, property = "type")
	@JsonSubTypes({@JsonSubTypes.Type(value = ResponseFormat.Text.class, name = "text"),
			@JsonSubTypes.Type(value = ResponseFormat.JsonObject.class, name = "json_object"),
			@JsonSubTypes.Type(value = ResponseFormat.JsonSchema.class, name = "json_schema")})
	public sealed interface ResponseFormat
			permits ResponseFormat.Text, ResponseFormat.JsonObject, ResponseFormat.JsonSchema {

		/** Plain text output (the default). */
		Text TEXT = new Text();

		/** JSON object output without a predefined schema. */
		JsonObject JSON_OBJECT = new JsonObject();

		/** Creates a JSON Schema response format. */
		static JsonSchema jsonSchema(JsonSchemaFormat schema) {
			return new JsonSchema(schema);
		}

		/** Plain text response format. */
		record Text() implements ResponseFormat {
		}

		/** Unconstrained JSON object response format. */
		record JsonObject() implements ResponseFormat {
		}

		/** Structured JSON output constrained by a named schema. */
		record JsonSchema(@JsonProperty("json_schema") JsonSchemaFormat jsonSchema) implements ResponseFormat {
		}
	}

	/** Schema descriptor used with {@link ResponseFormat.JsonSchema}. */
	@JsonInclude(JsonInclude.Include.NON_NULL)
	@JsonIgnoreProperties(ignoreUnknown = true)
	public record JsonSchemaFormat(@JsonProperty("name") String name, @JsonProperty("description") String description,
			@JsonProperty("schema") JsonNode schema, @JsonProperty("strict") Boolean strict) {

		/** Creates a named schema without description or strict mode. */
		public JsonSchemaFormat(String name, JsonNode schema) {
			this(name, null, schema, null);
		}
	}

	// ─── Usage ────────────────────────────────────────────────────────────────

	/** Token consumption statistics for a completed request. */
	@JsonIgnoreProperties(ignoreUnknown = true)
	public record Usage(@JsonProperty("prompt_tokens") long promptTokens,
			@JsonProperty("completion_tokens") long completionTokens, @JsonProperty("total_tokens") long totalTokens) {
	}

	// ─── Stop Sequence ────────────────────────────────────────────────────────

	/**
	 * A stop sequence: either a single string or a list of strings.
	 *
	 * <p>
	 * Use {@link #of(String)} or {@link #of(List)}.
	 */
	public sealed interface StopSequence permits StopSequence.Single, StopSequence.Multiple {

		/** Creates a stop sequence from a single string. */
		static Single of(String stop) {
			return new Single(stop);
		}

		/** Creates a stop sequence from multiple strings. */
		static Multiple of(List<String> stops) {
			return new Multiple(stops);
		}

		/** A single stop string. */
		record Single(@JsonValue String value) implements StopSequence {
		}

		/** Multiple stop strings. */
		record Multiple(@JsonValue List<String> values) implements StopSequence {
		}
	}

	// ─── Chat Request ─────────────────────────────────────────────────────────

	/** Options that control streaming response behavior. */
	@JsonInclude(JsonInclude.Include.NON_NULL)
	@JsonIgnoreProperties(ignoreUnknown = true)
	public record StreamOptions(@JsonProperty("include_usage") Boolean includeUsage) {
	}

	/**
	 * Request body for a chat completion API call.
	 *
	 * <p>
	 * Only {@code model} and {@code messages} are required. All other fields are
	 * optional and omitted from the serialized JSON when {@code null}.
	 *
	 * <p>
	 * Use {@link ChatCompletionRequest.Builder} for a fluent construction API.
	 */
	@JsonInclude(JsonInclude.Include.NON_NULL)
	@JsonIgnoreProperties(ignoreUnknown = true)
	public record ChatCompletionRequest(@JsonProperty("model") String model,
			@JsonProperty("messages") List<Message> messages, @JsonProperty("temperature") Double temperature,
			@JsonProperty("top_p") Double topP, @JsonProperty("n") Integer n, @JsonProperty("stream") Boolean stream,
			@JsonProperty("stop") StopSequence stop, @JsonProperty("max_tokens") Long maxTokens,
			@JsonProperty("presence_penalty") Double presencePenalty,
			@JsonProperty("frequency_penalty") Double frequencyPenalty,
			@JsonProperty("logit_bias") Map<String, Double> logitBias, @JsonProperty("user") String user,
			@JsonProperty("tools") List<ChatCompletionTool> tools, @JsonProperty("tool_choice") ToolChoice toolChoice,
			@JsonProperty("parallel_tool_calls") Boolean parallelToolCalls,
			@JsonProperty("response_format") ResponseFormat responseFormat,
			@JsonProperty("stream_options") StreamOptions streamOptions, @JsonProperty("seed") Long seed) {

		/** Creates a minimal request with only model and messages. */
		public ChatCompletionRequest(String model, List<Message> messages) {
			this(model, messages, null, null, null, null, null, null, null, null, null, null, null, null, null, null,
					null, null);
		}

		/** Returns a new {@link Builder} for constructing a request. */
		public static Builder builder(String model, List<Message> messages) {
			return new Builder(model, messages);
		}

		/** Fluent builder for {@link ChatCompletionRequest}. */
		public static final class Builder {
			private final String model;
			private final List<Message> messages;
			private Double temperature;
			private Double topP;
			private Integer n;
			private Boolean stream;
			private StopSequence stop;
			private Long maxTokens;
			private Double presencePenalty;
			private Double frequencyPenalty;
			private Map<String, Double> logitBias;
			private String user;
			private List<ChatCompletionTool> tools;
			private ToolChoice toolChoice;
			private Boolean parallelToolCalls;
			private ResponseFormat responseFormat;
			private StreamOptions streamOptions;
			private Long seed;

			private Builder(String model, List<Message> messages) {
				this.model = model;
				this.messages = messages;
			}

			public Builder temperature(double temperature) {
				this.temperature = temperature;
				return this;
			}

			public Builder topP(double topP) {
				this.topP = topP;
				return this;
			}

			public Builder n(int n) {
				this.n = n;
				return this;
			}

			public Builder stream(boolean stream) {
				this.stream = stream;
				return this;
			}

			public Builder stop(StopSequence stop) {
				this.stop = stop;
				return this;
			}

			public Builder maxTokens(long maxTokens) {
				this.maxTokens = maxTokens;
				return this;
			}

			public Builder presencePenalty(double presencePenalty) {
				this.presencePenalty = presencePenalty;
				return this;
			}

			public Builder frequencyPenalty(double frequencyPenalty) {
				this.frequencyPenalty = frequencyPenalty;
				return this;
			}

			public Builder logitBias(Map<String, Double> logitBias) {
				this.logitBias = logitBias;
				return this;
			}

			public Builder user(String user) {
				this.user = user;
				return this;
			}

			public Builder tools(List<ChatCompletionTool> tools) {
				this.tools = tools;
				return this;
			}

			public Builder toolChoice(ToolChoice toolChoice) {
				this.toolChoice = toolChoice;
				return this;
			}

			public Builder parallelToolCalls(boolean parallelToolCalls) {
				this.parallelToolCalls = parallelToolCalls;
				return this;
			}

			public Builder responseFormat(ResponseFormat responseFormat) {
				this.responseFormat = responseFormat;
				return this;
			}

			public Builder streamOptions(StreamOptions streamOptions) {
				this.streamOptions = streamOptions;
				return this;
			}

			public Builder seed(long seed) {
				this.seed = seed;
				return this;
			}

			/** Builds the {@link ChatCompletionRequest}. */
			public ChatCompletionRequest build() {
				return new ChatCompletionRequest(model, messages, temperature, topP, n, stream, stop, maxTokens,
						presencePenalty, frequencyPenalty, logitBias, user, tools, toolChoice, parallelToolCalls,
						responseFormat, streamOptions, seed);
			}
		}
	}

	// ─── Chat Response ────────────────────────────────────────────────────────

	/** Why a choice stopped generating tokens. */
	public enum FinishReason {
		@JsonProperty("stop")
		STOP, @JsonProperty("length")
		LENGTH, @JsonProperty("tool_calls")
		TOOL_CALLS, @JsonProperty("content_filter")
		CONTENT_FILTER,
		/** Deprecated: legacy finish reason retained for API compatibility. */
		@JsonProperty("function_call")
		FUNCTION_CALL,
		/** Catch-all for unknown finish reasons returned by non-OpenAI providers. */
		OTHER;

		@JsonCreator
		public static FinishReason fromJson(String value) {
			return switch (value) {
				case "stop" -> STOP;
				case "length" -> LENGTH;
				case "tool_calls" -> TOOL_CALLS;
				case "content_filter" -> CONTENT_FILTER;
				case "function_call" -> FUNCTION_CALL;
				default -> OTHER;
			};
		}
	}

	/** One completion alternative in a {@link ChatCompletionResponse}. */
	@JsonIgnoreProperties(ignoreUnknown = true)
	public record Choice(@JsonProperty("index") int index, @JsonProperty("message") AssistantMessage message,
			@JsonProperty("finish_reason") FinishReason finishReason) {
	}

	/** Response body for a non-streaming chat completion request. */
	@JsonInclude(JsonInclude.Include.NON_NULL)
	@JsonIgnoreProperties(ignoreUnknown = true)
	public record ChatCompletionResponse(@JsonProperty("id") String id, @JsonProperty("object") String object,
			@JsonProperty("created") long created, @JsonProperty("model") String model,
			@JsonProperty("choices") List<Choice> choices, @JsonProperty("usage") Usage usage,
			@JsonProperty("system_fingerprint") String systemFingerprint,
			@JsonProperty("service_tier") String serviceTier) {
	}

	// ─── Stream Chunk ─────────────────────────────────────────────────────────

	/** Incremental content for one choice in a streaming response. */
	@JsonInclude(JsonInclude.Include.NON_NULL)
	@JsonIgnoreProperties(ignoreUnknown = true)
	public record StreamDelta(@JsonProperty("role") String role, @JsonProperty("content") String content,
			@JsonProperty("tool_calls") List<StreamToolCall> toolCalls,
			@JsonProperty("function_call") StreamFunctionCall functionCall, @JsonProperty("refusal") String refusal) {
	}

	/** An incremental update to a tool call within a streaming chunk. */
	@JsonInclude(JsonInclude.Include.NON_NULL)
	@JsonIgnoreProperties(ignoreUnknown = true)
	public record StreamToolCall(@JsonProperty("index") int index, @JsonProperty("id") String id,
			@JsonProperty("type") ToolType type, @JsonProperty("function") StreamFunctionCall function) {
	}

	/**
	 * Incremental function call fields within a streaming delta.
	 *
	 * @deprecated Retained for legacy function-call compatibility.
	 */
	@JsonInclude(JsonInclude.Include.NON_NULL)
	@JsonIgnoreProperties(ignoreUnknown = true)
	public record StreamFunctionCall(@JsonProperty("name") String name, @JsonProperty("arguments") String arguments) {
	}

	/** One choice entry within a {@link ChatCompletionChunk}. */
	@JsonInclude(JsonInclude.Include.NON_NULL)
	@JsonIgnoreProperties(ignoreUnknown = true)
	public record StreamChoice(@JsonProperty("index") int index, @JsonProperty("delta") StreamDelta delta,
			@JsonProperty("finish_reason") FinishReason finishReason) {
	}

	/** A single server-sent event emitted during a streaming chat completion. */
	@JsonInclude(JsonInclude.Include.NON_NULL)
	@JsonIgnoreProperties(ignoreUnknown = true)
	public record ChatCompletionChunk(@JsonProperty("id") String id, @JsonProperty("object") String object,
			@JsonProperty("created") long created, @JsonProperty("model") String model,
			@JsonProperty("choices") List<StreamChoice> choices, @JsonProperty("usage") Usage usage,
			@JsonProperty("service_tier") String serviceTier) {
	}

	// ─── Embedding ────────────────────────────────────────────────────────────

	/**
	 * Input for an embedding request.
	 *
	 * <p>
	 * Use {@link #of(String)} for a single text or {@link #of(List)} for a batch.
	 */
	public sealed interface EmbeddingInput permits EmbeddingInput.Single, EmbeddingInput.Multiple {

		/** Wraps a single string as an embedding input. */
		static Single of(String text) {
			return new Single(text);
		}

		/** Wraps multiple strings as an embedding input batch. */
		static Multiple of(List<String> texts) {
			return new Multiple(texts);
		}

		/** A single-string embedding input. */
		record Single(@JsonValue String value) implements EmbeddingInput {
		}

		/** A multi-string embedding input batch. */
		record Multiple(@JsonValue List<String> values) implements EmbeddingInput {
		}
	}

	/** Request body for an embedding API call. */
	@JsonInclude(JsonInclude.Include.NON_NULL)
	@JsonIgnoreProperties(ignoreUnknown = true)
	public record EmbeddingRequest(@JsonProperty("model") String model, @JsonProperty("input") EmbeddingInput input,
			@JsonProperty("encoding_format") String encodingFormat, @JsonProperty("dimensions") Integer dimensions,
			@JsonProperty("user") String user) {

		/** Creates an embedding request with model and input only. */
		public EmbeddingRequest(String model, EmbeddingInput input) {
			this(model, input, null, null, null);
		}
	}

	/** A single embedding vector returned by the API. */
	@JsonIgnoreProperties(ignoreUnknown = true)
	public record EmbeddingObject(@JsonProperty("object") String object,
			@JsonProperty("embedding") List<Double> embedding, @JsonProperty("index") int index) {
	}

	/** Response body for an embedding request. */
	@JsonIgnoreProperties(ignoreUnknown = true)
	public record EmbeddingResponse(@JsonProperty("object") String object,
			@JsonProperty("data") List<EmbeddingObject> data, @JsonProperty("model") String model,
			@JsonProperty("usage") Usage usage) {
	}

	// ─── Models ───────────────────────────────────────────────────────────────

	/** A single model entry returned by the list-models API. */
	@JsonIgnoreProperties(ignoreUnknown = true)
	public record ModelObject(@JsonProperty("id") String id, @JsonProperty("object") String object,
			@JsonProperty("created") long created, @JsonProperty("owned_by") String ownedBy) {
	}

	/** Response body for the list-models API. */
	@JsonIgnoreProperties(ignoreUnknown = true)
	public record ModelsListResponse(@JsonProperty("object") String object,
			@JsonProperty("data") List<ModelObject> data) {
	}

	// ─── Image Generation ─────────────────────────────────────────────────────

	/** Request body for an image generation API call. */
	@JsonInclude(JsonInclude.Include.NON_NULL)
	@JsonIgnoreProperties(ignoreUnknown = true)
	public record CreateImageRequest(@JsonProperty("prompt") String prompt, @JsonProperty("model") String model,
			@JsonProperty("n") Integer n, @JsonProperty("quality") String quality,
			@JsonProperty("response_format") String responseFormat, @JsonProperty("size") String size,
			@JsonProperty("style") String style, @JsonProperty("user") String user) {

		/** Creates an image request with a prompt only. */
		public CreateImageRequest(String prompt) {
			this(prompt, null, null, null, null, null, null, null);
		}
	}

	/** A single image result returned by the API. */
	@JsonInclude(JsonInclude.Include.NON_NULL)
	@JsonIgnoreProperties(ignoreUnknown = true)
	public record ImageData(@JsonProperty("url") String url, @JsonProperty("b64_json") String b64Json,
			@JsonProperty("revised_prompt") String revisedPrompt) {
	}

	/** Response body for an image generation request. */
	@JsonIgnoreProperties(ignoreUnknown = true)
	public record ImagesResponse(@JsonProperty("created") long created, @JsonProperty("data") List<ImageData> data) {
	}

	// ─── Speech ───────────────────────────────────────────────────────────────

	/** Request body for a speech generation API call. */
	@JsonInclude(JsonInclude.Include.NON_NULL)
	@JsonIgnoreProperties(ignoreUnknown = true)
	public record CreateSpeechRequest(@JsonProperty("model") String model, @JsonProperty("input") String input,
			@JsonProperty("voice") String voice, @JsonProperty("response_format") String responseFormat,
			@JsonProperty("speed") Double speed) {

		/** Creates a speech request with model, input, and voice only. */
		public CreateSpeechRequest(String model, String input, String voice) {
			this(model, input, voice, null, null);
		}
	}

	// ─── Transcription ────────────────────────────────────────────────────────

	/** Request body for an audio transcription API call. */
	@JsonInclude(JsonInclude.Include.NON_NULL)
	@JsonIgnoreProperties(ignoreUnknown = true)
	public record CreateTranscriptionRequest(@JsonProperty("file") String file, @JsonProperty("model") String model,
			@JsonProperty("language") String language, @JsonProperty("prompt") String prompt,
			@JsonProperty("response_format") String responseFormat, @JsonProperty("temperature") Double temperature) {

		/** Creates a transcription request with file and model only. */
		public CreateTranscriptionRequest(String file, String model) {
			this(file, model, null, null, null, null);
		}
	}

	/** Response body for a transcription request. */
	@JsonIgnoreProperties(ignoreUnknown = true)
	public record TranscriptionResponse(@JsonProperty("text") String text) {
	}

	// ─── Moderation ───────────────────────────────────────────────────────────

	/** Request body for a moderation API call. */
	@JsonInclude(JsonInclude.Include.NON_NULL)
	@JsonIgnoreProperties(ignoreUnknown = true)
	public record ModerationRequest(@JsonProperty("input") JsonNode input, @JsonProperty("model") String model) {
	}

	/** Per-category boolean flags for a moderation result. */
	@JsonIgnoreProperties(ignoreUnknown = true)
	public record ModerationCategories(@JsonProperty("sexual") boolean sexual, @JsonProperty("hate") boolean hate,
			@JsonProperty("harassment") boolean harassment, @JsonProperty("self-harm") boolean selfHarm,
			@JsonProperty("violence") boolean violence, @JsonProperty("sexual/minors") boolean sexualMinors,
			@JsonProperty("hate/threatening") boolean hateThreatening,
			@JsonProperty("violence/graphic") boolean violenceGraphic,
			@JsonProperty("self-harm/intent") boolean selfHarmIntent,
			@JsonProperty("self-harm/instructions") boolean selfHarmInstructions,
			@JsonProperty("harassment/threatening") boolean harassmentThreatening) {
	}

	/** Per-category confidence scores for a moderation result. */
	@JsonIgnoreProperties(ignoreUnknown = true)
	public record ModerationCategoryScores(@JsonProperty("sexual") double sexual, @JsonProperty("hate") double hate,
			@JsonProperty("harassment") double harassment, @JsonProperty("self-harm") double selfHarm,
			@JsonProperty("violence") double violence, @JsonProperty("sexual/minors") double sexualMinors,
			@JsonProperty("hate/threatening") double hateThreatening,
			@JsonProperty("violence/graphic") double violenceGraphic,
			@JsonProperty("self-harm/intent") double selfHarmIntent,
			@JsonProperty("self-harm/instructions") double selfHarmInstructions,
			@JsonProperty("harassment/threatening") double harassmentThreatening) {
	}

	/** A single moderation result for one input. */
	@JsonIgnoreProperties(ignoreUnknown = true)
	public record ModerationResult(@JsonProperty("flagged") boolean flagged,
			@JsonProperty("categories") ModerationCategories categories,
			@JsonProperty("category_scores") ModerationCategoryScores categoryScores) {
	}

	/** Response body for a moderation request. */
	@JsonIgnoreProperties(ignoreUnknown = true)
	public record ModerationResponse(@JsonProperty("id") String id, @JsonProperty("model") String model,
			@JsonProperty("results") List<ModerationResult> results) {
	}

	// ─── Rerank ───────────────────────────────────────────────────────────────

	/** Request body for a rerank API call. */
	@JsonInclude(JsonInclude.Include.NON_NULL)
	@JsonIgnoreProperties(ignoreUnknown = true)
	public record RerankRequest(@JsonProperty("model") String model, @JsonProperty("query") String query,
			@JsonProperty("documents") JsonNode documents, @JsonProperty("top_n") Integer topN) {

		/** Creates a rerank request without top_n. */
		public RerankRequest(String model, String query, JsonNode documents) {
			this(model, query, documents, null);
		}
	}

	/** A single reranked document result. */
	@JsonIgnoreProperties(ignoreUnknown = true)
	public record RerankResult(@JsonProperty("index") int index,
			@JsonProperty("relevance_score") double relevanceScore) {
	}

	/** Response body for a rerank request. */
	@JsonInclude(JsonInclude.Include.NON_NULL)
	@JsonIgnoreProperties(ignoreUnknown = true)
	public record RerankResponse(@JsonProperty("results") List<RerankResult> results,
			@JsonProperty("model") String model, @JsonProperty("usage") Usage usage) {
	}

	// ─── Files ────────────────────────────────────────────────────────────────

	/** Request body for a file upload API call. */
	@JsonInclude(JsonInclude.Include.NON_NULL)
	@JsonIgnoreProperties(ignoreUnknown = true)
	public record CreateFileRequest(@JsonProperty("file") String file, @JsonProperty("purpose") String purpose,
			@JsonProperty("filename") String filename) {

		/** Creates a file upload request without a filename override. */
		public CreateFileRequest(String file, String purpose) {
			this(file, purpose, null);
		}
	}

	/** Metadata for an uploaded file. */
	@JsonInclude(JsonInclude.Include.NON_NULL)
	@JsonIgnoreProperties(ignoreUnknown = true)
	public record FileObject(@JsonProperty("id") String id, @JsonProperty("object") String object,
			@JsonProperty("bytes") long bytes, @JsonProperty("created_at") long createdAt,
			@JsonProperty("filename") String filename, @JsonProperty("purpose") String purpose,
			@JsonProperty("status") String status, @JsonProperty("status_details") String statusDetails) {
	}

	/** Response body for a delete operation. */
	@JsonIgnoreProperties(ignoreUnknown = true)
	public record DeleteResponse(@JsonProperty("id") String id, @JsonProperty("object") String object,
			@JsonProperty("deleted") boolean deleted) {
	}

	/** Optional query parameters for listing files. */
	@JsonInclude(JsonInclude.Include.NON_NULL)
	@JsonIgnoreProperties(ignoreUnknown = true)
	public record FileListQuery(@JsonProperty("purpose") String purpose, @JsonProperty("limit") Integer limit,
			@JsonProperty("after") String after) {
	}

	/** Response body for listing files. */
	@JsonIgnoreProperties(ignoreUnknown = true)
	public record FileListResponse(@JsonProperty("object") String object, @JsonProperty("data") List<FileObject> data) {
	}

	// ─── Batches ──────────────────────────────────────────────────────────────

	/** Request body for creating a batch job. */
	@JsonInclude(JsonInclude.Include.NON_NULL)
	@JsonIgnoreProperties(ignoreUnknown = true)
	public record CreateBatchRequest(@JsonProperty("input_file_id") String inputFileId,
			@JsonProperty("endpoint") String endpoint, @JsonProperty("completion_window") String completionWindow,
			@JsonProperty("metadata") Map<String, String> metadata) {

		/** Creates a batch request without metadata. */
		public CreateBatchRequest(String inputFileId, String endpoint, String completionWindow) {
			this(inputFileId, endpoint, completionWindow, null);
		}
	}

	/** Counts for a batch job. */
	@JsonIgnoreProperties(ignoreUnknown = true)
	public record BatchRequestCounts(@JsonProperty("total") long total, @JsonProperty("completed") long completed,
			@JsonProperty("failed") long failed) {
	}

	/** Metadata for a batch processing job. */
	@JsonInclude(JsonInclude.Include.NON_NULL)
	@JsonIgnoreProperties(ignoreUnknown = true)
	public record BatchObject(@JsonProperty("id") String id, @JsonProperty("object") String object,
			@JsonProperty("endpoint") String endpoint, @JsonProperty("input_file_id") String inputFileId,
			@JsonProperty("completion_window") String completionWindow, @JsonProperty("status") String status,
			@JsonProperty("output_file_id") String outputFileId, @JsonProperty("error_file_id") String errorFileId,
			@JsonProperty("created_at") long createdAt, @JsonProperty("in_progress_at") Long inProgressAt,
			@JsonProperty("expires_at") Long expiresAt, @JsonProperty("finalizing_at") Long finalizingAt,
			@JsonProperty("completed_at") Long completedAt, @JsonProperty("failed_at") Long failedAt,
			@JsonProperty("expired_at") Long expiredAt, @JsonProperty("cancelling_at") Long cancellingAt,
			@JsonProperty("cancelled_at") Long cancelledAt,
			@JsonProperty("request_counts") BatchRequestCounts requestCounts,
			@JsonProperty("metadata") Map<String, String> metadata) {
	}

	/** Optional query parameters for listing batches. */
	@JsonInclude(JsonInclude.Include.NON_NULL)
	@JsonIgnoreProperties(ignoreUnknown = true)
	public record BatchListQuery(@JsonProperty("limit") Integer limit, @JsonProperty("after") String after) {
	}

	/** Response body for listing batches. */
	@JsonIgnoreProperties(ignoreUnknown = true)
	public record BatchListResponse(@JsonProperty("object") String object,
			@JsonProperty("data") List<BatchObject> data) {
	}

	// ─── Responses API ────────────────────────────────────────────────────────

	/** Request body for creating a response via the Responses API. */
	@JsonInclude(JsonInclude.Include.NON_NULL)
	@JsonIgnoreProperties(ignoreUnknown = true)
	public record CreateResponseRequest(@JsonProperty("model") String model, @JsonProperty("input") JsonNode input,
			@JsonProperty("instructions") String instructions, @JsonProperty("max_output_tokens") Long maxOutputTokens,
			@JsonProperty("temperature") Double temperature, @JsonProperty("top_p") Double topP,
			@JsonProperty("stream") Boolean stream, @JsonProperty("metadata") Map<String, String> metadata) {

		/** Creates a response request with model and input only. */
		public CreateResponseRequest(String model, JsonNode input) {
			this(model, input, null, null, null, null, null, null);
		}
	}

	/** A response object from the Responses API. */
	@JsonInclude(JsonInclude.Include.NON_NULL)
	@JsonIgnoreProperties(ignoreUnknown = true)
	public record ResponseObject(@JsonProperty("id") String id, @JsonProperty("object") String object,
			@JsonProperty("created_at") long createdAt, @JsonProperty("status") String status,
			@JsonProperty("model") String model, @JsonProperty("output") JsonNode output,
			@JsonProperty("usage") Usage usage, @JsonProperty("metadata") Map<String, String> metadata,
			@JsonProperty("error") JsonNode error) {
	}
}
