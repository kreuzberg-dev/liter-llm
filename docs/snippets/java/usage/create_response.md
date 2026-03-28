<!-- snippet:compile-only -->

```java
import dev.kreuzberg.literllm.LlmClient;
import dev.kreuzberg.literllm.Types.*;

public class Main {
    public static void main(String[] args) throws Exception {
        try (var client = LlmClient.builder()
                .apiKey(System.getenv("OPENAI_API_KEY"))
                .build()) {
            var response = client.createResponse(new CreateResponseRequest(
                "openai/gpt-4o",
                "Explain quantum computing in one sentence."
            ));
            System.out.println(response);
        }
    }
}
```
