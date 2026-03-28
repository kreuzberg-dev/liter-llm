<!-- snippet:compile-only -->

```csharp
using LiterLlm;

await using var client = new LlmClient(
    apiKey: Environment.GetEnvironmentVariable("OPENAI_API_KEY")!);

var audioBytes = await File.ReadAllBytesAsync("audio.mp3");
var response = await client.TranscribeAsync(new CreateTranscriptionRequest(
    Model: "openai/whisper-1",
    File: audioBytes,
    Filename: "audio.mp3"
));
Console.WriteLine(response.Text);
```
