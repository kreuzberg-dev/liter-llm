// Smoke tests for the LiterLlm published .NET package.
//
// Validates the published package works against real LLM APIs.
// Requires API keys in environment variables or .env file at repo root.

using LiterLlm;

// ── .env loader ─────────────────────────────────────────────────────────────

LoadDotenv();

static void LoadDotenv()
{
    var dir = new DirectoryInfo(AppContext.BaseDirectory);
    for (var i = 0; i < 8; i++)
    {
        if (dir is null) break;
        var envFile = Path.Combine(dir.FullName, ".env");
        if (File.Exists(envFile))
        {
            foreach (var line in File.ReadAllLines(envFile))
            {
                var trimmed = line.Trim();
                if (string.IsNullOrEmpty(trimmed) || trimmed.StartsWith('#')) continue;
                var idx = trimmed.IndexOf('=');
                if (idx < 0) continue;
                var key = trimmed[..idx].Trim();
                var value = trimmed[(idx + 1)..].Trim();
                if (string.IsNullOrEmpty(Environment.GetEnvironmentVariable(key)))
                {
                    Environment.SetEnvironmentVariable(key, value);
                }
            }
            break;
        }
        dir = dir.Parent;
    }
}

static string? EnvKey(string name)
{
    var value = Environment.GetEnvironmentVariable(name);
    return string.IsNullOrEmpty(value) ? null : value;
}

// ── Test runner ─────────────────────────────────────────────────────────────

var passed = 0;
var failed = 0;
var skipped = 0;

async Task Run(string name, Func<Task<string?>> fn)
{
    Console.Write($"  {name}... ");
    try
    {
        var result = await fn();
        if (result is null)
        {
            Console.WriteLine("SKIP");
            skipped++;
        }
        else
        {
            Console.WriteLine("PASS");
            passed++;
        }
    }
    catch (Exception ex)
    {
        Console.WriteLine($"FAIL: {ex.Message}");
        failed++;
    }
}

int Summary()
{
    var total = passed + failed + skipped;
    Console.WriteLine();
    Console.WriteLine(new string('=', 60));
    Console.WriteLine($"Results: {passed} passed, {failed} failed, {skipped} skipped ({total} total)");
    return failed > 0 ? 1 : 0;
}

// ── Test cases ──────────────────────────────────────────────────────────────

async Task<string?> TestChatOpenAI()
{
    var key = EnvKey("OPENAI_API_KEY");
    if (key is null) return null;
    await using var client = new LlmClient(apiKey: key);
    var request = new ChatCompletionRequest(
        Model: "openai/gpt-4o-mini",
        Messages: [new UserMessage("Say hello in one word.")],
        MaxTokens: 10);
    var r = await client.ChatAsync(request);
    if (r.Choices.Count == 0) throw new Exception("no choices in response");
    if (string.IsNullOrEmpty(r.Choices[0].Message.Content)) throw new Exception("empty content");
    if (r.Usage is null || r.Usage.TotalTokens <= 0) throw new Exception("no usage data");
    return "ok";
}

async Task<string?> TestChatAnthropic()
{
    var key = EnvKey("ANTHROPIC_API_KEY");
    if (key is null) return null;
    await using var client = new LlmClient(apiKey: key);
    var request = new ChatCompletionRequest(
        Model: "anthropic/claude-3-5-haiku-20241022",
        Messages: [new UserMessage("Say hello in one word.")],
        MaxTokens: 10);
    var r = await client.ChatAsync(request);
    if (r.Choices.Count == 0) throw new Exception("no choices");
    if (string.IsNullOrEmpty(r.Choices[0].Message.Content)) throw new Exception("empty content");
    return "ok";
}

async Task<string?> TestChatGemini()
{
    var key = EnvKey("GEMINI_API_KEY");
    if (key is null) return null;
    await using var client = new LlmClient(apiKey: key);
    var request = new ChatCompletionRequest(
        Model: "google/gemini-2.5-flash-lite",
        Messages: [new UserMessage("Say hello in one word.")],
        MaxTokens: 10);
    var r = await client.ChatAsync(request);
    if (r.Choices.Count == 0) throw new Exception("no choices");
    if (string.IsNullOrEmpty(r.Choices[0].Message.Content)) throw new Exception("empty content");
    return "ok";
}

