package literllm

import (
	"bufio"
	"bytes"
	"context"
	"encoding/json"
	"fmt"
	"io"
	"net/http"
	"strings"
	"time"
)

const (
	defaultBaseURL         = "https://api.openai.com/v1"
	defaultTimeout         = 120 * time.Second
	headerAuthorization    = "Authorization"
	headerContentType      = "Content-Type"
	headerAccept           = "Accept"
	contentTypeJSON        = "application/json"
	contentTypeEventStream = "text/event-stream"
)

// ─── Interface ────────────────────────────────────────────────────────────────

// LlmClient is the contract that all liter-llm client implementations satisfy.
type LlmClient interface {
	// Chat sends a non-streaming chat completion request.
	Chat(ctx context.Context, req *ChatCompletionRequest) (*ChatCompletionResponse, error)

	// ChatStream sends a streaming chat completion request and invokes the
	// supplied handler for each received chunk.  The stream is fully consumed
	// before this method returns; cancel ctx to abort early.
	ChatStream(ctx context.Context, req *ChatCompletionRequest, handler func(*ChatCompletionChunk) error) error

	// Embed sends an embedding request.
	Embed(ctx context.Context, req *EmbeddingRequest) (*EmbeddingResponse, error)

	// ListModels returns the list of models available via the configured
	// provider endpoint.
	ListModels(ctx context.Context) (*ModelsListResponse, error)

	// ImageGenerate generates an image from a text prompt.
	ImageGenerate(ctx context.Context, req *CreateImageRequest) (*ImagesResponse, error)

	// Speech generates audio speech from text, returning raw audio bytes.
	Speech(ctx context.Context, req *CreateSpeechRequest) ([]byte, error)

	// Transcribe transcribes audio to text.
	Transcribe(ctx context.Context, req *CreateTranscriptionRequest) (*TranscriptionResponse, error)

	// Moderate checks content against moderation policies.
	Moderate(ctx context.Context, req *ModerationRequest) (*ModerationResponse, error)

	// Rerank reranks documents by relevance to a query.
	Rerank(ctx context.Context, req *RerankRequest) (*RerankResponse, error)

	// CreateFile uploads a file.
	CreateFile(ctx context.Context, req *CreateFileRequest) (*FileObject, error)

	// RetrieveFile retrieves metadata for a file by ID.
	RetrieveFile(ctx context.Context, fileID string) (*FileObject, error)

	// DeleteFile deletes a file by ID.
	DeleteFile(ctx context.Context, fileID string) (*DeleteResponse, error)

	// ListFiles lists files, optionally filtered by query parameters.
	ListFiles(ctx context.Context, query *FileListQuery) (*FileListResponse, error)

	// FileContent retrieves the raw content of a file.
	FileContent(ctx context.Context, fileID string) ([]byte, error)

	// CreateBatch creates a new batch job.
	CreateBatch(ctx context.Context, req *CreateBatchRequest) (*BatchObject, error)

	// RetrieveBatch retrieves a batch by ID.
	RetrieveBatch(ctx context.Context, batchID string) (*BatchObject, error)

	// ListBatches lists batches, optionally filtered by query parameters.
	ListBatches(ctx context.Context, query *BatchListQuery) (*BatchListResponse, error)

	// CancelBatch cancels an in-progress batch.
	CancelBatch(ctx context.Context, batchID string) (*BatchObject, error)

	// CreateResponse creates a new response via the Responses API.
	CreateResponse(ctx context.Context, req *CreateResponseRequest) (*ResponseObject, error)

	// RetrieveResponse retrieves a response by ID.
	RetrieveResponse(ctx context.Context, responseID string) (*ResponseObject, error)

	// CancelResponse cancels an in-progress response.
	CancelResponse(ctx context.Context, responseID string) (*ResponseObject, error)
}

// ─── Config ───────────────────────────────────────────────────────────────────

// ClientConfig holds all options for constructing a [Client].
// Use [NewConfig] or individual With* option functions.
type ClientConfig struct {
	apiKey     string
	baseURL    string
	httpClient *http.Client
	cache      *CacheConfig
	budget     *BudgetConfig
}

// Option is a functional option for [NewClient].
type Option func(*ClientConfig)

// WithAPIKey sets the API key sent as a Bearer token on every request.
func WithAPIKey(key string) Option {
	return func(c *ClientConfig) {
		c.apiKey = key
	}
}

// WithBaseURL overrides the base URL used for all requests.
// The URL must not have a trailing slash.
//
// Example: "https://api.groq.com/openai/v1"
func WithBaseURL(url string) Option {
	return func(c *ClientConfig) {
		c.baseURL = strings.TrimRight(url, "/")
	}
}

// WithHTTPClient replaces the default [http.Client].  Use this to configure
// custom TLS, proxies, or transport behavior.
func WithHTTPClient(hc *http.Client) Option {
	return func(c *ClientConfig) {
		c.httpClient = hc
	}
}

