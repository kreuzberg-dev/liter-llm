```java
import dev.kreuzberg.literllm.LlmClient;
import dev.kreuzberg.literllm.ChatRequest;
import dev.kreuzberg.literllm.Message;

public class Main {
    public static void main(String[] args) throws Exception {
        try (var client = new LlmClient()) {
            var response = client.chat(ChatRequest.builder()
                .model("openai/gpt-4o")
                .message(new Message("user", "Hello!"))
                .build());
            System.out.println(response.getContent());
        }
    }
}
```
