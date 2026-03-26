using System.Text.Json;
using System.Text.Json.Nodes;
using System.Text.Json.Serialization;

namespace LiterLlm;

// ─── Messages ─────────────────────────────────────────────────────────────────

/// <summary>
/// A single turn in a chat conversation.
/// </summary>
/// <remarks>
/// Use the concrete subtypes: <see cref="SystemMessage"/>, <see cref="UserMessage"/>,
/// <see cref="AssistantMessage"/>, <see cref="ToolMessage"/>, <see cref="DeveloperMessage"/>,
/// and <see cref="FunctionMessage"/>.
/// </remarks>
[JsonPolymorphic(TypeDiscriminatorPropertyName = "role")]
[JsonDerivedType(typeof(SystemMessage), "system")]
[JsonDerivedType(typeof(UserMessage), "user")]
[JsonDerivedType(typeof(AssistantMessage), "assistant")]
[JsonDerivedType(typeof(ToolMessage), "tool")]
[JsonDerivedType(typeof(DeveloperMessage), "developer")]
[JsonDerivedType(typeof(FunctionMessage), "function")]
public abstract record Message;

/// <summary>A system-role message providing context or instructions to the model.</summary>
public record SystemMessage(
    [property: JsonPropertyName("content")] string Content,
    [property: JsonPropertyName("name"), JsonIgnore(Condition = JsonIgnoreCondition.WhenWritingNull)]
    string? Name = null) : Message;

/// <summary>
/// A user-role message.
/// </summary>
/// <remarks>
/// The <see cref="Content"/> field may be a plain <see cref="string"/> or a
/// <c>List&lt;ContentPart&gt;</c> for multi-modal inputs. Deserialize via
/// <see cref="LiterLlmJson.SerializerOptions"/>.
/// </remarks>
public record UserMessage(
    [property: JsonPropertyName("content")] object Content,
    [property: JsonPropertyName("name"), JsonIgnore(Condition = JsonIgnoreCondition.WhenWritingNull)]
    string? Name = null) : Message;

/// <summary>An assistant-role message, typically the model's reply.</summary>
public record AssistantMessage(
    [property: JsonPropertyName("content"), JsonIgnore(Condition = JsonIgnoreCondition.WhenWritingNull)]
    string? Content = null,
    [property: JsonPropertyName("name"), JsonIgnore(Condition = JsonIgnoreCondition.WhenWritingNull)]
    string? Name = null,
    [property: JsonPropertyName("tool_calls"), JsonIgnore(Condition = JsonIgnoreCondition.WhenWritingNull)]
    IReadOnlyList<ToolCall>? ToolCalls = null,
    [property: JsonPropertyName("refusal"), JsonIgnore(Condition = JsonIgnoreCondition.WhenWritingNull)]
    string? Refusal = null,
    /// <summary>Deprecated: legacy function_call field retained for API compatibility.</summary>
    [property: JsonPropertyName("function_call"), JsonIgnore(Condition = JsonIgnoreCondition.WhenWritingNull)]
    FunctionCall? FunctionCall = null) : Message;

/// <summary>A tool-role message returning the result of a tool call.</summary>
public record ToolMessage(
    [property: JsonPropertyName("content")] string Content,
    [property: JsonPropertyName("tool_call_id")] string ToolCallId,
    [property: JsonPropertyName("name"), JsonIgnore(Condition = JsonIgnoreCondition.WhenWritingNull)]
    string? Name = null) : Message;

/// <summary>A developer-role message (used by some providers for system-level guidance).</summary>
public record DeveloperMessage(
    [property: JsonPropertyName("content")] string Content,
    [property: JsonPropertyName("name"), JsonIgnore(Condition = JsonIgnoreCondition.WhenWritingNull)]
    string? Name = null) : Message;

/// <summary>A function-role message.</summary>
/// <remarks>Deprecated: retained for API compatibility with legacy OpenAI function calling. Use tool messages instead.</remarks>
public record FunctionMessage(
    [property: JsonPropertyName("content")] string Content,
    [property: JsonPropertyName("name")] string Name) : Message;

// ─── Content Parts ────────────────────────────────────────────────────────────

/// <summary>A single part of a multi-modal message content array.</summary>
[JsonPolymorphic(TypeDiscriminatorPropertyName = "type")]
[JsonDerivedType(typeof(TextPart), "text")]
[JsonDerivedType(typeof(ImageUrlPart), "image_url")]
public abstract record ContentPart;

/// <summary>A text segment within a multi-part message.</summary>
public record TextPart(
    [property: JsonPropertyName("text")] string Text) : ContentPart;

/// <summary>An image segment within a multi-part message, referenced by URL.</summary>
public record ImageUrlPart(
    [property: JsonPropertyName("image_url")] ImageUrl ImageUrl) : ContentPart;

/// <summary>An image URL with optional detail level.</summary>
public record ImageUrl(
    [property: JsonPropertyName("url")] string Url,
    [property: JsonPropertyName("detail"), JsonIgnore(Condition = JsonIgnoreCondition.WhenWritingNull)]
    ImageDetail? Detail = null);

