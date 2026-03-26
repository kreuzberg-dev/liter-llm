using System.Text.Json;
using FluentAssertions;
using Xunit;

namespace LiterLlm.Tests;

/// <summary>Unit tests for liter-llm C# type serialization and deserialization.</summary>
public class TypesTests
{
    // ─── Messages ─────────────────────────────────────────────────────────────

    [Fact]
    public void SystemMessage_RoundTrips()
    {
        Message msg = new SystemMessage("You are helpful.");
        var json = LiterLlmJson.Serialize(msg);

        json.Should().Contain("\"role\":\"system\"");
        json.Should().Contain("You are helpful.");
        json.Should().NotContain("\"name\"");

        var decoded = LiterLlmJson.Deserialize<Message>(json);
        decoded.Should().BeOfType<SystemMessage>()
            .Which.Content.Should().Be("You are helpful.");
    }

    [Fact]
    public void UserMessage_TextContent_RoundTrips()
    {
        Message msg = new UserMessage("Hello!");
        var json = LiterLlmJson.Serialize(msg);

        json.Should().Contain("\"role\":\"user\"");
        json.Should().Contain("Hello!");
    }

    [Fact]
    public void AssistantMessage_WithToolCalls_RoundTrips()
    {
        var toolCall = new ToolCall("call-1", ToolType.Function,
            new FunctionCall("get_weather", "{\"city\": \"Berlin\"}"));
        Message msg = new AssistantMessage(ToolCalls: [toolCall]);
        var json = LiterLlmJson.Serialize(msg);

        json.Should().Contain("\"role\":\"assistant\"");
        json.Should().Contain("tool_calls");
        json.Should().Contain("get_weather");
    }

    // ─── Chat Request ─────────────────────────────────────────────────────────

    [Fact]
    public void ChatCompletionRequest_OmitsNullFields()
    {
        var request = new ChatCompletionRequest(
            Model: "gpt-4o-mini",
            Messages: [new UserMessage("Hi")],
            MaxTokens: 100,
            Temperature: 0.7);

        var json = LiterLlmJson.Serialize(request);

        json.Should().Contain("\"model\":\"gpt-4o-mini\"");
        json.Should().Contain("\"max_tokens\":100");
        json.Should().Contain("\"temperature\":0.7");
        json.Should().NotContain("\"stream\"");
        json.Should().NotContain("\"seed\"");
    }

    // ─── Finish Reason ────────────────────────────────────────────────────────

    [Theory]
    [InlineData("stop", FinishReason.Stop)]
    [InlineData("length", FinishReason.Length)]
    [InlineData("tool_calls", FinishReason.ToolCalls)]
    [InlineData("content_filter", FinishReason.ContentFilter)]
    [InlineData("function_call", FinishReason.FunctionCall)]
    [InlineData("some_new_value", FinishReason.Other)]
    public void FinishReason_Deserializes(string wire, FinishReason expected)
    {
        var json = $"\"{wire}\"";
        var decoded = JsonSerializer.Deserialize<FinishReason>(json, LiterLlmJson.SerializerOptions);
        decoded.Should().Be(expected);
    }

    // ─── Usage ────────────────────────────────────────────────────────────────

    [Fact]
    public void Usage_RoundTrips()
    {
        var usage = new Usage(100L, 50L, 150L);
        var json = LiterLlmJson.Serialize(usage);

        json.Should().Contain("\"prompt_tokens\":100");
        json.Should().Contain("\"completion_tokens\":50");
        json.Should().Contain("\"total_tokens\":150");

        var decoded = LiterLlmJson.Deserialize<Usage>(json);
        decoded.Should().BeEquivalentTo(usage);
    }

    // ─── Tool Choice ──────────────────────────────────────────────────────────

    [Fact]
    public void ToolChoice_Auto_SerializesAsString()
    {
        var json = LiterLlmJson.Serialize(ToolChoice.Auto);
        json.Should().Be("\"auto\"");
    }

    [Fact]
    public void ToolChoice_Required_SerializesAsString()
    {
        var json = LiterLlmJson.Serialize(ToolChoice.Required);
        json.Should().Be("\"required\"");
    }

