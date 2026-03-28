// Smoke tests for the liter-llm Go package.
//
// Validates the published package works against real LLM APIs.
// Requires API keys in environment variables or .env file at repo root.
package main

import (
	"bufio"
	"context"
	"fmt"
	"os"
	"path/filepath"
	"runtime"
	"strings"

	literllm "github.com/kreuzberg-dev/liter-llm/packages/go"
)

// ─── .env loader ────────────────────────────────────────────────────────────

func loadDotenv() {
	_, file, _, _ := runtime.Caller(0)
	dir := filepath.Dir(file)
	for i := 0; i < 5; i++ {
		dir = filepath.Dir(dir)
		envPath := filepath.Join(dir, ".env")
		f, err := os.Open(envPath)
		if err != nil {
			continue
		}
		scanner := bufio.NewScanner(f)
		for scanner.Scan() {
			line := strings.TrimSpace(scanner.Text())
			if line == "" || strings.HasPrefix(line, "#") {
				continue
			}
			idx := strings.Index(line, "=")
			if idx < 0 {
				continue
			}
			key := strings.TrimSpace(line[:idx])
			value := strings.TrimSpace(line[idx+1:])
			if _, exists := os.LookupEnv(key); !exists {
				os.Setenv(key, value)
			}
		}
		f.Close()
		break
	}
}

func envKey(name string) string {
	return os.Getenv(name)
}

// ─── Test runner ────────────────────────────────────────────────────────────

type smokeTest struct {
	passed  int
	failed  int
	skipped int
}

func (s *smokeTest) run(name string, fn func() (string, error)) {
	fmt.Printf("  %s... ", name)
	result, err := fn()
	if err != nil {
		fmt.Printf("FAIL: %v\n", err)
		s.failed++
	} else if result == "" {
		fmt.Println("SKIP")
		s.skipped++
	} else {
		fmt.Println("PASS")
		s.passed++
	}
}

func (s *smokeTest) summary() int {
	total := s.passed + s.failed + s.skipped
	fmt.Println()
	fmt.Println(strings.Repeat("=", 60))
	fmt.Printf("Results: %d passed, %d failed, %d skipped (%d total)\n",
		s.passed, s.failed, s.skipped, total)
	if s.failed > 0 {
		return 1
	}
	return 0
}

// ─── Test cases ─────────────────────────────────────────────────────────────

func testChatOpenAI() (string, error) {
	key := envKey("OPENAI_API_KEY")
	if key == "" {
		return "", nil
	}
	client, err := literllm.NewClient(literllm.WithAPIKey(key))
	if err != nil {
		return "", err
	}
	defer client.Close()

	resp, err := client.Chat(context.Background(), &literllm.ChatCompletionRequest{
		Model:    "openai/gpt-4o-mini",
		Messages: []literllm.Message{{Role: literllm.RoleUser, Content: "Say hello in one word."}},
		MaxTokens: intPtr(10),
	})
	if err != nil {
		return "", err
	}
	if len(resp.Choices) == 0 {
		return "", fmt.Errorf("no choices in response")
	}
	if resp.Choices[0].Message.Content == "" {
		return "", fmt.Errorf("empty content")
	}
	if resp.Usage == nil || resp.Usage.TotalTokens <= 0 {
		return "", fmt.Errorf("no usage data")
	}
	return "ok", nil
}

func testChatAnthropic() (string, error) {
	key := envKey("ANTHROPIC_API_KEY")
	if key == "" {
		return "", nil
	}
	client, err := literllm.NewClient(literllm.WithAPIKey(key))
	if err != nil {
		return "", err
	}
	defer client.Close()

	resp, err := client.Chat(context.Background(), &literllm.ChatCompletionRequest{
		Model:    "anthropic/claude-sonnet-4-20250514",
		Messages: []literllm.Message{{Role: literllm.RoleUser, Content: "Say hello in one word."}},
		MaxTokens: intPtr(10),
	})
	if err != nil {
		return "", err
	}
	if len(resp.Choices) == 0 {
		return "", fmt.Errorf("no choices")
	}
	if resp.Choices[0].Message.Content == "" {
		return "", fmt.Errorf("empty content")
	}
	return "ok", nil
}

func testChatGemini() (string, error) {
	key := envKey("GEMINI_API_KEY")
	if key == "" {
		return "", nil
	}
	client, err := literllm.NewClient(literllm.WithAPIKey(key))
	if err != nil {
		return "", err
	}
	defer client.Close()

	resp, err := client.Chat(context.Background(), &literllm.ChatCompletionRequest{
		Model:    "gemini/gemini-2.5-flash-lite",
		Messages: []literllm.Message{{Role: literllm.RoleUser, Content: "Say hello in one word."}},
		MaxTokens: intPtr(10),
	})
	if err != nil {
		return "", err
	}
	if len(resp.Choices) == 0 {
		return "", fmt.Errorf("no choices")
	}
	if resp.Choices[0].Message.Content == "" {
		return "", fmt.Errorf("empty content")
	}
	return "ok", nil
}

func testStreamingOpenAI() (string, error) {
	key := envKey("OPENAI_API_KEY")
	if key == "" {
		return "", nil
	}
	client, err := literllm.NewClient(literllm.WithAPIKey(key))
	if err != nil {
		return "", err
	}
	defer client.Close()

	var chunkCount int
	err = client.ChatStream(context.Background(), &literllm.ChatCompletionRequest{
		Model:    "openai/gpt-4o-mini",
		Messages: []literllm.Message{{Role: literllm.RoleUser, Content: "Count from 1 to 5."}},
		MaxTokens: intPtr(50),
	}, func(chunk *literllm.ChatCompletionChunk) error {
		chunkCount++
		return nil
	})
	if err != nil {
		return "", err
	}
	if chunkCount == 0 {
		return "", fmt.Errorf("no chunks received")
	}
	return "ok", nil
}