/// <summary>Controls the resolution at which the model processes an image.</summary>
[JsonConverter(typeof(JsonStringEnumConverter<ImageDetail>))]
public enum ImageDetail
{
    [JsonStringEnumMemberName("low")] Low,
    [JsonStringEnumMemberName("high")] High,
    [JsonStringEnumMemberName("auto")] Auto,
}

// ─── Tools ────────────────────────────────────────────────────────────────────

/// <summary>
/// The type discriminator for tool and tool-call objects.
/// Per the OpenAI spec this is always <c>"function"</c>.
/// </summary>
[JsonConverter(typeof(JsonStringEnumConverter<ToolType>))]
public enum ToolType
{
    [JsonStringEnumMemberName("function")] Function,
}

/// <summary>Describes a function the model may call.</summary>
public record ChatCompletionTool(
    [property: JsonPropertyName("type")] ToolType Type,
    [property: JsonPropertyName("function")] FunctionDefinition Function)
{
    /// <summary>Creates a tool with type <see cref="ToolType.Function"/>.</summary>
    public ChatCompletionTool(FunctionDefinition function) : this(ToolType.Function, function) { }
}

/// <summary>Schema description of a callable function.</summary>
public record FunctionDefinition(
    [property: JsonPropertyName("name")] string Name,
    [property: JsonPropertyName("description"), JsonIgnore(Condition = JsonIgnoreCondition.WhenWritingNull)]
    string? Description = null,
    /// <summary>Parameters as a JSON Schema object.</summary>
    [property: JsonPropertyName("parameters"), JsonIgnore(Condition = JsonIgnoreCondition.WhenWritingNull)]
    JsonObject? Parameters = null,
    [property: JsonPropertyName("strict"), JsonIgnore(Condition = JsonIgnoreCondition.WhenWritingNull)]
    bool? Strict = null);

/// <summary>A request from the model to invoke a named tool.</summary>
public record ToolCall(
    [property: JsonPropertyName("id")] string Id,
    [property: JsonPropertyName("type")] ToolType Type,
    [property: JsonPropertyName("function")] FunctionCall Function);

/// <summary>The name and JSON-encoded arguments for a tool call.</summary>
public record FunctionCall(
    [property: JsonPropertyName("name")] string Name,
    [property: JsonPropertyName("arguments")] string Arguments);

// ─── Tool Choice ──────────────────────────────────────────────────────────────

/// <summary>
/// Controls whether and how the model calls tools.
/// </summary>
/// <remarks>
/// Use <see cref="ToolChoice.Auto"/>, <see cref="ToolChoice.Required"/>,
/// <see cref="ToolChoice.None"/>, or <see cref="ToolChoice.ForFunction(string)"/>
/// for a specific function.
/// </remarks>
[JsonConverter(typeof(ToolChoiceConverter))]
public abstract record ToolChoice
{
    /// <summary>Let the model decide whether to call a tool.</summary>
    public static readonly ToolChoice Auto = new ModeChoice("auto");

    /// <summary>Force the model to call at least one tool.</summary>
    public static readonly ToolChoice Required = new ModeChoice("required");

    /// <summary>Prevent the model from calling any tools.</summary>
    public static readonly ToolChoice None = new ModeChoice("none");

    /// <summary>
    /// Returns a <see cref="SpecificChoice"/> that forces the model to call the named function.
    /// </summary>
    public static ToolChoice ForFunction(string functionName) =>
        new SpecificChoice(ToolType.Function, new SpecificFunction(functionName));

    /// <summary>A string-valued tool choice mode (auto / required / none).</summary>
    public sealed record ModeChoice(string Value) : ToolChoice;

    /// <summary>A tool choice that targets a specific named function.</summary>
    public sealed record SpecificChoice(
        [property: JsonPropertyName("type")] ToolType Type,
        [property: JsonPropertyName("function")] SpecificFunction Function) : ToolChoice;
}

/// <summary>Identifies a specific function by name for <see cref="ToolChoice.SpecificChoice"/>.</summary>
public record SpecificFunction(
    [property: JsonPropertyName("name")] string Name);

/// <summary>Custom JSON converter that handles the dual-representation of ToolChoice.</summary>
internal sealed class ToolChoiceConverter : JsonConverter<ToolChoice>
{
    public override ToolChoice Read(ref Utf8JsonReader reader, Type typeToConvert, JsonSerializerOptions options)
    {
        if (reader.TokenType == JsonTokenType.String)
        {
            return new ToolChoice.ModeChoice(reader.GetString()!);
        }

        using var doc = JsonDocument.ParseValue(ref reader);
        var root = doc.RootElement;
        var type = root.GetProperty("type").GetString();
        var functionName = root.GetProperty("function").GetProperty("name").GetString()!;
        return ToolChoice.ForFunction(functionName);
    }

    public override void Write(Utf8JsonWriter writer, ToolChoice value, JsonSerializerOptions options)
    {
        switch (value)
        {
            case ToolChoice.ModeChoice mode:
                writer.WriteStringValue(mode.Value);
                break;
            case ToolChoice.SpecificChoice specific:
                writer.WriteStartObject();
                writer.WriteString("type", "function");
                writer.WritePropertyName("function");
                writer.WriteStartObject();
                writer.WriteString("name", specific.Function.Name);
                writer.WriteEndObject();
                writer.WriteEndObject();
                break;
            default:
                throw new JsonException($"Unknown ToolChoice subtype: {value.GetType()}");
        }
    }
}

