package literllm_test

import (
	"context"
	"encoding/json"
	"errors"
	"net/http"
	"net/http/httptest"
	"strings"
	"testing"

	literllm "github.com/kreuzberg-dev/liter-llm/go"
)

// ─── Helpers ──────────────────────────────────────────────────────────────────

// newTestClient creates a Client pointed at the given test server URL with a
// dummy API key.
func newTestClient(serverURL string) *literllm.Client {
	return literllm.NewClient(
		literllm.WithAPIKey("test-key"),
		literllm.WithBaseURL(serverURL),
	)
}

// ptr is a generic helper that returns a pointer to v.

// ─── Types ────────────────────────────────────────────────────────────────────

func TestNewTextMessage(t *testing.T) {
	t.Parallel()
	msg := literllm.NewTextMessage(literllm.RoleUser, "hello")
	if msg.Role != literllm.RoleUser {
		t.Errorf("expected role %q, got %q", literllm.RoleUser, msg.Role)
	}
	var content string
	if err := json.Unmarshal(msg.Content, &content); err != nil {
		t.Fatalf("content should be a JSON string: %v", err)
	}
	if content != "hello" {
		t.Errorf("expected content %q, got %q", "hello", content)
	}
}

func TestNewPartsMessage(t *testing.T) {
	t.Parallel()
	parts := []literllm.ContentPart{
		{Type: literllm.ContentPartTypeText, Text: "describe this"},
		{Type: literllm.ContentPartTypeImageURL, ImageURL: &literllm.ImageURL{URL: "https://example.com/img.png"}},
	}
	msg := literllm.NewPartsMessage(literllm.RoleUser, parts)
	var decoded []literllm.ContentPart
	if err := json.Unmarshal(msg.Content, &decoded); err != nil {
		t.Fatalf("content should be a JSON array: %v", err)
	}
	if len(decoded) != 2 {
		t.Errorf("expected 2 parts, got %d", len(decoded))
	}
}

func TestToolChoiceMarshal(t *testing.T) {
	t.Parallel()
	tests := []struct {
		name  string
		input literllm.ToolChoice
		want  string
	}{
		{"auto", literllm.ToolChoiceAuto, `"auto"`},
		{"required", literllm.ToolChoiceRequired, `"required"`},
		{"none", literllm.ToolChoiceNone, `"none"`},
	}
	for _, tc := range tests {
		t.Run(tc.name, func(t *testing.T) {
			t.Parallel()
			got, err := json.Marshal(tc.input)
			if err != nil {
				t.Fatalf("marshal error: %v", err)
			}
			if string(got) != tc.want {
				t.Errorf("expected %s, got %s", tc.want, got)
			}
		})
	}
}

func TestNewSpecificToolChoice(t *testing.T) {
	t.Parallel()
	tc := literllm.NewSpecificToolChoice("get_weather")
	raw, _ := json.Marshal(tc)
	var obj struct {
		Type     string `json:"type"`
		Function struct {
			Name string `json:"name"`
		} `json:"function"`
	}
	if err := json.Unmarshal(raw, &obj); err != nil {
		t.Fatalf("unmarshal error: %v", err)
	}
	if obj.Type != "function" {
		t.Errorf("expected type %q, got %q", "function", obj.Type)
	}
	if obj.Function.Name != "get_weather" {
		t.Errorf("expected function name %q, got %q", "get_weather", obj.Function.Name)
	}
}

func TestStopSequenceRoundtrip(t *testing.T) {
	t.Parallel()
	single := literllm.NewStopString("STOP")
	b, _ := json.Marshal(single)
	if string(b) != `"STOP"` {
		t.Errorf("single stop: expected %q, got %s", `"STOP"`, b)
	}

	multi := literllm.NewStopStrings([]string{"END", "DONE"})
	b, _ = json.Marshal(multi)
	if string(b) != `["END","DONE"]` {
		t.Errorf("multi stop: expected %q, got %s", `["END","DONE"]`, b)
	}
}

