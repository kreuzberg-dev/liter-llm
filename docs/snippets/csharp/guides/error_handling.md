```csharp
using System;
using System.Threading.Tasks;
using LiterLlm;

class Program
{
    static async Task Main()
    {
        await using var client = new LlmClient(
            apiKey: Environment.GetEnvironmentVariable("OPENAI_API_KEY")!
        );

        try
        {
            var response = await client.ChatAsync(new ChatCompletionRequest(
                model: "openai/gpt-4o",
                messages: new[] { new UserMessage("Hello") }
            ));
            Console.WriteLine(response.Choices[0].Message.Content);
        }
        catch (AuthenticationException e)
        {
            // 401/403 — rotate the key.
            Console.Error.WriteLine($"auth failed: {e.Message}");
        }
        catch (RateLimitException e)
        {
            // 429 — transient, retry with backoff.
            Console.Error.WriteLine($"rate limited: {e.Message}");
        }
        catch (BudgetExceededException e)
        {
            Console.Error.WriteLine($"budget exceeded: {e.Message}");
        }
        catch (ProviderException e)
        {
            Console.Error.WriteLine($"provider error: {e.Message}");
        }
        catch (LlmException e)
        {
            // Catch-all for the remaining liter-llm errors.
            Console.Error.WriteLine($"llm error ({e.ErrorCode}): {e.Message}");
        }
    }
}
```
