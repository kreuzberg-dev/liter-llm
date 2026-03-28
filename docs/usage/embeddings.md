---
description: "Generate text embeddings and rerank documents with liter-llm."
---

# Embeddings & Rerank

## Embeddings

Generate vector embeddings from text. Embeddings are fixed-length numeric arrays that capture semantic meaning -- useful for search, clustering, and RAG.

=== "Python"

    --8<-- "snippets/python/guides/embeddings.md"

=== "TypeScript"

    --8<-- "snippets/typescript/guides/embeddings.md"

=== "Rust"

    --8<-- "snippets/rust/usage/embeddings.md"

=== "Go"

    --8<-- "snippets/go/guides/embeddings.md"

=== "Java"

    --8<-- "snippets/java/usage/embeddings.md"

=== "C#"

    --8<-- "snippets/csharp/usage/embeddings.md"

=== "Ruby"

    --8<-- "snippets/ruby/usage/embeddings.md"

=== "PHP"

    --8<-- "snippets/php/usage/embeddings.md"

=== "Elixir"

    --8<-- "snippets/elixir/usage/embeddings.md"

=== "WASM"

    --8<-- "snippets/wasm/usage/embeddings.md"

### Embedding Parameters

| Parameter | Type | Description |
| --- | --- | --- |
| `model` | string | Embedding model (e.g. `"openai/text-embedding-3-small"`) |
| `input` | string/array | Text(s) to embed |
| `encoding_format` | string | Output format (`"float"` or `"base64"`) |
| `dimensions` | int | Output dimensionality (model-dependent) |

### Embedding Providers

| Provider | Prefix | Example Model |
| --- | --- | --- |
| OpenAI | `openai/` | `text-embedding-3-small`, `text-embedding-3-large` |
| Cohere | `cohere/` | `embed-english-v3.0` |
| Voyage AI | `voyage/` | `voyage-3` |
| Mistral | `mistral/` | `mistral-embed` |
| Google Vertex AI | `vertex_ai/` | `text-embedding-004` |
| AWS Bedrock | `bedrock/` | `amazon.titan-embed-text-v2:0` |
| Ollama | `ollama/` | `nomic-embed-text` |
| Jina AI | `jina_ai/` | `jina-embeddings-v3` |

See the [Providers](../providers.md) page for the complete capability matrix.

## Rerank

Rerank documents by relevance to a query. Useful for improving retrieval quality in RAG pipelines:

=== "Python"

    --8<-- "snippets/python/usage/rerank.md"

=== "TypeScript"

    --8<-- "snippets/typescript/usage/rerank.md"

=== "Rust"

    --8<-- "snippets/rust/usage/rerank.md"

=== "Go"

    --8<-- "snippets/go/usage/rerank.md"

=== "Java"

    --8<-- "snippets/java/usage/rerank.md"

=== "C#"

    --8<-- "snippets/csharp/usage/rerank.md"

=== "Ruby"

    --8<-- "snippets/ruby/usage/rerank.md"

=== "PHP"

    --8<-- "snippets/php/usage/rerank.md"

=== "Elixir"

    --8<-- "snippets/elixir/usage/rerank.md"

=== "WASM"

    --8<-- "snippets/wasm/usage/rerank.md"

### Rerank Parameters

| Parameter | Type | Description |
| --- | --- | --- |
| `model` | string | Rerank model (e.g. `"cohere/rerank-v3.5"`) |
| `query` | string | The query to rank documents against |
| `documents` | array | Documents to rerank |
| `top_n` | int | Number of top results to return |
| `return_documents` | bool | Include document text in results |
