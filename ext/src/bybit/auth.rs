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
//! [`sign_query`] / [`sign_body`] / [`current_timestamp_ms`] from this module
//! and passes the resulting headers into the generated client method.
//!
//! Primitives delegated to [`crate::hmac_auth`]; this module only owns Bybit's
//! recv-window constant + alphabetical-key build_query (specific to Bybit's
//! signing convention) + the recv-window-aware prehash recipe.

use crate::hmac_auth;

/// The recv-window value we send on every signed request. 5_000ms is the
/// Bybit-recommended default.
pub const RECV_WINDOW: &str = "5000";

/// Current millis since UNIX epoch as a string. Bybit expects this as the
/// `X-BAPI-TIMESTAMP` header.
pub fn current_timestamp_ms() -> String {
    hmac_auth::current_timestamp_ms().to_string()
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
    hmac_auth::hmac_sha256_hex(secret.as_bytes(), sign_str.as_bytes())
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
///
/// Note: this is *not* the shared [`hmac_auth::build_query`] because that one
/// preserves insertion order. Bybit specifically needs alphabetical sort, so
/// we keep the sort here and reuse the shared `urlencode` for the values.
pub fn build_query(pairs: &[(&str, Option<&str>)]) -> String {
    let mut kept: Vec<(&str, &str)> = pairs
        .iter()
        .filter_map(|(k, v)| v.map(|val| (*k, val)))
        .collect();
    kept.sort_by_key(|(k, _)| *k);
    kept.iter()
        .map(|(k, v)| format!("{}={}", k, hmac_auth::urlencode(v)))
        .collect::<Vec<_>>()
        .join("&")
}
