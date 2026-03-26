use std::pin::Pin;
use std::task::{Context, Poll};

use bytes::Bytes;
use futures_core::Stream;
use memchr::memchr;
use pin_project_lite::pin_project;

use crate::error::{LiterLmError, Result};
use crate::http::request::with_retry;
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
///
/// `parse_event` translates a raw SSE `data:` payload string into a
/// `ChatCompletionChunk`.  Pass the provider's `parse_stream_event` method
/// to support non-OpenAI SSE formats.
#[cfg_attr(
    feature = "tracing",
    tracing::instrument(
        skip_all,
        fields(
            http.method = "POST",
            http.url = %url,
            http.status_code = tracing::field::Empty,
            http.retry_count = tracing::field::Empty,
        )
    )
)]
pub async fn post_stream<P>(
    client: &reqwest::Client,
    url: &str,
    auth_header: Option<(&str, &str)>,
    extra_headers: &[(&str, &str)],
    body: Bytes,
    max_retries: u32,
    parse_event: P,
) -> Result<Pin<Box<dyn Stream<Item = Result<ChatCompletionChunk>> + Send>>>
where
    P: Fn(&str) -> Result<Option<ChatCompletionChunk>> + Send + 'static,
{
    let mut retry_count = 0u32;

    let resp = with_retry(max_retries, || {
        // Clone is a zero-copy ref-count bump on `Bytes`.
        let mut builder = client
            .post(url)
            .header(reqwest::header::CONTENT_TYPE, "application/json")
            .body(body.clone());
        if let Some((name, value)) = auth_header {
            builder = builder.header(name, value);
        }
        for (name, value) in extra_headers {
            builder = builder.header(*name, *value);
        }
        retry_count += 1;
        builder.send()
    })
    .await?;

    #[cfg(feature = "tracing")]
    {
        let span = tracing::Span::current();
        span.record("http.status_code", resp.status().as_u16());
        span.record("http.retry_count", retry_count.saturating_sub(1));
    }

    let byte_stream = resp.bytes_stream();
    let stream = SseParser::new(byte_stream, parse_event);
    Ok(Box::pin(stream))
}

// ---------------------------------------------------------------------------
// SSE parser
// ---------------------------------------------------------------------------

pin_project! {
    /// Wraps a `bytes::Bytes` stream and yields parsed `ChatCompletionChunk`s.
    ///
    /// The `P` type parameter is the parse function used to translate a raw
    /// SSE `data:` payload string into a `ChatCompletionChunk`.  This allows
    /// non-OpenAI SSE formats (e.g. Anthropic, Vertex) to plug in their own
    /// event parsers without duplicating the byte-buffering and line-splitting
    /// logic.
    struct SseParser<S, P> {
        #[pin]
        inner: S,
        buffer: String,
        // Set to true once the inner stream is exhausted.
        done: bool,
        // Provider-supplied event parser; translates raw SSE data payloads.
        parse_event: P,
    }
}

impl<S, P> SseParser<S, P>
where
    P: Fn(&str) -> Result<Option<ChatCompletionChunk>>,
{
    fn new(inner: S, parse_event: P) -> Self {
        Self {
            inner,
            // Pre-allocate 4 KiB — a reasonable size for SSE lines to
            // reduce reallocations during the first few chunks.
            buffer: String::with_capacity(4096),
            done: false,
            parse_event,
        }
    }
}

impl<S, P> Stream for SseParser<S, P>
where
    S: Stream<Item = std::result::Result<Bytes, reqwest::Error>> + Send,
    P: Fn(&str) -> Result<Option<ChatCompletionChunk>>,
{
    type Item = Result<ChatCompletionChunk>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let mut this = self.project();

        loop {
            // --- Process any complete lines already in the buffer ---
            // Use memchr for fast newline scanning on the hot streaming path.
            if let Some(newline_pos) = memchr(b'\n', this.buffer.as_bytes()) {
                // Borrow the line slice *before* draining — zero allocation on
                // the hot path.  All decisions (empty check, prefix match, JSON
                // parse) operate on this borrowed `&str`; we drain only after
                // extracting everything we need.
                let line = this.buffer[..newline_pos].trim_end_matches('\r').trim();

                // Skip empty lines and SSE comments.
                if line.is_empty() || line.starts_with(':') {
                    this.buffer.drain(..=newline_pos);
                    continue;
                }

                if let Some(raw) = line.strip_prefix("data:") {
                    // Strip exactly one optional leading space (RFC 8895 §3.3).
                    let data = raw.strip_prefix(' ').unwrap_or(raw).trim();

                    // Handle the OpenAI `[DONE]` sentinel at the SSE parser
                    // level — this terminates the stream regardless of provider.
                    if data == "[DONE]" {
                        this.buffer.drain(..=newline_pos);
                        return Poll::Ready(None);
                    }

                    // Delegate to the provider-supplied parser.
                    // - `Ok(Some(chunk))` → yield the chunk.
                    // - `Ok(None)` → skip this event (e.g. Anthropic ping,
                    //   content_block_stop, message_stop) and continue parsing.
                    // - `Err(e)` → yield the error to the consumer.
                    let result = (this.parse_event)(data);
                    this.buffer.drain(..=newline_pos);
                    match result {
                        Ok(None) => continue,
                        Ok(Some(chunk)) => return Poll::Ready(Some(Ok(chunk))),
                        Err(e) => return Poll::Ready(Some(Err(e))),
                    }
                }

                // Ignore other SSE fields (event:, id:, retry:).
                this.buffer.drain(..=newline_pos);
                continue;
            }

            // --- Buffer is empty or has only a partial line; fetch more bytes ---

            if *this.done {
                // Any bytes remaining in the buffer after the stream ends were
                // not terminated by a newline — they form an incomplete SSE
                // line that would be silently dropped.  Emit a warning so that
                // protocol bugs or truncated responses are visible in logs.
                if !this.buffer.is_empty() {
                    #[cfg(feature = "tracing")]
                    tracing::warn!(
                        leftover_bytes = this.buffer.len(),
                        preview = &this.buffer[..this.buffer.len().min(64)],
                        "SSE stream ended with unterminated data in buffer; dropping partial line"
                    );
                    this.buffer.clear();
                }
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
                            // Mark done so the next poll does not try to read
                            // more data from the (now-corrupt) stream.
                            *this.done = true;
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
