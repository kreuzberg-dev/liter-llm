<!-- snippet:compile-only -->

```java
import dev.kreuzberg.literllm.LlmClient;
import dev.kreuzberg.literllm.Types.*;

public class Main {
    public static void main(String[] args) throws Exception {
        try (var client = LlmClient.builder()
                .apiKey(System.getenv("OPENAI_API_KEY"))
                .build()) {
            var response = client.moderate(new CreateModerationRequest(
                "openai/omni-moderation-latest",
                "This is a test message."
            ));
            var result = response.results().getFirst();
            System.out.println("Flagged: " + result.flagged());
            result.categories().forEach((category, flagged) -> {
                if (flagged) {
                    System.out.printf("  %s: %.4f%n",
                        category, result.categoryScores().get(category));
                }
            });
        }
    }
}
```