// ─── Response Format ──────────────────────────────────────────────────────────

/// <summary>
/// Instructs the model to produce output in a specific format.
/// </summary>
[JsonPolymorphic(TypeDiscriminatorPropertyName = "type")]
[JsonDerivedType(typeof(TextFormat), "text")]
[JsonDerivedType(typeof(JsonObjectFormat), "json_object")]
[JsonDerivedType(typeof(JsonSchemaResponseFormat), "json_schema")]
public abstract record ResponseFormat
{
    /// <summary>Plain text output (the default).</summary>
    public static readonly ResponseFormat Text = new TextFormat();

    /// <summary>JSON object output without a predefined schema.</summary>
    public static readonly ResponseFormat JsonObject = new JsonObjectFormat();

    /// <summary>Creates a JSON Schema response format.</summary>
    public static ResponseFormat WithJsonSchema(JsonSchemaFormat schema) =>
        new JsonSchemaResponseFormat(schema);

    /// <summary>Plain text response format.</summary>
    public sealed record TextFormat() : ResponseFormat;

    /// <summary>Unconstrained JSON object response format.</summary>
    public sealed record JsonObjectFormat() : ResponseFormat;

    /// <summary>Structured JSON output constrained by a named schema.</summary>
    public sealed record JsonSchemaResponseFormat(
        [property: JsonPropertyName("json_schema")] JsonSchemaFormat JsonSchema) : ResponseFormat;
}

/// <summary>Schema descriptor used with <see cref="ResponseFormat.JsonSchemaResponseFormat"/>.</summary>
public record JsonSchemaFormat(
    [property: JsonPropertyName("name")] string Name,
    [property: JsonPropertyName("description"), JsonIgnore(Condition = JsonIgnoreCondition.WhenWritingNull)]
    string? Description = null,
    [property: JsonPropertyName("schema"), JsonIgnore(Condition = JsonIgnoreCondition.WhenWritingNull)]
    JsonObject? Schema = null,
    [property: JsonPropertyName("strict"), JsonIgnore(Condition = JsonIgnoreCondition.WhenWritingNull)]
    bool? Strict = null);

// ─── Usage ────────────────────────────────────────────────────────────────────

/// <summary>Token consumption statistics for a completed request.</summary>
public record Usage(
    [property: JsonPropertyName("prompt_tokens")] long PromptTokens,
    [property: JsonPropertyName("completion_tokens")] long CompletionTokens,
    [property: JsonPropertyName("total_tokens")] long TotalTokens);

// ─── Stop Sequence ────────────────────────────────────────────────────────────

/// <summary>
/// A stop sequence: either a single string or a list of strings.
/// </summary>
/// <remarks>
/// Use <see cref="StopSequence.FromString(string)"/> or
/// <see cref="StopSequence.FromList(IReadOnlyList{string})"/>.
/// </remarks>
[JsonConverter(typeof(StopSequenceConverter))]
public abstract record StopSequence
{
    /// <summary>Creates a stop sequence from a single string.</summary>
    public static StopSequence FromString(string stop) => new SingleStop(stop);

    /// <summary>Creates a stop sequence from multiple strings.</summary>
    public static StopSequence FromList(IReadOnlyList<string> stops) => new MultipleStop(stops);

    /// <summary>A single stop string.</summary>
    public sealed record SingleStop(string Value) : StopSequence;

    /// <summary>Multiple stop strings.</summary>
    public sealed record MultipleStop(IReadOnlyList<string> Values) : StopSequence;
}

/// <summary>Custom JSON converter that handles the string/array duality of StopSequence.</summary>
internal sealed class StopSequenceConverter : JsonConverter<StopSequence>
{
    public override StopSequence Read(ref Utf8JsonReader reader, Type typeToConvert, JsonSerializerOptions options)
    {
        if (reader.TokenType == JsonTokenType.String)
        {
            return StopSequence.FromString(reader.GetString()!);
        }

        var values = JsonSerializer.Deserialize<List<string>>(ref reader, options)!;
        return StopSequence.FromList(values);
    }

    public override void Write(Utf8JsonWriter writer, StopSequence value, JsonSerializerOptions options)
    {
        switch (value)
        {
            case StopSequence.SingleStop single:
                writer.WriteStringValue(single.Value);
                break;
            case StopSequence.MultipleStop multiple:
                JsonSerializer.Serialize(writer, multiple.Values, options);
                break;
            default:
                throw new JsonException($"Unknown StopSequence subtype: {value.GetType()}");
        }
    }
}

// ─── Chat Request ─────────────────────────────────────────────────────────────

/// <summary>Options that control streaming response behavior.</summary>
public record StreamOptions(
    [property: JsonPropertyName("include_usage"), JsonIgnore(Condition = JsonIgnoreCondition.WhenWritingNull)]
    bool? IncludeUsage = null);

