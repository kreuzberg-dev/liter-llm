---
description: "liter-llm C / FFI API reference"
---

# C / FFI API Reference

The C FFI layer provides an `extern "C"` interface for languages that interoperate via C calling conventions (Go via cgo, Java via Panama FFM, C# via P/Invoke). The header file is `liter_llm.h`.

## Installation

Link against `libliter_llm_ffi` (shared or static) and include the header:

```c
#include "liter_llm.h"
```

## Constants

```c
#define LITER_LLM_VERSION_MAJOR 1
#define LITER_LLM_VERSION_MINOR 0
#define LITER_LLM_VERSION_PATCH 0
#define LITER_LLM_VERSION "1.0.0-rc.1"
```

## Opaque Handle

```c
typedef struct LiterLlmClient LiterLlmClient;
```

All operations go through an opaque `LiterLlmClient*` handle. Never dereference or inspect its contents.

## Function Summary

| Function | Returns | Description |
|----------|---------|-------------|
| `literllm_client_new` | `LiterLlmClient*` | Create a client handle |
| `literllm_client_free` | `void` | Free a client handle |
| `literllm_chat` | `char*` | Chat completion |
| `literllm_chat_stream` | `int32_t` | Streaming chat with callback |
| `literllm_embed` | `char*` | Embeddings |
| `literllm_list_models` | `char*` | List models |
| `literllm_image_generate` | `char*` | Image generation |
| `literllm_speech` | `char*` | Text-to-speech (base64) |
| `literllm_transcribe` | `char*` | Speech-to-text |
| `literllm_moderate` | `char*` | Content moderation |
| `literllm_rerank` | `char*` | Document reranking |
| `literllm_set_hooks` | `int32_t` | Register hook callbacks |
| `literllm_register_provider` | `int32_t` | Register a custom provider |
| `literllm_unregister_provider` | `int32_t` | Remove a custom provider |
| `literllm_budget_used` | `double` | Query budget spend |
| `literllm_last_error` | `const char*` | Last error message |
| `literllm_free_string` | `void` | Free a returned string |
| `literllm_version` | `const char*` | Library version |

## Functions

### `literllm_client_new`

Create a new client. Returns `NULL` on failure. The caller owns the returned pointer and must free it with `literllm_client_free()`.

```c
LiterLlmClient *literllm_client_new(
    const char *api_key,
    const char *base_url,      // NULL for default routing
    const char *config_json    // NULL for defaults, or JSON with extra options
);
```

The optional `config_json` parameter accepts a JSON string with additional settings:

```json
{
  "model_hint": "groq/llama3-70b",
  "max_retries": 3,
  "timeout_secs": 60,
  "cache": { "max_entries": 256, "ttl_seconds": 300 },
  "budget": { "global_limit": 10.0, "enforcement": "hard" }
}
```

| Config Key | Type | Default | Description |
|------------|------|---------|-------------|
| `model_hint` | `string` | `null` | Hint for provider auto-detection |
| `max_retries` | `int` | `3` | Retries on 429/5xx |
| `timeout_secs` | `int` | `60` | Request timeout in seconds |
| `cache.max_entries` | `int` | `256` | Maximum cached responses |
| `cache.ttl_seconds` | `int` | `300` | Cache entry time-to-live |
| `budget.global_limit` | `float` | `none` | Maximum spend in USD |
| `budget.enforcement` | `string` | `"soft"` | `"hard"` or `"soft"` |

### `literllm_client_free`

Free a client handle. Passing `NULL` is safe (no-op).

```c
void literllm_client_free(LiterLlmClient *client);
```

### `literllm_chat`

Send a chat completion request. Returns a heap-allocated JSON string (`ChatCompletionResponse`) on success, `NULL` on failure. Free with `literllm_free_string()`.

```c
char *literllm_chat(const LiterLlmClient *client, const char *request_json);
```

```c
char *resp = literllm_chat(client,
    "{\"model\":\"gpt-4\",\"messages\":"
    "[{\"role\":\"user\",\"content\":\"Hello!\"}]}");
if (!resp) {
    fprintf(stderr, "Error: %s\n", literllm_last_error());
} else {
    printf("%s\n", resp);
    literllm_free_string(resp);
}
```

### `literllm_chat_stream`

Send a streaming chat completion. Invokes the callback for each SSE chunk. Returns `0` on success, `-1` on failure.

```c
typedef void (*LiterLlmStreamCallback)(const char *chunk_json, void *user_data);

int32_t literllm_chat_stream(
    const LiterLlmClient *client,
    const char *request_json,
    LiterLlmStreamCallback callback,
    void *user_data
);
```

The `chunk_json` pointer passed to the callback is valid only for the duration of each invocation.

```c
void on_chunk(const char *chunk_json, void *user_data) {
    printf("%s\n", chunk_json);
}

int32_t rc = literllm_chat_stream(client, request_json, on_chunk, NULL);
if (rc != 0) {
    fprintf(stderr, "Stream error: %s\n", literllm_last_error());
}
```

### `literllm_embed`

Send an embedding request. Returns JSON on success, `NULL` on failure.

```c
char *literllm_embed(const LiterLlmClient *client, const char *request_json);
```

```c
char *resp = literllm_embed(client,
    "{\"model\":\"text-embedding-3-small\",\"input\":\"Hello\"}");
```

### `literllm_list_models`

List available models. Returns JSON on success, `NULL` on failure.

```c
char *literllm_list_models(const LiterLlmClient *client);
```

### `literllm_image_generate`

Generate an image from a text prompt. Returns JSON on success, `NULL` on failure.

```c
char *literllm_image_generate(const LiterLlmClient *client, const char *request_json);
```

### `literllm_speech`

Generate speech audio. Returns a base64-encoded string of the audio bytes on success, `NULL` on failure.

```c
char *literllm_speech(const LiterLlmClient *client, const char *request_json);
```

### `literllm_transcribe`

Transcribe audio to text. Returns JSON on success, `NULL` on failure.

```c
char *literllm_transcribe(const LiterLlmClient *client, const char *request_json);
```

### `literllm_moderate`

Check content against moderation policies. Returns JSON on success, `NULL` on failure.

```c
char *literllm_moderate(const LiterLlmClient *client, const char *request_json);
```

### `literllm_rerank`

Rerank documents by relevance to a query. Returns JSON on success, `NULL` on failure.

```c
char *literllm_rerank(const LiterLlmClient *client, const char *request_json);
```

### File Management

```c
char *literllm_create_file(const LiterLlmClient *client, const char *request_json);
char *literllm_retrieve_file(const LiterLlmClient *client, const char *file_id);
char *literllm_delete_file(const LiterLlmClient *client, const char *file_id);
char *literllm_list_files(const LiterLlmClient *client, const char *query_json);  // query_json may be NULL
char *literllm_file_content(const LiterLlmClient *client, const char *file_id);   // returns base64
```

All return `NULL` on failure. Free returned strings with `literllm_free_string()`.

### Batch Management

```c
char *literllm_create_batch(const LiterLlmClient *client, const char *request_json);
char *literllm_retrieve_batch(const LiterLlmClient *client, const char *batch_id);
char *literllm_list_batches(const LiterLlmClient *client, const char *query_json);  // query_json may be NULL
char *literllm_cancel_batch(const LiterLlmClient *client, const char *batch_id);
```

### Responses API

```c
char *literllm_create_response(const LiterLlmClient *client, const char *request_json);
char *literllm_retrieve_response(const LiterLlmClient *client, const char *response_id);
char *literllm_cancel_response(const LiterLlmClient *client, const char *response_id);
```

### `literllm_set_hooks`

Register hook callbacks for observing requests, responses, and errors. Returns `0` on success, `-1` on failure. Pass `NULL` for any callback you do not need.

```c
typedef void (*LiterLlmOnRequest)(const char *request_json, void *user_data);
typedef void (*LiterLlmOnResponse)(const char *request_json, const char *response_json, void *user_data);
typedef void (*LiterLlmOnError)(const char *request_json, const char *error_msg, void *user_data);

int32_t literllm_set_hooks(
    LiterLlmClient *client,
    LiterLlmOnRequest on_request,
    LiterLlmOnResponse on_response,
    LiterLlmOnError on_error,
    void *user_data
);
```

```c
void my_on_request(const char *req, void *ud) {
    fprintf(stderr, "Request: %s\n", req);
}

void my_on_error(const char *req, const char *err, void *ud) {
    fprintf(stderr, "Error: %s\n", err);
}

literllm_set_hooks(client, my_on_request, NULL, my_on_error, NULL);
```

### `literllm_register_provider`

Register a custom provider at runtime. `provider_json` is a JSON string with provider configuration. Returns `0` on success, `-1` on failure.

```c
int32_t literllm_register_provider(LiterLlmClient *client, const char *provider_json);
```

```c
int32_t rc = literllm_register_provider(client,
    "{\"name\":\"my-provider\","
    "\"base_url\":\"https://api.my-provider.com/v1\","
    "\"auth_header\":\"Authorization\","
    "\"model_prefixes\":[\"my-provider/\"]}");
```

### `literllm_unregister_provider`

Remove a previously registered provider by name. Returns `0` on success, `-1` on failure.

```c
int32_t literllm_unregister_provider(LiterLlmClient *client, const char *name);
```

### `literllm_budget_used`

Returns the total spend tracked by the budget system (in USD). Returns `0.0` if no budget is configured.

```c
double literllm_budget_used(const LiterLlmClient *client);
```

### Utility Functions

#### `literllm_last_error`

Retrieve the last error message for the current thread. Returns `NULL` if no error. The pointer is valid until the next liter-llm call on the same thread. Do NOT free this pointer.

```c
const char *literllm_last_error(void);
```

#### `literllm_free_string`

Free a string returned by any `literllm_*` function. Passing `NULL` is safe. Do NOT pass the pointer from `literllm_last_error()`.

```c
void literllm_free_string(char *s);
```

#### `literllm_version`

Returns the library version string. Valid for the program lifetime. Do NOT free.

```c
const char *literllm_version(void);
```

## Error Handling

All functions that return `char*` return `NULL` on failure. All functions that return `int32_t` return `-1` on failure. Always check `literllm_last_error()` after a `NULL` or `-1` return.

The error message includes a bracketed category prefix for programmatic matching:

| Category | Trigger |
|----------|---------|
| `[Authentication]` | API key rejected (HTTP 401/403) |
| `[RateLimited]` | Rate limit exceeded (HTTP 429) |
| `[BadRequest]` | Malformed request (HTTP 400) |
| `[ContextWindowExceeded]` | Prompt exceeds context window |
| `[ContentPolicy]` | Content policy violation |
| `[NotFound]` | Model/resource not found (HTTP 404) |
| `[ServerError]` | Provider 5xx error |
| `[ServiceUnavailable]` | Provider temporarily unavailable (HTTP 502/503) |
| `[Timeout]` | Request timed out |
| `[Network]` | Network-level failure |
| `[Streaming]` | Error reading streaming response |
| `[EndpointNotSupported]` | Provider does not support the endpoint |
| `[Serialization]` | JSON serialization/deserialization failure |

```c
char *result = literllm_chat(client, request_json);
if (result == NULL) {
    const char *err = literllm_last_error();
    if (err && strncmp(err, "[RateLimited]", 13) == 0) {
        // back off and retry
    } else {
        fprintf(stderr, "Error: %s\n", err ? err : "unknown");
    }
    return 1;
}
// Use result...
literllm_free_string(result);
```

## Types

All request and response data is exchanged as JSON strings using the OpenAI-compatible wire format. Parse the returned `char*` as JSON to access fields:

| Response type | Key fields |
|---------------|------------|
| Chat completion | `id`, `model`, `choices[].message.content`, `choices[].finish_reason`, `usage.{prompt_tokens, completion_tokens, total_tokens}` |
| Embedding | `data[].embedding`, `model`, `usage` |
| Models list | `data[].id`, `data[].object` |
| Image | `data[].url` or `data[].b64_json` |
| Transcription | `text`, `language`, `duration` |
| Moderation | `results[].flagged`, `results[].categories` |
| Rerank | `results[].index`, `results[].relevance_score` |

## Memory Rules

| Pointer source | Who frees? | How? |
|----------------|------------|------|
| `literllm_client_new()` | Caller | `literllm_client_free()` |
| `literllm_chat()`, `literllm_embed()`, etc. | Caller | `literllm_free_string()` |
| `literllm_last_error()` | Nobody | Do NOT free (thread-local, overwritten on next call) |
| `literllm_version()` | Nobody | Do NOT free (static lifetime) |
| `chunk_json` in stream callback | Nobody | Valid only during callback invocation |
| Hook callback parameters | Nobody | Valid only during callback invocation |

## Example (C)

```c
#include <stdio.h>
#include <string.h>
#include "liter_llm.h"

int main(void) {
    // Create client with cache and budget
    LiterLlmClient *client = literllm_client_new(
        "sk-...", NULL,
        "{\"cache\":{\"max_entries\":256},\"budget\":{\"global_limit\":5.0}}");
    if (!client) {
        fprintf(stderr, "Error: %s\n", literllm_last_error());
        return 1;
    }

    const char *request = "{\"model\":\"gpt-4\",\"messages\":"
                          "[{\"role\":\"user\",\"content\":\"Hello!\"}]}";

    char *response = literllm_chat(client, request);
    if (!response) {
        fprintf(stderr, "Error: %s\n", literllm_last_error());
        literllm_client_free(client);
        return 1;
    }

    printf("%s\n", response);
    printf("Budget used: $%.4f\n", literllm_budget_used(client));

    literllm_free_string(response);
    literllm_client_free(client);
    return 0;
}
```

## Example (Go via cgo)

```go
/*
#cgo LDFLAGS: -lliter_llm_ffi
#include "liter_llm.h"
#include <stdlib.h>
*/
import "C"
import (
    "fmt"
    "unsafe"
)

func main() {
    apiKey := C.CString("sk-...")
    defer C.free(unsafe.Pointer(apiKey))

    client := C.literllm_client_new(apiKey, nil, nil)
    defer C.literllm_client_free(client)

    req := C.CString(`{"model":"gpt-4","messages":[{"role":"user","content":"Hi"}]}`)
    defer C.free(unsafe.Pointer(req))

    resp := C.literllm_chat(client, req)
    if resp == nil {
        panic(C.GoString(C.literllm_last_error()))
    }
    defer C.literllm_free_string(resp)

    fmt.Println(C.GoString(resp))
}
```
