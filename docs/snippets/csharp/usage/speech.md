<!-- snippet:compile-only -->

```csharp
using LiterLlm;

await using var client = new LlmClient(
    apiKey: Environment.GetEnvironmentVariable("OPENAI_API_KEY")!);

var audioBytes = await client.SpeechAsync(new CreateSpeechRequest(
    Model: "openai/tts-1",
    Input: "Hello, world!",
    Voice: "alloy"
));
await File.WriteAllBytesAsync("output.mp3", audioBytes);
Console.WriteLine($"Wrote {audioBytes.Length} bytes to output.mp3");
```
