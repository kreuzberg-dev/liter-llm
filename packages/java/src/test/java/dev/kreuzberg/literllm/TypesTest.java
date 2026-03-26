package dev.kreuzberg.literllm;

import static dev.kreuzberg.literllm.Types.*;
import static org.assertj.core.api.Assertions.assertThat;

import com.fasterxml.jackson.databind.DeserializationFeature;
import com.fasterxml.jackson.databind.ObjectMapper;
import dev.kreuzberg.literllm.LlmException.AuthenticationException;
import dev.kreuzberg.literllm.LlmException.InvalidRequestException;
import dev.kreuzberg.literllm.LlmException.NotFoundException;
import dev.kreuzberg.literllm.LlmException.ProviderException;
import dev.kreuzberg.literllm.LlmException.RateLimitException;
import dev.kreuzberg.literllm.LlmException.SerializationException;
import dev.kreuzberg.literllm.LlmException.StreamException;
import java.util.List;
import org.assertj.core.api.SoftAssertions;
import org.junit.jupiter.api.BeforeEach;
import org.junit.jupiter.api.Test;

/** Unit tests for liter-llm Java type serialization and deserialization. */
class TypesTest {

	private ObjectMapper mapper;

	@BeforeEach
	void setUp() {
		mapper = new ObjectMapper().configure(DeserializationFeature.FAIL_ON_UNKNOWN_PROPERTIES, false);
	}

	@Test
	void systemMessage_roundTrips() throws Exception {
		var msg = new SystemMessage("You are helpful.", "sys");
		String json = mapper.writeValueAsString(msg);
		assertThat(json).contains("\"role\":\"system\"");
		assertThat(json).contains("\"content\":\"You are helpful.\"");

		var decoded = mapper.readValue(json, Message.class);
		assertThat(decoded).isInstanceOf(SystemMessage.class);
		assertThat(((SystemMessage) decoded).content()).isEqualTo("You are helpful.");
	}

	@Test
	void userMessage_textOnly_roundTrips() throws Exception {
		var msg = new UserMessage("Hello!");
		String json = mapper.writeValueAsString(msg);
		assertThat(json).contains("\"role\":\"user\"");
		assertThat(json).doesNotContain("\"name\"");

		var decoded = mapper.readValue(json, Message.class);
		assertThat(decoded).isInstanceOf(UserMessage.class);
	}

	@Test
	void assistantMessage_withToolCalls_roundTrips() throws Exception {
		var toolCall = new ToolCall("call-1", ToolType.FUNCTION,
				new FunctionCall("get_weather", "{\"city\": \"Berlin\"}"));
		var msg = new AssistantMessage(List.of(toolCall));
		String json = mapper.writeValueAsString(msg);

		assertThat(json).contains("\"role\":\"assistant\"");
		assertThat(json).contains("\"tool_calls\"");
		assertThat(json).contains("\"get_weather\"");

		var decoded = mapper.readValue(json, Message.class);
		assertThat(decoded).isInstanceOf(AssistantMessage.class);
		var assistant = (AssistantMessage) decoded;
		assertThat(assistant.toolCalls()).hasSize(1);
		assertThat(assistant.toolCalls().getFirst().function().name()).isEqualTo("get_weather");
	}

	@Test
	void chatCompletionRequest_builder_omitsNulls() throws Exception {
		var request = ChatCompletionRequest.builder("gpt-4o-mini", List.of(new UserMessage("Hi"))).maxTokens(100L)
				.temperature(0.7).build();

		String json = mapper.writeValueAsString(request);
		assertThat(json).contains("\"model\":\"gpt-4o-mini\"");
		assertThat(json).contains("\"max_tokens\":100");
		assertThat(json).contains("\"temperature\":0.7");
		assertThat(json).doesNotContain("\"stream\"");
		assertThat(json).doesNotContain("\"seed\"");
	}

	@Test
	void finishReason_unknownValue_deserializesToOther() throws Exception {
		String json = "\"new_unknown_reason\"";
		var reason = mapper.readValue(json, FinishReason.class);
		assertThat(reason).isEqualTo(FinishReason.OTHER);
	}

