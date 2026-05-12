//! Hand-written prehash recipe for Limitless's `lmts-*` HMAC headers.
//!
//! All HMAC / base64 / timestamp primitives come from
//! [`aomi_ext::hmac_auth`]. This file only owns the venue-specific bits:
//!
//!   1. Limitless's secret is *itself* base64-encoded ("zDOj…="); we have to
//!      decode it before using as the HMAC key. (OKX, by contrast, uses the
//!      raw string secret.)
//!   2. The prehash format is `"{ts}\n{METHOD}\n{path+query}\n{body}"` with
//!      newline separators. Other venues concatenate without newlines, or
//!      include extra fields (recv-window, api-key, …) — they each keep
//!      their own `auth.rs` for the same reason.
//!
//! The three headers each authenticated request must carry:
//!   - `lmts-api-key`   — the token id (the "API Key" string from the dashboard)
//!   - `lmts-timestamp` — ISO-8601 UTC, ms precision, within 30s of server
//!   - `lmts-signature` — output of [`sign`] below

use aomi_ext::hmac_auth::{base64_decode, hmac_sha256_base64, iso_timestamp_ms};

/// Compute the `lmts-signature` value for one request.
///
/// `secret_b64` is the raw string from the dashboard (already base64).
/// `path_with_query` MUST include leading `/` and any `?query=...` suffix.
/// `body` is `""` for GET / DELETE without bodies.
pub fn sign(
    secret_b64: &str,
    timestamp: &str,
    method: &str,
    path_with_query: &str,
    body: &str,
) -> Result<String, String> {
    let key = base64_decode(secret_b64)
        .map_err(|e| format!("[limitless] secret is not valid base64: {e}"))?;
    let prehash = format!("{timestamp}\n{method}\n{path_with_query}\n{body}");
    Ok(hmac_sha256_base64(&key, prehash.as_bytes()))
}

/// Re-export the shared millisecond-precision ISO timestamp under the name
/// the rest of `apps/limitless` already imports.
pub fn iso_timestamp() -> String {
    iso_timestamp_ms()
}
