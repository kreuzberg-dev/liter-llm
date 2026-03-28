---
description: "File uploads, batch processing, and the Responses API with liter-llm."
---

# Files & Batches

## File Operations

Upload, retrieve, list, and delete files. Files are used with batch processing, fine-tuning, and assistants.

### Upload a File

=== "Python"

    --8<-- "snippets/python/usage/create_file.md"

=== "TypeScript"

    --8<-- "snippets/typescript/usage/create_file.md"

=== "Rust"

    --8<-- "snippets/rust/usage/create_file.md"

=== "Go"

    --8<-- "snippets/go/usage/create_file.md"

=== "Java"

    --8<-- "snippets/java/usage/create_file.md"

=== "C#"

    --8<-- "snippets/csharp/usage/create_file.md"

=== "Ruby"

    --8<-- "snippets/ruby/usage/create_file.md"

=== "PHP"

    --8<-- "snippets/php/usage/create_file.md"

=== "Elixir"

    --8<-- "snippets/elixir/usage/create_file.md"

=== "WASM"

    --8<-- "snippets/wasm/usage/create_file.md"

### File Methods

| Method | Description |
| --- | --- |
| `create_file` | Upload a file with a purpose (`"batch"`, `"fine-tune"`, `"assistants"`) |
| `retrieve_file` | Get metadata for an uploaded file by ID |
| `delete_file` | Delete an uploaded file by ID |
| `list_files` | List all uploaded files, optionally filtered by purpose |
| `file_content` | Download the raw content of an uploaded file |

## Batch Processing

Create batch jobs to process multiple requests asynchronously at reduced cost:

=== "Python"

    --8<-- "snippets/python/usage/create_batch.md"

=== "TypeScript"

    --8<-- "snippets/typescript/usage/create_batch.md"

=== "Rust"

    --8<-- "snippets/rust/usage/create_batch.md"

=== "Go"

    --8<-- "snippets/go/usage/create_batch.md"

=== "Java"

    --8<-- "snippets/java/usage/create_batch.md"

=== "C#"

    --8<-- "snippets/csharp/usage/create_batch.md"

=== "Ruby"

    --8<-- "snippets/ruby/usage/create_batch.md"

=== "PHP"

    --8<-- "snippets/php/usage/create_batch.md"

=== "Elixir"

    --8<-- "snippets/elixir/usage/create_batch.md"

=== "WASM"

    --8<-- "snippets/wasm/usage/create_batch.md"

### Batch Methods

| Method | Description |
| --- | --- |
| `create_batch` | Create a batch from an uploaded JSONL file |
| `retrieve_batch` | Get batch status and results by ID |
| `list_batches` | List all batches |
| `cancel_batch` | Cancel a running batch |

### Batch Parameters

| Parameter | Type | Description |
| --- | --- | --- |
| `input_file_id` | string | ID of the uploaded JSONL file |
| `endpoint` | string | API endpoint (`"/v1/chat/completions"`, `"/v1/embeddings"`) |
| `completion_window` | string | Processing window (`"24h"`) |
| `metadata` | object | Optional key-value metadata |

## Responses API

Create, retrieve, and cancel responses via the Responses API:

=== "Python"

    --8<-- "snippets/python/usage/create_response.md"

=== "TypeScript"

    --8<-- "snippets/typescript/usage/create_response.md"

=== "Rust"

    --8<-- "snippets/rust/usage/create_response.md"

=== "Go"

    --8<-- "snippets/go/usage/create_response.md"

=== "Java"

    --8<-- "snippets/java/usage/create_response.md"

=== "C#"

    --8<-- "snippets/csharp/usage/create_response.md"

=== "Ruby"

    --8<-- "snippets/ruby/usage/create_response.md"

=== "PHP"

    --8<-- "snippets/php/usage/create_response.md"

=== "Elixir"

    --8<-- "snippets/elixir/usage/create_response.md"

=== "WASM"

    --8<-- "snippets/wasm/usage/create_response.md"

### Response Methods

| Method | Description |
| --- | --- |
| `create_response` | Create a new response |
| `retrieve_response` | Get a response by ID |
| `cancel_response` | Cancel a response |

### Response Parameters

| Parameter | Type | Description |
| --- | --- | --- |
| `model` | string | Model to use |
| `input` | string | Input text or conversation |
| `instructions` | string | System-level instructions |
| `max_output_tokens` | int | Maximum tokens to generate |
| `temperature` | float | Sampling temperature |
