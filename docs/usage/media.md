---
description: "Image generation, text-to-speech, speech-to-text, and content moderation with liter-llm."
---

# Media (Images, Speech, Transcription)

## Image Generation

Generate images from text prompts:

=== "Python"

    --8<-- "snippets/python/usage/image_generate.md"

=== "TypeScript"

    --8<-- "snippets/typescript/usage/image_generate.md"

=== "Rust"

    --8<-- "snippets/rust/usage/image_generate.md"

=== "Go"

    --8<-- "snippets/go/usage/image_generate.md"

=== "Java"

    --8<-- "snippets/java/usage/image_generate.md"

=== "C#"

    --8<-- "snippets/csharp/usage/image_generate.md"

=== "Ruby"

    --8<-- "snippets/ruby/usage/image_generate.md"

=== "PHP"

    --8<-- "snippets/php/usage/image_generate.md"

=== "Elixir"

    --8<-- "snippets/elixir/usage/image_generate.md"

=== "WASM"

    --8<-- "snippets/wasm/usage/image_generate.md"

### Image Parameters

| Parameter | Type | Description |
| --- | --- | --- |
| `model` | string | Image model (e.g. `"openai/dall-e-3"`) |
| `prompt` | string | Text description of the image |
| `n` | int | Number of images to generate |
| `size` | string | Image size (`"1024x1024"`, `"1792x1024"`, `"1024x1792"`) |
| `quality` | string | Quality level (`"standard"` or `"hd"`) |
| `style` | string | Style (`"vivid"` or `"natural"`) |

## Text-to-Speech

Generate audio from text:

=== "Python"

    --8<-- "snippets/python/usage/speech.md"

=== "TypeScript"

    --8<-- "snippets/typescript/usage/speech.md"

=== "Rust"

    --8<-- "snippets/rust/usage/speech.md"

=== "Go"

    --8<-- "snippets/go/usage/speech.md"

=== "Java"

    --8<-- "snippets/java/usage/speech.md"

=== "C#"

    --8<-- "snippets/csharp/usage/speech.md"

=== "Ruby"

    --8<-- "snippets/ruby/usage/speech.md"

=== "PHP"

    --8<-- "snippets/php/usage/speech.md"

=== "Elixir"

    --8<-- "snippets/elixir/usage/speech.md"

=== "WASM"

    --8<-- "snippets/wasm/usage/speech.md"

### Speech Parameters

| Parameter | Type | Description |
| --- | --- | --- |
| `model` | string | TTS model (e.g. `"openai/tts-1"`) |
| `input` | string | Text to synthesize |
| `voice` | string | Voice preset (`"alloy"`, `"echo"`, `"fable"`, `"onyx"`, `"nova"`, `"shimmer"`) |
| `response_format` | string | Audio format (`"mp3"`, `"opus"`, `"aac"`, `"flac"`) |
| `speed` | float | Playback speed (0.25-4.0) |

## Speech-to-Text

Transcribe audio to text:

=== "Python"

    --8<-- "snippets/python/usage/transcribe.md"

=== "TypeScript"

    --8<-- "snippets/typescript/usage/transcribe.md"

=== "Rust"

    --8<-- "snippets/rust/usage/transcribe.md"

=== "Go"

    --8<-- "snippets/go/usage/transcribe.md"

=== "Java"

    --8<-- "snippets/java/usage/transcribe.md"

=== "C#"

    --8<-- "snippets/csharp/usage/transcribe.md"

=== "Ruby"

    --8<-- "snippets/ruby/usage/transcribe.md"

=== "PHP"

    --8<-- "snippets/php/usage/transcribe.md"

=== "Elixir"

    --8<-- "snippets/elixir/usage/transcribe.md"

=== "WASM"

    --8<-- "snippets/wasm/usage/transcribe.md"

### Transcription Parameters

| Parameter | Type | Description |
| --- | --- | --- |
| `model` | string | STT model (e.g. `"openai/whisper-1"`) |
| `file` | bytes | Audio file data |
| `language` | string | ISO-639-1 language code |
| `prompt` | string | Optional context hint |
| `temperature` | float | Sampling temperature |
| `response_format` | string | Output format (`"json"`, `"text"`, `"srt"`, `"vtt"`) |

## Content Moderation

Classify content for policy violations:

=== "Python"

    --8<-- "snippets/python/usage/moderate.md"

=== "TypeScript"

    --8<-- "snippets/typescript/usage/moderate.md"

=== "Rust"

    --8<-- "snippets/rust/usage/moderate.md"

=== "Go"

    --8<-- "snippets/go/usage/moderate.md"

=== "Java"

    --8<-- "snippets/java/usage/moderate.md"

=== "C#"

    --8<-- "snippets/csharp/usage/moderate.md"

=== "Ruby"

    --8<-- "snippets/ruby/usage/moderate.md"

=== "PHP"

    --8<-- "snippets/php/usage/moderate.md"

=== "Elixir"

    --8<-- "snippets/elixir/usage/moderate.md"

=== "WASM"

    --8<-- "snippets/wasm/usage/moderate.md"

### Moderation Parameters

| Parameter | Type | Description |
| --- | --- | --- |
| `input` | string/array | Content to classify |
| `model` | string | Moderation model (e.g. `"openai/omni-moderation-latest"`) |