    [Fact]
    public void ToolChoice_ForFunction_SerializesCorrectly()
    {
        var choice = ToolChoice.ForFunction("get_weather");
        var json = LiterLlmJson.Serialize(choice);

        json.Should().Contain("\"type\":\"function\"");
        json.Should().Contain("\"get_weather\"");
    }

    // ─── Response Format ──────────────────────────────────────────────────────

    [Fact]
    public void ResponseFormat_Text_RoundTrips()
    {
        var json = LiterLlmJson.Serialize(ResponseFormat.Text);
        json.Should().Contain("\"type\":\"text\"");
    }

    [Fact]
    public void ResponseFormat_JsonObject_RoundTrips()
    {
        var json = LiterLlmJson.Serialize(ResponseFormat.JsonObject);
        json.Should().Contain("\"type\":\"json_object\"");
    }

    // ─── Stop Sequence ────────────────────────────────────────────────────────

    [Fact]
    public void StopSequence_Single_SerializesAsString()
    {
        var stop = StopSequence.FromString("\n\n");
        var json = LiterLlmJson.Serialize(stop);
        json.Should().Be("\"\\n\\n\"");
    }

    [Fact]
    public void StopSequence_Multiple_SerializesAsArray()
    {
        var stop = StopSequence.FromList(["stop1", "stop2"]);
        var json = LiterLlmJson.Serialize(stop);

        json.Should().Contain("stop1");
        json.Should().Contain("stop2");
        json.Should().StartWith("[");
    }

    // ─── Embedding ────────────────────────────────────────────────────────────

    [Fact]
    public void EmbeddingRequest_SingleInput_RoundTrips()
    {
        var request = new EmbeddingRequest(
            Model: "text-embedding-3-small",
            Input: EmbeddingInput.FromString("Hello, world!"));
        var json = LiterLlmJson.Serialize(request);

        json.Should().Contain("\"model\":\"text-embedding-3-small\"");
        json.Should().Contain("Hello, world!");
    }

    [Fact]
    public void EmbeddingRequest_MultipleInput_RoundTrips()
    {
        var request = new EmbeddingRequest(
            Model: "text-embedding-3-small",
            Input: EmbeddingInput.FromList(["first", "second"]));
        var json = LiterLlmJson.Serialize(request);

        json.Should().Contain("first");
        json.Should().Contain("second");
    }

    // ─── Models ───────────────────────────────────────────────────────────────

    [Fact]
    public void ModelObject_Deserializes()
    {
        const string json =
            """{"id":"gpt-4o","object":"model","created":1712361441,"owned_by":"openai"}""";
        var model = LiterLlmJson.Deserialize<ModelObject>(json);

        model.Should().NotBeNull();
        model!.Id.Should().Be("gpt-4o");
        model.Object.Should().Be("model");
        model.OwnedBy.Should().Be("openai");
    }

    // ─── Errors ───────────────────────────────────────────────────────────────

    [Fact]
    public void InvalidRequestException_HasCorrectErrorCode()
    {
        var ex = new InvalidRequestException("bad field");
        ex.ErrorCode.Should().Be(LlmException.ErrorCodes.InvalidRequest);
        ex.Message.Should().Contain("invalid request");
        ex.Message.Should().Contain("bad field");
    }

    [Fact]
    public void AuthenticationException_HasCorrectErrorCode()
    {
        var ex = new AuthenticationException("invalid key");
        ex.ErrorCode.Should().Be(LlmException.ErrorCodes.Authentication);
    }

    [Fact]
    public void ProviderException_ExposesHttpStatus()
    {
        var ex = new ProviderException(503, "Service Unavailable");
        ex.HttpStatus.Should().Be(503);
        ex.ErrorCode.Should().Be(LlmException.ErrorCodes.ProviderError);
    }

    // ─── Additional Tests ─────────────────────────────────────────────────────

    [Fact]
    public void EmbeddingResponse_Serialization()
    {
        var embedding = new EmbeddingObject(Object: "embedding", Embedding: [0.1, 0.2, 0.3], Index: 0);
        var response = new EmbeddingResponse(
            Object: "list",
            Data: [embedding],
            Model: "text-embedding-3-small",
            Usage: new Usage(10L, 0L, 10L));
        var json = LiterLlmJson.Serialize(response);

        json.Should().Contain("\"model\":\"text-embedding-3-small\"");
        json.Should().Contain("\"data\"");
        json.Should().Contain("0.1");
        json.Should().Contain("0.2");

        var decoded = LiterLlmJson.Deserialize<EmbeddingResponse>(json);
        decoded.Should().NotBeNull();
        decoded!.Model.Should().Be("text-embedding-3-small");
        decoded.Data.Should().HaveCount(1);
        decoded.Data[0].Embedding.Should().HaveCount(3);
    }