/// <summary>
/// Request body for a chat completion API call.
/// </summary>
/// <remarks>
/// Only <see cref="Model"/> and <see cref="Messages"/> are required. All other
/// properties are optional and omitted from serialized JSON when <c>null</c>.
/// </remarks>
public record ChatCompletionRequest(
    [property: JsonPropertyName("model")] string Model,
    [property: JsonPropertyName("messages")] IReadOnlyList<Message> Messages,
    [property: JsonPropertyName("temperature"), JsonIgnore(Condition = JsonIgnoreCondition.WhenWritingNull)]
    double? Temperature = null,
    [property: JsonPropertyName("top_p"), JsonIgnore(Condition = JsonIgnoreCondition.WhenWritingNull)]
    double? TopP = null,
    [property: JsonPropertyName("n"), JsonIgnore(Condition = JsonIgnoreCondition.WhenWritingNull)]
    int? N = null,
    [property: JsonPropertyName("stream"), JsonIgnore(Condition = JsonIgnoreCondition.WhenWritingNull)]
    bool? Stream = null,
    [property: JsonPropertyName("stop"), JsonIgnore(Condition = JsonIgnoreCondition.WhenWritingNull)]
    StopSequence? Stop = null,
    [property: JsonPropertyName("max_tokens"), JsonIgnore(Condition = JsonIgnoreCondition.WhenWritingNull)]
    long? MaxTokens = null,
    [property: JsonPropertyName("presence_penalty"), JsonIgnore(Condition = JsonIgnoreCondition.WhenWritingNull)]
    double? PresencePenalty = null,
    [property: JsonPropertyName("frequency_penalty"), JsonIgnore(Condition = JsonIgnoreCondition.WhenWritingNull)]
    double? FrequencyPenalty = null,
    [property: JsonPropertyName("logit_bias"), JsonIgnore(Condition = JsonIgnoreCondition.WhenWritingNull)]
    IReadOnlyDictionary<string, double>? LogitBias = null,
    [property: JsonPropertyName("user"), JsonIgnore(Condition = JsonIgnoreCondition.WhenWritingNull)]
    string? User = null,
    [property: JsonPropertyName("tools"), JsonIgnore(Condition = JsonIgnoreCondition.WhenWritingNull)]
    IReadOnlyList<ChatCompletionTool>? Tools = null,
    [property: JsonPropertyName("tool_choice"), JsonIgnore(Condition = JsonIgnoreCondition.WhenWritingNull)]
    ToolChoice? ToolChoice = null,
    [property: JsonPropertyName("parallel_tool_calls"), JsonIgnore(Condition = JsonIgnoreCondition.WhenWritingNull)]
    bool? ParallelToolCalls = null,
    [property: JsonPropertyName("response_format"), JsonIgnore(Condition = JsonIgnoreCondition.WhenWritingNull)]
    ResponseFormat? ResponseFormat = null,
    [property: JsonPropertyName("stream_options"), JsonIgnore(Condition = JsonIgnoreCondition.WhenWritingNull)]
    StreamOptions? StreamOptions = null,
    [property: JsonPropertyName("seed"), JsonIgnore(Condition = JsonIgnoreCondition.WhenWritingNull)]
    long? Seed = null);

// ─── Chat Response ────────────────────────────────────────────────────────────

/// <summary>Why a choice stopped generating tokens.</summary>
[JsonConverter(typeof(FinishReasonConverter))]
public enum FinishReason
{
    Stop,
    Length,
    ToolCalls,
    ContentFilter,
    /// <summary>Deprecated: legacy finish reason retained for API compatibility.</summary>
    FunctionCall,
    /// <summary>Catch-all for unknown finish reasons returned by non-OpenAI providers.</summary>
    Other,
}

/// <summary>Custom converter that maps snake_case strings to <see cref="FinishReason"/>.</summary>
internal sealed class FinishReasonConverter : JsonConverter<FinishReason>
{
    public override FinishReason Read(ref Utf8JsonReader reader, Type typeToConvert, JsonSerializerOptions options)
    {
        var value = reader.GetString();
        return value switch
        {
            "stop" => FinishReason.Stop,
            "length" => FinishReason.Length,
            "tool_calls" => FinishReason.ToolCalls,
            "content_filter" => FinishReason.ContentFilter,
            "function_call" => FinishReason.FunctionCall,
            _ => FinishReason.Other,
        };
    }

    public override void Write(Utf8JsonWriter writer, FinishReason value, JsonSerializerOptions options)
    {
        var str = value switch
        {
            FinishReason.Stop => "stop",
            FinishReason.Length => "length",
            FinishReason.ToolCalls => "tool_calls",
            FinishReason.ContentFilter => "content_filter",
            FinishReason.FunctionCall => "function_call",
            FinishReason.Other => "other",
            _ => "other",
        };
        writer.WriteStringValue(str);
    }
}

/// <summary>One completion alternative in a <see cref="ChatCompletionResponse"/>.</summary>
public record Choice(
    [property: JsonPropertyName("index")] int Index,
    [property: JsonPropertyName("message")] AssistantMessage Message,
    [property: JsonPropertyName("finish_reason")] FinishReason? FinishReason);

