<!-- snippet:compile-only -->

```elixir
{:ok, response} =
  LiterLlm.ocr(
    %{
      model: "mistral/mistral-ocr-latest",
      document: %{type: "document_url", url: "https://example.com/invoice.pdf"}
    },
    api_key: System.fetch_env!("MISTRAL_API_KEY")
  )

for page <- response["pages"] do
  IO.puts("Page #{page["index"]}: #{String.slice(page["markdown"], 0, 100)}...")
end
```
