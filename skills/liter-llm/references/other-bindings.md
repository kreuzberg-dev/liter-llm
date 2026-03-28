# Other Language Bindings Reference

Quick reference for Go, Java, C#, Ruby, PHP, Elixir, WASM, and C FFI bindings.

## Go

Pure-Go HTTP client. No cgo or shared libraries required.

```bash
go get github.com/kreuzberg-dev/liter-llm/packages/go
```

```go
import literllm "github.com/kreuzberg-dev/liter-llm/packages/go"

client := literllm.NewClient(
    literllm.WithAPIKey(os.Getenv("OPENAI_API_KEY")),
    literllm.WithCache(literllm.CacheConfig{MaxEntries: 256, TTLSeconds: 300}),
    literllm.WithBudget(literllm.BudgetConfig{GlobalLimit: 10.0, Enforcement: "hard"}),
)

// Chat
resp, err := client.Chat(ctx, &literllm.ChatCompletionRequest{
    Model:    "gpt-4",
    Messages: []literllm.Message{literllm.NewTextMessage(literllm.RoleUser, "Hello!")},
})

// Streaming
err := client.ChatStream(ctx, &literllm.ChatCompletionRequest{
    Model:    "gpt-4",
    Messages: []literllm.Message{literllm.NewTextMessage(literllm.RoleUser, "Hello!")},
}, func(chunk *literllm.ChatCompletionChunk) error {
    if len(chunk.Choices) > 0 && chunk.Choices[0].Delta.Content != nil {
        fmt.Print(*chunk.Choices[0].Delta.Content)
    }
    return nil
})

// Embeddings
resp, err := client.Embed(ctx, &literllm.EmbeddingRequest{
    Model: "text-embedding-3-small",
    Input: literllm.NewEmbeddingInputSingle("Hello"),
})
```

**Error handling** -- sentinel errors with `errors.Is`/`errors.As`:

```go
if errors.Is(err, literllm.ErrRateLimit) { /* backoff */ }
var apiErr *literllm.APIError
if errors.As(err, &apiErr) {
    fmt.Printf("HTTP %d: %s\n", apiErr.StatusCode, apiErr.Message)
}
```

**Key notes:** Client is safe for concurrent use. Functional options pattern (`WithAPIKey`, `WithCache`, etc.). Implements `LlmHook` interface for lifecycle hooks. Full 23-method `LlmClient` interface.

---

## Java

Pure-Java HTTP client using `java.net.http.HttpClient` (Java 17+). No FFI required.

```xml
<dependency>
    <groupId>dev.kreuzberg</groupId>
    <artifactId>liter-llm</artifactId>
    <version>1.0.0-rc.1</version>
</dependency>
```

```java
import dev.kreuzberg.literllm.LlmClient;

var client = LlmClient.builder()
    .apiKey(System.getenv("OPENAI_API_KEY"))
    .cacheConfig(new CacheConfig(256, 300))
    .budgetConfig(new BudgetConfig(10.0, Map.of(), "hard"))
    .build();

// Chat
var request = Types.ChatCompletionRequest.builder(
    "gpt-4o-mini",
    List.of(new Types.UserMessage("Hello!"))
).maxTokens(256L).build();

var response = client.chat(request);
System.out.println(response.choices().getFirst().message().content());

// Streaming
client.chatStream(request, chunk -> {
    var choices = chunk.choices();
    if (!choices.isEmpty() && choices.getFirst().delta().content() != null) {
        System.out.print(choices.getFirst().delta().content());
    }
});

// Embeddings
var response = client.embed(new EmbeddingRequest(
    "text-embedding-3-small", List.of("Hello, world!")
));
```

**Error handling** -- exception hierarchy with numeric codes (1000+):

```java
try {
    var response = client.chat(request);
} catch (LlmException.RateLimitException e) {       // code 1429
    System.err.println("Rate limited: " + e.getMessage());
} catch (LlmException.AuthenticationException e) {   // code 1401
    System.err.println("Auth failed: " + e.getMessage());
} catch (LlmException e) {
    System.err.printf("Error %d: %s%n", e.getErrorCode(), e.getMessage());
}
```

