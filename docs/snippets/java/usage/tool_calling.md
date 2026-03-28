```java
import dev.kreuzberg.literllm.LlmClient;
import dev.kreuzberg.literllm.Types.*;
import java.util.List;
import java.util.Map;

public class Main {
    public static void main(String[] args) throws Exception {
        try (var client = LlmClient.builder()
                .apiKey(System.getenv("OPENAI_API_KEY"))
                .build()) {
            var tools = List.of(new Tool(
                "function",
                new FunctionDefinition(
                    "get_weather",
                    "Get the current weather for a location",
                    Map.of(
                        "type", "object",
                        "properties", Map.of(
                            "location", Map.of("type", "string", "description", "City name")
                        ),
                        "required", List.of("location")
                    )
                )
            ));

            var response = client.chat(new ChatCompletionRequest(
                "openai/gpt-4o",
                List.of(new UserMessage("What is the weather in Berlin?")),
                tools
            ));

            for (var call : response.choices().getFirst().message().toolCalls()) {
                System.out.printf("Tool: %s, Args: %s%n",
                    call.function().name(), call.function().arguments());
            }
        }
    }
}
```