// WithTimeout sets the timeout on the default HTTP client.  This option is
// ignored when [WithHTTPClient] is also provided.
func WithTimeout(d time.Duration) Option {
	return func(c *ClientConfig) {
		if c.httpClient != nil {
			c.httpClient.Timeout = d
		}
	}
}

// WithCache enables response caching with the given configuration.
func WithCache(cfg CacheConfig) Option {
	return func(c *ClientConfig) {
		c.cache = &cfg
	}
}

// WithBudget enables cost budget enforcement with the given configuration.
func WithBudget(cfg BudgetConfig) Option {
	return func(c *ClientConfig) {
		c.budget = &cfg
	}
}

// ─── Client ───────────────────────────────────────────────────────────────────

// Client is the default implementation of [LlmClient].  It calls the
// OpenAI-compatible HTTP API directly; no CGO or shared library is required.
//
// Construct one with [NewClient].  Client is safe for concurrent use.
type Client struct {
	config    ClientConfig
	hooks     []Hook
	providers []ProviderConfig
}

// NewClient constructs a Client with the supplied options.
//
// At minimum, provide [WithAPIKey] (or set the OPENAI_API_KEY environment
// variable yourself and pass an empty key if the provider does not need it).
//
//	client := literllm.NewClient(
//	    literllm.WithAPIKey(os.Getenv("OPENAI_API_KEY")),
//	)
func NewClient(opts ...Option) *Client {
	cfg := ClientConfig{
		baseURL: defaultBaseURL,
		httpClient: &http.Client{
			Timeout: defaultTimeout,
		},
	}
	for _, opt := range opts {
		opt(&cfg)
	}
	return &Client{config: cfg}
}

// ─── Hooks & Custom Providers ─────────────────────────────────────────────

// AddHook registers a lifecycle hook.  Hooks are invoked in registration order
// before each request, after each successful response, and after each error.
func (c *Client) AddHook(hook Hook) {
	c.hooks = append(c.hooks, hook)
}

// RegisterProvider adds a custom provider configuration.  Requests whose model
// name starts with one of the provider's prefixes are routed to its base URL.
func (c *Client) RegisterProvider(cfg ProviderConfig) {
	c.providers = append(c.providers, cfg)
}

// runOnRequest invokes OnRequest on every registered hook.  The first non-nil
// error aborts and is returned wrapped with ErrHookRejected.
func (c *Client) runOnRequest(ctx context.Context, req interface{}) error {
	for _, h := range c.hooks {
		if err := h.OnRequest(ctx, req); err != nil {
			return fmt.Errorf("%w: %v", ErrHookRejected, err)
		}
	}
	return nil
}

// runOnResponse invokes OnResponse on every registered hook.
func (c *Client) runOnResponse(ctx context.Context, req, resp interface{}) {
	for _, h := range c.hooks {
		h.OnResponse(ctx, req, resp)
	}
}

// runOnError invokes OnError on every registered hook.
func (c *Client) runOnError(ctx context.Context, req interface{}, err error) {
	for _, h := range c.hooks {
		h.OnError(ctx, req, err)
	}
}

// ─── HTTP helpers ─────────────────────────────────────────────────────────────

// buildRequest creates an HTTP request with the common headers set.
func (c *Client) buildRequest(ctx context.Context, method, path string, body io.Reader, stream bool) (*http.Request, error) {
	url := c.config.baseURL + path
	req, err := http.NewRequestWithContext(ctx, method, url, body)
	if err != nil {
		return nil, fmt.Errorf("literllm: build request: %w", err)
	}
	if c.config.apiKey != "" {
		req.Header.Set(headerAuthorization, "Bearer "+c.config.apiKey)
	}
	if body != nil {
		req.Header.Set(headerContentType, contentTypeJSON)
	}
	if stream {
		req.Header.Set(headerAccept, contentTypeEventStream)
	} else {
		req.Header.Set(headerAccept, contentTypeJSON)
	}
	return req, nil
}

// do executes an HTTP request and returns the response body, or an *APIError
// for non-2xx responses.  The caller is responsible for closing the body.
func (c *Client) do(req *http.Request) (*http.Response, error) {
	resp, err := c.config.httpClient.Do(req) //nolint:gosec // URL is from trusted config, not user input
	if err != nil {
		return nil, fmt.Errorf("literllm: HTTP request failed: %w", err)
	}
	if resp.StatusCode < 200 || resp.StatusCode > 299 {
		defer resp.Body.Close()
		msg := extractErrorMessage(resp)
		return nil, newAPIError(resp.StatusCode, msg)
	}
	return resp, nil
}

