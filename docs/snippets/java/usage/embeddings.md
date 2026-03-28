```java
import dev.kreuzberg.literllm.LlmClient;
import dev.kreuzberg.literllm.Types.*;
import java.util.List;

public class Main {
    public static void main(String[] args) throws Exception {
        try (var client = LlmClient.builder()
                .apiKey(System.getenv("OPENAI_API_KEY"))
                .build()) {
            var response = client.embed(new EmbeddingRequest(
                "openai/text-embedding-3-small",
                List.of("The quick brown fox jumps over the lazy dog")
            ));
            var embedding = response.data().getFirst().embedding();
            System.out.println("Dimensions: " + embedding.size());
            System.out.println("First 5 values: " + embedding.subList(0, 5));
        }
    }
}
```
