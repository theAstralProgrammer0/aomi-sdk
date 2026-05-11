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

use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64};
use hmac::{Hmac, Mac};
use sha2::Sha256;

type HmacSha256 = Hmac<Sha256>;

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
    let mut mac = HmacSha256::new_from_slice(secret_key.as_bytes())
        .map_err(|e| format!("[okx] HMAC key error: {e}"))?;
    mac.update(prehash.as_bytes());
    Ok(BASE64.encode(mac.finalize().into_bytes()))
}

/// Current UTC time as ISO-8601 with millisecond precision (no trailing offset
/// other than `Z`), e.g. `2024-01-01T00:00:00.000Z` — OKX's required format.
pub fn iso_timestamp() -> String {
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default();
    let secs = now.as_secs();
    let millis = now.subsec_millis();
    let time_of_day = secs % 86400;
    let hours = time_of_day / 3600;
    let minutes = (time_of_day % 3600) / 60;
    let seconds = time_of_day % 60;

    let (year, month, day) = days_to_ymd(secs / 86400);
    format!("{year:04}-{month:02}-{day:02}T{hours:02}:{minutes:02}:{seconds:02}.{millis:03}Z")
}

/// Convert days since Unix epoch to (year, month, day).
/// Algorithm from http://howardhinnant.github.io/date_algorithms.html.
fn days_to_ymd(days: u64) -> (u64, u64, u64) {
    let z = days + 719468;
    let era = z / 146097;
    let doe = z - era * 146097;
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;
    let y = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    let y = if m <= 2 { y + 1 } else { y };
    (y, m, d)
}
