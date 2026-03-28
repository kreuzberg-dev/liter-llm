<!-- snippet:compile-only -->

```php
<?php

declare(strict_types=1);

use LiterLlm\LlmClient;

$client = new LlmClient(apiKey: getenv('OPENAI_API_KEY') ?: '');

$response = json_decode($client->createResponse(json_encode([
    'model' => 'openai/gpt-4o',
    'input' => 'Explain quantum computing in one sentence.',
])), true);

print_r($response);
```
