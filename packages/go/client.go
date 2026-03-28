package literllm

/*
#include "internal/ffi/liter_llm.h"
#include <stdlib.h>

// goStreamCallback is exported from Go via //export; declare it here so
// the static C helper below can reference it.
extern void goStreamCallback(char *chunk_json, void *user_data);

// stream_callback_wrapper casts the Go-exported callback to match the
// const char* signature expected by literllm_chat_stream.
static void stream_callback_wrapper(const char *chunk_json, void *user_data) {
    goStreamCallback((char *)chunk_json, user_data);
}

// call_chat_stream wraps literllm_chat_stream with the Go-exported callback
// so that Go code never needs to take the address of goStreamCallback via
// C.goStreamCallback (which cgo cannot resolve directly).
static int call_chat_stream(const LiterLlmClient *client,
                            const char *request_json,
                            void *user_data) {
    return literllm_chat_stream(client, request_json, stream_callback_wrapper, user_data);
}
*/
import "C"

import (
	"context"
	"encoding/base64"
	"encoding/json"
	"fmt"
	"net/http"
	"runtime"
	"strings"
	"sync"
	"sync/atomic"
	"unsafe"
)

// ─── Stream callback registry ────────────────────────────────────────────────

// streamCallbackRegistry maps unique IDs to Go stream handler functions.
// This is necessary because cgo cannot pass Go function pointers directly
// to C; instead we pass an opaque integer ID as user_data and look up the
// handler in this registry.
var (
	streamCallbackMu   sync.Mutex
	streamCallbackMap  = make(map[uintptr]streamCallbackEntry)
	streamCallbackNext uintptr
)

type streamCallbackEntry struct {
	handler func(*ChatCompletionChunk) error
	err     error
}

func registerStreamCallback(handler func(*ChatCompletionChunk) error) uintptr {
	streamCallbackMu.Lock()
	defer streamCallbackMu.Unlock()
	streamCallbackNext++
	id := streamCallbackNext
	streamCallbackMap[id] = streamCallbackEntry{handler: handler}
	return id
}

func unregisterStreamCallback(id uintptr) error {
	streamCallbackMu.Lock()
	defer streamCallbackMu.Unlock()
	entry := streamCallbackMap[id]
	delete(streamCallbackMap, id)
	return entry.err
}

//export goStreamCallback
func goStreamCallback(chunkJSON *C.char, userData unsafe.Pointer) {
	id := uintptr(userData)
	streamCallbackMu.Lock()
	entry, ok := streamCallbackMap[id]
	streamCallbackMu.Unlock()
	if !ok {
		return
	}

	// If a previous callback already errored, skip processing.
	if entry.err != nil {
		return
	}

	var chunk ChatCompletionChunk
	if err := json.Unmarshal([]byte(C.GoString(chunkJSON)), &chunk); err != nil {
		streamCallbackMu.Lock()
		streamCallbackMap[id] = streamCallbackEntry{handler: entry.handler, err: newStreamError("failed to parse chunk JSON", err)}
		streamCallbackMu.Unlock()
		return
	}

	if err := entry.handler(&chunk); err != nil {
		streamCallbackMu.Lock()
		streamCallbackMap[id] = streamCallbackEntry{handler: entry.handler, err: err}
		streamCallbackMu.Unlock()
	}
}

// ─── Interface ───────────────────────────────────────────────────────────────

// LlmClient is the contract that all liter-llm client implementations satisfy.
//
// Note: context.Context parameters are accepted for interface compatibility and
// to follow Go conventions, but they are NOT propagated to the FFI layer.
// The underlying Rust library does not support context-based cancellation;
// once an FFI call begins it will run to completion.
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

	// Search performs a web search using the configured provider.
	Search(ctx context.Context, req *SearchRequest) (*SearchResponse, error)

	// Ocr performs optical character recognition on images.
	Ocr(ctx context.Context, req *OcrRequest) (*OcrResponse, error)
}

// ─── Config ──────────────────────────────────────────────────────────────────

// ffiConfig is the JSON schema accepted by literllm_client_new_with_config.
type ffiConfig struct {
	APIKey          string            `json:"api_key"`
	BaseURL         string            `json:"base_url,omitempty"`
	ModelHint       string            `json:"model_hint,omitempty"`
	MaxRetries      *int              `json:"max_retries,omitempty"`
	TimeoutSecs     *int              `json:"timeout_secs,omitempty"`
	ExtraHeaders    map[string]string `json:"extra_headers,omitempty"`
	Cache           *CacheConfig      `json:"cache,omitempty"`
	Budget          *BudgetConfig     `json:"budget,omitempty"`
	CooldownSecs    *int              `json:"cooldown_secs,omitempty"`
	RateLimit       *ffiRateLimit     `json:"rate_limit,omitempty"`
	HealthCheckSecs *int              `json:"health_check_secs,omitempty"`
	CostTracking    bool              `json:"cost_tracking,omitempty"`
	Tracing         bool              `json:"tracing,omitempty"`
}

