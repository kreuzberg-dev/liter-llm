<!-- snippet:compile-only -->

```java
import dev.kreuzberg.literllm.LlmClient;
import dev.kreuzberg.literllm.Types.*;

public class Main {
    public static void main(String[] args) throws Exception {
        try (var client = LlmClient.builder()
                .apiKey(System.getenv("MISTRAL_API_KEY"))
                .build()) {
            var response = client.ocr(new OcrRequest(
                "mistral/mistral-ocr-latest",
                new DocumentInput("document_url", "https://example.com/invoice.pdf")
            ));
            for (var page : response.pages()) {
                System.out.printf("Page %d: %.100s...%n",
                    page.index(), page.markdown());
            }
        }
    }
}
```
