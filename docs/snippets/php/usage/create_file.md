<!-- snippet:compile-only -->

```php
<?php

declare(strict_types=1);

use LiterLlm\LlmClient;

$client = new LlmClient(apiKey: getenv('OPENAI_API_KEY') ?: '');

$fileBytes = file_get_contents('data.jsonl');
$response = json_decode($client->createFile(json_encode([
    'filename' => 'data.jsonl',
    'purpose' => 'batch',
]), $fileBytes), true);

echo "File ID: {$response['id']}" . PHP_EOL;
echo "Size: {$response['bytes']} bytes" . PHP_EOL;
```
