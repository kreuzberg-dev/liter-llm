```php
<?php

declare(strict_types=1);

use LiterLlm\LlmClient;

$client = new LlmClient(apiKey: getenv('OPENAI_API_KEY') ?: '');

$response = json_decode($client->chat(json_encode([
    'model' => 'openai/gpt-4o',
    'messages' => [
        ['role' => 'user', 'content' => 'Hello!'],
    ],
])), true);

echo $response['choices'][0]['message']['content'] . PHP_EOL;
```