**Key notes:** Builder pattern. Implements `AutoCloseable` (use try-with-resources). Java records for types. `LlmHook` interface for hooks via `addHook()`.

---

## C# / .NET

Pure .NET HTTP client targeting .NET 8+. No FFI required.

```bash
dotnet add package LiterLlm
```

```csharp
using LiterLlm;

await using var client = new LlmClient(
    apiKey: Environment.GetEnvironmentVariable("OPENAI_API_KEY")!,
    cacheConfig: new CacheConfig(256, 300),
    budgetConfig: new BudgetConfig(10.0, null, "hard")
);

// Chat
var request = new ChatCompletionRequest(
    Model: "gpt-4o-mini",
    Messages: [new UserMessage("Hello!")],
    MaxTokens: 256);
var response = await client.ChatAsync(request);
Console.WriteLine(response.Choices[0].Message.Content);

// Streaming -- IAsyncEnumerable
await foreach (var chunk in client.ChatStreamAsync(request))
{
    if (chunk.Choices[0].Delta.Content is { } content)
        Console.Write(content);
}

// Embeddings
var response = await client.EmbedAsync(new EmbeddingRequest(
    Model: "text-embedding-3-small", Input: ["Hello, world!"]));
```

**Error handling** -- exception hierarchy with numeric codes:

```csharp
try {
    var response = await client.ChatAsync(request);
} catch (RateLimitException ex) {        // code 1429
    Console.Error.WriteLine($"Rate limited: {ex.Message}");
} catch (LlmException ex) {
    Console.Error.WriteLine($"Error {ex.ErrorCode}: {ex.Message}");
}
```

**Key notes:** All methods async with `CancellationToken` support. Implements `IDisposable`/`IAsyncDisposable`. C# records for types. `ILlmHook` interface. `BudgetUsed` is a property.

---

## Ruby

Wraps Rust core via Magnus. All request/response data as JSON strings.

```bash
gem install liter_llm
```

```ruby
require 'liter_llm'

client = LiterLlm::LlmClient.new(ENV.fetch('OPENAI_API_KEY'),
  cache: { max_entries: 256, ttl_seconds: 300 },
  budget: { global_limit: 10.0, enforcement: 'hard' }
)

# Chat
response = JSON.parse(client.chat(JSON.generate(
  model: 'gpt-4',
  messages: [{ role: 'user', content: 'Hello!' }],
  max_tokens: 256
)))
puts response.dig('choices', 0, 'message', 'content')

# Streaming -- returns JSON array of all chunks
chunks = JSON.parse(client.chat_stream(JSON.generate(
  model: 'gpt-4',
  messages: [{ role: 'user', content: 'Tell me a joke' }]
)))
chunks.each do |chunk|
  content = chunk.dig('choices', 0, 'delta', 'content')
  print content if content
end

# Embeddings
response = JSON.parse(client.embed(JSON.generate(
  model: 'text-embedding-3-small', input: 'Hello'
)))
```

**Error handling** -- exception hierarchy from `LiterLlm::Error`:

```ruby
begin
  response = JSON.parse(client.chat(JSON.generate(request)))
rescue LiterLlm::RateLimitError => e
  puts "Rate limited: #{e.message}"
rescue LiterLlm::AuthenticationError => e
  puts "Auth failed: #{e.message}"
rescue LiterLlm::BudgetExceededError => e
  puts "Budget exceeded: #{e.message}"
rescue LiterLlm::Error => e
  puts "Error: #{e.message}"
end
```

**Key notes:** JSON string in, JSON string out (use `JSON.parse`/`JSON.generate`). Synchronous methods (block on Tokio internally). Client is immutable and thread-safe. Hooks are Hashes with lambda callbacks. `chat_stream` returns buffered array, not true incremental streaming.

---

## PHP

Wraps Rust core via `ext-php-rs`. JSON string in/out like Ruby.

