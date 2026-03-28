<!-- snippet:compile-only -->

```php
<?php

declare(strict_types=1);

use LiterLlm\LlmClient;

$client = new LlmClient(apiKey: getenv('OPENAI_API_KEY') ?: '');

$response = json_decode($client->createBatch(json_encode([
    'input_file_id' => 'file-abc123',
    'endpoint' => '/v1/chat/completions',
    'completion_window' => '24h',
])), true);

echo "Batch ID: {$response['id']}" . PHP_EOL;
echo "Status: {$response['status']}" . PHP_EOL;
```
