```php
<?php

declare(strict_types=1);

use LiterLlm\LlmClient;

$client = new LlmClient(apiKey: getenv('OPENAI_API_KEY') ?: '');

$response = json_decode($client->embed(json_encode([
    'model' => 'openai/text-embedding-3-small',
    'input' => ['The quick brown fox jumps over the lazy dog'],
])), true);

$embedding = $response['data'][0]['embedding'];
echo 'Dimensions: ' . count($embedding) . PHP_EOL;
echo 'First 5 values: ' . json_encode(array_slice($embedding, 0, 5)) . PHP_EOL;
```
