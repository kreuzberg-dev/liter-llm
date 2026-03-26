//! AWS EventStream binary protocol parser for Bedrock ConverseStream.
//!
//! The EventStream protocol frames each message as:
//!
//! ```text
//! [total_length:4][headers_length:4][prelude_crc:4][headers:N][payload:M][message_crc:4]
//! ```
//!
//! Where lengths are big-endian `u32` values.  Each header is:
//!
//! ```text
//! [name_len:1][name:name_len][type:1][value_len:2][value:value_len]
//! ```
//!
//! This implementation focuses on correctness and zero-copy where possible.
//! CRC validation is performed to detect corrupted frames.

use std::pin::Pin;
use std::task::{Context, Poll};

use bytes::{Bytes, BytesMut};
use futures_core::Stream;
use pin_project_lite::pin_project;

use crate::error::{LiterLmError, Result};
use crate::http::request::with_retry;
use crate::types::ChatCompletionChunk;

/// Minimum frame size: prelude (12) + message CRC (4) = 16 bytes.
const MIN_FRAME_SIZE: usize = 16;

/// Maximum frame size to prevent unbounded buffering (16 MiB).
const MAX_FRAME_SIZE: usize = 16 * 1024 * 1024;

/// Header value type for UTF-8 strings.
const HEADER_TYPE_STRING: u8 = 7;

// ---------------------------------------------------------------------------
// Public entry point
// ---------------------------------------------------------------------------

/// Send a streaming POST request and return a stream of `ChatCompletionChunk`s
/// parsed from AWS EventStream binary frames.
///
/// The `parse_event` function receives `(event_type, payload_json)` for each
/// event and returns a parsed chunk or `None` for terminal events.
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
pub async fn post_eventstream<P>(
    client: &reqwest::Client,
    url: &str,
    auth_header: Option<(&str, &str)>,
    extra_headers: &[(&str, &str)],
    body: Bytes,
    max_retries: u32,
    parse_event: P,
) -> Result<Pin<Box<dyn Stream<Item = Result<ChatCompletionChunk>> + Send>>>
where
    P: Fn(&str, &str) -> Result<Option<ChatCompletionChunk>> + Send + 'static,
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
    let stream = EventStreamParser::new(byte_stream, parse_event);
    Ok(Box::pin(stream))
}

// ---------------------------------------------------------------------------
// EventStream frame parser
// ---------------------------------------------------------------------------

/// A parsed header from an EventStream frame.
struct EventHeader {
    name: String,
    value: String,
}

/// Parse headers from the header section of an EventStream frame.
///
/// Only extracts string-typed headers (type 7); other types are skipped.
fn parse_headers(mut data: &[u8]) -> Result<Vec<EventHeader>> {
    let mut headers = Vec::new();
    while !data.is_empty() {
        let name_len = data[0] as usize;
        data = &data[1..];
        if data.len() < name_len {
            return Err(LiterLmError::Streaming {
                message: "EventStream header name truncated".into(),
            });
        }
        let name = std::str::from_utf8(&data[..name_len])
            .map_err(|_| LiterLmError::Streaming {
                message: "EventStream header name is not UTF-8".into(),
            })?
            .to_owned();
        data = &data[name_len..];

        if data.is_empty() {
            return Err(LiterLmError::Streaming {
                message: "EventStream header type byte missing".into(),
            });
        }
        let value_type = data[0];
        data = &data[1..];

        if value_type == HEADER_TYPE_STRING {
            // String: 2-byte big-endian length + UTF-8 value.
            if data.len() < 2 {
                return Err(LiterLmError::Streaming {
                    message: "EventStream string header length truncated".into(),
                });
            }
            let value_len = u16::from_be_bytes([data[0], data[1]]) as usize;
            data = &data[2..];
            if data.len() < value_len {
                return Err(LiterLmError::Streaming {
                    message: "EventStream string header value truncated".into(),
                });
            }
            let value = std::str::from_utf8(&data[..value_len])
                .map_err(|_| LiterLmError::Streaming {
                    message: "EventStream header value is not UTF-8".into(),
                })?
                .to_owned();
            data = &data[value_len..];
            headers.push(EventHeader { name, value });
        } else {
            // Skip non-string header types based on their wire sizes.
            // AWS EventStream spec: bool types have no value bytes (the type
            // byte itself encodes true/false).
            let skip = match value_type {
                0 => 0, // bool_true  — no value bytes
                1 => 0, // bool_false — no value bytes
                2 => 1, // byte
                3 => 2, // short
                4 => 4, // int
                5 => 8, // long
                6 => {
                    // bytes: 2-byte length prefix
                    if data.len() < 2 {
                        return Err(LiterLmError::Streaming {
                            message: "EventStream bytes header length truncated".into(),
                        });
                    }
                    let len = u16::from_be_bytes([data[0], data[1]]) as usize;
                    2 + len
                }
                8 => 8,  // timestamp
                9 => 16, // uuid
                _ => {
                    return Err(LiterLmError::Streaming {
                        message: format!("unknown EventStream header type: {value_type}"),
                    });
                }
            };
            if data.len() < skip {
                return Err(LiterLmError::Streaming {
                    message: "EventStream header value data truncated".into(),
                });
            }
            data = &data[skip..];
        }
    }
    Ok(headers)
}

