<!-- snippet:compile-only -->

```elixir
{:ok, response} =
  LiterLlm.create_batch(
    %{
      input_file_id: "file-abc123",
      endpoint: "/v1/chat/completions",
      completion_window: "24h"
    },
    api_key: System.fetch_env!("OPENAI_API_KEY")
  )

IO.puts("Batch ID: #{response["id"]}")
IO.puts("Status: #{response["status"]}")
```