// extractErrorMessage reads a JSON error body and returns the message string.
// Falls back to the HTTP status text on any parse failure.
func extractErrorMessage(resp *http.Response) string {
	body, err := io.ReadAll(io.LimitReader(resp.Body, 8192))
	if err != nil || len(body) == 0 {
		return http.StatusText(resp.StatusCode)
	}

	// Try OpenAI-style {"error": {"message": "..."}}
	var envelope struct {
		Error struct {
			Message string `json:"message"`
		} `json:"error"`
	}
	if json.Unmarshal(body, &envelope) == nil && envelope.Error.Message != "" {
		return envelope.Error.Message
	}

	// Try flat {"message": "..."}
	var flat struct {
		Message string `json:"message"`
	}
	if json.Unmarshal(body, &flat) == nil && flat.Message != "" {
		return flat.Message
	}

	return string(body)
}

// marshalBody JSON-encodes v and returns an io.Reader.
func marshalBody(v any) (io.Reader, error) {
	data, err := json.Marshal(v)
	if err != nil {
		return nil, fmt.Errorf("literllm: marshal request body: %w", err)
	}
	return bytes.NewReader(data), nil
}

// ─── Chat ─────────────────────────────────────────────────────────────────────

// Chat sends a non-streaming chat completion request and returns the full
// response.
//
// The req.Stream field is forced to false; use [Client.ChatStream] for
// streaming.
func (c *Client) Chat(ctx context.Context, req *ChatCompletionRequest) (*ChatCompletionResponse, error) {
	if req == nil {
		return nil, fmt.Errorf("%w: request must not be nil", ErrInvalidRequest)
	}
	if req.Model == "" {
		return nil, fmt.Errorf("%w: model is required", ErrInvalidRequest)
	}
	if len(req.Messages) == 0 {
		return nil, fmt.Errorf("%w: messages must not be empty", ErrInvalidRequest)
	}

	// Run pre-request hooks.
	if err := c.runOnRequest(ctx, req); err != nil {
		return nil, err
	}

	// Ensure stream is off for this path.
	streamFalse := false
	copy := *req
	copy.Stream = &streamFalse

	body, err := marshalBody(&copy)
	if err != nil {
		c.runOnError(ctx, req, err)
		return nil, err
	}

	httpReq, err := c.buildRequest(ctx, http.MethodPost, "/chat/completions", body, false)
	if err != nil {
		c.runOnError(ctx, req, err)
		return nil, err
	}

	resp, err := c.do(httpReq)
	if err != nil {
		c.runOnError(ctx, req, err)
		return nil, err
	}
	defer resp.Body.Close()

	var result ChatCompletionResponse
	if err := json.NewDecoder(resp.Body).Decode(&result); err != nil {
		decodeErr := fmt.Errorf("literllm: decode chat response: %w", err)
		c.runOnError(ctx, req, decodeErr)
		return nil, decodeErr
	}

	c.runOnResponse(ctx, req, &result)
	return &result, nil
}

// ChatStream sends a streaming chat completion request.
//
// The handler is invoked once for each server-sent event chunk.  If handler
// returns a non-nil error the stream is aborted and that error is returned by
// ChatStream.  Canceling ctx also aborts the stream.
//
// The req.Stream field is forced to true.
func (c *Client) ChatStream(ctx context.Context, req *ChatCompletionRequest, handler func(*ChatCompletionChunk) error) error {
	if req == nil {
		return fmt.Errorf("%w: request must not be nil", ErrInvalidRequest)
	}
	if req.Model == "" {
		return fmt.Errorf("%w: model is required", ErrInvalidRequest)
	}
	if len(req.Messages) == 0 {
		return fmt.Errorf("%w: messages must not be empty", ErrInvalidRequest)
	}
	if handler == nil {
		return fmt.Errorf("%w: handler must not be nil", ErrInvalidRequest)
	}

	// Run pre-request hooks.
	if err := c.runOnRequest(ctx, req); err != nil {
		return err
	}

	streamTrue := true
	copy := *req
	copy.Stream = &streamTrue

	body, err := marshalBody(&copy)
	if err != nil {
		c.runOnError(ctx, req, err)
		return err
	}

	httpReq, err := c.buildRequest(ctx, http.MethodPost, "/chat/completions", body, true)
	if err != nil {
		c.runOnError(ctx, req, err)
		return err
	}

	resp, err := c.do(httpReq)
	if err != nil {
		c.runOnError(ctx, req, err)
		return err
	}
	defer resp.Body.Close()

	if err := parseSSEStream(resp.Body, handler); err != nil {
		c.runOnError(ctx, req, err)
		return err
	}

	c.runOnResponse(ctx, req, nil)
	return nil
}

