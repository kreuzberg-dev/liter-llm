<!-- snippet:compile-only -->

```java
import dev.kreuzberg.literllm.LlmClient;
import dev.kreuzberg.literllm.Types.*;
import java.nio.file.Files;
import java.nio.file.Path;

public class Main {
    public static void main(String[] args) throws Exception {
        try (var client = LlmClient.builder()
                .apiKey(System.getenv("OPENAI_API_KEY"))
                .build()) {
            byte[] audioBytes = Files.readAllBytes(Path.of("audio.mp3"));
            var response = client.transcribe(new CreateTranscriptionRequest(
                "openai/whisper-1",
                audioBytes,
                "audio.mp3"
            ));
            System.out.println(response.text());
        }
    }
}
```
