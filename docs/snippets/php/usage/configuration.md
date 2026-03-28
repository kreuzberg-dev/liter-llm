```php
<?php

declare(strict_types=1);

use LiterLlm\LlmClient;

$client = new LlmClient(
    apiKey: 'sk-...',                                 // or getenv('OPENAI_API_KEY')
    baseUrl: 'https://api.openai.com/v1',             // override provider base URL
    modelHint: 'openai',                              // pre-resolve provider at construction
    maxRetries: 3,                                    // retry on transient failures
    timeoutSecs: 60                                   // request timeout in seconds
);

$response = json_decode($client->chat(json_encode([
    'model' => 'openai/gpt-4o',
    'messages' => [
        ['role' => 'user', 'content' => 'Hello!'],
    ],
])), true);

echo $response['choices'][0]['message']['content'] . PHP_EOL;
```
