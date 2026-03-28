<!-- snippet:compile-only -->

```php
<?php

declare(strict_types=1);

use LiterLlm\LlmClient;

$client = new LlmClient(apiKey: getenv('OPENAI_API_KEY') ?: '');

$response = json_decode($client->rerank(json_encode([
    'model' => 'cohere/rerank-v3.5',
    'query' => 'What is the capital of France?',
    'documents' => [
        'Paris is the capital of France.',
        'Berlin is the capital of Germany.',
        'London is the capital of England.',
    ],
])), true);

foreach ($response['results'] as $result) {
    echo "Index: {$result['index']}, Score: " . number_format($result['relevance_score'], 4) . PHP_EOL;
}
```