    [Fact]
    public void ChatCompletionChunk_Serialization()
    {
        var delta = new StreamDelta(Role: "assistant", Content: "Hello");
        var choice = new StreamChoice(Index: 0, Delta: delta, FinishReason: null);
        var chunk = new ChatCompletionChunk(
            Id: "chunk-1",
            Object: "chat.completion.chunk",
            Created: 1700000000,
            Model: "gpt-4o",
            Choices: [choice]);

        var json = LiterLlmJson.Serialize(chunk);

        json.Should().Contain("\"id\":\"chunk-1\"");
        json.Should().Contain("\"delta\"");
        json.Should().Contain("\"Hello\"");

        var decoded = LiterLlmJson.Deserialize<ChatCompletionChunk>(json);
        decoded.Should().NotBeNull();
        decoded!.Id.Should().Be("chunk-1");
        decoded.Choices.Should().HaveCount(1);
        decoded.Choices[0].Delta.Content.Should().Be("Hello");
    }

    [Theory]
    [InlineData(LlmException.ErrorCodes.Unknown)]
    [InlineData(LlmException.ErrorCodes.InvalidRequest)]
    [InlineData(LlmException.ErrorCodes.Authentication)]
    [InlineData(LlmException.ErrorCodes.RateLimit)]
    [InlineData(LlmException.ErrorCodes.NotFound)]
    [InlineData(LlmException.ErrorCodes.ProviderError)]
    [InlineData(LlmException.ErrorCodes.StreamError)]
    [InlineData(LlmException.ErrorCodes.Serialization)]
    public void ErrorCodes_AllValuesPresent(int errorCode)
    {
        errorCode.Should().BeGreaterThanOrEqualTo(1000);
    }

    [Fact]
    public void ToolMessage_Serialization()
    {
        Message msg = new ToolMessage(Content: "Tool result here", ToolCallId: "call-123");
        var json = LiterLlmJson.Serialize(msg);

        json.Should().Contain("\"role\":\"tool\"");
        json.Should().Contain("\"tool_call_id\":\"call-123\"");
        json.Should().Contain("Tool result here");

        var decoded = LiterLlmJson.Deserialize<Message>(json);
        decoded.Should().BeOfType<ToolMessage>()
            .Which.Content.Should().Be("Tool result here");
    }

    [Fact]
    public void DeveloperMessage_Serialization()
    {
        Message msg = new DeveloperMessage("Always be accurate.");
        var json = LiterLlmJson.Serialize(msg);

        json.Should().Contain("\"role\":\"developer\"");
        json.Should().Contain("Always be accurate.");

        var decoded = LiterLlmJson.Deserialize<Message>(json);
        decoded.Should().BeOfType<DeveloperMessage>()
            .Which.Content.Should().Be("Always be accurate.");
    }

    [Fact]
    public void ResponseFormat_JsonObject_Serialization()
    {
        var json = LiterLlmJson.Serialize(ResponseFormat.JsonObject);

        json.Should().Contain("\"type\":\"json_object\"");
    }

    [Fact]
    public void FunctionMessage_Serialization()
    {
        Message msg = new FunctionMessage(Content: "Function result", Name: "my_function");
        var json = LiterLlmJson.Serialize(msg);

        json.Should().Contain("\"role\":\"function\"");
        json.Should().Contain("\"name\":\"my_function\"");
        json.Should().Contain("Function result");
    }

    [Fact]
    public void ProviderException_AllHttpStatuses()
    {
        var ex502 = new ProviderException(502, "Bad Gateway");
        ex502.HttpStatus.Should().Be(502);

        var ex503 = new ProviderException(503, "Unavailable");
        ex503.HttpStatus.Should().Be(503);

        var ex504 = new ProviderException(504, "Timeout");
        ex504.HttpStatus.Should().Be(504);
    }
}