// parseSSEStream reads an SSE response body, parses each data line as a
// ChatCompletionChunk, and invokes handler for each chunk.
func parseSSEStream(body io.Reader, handler func(*ChatCompletionChunk) error) error {
	scanner := bufio.NewScanner(body)
	for scanner.Scan() {
		line := scanner.Text()

		// SSE lines that do not start with "data:" are comments or blank —
		// skip them.
		if !strings.HasPrefix(line, "data:") {
			continue
		}

		payload := strings.TrimSpace(strings.TrimPrefix(line, "data:"))

		// "[DONE]" signals the end of the stream.
		if payload == "[DONE]" {
			break
		}

		var chunk ChatCompletionChunk
		if err := json.Unmarshal([]byte(payload), &chunk); err != nil {
			return newStreamError("failed to parse chunk JSON", err)
		}

		if err := handler(&chunk); err != nil {
			return err
		}
	}

	if err := scanner.Err(); err != nil {
		return newStreamError("error reading stream", err)
	}
	return nil
}

// ─── Embed ────────────────────────────────────────────────────────────────────

// Embed sends an embedding request and returns the response.
func (c *Client) Embed(ctx context.Context, req *EmbeddingRequest) (*EmbeddingResponse, error) {
	if req == nil {
		return nil, fmt.Errorf("%w: request must not be nil", ErrInvalidRequest)
	}
	if req.Model == "" {
		return nil, fmt.Errorf("%w: model is required", ErrInvalidRequest)
	}

	if err := c.runOnRequest(ctx, req); err != nil {
		return nil, err
	}

	body, err := marshalBody(req)
	if err != nil {
		c.runOnError(ctx, req, err)
		return nil, err
	}

	httpReq, err := c.buildRequest(ctx, http.MethodPost, "/embeddings", body, false)
	if err != nil {
		c.runOnError(ctx, req, err)
		return nil, err
	}

	resp, err := c.do(httpReq)
	if err != nil {
		c.runOnError(ctx, req, err)
		return nil, err
	}
	defer resp.Body.Close()

	var result EmbeddingResponse
	if err := json.NewDecoder(resp.Body).Decode(&result); err != nil {
		decodeErr := fmt.Errorf("literllm: decode embedding response: %w", err)
		c.runOnError(ctx, req, decodeErr)
		return nil, decodeErr
	}

	c.runOnResponse(ctx, req, &result)
	return &result, nil
}

// ─── List Models ──────────────────────────────────────────────────────────────

// ListModels retrieves the list of models from the configured provider endpoint.
func (c *Client) ListModels(ctx context.Context) (*ModelsListResponse, error) {
	// ListModels has no request body; use nil as the hook request sentinel.
	if err := c.runOnRequest(ctx, nil); err != nil {
		return nil, err
	}

	httpReq, err := c.buildRequest(ctx, http.MethodGet, "/models", nil, false)
	if err != nil {
		c.runOnError(ctx, nil, err)
		return nil, err
	}

	resp, err := c.do(httpReq)
	if err != nil {
		c.runOnError(ctx, nil, err)
		return nil, err
	}
	defer resp.Body.Close()

	var result ModelsListResponse
	if err := json.NewDecoder(resp.Body).Decode(&result); err != nil {
		decodeErr := fmt.Errorf("literllm: decode models response: %w", err)
		c.runOnError(ctx, nil, decodeErr)
		return nil, decodeErr
	}

	c.runOnResponse(ctx, nil, &result)
	return &result, nil
}

// ─── Image Generate ───────────────────────────────────────────────────────────

// ImageGenerate sends an image generation request and returns the response.
func (c *Client) ImageGenerate(ctx context.Context, req *CreateImageRequest) (*ImagesResponse, error) {
	if req == nil {
		return nil, fmt.Errorf("%w: request must not be nil", ErrInvalidRequest)
	}
	if req.Prompt == "" {
		return nil, fmt.Errorf("%w: prompt is required", ErrInvalidRequest)
	}

	if err := c.runOnRequest(ctx, req); err != nil {
		return nil, err
	}

	body, err := marshalBody(req)
	if err != nil {
		c.runOnError(ctx, req, err)
		return nil, err
	}

	httpReq, err := c.buildRequest(ctx, http.MethodPost, "/images/generations", body, false)
	if err != nil {
		c.runOnError(ctx, req, err)
		return nil, err
	}

	resp, err := c.do(httpReq)
	if err != nil {
		c.runOnError(ctx, req, err)
		return nil, err
	}
	defer resp.Body.Close()

	var result ImagesResponse
	if err := json.NewDecoder(resp.Body).Decode(&result); err != nil {
		decodeErr := fmt.Errorf("literllm: decode image response: %w", err)
		c.runOnError(ctx, req, decodeErr)
		return nil, decodeErr
	}

	c.runOnResponse(ctx, req, &result)
	return &result, nil
}

// ─── Speech ───────────────────────────────────────────────────────────────────