// ffiRateLimit is the rate-limit sub-object inside ffiConfig.
type ffiRateLimit struct {
	RPM int `json:"rpm"`
	TPM int `json:"tpm"`
}

// ClientConfig holds all options for constructing a [Client].
// Use individual With* option functions.
type ClientConfig struct {
	apiKey          string
	baseURL         string
	modelHint       string
	cache           *CacheConfig
	budget          *BudgetConfig
	cooldownSecs    *int
	rateLimitRPM    *int
	rateLimitTPM    *int
	healthCheckSecs *int
	costTracking    bool
	tracing         bool
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
//
// Example: "https://api.groq.com/openai/v1"
func WithBaseURL(url string) Option {
	return func(c *ClientConfig) {
		c.baseURL = url
	}
}

// WithModelHint sets a model name hint for provider auto-detection
// (e.g. "groq/llama3-70b").  Used only when BaseURL is not set.
func WithModelHint(hint string) Option {
	return func(c *ClientConfig) {
		c.modelHint = hint
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

// WithCooldown sets the cooldown period in seconds after a provider error
// before retrying that provider.
func WithCooldown(seconds int) Option {
	return func(c *ClientConfig) {
		c.cooldownSecs = &seconds
	}
}

// WithRateLimit sets the rate limit for requests per minute (RPM) and
// tokens per minute (TPM).
func WithRateLimit(rpm, tpm int) Option {
	return func(c *ClientConfig) {
		c.rateLimitRPM = &rpm
		c.rateLimitTPM = &tpm
	}
}

// WithHealthCheck sets the interval in seconds for provider health checks.
func WithHealthCheck(seconds int) Option {
	return func(c *ClientConfig) {
		c.healthCheckSecs = &seconds
	}
}

// WithCostTracking enables or disables cost tracking for requests.
func WithCostTracking(enabled bool) Option {
	return func(c *ClientConfig) {
		c.costTracking = enabled
	}
}

// WithTracing enables or disables distributed tracing for requests.
func WithTracing(enabled bool) Option {
	return func(c *ClientConfig) {
		c.tracing = enabled
	}
}

// ─── Client ──────────────────────────────────────────────────────────────────

// Client wraps an opaque FFI handle to the Rust liter-llm core library.
// All HTTP, caching, budget tracking, and provider routing is handled by
// the Rust core.
//
// Construct one with [NewClient].  Client is safe for concurrent use.
// Call [Client.Close] when done to free the underlying Rust resources.
type Client struct {
	handle *C.LiterLlmClient
	closed atomic.Bool
	mu     sync.Mutex
}

// NewClient constructs a Client with the supplied options.
//
// At minimum, provide [WithAPIKey] (or set the OPENAI_API_KEY environment
// variable yourself and pass an empty key if the provider does not need it).
//
//	client, err := literllm.NewClient(
//	    literllm.WithAPIKey(os.Getenv("OPENAI_API_KEY")),
//	)
//	if err != nil {
//	    log.Fatal(err)
//	}
//	defer client.Close()
func NewClient(opts ...Option) (*Client, error) {
	cfg := ClientConfig{}
	for _, opt := range opts {
		opt(&cfg)
	}

	fCfg := ffiConfig{
		APIKey:          cfg.apiKey,
		BaseURL:         cfg.baseURL,
		Cache:           cfg.cache,
		Budget:          cfg.budget,
		CooldownSecs:    cfg.cooldownSecs,
		HealthCheckSecs: cfg.healthCheckSecs,
		CostTracking:    cfg.costTracking,
		Tracing:         cfg.tracing,
	}
	if cfg.modelHint != "" {
		fCfg.ModelHint = cfg.modelHint
	}
	if cfg.rateLimitRPM != nil && cfg.rateLimitTPM != nil {
		fCfg.RateLimit = &ffiRateLimit{RPM: *cfg.rateLimitRPM, TPM: *cfg.rateLimitTPM}
	}

	configJSON, err := json.Marshal(fCfg)
	if err != nil {
		// Fall back to simple constructor if marshal fails (should not happen).
		return newClientSimple(cfg)
	}

	cConfig := C.CString(string(configJSON))
	defer C.free(unsafe.Pointer(cConfig))

	runtime.LockOSThread()
	handle := C.literllm_client_new_with_config(cConfig)
	if handle == nil {
		// Fall back to simple constructor.
		runtime.UnlockOSThread()
		return newClientSimple(cfg)
	}
	runtime.UnlockOSThread()

	return &Client{handle: handle}, nil
}

// newClientSimple creates a client using the basic literllm_client_new API.
func newClientSimple(cfg ClientConfig) (*Client, error) {
	cKey := C.CString(cfg.apiKey)
	defer C.free(unsafe.Pointer(cKey))

	var cBase *C.char
	if cfg.baseURL != "" {
		cBase = C.CString(cfg.baseURL)
		defer C.free(unsafe.Pointer(cBase))
	}

	var cHint *C.char
	if cfg.modelHint != "" {
		cHint = C.CString(cfg.modelHint)
		defer C.free(unsafe.Pointer(cHint))
	}

	runtime.LockOSThread()
	handle := C.literllm_client_new(cKey, cBase, cHint)
	if handle == nil {
		err := lastError()
		runtime.UnlockOSThread()
		return nil, fmt.Errorf("literllm: failed to create client: %w", err)
	}
	runtime.UnlockOSThread()

	return &Client{handle: handle}, nil
}

// Close frees the underlying Rust client resources.  After Close returns,
// all other methods will return errors.  Close is idempotent.
func (c *Client) Close() {
	if c.closed.CompareAndSwap(false, true) {
		c.mu.Lock()
		defer c.mu.Unlock()
		if c.handle != nil {
			C.literllm_client_free(c.handle)
			c.handle = nil
		}
	}
}

// ─── Error helpers ───────────────────────────────────────────────────────────

// ffiLabelMapping maps the [Label] prefix produced by the Rust FFI
// format_error function to a Go sentinel error and a default HTTP status code.
var ffiLabelMapping = map[string]struct {
	sentinel   error
	statusCode int
	// displayPrefix is the thiserror Display prefix that precedes the raw
	// provider message (e.g. "server error: ").  We strip it so that
	// APIError.Message contains the provider-originated text only.
	displayPrefix string
}{
	"Authentication":     {ErrAuthentication, http.StatusUnauthorized, "authentication failed: "},
	"RateLimited":        {ErrRateLimit, http.StatusTooManyRequests, "rate limited: "},
	"BadRequest":         {ErrInvalidRequest, http.StatusBadRequest, "bad request: "},
	"NotFound":           {ErrNotFound, http.StatusNotFound, "not found: "},
	"ServerError":        {ErrProviderError, http.StatusInternalServerError, "server error: "},
	"ServiceUnavailable": {ErrProviderError, http.StatusServiceUnavailable, "service unavailable: "},
	"BudgetExceeded":     {ErrBudgetExceeded, 0, "budget exceeded: "},
	"HookRejected":       {ErrHookRejected, 0, "hook rejected: "},
}

// parseFFIError inspects the raw FFI error string for a [Label] prefix and
// returns a typed *APIError when a known label is found.  Falls back to a
// plain error otherwise.
func parseFFIError(raw string) error {
	// Find the [Label] block anywhere in the string.
	start := strings.Index(raw, "[")
	if start == -1 {
		return fmt.Errorf("literllm: %s", raw)
	}
	end := strings.Index(raw[start:], "]")
	if end == -1 {
		return fmt.Errorf("literllm: %s", raw)
	}
	label := raw[start+1 : start+end]

	mapping, ok := ffiLabelMapping[label]
	if !ok {
		return fmt.Errorf("literllm: %s", raw)
	}

	// Everything after "[Label] " is the Rust Display output of the error.
	msg := raw[start+end+1:]
	msg = strings.TrimPrefix(msg, " ")

	// Strip the thiserror Display prefix to recover the raw provider message.
	msg = strings.TrimPrefix(msg, mapping.displayPrefix)

	return &APIError{
		StatusCode: mapping.statusCode,
		Message:    msg,
		sentinel:   mapping.sentinel,
	}
}

// lastError retrieves the last error message from the FFI layer.
func lastError() error {
	errPtr := C.literllm_last_error()
	if errPtr == nil {
		return fmt.Errorf("literllm: unknown error")
	}
	return parseFFIError(C.GoString(errPtr))
}

// checkHandle returns an error if the client handle is nil or closed.
func (c *Client) checkHandle() error {
	if c.closed.Load() {
		return fmt.Errorf("%w: client is closed", ErrInvalidRequest)
	}
	if c.handle == nil {
		return fmt.Errorf("%w: client handle is nil", ErrInvalidRequest)
	}
	return nil
}

// ─── Generic JSON call helper ────────────────────────────────────────────────

// callJSON marshals a request to JSON, calls the given FFI function, and
// returns the raw JSON response bytes.
func (c *Client) callJSON(reqJSON []byte, ffiFunc func(*C.LiterLlmClient, *C.char) *C.char) ([]byte, error) {
	if err := c.checkHandle(); err != nil {
		return nil, err
	}

	cReq := C.CString(string(reqJSON))
	defer C.free(unsafe.Pointer(cReq))

	runtime.LockOSThread()
	c.mu.Lock()
	cResp := ffiFunc(c.handle, cReq)
	c.mu.Unlock()
	if cResp == nil {
		err := lastError()
		runtime.UnlockOSThread()
		return nil, err
	}
	runtime.UnlockOSThread()
	defer C.literllm_free_string(cResp)
	return []byte(C.GoString(cResp)), nil
}

// callNoBody calls an FFI function that takes only the client handle
// (no request body) and returns a JSON response.
func (c *Client) callNoBody(ffiFunc func(*C.LiterLlmClient) *C.char) ([]byte, error) {
	if err := c.checkHandle(); err != nil {
		return nil, err
	}

	runtime.LockOSThread()
	c.mu.Lock()
	cResp := ffiFunc(c.handle)
	c.mu.Unlock()
	if cResp == nil {
		err := lastError()
		runtime.UnlockOSThread()
		return nil, err
	}
	runtime.UnlockOSThread()
	defer C.literllm_free_string(cResp)
	return []byte(C.GoString(cResp)), nil
}

// callByID calls an FFI function that takes a string ID parameter and
// returns a JSON response.
func (c *Client) callByID(id string, ffiFunc func(*C.LiterLlmClient, *C.char) *C.char) ([]byte, error) {
	if err := c.checkHandle(); err != nil {
		return nil, err
	}

	cID := C.CString(id)
	defer C.free(unsafe.Pointer(cID))

	runtime.LockOSThread()
	c.mu.Lock()
	cResp := ffiFunc(c.handle, cID)
	c.mu.Unlock()
	if cResp == nil {
		err := lastError()
		runtime.UnlockOSThread()
		return nil, err
	}
	runtime.UnlockOSThread()
	defer C.literllm_free_string(cResp)
	return []byte(C.GoString(cResp)), nil
}

// ─── Chat ────────────────────────────────────────────────────────────────────

// Chat sends a non-streaming chat completion request and returns the full
// response.
func (c *Client) Chat(_ context.Context, req *ChatCompletionRequest) (*ChatCompletionResponse, error) {
	if req == nil {
		return nil, fmt.Errorf("%w: request must not be nil", ErrInvalidRequest)
	}

	reqJSON, err := json.Marshal(req)
	if err != nil {
		return nil, fmt.Errorf("literllm: marshal request: %w", err)
	}

	respJSON, err := c.callJSON(reqJSON, func(h *C.LiterLlmClient, r *C.char) *C.char {
		return C.literllm_chat(h, r)
	})
	if err != nil {
		return nil, err
	}

	var resp ChatCompletionResponse
	if err := json.Unmarshal(respJSON, &resp); err != nil {
		return nil, fmt.Errorf("literllm: unmarshal chat response: %w", err)
	}
	return &resp, nil
}

// ChatStream sends a streaming chat completion request.
//
// The handler is invoked once for each server-sent event chunk.  If handler
// returns a non-nil error the stream is aborted and that error is returned by
// ChatStream.
func (c *Client) ChatStream(_ context.Context, req *ChatCompletionRequest, handler func(*ChatCompletionChunk) error) error {
	if req == nil {
		return fmt.Errorf("%w: request must not be nil", ErrInvalidRequest)
	}
	if handler == nil {
		return fmt.Errorf("%w: handler must not be nil", ErrInvalidRequest)
	}
	if err := c.checkHandle(); err != nil {
		return err
	}

	// Force stream to true for the FFI call.
	streamTrue := true
	reqCopy := *req
	reqCopy.Stream = &streamTrue

	reqJSON, err := json.Marshal(&reqCopy)
	if err != nil {
		return fmt.Errorf("literllm: marshal request: %w", err)
	}

	cReq := C.CString(string(reqJSON))
	defer C.free(unsafe.Pointer(cReq))

	// Register the Go handler in our callback map and pass the ID as user_data.
	callbackID := registerStreamCallback(handler)

	runtime.LockOSThread()
	c.mu.Lock()
	result := C.call_chat_stream(
		c.handle,
		cReq,
		unsafe.Pointer(uintptr(callbackID)), //nolint:govet // integer smuggled through void* (not a real pointer)
	)
	c.mu.Unlock()

	// Retrieve and clean up the callback entry, capturing any handler error.
	handlerErr := unregisterStreamCallback(callbackID)

	if handlerErr != nil {
		runtime.UnlockOSThread()
		return handlerErr
	}

	if result != 0 {
		err := lastError()
		runtime.UnlockOSThread()
		return err
	}
	runtime.UnlockOSThread()
	return nil
}

// ─── Embed ───────────────────────────────────────────────────────────────────

// Embed sends an embedding request and returns the response.
func (c *Client) Embed(_ context.Context, req *EmbeddingRequest) (*EmbeddingResponse, error) {
	if req == nil {
		return nil, fmt.Errorf("%w: request must not be nil", ErrInvalidRequest)
	}

	reqJSON, err := json.Marshal(req)
	if err != nil {
		return nil, fmt.Errorf("literllm: marshal request: %w", err)
	}

	respJSON, err := c.callJSON(reqJSON, func(h *C.LiterLlmClient, r *C.char) *C.char {
		return C.literllm_embed(h, r)
	})
	if err != nil {
		return nil, err
	}

	var resp EmbeddingResponse
	if err := json.Unmarshal(respJSON, &resp); err != nil {
		return nil, fmt.Errorf("literllm: unmarshal embedding response: %w", err)
	}
	return &resp, nil
}

// ─── List Models ─────────────────────────────────────────────────────────────

// ListModels retrieves the list of models from the configured provider endpoint.
func (c *Client) ListModels(_ context.Context) (*ModelsListResponse, error) {
	respJSON, err := c.callNoBody(func(h *C.LiterLlmClient) *C.char {
		return C.literllm_list_models(h)
	})
	if err != nil {
		return nil, err
	}

	var resp ModelsListResponse
	if err := json.Unmarshal(respJSON, &resp); err != nil {
		return nil, fmt.Errorf("literllm: unmarshal models response: %w", err)
	}
	return &resp, nil
}

// ─── Image Generate ──────────────────────────────────────────────────────────

// ImageGenerate sends an image generation request and returns the response.
func (c *Client) ImageGenerate(_ context.Context, req *CreateImageRequest) (*ImagesResponse, error) {
	if req == nil {
		return nil, fmt.Errorf("%w: request must not be nil", ErrInvalidRequest)
	}

	reqJSON, err := json.Marshal(req)
	if err != nil {
		return nil, fmt.Errorf("literllm: marshal request: %w", err)
	}

	respJSON, err := c.callJSON(reqJSON, func(h *C.LiterLlmClient, r *C.char) *C.char {
		return C.literllm_image_generate(h, r)
	})
	if err != nil {
		return nil, err
	}

	var resp ImagesResponse
	if err := json.Unmarshal(respJSON, &resp); err != nil {
		return nil, fmt.Errorf("literllm: unmarshal image response: %w", err)
	}
	return &resp, nil
}

// ─── Speech ──────────────────────────────────────────────────────────────────

// Speech generates audio from text and returns raw audio bytes.
// The FFI layer returns a base64-encoded string which is decoded here.
func (c *Client) Speech(_ context.Context, req *CreateSpeechRequest) ([]byte, error) {
	if req == nil {
		return nil, fmt.Errorf("%w: request must not be nil", ErrInvalidRequest)
	}

	reqJSON, err := json.Marshal(req)
	if err != nil {
		return nil, fmt.Errorf("literllm: marshal request: %w", err)
	}

	respJSON, err := c.callJSON(reqJSON, func(h *C.LiterLlmClient, r *C.char) *C.char {
		return C.literllm_speech(h, r)
	})
	if err != nil {
		return nil, err
	}

	// The FFI returns base64-encoded audio bytes.
	data, err := base64.StdEncoding.DecodeString(string(respJSON))
	if err != nil {
		return nil, fmt.Errorf("literllm: decode speech audio: %w", err)
	}
	return data, nil
}

// ─── Transcribe ──────────────────────────────────────────────────────────────

// Transcribe sends a transcription request and returns the response.
func (c *Client) Transcribe(_ context.Context, req *CreateTranscriptionRequest) (*TranscriptionResponse, error) {
	if req == nil {
		return nil, fmt.Errorf("%w: request must not be nil", ErrInvalidRequest)
	}

	reqJSON, err := json.Marshal(req)
	if err != nil {
		return nil, fmt.Errorf("literllm: marshal request: %w", err)
	}

	respJSON, err := c.callJSON(reqJSON, func(h *C.LiterLlmClient, r *C.char) *C.char {
		return C.literllm_transcribe(h, r)
	})
	if err != nil {
		return nil, err
	}

	var resp TranscriptionResponse
	if err := json.Unmarshal(respJSON, &resp); err != nil {
		return nil, fmt.Errorf("literllm: unmarshal transcription response: %w", err)
	}
	return &resp, nil
}

// ─── Moderate ────────────────────────────────────────────────────────────────

// Moderate checks content against moderation policies.
func (c *Client) Moderate(_ context.Context, req *ModerationRequest) (*ModerationResponse, error) {
	if req == nil {
		return nil, fmt.Errorf("%w: request must not be nil", ErrInvalidRequest)
	}

	reqJSON, err := json.Marshal(req)
	if err != nil {
		return nil, fmt.Errorf("literllm: marshal request: %w", err)
	}

	respJSON, err := c.callJSON(reqJSON, func(h *C.LiterLlmClient, r *C.char) *C.char {
		return C.literllm_moderate(h, r)
	})
	if err != nil {
		return nil, err
	}

	var resp ModerationResponse
	if err := json.Unmarshal(respJSON, &resp); err != nil {
		return nil, fmt.Errorf("literllm: unmarshal moderation response: %w", err)
	}
	return &resp, nil
}

// ─── Rerank ──────────────────────────────────────────────────────────────────

// Rerank reranks documents by relevance to a query.
func (c *Client) Rerank(_ context.Context, req *RerankRequest) (*RerankResponse, error) {
	if req == nil {
		return nil, fmt.Errorf("%w: request must not be nil", ErrInvalidRequest)
	}

	reqJSON, err := json.Marshal(req)
	if err != nil {
		return nil, fmt.Errorf("literllm: marshal request: %w", err)
	}

	respJSON, err := c.callJSON(reqJSON, func(h *C.LiterLlmClient, r *C.char) *C.char {
		return C.literllm_rerank(h, r)
	})
	if err != nil {
		return nil, err
	}

	var resp RerankResponse
	if err := json.Unmarshal(respJSON, &resp); err != nil {
		return nil, fmt.Errorf("literllm: unmarshal rerank response: %w", err)
	}
	return &resp, nil
}

// ─── File Management ─────────────────────────────────────────────────────────

// CreateFile uploads a file.
func (c *Client) CreateFile(_ context.Context, req *CreateFileRequest) (*FileObject, error) {
	if req == nil {
		return nil, fmt.Errorf("%w: request must not be nil", ErrInvalidRequest)
	}

	reqJSON, err := json.Marshal(req)
	if err != nil {
		return nil, fmt.Errorf("literllm: marshal request: %w", err)
	}

	respJSON, err := c.callJSON(reqJSON, func(h *C.LiterLlmClient, r *C.char) *C.char {
		return C.literllm_create_file(h, r)
	})
	if err != nil {
		return nil, err
	}

	var resp FileObject
	if err := json.Unmarshal(respJSON, &resp); err != nil {
		return nil, fmt.Errorf("literllm: unmarshal file response: %w", err)
	}
	return &resp, nil
}

// RetrieveFile retrieves metadata for a file by ID.
func (c *Client) RetrieveFile(_ context.Context, fileID string) (*FileObject, error) {
	if fileID == "" {
		return nil, fmt.Errorf("%w: file_id is required", ErrInvalidRequest)
	}

	respJSON, err := c.callByID(fileID, func(h *C.LiterLlmClient, id *C.char) *C.char {
		return C.literllm_retrieve_file(h, id)
	})
	if err != nil {
		return nil, err
	}

	var resp FileObject
	if err := json.Unmarshal(respJSON, &resp); err != nil {
		return nil, fmt.Errorf("literllm: unmarshal file response: %w", err)
	}
	return &resp, nil
}

// DeleteFile deletes a file by ID.
func (c *Client) DeleteFile(_ context.Context, fileID string) (*DeleteResponse, error) {
	if fileID == "" {
		return nil, fmt.Errorf("%w: file_id is required", ErrInvalidRequest)
	}

	respJSON, err := c.callByID(fileID, func(h *C.LiterLlmClient, id *C.char) *C.char {
		return C.literllm_delete_file(h, id)
	})
	if err != nil {
		return nil, err
	}

	var resp DeleteResponse
	if err := json.Unmarshal(respJSON, &resp); err != nil {
		return nil, fmt.Errorf("literllm: unmarshal delete response: %w", err)
	}
	return &resp, nil
}

// ListFiles lists files, optionally filtered by query parameters.
func (c *Client) ListFiles(_ context.Context, query *FileListQuery) (*FileListResponse, error) {
	var queryJSON []byte
	var err error
	if query != nil {
		queryJSON, err = json.Marshal(query)
		if err != nil {
			return nil, fmt.Errorf("literllm: marshal query: %w", err)
		}
	}

	if err := c.checkHandle(); err != nil {
		return nil, err
	}

	var cQuery *C.char
	if queryJSON != nil {
		cQuery = C.CString(string(queryJSON))
		defer C.free(unsafe.Pointer(cQuery))
	}

	runtime.LockOSThread()
	c.mu.Lock()
	cResp := C.literllm_list_files(c.handle, cQuery)
	c.mu.Unlock()
	if cResp == nil {
		err := lastError()
		runtime.UnlockOSThread()
		return nil, err
	}
	runtime.UnlockOSThread()
	defer C.literllm_free_string(cResp)

	var resp FileListResponse
	if err := json.Unmarshal([]byte(C.GoString(cResp)), &resp); err != nil {
		return nil, fmt.Errorf("literllm: unmarshal file list response: %w", err)
	}
	return &resp, nil
}

// FileContent retrieves the raw content of a file.
// The FFI layer returns base64-encoded content which is decoded here.
func (c *Client) FileContent(_ context.Context, fileID string) ([]byte, error) {
	if fileID == "" {
		return nil, fmt.Errorf("%w: file_id is required", ErrInvalidRequest)
	}

	respB64, err := c.callByID(fileID, func(h *C.LiterLlmClient, id *C.char) *C.char {
		return C.literllm_file_content(h, id)
	})
	if err != nil {
		return nil, err
	}

	data, err := base64.StdEncoding.DecodeString(string(respB64))
	if err != nil {
		return nil, fmt.Errorf("literllm: decode file content: %w", err)
	}
	return data, nil
}

// ─── Batch Management ────────────────────────────────────────────────────────

// CreateBatch creates a new batch job.
func (c *Client) CreateBatch(_ context.Context, req *CreateBatchRequest) (*BatchObject, error) {
	if req == nil {
		return nil, fmt.Errorf("%w: request must not be nil", ErrInvalidRequest)
	}

	reqJSON, err := json.Marshal(req)
	if err != nil {
		return nil, fmt.Errorf("literllm: marshal request: %w", err)
	}

	respJSON, err := c.callJSON(reqJSON, func(h *C.LiterLlmClient, r *C.char) *C.char {
		return C.literllm_create_batch(h, r)
	})
	if err != nil {
		return nil, err
	}

	var resp BatchObject
	if err := json.Unmarshal(respJSON, &resp); err != nil {
		return nil, fmt.Errorf("literllm: unmarshal batch response: %w", err)
	}
	return &resp, nil
}

// RetrieveBatch retrieves a batch by ID.
func (c *Client) RetrieveBatch(_ context.Context, batchID string) (*BatchObject, error) {
	if batchID == "" {
		return nil, fmt.Errorf("%w: batch_id is required", ErrInvalidRequest)
	}

	respJSON, err := c.callByID(batchID, func(h *C.LiterLlmClient, id *C.char) *C.char {
		return C.literllm_retrieve_batch(h, id)
	})
	if err != nil {
		return nil, err
	}

	var resp BatchObject
	if err := json.Unmarshal(respJSON, &resp); err != nil {
		return nil, fmt.Errorf("literllm: unmarshal batch response: %w", err)
	}
	return &resp, nil
}

// ListBatches lists batches, optionally filtered by query parameters.
func (c *Client) ListBatches(_ context.Context, query *BatchListQuery) (*BatchListResponse, error) {
	var queryJSON []byte
	var err error
	if query != nil {
		queryJSON, err = json.Marshal(query)
		if err != nil {
			return nil, fmt.Errorf("literllm: marshal query: %w", err)
		}
	}

	if err := c.checkHandle(); err != nil {
		return nil, err
	}

	var cQuery *C.char
	if queryJSON != nil {
		cQuery = C.CString(string(queryJSON))
		defer C.free(unsafe.Pointer(cQuery))
	}

	runtime.LockOSThread()
	c.mu.Lock()
	cResp := C.literllm_list_batches(c.handle, cQuery)
	c.mu.Unlock()
	if cResp == nil {
		err := lastError()
		runtime.UnlockOSThread()
		return nil, err
	}
	runtime.UnlockOSThread()
	defer C.literllm_free_string(cResp)

	var resp BatchListResponse
	if err := json.Unmarshal([]byte(C.GoString(cResp)), &resp); err != nil {
		return nil, fmt.Errorf("literllm: unmarshal batch list response: %w", err)
	}
	return &resp, nil
}

// CancelBatch cancels an in-progress batch.
func (c *Client) CancelBatch(_ context.Context, batchID string) (*BatchObject, error) {
	if batchID == "" {
		return nil, fmt.Errorf("%w: batch_id is required", ErrInvalidRequest)
	}

	respJSON, err := c.callByID(batchID, func(h *C.LiterLlmClient, id *C.char) *C.char {
		return C.literllm_cancel_batch(h, id)
	})
	if err != nil {
		return nil, err
	}

	var resp BatchObject
	if err := json.Unmarshal(respJSON, &resp); err != nil {
		return nil, fmt.Errorf("literllm: unmarshal batch response: %w", err)
	}
	return &resp, nil
}

// ─── Responses API ───────────────────────────────────────────────────────────

// CreateResponse creates a new response via the Responses API.
func (c *Client) CreateResponse(_ context.Context, req *CreateResponseRequest) (*ResponseObject, error) {
	if req == nil {
		return nil, fmt.Errorf("%w: request must not be nil", ErrInvalidRequest)
	}

	reqJSON, err := json.Marshal(req)
	if err != nil {
		return nil, fmt.Errorf("literllm: marshal request: %w", err)
	}

	respJSON, err := c.callJSON(reqJSON, func(h *C.LiterLlmClient, r *C.char) *C.char {
		return C.literllm_create_response(h, r)
	})
	if err != nil {
		return nil, err
	}

	var resp ResponseObject
	if err := json.Unmarshal(respJSON, &resp); err != nil {
		return nil, fmt.Errorf("literllm: unmarshal response: %w", err)
	}
	return &resp, nil
}

// RetrieveResponse retrieves a response by ID.
func (c *Client) RetrieveResponse(_ context.Context, responseID string) (*ResponseObject, error) {
	if responseID == "" {
		return nil, fmt.Errorf("%w: response_id is required", ErrInvalidRequest)
	}

	respJSON, err := c.callByID(responseID, func(h *C.LiterLlmClient, id *C.char) *C.char {
		return C.literllm_retrieve_response(h, id)
	})
	if err != nil {
		return nil, err
	}

	var resp ResponseObject
	if err := json.Unmarshal(respJSON, &resp); err != nil {
		return nil, fmt.Errorf("literllm: unmarshal response: %w", err)
	}
	return &resp, nil
}

// CancelResponse cancels an in-progress response.
func (c *Client) CancelResponse(_ context.Context, responseID string) (*ResponseObject, error) {
	if responseID == "" {
		return nil, fmt.Errorf("%w: response_id is required", ErrInvalidRequest)
	}

	respJSON, err := c.callByID(responseID, func(h *C.LiterLlmClient, id *C.char) *C.char {
		return C.literllm_cancel_response(h, id)
	})
	if err != nil {
		return nil, err
	}

	var resp ResponseObject
	if err := json.Unmarshal(respJSON, &resp); err != nil {
		return nil, fmt.Errorf("literllm: unmarshal response: %w", err)
	}
	return &resp, nil
}

// ─── Search & OCR ────────────────────────────────────────────────────────────

// Search performs a web search using the configured provider.
func (c *Client) Search(_ context.Context, req *SearchRequest) (*SearchResponse, error) {
	if req == nil {
		return nil, fmt.Errorf("%w: request must not be nil", ErrInvalidRequest)
	}

	reqJSON, err := json.Marshal(req)
	if err != nil {
		return nil, fmt.Errorf("literllm: marshal request: %w", err)
	}

	respJSON, err := c.callJSON(reqJSON, func(h *C.LiterLlmClient, r *C.char) *C.char {
		return C.literllm_search(h, r)
	})
	if err != nil {
		return nil, err
	}

	var resp SearchResponse
	if err := json.Unmarshal(respJSON, &resp); err != nil {
		return nil, fmt.Errorf("literllm: unmarshal search response: %w", err)
	}
	return &resp, nil
}

// Ocr performs optical character recognition on images.
func (c *Client) Ocr(_ context.Context, req *OcrRequest) (*OcrResponse, error) {
	if req == nil {
		return nil, fmt.Errorf("%w: request must not be nil", ErrInvalidRequest)
	}

	reqJSON, err := json.Marshal(req)
	if err != nil {
		return nil, fmt.Errorf("literllm: marshal request: %w", err)
	}

	respJSON, err := c.callJSON(reqJSON, func(h *C.LiterLlmClient, r *C.char) *C.char {
		return C.literllm_ocr(h, r)
	})
	if err != nil {
		return nil, err
	}

	var resp OcrResponse
	if err := json.Unmarshal(respJSON, &resp); err != nil {
		return nil, fmt.Errorf("literllm: unmarshal ocr response: %w", err)
	}
	return &resp, nil
}

// ─── Budget ──────────────────────────────────────────────────────────────────

// BudgetUsed returns the cumulative global spend tracked by the Rust budget
// layer.  Returns 0.0 if no budget is configured.
func (c *Client) BudgetUsed() float64 {
	if err := c.checkHandle(); err != nil {
		return 0.0
	}
	c.mu.Lock()
	defer c.mu.Unlock()
	return float64(C.literllm_budget_usage(c.handle))
}

// ─── Provider Registration ───────────────────────────────────────────────────

// RegisterProvider registers a custom LLM provider at runtime.
// This is a global operation — the provider becomes available to all clients.
func RegisterProvider(config ProviderConfig) error {
	configJSON, err := json.Marshal(config)
	if err != nil {
		return fmt.Errorf("literllm: marshal provider config: %w", err)
	}

	cConfig := C.CString(string(configJSON))
	defer C.free(unsafe.Pointer(cConfig))

	runtime.LockOSThread()
	if C.literllm_register_provider(cConfig) != 0 {
		err := lastError()
		runtime.UnlockOSThread()
		return err
	}
	runtime.UnlockOSThread()
	return nil
}

// UnregisterProvider removes a previously registered custom provider by name.
// Returns nil if the provider was found and removed.
func UnregisterProvider(name string) error {
	cName := C.CString(name)
	defer C.free(unsafe.Pointer(cName))

	runtime.LockOSThread()
	result := C.literllm_unregister_provider(cName)
	if result == -1 {
		err := lastError()
		runtime.UnlockOSThread()
		return err
	}
	runtime.UnlockOSThread()
	return nil
}

// ─── Version ─────────────────────────────────────────────────────────────────

// Version returns the version string of the liter-llm Rust core library.
func Version() string {
	return C.GoString(C.literllm_version())
}

// compile-time assertion: *Client must implement LlmClient.
var _ LlmClient = (*Client)(nil)
