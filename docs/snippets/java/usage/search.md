<!-- snippet:compile-only -->

```java
import dev.kreuzberg.literllm.LlmClient;
import dev.kreuzberg.literllm.Types.*;

public class Main {
    public static void main(String[] args) throws Exception {
        try (var client = LlmClient.builder()
                .apiKey(System.getenv("BRAVE_API_KEY"))
                .build()) {
            var response = client.search(new SearchRequest(
                "brave/web-search",
                "What is Rust programming language?",
                5
            ));
            for (var result : response.results()) {
                System.out.printf("%s: %s%n", result.title(), result.url());
            }
        }
    }
}
```
