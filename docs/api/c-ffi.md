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

## Functions

### `literllm_client_new`

Create a new client.

```c
LiterLlmClient *literllm_client_new(
    const char *api_key,
    const char *base_url,    // NULL for default routing
    const char *model_hint   // NULL for OpenAI default
);
```

Returns `NULL` on failure. Check `literllm_last_error()` for details. The caller owns the returned pointer and must free it with `literllm_client_free()`.

### `literllm_client_free`

Free a client handle. Passing `NULL` is safe (no-op).

```c
void literllm_client_free(LiterLlmClient *client);
```

### `literllm_chat`

Send a chat completion request.

```c
char *literllm_chat(const LiterLlmClient *client, const char *request_json);
```

Returns a heap-allocated JSON string (`ChatCompletionResponse`) on success, `NULL` on failure. Free with `literllm_free_string()`.

### `literllm_chat_stream`

Send a streaming chat completion. Invokes the callback for each SSE chunk.

```c
typedef void (*LiterLlmStreamCallback)(const char *chunk_json, void *user_data);

int32_t literllm_chat_stream(
    const LiterLlmClient *client,
    const char *request_json,
    LiterLlmStreamCallback callback,
    void *user_data
);
```

Returns `0` on success, `-1` on failure. The `chunk_json` pointer passed to the callback is valid only for the duration of each invocation.

### `literllm_embed`

Send an embedding request.

```c
char *literllm_embed(const LiterLlmClient *client, const char *request_json);
```

### `literllm_list_models`

List available models.

```c
char *literllm_list_models(const LiterLlmClient *client);
```

### `literllm_image_generate`

Generate an image from a text prompt.

```c
char *literllm_image_generate(const LiterLlmClient *client, const char *request_json);
```

### `literllm_speech`

Generate speech audio. Returns a base64-encoded string of the audio bytes.

```c
char *literllm_speech(const LiterLlmClient *client, const char *request_json);
```

### `literllm_transcribe`

Transcribe audio to text.

```c
char *literllm_transcribe(const LiterLlmClient *client, const char *request_json);
```

### `literllm_moderate`

Check content against moderation policies.

```c
char *literllm_moderate(const LiterLlmClient *client, const char *request_json);
```

### `literllm_rerank`

Rerank documents by relevance to a query.

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

### Utility Functions

#### `literllm_last_error`

Retrieve the last error message for the current thread.

```c
const char *literllm_last_error(void);
```

Returns `NULL` if no error. The pointer is valid until the next liter-llm call on the same thread. Do NOT free this pointer.

#### `literllm_free_string`

Free a string returned by any `literllm_*` function.

```c
void literllm_free_string(char *s);
```

Passing `NULL` is safe. Do NOT pass the pointer from `literllm_last_error()`.

#### `literllm_version`

Returns the library version string. Valid for the program lifetime. Do NOT free.

```c
const char *literllm_version(void);
```

## Error Handling

All functions that return `char*` return `NULL` on failure. All functions that return `int32_t` return `-1` on failure. Always check `literllm_last_error()` after a `NULL` or `-1` return.

```c
char *result = literllm_chat(client, request_json);
if (result == NULL) {
    const char *err = literllm_last_error();
    fprintf(stderr, "Error: %s\n", err ? err : "unknown");
    return 1;
}
// Use result...
literllm_free_string(result);
```

## Memory Rules

| Pointer source | Who frees? | How? |
|----------------|------------|------|
| `literllm_client_new()` | Caller | `literllm_client_free()` |
| `literllm_chat()`, `literllm_embed()`, etc. | Caller | `literllm_free_string()` |
| `literllm_last_error()` | Nobody | Do NOT free (thread-local, overwritten on next call) |
| `literllm_version()` | Nobody | Do NOT free (static lifetime) |
| `chunk_json` in stream callback | Nobody | Valid only during callback invocation |

## Example (C)

```c
#include <stdio.h>
#include "liter_llm.h"

int main(void) {
    LiterLlmClient *client = literllm_client_new("sk-...", NULL, NULL);
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
import "unsafe"

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
