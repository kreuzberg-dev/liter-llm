use std::pin::Pin;
use std::task::{Context, Poll};

use bytes::Bytes;
use futures_core::Stream;
use memchr::memchr;
use pin_project_lite::pin_project;

use crate::error::{LiterLmError, Result};
use crate::http::request::retry_after_from_response;
use crate::http::retry;
use crate::types::ChatCompletionChunk;

/// Maximum number of bytes buffered before declaring a streaming error.
const MAX_BUFFER_BYTES: usize = 1024 * 1024; // 1 MiB

// ---------------------------------------------------------------------------
// Public entry point
// ---------------------------------------------------------------------------

/// Send a streaming POST request and return an SSE stream of
/// `ChatCompletionChunk`s.
///
/// Before opening the stream, retries on 429 / 500 / 502 / 503 / 504 up to
/// `max_retries` times honouring any `Retry-After` header.  Once the stream
/// is open, individual chunk errors are yielded as `Err` items rather than
/// causing a retry.
///
/// `auth_header` is `Some((name, value))` when the provider requires
/// authentication, or `None` when no auth header should be added.
///
/// `extra_headers` carries provider-specific mandatory headers (e.g.
/// `anthropic-version`) beyond the single auth header.
pub async fn post_stream(
    client: &reqwest::Client,
    url: &str,
    auth_header: Option<(&str, &str)>,
    extra_headers: &[(&str, &str)],
    body: serde_json::Value,
    max_retries: u32,
) -> Result<Pin<Box<dyn Stream<Item = Result<ChatCompletionChunk>> + Send>>> {
    let mut attempt = 0u32;

    loop {
        let mut builder = client
            .post(url)
            .header(reqwest::header::CONTENT_TYPE, "application/json")
            .json(&body);
        if let Some((name, value)) = auth_header {
            builder = builder.header(name, value);
        }
        for (name, value) in extra_headers {
            builder = builder.header(*name, *value);
        }
        let resp = builder.send().await?;

        let status = resp.status().as_u16();

        if resp.status().is_success() {
            let byte_stream = resp.bytes_stream();
            let stream = SseParser::new(byte_stream);
            return Ok(Box::pin(stream));
        }

        // Parse Retry-After before consuming the body.
        let server_retry_after = retry_after_from_response(&resp);

        if let Some(delay) = retry::should_retry(status, attempt, max_retries, server_retry_after) {
            attempt += 1;
            tokio::time::sleep(delay).await;
            continue;
        }

        let text = resp
            .text()
            .await
            .unwrap_or_else(|e| format!("(failed to read body: {e})"));
        return Err(LiterLmError::from_status(status, &text, server_retry_after));
    }
}

// ---------------------------------------------------------------------------
// SSE parser
// ---------------------------------------------------------------------------

pin_project! {
    /// Wraps a `bytes::Bytes` stream and yields parsed `ChatCompletionChunk`s.
    struct SseParser<S> {
        #[pin]
        inner: S,
        buffer: String,
        // Set to true once the inner stream is exhausted.
        done: bool,
    }
}

impl<S> SseParser<S> {
    fn new(inner: S) -> Self {
        Self {
            inner,
            buffer: String::new(),
            done: false,
        }
    }
}

impl<S> Stream for SseParser<S>
where
    S: Stream<Item = std::result::Result<Bytes, reqwest::Error>> + Send,
{
    type Item = Result<ChatCompletionChunk>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let mut this = self.project();

        loop {
            // --- Process any complete lines already in the buffer ---
            // Use memchr for fast newline scanning on the hot streaming path.
            if let Some(newline_pos) = memchr(b'\n', this.buffer.as_bytes()) {
                // Read the line without allocating: slice up to the newline,
                // strip optional trailing '\r' (CRLF), and trim whitespace.
                // We allocate only the final trimmed string, then drain the
                // buffer past the newline.
                let line = this.buffer[..newline_pos].trim_end_matches('\r').trim().to_string();
                // Advance the buffer past the '\n'.
                this.buffer.drain(..=newline_pos);

                // Skip empty lines and SSE comments.
                if line.is_empty() || line.starts_with(':') {
                    continue;
                }

                if let Some(raw) = line.strip_prefix("data:") {
                    // Strip exactly one optional leading space (RFC 8895 §3.3).
                    let data = raw.strip_prefix(' ').unwrap_or(raw).trim();
                    if data == "[DONE]" {
                        return Poll::Ready(None);
                    }
                    return Poll::Ready(Some(serde_json::from_str::<ChatCompletionChunk>(data).map_err(|e| {
                        LiterLmError::Streaming {
                            message: format!("failed to parse SSE data: {e}"),
                        }
                    })));
                }

                // Ignore other SSE fields (event:, id:, retry:).
                continue;
            }

            // --- Buffer is empty or has only a partial line; fetch more bytes ---

            if *this.done {
                // Drain any remaining buffered text without a newline.
                // Real SSE streams always end cleanly, so just signal end.
                return Poll::Ready(None);
            }

            match this.inner.as_mut().poll_next(cx) {
                Poll::Ready(Some(Ok(bytes))) => {
                    // Guard against unbounded growth.
                    if this.buffer.len() + bytes.len() > MAX_BUFFER_BYTES {
                        // Mark done so subsequent polls don't continue reading.
                        *this.done = true;
                        return Poll::Ready(Some(Err(LiterLmError::Streaming {
                            message: format!("SSE buffer exceeded {MAX_BUFFER_BYTES} bytes; stream aborted"),
                        })));
                    }
                    match std::str::from_utf8(&bytes) {
                        Ok(s) => this.buffer.push_str(s),
                        Err(e) => {
                            return Poll::Ready(Some(Err(LiterLmError::Streaming {
                                message: format!("invalid UTF-8 in SSE stream: {e}"),
                            })));
                        }
                    }
                }
                Poll::Ready(Some(Err(e))) => {
                    return Poll::Ready(Some(Err(LiterLmError::from(e))));
                }
                Poll::Ready(None) => {
                    *this.done = true;
                    // Loop once more to flush any remaining buffered line.
                    continue;
                }
                Poll::Pending => {
                    return Poll::Pending;
                }
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Utility
// ---------------------------------------------------------------------------

/// Parse a single SSE `data:` line into a `ChatCompletionChunk`.
///
/// Returns `None` for the terminal `[DONE]` sentinel.
///
/// Only used in crate-internal tests; external consumers should use the
/// streaming API instead.
#[cfg(test)]
pub(crate) fn parse_sse_line(line: &str) -> Option<Result<ChatCompletionChunk>> {
    // Strip "data:" then optionally one leading space (RFC 8895 §3.3).
    let raw = line.strip_prefix("data:")?;
    let data = raw.strip_prefix(' ').unwrap_or(raw).trim();
    if data == "[DONE]" {
        return None;
    }
    Some(serde_json::from_str(data).map_err(|e| LiterLmError::Streaming {
        message: format!("failed to parse SSE data: {e}"),
    }))
}
