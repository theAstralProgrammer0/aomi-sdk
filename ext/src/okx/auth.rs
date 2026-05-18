//! Hand-written companion to the generated `client.rs`.
//!
//! OKX V5 signed endpoints require these headers on every signed call:
//!   - `OK-ACCESS-KEY`        — your API key
//!   - `OK-ACCESS-SIGN`       — base64(HMAC-SHA256(secret, prehash))
//!   - `OK-ACCESS-TIMESTAMP`  — ISO-8601 timestamp (e.g. 2024-01-01T00:00:00.000Z)
//!   - `OK-ACCESS-PASSPHRASE` — passphrase set when the API key was created
//!
//! Prehash is `timestamp + method + requestPath + body` where `requestPath`
//! includes the query string. The HMAC key is the API secret.
//!
//! The curated tool layer calls [`sign`] / [`iso_timestamp`] from this module
//! and passes the resulting headers into the generated client method.
//!
//! Primitives delegated to [`crate::hmac_auth`]; this module only owns the
//! OKX-specific prehash recipe and the public function names callers expect.

use crate::hmac_auth;

/// HMAC-SHA256 signature over `timestamp + method + request_path + body`,
/// base64-encoded — exactly what `OK-ACCESS-SIGN` expects.
pub fn sign(
    secret_key: &str,
    timestamp: &str,
    method: &str,
    request_path: &str,
    body: &str,
) -> Result<String, String> {
    let prehash = format!("{timestamp}{method}{request_path}{body}");
    Ok(hmac_auth::hmac_sha256_base64(
        secret_key.as_bytes(),
        prehash.as_bytes(),
    ))
}

/// Current UTC time as ISO-8601 with millisecond precision (no trailing offset
/// other than `Z`), e.g. `2024-01-01T00:00:00.000Z` — OKX's required format.
pub fn iso_timestamp() -> String {
    hmac_auth::iso_timestamp_ms()
}
