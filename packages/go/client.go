package literllm

import (
	"bufio"
	"bytes"
	"context"
	"crypto/sha256"
	"encoding/hex"
	"encoding/json"
	"fmt"
	"io"
	"net/http"
	"strings"
	"sync"
	"time"
)

// ─── In-Memory LRU Cache ──────────────────────────────────────────────────────

// cacheEntry holds a cached JSON response with its creation timestamp.
type cacheEntry struct {
	response json.RawMessage
	created  time.Time
}

// lruCache is a simple thread-safe LRU cache for response data.
type lruCache struct {
	mu      sync.Mutex
	entries map[string]*cacheEntry
	order   []string // oldest first
	config  CacheConfig
}

// newLRUCache creates a new cache with the given configuration.
func newLRUCache(cfg CacheConfig) *lruCache {
	return &lruCache{
		entries: make(map[string]*cacheEntry, cfg.MaxEntries),
		order:   make([]string, 0, cfg.MaxEntries),
		config:  cfg,
	}
}

// get returns the cached response for the given key, or nil if not found or expired.
func (c *lruCache) get(key string) json.RawMessage {
	c.mu.Lock()
	defer c.mu.Unlock()

	entry, ok := c.entries[key]
	if !ok {
		return nil
	}

	ttl := time.Duration(c.config.TTLSeconds) * time.Second
	if time.Since(entry.created) > ttl {
		// Expired — remove it.
		c.removeLocked(key)
		return nil
	}

	// Move to end (most recently used).
	c.moveToEndLocked(key)
	return entry.response
}

// put stores a response in the cache, evicting the oldest entry if at capacity.
func (c *lruCache) put(key string, response json.RawMessage) {
	c.mu.Lock()
	defer c.mu.Unlock()

	// If the key already exists, update it.
	if _, ok := c.entries[key]; ok {
		c.entries[key] = &cacheEntry{response: response, created: time.Now()}
		c.moveToEndLocked(key)
		return
	}

	// Evict oldest entries if at capacity.
	for len(c.order) >= c.config.MaxEntries && len(c.order) > 0 {
		oldest := c.order[0]
		c.removeLocked(oldest)
	}

	c.entries[key] = &cacheEntry{response: response, created: time.Now()}
	c.order = append(c.order, key)
}

func (c *lruCache) removeLocked(key string) {
	delete(c.entries, key)
	for i, k := range c.order {
		if k == key {
			c.order = append(c.order[:i], c.order[i+1:]...)
			break
		}
	}
}

func (c *lruCache) moveToEndLocked(key string) {
	for i, k := range c.order {
		if k == key {
			c.order = append(c.order[:i], c.order[i+1:]...)
			c.order = append(c.order, key)
			break
		}
	}
}

// cacheKey hashes the request JSON to produce a deterministic cache key.
func cacheKey(requestJSON []byte) string {
	h := sha256.Sum256(requestJSON)
	return hex.EncodeToString(h[:])
}

// ─── Budget State ─────────────────────────────────────────────────────────────

// budgetState tracks cumulative spend for budget enforcement.
type budgetState struct {
	mu          sync.Mutex
	globalSpend float64
	modelSpend  map[string]float64
}

// newBudgetState creates a new budget tracker.
func newBudgetState() *budgetState {
	return &budgetState{
		modelSpend: make(map[string]float64),
	}
}

// checkBudget validates that the given model has not exceeded its budget.
// Returns ErrBudgetExceeded if the budget is exceeded in strict mode.
func (b *budgetState) checkBudget(model string, cfg *BudgetConfig) error {
	if cfg == nil {
		return nil
	}
	if cfg.Enforcement != "strict" {
		return nil
	}

	b.mu.Lock()
	defer b.mu.Unlock()

	if cfg.GlobalLimit != nil && b.globalSpend >= *cfg.GlobalLimit {
		return fmt.Errorf("%w: global spend %.6f >= limit %.6f", ErrBudgetExceeded, b.globalSpend, *cfg.GlobalLimit)
	}

	if limit, ok := cfg.ModelLimits[model]; ok {
		if b.modelSpend[model] >= limit {
			return fmt.Errorf("%w: model %q spend %.6f >= limit %.6f", ErrBudgetExceeded, model, b.modelSpend[model], limit)
		}
	}

	return nil
}

