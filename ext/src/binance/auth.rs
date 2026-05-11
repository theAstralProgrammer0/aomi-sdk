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

use hmac::{Hmac, Mac};
use sha2::Sha256;

type HmacSha256 = Hmac<Sha256>;

/// Compute HMAC-SHA256 over `query_string` using `secret_key`, return as hex.
pub fn sign(secret_key: &str, query_string: &str) -> Result<String, String> {
    let mut mac = HmacSha256::new_from_slice(secret_key.as_bytes())
        .map_err(|e| format!("[binance] failed to create HMAC key: {e}"))?;
    mac.update(query_string.as_bytes());
    let result = mac.finalize();
    Ok(hex_encode(&result.into_bytes()))
}

fn hex_encode(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}

/// Current millis since UNIX epoch.
pub fn current_timestamp_ms() -> Result<i64, String> {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_millis() as i64)
        .map_err(|e| format!("[binance] failed to get timestamp: {e}"))
}

/// URL-encode a single value the same way reqwest's `.query(...)` chain does
/// (RFC 3986 unreserved characters left as-is, everything else percent-encoded).
pub fn urlencode(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for b in s.as_bytes() {
        let c = *b as char;
        if c.is_ascii_alphanumeric() || matches!(c, '-' | '_' | '.' | '~') {
            out.push(c);
        } else {
            use std::fmt::Write;
            let _ = write!(out, "%{:02X}", b);
        }
    }
    out
}

/// Build a Binance-canonical query string from `(key, value)` pairs.
/// Pairs whose value is `None` are skipped. Order is preserved (Binance signs
/// the literal payload, but reqwest emits params in insertion order).
pub fn build_query(pairs: &[(&str, Option<String>)]) -> String {
    pairs
        .iter()
        .filter_map(|(k, v)| v.as_ref().map(|val| format!("{}={}", k, urlencode(val))))
        .collect::<Vec<_>>()
        .join("&")
}
