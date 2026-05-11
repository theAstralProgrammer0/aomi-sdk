//! Client layer for the byreal app.
//!
//! Three concrete clients live here, one per byreal product line:
//!   * [`perps`]  — Hyperliquid perpetuals (`api.hyperliquid.xyz`)
//!   * [`spot`]   — byreal spot / CLMM / RFQ on Solana (`api2.byreal.io`)
//!   * [`lp`]     — byreal Copy Farming + incentives on Solana (`api2.byreal.io`)
//!
//! All three follow the same shape:
//! - A unit-struct `*Client` with a `reqwest::blocking::Client` + base URL.
//! - A `*_client()` accessor backed by `OnceLock` for lazy init.
//! - Plain HTTP wrappers (`get`, `post`) that surface non-2xx as `Err(String)`
//!   and decode the JSON envelope into typed responses.
//!
//! [`ByrealApp`] is the empty marker struct every `DynAomiTool::App` points to.

pub(crate) mod lp;
pub(crate) mod perps;
pub(crate) mod spot;

use serde::Serialize;
use serde_json::Value;
use std::time::Duration;

#[derive(Clone, Default)]
pub(crate) struct ByrealApp;

/// Default base URL for the byreal Solana-side API. Both [`spot`] and [`lp`]
/// hit this host (perps is separate, talks straight to Hyperliquid).
pub(crate) const BYREAL_API_BASE: &str = "https://api2.byreal.io";

/// Build a blocking reqwest client with sane defaults for byreal calls.
pub(crate) fn http_client() -> Result<reqwest::blocking::Client, String> {
    reqwest::blocking::Client::builder()
        .timeout(Duration::from_secs(30))
        .build()
        .map_err(|e| format!("[byreal] failed to build HTTP client: {e}"))
}

/// byreal's response envelope. Every endpoint wraps the actual payload under
/// `result.data`; the outer `retCode`/`retMsg` carries transport-level status.
#[derive(serde::Deserialize)]
struct ByrealEnvelope {
    #[serde(rename = "retCode")]
    ret_code: i64,
    #[serde(rename = "retMsg", default)]
    ret_msg: String,
    #[serde(default)]
    result: Option<ByrealEnvelopeResult>,
}

#[derive(serde::Deserialize)]
struct ByrealEnvelopeResult {
    #[serde(default)]
    data: Option<Value>,
    // Some endpoints (e.g. swap quote) put the data fields directly at the
    // result level instead of under `result.data`. We capture the rest so
    // callers can fall back to the whole `result` object when `data` is absent.
    #[serde(flatten)]
    rest: serde_json::Map<String, Value>,
}

/// GET helper. Strips the byreal envelope and returns the inner `data`
/// (falling back to the full `result` object for endpoints that flatten).
pub(crate) fn byreal_get(
    http: &reqwest::blocking::Client,
    url: &str,
    query: &[(&str, String)],
) -> Result<Value, String> {
    let resp = http
        .get(url)
        .query(query)
        .send()
        .map_err(|e| format!("[byreal] GET {url} failed: {e}"))?;
    decode_envelope(resp, url)
}

/// POST helper with JSON body.
pub(crate) fn byreal_post<B: Serialize>(
    http: &reqwest::blocking::Client,
    url: &str,
    body: &B,
) -> Result<Value, String> {
    let resp = http
        .post(url)
        .json(body)
        .send()
        .map_err(|e| format!("[byreal] POST {url} failed: {e}"))?;
    decode_envelope(resp, url)
}

fn decode_envelope(resp: reqwest::blocking::Response, url: &str) -> Result<Value, String> {
    let status = resp.status();
    let text = resp.text().unwrap_or_default();
    if !status.is_success() {
        return Err(format!("[byreal] {url} returned HTTP {status}: {text}"));
    }
    let env: ByrealEnvelope = serde_json::from_str(&text)
        .map_err(|e| format!("[byreal] {url} envelope decode failed: {e}; body: {text}"))?;
    if env.ret_code != 0 {
        return Err(format!(
            "[byreal] {url} returned retCode={} retMsg={}",
            env.ret_code, env.ret_msg
        ));
    }
    match env.result {
        Some(r) => Ok(r.data.unwrap_or(Value::Object(r.rest))),
        None => Ok(Value::Null),
    }
}
