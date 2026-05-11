//! Hand-written companion to the generated `client.rs`.
//!
//! Bybit's V5 unified API requires every signed request to carry four headers:
//!   - `X-BAPI-API-KEY`     — the api key
//!   - `X-BAPI-TIMESTAMP`   — current millis since epoch
//!   - `X-BAPI-RECV-WINDOW` — request validity window (we use `5000`)
//!   - `X-BAPI-SIGN`        — HMAC-SHA256 hex signature
//!
//! The signature payload is `timestamp + api_key + recv_window + payload`, where
//! `payload` is the URL-encoded query string for GET requests, or the raw JSON
//! body for POST requests. The HMAC key is the API secret.
//!
//! Progenitor doesn't know how to compute that, so the curated tool layer calls
//! [`sign_query`] / [`sign_body`] / [`current_timestamp_ms`] from `auth.rs` and
//! passes the resulting headers into the generated client method.

use hmac::{Hmac, Mac};
use sha2::Sha256;

type HmacSha256 = Hmac<Sha256>;

/// The recv-window value we send on every signed request. 5_000ms is the
/// Bybit-recommended default.
pub const RECV_WINDOW: &str = "5000";

/// Current millis since UNIX epoch as a string. Bybit expects this as the
/// `X-BAPI-TIMESTAMP` header.
pub fn current_timestamp_ms() -> String {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("system clock before UNIX epoch")
        .as_millis()
        .to_string()
}

/// HMAC-SHA256 hex signature for a Bybit V5 GET request.
///
/// `query` should be the URL-encoded query string with parameters in the same
/// order they appear on the wire (no leading `?`). For requests with no query
/// params, pass `""`.
pub fn sign_query(timestamp: &str, api_key: &str, secret: &str, query: &str) -> String {
    sign_payload(timestamp, api_key, secret, query)
}

/// HMAC-SHA256 hex signature for a Bybit V5 POST request.
///
/// `body` must be the exact JSON string sent on the wire. Re-serialising before
/// signing will give a different signature than what the server computes.
pub fn sign_body(timestamp: &str, api_key: &str, secret: &str, body: &str) -> String {
    sign_payload(timestamp, api_key, secret, body)
}

fn sign_payload(timestamp: &str, api_key: &str, secret: &str, payload: &str) -> String {
    let sign_str = format!("{timestamp}{api_key}{RECV_WINDOW}{payload}");
    let mut mac =
        HmacSha256::new_from_slice(secret.as_bytes()).expect("HMAC can take key of any size");
    mac.update(sign_str.as_bytes());
    hex_encode(mac.finalize().into_bytes())
}

/// Build a Bybit-canonical query string from `(key, value)` pairs and return
/// it sorted alphabetically by key. Pairs whose value is `None` are skipped.
///
/// Bybit's V5 spec doesn't actually mandate alphabetical ordering server-side
/// (it signs the literal payload sent), but progenitor's reqwest `.query(...)`
/// chain happens to emit params in alphabetical order — so the safest pattern
/// for the curated tool layer is to compute the signature against the same
/// ordering. Use this helper to build the string passed to [`sign_query`] AND
/// be sure to pass the SAME values into the generated client call.
pub fn build_query(pairs: &[(&str, Option<&str>)]) -> String {
    let mut kept: Vec<(&str, &str)> = pairs
        .iter()
        .filter_map(|(k, v)| v.map(|val| (*k, val)))
        .collect();
    kept.sort_by_key(|(k, _)| *k);
    kept.iter()
        .map(|(k, v)| format!("{}={}", k, urlencode(v)))
        .collect::<Vec<_>>()
        .join("&")
}

fn urlencode(s: &str) -> String {
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

fn hex_encode(bytes: impl AsRef<[u8]>) -> String {
    bytes.as_ref().iter().fold(String::new(), |mut acc, b| {
        use std::fmt::Write;
        let _ = write!(acc, "{b:02x}");
        acc
    })
}
