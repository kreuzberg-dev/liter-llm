---
description: "How to generate text embeddings with liter-llm across multiple providers."
---

# Embeddings

The `embed` method generates vector embeddings from text input. Embeddings are fixed-length numeric arrays that capture semantic meaning -- useful for search, clustering, and retrieval-augmented generation (RAG).

## Basic Usage

=== "Python"

    --8<-- "snippets/python/guides/embeddings.md"

=== "TypeScript"

    --8<-- "snippets/typescript/guides/embeddings.md"

=== "Go"

    --8<-- "snippets/go/guides/embeddings.md"

## Supported Providers

Not all providers support embeddings. The major embedding providers include:

| Provider | Prefix | Example model |
| --- | --- | --- |
| OpenAI | `openai/` | `text-embedding-3-small`, `text-embedding-3-large` |
| Azure | `azure/` | `text-embedding-ada-002` |
| Cohere | `cohere/` | `embed-english-v3.0` |
| Voyage AI | `voyage/` | `voyage-3` |
| Mistral | `mistral/` | `mistral-embed` |
| Hugging Face | `huggingface/` | Various |
| Google Vertex AI | `vertex_ai/` | `text-embedding-004` |
| AWS Bedrock | `bedrock/` | `amazon.titan-embed-text-v2:0` |
| Ollama | `ollama/` | `nomic-embed-text` |
| Jina AI | `jina_ai/` | `jina-embeddings-v3` |

See the [Providers](../providers.md) page for the complete capability matrix.

## Batch Embeddings

Pass multiple strings to embed them in a single request:

```python
response = await client.embed(
    model="openai/text-embedding-3-small",
    input=[
        "First document to embed",
        "Second document to embed",
        "Third document to embed",
    ],
)
for i, item in enumerate(response.data):
    print(f"Document {i}: {len(item.embedding)} dimensions")
```

## Choosing a Model

Key considerations when selecting an embedding model:

| Factor | Guidance |
| --- | --- |
| **Dimensions** | Higher dimensions capture more nuance but use more storage. OpenAI's `text-embedding-3-small` outputs 1536 dimensions. |
| **Cost** | Embedding models are significantly cheaper per token than chat models. |
| **Latency** | Local providers (Ollama) have lower latency but may produce lower-quality embeddings. |
| **Quality** | Evaluate on your specific retrieval task. MTEB leaderboard is a good starting point. |
