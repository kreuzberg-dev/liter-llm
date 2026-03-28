<!-- snippet:compile-only -->

```php
<?php

declare(strict_types=1);

use LiterLlm\LlmClient;

$client = new LlmClient(apiKey: getenv('OPENAI_API_KEY') ?: '');

$audioBytes = file_get_contents('audio.mp3');
$response = json_decode($client->transcribe(json_encode([
    'model' => 'openai/whisper-1',
    'filename' => 'audio.mp3',
]), $audioBytes), true);

echo $response['text'] . PHP_EOL;
```