// recordCost adds the cost of the given usage to the budget tracker.
func (b *budgetState) recordCost(model string, usage *Usage) {
	if usage == nil {
		return
	}
	cost := estimateCost(model, usage)
	if cost <= 0 {
		return
	}

	b.mu.Lock()
	defer b.mu.Unlock()
	b.globalSpend += cost
	b.modelSpend[model] += cost
}

// modelPricing holds per-million-token pricing for known models.
type modelPricing struct {
	promptPerMillion     float64
	completionPerMillion float64
}

// defaultPricing provides approximate pricing for common models.
// Prices are in USD per million tokens.
var defaultPricing = map[string]modelPricing{
	"gpt-4o":         {promptPerMillion: 2.50, completionPerMillion: 10.00},
	"gpt-4o-mini":    {promptPerMillion: 0.15, completionPerMillion: 0.60},
	"gpt-4-turbo":    {promptPerMillion: 10.00, completionPerMillion: 30.00},
	"gpt-4":          {promptPerMillion: 30.00, completionPerMillion: 60.00},
	"gpt-3.5-turbo":  {promptPerMillion: 0.50, completionPerMillion: 1.50},
	"claude-3-opus":  {promptPerMillion: 15.00, completionPerMillion: 75.00},
	"claude-3-sonnet": {promptPerMillion: 3.00, completionPerMillion: 15.00},
	"claude-3-haiku": {promptPerMillion: 0.25, completionPerMillion: 1.25},
}

// estimateCost calculates the estimated cost for the given model and usage.
func estimateCost(model string, usage *Usage) float64 {
	if usage == nil {
		return 0
	}

	// Try exact match first, then prefix match.
	pricing, ok := defaultPricing[model]
	if !ok {
		// Strip provider prefix (e.g. "openai/gpt-4o" -> "gpt-4o").
		if idx := strings.Index(model, "/"); idx >= 0 {
			pricing, ok = defaultPricing[model[idx+1:]]
		}
	}
	if !ok {
		// Try prefix matching for versioned models.
		for prefix, p := range defaultPricing {
			if strings.HasPrefix(model, prefix) || strings.HasPrefix(stripProvider(model), prefix) {
				pricing = p
				ok = true
				break
			}
		}
	}
	if !ok {
		// Fallback: use a generic pricing as an approximation.
		pricing = modelPricing{promptPerMillion: 1.00, completionPerMillion: 2.00}
	}

	promptCost := float64(usage.PromptTokens) * pricing.promptPerMillion / 1_000_000
	completionCost := float64(usage.CompletionTokens) * pricing.completionPerMillion / 1_000_000
	return promptCost + completionCost
}