func TestEmbeddingInputRoundtrip(t *testing.T) {
	t.Parallel()
	single := literllm.NewEmbeddingInputSingle("hello world")
	b, _ := json.Marshal(single)
	if string(b) != `"hello world"` {
		t.Errorf("single input: expected %q, got %s", `"hello world"`, b)
	}

	multi := literllm.NewEmbeddingInputMultiple([]string{"a", "b"})
	b, _ = json.Marshal(multi)
	if string(b) != `["a","b"]` {
		t.Errorf("multi input: expected %q, got %s", `["a","b"]`, b)
	}
}

// ─── Errors ───────────────────────────────────────────────────────────────────

func TestAPIErrorIs(t *testing.T) {
	t.Parallel()
	tests := []struct {
		statusCode int
		sentinel   error
	}{
		{http.StatusUnauthorized, literllm.ErrAuthentication},
		{http.StatusForbidden, literllm.ErrAuthentication},
		{http.StatusTooManyRequests, literllm.ErrRateLimit},
		{http.StatusNotFound, literllm.ErrNotFound},
		{http.StatusBadRequest, literllm.ErrInvalidRequest},
		{http.StatusInternalServerError, literllm.ErrProviderError},
	}
	for _, tc := range tests {
		server := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
			w.WriteHeader(tc.statusCode)
		}))
		client := newTestClient(server.URL)
		req := &literllm.ChatCompletionRequest{
			Model:    "gpt-4o",
			Messages: []literllm.Message{literllm.NewTextMessage(literllm.RoleUser, "hi")},
		}
		_, err := client.Chat(context.Background(), req)
		server.Close()
		if err == nil {
			t.Errorf("status %d: expected error, got nil", tc.statusCode)
			continue
		}
		if !errors.Is(err, tc.sentinel) {
			t.Errorf("status %d: expected errors.Is(%v), got %T: %v", tc.statusCode, tc.sentinel, err, err)
		}
	}
}

func TestStreamErrorIs(t *testing.T) {
	t.Parallel()
	server := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		w.Header().Set("Content-Type", "text/event-stream")
		// Send malformed JSON chunk.
		w.Write([]byte("data: {not-valid-json}\n\n")) //nolint:errcheck
	}))
	defer server.Close()

	client := newTestClient(server.URL)
	req := &literllm.ChatCompletionRequest{
		Model:    "gpt-4o",
		Messages: []literllm.Message{literllm.NewTextMessage(literllm.RoleUser, "hi")},
	}
	err := client.ChatStream(context.Background(), req, func(c *literllm.ChatCompletionChunk) error { return nil })
	if err == nil {
		t.Fatal("expected error for malformed chunk, got nil")
	}
	if !errors.Is(err, literllm.ErrStream) {
		t.Errorf("expected errors.Is(ErrStream), got %T: %v", err, err)
	}
}

// ─── Client — validation ──────────────────────────────────────────────────────

func TestChat_NilRequest(t *testing.T) {
	t.Parallel()
	client := literllm.NewClient(literllm.WithAPIKey("k"))
	_, err := client.Chat(context.Background(), nil)
	if !errors.Is(err, literllm.ErrInvalidRequest) {
		t.Errorf("expected ErrInvalidRequest, got %v", err)
	}
}

func TestChat_EmptyModel(t *testing.T) {
	t.Parallel()
	client := literllm.NewClient(literllm.WithAPIKey("k"))
	_, err := client.Chat(context.Background(), &literllm.ChatCompletionRequest{
		Messages: []literllm.Message{literllm.NewTextMessage(literllm.RoleUser, "hi")},
	})
	if !errors.Is(err, literllm.ErrInvalidRequest) {
		t.Errorf("expected ErrInvalidRequest for empty model, got %v", err)
	}
}

