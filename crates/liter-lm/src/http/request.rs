use serde::de::DeserializeOwned;

use crate::error::{LiterLmError, Result};
use crate::http::retry;

// ---------------------------------------------------------------------------
// Shared retry loop helpers
// ---------------------------------------------------------------------------

/// Extract an optional `Retry-After` delay from a response.
pub(crate) fn retry_after_from_response(resp: &reqwest::Response) -> Option<std::time::Duration> {
    let value = resp.headers().get(reqwest::header::RETRY_AFTER)?.to_str().ok()?;
    retry::parse_retry_after(value)
}

// ---------------------------------------------------------------------------
// Public API
// ---------------------------------------------------------------------------

// TODO: extract a shared `retry_loop` helper used by both `post_json` and
// `get_json` to eliminate the duplicated retry/backoff logic in each function.

/// Send a POST request with a JSON body and deserialize the JSON response.
///
/// Retries on 429 / 5xx according to `max_retries`.  When the server sends a
/// `Retry-After` header on a 429 response the supplied delay is respected
/// instead of the default exponential backoff.
///
/// `auth_header` is `Some((name, value))` when the provider requires
/// authentication, or `None` when no auth header should be added (e.g. local
/// models or providers with `auth: none`).
///
/// `extra_headers` carries provider-specific mandatory headers (e.g.
/// `anthropic-version`) beyond the single auth header.
pub async fn post_json<T: DeserializeOwned>(
    client: &reqwest::Client,
    url: &str,
    auth_header: Option<(&str, &str)>,
    extra_headers: &[(&str, &str)],
    body: serde_json::Value,
    max_retries: u32,
) -> Result<T> {
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
            // Use reqwest's built-in JSON deserializer — avoids buffering
            // the entire body as a String before parsing.
            return resp.json::<T>().await.map_err(LiterLmError::from);
        }

        // Check Retry-After before consuming the body.
        let server_retry_after = retry_after_from_response(&resp);

        if let Some(delay) = retry::should_retry(status, attempt, max_retries, server_retry_after) {
            attempt += 1;
            tokio::time::sleep(delay).await;
            continue;
        }

        // Non-retryable error — read the body for a useful error message.
        let text = resp
            .text()
            .await
            .unwrap_or_else(|e| format!("(failed to read body: {e})"));
        return Err(LiterLmError::from_status(status, &text, server_retry_after));
    }
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
pub async fn get_json<T: DeserializeOwned>(
    client: &reqwest::Client,
    url: &str,
    auth_header: Option<(&str, &str)>,
    extra_headers: &[(&str, &str)],
    max_retries: u32,
) -> Result<T> {
    let mut attempt = 0u32;

    loop {
        let mut builder = client.get(url);
        if let Some((name, value)) = auth_header {
            builder = builder.header(name, value);
        }
        for (name, value) in extra_headers {
            builder = builder.header(*name, *value);
        }
        let resp = builder.send().await?;

        let status = resp.status().as_u16();

        if resp.status().is_success() {
            return resp.json::<T>().await.map_err(LiterLmError::from);
        }

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