```php
// Native extension: extension=liter_llm in php.ini
// Or via Composer:
composer require kreuzberg/liter-llm
```

```php
<?php
declare(strict_types=1);
use LiterLlm\LlmClient;

$client = new LlmClient(
    apiKey: getenv('OPENAI_API_KEY') ?: '',
    cacheConfig: ['max_entries' => 256, 'ttl_seconds' => 300],
    budgetConfig: ['global_limit' => 10.0, 'enforcement' => 'hard'],
);

// Chat
$response = json_decode($client->chat(json_encode([
    'model'    => 'gpt-4',
    'messages' => [['role' => 'user', 'content' => 'Hello']],
])), true);
echo $response['choices'][0]['message']['content'];

// Streaming -- returns JSON array of all chunks
$chunks = json_decode($client->chatStream(json_encode([
    'model'    => 'gpt-4',
    'messages' => [['role' => 'user', 'content' => 'Hello']],
])), true);
foreach ($chunks as $chunk) {
    echo $chunk['choices'][0]['delta']['content'] ?? '';
}

// Embeddings
$response = json_decode($client->embed(json_encode([
    'model' => 'text-embedding-3-small', 'input' => 'Hello',
])), true);
```

**Error handling** -- exception hierarchy from `LlmException`:

```php
try {
    $response = json_decode($client->chat(json_encode($request)), true);
} catch (RateLimitedException $e) {
    echo "Rate limited: " . $e->getMessage() . "\n";
} catch (AuthenticationException $e) {
    echo "Auth failed: " . $e->getMessage() . "\n";
} catch (LlmException $e) {
    echo "Error: " . $e->getMessage() . "\n";
}
```

**Key notes:** Named constructor params (PHP 8.2+). JSON string in/out. `chatStream` not truly incremental (full SSE consumed in Rust, returned as JSON array). Hooks are anonymous classes with `onRequest`/`onResponse`/`onError` methods. Provider registration via `registerProvider(json_encode(...))`.

---

## Elixir

Pure-Elixir HTTP client using `Req`. No NIFs required.

```elixir
# mix.exs
defp deps do
  [{:liter_llm, "~> 1.0"}]
end
```

```elixir
client = LiterLlm.Client.new(
  api_key: System.fetch_env!("OPENAI_API_KEY"),
  cache: [max_entries: 256, ttl_seconds: 300],
  budget: [global_limit: 10.0, enforcement: "hard"]
)

# Chat
{:ok, response} = LiterLlm.Client.chat(client, %{
  model: "gpt-4o-mini",
  messages: [%{role: "user", content: "Hello!"}],
  max_tokens: 256
})
content = get_in(response, ["choices", Access.at(0), "message", "content"])

# Streaming -- returns {:ok, chunks} list
{:ok, chunks} = LiterLlm.Client.chat_stream(client, %{
  model: "gpt-4",
  messages: [%{role: "user", content: "Tell me a joke"}]
})
for chunk <- chunks do
  case get_in(chunk, ["choices", Access.at(0), "delta", "content"]) do
    nil -> :skip
    content -> IO.write(content)
  end
end

# Embeddings
{:ok, response} = LiterLlm.Client.embed(client, %{
  model: "text-embedding-3-small", input: "Hello"
})
```

**Error handling** -- `{:ok, result}` / `{:error, %LiterLlm.Error{}}` tuples, pattern match on `:kind`:

```elixir
case LiterLlm.Client.chat(client, request) do
  {:ok, response} -> process(response)
  {:error, %LiterLlm.Error{kind: :rate_limit}} -> retry_after_backoff()
  {:error, %LiterLlm.Error{kind: :authentication, message: msg}} -> raise "Auth failed: #{msg}"
  {:error, %LiterLlm.Error{kind: :budget_exceeded}} -> IO.puts("Budget limit reached")
  {:error, %LiterLlm.Error{} = err} -> Logger.error("LLM error: #{err}")
end
```

