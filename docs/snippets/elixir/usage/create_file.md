<!-- snippet:compile-only -->

```elixir
file_bytes = File.read!("data.jsonl")

{:ok, response} =
  LiterLlm.create_file(
    %{
      file: file_bytes,
      filename: "data.jsonl",
      purpose: "batch"
    },
    api_key: System.fetch_env!("OPENAI_API_KEY")
  )

IO.puts("File ID: #{response["id"]}")
IO.puts("Size: #{response["bytes"]} bytes")
```
