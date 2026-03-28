```php
<?php

declare(strict_types=1);

use LiterLlm\LlmClient;

$client = new LlmClient(apiKey: getenv('OPENAI_API_KEY') ?: '');

$chunks = json_decode($client->chatStream(json_encode([
    'model' => 'openai/gpt-4o',
    'messages' => [
        ['role' => 'user', 'content' => 'Explain quantum computing briefly'],
    ],
])), true);

$fullText = '';
foreach ($chunks as $chunk) {
    $delta = $chunk['choices'][0]['delta']['content'] ?? null;
    if ($delta !== null) {
        $fullText .= $delta;
        echo $delta;
    }
}
echo PHP_EOL;
echo "\nFull response length: " . strlen($fullText) . " characters" . PHP_EOL;
```