/// <summary>Response body for a non-streaming chat completion request.</summary>
public record ChatCompletionResponse(
    [property: JsonPropertyName("id")] string Id,
    /// <summary>Always <c>"chat.completion"</c> from OpenAI-compatible APIs.</summary>
    [property: JsonPropertyName("object")] string Object,
    [property: JsonPropertyName("created")] long Created,
    [property: JsonPropertyName("model")] string Model,
    [property: JsonPropertyName("choices")] IReadOnlyList<Choice> Choices,
    [property: JsonPropertyName("usage")] Usage? Usage = null,
    [property: JsonPropertyName("system_fingerprint")] string? SystemFingerprint = null,
    [property: JsonPropertyName("service_tier")] string? ServiceTier = null);

// ─── Stream Chunk ─────────────────────────────────────────────────────────────

/// <summary>Incremental content for one choice in a streaming response.</summary>
public record StreamDelta(
    [property: JsonPropertyName("role")] string? Role = null,
    [property: JsonPropertyName("content")] string? Content = null,
    [property: JsonPropertyName("tool_calls")] IReadOnlyList<StreamToolCall>? ToolCalls = null,
    /// <summary>Deprecated: legacy function_call delta retained for API compatibility.</summary>
    [property: JsonPropertyName("function_call")] StreamFunctionCall? FunctionCall = null,
    [property: JsonPropertyName("refusal")] string? Refusal = null);

/// <summary>An incremental update to a tool call within a streaming chunk.</summary>
public record StreamToolCall(
    [property: JsonPropertyName("index")] int Index,
    [property: JsonPropertyName("id")] string? Id = null,
    [property: JsonPropertyName("type")] ToolType? Type = null,
    [property: JsonPropertyName("function")] StreamFunctionCall? Function = null);

/// <summary>Incremental function call fields within a streaming delta.</summary>
/// <remarks>Deprecated: retained for legacy function-call compatibility. Use <see cref="StreamToolCall"/> instead.</remarks>
public record StreamFunctionCall(
    [property: JsonPropertyName("name")] string? Name = null,
    [property: JsonPropertyName("arguments")] string? Arguments = null);

/// <summary>One choice entry within a <see cref="ChatCompletionChunk"/>.</summary>
public record StreamChoice(
    [property: JsonPropertyName("index")] int Index,
    [property: JsonPropertyName("delta")] StreamDelta Delta,
    [property: JsonPropertyName("finish_reason")] FinishReason? FinishReason);

/// <summary>A single server-sent event emitted during a streaming chat completion.</summary>
public record ChatCompletionChunk(
    [property: JsonPropertyName("id")] string Id,
    /// <summary>Always <c>"chat.completion.chunk"</c> from OpenAI-compatible APIs.</summary>
    [property: JsonPropertyName("object")] string Object,
    [property: JsonPropertyName("created")] long Created,
    [property: JsonPropertyName("model")] string Model,
    [property: JsonPropertyName("choices")] IReadOnlyList<StreamChoice> Choices,
    [property: JsonPropertyName("usage")] Usage? Usage = null,
    [property: JsonPropertyName("service_tier")] string? ServiceTier = null);

// ─── Embedding ────────────────────────────────────────────────────────────────

/// <summary>
/// Input for an embedding request.
/// </summary>
/// <remarks>
/// Use <see cref="EmbeddingInput.FromString(string)"/> for a single text or
/// <see cref="EmbeddingInput.FromList(IReadOnlyList{string})"/> for a batch.
/// </remarks>
[JsonConverter(typeof(EmbeddingInputConverter))]
public abstract record EmbeddingInput
{
    /// <summary>Wraps a single string as an embedding input.</summary>
    public static EmbeddingInput FromString(string text) => new SingleInput(text);

    /// <summary>Wraps multiple strings as an embedding input batch.</summary>
    public static EmbeddingInput FromList(IReadOnlyList<string> texts) => new MultipleInput(texts);

    /// <summary>A single-string embedding input.</summary>
    public sealed record SingleInput(string Value) : EmbeddingInput;

    /// <summary>A multi-string embedding input batch.</summary>
    public sealed record MultipleInput(IReadOnlyList<string> Values) : EmbeddingInput;
}

/// <summary>Custom JSON converter for <see cref="EmbeddingInput"/>.</summary>
internal sealed class EmbeddingInputConverter : JsonConverter<EmbeddingInput>
{
    public override EmbeddingInput Read(ref Utf8JsonReader reader, Type typeToConvert, JsonSerializerOptions options)
    {
        if (reader.TokenType == JsonTokenType.String)
        {
            return EmbeddingInput.FromString(reader.GetString()!);
        }

        var values = JsonSerializer.Deserialize<List<string>>(ref reader, options)!;
        return EmbeddingInput.FromList(values);
    }

    public override void Write(Utf8JsonWriter writer, EmbeddingInput value, JsonSerializerOptions options)
    {
        switch (value)
        {
            case EmbeddingInput.SingleInput single:
                writer.WriteStringValue(single.Value);
                break;
            case EmbeddingInput.MultipleInput multiple:
                JsonSerializer.Serialize(writer, multiple.Values, options);
                break;
            default:
                throw new JsonException($"Unknown EmbeddingInput subtype: {value.GetType()}");
        }
    }
}

