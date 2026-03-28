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
            byte[] fileBytes = Files.readAllBytes(Path.of("data.jsonl"));
            var response = client.createFile(new CreateFileRequest(
                fileBytes,
                "data.jsonl",
                "batch"
            ));
            System.out.println("File ID: " + response.id());
            System.out.println("Size: " + response.bytes() + " bytes");
        }
    }
}
```