func TestChat_EmptyMessages(t *testing.T) {
	t.Parallel()
	client := literllm.NewClient(literllm.WithAPIKey("k"))
	_, err := client.Chat(context.Background(), &literllm.ChatCompletionRequest{
		Model: "gpt-4o",
	})
	if !errors.Is(err, literllm.ErrInvalidRequest) {
		t.Errorf("expected ErrInvalidRequest for empty messages, got %v", err)
	}
}

func TestEmbed_NilRequest(t *testing.T) {
	t.Parallel()
	client := literllm.NewClient(literllm.WithAPIKey("k"))
	_, err := client.Embed(context.Background(), nil)
	if !errors.Is(err, literllm.ErrInvalidRequest) {
		t.Errorf("expected ErrInvalidRequest, got %v", err)
	}
}

func TestEmbed_EmptyModel(t *testing.T) {
	t.Parallel()
	client := literllm.NewClient(literllm.WithAPIKey("k"))
	_, err := client.Embed(context.Background(), &literllm.EmbeddingRequest{
		Input: literllm.NewEmbeddingInputSingle("text"),
	})
	if !errors.Is(err, literllm.ErrInvalidRequest) {
		t.Errorf("expected ErrInvalidRequest for empty model, got %v", err)
	}
}

func TestChatStream_NilHandler(t *testing.T) {
	t.Parallel()
	client := literllm.NewClient(literllm.WithAPIKey("k"))
	err := client.ChatStream(context.Background(), &literllm.ChatCompletionRequest{
		Model:    "gpt-4o",
		Messages: []literllm.Message{literllm.NewTextMessage(literllm.RoleUser, "hi")},
	}, nil)
	if !errors.Is(err, literllm.ErrInvalidRequest) {
		t.Errorf("expected ErrInvalidRequest for nil handler, got %v", err)
	}
}

// ─── Client — happy path (mock HTTP server) ───────────────────────────────────

func TestChat_Success(t *testing.T) {
	t.Parallel()
	respBody := `{
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
	}`
	server := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		if r.Method != http.MethodPost {
			t.Errorf("expected POST, got %s", r.Method)
		}
		if !strings.HasSuffix(r.URL.Path, "/chat/completions") {
			t.Errorf("unexpected path: %s", r.URL.Path)
		}
		if r.Header.Get("Authorization") != "Bearer test-key" {
			t.Errorf("missing or wrong Authorization header: %q", r.Header.Get("Authorization"))
		}
		w.Header().Set("Content-Type", "application/json")
		w.Write([]byte(respBody)) //nolint:errcheck
	}))
	defer server.Close()

	client := newTestClient(server.URL)
	resp, err := client.Chat(context.Background(), &literllm.ChatCompletionRequest{
		Model:    "gpt-4o",
		Messages: []literllm.Message{literllm.NewTextMessage(literllm.RoleUser, "hi")},
	})
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if resp.ID != "chatcmpl-test" {
		t.Errorf("expected ID %q, got %q", "chatcmpl-test", resp.ID)
	}
	if len(resp.Choices) != 1 {
		t.Fatalf("expected 1 choice, got %d", len(resp.Choices))
	}
	if resp.Usage == nil {
		t.Fatal("expected non-nil usage")
	}
	if resp.Usage.TotalTokens != 8 {
		t.Errorf("expected total_tokens 8, got %d", resp.Usage.TotalTokens)
	}
}

