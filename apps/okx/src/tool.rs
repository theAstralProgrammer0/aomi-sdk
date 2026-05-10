//! Curated tool layer for OKX V5. Hand-written from the
//! progenitor-generated client in `aomi_ext::okx` — see `ext/specs/okx.yaml`
//! for the underlying surface and `ext/src/okx/auth.rs` for the
//! HMAC-SHA256 + base64 signing helpers.
//!
//! Designed for the user story: trade SPOT/SWAP/FUTURES on OKX — check
//! prices/depth/candles, place and cancel orders, view balance/positions,
//! adjust leverage.
//!
//! 8 curated tools (preserved from the prior hand-written client):
//!   * okx_get_tickers     — tickers across an instType
//!   * okx_get_order_book  — bids/asks for an instrument
//!   * okx_get_candles     — OHLCV candles
//!   * okx_place_order     — place an order (signed)
//!   * okx_cancel_order    — cancel an open order (signed)
//!   * okx_get_balance     — unified-account balance (signed)
//!   * okx_get_positions   — open derivative positions (signed)
//!   * okx_set_leverage    — change leverage on an instrument (signed)

use aomi_ext::okx::types::{CancelOrderRequest, PlaceOrderRequest, SetLeverageRequest};
use aomi_ext::okx::{iso_timestamp, sign, Client as OkxClient};
use aomi_sdk::schemars::JsonSchema;
use aomi_sdk::*;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

#[derive(Clone, Default)]
pub(crate) struct OkxApp;

const BASE_URL: &str = "https://www.okx.com";

// ============================================================================
// Helpers
// ============================================================================

fn ok<T: Serialize>(value: T) -> Result<Value, String> {
    let value = serde_json::to_value(value)
        .map_err(|e| format!("[okx] failed to serialize response: {e}"))?;
    Ok(match value {
        Value::Object(mut map) => {
            map.insert("source".to_string(), Value::String("okx".to_string()));
            Value::Object(map)
        }
        other => json!({ "source": "okx", "data": other }),
    })
}

fn rt() -> Result<tokio::runtime::Runtime, String> {
    tokio::runtime::Runtime::new().map_err(|e| format!("[okx] runtime: {e}"))
}

fn resolve_creds(
    api_key: Option<&str>,
    secret_key: Option<&str>,
    passphrase: Option<&str>,
) -> Result<(String, String, String), String> {
    let api = resolve_secret_value(
        api_key,
        "OKX_API_KEY",
        "[okx] missing api_key argument and OKX_API_KEY environment variable",
    )?;
    let sec = resolve_secret_value(
        secret_key,
        "OKX_SECRET_KEY",
        "[okx] missing secret_key argument and OKX_SECRET_KEY environment variable",
    )?;
    let pass = resolve_secret_value(
        passphrase,
        "OKX_PASSPHRASE",
        "[okx] missing passphrase argument and OKX_PASSPHRASE environment variable",
    )?;
    Ok((api, sec, pass))
}

