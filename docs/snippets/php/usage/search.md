<!-- snippet:compile-only -->

```php
<?php

declare(strict_types=1);

use LiterLlm\LlmClient;

$client = new LlmClient(apiKey: getenv('BRAVE_API_KEY') ?: '');

$response = json_decode($client->search(json_encode([
    'model' => 'brave/web-search',
    'query' => 'What is Rust programming language?',
    'max_results' => 5,
])), true);

foreach ($response['results'] as $result) {
    echo "{$result['title']}: {$result['url']}" . PHP_EOL;
}
```
