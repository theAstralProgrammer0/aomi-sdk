//! Shared HMAC-SHA256 + URL/timestamp helpers for app `auth.rs` modules.
//!
//! Every venue that signs API requests (Binance, Bybit, OKX, Limitless, …)
//! does the same handful of low-level operations: compute an HMAC, encode
//! the result as hex or base64, generate a timestamp, percent-encode query
//! params. The exact *prehash format* differs per venue (which is why each
//! app keeps its own thin `auth.rs` wrapper), but the primitives don't.
//!
//! Behind the `hmac-auth` cargo feature on `aomi-ext`. Pull in from an app:
//!
//! ```toml
//! [dependencies]
//! aomi-ext = { path = "../../ext", features = ["hmac-auth"] }
//! ```
//!
//! Then in your venue's `auth.rs`:
//!
//! ```ignore
//! use aomi_ext::hmac_auth::{hmac_sha256_base64, iso_timestamp_ms};
//!
//! pub fn sign(secret: &str, ts: &str, method: &str, path: &str, body: &str)
//!     -> Result<String, String>
//! {
//!     let prehash = format!("{ts}\n{method}\n{path}\n{body}"); // venue-specific
//!     Ok(hmac_sha256_base64(secret.as_bytes(), prehash.as_bytes()))
//! }
//! ```

use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64};
use hmac::{Hmac, Mac};
use sha2::Sha256;

type HmacSha256 = Hmac<Sha256>;

// ---------------------------------------------------------------------------
// HMAC primitives
// ---------------------------------------------------------------------------

/// Raw HMAC-SHA256 output as bytes.
///
/// `key` is the HMAC key — for venues whose secret is base64-encoded
/// (Limitless), pass `&base64_decode(secret)`; otherwise pass `secret.as_bytes()`.
pub fn hmac_sha256(key: &[u8], msg: &[u8]) -> Vec<u8> {
    let mut mac = HmacSha256::new_from_slice(key).expect("HMAC accepts any key length");
    mac.update(msg);
    mac.finalize().into_bytes().to_vec()
}

/// HMAC-SHA256 output as a lowercase hex string.
/// Used by Binance (`signature` query param) and Bybit (`X-BAPI-SIGN` header).
pub fn hmac_sha256_hex(key: &[u8], msg: &[u8]) -> String {
    hex_encode(&hmac_sha256(key, msg))
}

/// HMAC-SHA256 output as a base64 string (standard alphabet, padded).
/// Used by OKX (`OK-ACCESS-SIGN`) and Limitless (`lmts-signature`).
pub fn hmac_sha256_base64(key: &[u8], msg: &[u8]) -> String {
    BASE64.encode(hmac_sha256(key, msg))
}

/// Decode a base64 string. Convenience for venues whose dashboard hands you
/// a base64-encoded secret that must be decoded before use as the HMAC key
/// (Limitless does this; OKX does not).
pub fn base64_decode(s: &str) -> Result<Vec<u8>, String> {
    BASE64
        .decode(s.as_bytes())
        .map_err(|e| format!("base64 decode error: {e}"))
}

// ---------------------------------------------------------------------------
// Timestamps
// ---------------------------------------------------------------------------

/// Current Unix time in milliseconds. Most signed-trading venues (Binance,
/// Bybit) want this on every request as `timestamp` / `X-BAPI-TIMESTAMP`.
pub fn current_timestamp_ms() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_millis() as u64)
        .unwrap_or(0)
}

/// ISO-8601 UTC timestamp with millisecond precision: `2024-01-01T00:00:00.000Z`.
/// Used by OKX and Limitless. Avoids pulling `chrono` into auth-only crates.
pub fn iso_timestamp_ms() -> String {
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

/// Days since Unix epoch → (year, month, day). Civil-from-days algorithm by
/// Howard Hinnant (http://howardhinnant.github.io/date_algorithms.html).
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

// ---------------------------------------------------------------------------
// URL helpers
// ---------------------------------------------------------------------------

/// Percent-encode a single value per RFC 3986 unreserved-character rules
/// (matches what reqwest's `.query(...)` chain produces, so signatures over
/// query strings stay byte-identical to the wire).
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

/// Build `k1=v1&k2=v2&…` from an ordered list of pairs, dropping any pair
/// whose value is `None`. Values are percent-encoded via [`urlencode`].
/// Returns the suffix without leading `?`.
pub fn build_query(pairs: &[(&str, Option<&str>)]) -> String {
    pairs
        .iter()
        .filter_map(|(k, v)| v.map(|val| format!("{}={}", k, urlencode(val))))
        .collect::<Vec<_>>()
        .join("&")
}

// ---------------------------------------------------------------------------
// Internal helpers
// ---------------------------------------------------------------------------

fn hex_encode(bytes: &[u8]) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut out = String::with_capacity(bytes.len() * 2);
    for b in bytes {
        out.push(HEX[(b >> 4) as usize] as char);
        out.push(HEX[(b & 0xf) as usize] as char);
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    // RFC 4231 test case 1: HMAC-SHA256 with key=0x0b*20, msg="Hi There"
    const RFC_KEY: [u8; 20] = [0x0b; 20];
    const RFC_HEX: &str = "b0344c61d8db38535ca8afceaf0bf12b881dc200c9833da726e9376c2e32cff7";

    #[test]
    fn hmac_matches_rfc_4231_vector_1() {
        assert_eq!(hmac_sha256_hex(&RFC_KEY, b"Hi There"), RFC_HEX);
    }

    #[test]
    fn base64_round_trip() {
        let raw = b"hello world";
        let encoded = BASE64.encode(raw);
        assert_eq!(base64_decode(&encoded).unwrap(), raw);
    }

    #[test]
    fn iso_timestamp_shape_is_correct() {
        let ts = iso_timestamp_ms();
        // YYYY-MM-DDTHH:MM:SS.mmmZ → 24 chars
        assert_eq!(ts.len(), 24);
        assert!(ts.ends_with('Z'));
        assert_eq!(&ts[4..5], "-");
        assert_eq!(&ts[10..11], "T");
        assert_eq!(&ts[19..20], ".");
    }

    #[test]
    fn urlencode_keeps_unreserved_and_escapes_others() {
        assert_eq!(urlencode("abc-_.~"), "abc-_.~");
        assert_eq!(urlencode("a b/c"), "a%20b%2Fc");
        assert_eq!(urlencode("=&?"), "%3D%26%3F");
    }

    #[test]
    fn build_query_drops_none_and_orders_pairs() {
        let q = build_query(&[
            ("limit", Some("10")),
            ("category", None),
            ("page", Some("2")),
        ]);
        assert_eq!(q, "limit=10&page=2");
    }
}
