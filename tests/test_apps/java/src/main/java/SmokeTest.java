import dev.kreuzberg.literllm.LlmClient;
import dev.kreuzberg.literllm.Types;
import dev.kreuzberg.literllm.Types.ChatCompletionRequest;
import dev.kreuzberg.literllm.Types.ChatCompletionResponse;
import dev.kreuzberg.literllm.Types.EmbeddingRequest;
import dev.kreuzberg.literllm.Types.EmbeddingResponse;
import dev.kreuzberg.literllm.Types.ModelsListResponse;
import dev.kreuzberg.literllm.Types.ChatCompletionChunk;

import java.io.BufferedReader;
import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Path;
import java.util.List;
import java.util.concurrent.atomic.AtomicInteger;

/**
 * Smoke tests for the liter-llm published Java package.
 *
 * Validates the published package works against real LLM APIs.
 * Requires API keys in environment variables or .env file at repo root.
 */
public final class SmokeTest {

    private int passed;
    private int failed;
    private int skipped;

    // ── .env loader ─────────────────────────────────────────────────────

    private static void loadDotenv() {
        Path dir = Path.of(System.getProperty("user.dir"));
        for (int i = 0; i < 5; i++) {
            Path envFile = dir.resolve(".env");
            if (Files.exists(envFile)) {
                try (BufferedReader reader = Files.newBufferedReader(envFile)) {
                    String line;
                    while ((line = reader.readLine()) != null) {
                        line = line.trim();
                        if (line.isEmpty() || line.startsWith("#")) continue;
                        int idx = line.indexOf('=');
                        if (idx < 0) continue;
                        String key = line.substring(0, idx).trim();
                        String value = line.substring(idx + 1).trim();
                        if (System.getenv(key) == null) {
                            // Cannot set env vars in Java; use system properties as fallback
                            System.setProperty("env." + key, value);
                        }
                    }
                } catch (IOException e) {
                    // Ignore
                }
                break;
            }
            dir = dir.getParent();
            if (dir == null) break;
        }
    }

    private static String envKey(String name) {
        String value = System.getenv(name);
        if (value != null && !value.isEmpty()) return value;
        value = System.getProperty("env." + name);
        return (value != null && !value.isEmpty()) ? value : null;
    }

    // ── Test runner ─────────────────────────────────────────────────────

    @FunctionalInterface
    interface TestFn {
        String run() throws Exception;
    }

    private void run(String name, TestFn fn) {
        System.out.print("  " + name + "... ");
        System.out.flush();
        try {
            String result = fn.run();
            if (result == null) {
                System.out.println("SKIP");
                skipped++;
            } else {
                System.out.println("PASS");
                passed++;
            }
        } catch (Exception e) {
            System.out.println("FAIL: " + e.getMessage());
            failed++;
        }
    }

    private int summary() {
        int total = passed + failed + skipped;
        System.out.println();
        System.out.println("=".repeat(60));
        System.out.printf("Results: %d passed, %d failed, %d skipped (%d total)%n",
                passed, failed, skipped, total);
        return failed > 0 ? 1 : 0;
    }

    // ── Test cases ──────────────────────────────────────────────────────

    private static String testChatOpenAI() throws Exception {
        String key = envKey("OPENAI_API_KEY");
        if (key == null) return null;
        try (var client = LlmClient.builder().apiKey(key).build()) {
            var resp = client.chat(ChatCompletionRequest.builder(
                    "openai/gpt-4o-mini",
                    List.of(new Types.UserMessage("Say hello in one word."))
            ).maxTokens(10L).build());
            if (resp.choices().isEmpty()) throw new AssertionError("no choices in response");
            if (resp.choices().getFirst().message().content() == null)
                throw new AssertionError("empty content");
            if (resp.usage() == null || resp.usage().totalTokens() <= 0)
                throw new AssertionError("no usage data");
            return "ok";
        }
    }

    private static String testChatAnthropic() throws Exception {
        String key = envKey("ANTHROPIC_API_KEY");
        if (key == null) return null;
        try (var client = LlmClient.builder().apiKey(key).build()) {
            var resp = client.chat(ChatCompletionRequest.builder(
                    "anthropic/claude-3-5-haiku-20241022",
                    List.of(new Types.UserMessage("Say hello in one word."))
            ).maxTokens(10L).build());
            if (resp.choices().isEmpty()) throw new AssertionError("no choices");
            if (resp.choices().getFirst().message().content() == null)
                throw new AssertionError("empty content");
            return "ok";
        }
    }

    private static String testChatGemini() throws Exception {
        String key = envKey("GEMINI_API_KEY");
        if (key == null) return null;
        try (var client = LlmClient.builder().apiKey(key).build()) {
            var resp = client.chat(ChatCompletionRequest.builder(
                    "google/gemini-2.5-flash-lite",
                    List.of(new Types.UserMessage("Say hello in one word."))
            ).maxTokens(10L).build());
            if (resp.choices().isEmpty()) throw new AssertionError("no choices");
            if (resp.choices().getFirst().message().content() == null)
                throw new AssertionError("empty content");
            return "ok";
        }
    }