// Speech generates audio from text and returns raw audio bytes.
func (c *Client) Speech(ctx context.Context, req *CreateSpeechRequest) ([]byte, error) {
	if req == nil {
		return nil, fmt.Errorf("%w: request must not be nil", ErrInvalidRequest)
	}
	if req.Model == "" {
		return nil, fmt.Errorf("%w: model is required", ErrInvalidRequest)
	}
	if req.Input == "" {
		return nil, fmt.Errorf("%w: input is required", ErrInvalidRequest)
	}
	if req.Voice == "" {
		return nil, fmt.Errorf("%w: voice is required", ErrInvalidRequest)
	}

	if err := c.runOnRequest(ctx, req); err != nil {
		return nil, err
	}

	body, err := marshalBody(req)
	if err != nil {
		c.runOnError(ctx, req, err)
		return nil, err
	}

	httpReq, err := c.buildRequest(ctx, http.MethodPost, "/audio/speech", body, false)
	if err != nil {
		c.runOnError(ctx, req, err)
		return nil, err
	}

	resp, err := c.do(httpReq)
	if err != nil {
		c.runOnError(ctx, req, err)
		return nil, err
	}
	defer resp.Body.Close()

	data, err := io.ReadAll(resp.Body)
	if err != nil {
		readErr := fmt.Errorf("literllm: read speech response: %w", err)
		c.runOnError(ctx, req, readErr)
		return nil, readErr
	}

	c.runOnResponse(ctx, req, data)
	return data, nil
}

// ─── Transcribe ───────────────────────────────────────────────────────────────

// Transcribe sends a transcription request and returns the response.
func (c *Client) Transcribe(ctx context.Context, req *CreateTranscriptionRequest) (*TranscriptionResponse, error) {
	if req == nil {
		return nil, fmt.Errorf("%w: request must not be nil", ErrInvalidRequest)
	}
	if req.Model == "" {
		return nil, fmt.Errorf("%w: model is required", ErrInvalidRequest)
	}

	if err := c.runOnRequest(ctx, req); err != nil {
		return nil, err
	}

	body, err := marshalBody(req)
	if err != nil {
		c.runOnError(ctx, req, err)
		return nil, err
	}

	httpReq, err := c.buildRequest(ctx, http.MethodPost, "/audio/transcriptions", body, false)
	if err != nil {
		c.runOnError(ctx, req, err)
		return nil, err
	}

	resp, err := c.do(httpReq)
	if err != nil {
		c.runOnError(ctx, req, err)
		return nil, err
	}
	defer resp.Body.Close()

	var result TranscriptionResponse
	if err := json.NewDecoder(resp.Body).Decode(&result); err != nil {
		decodeErr := fmt.Errorf("literllm: decode transcription response: %w", err)
		c.runOnError(ctx, req, decodeErr)
		return nil, decodeErr
	}

	c.runOnResponse(ctx, req, &result)
	return &result, nil
}

// ─── Moderate ─────────────────────────────────────────────────────────────────

// Moderate checks content against moderation policies.
func (c *Client) Moderate(ctx context.Context, req *ModerationRequest) (*ModerationResponse, error) {
	if req == nil {
		return nil, fmt.Errorf("%w: request must not be nil", ErrInvalidRequest)
	}

	if err := c.runOnRequest(ctx, req); err != nil {
		return nil, err
	}

	body, err := marshalBody(req)
	if err != nil {
		c.runOnError(ctx, req, err)
		return nil, err
	}

	httpReq, err := c.buildRequest(ctx, http.MethodPost, "/moderations", body, false)
	if err != nil {
		c.runOnError(ctx, req, err)
		return nil, err
	}

	resp, err := c.do(httpReq)
	if err != nil {
		c.runOnError(ctx, req, err)
		return nil, err
	}
	defer resp.Body.Close()

	var result ModerationResponse
	if err := json.NewDecoder(resp.Body).Decode(&result); err != nil {
		decodeErr := fmt.Errorf("literllm: decode moderation response: %w", err)
		c.runOnError(ctx, req, decodeErr)
		return nil, decodeErr
	}

	c.runOnResponse(ctx, req, &result)
	return &result, nil
}

// ─── Rerank ───────────────────────────────────────────────────────────────────

// Rerank reranks documents by relevance to a query.
func (c *Client) Rerank(ctx context.Context, req *RerankRequest) (*RerankResponse, error) {
	if req == nil {
		return nil, fmt.Errorf("%w: request must not be nil", ErrInvalidRequest)
	}
	if req.Model == "" {
		return nil, fmt.Errorf("%w: model is required", ErrInvalidRequest)
	}
	if req.Query == "" {
		return nil, fmt.Errorf("%w: query is required", ErrInvalidRequest)
	}

	if err := c.runOnRequest(ctx, req); err != nil {
		return nil, err
	}

	body, err := marshalBody(req)
	if err != nil {
		c.runOnError(ctx, req, err)
		return nil, err
	}

	httpReq, err := c.buildRequest(ctx, http.MethodPost, "/rerank", body, false)
	if err != nil {
		c.runOnError(ctx, req, err)
		return nil, err
	}

	resp, err := c.do(httpReq)
	if err != nil {
		c.runOnError(ctx, req, err)
		return nil, err
	}
	defer resp.Body.Close()

	var result RerankResponse
	if err := json.NewDecoder(resp.Body).Decode(&result); err != nil {
		decodeErr := fmt.Errorf("literllm: decode rerank response: %w", err)
		c.runOnError(ctx, req, decodeErr)
		return nil, decodeErr
	}

	c.runOnResponse(ctx, req, &result)
	return &result, nil
}

