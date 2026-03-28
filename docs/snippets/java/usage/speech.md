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
            byte[] audioBytes = client.speech(new CreateSpeechRequest(
                "openai/tts-1",
                "Hello, world!",
                "alloy"
            ));
            Files.write(Path.of("output.mp3"), audioBytes);
            System.out.printf("Wrote %d bytes to output.mp3%n", audioBytes.length);
        }
    }
}
```
