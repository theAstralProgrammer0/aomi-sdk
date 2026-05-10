use aomi_ext::okx::{CancelOrderRequest, OkxClient, PlaceOrderRequest, SetLeverageRequest};
use aomi_sdk::*;
use aomi_sdk::schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Clone, Default)]
pub(crate) struct OkxApp;

fn ok<T: Serialize>(value: T) -> Result<Value, String> {
    let value = serde_json::to_value(value)
        .map_err(|e| format!("[okx] failed to serialize response: {e}"))?;
    Ok(match value {
        Value::Object(mut map) => {
            map.insert("source".to_string(), Value::String("okx".to_string()));
            Value::Object(map)
        }
        other => serde_json::json!({ "source": "okx", "data": other }),
    })
}

fn resolve_okx_credentials(
    api_key: Option<&str>,
    secret_key: Option<&str>,
    passphrase: Option<&str>,
) -> Result<(String, String, String), String> {
    let api_key = resolve_secret_value(
        api_key,
        "OKX_API_KEY",
        "[okx] missing api_key argument and OKX_API_KEY environment variable",
    )?;
    let secret_key = resolve_secret_value(
        secret_key,
        "OKX_SECRET_KEY",
        "[okx] missing secret_key argument and OKX_SECRET_KEY environment variable",
    )?;
    let passphrase = resolve_secret_value(
        passphrase,
        "OKX_PASSPHRASE",
        "[okx] missing passphrase argument and OKX_PASSPHRASE environment variable",
    )?;
    Ok((api_key, secret_key, passphrase))
}

// ============================================================================
// Tool arg structs
// ============================================================================