**Key notes:** Idiomatic `{:ok, _}`/`{:error, _}` tuples. Client struct is immutable and process-safe. Hooks via `LiterLlm.Hook` behaviour module. Responses are maps with string keys.

---

## WebAssembly (WASM)

JavaScript-friendly `LlmClient` via `wasm-bindgen`. Uses native `fetch`. Works in browser and Node.js.

```bash
npm install @kreuzberg/liter-llm-wasm
```

```javascript
import init, { LlmClient } from '@kreuzberg/liter-llm-wasm';
await init();  // Required: initialize WASM module

const client = new LlmClient({
  apiKey: 'sk-...',
  cache: { maxEntries: 256, ttlSeconds: 300 },
  budget: { globalLimit: 10.0, enforcement: 'hard' },
});

// Chat
const resp = await client.chat({
  model: "gpt-4",
  messages: [{ role: "user", content: "Hello!" }],
  maxTokens: 256,
});
console.log(resp.choices[0].message.content);

// Streaming -- returns Promise<ChatCompletionChunk[]>
const chunks = await client.chatStream({
  model: "gpt-4",
  messages: [{ role: "user", content: "Tell me a joke" }],
});
for (const chunk of chunks) {
  process.stdout.write(chunk.choices[0]?.delta?.content ?? "");
}

// Embeddings
const resp = await client.embed({
  model: "text-embedding-3-small", input: "Hello",
});
```

**Error handling** -- JavaScript `Error` with bracketed category prefix:

```javascript
try {
  const resp = await client.chat({ model: "gpt-4", messages: [...] });
} catch (err) {
  if (err.message.startsWith("[RateLimited]")) { /* backoff */ }
  else if (err.message.startsWith("[Authentication]")) { /* bad key */ }
  else { console.error(err.message); }
}
```

**Key notes:** Must call `await init()` before use. All methods async (return Promises). **camelCase** keys (auto-converted from snake_case wire format). Ships with TypeScript `.d.ts` definitions. `chatStream` is not truly incremental (full SSE consumed in WASM). `budgetUsed` is a read-only property.

---

## C FFI

`extern "C"` interface for languages using C calling conventions. Header: `liter_llm.h`.

```c
#include "liter_llm.h"

// Create client (caller owns handle)
LiterLlmClient *client = literllm_client_new(
    "sk-...", NULL,
    "{\"cache\":{\"max_entries\":256},\"budget\":{\"global_limit\":5.0}}");

// Chat -- returns heap-allocated JSON string
char *resp = literllm_chat(client,
    "{\"model\":\"gpt-4\",\"messages\":"
    "[{\"role\":\"user\",\"content\":\"Hello!\"}]}");

// Streaming -- callback-based
void on_chunk(const char *chunk_json, void *user_data) {
    printf("%s\n", chunk_json);
}
int32_t rc = literllm_chat_stream(client, request_json, on_chunk, NULL);

// Cleanup
literllm_free_string(resp);
literllm_client_free(client);
```

**Error handling** -- NULL return + `literllm_last_error()`:

```c
char *result = literllm_chat(client, request_json);
if (result == NULL) {
    const char *err = literllm_last_error();
    if (err && strncmp(err, "[RateLimited]", 13) == 0) { /* backoff */ }
    else { fprintf(stderr, "Error: %s\n", err ? err : "unknown"); }
}
```

**Memory rules:**

| Source | Who frees | How |
|--------|-----------|-----|
| `literllm_client_new()` | Caller | `literllm_client_free()` |
| `literllm_chat()` etc. | Caller | `literllm_free_string()` |
| `literllm_last_error()` | Nobody | Thread-local, overwritten on next call |
| `literllm_version()` | Nobody | Static lifetime |
| `chunk_json` in callback | Nobody | Valid only during callback |

**Key notes:** Opaque `LiterLlmClient*` handle. Every `_new()` has a matching `_free()`. Config via JSON string. Bracketed error categories for programmatic matching. Used by Go (cgo), Java (Panama FFM), and C# (P/Invoke) as the underlying FFI layer when needed.