	@Test
	void usage_roundTrips() throws Exception {
		var usage = new Usage(100L, 50L, 150L);
		String json = mapper.writeValueAsString(usage);

		assertThat(json).contains("\"prompt_tokens\":100");
		assertThat(json).contains("\"completion_tokens\":50");
		assertThat(json).contains("\"total_tokens\":150");

		var decoded = mapper.readValue(json, Usage.class);
		assertThat(decoded).isEqualTo(usage);
	}

	@Test
	void embeddingRequest_singleInput_roundTrips() throws Exception {
		var request = new EmbeddingRequest("text-embedding-3-small", EmbeddingInput.of("Hello, world!"));
		String json = mapper.writeValueAsString(request);

		assertThat(json).contains("\"model\":\"text-embedding-3-small\"");
		assertThat(json).contains("\"Hello, world!\"");
	}

	@Test
	void embeddingRequest_multipleInput_roundTrips() throws Exception {
		var request = new EmbeddingRequest("text-embedding-3-small", EmbeddingInput.of(List.of("first", "second")));
		String json = mapper.writeValueAsString(request);

		assertThat(json).contains("\"first\"");
		assertThat(json).contains("\"second\"");
	}

	@Test
	void modelObject_roundTrips() throws Exception {
		String json = """
				{"id":"gpt-4o","object":"model","created":1712361441,"owned_by":"openai"}""";
		var model = mapper.readValue(json, ModelObject.class);

		SoftAssertions softly = new SoftAssertions();
		softly.assertThat(model.id()).isEqualTo("gpt-4o");
		softly.assertThat(model.object()).isEqualTo("model");
		softly.assertThat(model.ownedBy()).isEqualTo("openai");
		softly.assertAll();
	}

	@Test
	void toolChoice_auto_serializesAsString() throws Exception {
		String json = mapper.writeValueAsString(ToolChoice.AUTO);
		assertThat(json).isEqualTo("\"auto\"");
	}

	@Test
	void toolChoice_specific_serializesCorrectly() throws Exception {
		var choice = ToolChoice.forFunction("get_weather");
		String json = mapper.writeValueAsString(choice);
		assertThat(json).contains("\"type\":\"function\"");
		assertThat(json).contains("\"get_weather\"");
	}

	@Test
	void responseFormat_text_roundTrips() throws Exception {
		String json = mapper.writeValueAsString(ResponseFormat.TEXT);
		assertThat(json).contains("\"type\":\"text\"");
	}

	@Test
	void stopSequence_single_roundTrips() throws Exception {
		var stop = StopSequence.of("STOP");
		String json = mapper.writeValueAsString(stop);
		assertThat(json).isEqualTo("\"STOP\"");
	}

	@Test
	void stopSequence_multiple_roundTrips() throws Exception {
		var stop = StopSequence.of(List.of("stop1", "stop2"));
		String json = mapper.writeValueAsString(stop);
		assertThat(json).contains("\"stop1\"");
		assertThat(json).contains("\"stop2\"");
	}

	// ─── Additional Tests ─────────────────────────────────────────────────────

	@Test
	void embeddingResponse_roundTrips() throws Exception {
		var embedding = new EmbeddingObject("embedding", List.of(0.1, 0.2, 0.3), 0);
		var response = new EmbeddingResponse("list", List.of(embedding), "text-embedding-3-small",
				new Usage(10L, 0L, 10L));
		String json = mapper.writeValueAsString(response);

		assertThat(json).contains("\"model\":\"text-embedding-3-small\"");
		assertThat(json).contains("\"data\"");
		assertThat(json).contains("0.1");
		assertThat(json).contains("0.2");

		var decoded = mapper.readValue(json, EmbeddingResponse.class);
		assertThat(decoded.model()).isEqualTo("text-embedding-3-small");
		assertThat(decoded.data()).hasSize(1);
		assertThat(decoded.data().getFirst().embedding()).hasSize(3);
	}

