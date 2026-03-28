```java
import dev.kreuzberg.literllm.LlmClient;
import dev.kreuzberg.literllm.Types.*;
import java.util.List;

public class Main {
    public static void main(String[] args) throws Exception {
        try (var client = LlmClient.builder()
                .apiKey(System.getenv("OPENAI_API_KEY"))
                .build()) {
            var sb = new StringBuilder();
            client.chatStream(new ChatCompletionRequest(
                "openai/gpt-4o",
                List.of(new UserMessage("Explain quantum computing briefly"))
            ), chunk -> {
                var delta = chunk.choices().getFirst().delta().content();
                if (delta != null) {
                    sb.append(delta);
                    System.out.print(delta);
                }
            });
            System.out.println();
            System.out.printf("%nFull response length: %d characters%n", sb.length());
        }
    }
}
```