    private static String testStreamingOpenAI() throws Exception {
        String key = envKey("OPENAI_API_KEY");
        if (key == null) return null;
        try (var client = LlmClient.builder().apiKey(key).build()) {
            AtomicInteger chunkCount = new AtomicInteger(0);
            client.chatStream(
                    ChatCompletionRequest.builder(
                            "openai/gpt-4o-mini",
                            List.of(new Types.UserMessage("Count from 1 to 5."))
                    ).maxTokens(50L).build(),
                    chunk -> chunkCount.incrementAndGet()
            );
            if (chunkCount.get() == 0) throw new AssertionError("no chunks received");
            return "ok";
        }
    }

    private static String testEmbedOpenAI() throws Exception {
        String key = envKey("OPENAI_API_KEY");
        if (key == null) return null;
        try (var client = LlmClient.builder().apiKey(key).build()) {
            var resp = client.embed(EmbeddingRequest.builder(
                    "openai/text-embedding-3-small",
                    List.of("Hello, world!")
            ).build());
            if (resp.data().isEmpty()) throw new AssertionError("no embeddings");
            if (resp.data().getFirst().embedding().isEmpty())
                throw new AssertionError("empty embedding vector");
            return "ok";
        }
    }

    private static String testListModelsOpenAI() throws Exception {
        String key = envKey("OPENAI_API_KEY");
        if (key == null) return null;
        try (var client = LlmClient.builder().apiKey(key).build()) {
            var resp = client.listModels();
            if (resp.data().isEmpty()) throw new AssertionError("no models returned");
            return "ok";
        }
    }

    private static String testProviderRouting() throws Exception {
        String openaiKey = envKey("OPENAI_API_KEY");
        String anthropicKey = envKey("ANTHROPIC_API_KEY");
        if (openaiKey == null || anthropicKey == null) return null;

        var messages = List.of(new Types.UserMessage("Say hi."));

        try (var clientOpenAI = LlmClient.builder().apiKey(openaiKey).build()) {
            var r1 = clientOpenAI.chat(ChatCompletionRequest.builder(
                    "openai/gpt-4o-mini", messages).maxTokens(5L).build());
            if (r1.choices().isEmpty()) throw new AssertionError("OpenAI failed");
        }

        try (var clientAnthropic = LlmClient.builder().apiKey(anthropicKey).build()) {
            var r2 = clientAnthropic.chat(ChatCompletionRequest.builder(
                    "anthropic/claude-3-5-haiku-20241022", messages).maxTokens(5L).build());
            if (r2.choices().isEmpty()) throw new AssertionError("Anthropic failed");
        }
        return "ok";
    }

    private static String testCacheMemory() throws Exception {
        String key = envKey("OPENAI_API_KEY");
        if (key == null) return null;
        try (var client = LlmClient.builder()
                .apiKey(key)
                .cacheConfig(new dev.kreuzberg.literllm.CacheConfig(10, 60))
                .build()) {
            var messages = List.of(
                    new Types.UserMessage("What is 2+2? Answer with just the number."));
            var req = ChatCompletionRequest.builder(
                    "openai/gpt-4o-mini", messages).maxTokens(5L).build();
            var r1 = client.chat(req);
            var r2 = client.chat(req);
            if (r1.choices().isEmpty()) throw new AssertionError("first request failed");
            if (r2.choices().isEmpty()) throw new AssertionError("second request failed");
            if (!java.util.Objects.equals(
                    r1.choices().getFirst().message().content(),
                    r2.choices().getFirst().message().content())) {
                throw new AssertionError("cache miss - responses differ");
            }
            return "ok";
        }
    }

    // ── Main ────────────────────────────────────────────────────────────

    public static void main(String[] args) {
        loadDotenv();

        System.out.println("liter-llm Smoke Tests (Java)");
        System.out.println("=".repeat(60));
        System.out.println();

        var suite = new SmokeTest();

        System.out.println("Chat Completions:");
        suite.run("OpenAI gpt-4o-mini", SmokeTest::testChatOpenAI);
        suite.run("Anthropic claude-3-5-haiku", SmokeTest::testChatAnthropic);
        suite.run("Google gemini-2.5-flash-lite", SmokeTest::testChatGemini);

        suite.run("OpenAI streaming", SmokeTest::testStreamingOpenAI);
        suite.run("OpenAI text-embedding-3-small", SmokeTest::testEmbedOpenAI);
        suite.run("OpenAI list models", SmokeTest::testListModelsOpenAI);
        suite.run("Multi-provider routing", SmokeTest::testProviderRouting);
        suite.run("In-memory cache hit", SmokeTest::testCacheMemory);

        System.exit(suite.summary());
    }
}