// ─── File Management ──────────────────────────────────────────────────────────

// CreateFile uploads a file.
func (c *Client) CreateFile(ctx context.Context, req *CreateFileRequest) (*FileObject, error) {
	if req == nil {
		return nil, fmt.Errorf("%w: request must not be nil", ErrInvalidRequest)
	}

	if err := c.runOnRequest(ctx, req); err != nil {
		return nil, err
	}

	body, err := marshalBody(req)
	if err != nil {
		c.runOnError(ctx, req, err)
		return nil, err
	}

	httpReq, err := c.buildRequest(ctx, http.MethodPost, "/files", body, false)
	if err != nil {
		c.runOnError(ctx, req, err)
		return nil, err
	}

	resp, err := c.do(httpReq)
	if err != nil {
		c.runOnError(ctx, req, err)
		return nil, err
	}
	defer resp.Body.Close()

	var result FileObject
	if err := json.NewDecoder(resp.Body).Decode(&result); err != nil {
		decodeErr := fmt.Errorf("literllm: decode file response: %w", err)
		c.runOnError(ctx, req, decodeErr)
		return nil, decodeErr
	}

	c.runOnResponse(ctx, req, &result)
	return &result, nil
}

// RetrieveFile retrieves metadata for a file by ID.
func (c *Client) RetrieveFile(ctx context.Context, fileID string) (*FileObject, error) {
	if fileID == "" {
		return nil, fmt.Errorf("%w: file_id is required", ErrInvalidRequest)
	}

	if err := c.runOnRequest(ctx, fileID); err != nil {
		return nil, err
	}

	httpReq, err := c.buildRequest(ctx, http.MethodGet, "/files/"+fileID, nil, false)
	if err != nil {
		c.runOnError(ctx, fileID, err)
		return nil, err
	}

	resp, err := c.do(httpReq)
	if err != nil {
		c.runOnError(ctx, fileID, err)
		return nil, err
	}
	defer resp.Body.Close()

	var result FileObject
	if err := json.NewDecoder(resp.Body).Decode(&result); err != nil {
		decodeErr := fmt.Errorf("literllm: decode file response: %w", err)
		c.runOnError(ctx, fileID, decodeErr)
		return nil, decodeErr
	}

	c.runOnResponse(ctx, fileID, &result)
	return &result, nil
}

// DeleteFile deletes a file by ID.
func (c *Client) DeleteFile(ctx context.Context, fileID string) (*DeleteResponse, error) {
	if fileID == "" {
		return nil, fmt.Errorf("%w: file_id is required", ErrInvalidRequest)
	}

	if err := c.runOnRequest(ctx, fileID); err != nil {
		return nil, err
	}

	httpReq, err := c.buildRequest(ctx, http.MethodDelete, "/files/"+fileID, nil, false)
	if err != nil {
		c.runOnError(ctx, fileID, err)
		return nil, err
	}

	resp, err := c.do(httpReq)
	if err != nil {
		c.runOnError(ctx, fileID, err)
		return nil, err
	}
	defer resp.Body.Close()

	var result DeleteResponse
	if err := json.NewDecoder(resp.Body).Decode(&result); err != nil {
		decodeErr := fmt.Errorf("literllm: decode delete response: %w", err)
		c.runOnError(ctx, fileID, decodeErr)
		return nil, decodeErr
	}

	c.runOnResponse(ctx, fileID, &result)
	return &result, nil
}

// ListFiles lists files, optionally filtered by query parameters.
func (c *Client) ListFiles(ctx context.Context, query *FileListQuery) (*FileListResponse, error) {
	if err := c.runOnRequest(ctx, query); err != nil {
		return nil, err
	}

	path := "/files"
	if query != nil {
		var params []string
		if query.Purpose != nil {
			params = append(params, "purpose="+*query.Purpose)
		}
		if query.Limit != nil {
			params = append(params, fmt.Sprintf("limit=%d", *query.Limit))
		}
		if query.After != nil {
			params = append(params, "after="+*query.After)
		}
		if len(params) > 0 {
			path += "?" + strings.Join(params, "&")
		}
	}

	httpReq, err := c.buildRequest(ctx, http.MethodGet, path, nil, false)
	if err != nil {
		c.runOnError(ctx, query, err)
		return nil, err
	}

	resp, err := c.do(httpReq)
	if err != nil {
		c.runOnError(ctx, query, err)
		return nil, err
	}
	defer resp.Body.Close()

	var result FileListResponse
	if err := json.NewDecoder(resp.Body).Decode(&result); err != nil {
		decodeErr := fmt.Errorf("literllm: decode file list response: %w", err)
		c.runOnError(ctx, query, decodeErr)
		return nil, decodeErr
	}

	c.runOnResponse(ctx, query, &result)
	return &result, nil
}

