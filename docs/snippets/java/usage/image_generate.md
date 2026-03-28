<!-- snippet:compile-only -->

```java
import dev.kreuzberg.literllm.LlmClient;
import dev.kreuzberg.literllm.Types.*;

public class Main {
    public static void main(String[] args) throws Exception {
        try (var client = LlmClient.builder()
                .apiKey(System.getenv("OPENAI_API_KEY"))
                .build()) {
            var response = client.imageGenerate(new CreateImageRequest(
                "openai/dall-e-3",
                "A sunset over mountains",
                1,
                "1024x1024"
            ));
            System.out.println(response.data().getFirst().url());
        }
    }
}
```