/// CRC32C (Castagnoli) implementation for EventStream frame validation.
///
/// Uses a lookup table for the CRC32C polynomial (0x82F63B78).
/// This matches the checksum algorithm used by the AWS EventStream protocol.
fn crc32c(data: &[u8]) -> u32 {
    // CRC32C lookup table (Castagnoli polynomial 0x82F63B78).
    static TABLE: [u32; 256] = {
        let mut table = [0u32; 256];
        let mut i = 0;
        while i < 256 {
            let mut crc = i as u32;
            let mut j = 0;
            while j < 8 {
                if crc & 1 != 0 {
                    crc = (crc >> 1) ^ 0x82F6_3B78;
                } else {
                    crc >>= 1;
                }
                j += 1;
            }
            table[i] = crc;
            i += 1;
        }
        table
    };

    let mut crc = 0xFFFF_FFFFu32;
    for &byte in data {
        crc = TABLE[((crc ^ u32::from(byte)) & 0xFF) as usize] ^ (crc >> 8);
    }
    crc ^ 0xFFFF_FFFF
}

// ---------------------------------------------------------------------------
// EventStream Stream adapter
// ---------------------------------------------------------------------------

pin_project! {
    /// Wraps a `bytes::Bytes` stream and yields `ChatCompletionChunk`s parsed
    /// from AWS EventStream binary frames.
    ///
    /// The `P` type parameter is the parse function that receives
    /// `(event_type, payload_json)` for each event.
    struct EventStreamParser<S, P> {
        #[pin]
        inner: S,
        buffer: BytesMut,
        done: bool,
        parse_event: P,
    }
}

impl<S, P> EventStreamParser<S, P> {
    fn new(inner: S, parse_event: P) -> Self {
        Self {
            inner,
            buffer: BytesMut::new(),
            done: false,
            parse_event,
        }
    }
}

