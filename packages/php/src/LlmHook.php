<?php

declare(strict_types=1);

namespace LiterLlm;

/**
 * Lifecycle hook interface for intercepting LLM client request/response events.
 *
 * Implement this interface and register it via {@see LlmClient::addHook()}.
 * Hooks are invoked in registration order, synchronously.
 */
interface LlmHook
{
    /**
     * Called before the HTTP request is sent.
     *
     * Throw {@see HookRejectedException} to abort the request.
     *
     * @param mixed $request The request data about to be sent.
     *
     * @throws HookRejectedException To reject the request.
     */
    public function onRequest(mixed $request): void;

    /**
     * Called after a successful response is received.
     *
     * @param mixed $request  The original request.
     * @param mixed $response The response received from the provider.
     */
    public function onResponse(mixed $request, mixed $response): void;

    /**
     * Called after a request fails.
     *
     * @param mixed      $request The original request.
     * @param \Throwable $error   The exception that caused the failure.
     */
    public function onError(mixed $request, \Throwable $error): void;
}
