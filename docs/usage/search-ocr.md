---
description: "Web search and document OCR with liter-llm."
---

# Search & OCR

## Search

Search the web or documents across 12 providers (Brave, Tavily, Google PSE, etc.):

=== "Python"

    --8<-- "snippets/python/usage/search.md"

=== "TypeScript"

    --8<-- "snippets/typescript/usage/search.md"

=== "Rust"

    --8<-- "snippets/rust/usage/search.md"

=== "Go"

    --8<-- "snippets/go/usage/search.md"

=== "Java"

    --8<-- "snippets/java/usage/search.md"

=== "C#"

    --8<-- "snippets/csharp/usage/search.md"

=== "Ruby"

    --8<-- "snippets/ruby/usage/search.md"

=== "PHP"

    --8<-- "snippets/php/usage/search.md"

=== "Elixir"

    --8<-- "snippets/elixir/usage/search.md"

=== "WASM"

    --8<-- "snippets/wasm/usage/search.md"

### Search Parameters

| Parameter | Type | Description |
| --- | --- | --- |
| `model` | string | Search provider (e.g. `"brave/web-search"`) |
| `query` | string | Search query |
| `max_results` | int | Maximum results to return |
| `search_domain_filter` | array | Restrict to specific domains |
| `country` | string | ISO country code for localized results |

### Search Providers

| Provider | Prefix | Example Model |
| --- | --- | --- |
| Brave | `brave/` | `web-search` |
| Tavily | `tavily/` | `tavily-search` |
| Google PSE | `google_pse/` | `google-search` |
| Serper | `serper/` | `serper-search` |
| SerpAPI | `serpapi/` | `serpapi-search` |
| Bing | `bing/` | `bing-search` |

See the [Providers](../providers.md) page for the complete capability matrix.

## OCR

Extract text from documents via OCR across 4 providers (Mistral, Azure Doc Intelligence, etc.):

=== "Python"

    --8<-- "snippets/python/usage/ocr.md"

=== "TypeScript"

    --8<-- "snippets/typescript/usage/ocr.md"

=== "Rust"

    --8<-- "snippets/rust/usage/ocr.md"

=== "Go"

    --8<-- "snippets/go/usage/ocr.md"

=== "Java"

    --8<-- "snippets/java/usage/ocr.md"

=== "C#"

    --8<-- "snippets/csharp/usage/ocr.md"

=== "Ruby"

    --8<-- "snippets/ruby/usage/ocr.md"

=== "PHP"

    --8<-- "snippets/php/usage/ocr.md"

=== "Elixir"

    --8<-- "snippets/elixir/usage/ocr.md"

=== "WASM"

    --8<-- "snippets/wasm/usage/ocr.md"

### OCR Parameters

| Parameter | Type | Description |
| --- | --- | --- |
| `model` | string | OCR provider (e.g. `"mistral/mistral-ocr-latest"`) |
| `document` | object | Document input (URL or base64) |
| `pages` | array | Specific pages to process (1-indexed) |
| `include_image_base64` | bool | Include extracted images |

### Document Input Formats

**URL:**

```json
{"type": "document_url", "url": "https://example.com/invoice.pdf"}
```

**Base64:**

```json
{"type": "base64", "data": "...", "media_type": "application/pdf"}
```
