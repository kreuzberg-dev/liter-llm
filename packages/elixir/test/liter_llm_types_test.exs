defmodule LiterLlm.TypesTest do
  use ExUnit.Case, async: true

  alias LiterLlm.Types

  # ─── Message Constructors ─────────────────────────────────────────────────

  describe "system_message/2" do
    test "creates a system message with content" do
      msg = Types.system_message("You are helpful.")

      assert msg.role == "system"
      assert msg.content == "You are helpful."
      refute Map.has_key?(msg, :name)
    end

    test "includes optional name when provided" do
      msg = Types.system_message("Instructions.", name: "sys")

      assert msg.name == "sys"
    end
  end

  describe "user_message/2" do
    test "creates a user message with string content" do
      msg = Types.user_message("Hello!")

      assert msg.role == "user"
      assert msg.content == "Hello!"
    end

    test "creates a user message with multi-part content" do
      parts = [
        %{type: "text", text: "What's in this image?"},
        %{type: "image_url", image_url: %{url: "https://example.com/img.png"}}
      ]

      msg = Types.user_message(parts)

      assert msg.role == "user"
      assert is_list(msg.content)
      assert length(msg.content) == 2
    end
  end

  describe "assistant_message/1" do
    test "creates an assistant message with content" do
      msg = Types.assistant_message(content: "I can help!")

      assert msg.role == "assistant"
      assert msg.content == "I can help!"
    end

    test "creates an assistant message with tool calls" do
      tool_call = %{
        id: "call-1",
        type: "function",
        function: %{name: "get_weather", arguments: "{\"city\": \"Berlin\"}"}
      }

      msg = Types.assistant_message(tool_calls: [tool_call])

      assert msg.role == "assistant"
      assert length(msg.tool_calls) == 1
      assert hd(msg.tool_calls).function.name == "get_weather"
    end

    test "creates an empty assistant message" do
      msg = Types.assistant_message()

      assert msg.role == "assistant"
      refute Map.has_key?(msg, :content)
    end
  end

  describe "tool_message/3" do
    test "creates a tool result message" do
      msg = Types.tool_message("sunny, 22°C", "call-123")

      assert msg.role == "tool"
      assert msg.content == "sunny, 22°C"
      assert msg.tool_call_id == "call-123"
    end
  end

  describe "developer_message/2" do
    test "creates a developer message" do
      msg = Types.developer_message("Focus on accuracy.")

      assert msg.role == "developer"
      assert msg.content == "Focus on accuracy."
    end
  end

  # ─── Tool Constructor ─────────────────────────────────────────────────────

  describe "tool/4" do
    test "creates a function tool" do
      schema = %{"type" => "object", "properties" => %{"city" => %{"type" => "string"}}}
      tool = Types.tool("get_weather", "Get current weather", schema)

      assert tool.type == "function"
      assert tool.function.name == "get_weather"
      assert tool.function.description == "Get current weather"
      assert tool.function.parameters == schema
    end

    test "creates a minimal tool without description or parameters" do
      tool = Types.tool("noop")

      assert tool.type == "function"
      assert tool.function.name == "noop"
      refute Map.has_key?(tool.function, :description)
      refute Map.has_key?(tool.function, :parameters)
    end
  end

  # ─── Finish Reason Parsing ────────────────────────────────────────────────

  describe "parse_finish_reason/1" do
    test "maps known finish reasons" do
      assert Types.parse_finish_reason("stop") == :stop
      assert Types.parse_finish_reason("length") == :length
      assert Types.parse_finish_reason("tool_calls") == :tool_calls
      assert Types.parse_finish_reason("content_filter") == :content_filter
      assert Types.parse_finish_reason("function_call") == :function_call
    end

    test "maps nil to nil" do
      assert Types.parse_finish_reason(nil) == nil
    end

    test "maps unknown values to :other" do
      assert Types.parse_finish_reason("some_new_value") == :other
    end
  end

  # ─── JSON Round-Trip via Jason ────────────────────────────────────────────

  describe "JSON serialization" do
    test "system message serializes correctly" do
      msg = Types.system_message("Hello.")
      {:ok, json} = Jason.encode(msg)
      {:ok, decoded} = Jason.decode(json)

      assert decoded["role"] == "system"
      assert decoded["content"] == "Hello."
    end

    test "chat request serializes with snake_case keys" do
      request = %{
        model: "gpt-4o-mini",
        messages: [Types.user_message("Hi")],
        max_tokens: 100,
        temperature: 0.7
      }

      {:ok, json} = Jason.encode(request)

      assert json =~ "\"max_tokens\""
      assert json =~ "\"temperature\""
      refute json =~ "\"stream\""
    end

    test "embedding request serializes input as string" do
      request = %{
        model: "text-embedding-3-small",
        input: "The quick brown fox"
      }

      {:ok, json} = Jason.encode(request)

      assert json =~ "text-embedding-3-small"
      assert json =~ "The quick brown fox"
    end

    test "embedding request serializes input as list" do
      request = %{
        model: "text-embedding-3-small",
        input: ["first", "second"]
      }

      {:ok, json} = Jason.encode(request)

      assert json =~ "\"first\""
      assert json =~ "\"second\""
    end

    test "chat completion response round-trips from JSON string" do
      json_str = """
      {
        "id": "chatcmpl-test",
        "object": "chat.completion",
        "created": 1700000000,
        "model": "gpt-4o",
        "choices": [{
          "index": 0,
          "message": {"role": "assistant", "content": "Hello!"},
          "finish_reason": "stop"
        }],
        "usage": {"prompt_tokens": 5, "completion_tokens": 3, "total_tokens": 8}
      }
      """

      {:ok, decoded} = Jason.decode(json_str)

      assert decoded["id"] == "chatcmpl-test"
      assert decoded["object"] == "chat.completion"
      assert decoded["model"] == "gpt-4o"
      assert length(decoded["choices"]) == 1

      assert decoded["choices"] |> Enum.at(0) |> Map.get("message") |> Map.get("role") ==
               "assistant"

      assert decoded["usage"]["total_tokens"] == 8
    end

    test "streaming chunk parses correctly" do
      chunk_json = """
      {
        "id": "chunk-1",
        "object": "chat.completion.chunk",
        "created": 1700000000,
        "model": "gpt-4o",
        "choices": [{
          "index": 0,
          "delta": {"role": "assistant", "content": "streaming text"},
          "finish_reason": null
        }]
      }
      """

      {:ok, decoded} = Jason.decode(chunk_json)

      assert decoded["id"] == "chunk-1"
      assert decoded["object"] == "chat.completion.chunk"
      choice = Enum.at(decoded["choices"], 0)
      assert choice["delta"]["content"] == "streaming text"
      assert is_nil(choice["finish_reason"])
    end

    test "embedding response parses with multiple embeddings" do
      embedding_json = """
      {
        "object": "list",
        "data": [
          {"object": "embedding", "embedding": [0.1, 0.2, 0.3], "index": 0},
          {"object": "embedding", "embedding": [0.4, 0.5, 0.6], "index": 1}
        ],
        "model": "text-embedding-3-small",
        "usage": {"prompt_tokens": 10, "completion_tokens": 0, "total_tokens": 10}
      }
      """

      {:ok, decoded} = Jason.decode(embedding_json)

      assert decoded["object"] == "list"
      assert decoded["model"] == "text-embedding-3-small"
      data = decoded["data"]
      assert length(data) == 2

      first_embedding = Enum.at(data, 0)
      assert first_embedding["index"] == 0
      assert Enum.at(first_embedding["embedding"], 0) == 0.1

      second_embedding = Enum.at(data, 1)
      assert second_embedding["index"] == 1
      assert Enum.at(second_embedding["embedding"], 0) == 0.4
    end

    test "model list response parses correctly" do
      models_json = """
      {
        "object": "list",
        "data": [
          {"id": "gpt-4o", "object": "model", "created": 1712361441, "owned_by": "openai"},
          {"id": "gpt-3.5-turbo", "object": "model", "created": 1690000000, "owned_by": "openai"}
        ]
      }
      """

      {:ok, decoded} = Jason.decode(models_json)

      assert decoded["object"] == "list"
      data = decoded["data"]
      assert length(data) == 2

      first = Enum.at(data, 0)
      assert first["id"] == "gpt-4o"
      assert first["object"] == "model"
      assert first["owned_by"] == "openai"

      second = Enum.at(data, 1)
      assert second["id"] == "gpt-3.5-turbo"
    end

    test "tool message with tool_call_id serializes" do
      tool_msg = %{
        role: "tool",
        content: "tool result",
        tool_call_id: "call-123"
      }

      {:ok, json} = Jason.encode(tool_msg)

      assert json =~ "\"role\":\"tool\""
      assert json =~ "\"tool_call_id\":\"call-123\""
      assert json =~ "tool result"
    end

    test "developer message serializes" do
      dev_msg = %{
        role: "developer",
        content: "Focus on accuracy."
      }

      {:ok, json} = Jason.encode(dev_msg)

      assert json =~ "\"role\":\"developer\""
      assert json =~ "Focus on accuracy."
    end

    test "function message serializes with name" do
      func_msg = %{
        role: "function",
        content: "Function executed",
        name: "my_function"
      }

      {:ok, json} = Jason.encode(func_msg)

      assert json =~ "\"role\":\"function\""
      assert json =~ "\"name\":\"my_function\""
      assert json =~ "Function executed"
    end
  end
end