async Task<string?> TestStreamingOpenAI()
{
    var key = EnvKey("OPENAI_API_KEY");
    if (key is null) return null;
    await using var client = new LlmClient(apiKey: key);
    var request = new ChatCompletionRequest(
        Model: "openai/gpt-4o-mini",
        Messages: [new UserMessage("Count from 1 to 5.")],
        MaxTokens: 50);
    var chunkCount = 0;
    await foreach (var chunk in client.ChatStreamAsync(request))
    {
        chunkCount++;
    }
    if (chunkCount == 0) throw new Exception("no chunks received");
    return "ok";
}

async Task<string?> TestEmbedOpenAI()
{
    var key = EnvKey("OPENAI_API_KEY");
    if (key is null) return null;
    await using var client = new LlmClient(apiKey: key);
    var request = new EmbeddingRequest(
        Model: "openai/text-embedding-3-small",
        Input: ["Hello, world!"]);
    var r = await client.EmbedAsync(request);
    if (r.Data.Count == 0) throw new Exception("no embeddings");
    if (r.Data[0].Embedding.Count == 0) throw new Exception("empty embedding vector");
    return "ok";
}

async Task<string?> TestListModelsOpenAI()
{
    var key = EnvKey("OPENAI_API_KEY");
    if (key is null) return null;
    await using var client = new LlmClient(apiKey: key);
    var r = await client.ListModelsAsync();
    if (r.Data.Count == 0) throw new Exception("no models returned");
    return "ok";
}

async Task<string?> TestProviderRouting()
{
    var openaiKey = EnvKey("OPENAI_API_KEY");
    var anthropicKey = EnvKey("ANTHROPIC_API_KEY");
    if (openaiKey is null || anthropicKey is null) return null;

    Message[] messages = [new UserMessage("Say hi.")];

    await using var clientOpenAI = new LlmClient(apiKey: openaiKey);
    var r1 = await clientOpenAI.ChatAsync(new ChatCompletionRequest(
        Model: "openai/gpt-4o-mini", Messages: messages, MaxTokens: 5));
    if (r1.Choices.Count == 0) throw new Exception("OpenAI failed");

    await using var clientAnthropic = new LlmClient(apiKey: anthropicKey);
    var r2 = await clientAnthropic.ChatAsync(new ChatCompletionRequest(
        Model: "anthropic/claude-3-5-haiku-20241022", Messages: messages, MaxTokens: 5));
    if (r2.Choices.Count == 0) throw new Exception("Anthropic failed");
    return "ok";
}

async Task<string?> TestCacheMemory()
{
    var key = EnvKey("OPENAI_API_KEY");
    if (key is null) return null;
    await using var client = new LlmClient(
        apiKey: key,
        cacheConfig: new CacheConfig(MaxEntries: 10, TtlSeconds: 60));
    Message[] messages = [new UserMessage("What is 2+2? Answer with just the number.")];
    var req = new ChatCompletionRequest(
        Model: "openai/gpt-4o-mini", Messages: messages, MaxTokens: 5);
    var r1 = await client.ChatAsync(req);
    var r2 = await client.ChatAsync(req);
    if (r1.Choices.Count == 0) throw new Exception("first request failed");
    if (r2.Choices.Count == 0) throw new Exception("second request failed");
    if (r1.Choices[0].Message.Content != r2.Choices[0].Message.Content)
        throw new Exception("cache miss - responses differ");
    return "ok";
}

// ── Main ────────────────────────────────────────────────────────────────────

Console.WriteLine("liter-llm Smoke Tests (C#)");
Console.WriteLine(new string('=', 60));
Console.WriteLine();

Console.WriteLine("Chat Completions:");
await Run("OpenAI gpt-4o-mini", TestChatOpenAI);
await Run("Anthropic claude-3-5-haiku", TestChatAnthropic);
await Run("Google gemini-2.5-flash-lite", TestChatGemini);

await Run("OpenAI streaming", TestStreamingOpenAI);
await Run("OpenAI text-embedding-3-small", TestEmbedOpenAI);
await Run("OpenAI list models", TestListModelsOpenAI);
await Run("Multi-provider routing", TestProviderRouting);
await Run("In-memory cache hit", TestCacheMemory);

return Summary();
