use std::time::{Duration, SystemTime, UNIX_EPOCH};

/// Determine whether to retry based on status code and attempt number.
///
/// Returns `Some(delay)` if the request should be retried, `None` otherwise.
///
/// When `retry_after` is provided (parsed from the `Retry-After` response
/// header) it takes precedence over exponential backoff for 429 responses.
///
/// Exponential backoff includes jitter to prevent thundering-herd effects
/// when multiple clients retry simultaneously. The jitter scales the delay
/// to a random value in `[0.5 * base, 1.0 * base]` using the low-order bits
/// of the system clock as a lightweight entropy source.
pub fn should_retry(status: u16, attempt: u32, max_retries: u32, retry_after: Option<Duration>) -> Option<Duration> {
    if attempt >= max_retries {
        return None;
    }

    // Only retry on rate limit (429) and server errors (500, 502, 503, 504).
    if !matches!(status, 429 | 500 | 502 | 503 | 504) {
        return None;
    }

    // For 429, prefer the server-supplied Retry-After value when present.
    if status == 429
        && let Some(server_delay) = retry_after
    {
        // Cap the server-supplied delay to 60 seconds to avoid stalling forever.
        return Some(server_delay.min(Duration::from_secs(60)));
    }

    // Exponential backoff: 1s, 2s, 4s, 8s … capped at 30 s.
    // Use checked_shl to avoid overflow on large attempt counts.
    let base_delay = Duration::from_secs(1u64.checked_shl(attempt).unwrap_or(u64::MAX));
    let capped = base_delay.min(Duration::from_secs(30));

    // Apply jitter: scale to [0.5, 1.0] of the capped delay.
    // Use nanosecond component of the system clock as a lightweight entropy source.
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .subsec_nanos();
    // jitter_factor in [0.5, 1.0]
    let jitter_factor = 0.5 + (f64::from(nanos % 1000) / 2000.0);
    Some(capped.mul_f64(jitter_factor))
}

/// Parse the value of a `Retry-After` header into a `Duration`.
///
/// The header may be:
/// - A non-negative integer (number of seconds to wait), or
/// - An HTTP-date (RFC 7231 format; not yet parsed — falls back to exponential
///   backoff with a warning).
pub fn parse_retry_after(value: &str) -> Option<Duration> {
    let trimmed = value.trim();

    // Attempt to parse as a plain integer (seconds).
    if let Ok(secs) = trimmed.parse::<u64>() {
        return Some(Duration::from_secs(secs));
    }

    // HTTP-date format (e.g. "Wed, 21 Oct 2015 07:28:00 GMT") is not yet
    // parsed.  Emit a warning so operators know when servers use this format,
    // and return None to fall back to exponential backoff.
    #[cfg(feature = "tracing")]
    tracing::warn!(
        retry_after = trimmed,
        "Retry-After header uses HTTP-date format which is not yet supported; \
         falling back to exponential backoff"
    );
    None
}
