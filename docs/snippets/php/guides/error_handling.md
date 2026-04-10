```php
<?php

declare(strict_types=1);

use LiterLlm\LlmClient;
use LiterLlm\BudgetExceededException;
use LiterLlm\HookRejectedException;

$client = new LlmClient(apiKey: getenv('OPENAI_API_KEY'));

$request = [
    'model' => 'openai/gpt-4o',
    'messages' => [['role' => 'user', 'content' => 'Hello']],
];

try {
    $response = json_decode($client->chat(json_encode($request)), true);
    echo $response['choices'][0]['message']['content'] . PHP_EOL;
} catch (BudgetExceededException $e) {
    fwrite(STDERR, "budget exceeded: {$e->getMessage()}\n");
} catch (HookRejectedException $e) {
    fwrite(STDERR, "hook rejected: {$e->getMessage()}\n");
} catch (\RuntimeException $e) {
    // All other liter-llm errors surface as plain RuntimeException.
    // Branch on the provider message text.
    $msg = $e->getMessage();
    if (stripos($msg, 'authentication') !== false) {
        fwrite(STDERR, "auth failed: $msg\n");
    } elseif (stripos($msg, 'rate limit') !== false) {
        fwrite(STDERR, "rate limited: $msg\n");
    } else {
        fwrite(STDERR, "llm error: $msg\n");
    }
}
```
