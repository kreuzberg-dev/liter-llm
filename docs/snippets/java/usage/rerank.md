<!-- snippet:compile-only -->

```java
import dev.kreuzberg.literllm.LlmClient;
import dev.kreuzberg.literllm.Types.*;
import java.util.List;

public class Main {
    public static void main(String[] args) throws Exception {
        try (var client = LlmClient.builder()
                .apiKey(System.getenv("OPENAI_API_KEY"))
                .build()) {
            var response = client.rerank(new RerankRequest(
                "cohere/rerank-v3.5",
                "What is the capital of France?",
                List.of(
                    "Paris is the capital of France.",
                    "Berlin is the capital of Germany.",
                    "London is the capital of England."
                )
            ));
            for (var result : response.results()) {
                System.out.printf("Index: %d, Score: %.4f%n",
                    result.index(), result.relevanceScore());
            }
        }
    }
}
```
