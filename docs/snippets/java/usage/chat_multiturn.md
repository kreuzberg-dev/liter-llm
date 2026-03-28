```java
import dev.kreuzberg.literllm.LlmClient;
import dev.kreuzberg.literllm.Types.*;
import java.util.ArrayList;
import java.util.List;

public class Main {
    public static void main(String[] args) throws Exception {
        try (var client = LlmClient.builder()
                .apiKey(System.getenv("OPENAI_API_KEY"))
                .build()) {
            var messages = new ArrayList<>(List.of(
                new SystemMessage("You are a helpful assistant."),
                new UserMessage("What is the capital of France?")
            ));

            var response = client.chat(new ChatCompletionRequest(
                "openai/gpt-4o", messages
            ));
            var content = response.choices().getFirst().message().content();
            System.out.println("Assistant: " + content);

            // Continue the conversation
            messages.add(new AssistantMessage(content));
            messages.add(new UserMessage("What about Germany?"));

            response = client.chat(new ChatCompletionRequest(
                "openai/gpt-4o", messages
            ));
            System.out.println("Assistant: " + response.choices().getFirst().message().content());

            // Token usage
            var usage = response.usage();
            if (usage != null) {
                System.out.printf("Tokens: %d in, %d out%n",
                    usage.promptTokens(), usage.completionTokens());
            }
        }
    }
}
```
