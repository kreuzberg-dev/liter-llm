```java
import dev.kreuzberg.literllm.LlmClient;
import dev.kreuzberg.literllm.ChatRequest;
import dev.kreuzberg.literllm.Message;

public class Main {
    public static void main(String[] args) throws Exception {
        try (var client = new LlmClient()) {
            client.chatStream(
                ChatRequest.builder()
                    .model("openai/gpt-4o")
                    .message(new Message("user", "Tell me a story"))
                    .build(),
                chunk -> System.out.print(chunk.getDelta())
            );
            System.out.println();
        }
    }
}
```