func TestChatStream_Success(t *testing.T) {
	t.Parallel()
	sseBody := "data: {\"id\":\"chunk-1\",\"object\":\"chat.completion.chunk\",\"created\":1700000000,\"model\":\"gpt-4o\",\"choices\":[{\"index\":0,\"delta\":{\"role\":\"assistant\",\"content\":\"Hello\"},\"finish_reason\":null}]}\n\n" +
		"data: {\"id\":\"chunk-2\",\"object\":\"chat.completion.chunk\",\"created\":1700000000,\"model\":\"gpt-4o\",\"choices\":[{\"index\":0,\"delta\":{\"content\":\"!\"},\"finish_reason\":\"stop\"}]}\n\n" +
		"data: [DONE]\n\n"

	server := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		w.Header().Set("Content-Type", "text/event-stream")
		w.Write([]byte(sseBody)) //nolint:errcheck
	}))
	defer server.Close()

	client := newTestClient(server.URL)
	var chunks []*literllm.ChatCompletionChunk
	err := client.ChatStream(context.Background(), &literllm.ChatCompletionRequest{
		Model:    "gpt-4o",
		Messages: []literllm.Message{literllm.NewTextMessage(literllm.RoleUser, "hi")},
	}, func(c *literllm.ChatCompletionChunk) error {
		chunks = append(chunks, c)
		return nil
	})
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if len(chunks) != 2 {
		t.Fatalf("expected 2 chunks, got %d", len(chunks))
	}
	if chunks[0].ID != "chunk-1" {
		t.Errorf("expected chunk ID %q, got %q", "chunk-1", chunks[0].ID)
	}
	if chunks[0].Choices[0].Delta.Content == nil || *chunks[0].Choices[0].Delta.Content != "Hello" {
		t.Errorf("expected delta content %q", "Hello")
	}
}

func TestChatStream_HandlerError(t *testing.T) {
	t.Parallel()
	sseBody := "data: {\"id\":\"c1\",\"object\":\"chat.completion.chunk\",\"created\":0,\"model\":\"gpt-4o\",\"choices\":[{\"index\":0,\"delta\":{\"content\":\"hi\"},\"finish_reason\":null}]}\n\n" +
		"data: [DONE]\n\n"

	server := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		w.Header().Set("Content-Type", "text/event-stream")
		w.Write([]byte(sseBody)) //nolint:errcheck
	}))
	defer server.Close()

	sentinel := errors.New("handler abort")
	client := newTestClient(server.URL)
	err := client.ChatStream(context.Background(), &literllm.ChatCompletionRequest{
		Model:    "gpt-4o",
		Messages: []literllm.Message{literllm.NewTextMessage(literllm.RoleUser, "hi")},
	}, func(c *literllm.ChatCompletionChunk) error {
		return sentinel
	})
	if !errors.Is(err, sentinel) {
		t.Errorf("expected sentinel error, got %v", err)
	}
}

func TestEmbed_Success(t *testing.T) {
	t.Parallel()
	respBody := `{
		"object": "list",
		"data": [{"object": "embedding", "embedding": [0.1, 0.2, 0.3], "index": 0}],
		"model": "text-embedding-3-small",
		"usage": {"prompt_tokens": 3, "completion_tokens": 0, "total_tokens": 3}
	}`
	server := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		if !strings.HasSuffix(r.URL.Path, "/embeddings") {
			t.Errorf("unexpected path: %s", r.URL.Path)
		}
		w.Header().Set("Content-Type", "application/json")
		w.Write([]byte(respBody)) //nolint:errcheck
	}))
	defer server.Close()

	client := newTestClient(server.URL)
	resp, err := client.Embed(context.Background(), &literllm.EmbeddingRequest{
		Model: "text-embedding-3-small",
		Input: literllm.NewEmbeddingInputSingle("hello"),
	})
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if len(resp.Data) != 1 {
		t.Fatalf("expected 1 embedding, got %d", len(resp.Data))
	}
	if len(resp.Data[0].Embedding) != 3 {
		t.Errorf("expected embedding length 3, got %d", len(resp.Data[0].Embedding))
	}
}

