<!-- snippet:compile-only -->

```php
<?php

declare(strict_types=1);

use LiterLlm\LlmClient;

$client = new LlmClient(apiKey: getenv('OPENAI_API_KEY') ?: '');

$audioBytes = $client->speech(json_encode([
    'model' => 'openai/tts-1',
    'input' => 'Hello, world!',
    'voice' => 'alloy',
]));

file_put_contents('output.mp3', $audioBytes);
echo 'Wrote ' . strlen($audioBytes) . ' bytes to output.mp3' . PHP_EOL;
```
