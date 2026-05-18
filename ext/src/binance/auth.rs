//! Hand-written companion to the generated `client.rs`.
//!
//! Binance's signed endpoints require:
//!   - `X-MBX-APIKEY`     header — the api key
//!   - `timestamp`        query  — current millis since epoch
//!   - `signature`        query  — HMAC-SHA256(secret, query_string) hex
//!
//! The signature is computed over the URL-encoded query string of all OTHER
//! parameters (in the order they appear on the wire), with `&timestamp=…`
//! appended. Progenitor doesn't know how to compute that, so the curated tool
//! layer calls [`sign`] / [`current_timestamp_ms`] from this module and passes
//! the resulting signature/timestamp into the generated method as query params.
//!
//! Primitives delegated to [`crate::hmac_auth`]; this module only owns the
//! Binance-specific surface (return types, error wrapping) callers expect.

use crate::hmac_auth;

/// Compute HMAC-SHA256 over `query_string` using `secret_key`, return as hex.
pub fn sign(secret_key: &str, query_string: &str) -> Result<String, String> {
    Ok(hmac_auth::hmac_sha256_hex(
        secret_key.as_bytes(),
        query_string.as_bytes(),
    ))
}

/// Current millis since UNIX epoch.
pub fn current_timestamp_ms() -> Result<i64, String> {
    Ok(hmac_auth::current_timestamp_ms() as i64)
}

/// URL-encode a single value the same way reqwest's `.query(...)` chain does
/// (RFC 3986 unreserved characters left as-is, everything else percent-encoded).
pub fn urlencode(s: &str) -> String {
    hmac_auth::urlencode(s)
}

/// Build a Binance-canonical query string from `(key, value)` pairs.
/// Pairs whose value is `None` are skipped. Order is preserved (Binance signs
/// the literal payload, but reqwest emits params in insertion order).
pub fn build_query(pairs: &[(&str, Option<String>)]) -> String {
    // Adapt owned-string pairs to the borrowed-slice API the shared helper takes.
    let borrowed: Vec<(&str, Option<&str>)> =
        pairs.iter().map(|(k, v)| (*k, v.as_deref())).collect();
    hmac_auth::build_query(&borrowed)
}
