use std::future::Future;

use bytes::Bytes;
use serde::de::DeserializeOwned;

use crate::error::{LiterLmError, Result};
use crate::http::retry;

// ---------------------------------------------------------------------------
// Shared retry loop helper
// ---------------------------------------------------------------------------

/// Extract an optional `Retry-After` delay from a response.
pub(crate) fn retry_after_from_response(resp: &reqwest::Response) -> Option<std::time::Duration> {
    let value = resp.headers().get(reqwest::header::RETRY_AFTER)?.to_str().ok()?;
    retry::parse_retry_after(value)
}

/// Drive a single-request closure through the retry / back-off loop.
///
/// `send` is called once per attempt and must return a future that resolves to
/// a raw `reqwest::Response` (or a transport-level error).  The helper handles:
///
/// - Attempt counting and the `max_retries` budget.
/// - Parsing the `Retry-After` header before consuming the response body.
/// - Exponential back-off via [`retry::should_retry`].
/// - Reading the error body and mapping it to [`LiterLmError`] on final failure.
///
/// On success the **successful** `Response` is returned so the caller can
/// choose how to consume the body (JSON deserialisation, byte stream, …).
pub(crate) async fn with_retry<F, Fut>(max_retries: u32, mut send: F) -> Result<reqwest::Response>
where
    F: FnMut() -> Fut,
    Fut: Future<Output = std::result::Result<reqwest::Response, reqwest::Error>>,
{
    let mut attempt = 0u32;

    loop {
        let resp = send().await?;
        let status = resp.status().as_u16();

        if resp.status().is_success() {
            return Ok(resp);
        }

        // Parse Retry-After *before* consuming the body.
        let server_retry_after = retry_after_from_response(&resp);

        if let Some(delay) = retry::should_retry(status, attempt, max_retries, server_retry_after) {
            attempt += 1;
            tokio::time::sleep(delay).await;
            continue;
        }

        // Non-retryable — read the body for a useful error message.
        let text = resp
            .text()
            .await
            .unwrap_or_else(|e| format!("(failed to read body: {e})"));
        return Err(LiterLmError::from_status(status, &text, server_retry_after));
    }
}

// ---------------------------------------------------------------------------
// Public API
// ---------------------------------------------------------------------------

/// Send a POST request with a JSON body and return the raw response JSON.
///
/// Like [`post_json`] but returns a `serde_json::Value` instead of deserializing
/// into a typed `T`.  This allows the caller to mutate the response (e.g. via a
/// provider `transform_response`) before deserializing into the canonical type.
///
/// Retries on 429 / 5xx according to `max_retries`.
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
pub async fn post_json_raw(
    client: &reqwest::Client,
    url: &str,
    auth_header: Option<(&str, &str)>,
    extra_headers: &[(&str, &str)],
    body: Bytes,
    max_retries: u32,
) -> Result<serde_json::Value> {
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

    resp.json::<serde_json::Value>().await.map_err(LiterLmError::from)
}

/// Send a GET request and deserialize the JSON response.
///
/// Retries on 429 / 5xx according to `max_retries`, honouring any
/// `Retry-After` header from the server.
///
/// `auth_header` is `Some((name, value))` when the provider requires
/// authentication, or `None` when no auth header should be added.
///
/// `extra_headers` carries provider-specific mandatory headers beyond the
/// single auth header.
#[cfg_attr(
    feature = "tracing",
    tracing::instrument(
        skip_all,
        fields(
            http.method = "GET",
            http.url = %url,
            http.status_code = tracing::field::Empty,
            http.retry_count = tracing::field::Empty,
        )
    )
)]
pub async fn get_json<T: DeserializeOwned>(
    client: &reqwest::Client,
    url: &str,
    auth_header: Option<(&str, &str)>,
    extra_headers: &[(&str, &str)],
    max_retries: u32,
) -> Result<T> {
    let mut retry_count = 0u32;

    let resp = with_retry(max_retries, || {
        let mut builder = client.get(url);
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

    resp.json::<T>().await.map_err(LiterLmError::from)
}
