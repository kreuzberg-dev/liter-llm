<!-- snippet:compile-only -->

```elixir
{:ok, response} =
  LiterLlm.image_generate(
    %{
      model: "openai/dall-e-3",
      prompt: "A sunset over mountains",
      n: 1,
      size: "1024x1024"
    },
    api_key: System.fetch_env!("OPENAI_API_KEY")
  )

IO.puts(hd(response["data"])["url"])
```