func testEmbedOpenAI() (string, error) {
	key := envKey("OPENAI_API_KEY")
	if key == "" {
		return "", nil
	}
	client, err := literllm.NewClient(literllm.WithAPIKey(key))
	if err != nil {
		return "", err
	}
	defer client.Close()

	resp, err := client.Embed(context.Background(), &literllm.EmbeddingRequest{
		Model: "openai/text-embedding-3-small",
		Input: []string{"Hello, world!"},
	})
	if err != nil {
		return "", err
	}
	if len(resp.Data) == 0 {
		return "", fmt.Errorf("no embeddings")
	}
	if len(resp.Data[0].Embedding) == 0 {
		return "", fmt.Errorf("empty embedding vector")
	}
	return "ok", nil
}

func testListModelsOpenAI() (string, error) {
	key := envKey("OPENAI_API_KEY")
	if key == "" {
		return "", nil
	}
	client, err := literllm.NewClient(literllm.WithAPIKey(key))
	if err != nil {
		return "", err
	}
	defer client.Close()

	resp, err := client.ListModels(context.Background())
	if err != nil {
		return "", err
	}
	if len(resp.Data) == 0 {
		return "", fmt.Errorf("no models returned")
	}
	return "ok", nil
}

func testProviderRouting() (string, error) {
	openaiKey := envKey("OPENAI_API_KEY")
	anthropicKey := envKey("ANTHROPIC_API_KEY")
	if openaiKey == "" || anthropicKey == "" {
		return "", nil
	}

	messages := []literllm.Message{{Role: literllm.RoleUser, Content: "Say hi."}}

	clientOpenAI, err := literllm.NewClient(literllm.WithAPIKey(openaiKey))
	if err != nil {
		return "", err
	}
	defer clientOpenAI.Close()

	r1, err := clientOpenAI.Chat(context.Background(), &literllm.ChatCompletionRequest{
		Model:    "openai/gpt-4o-mini",
		Messages: messages,
		MaxTokens: intPtr(5),
	})
	if err != nil {
		return "", err
	}
	if len(r1.Choices) == 0 {
		return "", fmt.Errorf("OpenAI failed")
	}

	clientAnthropic, err := literllm.NewClient(literllm.WithAPIKey(anthropicKey))
	if err != nil {
		return "", err
	}
	defer clientAnthropic.Close()

	r2, err := clientAnthropic.Chat(context.Background(), &literllm.ChatCompletionRequest{
		Model:    "anthropic/claude-sonnet-4-20250514",
		Messages: messages,
		MaxTokens: intPtr(5),
	})
	if err != nil {
		return "", err
	}
	if len(r2.Choices) == 0 {
		return "", fmt.Errorf("Anthropic failed")
	}
	return "ok", nil
}

func testCacheMemory() (string, error) {
	key := envKey("OPENAI_API_KEY")
	if key == "" {
		return "", nil
	}
	client, err := literllm.NewClient(
		literllm.WithAPIKey(key),
		literllm.WithCache(literllm.CacheConfig{MaxEntries: 10, TTLSeconds: 60}),
	)
	if err != nil {
		return "", err
	}
	defer client.Close()

	messages := []literllm.Message{{Role: literllm.RoleUser, Content: "What is 2+2? Answer with just the number."}}

	r1, err := client.Chat(context.Background(), &literllm.ChatCompletionRequest{
		Model:    "openai/gpt-4o-mini",
		Messages: messages,
		MaxTokens: intPtr(5),
	})
	if err != nil {
		return "", err
	}
	r2, err := client.Chat(context.Background(), &literllm.ChatCompletionRequest{
		Model:    "openai/gpt-4o-mini",
		Messages: messages,
		MaxTokens: intPtr(5),
	})
	if err != nil {
		return "", err
	}
	if len(r1.Choices) == 0 {
		return "", fmt.Errorf("first request failed")
	}
	if len(r2.Choices) == 0 {
		return "", fmt.Errorf("second request failed")
	}
	if r1.Choices[0].Message.Content != r2.Choices[0].Message.Content {
		return "", fmt.Errorf("cache miss - responses differ")
	}
	return "ok", nil
}

// ─── Helpers ────────────────────────────────────────────────────────────────

func intPtr(n int) *int64 {
	v := int64(n)
	return &v
}

// ─── Main ───────────────────────────────────────────────────────────────────

func main() {
	loadDotenv()

	fmt.Println("liter-llm Smoke Tests (Go)")
	fmt.Println(strings.Repeat("=", 60))
	fmt.Println()

	suite := &smokeTest{}

	fmt.Println("Chat Completions:")
	suite.run("OpenAI gpt-4o-mini", testChatOpenAI)
	suite.run("Anthropic claude-3-5-haiku", testChatAnthropic)
	suite.run("Google gemini-2.5-flash-lite", testChatGemini)

	suite.run("OpenAI streaming", testStreamingOpenAI)
	suite.run("OpenAI text-embedding-3-small", testEmbedOpenAI)
	suite.run("OpenAI list models", testListModelsOpenAI)
	suite.run("Multi-provider routing", testProviderRouting)
	suite.run("In-memory cache hit", testCacheMemory)

	os.Exit(suite.summary())
}
