<!-- snippet:compile-only -->

```php
<?php

declare(strict_types=1);

use LiterLlm\LlmClient;

$client = new LlmClient(apiKey: getenv('MISTRAL_API_KEY') ?: '');

$response = json_decode($client->ocr(json_encode([
    'model' => 'mistral/mistral-ocr-latest',
    'document' => [
        'type' => 'document_url',
        'url' => 'https://example.com/invoice.pdf',
    ],
])), true);

foreach ($response['pages'] as $page) {
    echo "Page {$page['index']}: " . substr($page['markdown'], 0, 100) . "..." . PHP_EOL;
}
```
