<!-- snippet:compile-only -->

```php
<?php

declare(strict_types=1);

use LiterLlm\LlmClient;

$client = new LlmClient(apiKey: getenv('OPENAI_API_KEY') ?: '');

$response = json_decode($client->moderate(json_encode([
    'model' => 'openai/omni-moderation-latest',
    'input' => 'This is a test message.',
])), true);

$result = $response['results'][0];
echo "Flagged: " . ($result['flagged'] ? 'true' : 'false') . PHP_EOL;
foreach ($result['categories'] as $category => $flagged) {
    if ($flagged) {
        echo "  {$category}: " . number_format($result['category_scores'][$category], 4) . PHP_EOL;
    }
}
```
