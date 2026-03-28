```php
<?php

declare(strict_types=1);

use LiterLlm\LlmClient;

$client = new LlmClient(apiKey: getenv('OPENAI_API_KEY') ?: '');

$messages = [
    ['role' => 'system', 'content' => 'You are a helpful assistant.'],
    ['role' => 'user', 'content' => 'What is the capital of France?'],
];

$response = json_decode($client->chat(json_encode([
    'model' => 'openai/gpt-4o',
    'messages' => $messages,
])), true);
$content = $response['choices'][0]['message']['content'];
echo "Assistant: {$content}" . PHP_EOL;

// Continue the conversation
$messages[] = ['role' => 'assistant', 'content' => $content];
$messages[] = ['role' => 'user', 'content' => 'What about Germany?'];

$response = json_decode($client->chat(json_encode([
    'model' => 'openai/gpt-4o',
    'messages' => $messages,
])), true);
echo "Assistant: {$response['choices'][0]['message']['content']}" . PHP_EOL;

// Token usage
if (isset($response['usage'])) {
    echo "Tokens: {$response['usage']['prompt_tokens']} in, {$response['usage']['completion_tokens']} out" . PHP_EOL;
}
```