/// Build a query string in the order params will appear on the wire (matches
/// the order the generated client emits `.query(...)` in). Pairs whose value
/// is None are skipped. Returns the suffix without leading `?` (callers add
/// it when constructing `requestPath` for the signature prehash).
fn build_query(pairs: &[(&str, Option<&str>)]) -> String {
    pairs
        .iter()
        .filter_map(|(k, v)| v.map(|val| format!("{}={}", k, urlencode(val))))
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

// ============================================================================
// Tool 1: GetTickers (public)
// ============================================================================

pub(crate) struct GetTickers;

#[derive(Debug, Deserialize, JsonSchema)]
pub(crate) struct GetTickersArgs {
    /// Instrument type: SPOT, SWAP, FUTURES, or OPTION
    pub(crate) inst_type: String,
}

impl DynAomiTool for GetTickers {
    type App = OkxApp;
    type Args = GetTickersArgs;
    const NAME: &'static str = "okx_get_tickers";
    const DESCRIPTION: &'static str = "Use when the user asks for prices or 24h stats across an OKX category. Returns last price, volume, and 24h change for every instrument of the given type (SPOT, SWAP, FUTURES, OPTION).";

    fn run(_app: &OkxApp, args: Self::Args, _ctx: DynToolCallCtx) -> Result<Value, String> {
        let runtime = rt()?;
        runtime.block_on(async move {
            let client = OkxClient::new(BASE_URL);
            let resp = client
                .get_tickers(args.inst_type.as_str())
                .await
                .map_err(|e| format!("[okx] get_tickers: {e}"))?
                .into_inner();
            ok(resp)
        })
    }
}

// ============================================================================
// Tool 2: GetOrderBook (public)
// ============================================================================

pub(crate) struct GetOrderBook;

#[derive(Debug, Deserialize, JsonSchema)]
pub(crate) struct GetOrderBookArgs {
    /// Instrument ID, e.g. BTC-USDT, BTC-USDT-SWAP
    pub(crate) inst_id: String,
    /// Order book depth (max 400). Default 1.
    pub(crate) sz: Option<String>,
}

impl DynAomiTool for GetOrderBook {
    type App = OkxApp;
    type Args = GetOrderBookArgs;
    const NAME: &'static str = "okx_get_order_book";
    const DESCRIPTION: &'static str =
        "Use when the user wants order-book depth for an OKX instrument (e.g. before a limit order). Returns bid/ask levels with sizes. sz is depth (max 400; default 1 — pass e.g. \"50\" for a useful snapshot).";

    fn run(_app: &OkxApp, args: Self::Args, _ctx: DynToolCallCtx) -> Result<Value, String> {
        let runtime = rt()?;
        runtime.block_on(async move {
            let client = OkxClient::new(BASE_URL);
            let resp = client
                .get_order_book(args.inst_id.as_str(), args.sz.as_deref())
                .await
                .map_err(|e| format!("[okx] get_order_book: {e}"))?
                .into_inner();
            ok(resp)
        })
    }
}

// ============================================================================
// Tool 3: GetCandles (public)
// ============================================================================

pub(crate) struct GetCandles;

#[derive(Debug, Deserialize, JsonSchema)]
pub(crate) struct GetCandlesArgs {
    /// Instrument ID, e.g. BTC-USDT
    pub(crate) inst_id: String,
    /// Bar size, e.g. 1m, 5m, 15m, 30m, 1H, 4H, 1D, 1W, 1M
    pub(crate) bar: Option<String>,
    /// Pagination: return records newer than this timestamp (ms)
    pub(crate) after: Option<String>,
    /// Pagination: return records older than this timestamp (ms)
    pub(crate) before: Option<String>,
    /// Number of results (max 300, default 100)
    pub(crate) limit: Option<String>,
}

impl DynAomiTool for GetCandles {
    type App = OkxApp;
    type Args = GetCandlesArgs;
    const NAME: &'static str = "okx_get_candles";
    const DESCRIPTION: &'static str = "Use when the user asks for OKX price history or chart data. Returns OHLCV candles for an instrument. bar values: 1m, 3m, 5m, 15m, 30m, 1H, 2H, 4H, 6H, 12H, 1D, 1W, 1M. Default 100 candles, max 300.";

    fn run(_app: &OkxApp, args: Self::Args, _ctx: DynToolCallCtx) -> Result<Value, String> {
        let runtime = rt()?;
        runtime.block_on(async move {
            let client = OkxClient::new(BASE_URL);
            let resp = client
                .get_candles(
                    args.after.as_deref(),
                    args.bar.as_deref(),
                    args.before.as_deref(),
                    args.inst_id.as_str(),
                    args.limit.as_deref(),
                )
                .await
                .map_err(|e| format!("[okx] get_candles: {e}"))?
                .into_inner();
            ok(resp)
        })
    }
}

// ============================================================================
// Tool 4: PlaceOrder (signed)
// ============================================================================

pub(crate) struct PlaceOrder;

#[derive(Debug, Deserialize, JsonSchema)]
pub(crate) struct PlaceOrderArgs {
    /// OKX API key
    pub(crate) api_key: Option<String>,
    /// OKX API secret key
    pub(crate) secret_key: Option<String>,
    /// OKX API passphrase
    pub(crate) passphrase: Option<String>,
    /// Instrument ID, e.g. BTC-USDT
    pub(crate) inst_id: String,
    /// Trade mode: cash, cross, or isolated
    pub(crate) td_mode: String,
    /// Order side: buy or sell
    pub(crate) side: String,
    /// Order type: market, limit, post_only, fok, ioc
    pub(crate) ord_type: String,
    /// Quantity to trade
    pub(crate) sz: String,
    /// Price (required for limit orders)
    pub(crate) px: Option<String>,
}

impl DynAomiTool for PlaceOrder {
    type App = OkxApp;
    type Args = PlaceOrderArgs;
    const NAME: &'static str = "okx_place_order";
    const DESCRIPTION: &'static str = "Use when the user wants to place an OKX order. tdMode is 'cash' for spot, 'cross' or 'isolated' for derivatives (mismatch is the most common rejection). For limit orders pass px; for market orders omit it. Reads OKX_API_KEY/OKX_SECRET_KEY/OKX_PASSPHRASE from env if not passed.";

    fn run(_app: &OkxApp, args: Self::Args, _ctx: DynToolCallCtx) -> Result<Value, String> {
        let (api_key, secret_key, passphrase) = resolve_creds(
            args.api_key.as_deref(),
            args.secret_key.as_deref(),
            args.passphrase.as_deref(),
        )?;
        let body = PlaceOrderRequest {
            inst_id: args.inst_id,
            td_mode: args.td_mode,
            side: args.side,
            ord_type: args.ord_type,
            sz: args.sz,
            px: args.px,
        };
        let body_str =
            serde_json::to_string(&body).map_err(|e| format!("[okx] serialize body: {e}"))?;
        let timestamp = iso_timestamp();
        let signature = sign(&secret_key, &timestamp, "POST", "/api/v5/trade/order", &body_str)?;
        let runtime = rt()?;
        runtime.block_on(async move {
            let client = OkxClient::new(BASE_URL);
            let resp = client
                .place_order(
                    api_key.as_str(),
                    passphrase.as_str(),
                    signature.as_str(),
                    timestamp.as_str(),
                    &body,
                )
                .await
                .map_err(|e| format!("[okx] place_order: {e}"))?
                .into_inner();
            ok(resp)
        })
    }
}

// ============================================================================
// Tool 5: CancelOrder (signed)
// ============================================================================

pub(crate) struct CancelOrder;

#[derive(Debug, Deserialize, JsonSchema)]
pub(crate) struct CancelOrderArgs {
    pub(crate) api_key: Option<String>,
    pub(crate) secret_key: Option<String>,
    pub(crate) passphrase: Option<String>,
    /// Instrument ID, e.g. BTC-USDT
    pub(crate) inst_id: String,
    /// Order ID to cancel
    pub(crate) ord_id: String,
}

impl DynAomiTool for CancelOrder {
    type App = OkxApp;
    type Args = CancelOrderArgs;
    const NAME: &'static str = "okx_cancel_order";
    const DESCRIPTION: &'static str =
        "Use when the user wants to cancel an open OKX order. Pass instId and the ordId returned by place_order. Reads OKX credentials from env if not passed.";

    fn run(_app: &OkxApp, args: Self::Args, _ctx: DynToolCallCtx) -> Result<Value, String> {
        let (api_key, secret_key, passphrase) = resolve_creds(
            args.api_key.as_deref(),
            args.secret_key.as_deref(),
            args.passphrase.as_deref(),
        )?;
        let body = CancelOrderRequest {
            inst_id: args.inst_id,
            ord_id: args.ord_id,
        };
        let body_str =
            serde_json::to_string(&body).map_err(|e| format!("[okx] serialize body: {e}"))?;
        let timestamp = iso_timestamp();
        let signature = sign(
            &secret_key,
            &timestamp,
            "POST",
            "/api/v5/trade/cancel-order",
            &body_str,
        )?;
        let runtime = rt()?;
        runtime.block_on(async move {
            let client = OkxClient::new(BASE_URL);
            let resp = client
                .cancel_order(
                    api_key.as_str(),
                    passphrase.as_str(),
                    signature.as_str(),
                    timestamp.as_str(),
                    &body,
                )
                .await
                .map_err(|e| format!("[okx] cancel_order: {e}"))?
                .into_inner();
            ok(resp)
        })
    }
}

// ============================================================================
// Tool 6: GetBalance (signed)
// ============================================================================

pub(crate) struct GetBalance;

#[derive(Debug, Deserialize, JsonSchema)]
pub(crate) struct GetBalanceArgs {
    pub(crate) api_key: Option<String>,
    pub(crate) secret_key: Option<String>,
    pub(crate) passphrase: Option<String>,
    /// Optional comma-separated currency list, e.g. BTC,USDT
    pub(crate) ccy: Option<String>,
}

impl DynAomiTool for GetBalance {
    type App = OkxApp;
    type Args = GetBalanceArgs;
    const NAME: &'static str = "okx_get_balance";
    const DESCRIPTION: &'static str = "Use when the user asks about their OKX balance. Returns unified-account balances; optional ccy is a comma-separated currency list (e.g. \"BTC,USDT\"). Reads OKX credentials from env if not passed.";

    fn run(_app: &OkxApp, args: Self::Args, _ctx: DynToolCallCtx) -> Result<Value, String> {
        let (api_key, secret_key, passphrase) = resolve_creds(
            args.api_key.as_deref(),
            args.secret_key.as_deref(),
            args.passphrase.as_deref(),
        )?;
        // GET requests sign over `request_path` including query string.
        let q = build_query(&[("ccy", args.ccy.as_deref())]);
        let request_path = if q.is_empty() {
            "/api/v5/account/balance".to_string()
        } else {
            format!("/api/v5/account/balance?{q}")
        };
        let timestamp = iso_timestamp();
        let signature = sign(&secret_key, &timestamp, "GET", &request_path, "")?;
        let runtime = rt()?;
        runtime.block_on(async move {
            let client = OkxClient::new(BASE_URL);
            let resp = client
                .get_balance(
                    args.ccy.as_deref(),
                    api_key.as_str(),
                    passphrase.as_str(),
                    signature.as_str(),
                    timestamp.as_str(),
                )
                .await
                .map_err(|e| format!("[okx] get_balance: {e}"))?
                .into_inner();
            ok(resp)
        })
    }
}

// ============================================================================
// Tool 7: GetPositions (signed)
// ============================================================================

pub(crate) struct GetPositions;

#[derive(Debug, Deserialize, JsonSchema)]
pub(crate) struct GetPositionsArgs {
    pub(crate) api_key: Option<String>,
    pub(crate) secret_key: Option<String>,
    pub(crate) passphrase: Option<String>,
    /// Instrument type: SPOT, SWAP, FUTURES, OPTION (optional)
    pub(crate) inst_type: Option<String>,
    /// Instrument ID (optional)
    pub(crate) inst_id: Option<String>,
}

impl DynAomiTool for GetPositions {
    type App = OkxApp;
    type Args = GetPositionsArgs;
    const NAME: &'static str = "okx_get_positions";
    const DESCRIPTION: &'static str = "Use when the user asks about their open derivative positions on OKX. Optionally scope by instType (SWAP/FUTURES/OPTION) and/or instId. Reads OKX credentials from env if not passed.";

    fn run(_app: &OkxApp, args: Self::Args, _ctx: DynToolCallCtx) -> Result<Value, String> {
        let (api_key, secret_key, passphrase) = resolve_creds(
            args.api_key.as_deref(),
            args.secret_key.as_deref(),
            args.passphrase.as_deref(),
        )?;
        // wire order from get_positions: instId, instType.
        let q = build_query(&[
            ("instId", args.inst_id.as_deref()),
            ("instType", args.inst_type.as_deref()),
        ]);
        let request_path = if q.is_empty() {
            "/api/v5/account/positions".to_string()
        } else {
            format!("/api/v5/account/positions?{q}")
        };
        let timestamp = iso_timestamp();
        let signature = sign(&secret_key, &timestamp, "GET", &request_path, "")?;
        let runtime = rt()?;
        runtime.block_on(async move {
            let client = OkxClient::new(BASE_URL);
            let resp = client
                .get_positions(
                    args.inst_id.as_deref(),
                    args.inst_type.as_deref(),
                    api_key.as_str(),
                    passphrase.as_str(),
                    signature.as_str(),
                    timestamp.as_str(),
                )
                .await
                .map_err(|e| format!("[okx] get_positions: {e}"))?
                .into_inner();
            ok(resp)
        })
    }
}

// ============================================================================
// Tool 8: SetLeverage (signed)
// ============================================================================

pub(crate) struct SetLeverage;

#[derive(Debug, Deserialize, JsonSchema)]
pub(crate) struct SetLeverageArgs {
    pub(crate) api_key: Option<String>,
    pub(crate) secret_key: Option<String>,
    pub(crate) passphrase: Option<String>,
    /// Instrument ID, e.g. BTC-USDT-SWAP
    pub(crate) inst_id: String,
    /// Leverage ratio, e.g. "10"
    pub(crate) lever: String,
    /// Margin mode: cross or isolated
    pub(crate) mgn_mode: String,
}

impl DynAomiTool for SetLeverage {
    type App = OkxApp;
    type Args = SetLeverageArgs;
    const NAME: &'static str = "okx_set_leverage";
    const DESCRIPTION: &'static str = "Use when the user wants to change leverage on an OKX instrument before trading derivatives. lever is a string (e.g. \"10\"). mgnMode must match the tdMode planned for the order: 'cross' or 'isolated'. Reads OKX credentials from env if not passed.";

    fn run(_app: &OkxApp, args: Self::Args, _ctx: DynToolCallCtx) -> Result<Value, String> {
        let (api_key, secret_key, passphrase) = resolve_creds(
            args.api_key.as_deref(),
            args.secret_key.as_deref(),
            args.passphrase.as_deref(),
        )?;
        let body = SetLeverageRequest {
            inst_id: args.inst_id,
            lever: args.lever,
            mgn_mode: args.mgn_mode,
        };
        let body_str =
            serde_json::to_string(&body).map_err(|e| format!("[okx] serialize body: {e}"))?;
        let timestamp = iso_timestamp();
        let signature = sign(
            &secret_key,
            &timestamp,
            "POST",
            "/api/v5/account/set-leverage",
            &body_str,
        )?;
        let runtime = rt()?;
        runtime.block_on(async move {
            let client = OkxClient::new(BASE_URL);
            let resp = client
                .set_leverage(
                    api_key.as_str(),
                    passphrase.as_str(),
                    signature.as_str(),
                    timestamp.as_str(),
                    &body,
                )
                .await
                .map_err(|e| format!("[okx] set_leverage: {e}"))?
                .into_inner();
            ok(resp)
        })
    }
}
