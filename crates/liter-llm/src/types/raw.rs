/// The raw request and response JSON exchanged with the provider,
/// paired with the typed (normalized) response.
///
/// Returned by every `_raw` method on [`crate::LlmClientRaw`]. Useful for
/// debugging provider-specific transformations or implementing custom parsing.
#[derive(Debug, Clone)]
pub struct RawExchange<T> {
    /// The typed, normalized response.
    pub data: T,
    /// The final request body sent to the provider (after `transform_request`).
    pub raw_request: serde_json::Value,
    /// The raw response body from the provider, before `transform_response`.
    /// `None` for binary endpoints (speech) or when not applicable.
    pub raw_response: Option<serde_json::Value>,
}

/// Raw exchange data for streaming responses.
///
/// Returned by [`crate::LlmClientRaw::chat_stream_raw`]. The stream itself is
/// not captured in its entirety — only the request body is available upfront.
/// `RawStreamExchange` intentionally does not implement `Clone` because streams
/// cannot be duplicated.
#[derive(Debug)]
pub struct RawStreamExchange<S> {
    /// The chunk stream, unchanged.
    pub stream: S,
    /// The final request body sent to the provider.
    pub raw_request: serde_json::Value,
}