// FileContent retrieves the raw content of a file.
func (c *Client) FileContent(ctx context.Context, fileID string) ([]byte, error) {
	if fileID == "" {
		return nil, fmt.Errorf("%w: file_id is required", ErrInvalidRequest)
	}

	if err := c.runOnRequest(ctx, fileID); err != nil {
		return nil, err
	}

	httpReq, err := c.buildRequest(ctx, http.MethodGet, "/files/"+fileID+"/content", nil, false)
	if err != nil {
		c.runOnError(ctx, fileID, err)
		return nil, err
	}

	resp, err := c.do(httpReq)
	if err != nil {
		c.runOnError(ctx, fileID, err)
		return nil, err
	}
	defer resp.Body.Close()

	data, err := io.ReadAll(resp.Body)
	if err != nil {
		readErr := fmt.Errorf("literllm: read file content: %w", err)
		c.runOnError(ctx, fileID, readErr)
		return nil, readErr
	}

	c.runOnResponse(ctx, fileID, data)
	return data, nil
}

// ─── Batch Management ─────────────────────────────────────────────────────────

// CreateBatch creates a new batch job.
func (c *Client) CreateBatch(ctx context.Context, req *CreateBatchRequest) (*BatchObject, error) {
	if req == nil {
		return nil, fmt.Errorf("%w: request must not be nil", ErrInvalidRequest)
	}

	if err := c.runOnRequest(ctx, req); err != nil {
		return nil, err
	}

	body, err := marshalBody(req)
	if err != nil {
		c.runOnError(ctx, req, err)
		return nil, err
	}

	httpReq, err := c.buildRequest(ctx, http.MethodPost, "/batches", body, false)
	if err != nil {
		c.runOnError(ctx, req, err)
		return nil, err
	}

	resp, err := c.do(httpReq)
	if err != nil {
		c.runOnError(ctx, req, err)
		return nil, err
	}
	defer resp.Body.Close()

	var result BatchObject
	if err := json.NewDecoder(resp.Body).Decode(&result); err != nil {
		decodeErr := fmt.Errorf("literllm: decode batch response: %w", err)
		c.runOnError(ctx, req, decodeErr)
		return nil, decodeErr
	}

	c.runOnResponse(ctx, req, &result)
	return &result, nil
}

// RetrieveBatch retrieves a batch by ID.
func (c *Client) RetrieveBatch(ctx context.Context, batchID string) (*BatchObject, error) {
	if batchID == "" {
		return nil, fmt.Errorf("%w: batch_id is required", ErrInvalidRequest)
	}

	if err := c.runOnRequest(ctx, batchID); err != nil {
		return nil, err
	}

	httpReq, err := c.buildRequest(ctx, http.MethodGet, "/batches/"+batchID, nil, false)
	if err != nil {
		c.runOnError(ctx, batchID, err)
		return nil, err
	}

	resp, err := c.do(httpReq)
	if err != nil {
		c.runOnError(ctx, batchID, err)
		return nil, err
	}
	defer resp.Body.Close()

	var result BatchObject
	if err := json.NewDecoder(resp.Body).Decode(&result); err != nil {
		decodeErr := fmt.Errorf("literllm: decode batch response: %w", err)
		c.runOnError(ctx, batchID, decodeErr)
		return nil, decodeErr
	}

	c.runOnResponse(ctx, batchID, &result)
	return &result, nil
}

// ListBatches lists batches, optionally filtered by query parameters.
func (c *Client) ListBatches(ctx context.Context, query *BatchListQuery) (*BatchListResponse, error) {
	if err := c.runOnRequest(ctx, query); err != nil {
		return nil, err
	}

	path := "/batches"
	if query != nil {
		var params []string
		if query.Limit != nil {
			params = append(params, fmt.Sprintf("limit=%d", *query.Limit))
		}
		if query.After != nil {
			params = append(params, "after="+*query.After)
		}
		if len(params) > 0 {
			path += "?" + strings.Join(params, "&")
		}
	}

	httpReq, err := c.buildRequest(ctx, http.MethodGet, path, nil, false)
	if err != nil {
		c.runOnError(ctx, query, err)
		return nil, err
	}

	resp, err := c.do(httpReq)
	if err != nil {
		c.runOnError(ctx, query, err)
		return nil, err
	}
	defer resp.Body.Close()

	var result BatchListResponse
	if err := json.NewDecoder(resp.Body).Decode(&result); err != nil {
		decodeErr := fmt.Errorf("literllm: decode batch list response: %w", err)
		c.runOnError(ctx, query, decodeErr)
		return nil, decodeErr
	}

	c.runOnResponse(ctx, query, &result)
	return &result, nil
}