func TestListModels_Success(t *testing.T) {
	t.Parallel()
	respBody := `{
		"object": "list",
		"data": [
			{"id": "gpt-4o", "object": "model", "created": 1700000000, "owned_by": "openai"},
			{"id": "gpt-3.5-turbo", "object": "model", "created": 1690000000, "owned_by": "openai"}
		]
	}`
	server := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		if r.Method != http.MethodGet {
			t.Errorf("expected GET, got %s", r.Method)
		}
		if !strings.HasSuffix(r.URL.Path, "/models") {
			t.Errorf("unexpected path: %s", r.URL.Path)
		}
		w.Header().Set("Content-Type", "application/json")
		w.Write([]byte(respBody)) //nolint:errcheck
	}))
	defer server.Close()

	client := newTestClient(server.URL)
	resp, err := client.ListModels(context.Background())
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if len(resp.Data) != 2 {
		t.Fatalf("expected 2 models, got %d", len(resp.Data))
	}
	if resp.Data[0].ID != "gpt-4o" {
		t.Errorf("expected model ID %q, got %q", "gpt-4o", resp.Data[0].ID)
	}
}

func TestListModels_ProviderError(t *testing.T) {
	t.Parallel()
	server := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		w.WriteHeader(http.StatusInternalServerError)
		w.Write([]byte(`{"error":{"message":"internal server error"}}`)) //nolint:errcheck
	}))
	defer server.Close()

	client := newTestClient(server.URL)
	_, err := client.ListModels(context.Background())
	if err == nil {
		t.Fatal("expected error, got nil")
	}
	if !errors.Is(err, literllm.ErrProviderError) {
		t.Errorf("expected ErrProviderError, got %T: %v", err, err)
	}
	var apiErr *literllm.APIError
	switch {
	case !errors.As(err, &apiErr):
		t.Errorf("expected *APIError, got %T", err)
	case apiErr.StatusCode != http.StatusInternalServerError:
		t.Errorf("expected status 500, got %d", apiErr.StatusCode)
	case apiErr.Message != "internal server error":
		t.Errorf("expected message %q, got %q", "internal server error", apiErr.Message)
	}
}

func TestNewClient_DefaultBaseURL(t *testing.T) {
	t.Parallel()
	// Constructing without a base URL should not panic.
	client := literllm.NewClient(literllm.WithAPIKey("k"))
	if client == nil {
		t.Fatal("expected non-nil client")
	}
}

func TestWithBaseURL_TrailingSlashStripped(t *testing.T) {
	t.Parallel()
	// A trailing slash on the base URL must not produce double-slash paths.
	// We verify this by pointing the client at a server and checking the
	// request path.
	var gotPath string
	server := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		gotPath = r.URL.Path
		w.Header().Set("Content-Type", "application/json")
		w.Write([]byte(`{"object":"list","data":[]}`)) //nolint:errcheck
	}))
	defer server.Close()

	client := literllm.NewClient(
		literllm.WithAPIKey("k"),
		literllm.WithBaseURL(server.URL+"/"),
	)
	client.ListModels(context.Background()) //nolint:errcheck

	if strings.Contains(gotPath, "//") {
		t.Errorf("path contains double slash: %q", gotPath)
	}
}

// ─── Additional Tests ─────────────────────────────────────────────────────

func TestChat_WithToolCalling(t *testing.T) {
	t.Parallel()
	respBody := `{
		"id": "chatcmpl-with-tools",
		"object": "chat.completion",
		"created": 1700000000,
		"model": "gpt-4o",
		"choices": [{
			"index": 0,
			"message": {
				"role": "assistant",
				"tool_calls": [
					{
						"id": "call-1",
						"type": "function",
						"function": {"name": "get_weather", "arguments": "{\"city\": \"Berlin\"}"}
					}
				]
			},
			"finish_reason": "tool_calls"
		}],
		"usage": {"prompt_tokens": 100, "completion_tokens": 50, "total_tokens": 150}
	}`
	server := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		// Verify request contains tools and tool_choice
		var reqBody struct {
			Tools      []interface{} `json:"tools"`
			ToolChoice interface{}   `json:"tool_choice"`
		}
		if err := json.NewDecoder(r.Body).Decode(&reqBody); err != nil {
			t.Errorf("failed to decode request: %v", err)
		}
		if len(reqBody.Tools) == 0 {
			t.Error("expected tools in request, got none")
		}
		if reqBody.ToolChoice == nil {
			t.Error("expected tool_choice in request, got none")
		}

		w.Header().Set("Content-Type", "application/json")
		w.Write([]byte(respBody)) //nolint:errcheck
	}))
	defer server.Close()

	client := newTestClient(server.URL)
	toolChoice := literllm.NewSpecificToolChoice("get_weather")
	tool := literllm.ChatCompletionTool{
		Type: literllm.ToolTypeFunction,
		Function: literllm.FunctionDefinition{
			Name: "get_weather",
		},
	}

	resp, err := client.Chat(context.Background(), &literllm.ChatCompletionRequest{
		Model:      "gpt-4o",
		Messages:   []literllm.Message{literllm.NewTextMessage(literllm.RoleUser, "get weather")},
		Tools:      []literllm.ChatCompletionTool{tool},
		ToolChoice: &toolChoice,
	})
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if *resp.Choices[0].FinishReason != literllm.FinishReasonToolCalls {
		t.Errorf("expected finish_reason tool_calls, got %v", resp.Choices[0].FinishReason)
	}
}

