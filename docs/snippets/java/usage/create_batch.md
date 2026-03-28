<!-- snippet:compile-only -->

```java
import dev.kreuzberg.literllm.LlmClient;
import dev.kreuzberg.literllm.Types.*;

public class Main {
    public static void main(String[] args) throws Exception {
        try (var client = LlmClient.builder()
                .apiKey(System.getenv("OPENAI_API_KEY"))
                .build()) {
            var response = client.createBatch(new CreateBatchRequest(
                "file-abc123",
                "/v1/chat/completions",
                "24h"
            ));
            System.out.println("Batch ID: " + response.id());
            System.out.println("Status: " + response.status());
        }
    }
}
```
