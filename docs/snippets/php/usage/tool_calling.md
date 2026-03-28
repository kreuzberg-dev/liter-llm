```php
<?php

declare(strict_types=1);

use LiterLlm\LlmClient;

$client = new LlmClient(apiKey: getenv('OPENAI_API_KEY') ?: '');

$tools = [
    [
        'type' => 'function',
        'function' => [
            'name' => 'get_weather',
            'description' => 'Get the current weather for a location',
            'parameters' => [
                'type' => 'object',
                'properties' => [
                    'location' => ['type' => 'string', 'description' => 'City name'],
                ],
                'required' => ['location'],
            ],
        ],
    ],
];

$response = json_decode($client->chat(json_encode([
    'model' => 'openai/gpt-4o',
    'messages' => [
        ['role' => 'user', 'content' => 'What is the weather in Berlin?'],
    ],
    'tools' => $tools,
])), true);

foreach ($response['choices'][0]['message']['tool_calls'] ?? [] as $call) {
    echo "Tool: {$call['function']['name']}, Args: {$call['function']['arguments']}" . PHP_EOL;
}
```