impl<S, P> Stream for EventStreamParser<S, P>
where
    S: Stream<Item = std::result::Result<bytes::Bytes, reqwest::Error>> + Send,
    P: Fn(&str, &str) -> Result<Option<ChatCompletionChunk>>,
{
    type Item = Result<ChatCompletionChunk>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let mut this = self.project();

        loop {
            // --- Try to parse a complete frame from the buffer ---
            if this.buffer.len() >= MIN_FRAME_SIZE {
                let total_length =
                    u32::from_be_bytes([this.buffer[0], this.buffer[1], this.buffer[2], this.buffer[3]]) as usize;

                // Sanity check on frame size.
                if !(MIN_FRAME_SIZE..=MAX_FRAME_SIZE).contains(&total_length) {
                    return Poll::Ready(Some(Err(LiterLmError::Streaming {
                        message: format!(
                            "EventStream frame size {total_length} is out of range [{MIN_FRAME_SIZE}, {MAX_FRAME_SIZE}]"
                        ),
                    })));
                }

                // Wait for the full frame to be buffered.
                if this.buffer.len() < total_length {
                    // Need more data — fall through to read more bytes.
                } else {
                    // We have a complete frame. Extract and advance.
                    let frame = this.buffer.split_to(total_length);

                    let headers_length = u32::from_be_bytes([frame[4], frame[5], frame[6], frame[7]]) as usize;

                    // Validate prelude CRC (covers first 8 bytes).
                    let prelude_crc_expected = u32::from_be_bytes([frame[8], frame[9], frame[10], frame[11]]);
                    let prelude_crc_actual = crc32c(&frame[..8]);
                    if prelude_crc_expected != prelude_crc_actual {
                        return Poll::Ready(Some(Err(LiterLmError::Streaming {
                            message: format!(
                                "EventStream prelude CRC mismatch: expected {prelude_crc_expected:#010X}, got {prelude_crc_actual:#010X}"
                            ),
                        })));
                    }

                    // Validate message CRC (covers everything except the last 4 bytes).
                    let message_crc_expected = u32::from_be_bytes([
                        frame[total_length - 4],
                        frame[total_length - 3],
                        frame[total_length - 2],
                        frame[total_length - 1],
                    ]);
                    let message_crc_actual = crc32c(&frame[..total_length - 4]);
                    if message_crc_expected != message_crc_actual {
                        return Poll::Ready(Some(Err(LiterLmError::Streaming {
                            message: format!(
                                "EventStream message CRC mismatch: expected {message_crc_expected:#010X}, got {message_crc_actual:#010X}"
                            ),
                        })));
                    }

                    // Parse headers.
                    let headers_start = 12; // after prelude (total_len + headers_len + prelude_crc)
                    let headers_end = headers_start + headers_length;
                    if headers_end > total_length - 4 {
                        return Poll::Ready(Some(Err(LiterLmError::Streaming {
                            message: "EventStream headers extend past frame boundary".into(),
                        })));
                    }

                    let headers = match parse_headers(&frame[headers_start..headers_end]) {
                        Ok(h) => h,
                        Err(e) => return Poll::Ready(Some(Err(e))),
                    };

                    // Extract event-type and message-type headers.
                    let mut event_type = "";
                    let mut message_type = "";
                    for h in &headers {
                        match h.name.as_str() {
                            ":event-type" => event_type = &h.value,
                            ":message-type" => message_type = &h.value,
                            _ => {}
                        }
                    }

                    // Handle exceptions (error events from the service).
                    if message_type == "exception" {
                        let payload = &frame[headers_end..total_length - 4];
                        let payload_str = std::str::from_utf8(payload).unwrap_or("<binary>");
                        return Poll::Ready(Some(Err(LiterLmError::Streaming {
                            message: format!("Bedrock EventStream exception ({event_type}): {payload_str}"),
                        })));
                    }

                    // Only process "event" messages.
                    if message_type != "event" {
                        continue;
                    }

                    // Extract payload.
                    let payload = &frame[headers_end..total_length - 4];
                    let payload_str = match std::str::from_utf8(payload) {
                        Ok(s) => s,
                        Err(e) => {
                            return Poll::Ready(Some(Err(LiterLmError::Streaming {
                                message: format!("EventStream payload is not UTF-8: {e}"),
                            })));
                        }
                    };

                    // Delegate to the provider-supplied parser.
                    match (this.parse_event)(event_type, payload_str) {
                        Ok(None) => {
                            // Terminal event (e.g. messageStop) — stream complete.
                            // But don't end yet; there may be a metadata event after.
                            // Only truly end when the inner stream is done.
                            continue;
                        }
                        Ok(Some(chunk)) => return Poll::Ready(Some(Ok(chunk))),
                        Err(e) => return Poll::Ready(Some(Err(e))),
                    }
                }
            }

            // --- Need more bytes ---
            if *this.done {
                // If the buffer still has bytes, the stream ended mid-frame.
                if !this.buffer.is_empty() {
                    let leftover = this.buffer.len();
                    this.buffer.clear();
                    return Poll::Ready(Some(Err(LiterLmError::Streaming {
                        message: format!("EventStream ended with {leftover} bytes of incomplete frame data"),
                    })));
                }
                return Poll::Ready(None);
            }

            match this.inner.as_mut().poll_next(cx) {
                Poll::Ready(Some(Ok(bytes))) => {
                    if this.buffer.len() + bytes.len() > MAX_FRAME_SIZE {
                        *this.done = true;
                        return Poll::Ready(Some(Err(LiterLmError::Streaming {
                            message: format!("EventStream buffer exceeded {MAX_FRAME_SIZE} bytes"),
                        })));
                    }
                    this.buffer.extend_from_slice(&bytes);
                }
                Poll::Ready(Some(Err(e))) => {
                    return Poll::Ready(Some(Err(LiterLmError::from(e))));
                }
                Poll::Ready(None) => {
                    *this.done = true;
                    if this.buffer.is_empty() {
                        return Poll::Ready(None);
                    }
                    // Loop once more to try parsing any buffered frames.
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
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    /// Build a valid EventStream frame from headers and payload.
    fn build_frame(headers: &[(&str, &str)], payload: &[u8]) -> Vec<u8> {
        // Encode headers.
        let mut header_bytes = Vec::new();
        for (name, value) in headers {
            header_bytes.push(name.len() as u8);
            header_bytes.extend_from_slice(name.as_bytes());
            header_bytes.push(HEADER_TYPE_STRING); // type 7 = string
            let value_bytes = value.as_bytes();
            header_bytes.extend_from_slice(&(value_bytes.len() as u16).to_be_bytes());
            header_bytes.extend_from_slice(value_bytes);
        }

        let headers_length = header_bytes.len() as u32;
        let total_length = 12 + header_bytes.len() + payload.len() + 4;

        let mut frame = Vec::with_capacity(total_length);

        // Prelude: total_length + headers_length.
        frame.extend_from_slice(&(total_length as u32).to_be_bytes());
        frame.extend_from_slice(&headers_length.to_be_bytes());

        // Prelude CRC.
        let prelude_crc = crc32c(&frame[..8]);
        frame.extend_from_slice(&prelude_crc.to_be_bytes());

        // Headers + payload.
        frame.extend_from_slice(&header_bytes);
        frame.extend_from_slice(payload);

        // Message CRC.
        let message_crc = crc32c(&frame);
        frame.extend_from_slice(&message_crc.to_be_bytes());

        frame
    }

    #[test]
    fn crc32c_known_value() {
        // CRC32C of empty string is 0x00000000.
        assert_eq!(crc32c(b""), 0x0000_0000);
        // CRC32C of "123456789" is 0xE3069283.
        assert_eq!(crc32c(b"123456789"), 0xE306_9283);
    }

    #[test]
    fn parse_headers_basic() {
        let headers_data = {
            let mut buf = Vec::new();
            // Header: ":event-type" = "contentBlockDelta"
            let name = b":event-type";
            buf.push(name.len() as u8);
            buf.extend_from_slice(name);
            buf.push(HEADER_TYPE_STRING);
            let value = b"contentBlockDelta";
            buf.extend_from_slice(&(value.len() as u16).to_be_bytes());
            buf.extend_from_slice(value);
            buf
        };

        let headers = parse_headers(&headers_data).unwrap();
        assert_eq!(headers.len(), 1);
        assert_eq!(headers[0].name, ":event-type");
        assert_eq!(headers[0].value, "contentBlockDelta");
    }

    #[test]
    fn build_and_parse_frame() {
        let payload = br#"{"delta":{"text":"hello"}}"#;
        let frame = build_frame(
            &[
                (":message-type", "event"),
                (":event-type", "contentBlockDelta"),
                (":content-type", "application/json"),
            ],
            payload,
        );

        // Verify frame is parseable: check total_length.
        let total_length = u32::from_be_bytes([frame[0], frame[1], frame[2], frame[3]]) as usize;
        assert_eq!(total_length, frame.len());

        // Verify CRCs.
        let prelude_crc_stored = u32::from_be_bytes([frame[8], frame[9], frame[10], frame[11]]);
        assert_eq!(crc32c(&frame[..8]), prelude_crc_stored);

        let message_crc_stored = u32::from_be_bytes([
            frame[total_length - 4],
            frame[total_length - 3],
            frame[total_length - 2],
            frame[total_length - 1],
        ]);
        assert_eq!(crc32c(&frame[..total_length - 4]), message_crc_stored);
    }

    #[tokio::test]
    async fn eventstream_parser_yields_chunks() {
        use super::once_future_stream::once_future;
        use std::pin::pin;
        use std::task::{Context, Poll};

        // Build a few EventStream frames.
        let frame1 = build_frame(
            &[(":message-type", "event"), (":event-type", "contentBlockDelta")],
            br#"{"contentBlockIndex":0,"delta":{"text":"Hello"}}"#,
        );
        let frame2 = build_frame(
            &[(":message-type", "event"), (":event-type", "messageStop")],
            br#"{"stopReason":"end_turn"}"#,
        );

        // Concatenate frames into a single bytes chunk.
        let mut all_bytes = Vec::new();
        all_bytes.extend_from_slice(&frame1);
        all_bytes.extend_from_slice(&frame2);

        // Create a simple stream that yields the bytes in one chunk.
        let byte_stream = once_future(async { Ok::<_, reqwest::Error>(bytes::Bytes::from(all_bytes)) });

        // Parse function that creates a chunk for text deltas and returns None for stop.
        let parse = |event_type: &str, payload: &str| -> Result<Option<ChatCompletionChunk>> {
            match event_type {
                "contentBlockDelta" => {
                    let v: serde_json::Value = serde_json::from_str(payload)
                        .map_err(|e| LiterLmError::Streaming { message: e.to_string() })?;
                    let text = v.pointer("/delta/text").and_then(|t| t.as_str()).unwrap_or("");
                    // Build a minimal ChatCompletionChunk.
                    let chunk_json = serde_json::json!({
                        "id": "test",
                        "object": "chat.completion.chunk",
                        "created": 0,
                        "model": "test",
                        "choices": [{
                            "index": 0,
                            "delta": {"content": text},
                            "finish_reason": null
                        }]
                    });
                    let chunk: ChatCompletionChunk = serde_json::from_value(chunk_json)
                        .map_err(|e| LiterLmError::Streaming { message: e.to_string() })?;
                    Ok(Some(chunk))
                }
                "messageStop" => Ok(None),
                _ => Ok(None),
            }
        };

        let parser = EventStreamParser::new(byte_stream, parse);
        let mut pinned = pin!(parser);

        // Poll once — should get the text delta chunk.
        let waker = std::task::Waker::noop();
        let mut cx = Context::from_waker(waker);

        match pinned.as_mut().poll_next(&mut cx) {
            Poll::Ready(Some(Ok(chunk))) => {
                assert_eq!(chunk.choices[0].delta.content.as_deref(), Some("Hello"));
            }
            other => panic!("expected Ready(Some(Ok(chunk))), got {other:?}"),
        }
    }

    #[test]
    fn exception_frame_yields_error() {
        let frame = build_frame(
            &[(":message-type", "exception"), (":event-type", "validationException")],
            br#"{"message":"Invalid request"}"#,
        );

        let headers_length = u32::from_be_bytes([frame[4], frame[5], frame[6], frame[7]]) as usize;
        let headers_start = 12;
        let headers_end = headers_start + headers_length;
        let total_length = frame.len();

        let headers = parse_headers(&frame[headers_start..headers_end]).unwrap();
        let message_type = headers.iter().find(|h| h.name == ":message-type").unwrap();
        assert_eq!(message_type.value, "exception");

        let payload = std::str::from_utf8(&frame[headers_end..total_length - 4]).unwrap();
        assert!(payload.contains("Invalid request"));
    }

    #[test]
    fn corrupt_prelude_crc_detected() {
        let mut frame = build_frame(
            &[(":message-type", "event"), (":event-type", "messageStop")],
            br#"{"stopReason":"end_turn"}"#,
        );
        // Corrupt byte 9 (part of prelude CRC).
        frame[9] ^= 0xFF;

        let total_length = u32::from_be_bytes([frame[0], frame[1], frame[2], frame[3]]) as usize;
        assert_eq!(total_length, frame.len());

        let prelude_crc_stored = u32::from_be_bytes([frame[8], frame[9], frame[10], frame[11]]);
        let prelude_crc_actual = crc32c(&frame[..8]);
        assert_ne!(prelude_crc_stored, prelude_crc_actual);
    }

    #[test]
    fn corrupt_message_crc_detected() {
        let mut frame = build_frame(
            &[(":message-type", "event"), (":event-type", "messageStop")],
            br#"{"stopReason":"end_turn"}"#,
        );
        let len = frame.len();
        // Corrupt a payload byte.
        frame[len / 2] ^= 0xFF;

        let message_crc_stored = u32::from_be_bytes([frame[len - 4], frame[len - 3], frame[len - 2], frame[len - 1]]);
        let message_crc_actual = crc32c(&frame[..len - 4]);
        assert_ne!(message_crc_stored, message_crc_actual);
    }

    #[test]
    fn empty_payload_frame() {
        let frame = build_frame(&[(":message-type", "event"), (":event-type", "contentBlockStop")], b"");
        let total_length = u32::from_be_bytes([frame[0], frame[1], frame[2], frame[3]]) as usize;
        assert_eq!(total_length, frame.len());
    }

    #[tokio::test]
    async fn parser_handles_split_frames() {
        use super::vec_stream::VecStream;
        use std::pin::pin;
        use std::task::{Context, Poll};

        let frame = build_frame(
            &[(":message-type", "event"), (":event-type", "contentBlockDelta")],
            br#"{"contentBlockIndex":0,"delta":{"text":"split"}}"#,
        );

        // Split the frame in half to simulate TCP segmentation.
        let mid = frame.len() / 2;
        let chunk1 = bytes::Bytes::from(frame[..mid].to_vec());
        let chunk2 = bytes::Bytes::from(frame[mid..].to_vec());

        let byte_stream = VecStream::new(vec![Ok(chunk1), Ok(chunk2)]);
        let parse = |event_type: &str, payload: &str| -> Result<Option<ChatCompletionChunk>> {
            if event_type == "contentBlockDelta" {
                let v: serde_json::Value = serde_json::from_str(payload).unwrap();
                let text = v.pointer("/delta/text").and_then(|t| t.as_str()).unwrap_or("");
                let chunk: ChatCompletionChunk = serde_json::from_value(serde_json::json!({
                    "id": "t", "object": "chat.completion.chunk", "created": 0, "model": "t",
                    "choices": [{"index": 0, "delta": {"content": text}, "finish_reason": null}]
                }))
                .unwrap();
                Ok(Some(chunk))
            } else {
                Ok(None)
            }
        };

        let parser = EventStreamParser::new(byte_stream, parse);
        let mut pinned = pin!(parser);

        let waker = std::task::Waker::noop();
        let mut cx = Context::from_waker(waker);

        // First poll may return Pending (first chunk doesn't complete the frame).
        // Keep polling until we get the result.
        let mut result = None;
        for _ in 0..10 {
            match pinned.as_mut().poll_next(&mut cx) {
                Poll::Ready(Some(Ok(chunk))) => {
                    result = Some(chunk);
                    break;
                }
                Poll::Ready(Some(Err(e))) => panic!("unexpected error: {e}"),
                Poll::Ready(None) => panic!("unexpected stream end"),
                Poll::Pending => continue,
            }
        }
        let chunk = result.expect("should have parsed the split frame");
        assert_eq!(chunk.choices[0].delta.content.as_deref(), Some("split"));
    }

    #[tokio::test]
    async fn parser_errors_on_truncated_stream() {
        use super::vec_stream::VecStream;
        use std::pin::pin;
        use std::task::{Context, Poll};

        let frame = build_frame(
            &[(":message-type", "event"), (":event-type", "contentBlockDelta")],
            br#"{"contentBlockIndex":0,"delta":{"text":"truncated"}}"#,
        );

        // Only deliver the first half — simulate connection drop.
        let partial = bytes::Bytes::from(frame[..frame.len() / 2].to_vec());
        let byte_stream = VecStream::new(vec![Ok(partial)]);

        let parse = |_: &str, _: &str| -> Result<Option<ChatCompletionChunk>> { Ok(None) };

        let parser = EventStreamParser::new(byte_stream, parse);
        let mut pinned = pin!(parser);

        let waker = std::task::Waker::noop();
        let mut cx = Context::from_waker(waker);

        // Poll until we get an error about truncated data.
        let mut got_error = false;
        for _ in 0..10 {
            match pinned.as_mut().poll_next(&mut cx) {
                Poll::Ready(Some(Err(e))) => {
                    let msg = e.to_string();
                    assert!(
                        msg.contains("incomplete frame"),
                        "expected truncation error, got: {msg}"
                    );
                    got_error = true;
                    break;
                }
                Poll::Ready(Some(Ok(_))) => panic!("unexpected success"),
                Poll::Ready(None) => panic!("unexpected clean end"),
                Poll::Pending => continue,
            }
        }
        assert!(got_error, "should have received a truncation error");
    }

    #[tokio::test]
    async fn parser_exception_frame_through_stream() {
        use super::vec_stream::VecStream;
        use std::pin::pin;
        use std::task::{Context, Poll};

        let frame = build_frame(
            &[(":message-type", "exception"), (":event-type", "throttlingException")],
            br#"{"message":"Rate exceeded"}"#,
        );

        let byte_stream = VecStream::new(vec![Ok(bytes::Bytes::from(frame))]);
        let parse = |_: &str, _: &str| -> Result<Option<ChatCompletionChunk>> { Ok(None) };

        let parser = EventStreamParser::new(byte_stream, parse);
        let mut pinned = pin!(parser);

        let waker = std::task::Waker::noop();
        let mut cx = Context::from_waker(waker);

        match pinned.as_mut().poll_next(&mut cx) {
            Poll::Ready(Some(Err(e))) => {
                let msg = e.to_string();
                assert!(msg.contains("throttlingException"), "got: {msg}");
                assert!(msg.contains("Rate exceeded"), "got: {msg}");
            }
            other => panic!("expected error, got {other:?}"),
        }
    }
}

/// Test helper: a stream that yields items from a Vec.
#[cfg(test)]
mod vec_stream {
    use std::collections::VecDeque;
    use std::pin::Pin;
    use std::task::{Context, Poll};

    use futures_core::Stream;

    pub struct VecStream<T> {
        items: VecDeque<T>,
    }

    impl<T> VecStream<T> {
        pub fn new(items: Vec<T>) -> Self {
            Self { items: items.into() }
        }
    }

    impl<T: Unpin> Stream for VecStream<T> {
        type Item = T;

        fn poll_next(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
            let this = self.get_mut();
            match this.items.pop_front() {
                Some(item) => Poll::Ready(Some(item)),
                None => Poll::Ready(None),
            }
        }
    }
}

/// Helper: create a stream that yields a single future's result.
#[cfg(test)]
mod once_future_stream {
    use std::future::Future;
    use std::pin::Pin;
    use std::task::{Context, Poll};

    use futures_core::Stream;

    pin_project_lite::pin_project! {
        pub struct OnceFuture<F> {
            #[pin]
            future: Option<F>,
        }
    }

    impl<F: Future> Stream for OnceFuture<F> {
        type Item = F::Output;

        fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
            let mut this = self.project();
            match this.future.as_mut().as_pin_mut() {
                Some(f) => match f.poll(cx) {
                    Poll::Ready(val) => {
                        this.future.set(None);
                        Poll::Ready(Some(val))
                    }
                    Poll::Pending => Poll::Pending,
                },
                None => Poll::Ready(None),
            }
        }
    }

    pub fn once_future<F: Future>(f: F) -> OnceFuture<F> {
        OnceFuture { future: Some(f) }
    }
}