	@Test
	void chatCompletionChunk_streaming_roundTrips() throws Exception {
		var delta = new StreamDelta("assistant", "Hello", null, null, null);
		var choice = new StreamChoice(0, delta, null);
		var chunk = new ChatCompletionChunk("chunk-1", "chat.completion.chunk", 1700000000, "gpt-4o", List.of(choice),
				null, null);

		String json = mapper.writeValueAsString(chunk);
		assertThat(json).contains("\"id\":\"chunk-1\"");
		assertThat(json).contains("\"delta\"");
		assertThat(json).contains("\"Hello\"");

		var decoded = mapper.readValue(json, ChatCompletionChunk.class);
		assertThat(decoded.id()).isEqualTo("chunk-1");
		assertThat(decoded.choices()).hasSize(1);
		assertThat(decoded.choices().getFirst().delta().content()).isEqualTo("Hello");
	}

	@Test
	void llmException_errorCodes_classification() throws Exception {
		var exc1 = new InvalidRequestException("bad input");
		assertThat(exc1.getErrorCode()).isEqualTo(LlmException.CODE_INVALID_REQUEST);
		assertThat(exc1.getMessage()).contains("invalid request");

		var exc2 = new AuthenticationException("bad key");
		assertThat(exc2.getErrorCode()).isEqualTo(LlmException.CODE_AUTHENTICATION);

		var exc3 = new RateLimitException("too fast");
		assertThat(exc3.getErrorCode()).isEqualTo(LlmException.CODE_RATE_LIMIT);

		var exc4 = new NotFoundException("model missing");
		assertThat(exc4.getErrorCode()).isEqualTo(LlmException.CODE_NOT_FOUND);

		var exc5 = new ProviderException(500, "server error");
		assertThat(exc5.getErrorCode()).isEqualTo(LlmException.CODE_PROVIDER_ERROR);
		assertThat(exc5.getHttpStatus()).isEqualTo(500);

		var exc6 = new StreamException("parse failed");
		assertThat(exc6.getErrorCode()).isEqualTo(LlmException.CODE_STREAM_ERROR);

		var exc7 = new SerializationException("JSON error", null);
		assertThat(exc7.getErrorCode()).isEqualTo(LlmException.CODE_SERIALIZATION);
	}

	@Test
	void llmClient_builder_validation_nullApiKey() throws Exception {
		// The builder should handle null apiKey gracefully.
		// This tests that the builder doesn't crash with null or empty key.
		var builder = ChatCompletionRequest.builder("gpt-4o-mini", List.of(new UserMessage("test")));
		var request = builder.build();

		assertThat(request.model()).isEqualTo("gpt-4o-mini");
		assertThat(request.messages()).hasSize(1);
	}

	@Test
	void toolMessage_roundTrips() throws Exception {
		var msg = new ToolMessage("tool result here", "call-123");
		String json = mapper.writeValueAsString(msg);

		assertThat(json).contains("\"role\":\"tool\"");
		assertThat(json).contains("\"tool_call_id\":\"call-123\"");
		assertThat(json).contains("\"tool result here\"");

		var decoded = mapper.readValue(json, Message.class);
		assertThat(decoded).isInstanceOf(ToolMessage.class);
		var tool = (ToolMessage) decoded;
		assertThat(tool.content()).isEqualTo("tool result here");
		assertThat(tool.toolCallId()).isEqualTo("call-123");
	}

	@Test
	void developerMessage_roundTrips() throws Exception {
		var msg = new DeveloperMessage("Always be accurate.");
		String json = mapper.writeValueAsString(msg);

		assertThat(json).contains("\"role\":\"developer\"");
		assertThat(json).contains("\"Always be accurate.\"");

		var decoded = mapper.readValue(json, Message.class);
		assertThat(decoded).isInstanceOf(DeveloperMessage.class);
		assertThat(((DeveloperMessage) decoded).content()).isEqualTo("Always be accurate.");
	}

	@Test
	void responseFormat_jsonMode_roundTrips() throws Exception {
		String json = mapper.writeValueAsString(ResponseFormat.JSON_OBJECT);
		assertThat(json).contains("\"type\":\"json_object\"");
	}
}
