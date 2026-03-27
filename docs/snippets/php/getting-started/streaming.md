```php
<?php

declare(strict_types=1);

use LiterLlm\LlmClient;

$client = new LlmClient(apiKey: getenv('OPENAI_API_KEY') ?: '');

$chunksJson = $client->chatStream(json_encode([
    'model' => 'openai/gpt-4o',
    'messages' => [
        ['role' => 'user', 'content' => 'Tell me a story'],
    ],
]));

$chunks = json_decode($chunksJson, true);
foreach ($chunks as $chunk) {
    echo $chunk['choices'][0]['delta']['content'] ?? '';
}
echo PHP_EOL;
```