// stripProvider removes the provider prefix from a model name.
func stripProvider(model string) string {
	if idx := strings.Index(model, "/"); idx >= 0 {
		return model[idx+1:]
	}
	return model
}

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
	cache     *lruCache
	budget    *budgetState
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
	c := &Client{config: cfg}
	if cfg.cache != nil {
		c.cache = newLRUCache(*cfg.cache)
	}
	if cfg.budget != nil {
		c.budget = newBudgetState()
	}
	return c
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

	// Budget check before request.
	if c.budget != nil {
		if err := c.budget.checkBudget(req.Model, c.config.budget); err != nil {
			return nil, err
		}
	}

	// Run pre-request hooks.
	if err := c.runOnRequest(ctx, req); err != nil {
		return nil, err
	}

	// Ensure stream is off for this path.
	streamFalse := false
	reqCopy := *req
	reqCopy.Stream = &streamFalse

	bodyBytes, err := json.Marshal(&reqCopy)
	if err != nil {
		marshalErr := fmt.Errorf("literllm: marshal request body: %w", err)
		c.runOnError(ctx, req, marshalErr)
		return nil, marshalErr
	}

	// Check cache before HTTP call.
	if c.cache != nil {
		key := cacheKey(bodyBytes)
		if cached := c.cache.get(key); cached != nil {
			var result ChatCompletionResponse
			if err := json.Unmarshal(cached, &result); err == nil {
				c.runOnResponse(ctx, req, &result)
				return &result, nil
			}
		}
	}

	body := bytes.NewReader(bodyBytes)
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

	respBody, err := io.ReadAll(resp.Body)
	if err != nil {
		readErr := fmt.Errorf("literllm: read chat response: %w", err)
		c.runOnError(ctx, req, readErr)
		return nil, readErr
	}

	var result ChatCompletionResponse
	if err := json.Unmarshal(respBody, &result); err != nil {
		decodeErr := fmt.Errorf("literllm: decode chat response: %w", err)
		c.runOnError(ctx, req, decodeErr)
		return nil, decodeErr
	}

	// Store in cache after successful HTTP call.
	if c.cache != nil {
		c.cache.put(cacheKey(bodyBytes), json.RawMessage(respBody))
	}

	// Record cost for budget tracking.
	if c.budget != nil {
		c.budget.recordCost(req.Model, result.Usage)
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

	// Budget check before request (stream calls bypass cache).
	if c.budget != nil {
		if err := c.budget.checkBudget(req.Model, c.config.budget); err != nil {
			return err
		}
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

	// Budget check before request.
	if c.budget != nil {
		if err := c.budget.checkBudget(req.Model, c.config.budget); err != nil {
			return nil, err
		}
	}

	if err := c.runOnRequest(ctx, req); err != nil {
		return nil, err
	}

	bodyBytes, err := json.Marshal(req)
	if err != nil {
		marshalErr := fmt.Errorf("literllm: marshal request body: %w", err)
		c.runOnError(ctx, req, marshalErr)
		return nil, marshalErr
	}

	// Check cache before HTTP call.
	if c.cache != nil {
		key := cacheKey(bodyBytes)
		if cached := c.cache.get(key); cached != nil {
			var result EmbeddingResponse
			if err := json.Unmarshal(cached, &result); err == nil {
				c.runOnResponse(ctx, req, &result)
				return &result, nil
			}
		}
	}

	body := bytes.NewReader(bodyBytes)
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

	respBody, err := io.ReadAll(resp.Body)
	if err != nil {
		readErr := fmt.Errorf("literllm: read embedding response: %w", err)
		c.runOnError(ctx, req, readErr)
		return nil, readErr
	}

	var result EmbeddingResponse
	if err := json.Unmarshal(respBody, &result); err != nil {
		decodeErr := fmt.Errorf("literllm: decode embedding response: %w", err)
		c.runOnError(ctx, req, decodeErr)
		return nil, decodeErr
	}

	// Store in cache after successful HTTP call.
	if c.cache != nil {
		c.cache.put(cacheKey(bodyBytes), json.RawMessage(respBody))
	}

	// Record cost for budget tracking.
	if c.budget != nil {
		c.budget.recordCost(req.Model, &result.Usage)
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

	// Budget check before request.
	if c.budget != nil && req.Model != nil {
		if err := c.budget.checkBudget(*req.Model, c.config.budget); err != nil {
			return nil, err
		}
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

	// Budget check before request.
	if c.budget != nil {
		if err := c.budget.checkBudget(req.Model, c.config.budget); err != nil {
			return nil, err
		}
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

	// Budget check before request.
	if c.budget != nil {
		if err := c.budget.checkBudget(req.Model, c.config.budget); err != nil {
			return nil, err
		}
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

	// Budget check before request.
	if c.budget != nil && req.Model != nil {
		if err := c.budget.checkBudget(*req.Model, c.config.budget); err != nil {
			return nil, err
		}
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

	// Budget check before request.
	if c.budget != nil {
		if err := c.budget.checkBudget(req.Model, c.config.budget); err != nil {
			return nil, err
		}
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

	// Budget check before request.
	if c.budget != nil {
		if err := c.budget.checkBudget(req.Model, c.config.budget); err != nil {
			return nil, err
		}
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
