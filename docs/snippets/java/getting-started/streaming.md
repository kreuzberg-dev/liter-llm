```java
import dev.kreuzberg.literllm.LlmClient;
import dev.kreuzberg.literllm.Types.*;
import java.util.List;

public class Main {
    public static void main(String[] args) throws Exception {
        // Note: The Java client does not yet support streaming.
        // Use the non-streaming chat method instead.
        try (var client = LlmClient.builder()
                .apiKey(System.getenv("OPENAI_API_KEY"))
                .build()) {
            var response = client.chat(new ChatCompletionRequest(
                "openai/gpt-4o",
                List.of(new UserMessage("Tell me a story"))
            ));
            System.out.println(response.choices().getFirst().message().content());
        }
    }
}
```