/// <summary>Request body for an embedding API call.</summary>
public record EmbeddingRequest(
    [property: JsonPropertyName("model")] string Model,
    [property: JsonPropertyName("input")] EmbeddingInput Input,
    [property: JsonPropertyName("encoding_format"), JsonIgnore(Condition = JsonIgnoreCondition.WhenWritingNull)]
    string? EncodingFormat = null,
    [property: JsonPropertyName("dimensions"), JsonIgnore(Condition = JsonIgnoreCondition.WhenWritingNull)]
    int? Dimensions = null,
    [property: JsonPropertyName("user"), JsonIgnore(Condition = JsonIgnoreCondition.WhenWritingNull)]
    string? User = null);

/// <summary>A single embedding vector returned by the API.</summary>
public record EmbeddingObject(
    /// <summary>Always <c>"embedding"</c> from OpenAI-compatible APIs.</summary>
    [property: JsonPropertyName("object")] string Object,
    [property: JsonPropertyName("embedding")] IReadOnlyList<double> Embedding,
    [property: JsonPropertyName("index")] int Index);

/// <summary>Response body for an embedding request.</summary>
public record EmbeddingResponse(
    /// <summary>Always <c>"list"</c> from OpenAI-compatible APIs.</summary>
    [property: JsonPropertyName("object")] string Object,
    [property: JsonPropertyName("data")] IReadOnlyList<EmbeddingObject> Data,
    [property: JsonPropertyName("model")] string Model,
    [property: JsonPropertyName("usage")] Usage Usage);

// ─── Models ───────────────────────────────────────────────────────────────────

/// <summary>A single model entry returned by the list-models API.</summary>
public record ModelObject(
    [property: JsonPropertyName("id")] string Id,
    /// <summary>Always <c>"model"</c> from OpenAI-compatible APIs.</summary>
    [property: JsonPropertyName("object")] string Object,
    [property: JsonPropertyName("created")] long Created,
    [property: JsonPropertyName("owned_by")] string OwnedBy);

/// <summary>Response body for the list-models API.</summary>
public record ModelsListResponse(
    /// <summary>Always <c>"list"</c> from OpenAI-compatible APIs.</summary>
    [property: JsonPropertyName("object")] string Object,
    [property: JsonPropertyName("data")] IReadOnlyList<ModelObject> Data);

// ─── Image Generation ─────────────────────────────────────────────────────────

/// <summary>Request body for an image generation API call.</summary>
public record CreateImageRequest(
    [property: JsonPropertyName("prompt")] string Prompt,
    [property: JsonPropertyName("model"), JsonIgnore(Condition = JsonIgnoreCondition.WhenWritingNull)]
    string? Model = null,
    [property: JsonPropertyName("n"), JsonIgnore(Condition = JsonIgnoreCondition.WhenWritingNull)]
    int? N = null,
    [property: JsonPropertyName("quality"), JsonIgnore(Condition = JsonIgnoreCondition.WhenWritingNull)]
    string? Quality = null,
    [property: JsonPropertyName("response_format"), JsonIgnore(Condition = JsonIgnoreCondition.WhenWritingNull)]
    string? ResponseFormat = null,
    [property: JsonPropertyName("size"), JsonIgnore(Condition = JsonIgnoreCondition.WhenWritingNull)]
    string? Size = null,
    [property: JsonPropertyName("style"), JsonIgnore(Condition = JsonIgnoreCondition.WhenWritingNull)]
    string? Style = null,
    [property: JsonPropertyName("user"), JsonIgnore(Condition = JsonIgnoreCondition.WhenWritingNull)]
    string? User = null);

/// <summary>A single image result returned by the API.</summary>
public record ImageData(
    [property: JsonPropertyName("url")] string? Url = null,
    [property: JsonPropertyName("b64_json")] string? B64Json = null,
    [property: JsonPropertyName("revised_prompt")] string? RevisedPrompt = null);

/// <summary>Response body for an image generation request.</summary>
public record ImagesResponse(
    [property: JsonPropertyName("created")] long Created,
    [property: JsonPropertyName("data")] IReadOnlyList<ImageData> Data);

// ─── Speech ───────────────────────────────────────────────────────────────────

/// <summary>Request body for a speech generation API call.</summary>
public record CreateSpeechRequest(
    [property: JsonPropertyName("model")] string Model,
    [property: JsonPropertyName("input")] string Input,
    [property: JsonPropertyName("voice")] string Voice,
    [property: JsonPropertyName("response_format"), JsonIgnore(Condition = JsonIgnoreCondition.WhenWritingNull)]
    string? ResponseFormat = null,
    [property: JsonPropertyName("speed"), JsonIgnore(Condition = JsonIgnoreCondition.WhenWritingNull)]
    double? Speed = null);

// ─── Transcription ────────────────────────────────────────────────────────────

/// <summary>Request body for an audio transcription API call.</summary>
public record CreateTranscriptionRequest(
    [property: JsonPropertyName("file")] string File,
    [property: JsonPropertyName("model")] string Model,
    [property: JsonPropertyName("language"), JsonIgnore(Condition = JsonIgnoreCondition.WhenWritingNull)]
    string? Language = null,
    [property: JsonPropertyName("prompt"), JsonIgnore(Condition = JsonIgnoreCondition.WhenWritingNull)]
    string? Prompt = null,
    [property: JsonPropertyName("response_format"), JsonIgnore(Condition = JsonIgnoreCondition.WhenWritingNull)]
    string? ResponseFormat = null,
    [property: JsonPropertyName("temperature"), JsonIgnore(Condition = JsonIgnoreCondition.WhenWritingNull)]
    double? Temperature = null);

