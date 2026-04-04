package literllm

import "context"

// Role represents the role of a message sender.
type Role string

const (
	RoleSystem    Role = "system"
	RoleUser      Role = "user"
	RoleAssistant Role = "assistant"
	RoleTool      Role = "tool"
	RoleDeveloper Role = "developer"
	RoleFunction  Role = "function"
)

// ProviderConfig defines a custom LLM provider for registration.
type ProviderConfig struct {
	Name          string   `json:"name"`
	BaseURL       string   `json:"base_url"`
	AuthHeader    string   `json:"auth_header"`
	ModelPrefixes []string `json:"model_prefixes"`
}

// Hook is the interface for intercepting client lifecycle events.
type Hook interface {
	OnRequest(ctx context.Context, req interface{}) error
	OnResponse(ctx context.Context, req, resp interface{})
	OnError(ctx context.Context, req interface{}, err error)
}