#[derive(Debug, Deserialize, JsonSchema)]
pub(crate) struct GetTickersArgs {
    /// Instrument type: SPOT, SWAP, FUTURES, or OPTION
    pub(crate) inst_type: String,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub(crate) struct GetOrderBookArgs {
    /// Instrument ID, e.g. BTC-USDT, BTC-USDT-SWAP
    pub(crate) inst_id: String,
    /// Order book depth (max 400). Default 1.
    pub(crate) sz: Option<String>,
}

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

#[derive(Debug, Deserialize, JsonSchema)]
pub(crate) struct CancelOrderArgs {
    /// OKX API key
    pub(crate) api_key: Option<String>,
    /// OKX API secret key
    pub(crate) secret_key: Option<String>,
    /// OKX API passphrase
    pub(crate) passphrase: Option<String>,
    /// Instrument ID, e.g. BTC-USDT
    pub(crate) inst_id: String,
    /// Order ID to cancel
    pub(crate) ord_id: String,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub(crate) struct GetBalanceArgs {
    /// OKX API key
    pub(crate) api_key: Option<String>,
    /// OKX API secret key
    pub(crate) secret_key: Option<String>,
    /// OKX API passphrase
    pub(crate) passphrase: Option<String>,
    /// Optional comma-separated currency list, e.g. BTC,USDT
    pub(crate) ccy: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub(crate) struct GetPositionsArgs {
    /// OKX API key
    pub(crate) api_key: Option<String>,
    /// OKX API secret key
    pub(crate) secret_key: Option<String>,
    /// OKX API passphrase
    pub(crate) passphrase: Option<String>,
    /// Instrument type: SPOT, SWAP, FUTURES, OPTION (optional)
    pub(crate) inst_type: Option<String>,
    /// Instrument ID (optional)
    pub(crate) inst_id: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub(crate) struct SetLeverageArgs {
    /// OKX API key
    pub(crate) api_key: Option<String>,
    /// OKX API secret key
    pub(crate) secret_key: Option<String>,
    /// OKX API passphrase
    pub(crate) passphrase: Option<String>,
    /// Instrument ID, e.g. BTC-USDT-SWAP
    pub(crate) inst_id: String,
    /// Leverage ratio, e.g. "10"
    pub(crate) lever: String,
    /// Margin mode: cross or isolated
    pub(crate) mgn_mode: String,
}

// ============================================================================
// Tool structs
// ============================================================================

pub(crate) struct GetTickers;
pub(crate) struct GetOrderBook;
pub(crate) struct GetCandles;
pub(crate) struct PlaceOrder;
pub(crate) struct CancelOrder;
pub(crate) struct GetBalance;
pub(crate) struct GetPositions;
pub(crate) struct SetLeverage;

// ============================================================================
// Tool 1: GetTickers — GET /market/tickers
// ============================================================================

impl DynAomiTool for GetTickers {
    type App = OkxApp;
    type Args = GetTickersArgs;
    const NAME: &'static str = "okx_get_tickers";
    const DESCRIPTION: &'static str = "Use when the user asks for prices or 24h stats across an OKX category. Returns last price, volume, and 24h change for every instrument of the given type (SPOT, SWAP, FUTURES, OPTION).";

    fn run(_app: &OkxApp, args: Self::Args, _ctx: DynToolCallCtx) -> Result<Value, String> {
        let client = OkxClient::new()?;
        let path = format!("/market/tickers?instType={}", args.inst_type);
        ok(client.public_get(&path)?)
    }
}

// ============================================================================
// Tool 2: GetOrderBook — GET /market/books
// ============================================================================

impl DynAomiTool for GetOrderBook {
    type App = OkxApp;
    type Args = GetOrderBookArgs;
    const NAME: &'static str = "okx_get_order_book";
    const DESCRIPTION: &'static str =
        "Use when the user wants order-book depth for an OKX instrument (e.g. before a limit order). Returns bid/ask levels with sizes. sz is depth (max 400; default 1 — pass e.g. \"50\" for a useful snapshot).";

    fn run(_app: &OkxApp, args: Self::Args, _ctx: DynToolCallCtx) -> Result<Value, String> {
        let client = OkxClient::new()?;
        let mut path = format!("/market/books?instId={}", args.inst_id);
        if let Some(ref sz) = args.sz {
            path.push_str(&format!("&sz={sz}"));
        }
        ok(client.public_get(&path)?)
    }
}

// ============================================================================
// Tool 3: GetCandles — GET /market/candles
// ============================================================================

impl DynAomiTool for GetCandles {
    type App = OkxApp;
    type Args = GetCandlesArgs;
    const NAME: &'static str = "okx_get_candles";
    const DESCRIPTION: &'static str = "Use when the user asks for OKX price history or chart data. Returns OHLCV candles for an instrument. bar values: 1m, 3m, 5m, 15m, 30m, 1H, 2H, 4H, 6H, 12H, 1D, 1W, 1M. Default 100 candles, max 300.";

    fn run(_app: &OkxApp, args: Self::Args, _ctx: DynToolCallCtx) -> Result<Value, String> {
        let client = OkxClient::new()?;
        let mut path = format!("/market/candles?instId={}", args.inst_id);
        if let Some(ref bar) = args.bar {
            path.push_str(&format!("&bar={bar}"));
        }
        if let Some(ref after) = args.after {
            path.push_str(&format!("&after={after}"));
        }
        if let Some(ref before) = args.before {
            path.push_str(&format!("&before={before}"));
        }
        if let Some(ref limit) = args.limit {
            path.push_str(&format!("&limit={limit}"));
        }
        ok(client.public_get(&path)?)
    }
}

// ============================================================================
// Tool 4: PlaceOrder — POST /trade/order
// ============================================================================

impl DynAomiTool for PlaceOrder {
    type App = OkxApp;
    type Args = PlaceOrderArgs;
    const NAME: &'static str = "okx_place_order";
    const DESCRIPTION: &'static str = "Use when the user wants to place an OKX order. tdMode is 'cash' for spot, 'cross' or 'isolated' for derivatives (mismatch is the most common rejection). For limit orders pass px; for market orders omit it. Reads OKX_API_KEY/OKX_SECRET_KEY/OKX_PASSPHRASE from env if not passed.";

    fn run(_app: &OkxApp, args: Self::Args, _ctx: DynToolCallCtx) -> Result<Value, String> {
        let client = OkxClient::new()?;
        let (api_key, secret_key, passphrase) = resolve_okx_credentials(
            args.api_key.as_deref(),
            args.secret_key.as_deref(),
            args.passphrase.as_deref(),
        )?;
        let body = PlaceOrderRequest {
            inst_id: &args.inst_id,
            td_mode: &args.td_mode,
            side: &args.side,
            ord_type: &args.ord_type,
            sz: &args.sz,
            px: args.px.as_deref(),
        };
        let path = "/trade/order";
        ok(client.auth_post(path, &body, &api_key, &secret_key, &passphrase)?)
    }
}

// ============================================================================
// Tool 5: CancelOrder — POST /trade/cancel-order
// ============================================================================

impl DynAomiTool for CancelOrder {
    type App = OkxApp;
    type Args = CancelOrderArgs;
    const NAME: &'static str = "okx_cancel_order";
    const DESCRIPTION: &'static str =
        "Use when the user wants to cancel an open OKX order. Pass instId and the ordId returned by place_order. Reads OKX credentials from env if not passed.";

    fn run(_app: &OkxApp, args: Self::Args, _ctx: DynToolCallCtx) -> Result<Value, String> {
        let client = OkxClient::new()?;
        let (api_key, secret_key, passphrase) = resolve_okx_credentials(
            args.api_key.as_deref(),
            args.secret_key.as_deref(),
            args.passphrase.as_deref(),
        )?;
        let body = CancelOrderRequest {
            inst_id: &args.inst_id,
            ord_id: &args.ord_id,
        };
        let path = "/trade/cancel-order";
        ok(client.auth_post(path, &body, &api_key, &secret_key, &passphrase)?)
    }
}

// ============================================================================
// Tool 6: GetBalance — GET /account/balance
// ============================================================================

impl DynAomiTool for GetBalance {
    type App = OkxApp;
    type Args = GetBalanceArgs;
    const NAME: &'static str = "okx_get_balance";
    const DESCRIPTION: &'static str = "Use when the user asks about their OKX balance. Returns unified-account balances; optional ccy is a comma-separated currency list (e.g. \"BTC,USDT\"). Reads OKX credentials from env if not passed.";

    fn run(_app: &OkxApp, args: Self::Args, _ctx: DynToolCallCtx) -> Result<Value, String> {
        let client = OkxClient::new()?;
        let (api_key, secret_key, passphrase) = resolve_okx_credentials(
            args.api_key.as_deref(),
            args.secret_key.as_deref(),
            args.passphrase.as_deref(),
        )?;
        let mut path = "/account/balance".to_string();
        if let Some(ref ccy) = args.ccy {
            path.push_str(&format!("?ccy={ccy}"));
        }
        ok(client.auth_get(&path, &api_key, &secret_key, &passphrase)?)
    }
}

// ============================================================================
// Tool 7: GetPositions — GET /account/positions
// ============================================================================

impl DynAomiTool for GetPositions {
    type App = OkxApp;
    type Args = GetPositionsArgs;
    const NAME: &'static str = "okx_get_positions";
    const DESCRIPTION: &'static str = "Use when the user asks about their open derivative positions on OKX. Optionally scope by instType (SWAP/FUTURES/OPTION) and/or instId. Reads OKX credentials from env if not passed.";

    fn run(_app: &OkxApp, args: Self::Args, _ctx: DynToolCallCtx) -> Result<Value, String> {
        let client = OkxClient::new()?;
        let (api_key, secret_key, passphrase) = resolve_okx_credentials(
            args.api_key.as_deref(),
            args.secret_key.as_deref(),
            args.passphrase.as_deref(),
        )?;
        let mut path = "/account/positions".to_string();
        let mut params = Vec::new();
        if let Some(ref inst_type) = args.inst_type {
            params.push(format!("instType={inst_type}"));
        }
        if let Some(ref inst_id) = args.inst_id {
            params.push(format!("instId={inst_id}"));
        }
        if !params.is_empty() {
            path.push('?');
            path.push_str(&params.join("&"));
        }
        ok(client.auth_get(&path, &api_key, &secret_key, &passphrase)?)
    }
}

// ============================================================================
// Tool 8: SetLeverage — POST /account/set-leverage
// ============================================================================

impl DynAomiTool for SetLeverage {
    type App = OkxApp;
    type Args = SetLeverageArgs;
    const NAME: &'static str = "okx_set_leverage";
    const DESCRIPTION: &'static str = "Use when the user wants to change leverage on an OKX instrument before trading derivatives. lever is a string (e.g. \"10\"). mgnMode must match the tdMode planned for the order: 'cross' or 'isolated'. Reads OKX credentials from env if not passed.";

    fn run(_app: &OkxApp, args: Self::Args, _ctx: DynToolCallCtx) -> Result<Value, String> {
        let client = OkxClient::new()?;
        let (api_key, secret_key, passphrase) = resolve_okx_credentials(
            args.api_key.as_deref(),
            args.secret_key.as_deref(),
            args.passphrase.as_deref(),
        )?;
        let body = SetLeverageRequest {
            inst_id: &args.inst_id,
            lever: &args.lever,
            mgn_mode: &args.mgn_mode,
        };
        let path = "/account/set-leverage";
        ok(client.auth_post(path, &body, &api_key, &secret_key, &passphrase)?)
    }
}