// CancelBatch cancels an in-progress batch.
func (c *Client) CancelBatch(ctx context.Context, batchID string) (*BatchObject, error) {
	if batchID == "" {
		return nil, fmt.Errorf("%w: batch_id is required", ErrInvalidRequest)
	}

	if err := c.runOnRequest(ctx, batchID); err != nil {
		return nil, err
	}

	httpReq, err := c.buildRequest(ctx, http.MethodPost, "/batches/"+batchID+"/cancel", nil, false)
	if err != nil {
		c.runOnError(ctx, batchID, err)
		return nil, err
	}

	resp, err := c.do(httpReq)
	if err != nil {
		c.runOnError(ctx, batchID, err)
		return nil, err
	}
	defer resp.Body.Close()

	var result BatchObject
	if err := json.NewDecoder(resp.Body).Decode(&result); err != nil {
		decodeErr := fmt.Errorf("literllm: decode batch response: %w", err)
		c.runOnError(ctx, batchID, decodeErr)
		return nil, decodeErr
	}

	c.runOnResponse(ctx, batchID, &result)
	return &result, nil
}

// ─── Responses API ────────────────────────────────────────────────────────────

// CreateResponse creates a new response via the Responses API.
func (c *Client) CreateResponse(ctx context.Context, req *CreateResponseRequest) (*ResponseObject, error) {
	if req == nil {
		return nil, fmt.Errorf("%w: request must not be nil", ErrInvalidRequest)
	}
	if req.Model == "" {
		return nil, fmt.Errorf("%w: model is required", ErrInvalidRequest)
	}

	if err := c.runOnRequest(ctx, req); err != nil {
		return nil, err
	}

	body, err := marshalBody(req)
	if err != nil {
		c.runOnError(ctx, req, err)
		return nil, err
	}

	httpReq, err := c.buildRequest(ctx, http.MethodPost, "/responses", body, false)
	if err != nil {
		c.runOnError(ctx, req, err)
		return nil, err
	}

	resp, err := c.do(httpReq)
	if err != nil {
		c.runOnError(ctx, req, err)
		return nil, err
	}
	defer resp.Body.Close()

	var result ResponseObject
	if err := json.NewDecoder(resp.Body).Decode(&result); err != nil {
		decodeErr := fmt.Errorf("literllm: decode response: %w", err)
		c.runOnError(ctx, req, decodeErr)
		return nil, decodeErr
	}

	c.runOnResponse(ctx, req, &result)
	return &result, nil
}

// RetrieveResponse retrieves a response by ID.
func (c *Client) RetrieveResponse(ctx context.Context, responseID string) (*ResponseObject, error) {
	if responseID == "" {
		return nil, fmt.Errorf("%w: response_id is required", ErrInvalidRequest)
	}

	if err := c.runOnRequest(ctx, responseID); err != nil {
		return nil, err
	}

	httpReq, err := c.buildRequest(ctx, http.MethodGet, "/responses/"+responseID, nil, false)
	if err != nil {
		c.runOnError(ctx, responseID, err)
		return nil, err
	}

	resp, err := c.do(httpReq)
	if err != nil {
		c.runOnError(ctx, responseID, err)
		return nil, err
	}
	defer resp.Body.Close()

	var result ResponseObject
	if err := json.NewDecoder(resp.Body).Decode(&result); err != nil {
		decodeErr := fmt.Errorf("literllm: decode response: %w", err)
		c.runOnError(ctx, responseID, decodeErr)
		return nil, decodeErr
	}

	c.runOnResponse(ctx, responseID, &result)
	return &result, nil
}

// CancelResponse cancels an in-progress response.
func (c *Client) CancelResponse(ctx context.Context, responseID string) (*ResponseObject, error) {
	if responseID == "" {
		return nil, fmt.Errorf("%w: response_id is required", ErrInvalidRequest)
	}

	if err := c.runOnRequest(ctx, responseID); err != nil {
		return nil, err
	}

	httpReq, err := c.buildRequest(ctx, http.MethodPost, "/responses/"+responseID+"/cancel", nil, false)
	if err != nil {
		c.runOnError(ctx, responseID, err)
		return nil, err
	}

	resp, err := c.do(httpReq)
	if err != nil {
		c.runOnError(ctx, responseID, err)
		return nil, err
	}
	defer resp.Body.Close()

	var result ResponseObject
	if err := json.NewDecoder(resp.Body).Decode(&result); err != nil {
		decodeErr := fmt.Errorf("literllm: decode response: %w", err)
		c.runOnError(ctx, responseID, decodeErr)
		return nil, decodeErr
	}

	c.runOnResponse(ctx, responseID, &result)
	return &result, nil
}

// compile-time assertion: *Client must implement LlmClient.
var _ LlmClient = (*Client)(nil)