func TestChatStream_ChunkOrder(t *testing.T) {
	t.Parallel()
	sseBody := "data: {\"id\":\"c1\",\"object\":\"chat.completion.chunk\",\"created\":0,\"model\":\"gpt-4o\",\"choices\":[{\"index\":0,\"delta\":{\"content\":\"Hello\"},\"finish_reason\":null}]}\n\n" +
		"data: {\"id\":\"c2\",\"object\":\"chat.completion.chunk\",\"created\":0,\"model\":\"gpt-4o\",\"choices\":[{\"index\":0,\"delta\":{\"content\":\" \"},\"finish_reason\":null}]}\n\n" +
		"data: {\"id\":\"c3\",\"object\":\"chat.completion.chunk\",\"created\":0,\"model\":\"gpt-4o\",\"choices\":[{\"index\":0,\"delta\":{\"content\":\"world\"},\"finish_reason\":\"stop\"}]}\n\n" +
		"data: [DONE]\n\n"

	server := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		w.Header().Set("Content-Type", "text/event-stream")
		w.Write([]byte(sseBody)) //nolint:errcheck
	}))
	defer server.Close()

	client := newTestClient(server.URL)
	var chunks []*literllm.ChatCompletionChunk
	err := client.ChatStream(context.Background(), &literllm.ChatCompletionRequest{
		Model:    "gpt-4o",
		Messages: []literllm.Message{literllm.NewTextMessage(literllm.RoleUser, "hi")},
	}, func(c *literllm.ChatCompletionChunk) error {
		chunks = append(chunks, c)
		return nil
	})
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	// Verify chunks arrive in order
	if len(chunks) != 3 {
		t.Fatalf("expected 3 chunks, got %d", len(chunks))
	}
	if chunks[0].ID != "c1" || chunks[1].ID != "c2" || chunks[2].ID != "c3" {
		t.Error("chunks not in order")
	}
}

func TestEmbed_BatchInput(t *testing.T) {
	t.Parallel()
	respBody := `{
		"object": "list",
		"data": [
			{"object": "embedding", "embedding": [0.1, 0.2, 0.3], "index": 0},
			{"object": "embedding", "embedding": [0.4, 0.5, 0.6], "index": 1},
			{"object": "embedding", "embedding": [0.7, 0.8, 0.9], "index": 2}
		],
		"model": "text-embedding-3-small",
		"usage": {"prompt_tokens": 10, "completion_tokens": 0, "total_tokens": 10}
	}`
	server := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		var reqBody struct {
			Input interface{} `json:"input"`
		}
		if err := json.NewDecoder(r.Body).Decode(&reqBody); err != nil {
			t.Errorf("failed to decode: %v", err)
		}
		// Verify input is array
		if arr, ok := reqBody.Input.([]interface{}); !ok || len(arr) != 3 {
			t.Error("expected array of 3 inputs")
		}

		w.Header().Set("Content-Type", "application/json")
		w.Write([]byte(respBody)) //nolint:errcheck
	}))
	defer server.Close()

	client := newTestClient(server.URL)
	resp, err := client.Embed(context.Background(), &literllm.EmbeddingRequest{
		Model: "text-embedding-3-small",
		Input: literllm.NewEmbeddingInputMultiple([]string{"first", "second", "third"}),
	})
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if len(resp.Data) != 3 {
		t.Fatalf("expected 3 embeddings, got %d", len(resp.Data))
	}
}

