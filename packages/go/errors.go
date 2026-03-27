package literllm

import (
	"errors"
	"fmt"
	"net/http"
)

// ─── Sentinel errors ──────────────────────────────────────────────────────────

// ErrInvalidRequest is returned when the caller supplies a malformed request
// (e.g. missing required fields, invalid JSON).
var ErrInvalidRequest = errors.New("literllm: invalid request")

// ErrAuthentication is returned when the provider rejects the API key.
var ErrAuthentication = errors.New("literllm: authentication failed")

// ErrRateLimit is returned when the provider rate-limits the request.
var ErrRateLimit = errors.New("literllm: rate limit exceeded")

// ErrNotFound is returned when the requested model or resource does not exist.
var ErrNotFound = errors.New("literllm: not found")

// ErrProviderError is returned for provider-side 5xx errors.
var ErrProviderError = errors.New("literllm: provider error")

// ErrStream is returned when a streaming response cannot be parsed.
var ErrStream = errors.New("literllm: stream error")

// ErrBudgetExceeded is returned when a request would exceed the configured budget.
var ErrBudgetExceeded = errors.New("literllm: budget exceeded")

// ErrHookRejected is returned when a hook's OnRequest method rejects the request.
var ErrHookRejected = errors.New("literllm: hook rejected request")

// ─── Typed error ──────────────────────────────────────────────────────────────

// APIError represents an HTTP error response from an LLM provider.
// It wraps one of the sentinel errors above so callers can use errors.Is.
type APIError struct {
	// StatusCode is the HTTP status code returned by the provider.
	StatusCode int
	// Message is the human-readable error message from the provider response
	// body, falling back to the HTTP status text when no body is present.
	Message string
	// sentinel is the package-level sentinel that this error wraps.
	sentinel error
}

// Error implements the error interface.
func (e *APIError) Error() string {
	return fmt.Sprintf("literllm: API error %d: %s", e.StatusCode, e.Message)
}

// Unwrap makes errors.Is and errors.As traverse the sentinel chain.
func (e *APIError) Unwrap() error {
	return e.sentinel
}

// newAPIError constructs an *APIError and selects the appropriate sentinel
// based on the HTTP status code.
func newAPIError(statusCode int, message string) *APIError {
	sentinel := classifySentinel(statusCode)
	if message == "" {
		message = http.StatusText(statusCode)
	}
	return &APIError{StatusCode: statusCode, Message: message, sentinel: sentinel}
}

// classifySentinel maps HTTP status codes to sentinel errors.
func classifySentinel(statusCode int) error {
	switch statusCode {
	case http.StatusUnauthorized, http.StatusForbidden:
		return ErrAuthentication
	case http.StatusTooManyRequests:
		return ErrRateLimit
	case http.StatusNotFound:
		return ErrNotFound
	case http.StatusBadRequest, http.StatusUnprocessableEntity:
		return ErrInvalidRequest
	default:
		if statusCode >= 500 {
			return ErrProviderError
		}
		return ErrInvalidRequest
	}
}

// StreamError wraps a streaming parse error, implementing errors.Is against
// ErrStream.
type StreamError struct {
	Message string
	Cause   error
}

// Error implements the error interface.
func (e *StreamError) Error() string {
	if e.Cause != nil {
		return fmt.Sprintf("literllm: stream error: %s: %v", e.Message, e.Cause)
	}
	return fmt.Sprintf("literllm: stream error: %s", e.Message)
}

// Unwrap enables errors.Is chaining through ErrStream and the underlying cause.
func (e *StreamError) Unwrap() []error {
	errs := []error{ErrStream}
	if e.Cause != nil {
		errs = append(errs, e.Cause)
	}
	return errs
}

// newStreamError constructs a *StreamError.
func newStreamError(message string, cause error) *StreamError {
	return &StreamError{Message: message, Cause: cause}
}
