<!-- snippet:compile-only -->

```php
<?php

declare(strict_types=1);

use LiterLlm\LlmClient;

$client = new LlmClient(apiKey: getenv('OPENAI_API_KEY') ?: '');

$response = json_decode($client->imageGenerate(json_encode([
    'model' => 'openai/dall-e-3',
    'prompt' => 'A sunset over mountains',
    'n' => 1,
    'size' => '1024x1024',
])), true);

echo $response['data'][0]['url'] . PHP_EOL;
```
