---
name: embeddings-and-search
description: Use when generating embeddings, calling the 12 web-search providers, or running OCR over documents with the 4 OCR providers through liter-llm. Covers embed, search, and ocr methods plus reranking.
---

<!--
AI-RULEZ :: GENERATED FILE — DO NOT EDIT
Content-Hash: blake3:2486d6f9ae493f698ce3c5349658dacd142fb7dd3cd27a707c4acf3aa8877a55
Source-Hash: blake3:5982f9e920534d61a2cd6166e32a8ee98938c731fc6e892800aaecbeaf8c1221
Schema-Version: v1
-->

# Embeddings and Search

liter-llm exposes embeddings, web search (12 providers), OCR (4 providers), and
reranking through the same `provider/model` routing convention.

## Embeddings

```python
import asyncio, os
from liter_llm import create_client
from liter_llm._internal_bindings import EmbeddingRequest

async def main() -> None:
    client = create_client(api_key=os.environ["OPENAI_API_KEY"])
    request = EmbeddingRequest.from_json(
        '{"model":"openai/text-embedding-3-small","input":["first document","second document"]}'
    )
    response = await client.embed(request)
    for item in response.data:
        print(len(item.embedding))

asyncio.run(main())
```

Many embedding models support dimension selection and base64 output; set
`dimensions` / `encoding_format` in the request where the provider allows it.

## Web search (12 providers)

```python
from liter_llm._internal_bindings import SearchRequest

client = create_client(api_key=os.environ["BRAVE_API_KEY"])
request = SearchRequest.from_json(
    '{"model":"brave/web-search","query":"What is the Rust programming language?","max_results":5}'
)
response = await client.search(request)
for result in response.results:
    print(result.title, result.url)
```

## OCR (4 providers)

```python
from liter_llm._internal_bindings import OcrRequest

client = create_client(api_key=os.environ["MISTRAL_API_KEY"])
request = OcrRequest.from_json(
    '{"model":"mistral/mistral-ocr-latest",'
    '"document":{"type":"document_url","url":"https://example.com/invoice.pdf"}}'
)
response = await client.ocr(request)
for page in response.pages:
    print(page.index, page.markdown[:100])
```

## Reranking

Build a `RerankRequest` (model, query, documents) and call `client.rerank(request)`
to score and order candidate documents against a query for retrieval pipelines —
combine it with `embed` for hybrid retrieval. Each result carries `index` and
`relevance_score`. Routing follows the same `provider/model` convention.

## Notes

- Search and OCR providers each need their own API key (e.g. `BRAVE_API_KEY`,
  `MISTRAL_API_KEY`); read them from env vars.
- See the upstream provider reference for the full list of the 12 search and 4
  OCR backends and their model identifiers.