/// <summary>Response body for a transcription request.</summary>
public record TranscriptionResponse(
    [property: JsonPropertyName("text")] string Text);

// ─── Moderation ───────────────────────────────────────────────────────────────

/// <summary>Request body for a moderation API call.</summary>
public record ModerationRequest(
    [property: JsonPropertyName("input")] JsonNode Input,
    [property: JsonPropertyName("model"), JsonIgnore(Condition = JsonIgnoreCondition.WhenWritingNull)]
    string? Model = null);

/// <summary>Per-category boolean flags for a moderation result.</summary>
public record ModerationCategories(
    [property: JsonPropertyName("sexual")] bool Sexual,
    [property: JsonPropertyName("hate")] bool Hate,
    [property: JsonPropertyName("harassment")] bool Harassment,
    [property: JsonPropertyName("self-harm")] bool SelfHarm,
    [property: JsonPropertyName("violence")] bool Violence,
    [property: JsonPropertyName("sexual/minors")] bool SexualMinors,
    [property: JsonPropertyName("hate/threatening")] bool HateThreatening,
    [property: JsonPropertyName("violence/graphic")] bool ViolenceGraphic,
    [property: JsonPropertyName("self-harm/intent")] bool SelfHarmIntent,
    [property: JsonPropertyName("self-harm/instructions")] bool SelfHarmInstructions,
    [property: JsonPropertyName("harassment/threatening")] bool HarassmentThreatening);

/// <summary>Per-category confidence scores for a moderation result.</summary>
public record ModerationCategoryScores(
    [property: JsonPropertyName("sexual")] double Sexual,
    [property: JsonPropertyName("hate")] double Hate,
    [property: JsonPropertyName("harassment")] double Harassment,
    [property: JsonPropertyName("self-harm")] double SelfHarm,
    [property: JsonPropertyName("violence")] double Violence,
    [property: JsonPropertyName("sexual/minors")] double SexualMinors,
    [property: JsonPropertyName("hate/threatening")] double HateThreatening,
    [property: JsonPropertyName("violence/graphic")] double ViolenceGraphic,
    [property: JsonPropertyName("self-harm/intent")] double SelfHarmIntent,
    [property: JsonPropertyName("self-harm/instructions")] double SelfHarmInstructions,
    [property: JsonPropertyName("harassment/threatening")] double HarassmentThreatening);

/// <summary>A single moderation result for one input.</summary>
public record ModerationResult(
    [property: JsonPropertyName("flagged")] bool Flagged,
    [property: JsonPropertyName("categories")] ModerationCategories Categories,
    [property: JsonPropertyName("category_scores")] ModerationCategoryScores CategoryScores);

/// <summary>Response body for a moderation request.</summary>
public record ModerationResponse(
    [property: JsonPropertyName("id")] string Id,
    [property: JsonPropertyName("model")] string Model,
    [property: JsonPropertyName("results")] IReadOnlyList<ModerationResult> Results);

// ─── Rerank ───────────────────────────────────────────────────────────────────

/// <summary>Request body for a rerank API call.</summary>
public record RerankRequest(
    [property: JsonPropertyName("model")] string Model,
    [property: JsonPropertyName("query")] string Query,
    [property: JsonPropertyName("documents")] JsonNode Documents,
    [property: JsonPropertyName("top_n"), JsonIgnore(Condition = JsonIgnoreCondition.WhenWritingNull)]
    int? TopN = null);

/// <summary>A single reranked document result.</summary>
public record RerankResult(
    [property: JsonPropertyName("index")] int Index,
    [property: JsonPropertyName("relevance_score")] double RelevanceScore);

/// <summary>Response body for a rerank request.</summary>
public record RerankResponse(
    [property: JsonPropertyName("results")] IReadOnlyList<RerankResult> Results,
    [property: JsonPropertyName("model")] string Model,
    [property: JsonPropertyName("usage")] Usage? Usage = null);

// ─── Files ────────────────────────────────────────────────────────────────────

/// <summary>Request body for a file upload API call.</summary>
public record CreateFileRequest(
    [property: JsonPropertyName("file")] string File,
    [property: JsonPropertyName("purpose")] string Purpose,
    [property: JsonPropertyName("filename"), JsonIgnore(Condition = JsonIgnoreCondition.WhenWritingNull)]
    string? Filename = null);

/// <summary>Metadata for an uploaded file.</summary>
public record FileObject(
    [property: JsonPropertyName("id")] string Id,
    [property: JsonPropertyName("object")] string Object,
    [property: JsonPropertyName("bytes")] long Bytes,
    [property: JsonPropertyName("created_at")] long CreatedAt,
    [property: JsonPropertyName("filename")] string Filename,
    [property: JsonPropertyName("purpose")] string Purpose,
    [property: JsonPropertyName("status")] string? Status = null,
    [property: JsonPropertyName("status_details")] string? StatusDetails = null);

