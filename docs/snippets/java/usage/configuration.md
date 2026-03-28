```java
import dev.kreuzberg.literllm.LlmClient;
import dev.kreuzberg.literllm.Types.*;
import java.util.List;

public class Main {
    public static void main(String[] args) throws Exception {
        try (var client = LlmClient.builder()
                .apiKey("sk-...")                           // or System.getenv("OPENAI_API_KEY")
                .baseUrl("https://api.openai.com/v1")       // override provider base URL
                .modelHint("openai")                        // pre-resolve provider at construction
                .maxRetries(3)                              // retry on transient failures
                .timeoutSecs(60)                            // request timeout in seconds
                .build()) {
            var response = client.chat(new ChatCompletionRequest(
                "openai/gpt-4o",
                List.of(new UserMessage("Hello!"))
            ));
            System.out.println(response.choices().getFirst().message().content());
        }
    }
}
```
