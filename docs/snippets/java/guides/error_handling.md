```java
import java.util.List;

import dev.kreuzberg.literllm.LlmClient;
import dev.kreuzberg.literllm.LlmException;
import dev.kreuzberg.literllm.BudgetExceededException;
import dev.kreuzberg.literllm.types.ChatCompletionRequest;
import dev.kreuzberg.literllm.types.Types;

public class ErrorHandling {
    public static void main(String[] args) {
        try (var client = LlmClient.builder().apiKey(System.getenv("OPENAI_API_KEY")).build()) {
            var response = client.chat(new ChatCompletionRequest(
                "openai/gpt-4o",
                List.of(new Types.UserMessage("Hello"))
            ));
            System.out.println(response.choices().get(0).message().content());
        } catch (LlmException.AuthenticationException e) {
            // 401/403 — rotate the key.
            System.err.println("auth failed: " + e.getMessage());
        } catch (LlmException.RateLimitException e) {
            // 429 — transient, retry with backoff.
            System.err.println("rate limited: " + e.getMessage());
        } catch (BudgetExceededException e) {
            System.err.println("budget exceeded: " + e.getMessage());
        } catch (LlmException.ProviderException e) {
            // 5xx — inspect getHttpStatus() to decide next step.
            System.err.printf("provider %d: %s%n", e.getHttpStatus(), e.getMessage());
        } catch (LlmException e) {
            // Catch-all for the remaining liter-llm errors.
            System.err.println("llm error (" + e.getErrorCode() + "): " + e.getMessage());
        } catch (Exception e) {
            System.err.println("unexpected: " + e.getMessage());
        }
    }
}
```