func TestAPIErrorClassification(t *testing.T) {
	t.Parallel()
	tests := []struct {
		statusCode int
		sentinel   error
		name       string
	}{
		{http.StatusForbidden, literllm.ErrAuthentication, "403 Forbidden"},
		{http.StatusBadGateway, literllm.ErrProviderError, "502 Bad Gateway"},
		{http.StatusServiceUnavailable, literllm.ErrProviderError, "503 Service Unavailable"},
		{http.StatusGatewayTimeout, literllm.ErrProviderError, "504 Gateway Timeout"},
	}

	for _, tc := range tests {
		t.Run(tc.name, func(t *testing.T) {
			t.Parallel()
			server := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
				w.WriteHeader(tc.statusCode)
			}))
			defer server.Close()

			client := newTestClient(server.URL)
			_, err := client.Chat(context.Background(), &literllm.ChatCompletionRequest{
				Model:    "gpt-4o",
				Messages: []literllm.Message{literllm.NewTextMessage(literllm.RoleUser, "hi")},
			})
			if err == nil {
				t.Fatal("expected error, got nil")
			}
			if !errors.Is(err, tc.sentinel) {
				t.Errorf("expected %v, got %T: %v", tc.sentinel, err, err)
			}
		})
	}
}

func TestStreamError_Wrapping(t *testing.T) {
	t.Parallel()
	server := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		w.Header().Set("Content-Type", "text/event-stream")
		// Send valid chunk then invalid JSON
		w.Write([]byte("data: {\"id\":\"c1\",\"object\":\"chat.completion.chunk\",\"created\":0,\"model\":\"gpt-4o\",\"choices\":[{\"index\":0,\"delta\":{\"content\":\"ok\"},\"finish_reason\":null}]}\n\n")) //nolint:errcheck
		w.Write([]byte("data: {invalid json\n\n"))                                                                                                                                                             //nolint:errcheck
	}))
	defer server.Close()

	client := newTestClient(server.URL)
	count := 0
	err := client.ChatStream(context.Background(), &literllm.ChatCompletionRequest{
		Model:    "gpt-4o",
		Messages: []literllm.Message{literllm.NewTextMessage(literllm.RoleUser, "hi")},
	}, func(c *literllm.ChatCompletionChunk) error {
		count++
		return nil
	})
	if err == nil {
		t.Fatal("expected error, got nil")
	}
	if !errors.Is(err, literllm.ErrStream) {
		t.Errorf("expected ErrStream, got %T: %v", err, err)
	}
	// Verify we got the first chunk before error
	if count != 1 {
		t.Errorf("expected 1 chunk before error, got %d", count)
	}
}

func TestNewTextMessage_AllRoles(t *testing.T) {
	t.Parallel()
	roles := []literllm.Role{
		literllm.RoleSystem,
		literllm.RoleUser,
		literllm.RoleAssistant,
		literllm.RoleTool,
		literllm.RoleDeveloper,
	}

	for _, role := range roles {
		t.Run(string(role), func(t *testing.T) {
			t.Parallel()
			msg := literllm.NewTextMessage(role, "test content")
			if msg.Role != role {
				t.Errorf("expected role %q, got %q", role, msg.Role)
			}
			var content string
			if err := json.Unmarshal(msg.Content, &content); err != nil {
				t.Fatalf("failed to unmarshal content: %v", err)
			}
			if content != "test content" {
				t.Errorf("expected content %q, got %q", "test content", content)
			}
		})
	}
}