/// <summary>Response body for a delete operation.</summary>
public record DeleteResponse(
    [property: JsonPropertyName("id")] string Id,
    [property: JsonPropertyName("object")] string Object,
    [property: JsonPropertyName("deleted")] bool Deleted);

/// <summary>Optional query parameters for listing files.</summary>
public record FileListQuery(
    string? Purpose = null,
    int? Limit = null,
    string? After = null);

/// <summary>Response body for listing files.</summary>
public record FileListResponse(
    [property: JsonPropertyName("object")] string Object,
    [property: JsonPropertyName("data")] IReadOnlyList<FileObject> Data);

// ─── Batches ──────────────────────────────────────────────────────────────────

/// <summary>Request body for creating a batch job.</summary>
public record CreateBatchRequest(
    [property: JsonPropertyName("input_file_id")] string InputFileId,
    [property: JsonPropertyName("endpoint")] string Endpoint,
    [property: JsonPropertyName("completion_window")] string CompletionWindow,
    [property: JsonPropertyName("metadata"), JsonIgnore(Condition = JsonIgnoreCondition.WhenWritingNull)]
    IReadOnlyDictionary<string, string>? Metadata = null);

/// <summary>Counts for a batch job.</summary>
public record BatchRequestCounts(
    [property: JsonPropertyName("total")] long Total,
    [property: JsonPropertyName("completed")] long Completed,
    [property: JsonPropertyName("failed")] long Failed);

/// <summary>Metadata for a batch processing job.</summary>
public record BatchObject(
    [property: JsonPropertyName("id")] string Id,
    [property: JsonPropertyName("object")] string Object,
    [property: JsonPropertyName("endpoint")] string Endpoint,
    [property: JsonPropertyName("input_file_id")] string InputFileId,
    [property: JsonPropertyName("completion_window")] string CompletionWindow,
    [property: JsonPropertyName("status")] string Status,
    [property: JsonPropertyName("output_file_id")] string? OutputFileId = null,
    [property: JsonPropertyName("error_file_id")] string? ErrorFileId = null,
    [property: JsonPropertyName("created_at")] long CreatedAt = 0,
    [property: JsonPropertyName("in_progress_at")] long? InProgressAt = null,
    [property: JsonPropertyName("expires_at")] long? ExpiresAt = null,
    [property: JsonPropertyName("finalizing_at")] long? FinalizingAt = null,
    [property: JsonPropertyName("completed_at")] long? CompletedAt = null,
    [property: JsonPropertyName("failed_at")] long? FailedAt = null,
    [property: JsonPropertyName("expired_at")] long? ExpiredAt = null,
    [property: JsonPropertyName("cancelling_at")] long? CancellingAt = null,
    [property: JsonPropertyName("cancelled_at")] long? CancelledAt = null,
    [property: JsonPropertyName("request_counts")] BatchRequestCounts? RequestCounts = null,
    [property: JsonPropertyName("metadata")] IReadOnlyDictionary<string, string>? Metadata = null);

/// <summary>Optional query parameters for listing batches.</summary>
public record BatchListQuery(
    int? Limit = null,
    string? After = null);

/// <summary>Response body for listing batches.</summary>
public record BatchListResponse(
    [property: JsonPropertyName("object")] string Object,
    [property: JsonPropertyName("data")] IReadOnlyList<BatchObject> Data);

// ─── Responses API ────────────────────────────────────────────────────────────

/// <summary>Request body for creating a response via the Responses API.</summary>
public record CreateResponseRequest(
    [property: JsonPropertyName("model")] string Model,
    [property: JsonPropertyName("input")] JsonNode Input,
    [property: JsonPropertyName("instructions"), JsonIgnore(Condition = JsonIgnoreCondition.WhenWritingNull)]
    string? Instructions = null,
    [property: JsonPropertyName("max_output_tokens"), JsonIgnore(Condition = JsonIgnoreCondition.WhenWritingNull)]
    long? MaxOutputTokens = null,
    [property: JsonPropertyName("temperature"), JsonIgnore(Condition = JsonIgnoreCondition.WhenWritingNull)]
    double? Temperature = null,
    [property: JsonPropertyName("top_p"), JsonIgnore(Condition = JsonIgnoreCondition.WhenWritingNull)]
    double? TopP = null,
    [property: JsonPropertyName("stream"), JsonIgnore(Condition = JsonIgnoreCondition.WhenWritingNull)]
    bool? Stream = null,
    [property: JsonPropertyName("metadata"), JsonIgnore(Condition = JsonIgnoreCondition.WhenWritingNull)]
    IReadOnlyDictionary<string, string>? Metadata = null);

/// <summary>A response object from the Responses API.</summary>
public record ResponseObject(
    [property: JsonPropertyName("id")] string Id,
    [property: JsonPropertyName("object")] string Object,
    [property: JsonPropertyName("created_at")] long CreatedAt,
    [property: JsonPropertyName("status")] string Status,
    [property: JsonPropertyName("model")] string Model,
    [property: JsonPropertyName("output")] JsonNode? Output = null,
    [property: JsonPropertyName("usage")] Usage? Usage = null,
    [property: JsonPropertyName("metadata")] IReadOnlyDictionary<string, string>? Metadata = null,
    [property: JsonPropertyName("error")] JsonNode? Error = null);
