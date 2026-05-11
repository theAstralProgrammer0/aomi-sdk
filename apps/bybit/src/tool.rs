//! Curated tool layer for Bybit V5 unified API. Hand-written from the
//! generated client in `aomi_ext::bybit` — see `ext/specs/bybit.yaml` for
//! the underlying surface and `ext/src/bybit/auth.rs` for the HMAC signing
//! helpers.
//!
//! Designed for the user story: trade BTC/ETH on Bybit perpetuals — check
//! prices/depth, place limit and market orders, manage leverage, and view
//! balance / positions / order history.
//!
//! The 11 mechanical stubs from `aomi-build gen-tool` were collapsed into
//! 12 user-centric tools (the create-order stub split into separate
//! `place_limit_order` and `place_market_order` so the LLM can't mix
//! up which fields to fill).

use aomi_ext::bybit::types::{
    AmendOrderRequest, CancelOrderRequest, CreateOrderRequest, SetLeverageRequest,
};
use aomi_ext::bybit::{
    Client as BybitClient, RECV_WINDOW, build_query, current_timestamp_ms, sign_body, sign_query,
};
use aomi_sdk::schemars::JsonSchema;
use aomi_sdk::*;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

#[derive(Clone, Default)]
pub(crate) struct BybitApp;

const BASE_URL: &str = "https://api.bybit.com";

// ============================================================================
// Helpers
// ============================================================================

fn ok<T: Serialize>(value: T) -> Result<Value, String> {
    let value = serde_json::to_value(value).map_err(|e| format!("[bybit] serialize: {e}"))?;
    Ok(match value {
        Value::Object(mut m) => {
            m.insert("source".into(), Value::String("bybit".into()));
            Value::Object(m)
        }
        other => json!({ "source": "bybit", "data": other }),
    })
}

fn rt() -> Result<tokio::runtime::Runtime, String> {
    tokio::runtime::Runtime::new().map_err(|e| format!("[bybit] runtime: {e}"))
}

fn resolve_creds(
    api_key: Option<&str>,
    secret_key: Option<&str>,
) -> Result<(String, String), String> {
    let api = resolve_secret_value(
        api_key,
        "BYBIT_API_KEY",
        "[bybit] missing api_key argument and BYBIT_API_KEY env var",
    )?;
    let sec = resolve_secret_value(
        secret_key,
        "BYBIT_SECRET_KEY",
        "[bybit] missing secret_key argument and BYBIT_SECRET_KEY env var",
    )?;
    Ok((api, sec))
}

/// Sign a JSON body for a POST request, returning `(timestamp, signature)`.
/// We serialise the body once with `serde_json::to_string` to compute the
/// signature; the generated client then re-serialises the same value via
/// `.json(&body)` (also `serde_json`), producing the identical bytes.
fn sign_post<T: Serialize>(
    api_key: &str,
    secret: &str,
    body: &T,
) -> Result<(String, String), String> {
    let body_str =
        serde_json::to_string(body).map_err(|e| format!("[bybit] serialise body: {e}"))?;
    let ts = current_timestamp_ms();
    let sig = sign_body(&ts, api_key, secret, &body_str);
    Ok((ts, sig))
}

/// Sign an ordered list of GET query params. Returns `(timestamp, signature)`.
/// The pairs MUST match what the generated client will actually emit on the
/// wire — pass the same `(key, Some(value))` set into the client method.
fn sign_get(api_key: &str, secret: &str, pairs: &[(&str, Option<&str>)]) -> (String, String) {
    let q = build_query(pairs);
    let ts = current_timestamp_ms();
    let sig = sign_query(&ts, api_key, secret, &q);
    (ts, sig)
}

// ============================================================================
// Tool 1: get_price — last price + 24h stats (public)
// ============================================================================

pub(crate) struct GetPrice;

#[derive(Debug, Deserialize, JsonSchema)]
pub(crate) struct GetPriceArgs {
    /// Product category: `spot`, `linear`, `inverse`, or `option`.
    pub category: String,
    /// Trading pair like `BTCUSDT`. Omit to return every symbol in the category.
    #[serde(default)]
    pub symbol: Option<String>,
}

impl DynAomiTool for GetPrice {
    type App = BybitApp;
    type Args = GetPriceArgs;
    const NAME: &'static str = "bybit_get_price";
    const DESCRIPTION: &'static str = "Latest price snapshot for one or all symbols in a category. Returns last price, 24h % change, 24h volume, best bid/ask. Use this for 'what's the price of X' or 'how is X moving today' style questions.";

    fn run(_app: &BybitApp, args: Self::Args, _ctx: DynToolCallCtx) -> Result<Value, String> {
        let runtime = rt()?;
        runtime.block_on(async move {
            let client = BybitClient::new(BASE_URL);
            let resp = client
                .get_tickers(None, args.category.as_str(), None, args.symbol.as_deref())
                .await
                .map_err(|e| format!("[bybit] get_tickers: {e}"))?
                .into_inner();
            ok(resp)
        })
    }
}

// ============================================================================
// Tool 2: get_orderbook — bids/asks (public)
// ============================================================================

pub(crate) struct GetOrderbook;

#[derive(Debug, Deserialize, JsonSchema)]
pub(crate) struct GetOrderbookArgs {
    /// Product category: `spot`, `linear`, `inverse`, or `option`.
    pub category: String,
    /// Trading pair like `BTCUSDT`.
    pub symbol: String,
    /// Depth — common values 1, 25, 50, 200. Default 25.
    #[serde(default)]
    pub limit: Option<i32>,
}

impl DynAomiTool for GetOrderbook {
    type App = BybitApp;
    type Args = GetOrderbookArgs;
    const NAME: &'static str = "bybit_get_orderbook";
    const DESCRIPTION: &'static str = "Bids and asks for a symbol at the requested depth. Use this when the user wants liquidity / spread / depth info, not just a price.";

    fn run(_app: &BybitApp, args: Self::Args, _ctx: DynToolCallCtx) -> Result<Value, String> {
        let runtime = rt()?;
        runtime.block_on(async move {
            let client = BybitClient::new(BASE_URL);
            let limit = args.limit.or(Some(25));
            let resp = client
                .get_orderbook(args.category.as_str(), limit, args.symbol.as_str())
                .await
                .map_err(|e| format!("[bybit] get_orderbook: {e}"))?
                .into_inner();
            ok(resp)
        })
    }
}

// ============================================================================
// Tool 3: get_candles — OHLCV (public)
// ============================================================================

pub(crate) struct GetCandles;

#[derive(Debug, Deserialize, JsonSchema)]
pub(crate) struct GetCandlesArgs {
    /// Product category: `spot`, `linear`, `inverse`, or `option`.
    pub category: String,
    /// Trading pair like `BTCUSDT`.
    pub symbol: String,
    /// Interval — `1`, `3`, `5`, `15`, `30`, `60`, `120`, `240`, `360`, `720`, `D`, `W`, `M`.
    pub interval: String,
    /// Optional start timestamp in milliseconds since epoch.
    #[serde(default)]
    pub start: Option<i64>,
    /// Optional end timestamp in milliseconds since epoch.
    #[serde(default)]
    pub end: Option<i64>,
    /// Optional max number of candles (server default ~200, max 1000).
    #[serde(default)]
    pub limit: Option<i32>,
}

impl DynAomiTool for GetCandles {
    type App = BybitApp;
    type Args = GetCandlesArgs;
    const NAME: &'static str = "bybit_get_candles";
    const DESCRIPTION: &'static str = "OHLCV candles for a symbol at the requested interval. Use this for chart-style questions (price history, volatility window, recent trend).";

    fn run(_app: &BybitApp, args: Self::Args, _ctx: DynToolCallCtx) -> Result<Value, String> {
        let runtime = rt()?;
        runtime.block_on(async move {
            let client = BybitClient::new(BASE_URL);
            let resp = client
                .get_kline(
                    args.category.as_str(),
                    args.end,
                    args.interval.as_str(),
                    args.limit,
                    args.start,
                    args.symbol.as_str(),
                )
                .await
                .map_err(|e| format!("[bybit] get_kline: {e}"))?
                .into_inner();
            ok(resp)
        })
    }
}

// ============================================================================
// Tool 4: get_wallet_balance — account equity (signed)
// ============================================================================

pub(crate) struct GetWalletBalance;

#[derive(Debug, Deserialize, JsonSchema)]
pub(crate) struct GetWalletBalanceArgs {
    /// Bybit API key (falls back to env var `BYBIT_API_KEY`).
    #[serde(default)]
    pub api_key: Option<String>,
    /// Bybit API secret (falls back to env var `BYBIT_SECRET_KEY`).
    #[serde(default)]
    pub secret_key: Option<String>,
    /// `UNIFIED` (default — unified trading account) or `CONTRACT` (legacy derivatives).
    #[serde(default)]
    pub account_type: Option<String>,
    /// Optional coin filter like `USDT`. Omit to get every coin.
    #[serde(default)]
    pub coin: Option<String>,
}

impl DynAomiTool for GetWalletBalance {
    type App = BybitApp;
    type Args = GetWalletBalanceArgs;
    const NAME: &'static str = "bybit_get_wallet_balance";
    const DESCRIPTION: &'static str = "Wallet equity and per-coin balance for the unified (UTA) or contract account. Use this for 'how much do I have' / 'what's my buying power' questions.";

    fn run(_app: &BybitApp, args: Self::Args, _ctx: DynToolCallCtx) -> Result<Value, String> {
        let (api_key, secret) = resolve_creds(args.api_key.as_deref(), args.secret_key.as_deref())?;
        let account_type = args.account_type.unwrap_or_else(|| "UNIFIED".to_string());
        let coin = args.coin;
        let runtime = rt()?;
        runtime.block_on(async move {
            let client = BybitClient::new(BASE_URL);
            let (ts, sig) = sign_get(
                &api_key,
                &secret,
                &[
                    ("accountType", Some(account_type.as_str())),
                    ("coin", coin.as_deref()),
                ],
            );
            let resp = client
                .get_wallet_balance(
                    account_type.as_str(),
                    coin.as_deref(),
                    api_key.as_str(),
                    RECV_WINDOW,
                    sig.as_str(),
                    ts.as_str(),
                )
                .await
                .map_err(|e| format!("[bybit] get_wallet_balance: {e}"))?
                .into_inner();
            ok(resp)
        })
    }
}

// ============================================================================
// Tool 5: get_positions — open derivative positions (signed)
// ============================================================================

pub(crate) struct GetPositions;

#[derive(Debug, Deserialize, JsonSchema)]
pub(crate) struct GetPositionsArgs {
    #[serde(default)]
    pub api_key: Option<String>,
    #[serde(default)]
    pub secret_key: Option<String>,
    /// `linear` (USDT/USDC perps) or `inverse` (coin-margined perps).
    pub category: String,
    /// Optional symbol filter like `BTCUSDT`.
    #[serde(default)]
    pub symbol: Option<String>,
    /// Optional settle-coin filter (`USDT`, `USDC`, `BTC`, ...).
    #[serde(default)]
    pub settle_coin: Option<String>,
}

impl DynAomiTool for GetPositions {
    type App = BybitApp;
    type Args = GetPositionsArgs;
    const NAME: &'static str = "bybit_get_positions";
    const DESCRIPTION: &'static str = "Open perpetual or inverse derivative positions with size, avg price, unrealised PnL, leverage, and liquidation price. Use this for 'what positions do I have' / 'what's my PnL on X' questions. Requires category=linear or inverse — perps only, not spot.";

    fn run(_app: &BybitApp, args: Self::Args, _ctx: DynToolCallCtx) -> Result<Value, String> {
        let (api_key, secret) = resolve_creds(args.api_key.as_deref(), args.secret_key.as_deref())?;
        let category = args.category;
        let symbol = args.symbol;
        let settle_coin = args.settle_coin;
        let runtime = rt()?;
        runtime.block_on(async move {
            let client = BybitClient::new(BASE_URL);
            let (ts, sig) = sign_get(
                &api_key,
                &secret,
                &[
                    ("category", Some(category.as_str())),
                    ("settleCoin", settle_coin.as_deref()),
                    ("symbol", symbol.as_deref()),
                ],
            );
            let resp = client
                .get_positions(
                    category.as_str(),
                    settle_coin.as_deref(),
                    symbol.as_deref(),
                    api_key.as_str(),
                    RECV_WINDOW,
                    sig.as_str(),
                    ts.as_str(),
                )
                .await
                .map_err(|e| format!("[bybit] get_positions: {e}"))?
                .into_inner();
            ok(resp)
        })
    }
}

// ============================================================================
// Tool 6: get_open_orders — unfilled orders right now (signed)
// ============================================================================

pub(crate) struct GetOpenOrders;

#[derive(Debug, Deserialize, JsonSchema)]
pub(crate) struct GetOpenOrdersArgs {
    #[serde(default)]
    pub api_key: Option<String>,
    #[serde(default)]
    pub secret_key: Option<String>,
    pub category: String,
    #[serde(default)]
    pub symbol: Option<String>,
    /// Optional specific orderId.
    #[serde(default)]
    pub order_id: Option<String>,
    #[serde(default)]
    pub limit: Option<i32>,
}

impl DynAomiTool for GetOpenOrders {
    type App = BybitApp;
    type Args = GetOpenOrdersArgs;
    const NAME: &'static str = "bybit_get_open_orders";
    const DESCRIPTION: &'static str = "Real-time list of unfilled / partially-filled orders. Use for 'what's still working' / 'do I have any resting orders'. For closed orders use bybit_get_order_history.";

    fn run(_app: &BybitApp, args: Self::Args, _ctx: DynToolCallCtx) -> Result<Value, String> {
        let (api_key, secret) = resolve_creds(args.api_key.as_deref(), args.secret_key.as_deref())?;
        let category = args.category;
        let symbol = args.symbol;
        let order_id = args.order_id;
        let limit = args.limit;
        let limit_str = limit.map(|l| l.to_string());
        let runtime = rt()?;
        runtime.block_on(async move {
            let client = BybitClient::new(BASE_URL);
            let (ts, sig) = sign_get(
                &api_key,
                &secret,
                &[
                    ("category", Some(category.as_str())),
                    ("limit", limit_str.as_deref()),
                    ("orderId", order_id.as_deref()),
                    ("symbol", symbol.as_deref()),
                ],
            );
            let resp = client
                .get_open_orders(
                    category.as_str(),
                    limit,
                    order_id.as_deref(),
                    symbol.as_deref(),
                    api_key.as_str(),
                    RECV_WINDOW,
                    sig.as_str(),
                    ts.as_str(),
                )
                .await
                .map_err(|e| format!("[bybit] get_open_orders: {e}"))?
                .into_inner();
            ok(resp)
        })
    }
}

// ============================================================================
// Tool 7: get_order_history — closed/cancelled orders (signed)
// ============================================================================

pub(crate) struct GetOrderHistory;

#[derive(Debug, Deserialize, JsonSchema)]
pub(crate) struct GetOrderHistoryArgs {
    #[serde(default)]
    pub api_key: Option<String>,
    #[serde(default)]
    pub secret_key: Option<String>,
    pub category: String,
    #[serde(default)]
    pub symbol: Option<String>,
    #[serde(default)]
    pub order_id: Option<String>,
    #[serde(default)]
    pub limit: Option<i32>,
}

impl DynAomiTool for GetOrderHistory {
    type App = BybitApp;
    type Args = GetOrderHistoryArgs;
    const NAME: &'static str = "bybit_get_order_history";
    const DESCRIPTION: &'static str = "Past orders that have been filled, cancelled, or rejected. Use for 'show me my recent trades' / 'did my last order go through' questions. For currently-working orders use bybit_get_open_orders.";

    fn run(_app: &BybitApp, args: Self::Args, _ctx: DynToolCallCtx) -> Result<Value, String> {
        let (api_key, secret) = resolve_creds(args.api_key.as_deref(), args.secret_key.as_deref())?;
        let category = args.category;
        let symbol = args.symbol;
        let order_id = args.order_id;
        let limit = args.limit;
        let limit_str = limit.map(|l| l.to_string());
        let runtime = rt()?;
        runtime.block_on(async move {
            let client = BybitClient::new(BASE_URL);
            let (ts, sig) = sign_get(
                &api_key,
                &secret,
                &[
                    ("category", Some(category.as_str())),
                    ("limit", limit_str.as_deref()),
                    ("orderId", order_id.as_deref()),
                    ("symbol", symbol.as_deref()),
                ],
            );
            let resp = client
                .get_order_history(
                    category.as_str(),
                    limit,
                    order_id.as_deref(),
                    symbol.as_deref(),
                    api_key.as_str(),
                    RECV_WINDOW,
                    sig.as_str(),
                    ts.as_str(),
                )
                .await
                .map_err(|e| format!("[bybit] get_order_history: {e}"))?
                .into_inner();
            ok(resp)
        })
    }
}

// ============================================================================
// Tool 8: place_limit_order (signed)
// ============================================================================

pub(crate) struct PlaceLimitOrder;

#[derive(Debug, Deserialize, JsonSchema)]
pub(crate) struct PlaceLimitOrderArgs {
    #[serde(default)]
    pub api_key: Option<String>,
    #[serde(default)]
    pub secret_key: Option<String>,
    /// `spot`, `linear`, `inverse`, or `option`.
    pub category: String,
    /// Trading pair like `BTCUSDT`.
    pub symbol: String,
    /// `Buy` or `Sell` (capitalised).
    pub side: String,
    /// Order quantity as a string to preserve precision (e.g. `"0.05"`).
    pub qty: String,
    /// Limit price as a string (e.g. `"65000.5"`).
    pub price: String,
    /// Optional time-in-force: `GTC`, `IOC`, `FOK`, `PostOnly`. Default `GTC`.
    #[serde(default)]
    pub time_in_force: Option<String>,
    /// Optional flag — for derivatives, set true to ensure the order can only reduce, not flip, your position.
    #[serde(default)]
    pub reduce_only: Option<bool>,
}

impl DynAomiTool for PlaceLimitOrder {
    type App = BybitApp;
    type Args = PlaceLimitOrderArgs;
    const NAME: &'static str = "bybit_place_limit_order";
    const DESCRIPTION: &'static str = "Place a Limit Buy/Sell at an explicit price. Returns the orderId. Use this when the user names a price; for instant fills use bybit_place_market_order instead. For derivatives, set reduce_only=true to close-only.";

    fn run(_app: &BybitApp, args: Self::Args, _ctx: DynToolCallCtx) -> Result<Value, String> {
        let (api_key, secret) = resolve_creds(args.api_key.as_deref(), args.secret_key.as_deref())?;
        let body = CreateOrderRequest {
            category: args.category,
            order_link_id: None,
            order_type: "Limit".to_string(),
            price: Some(args.price),
            qty: args.qty,
            reduce_only: args.reduce_only,
            side: args.side,
            symbol: args.symbol,
            time_in_force: args.time_in_force.or_else(|| Some("GTC".to_string())),
        };
        let (ts, sig) = sign_post(&api_key, &secret, &body)?;
        let runtime = rt()?;
        runtime.block_on(async move {
            let client = BybitClient::new(BASE_URL);
            let resp = client
                .create_order(
                    api_key.as_str(),
                    RECV_WINDOW,
                    sig.as_str(),
                    ts.as_str(),
                    &body,
                )
                .await
                .map_err(|e| format!("[bybit] create_order (limit): {e}"))?
                .into_inner();
            ok(resp)
        })
    }
}

// ============================================================================
// Tool 9: place_market_order (signed)
// ============================================================================

pub(crate) struct PlaceMarketOrder;

#[derive(Debug, Deserialize, JsonSchema)]
pub(crate) struct PlaceMarketOrderArgs {
    #[serde(default)]
    pub api_key: Option<String>,
    #[serde(default)]
    pub secret_key: Option<String>,
    /// `spot`, `linear`, `inverse`, or `option`.
    pub category: String,
    pub symbol: String,
    /// `Buy` or `Sell` (capitalised).
    pub side: String,
    /// Order quantity as a string. Note: for spot Market Buy, Bybit treats `qty` as quote-asset amount (USDT).
    pub qty: String,
    /// Optional flag — for derivatives, set true to ensure the order can only reduce, not flip, your position.
    #[serde(default)]
    pub reduce_only: Option<bool>,
}

impl DynAomiTool for PlaceMarketOrder {
    type App = BybitApp;
    type Args = PlaceMarketOrderArgs;
    const NAME: &'static str = "bybit_place_market_order";
    const DESCRIPTION: &'static str = "Place a Market Buy/Sell that fills immediately at the best available price. Returns the orderId. Use only when the user explicitly wants instant execution; otherwise prefer bybit_place_limit_order so the user controls the price.";

    fn run(_app: &BybitApp, args: Self::Args, _ctx: DynToolCallCtx) -> Result<Value, String> {
        let (api_key, secret) = resolve_creds(args.api_key.as_deref(), args.secret_key.as_deref())?;
        let body = CreateOrderRequest {
            category: args.category,
            order_link_id: None,
            order_type: "Market".to_string(),
            price: None,
            qty: args.qty,
            reduce_only: args.reduce_only,
            side: args.side,
            symbol: args.symbol,
            time_in_force: None,
        };
        let (ts, sig) = sign_post(&api_key, &secret, &body)?;
        let runtime = rt()?;
        runtime.block_on(async move {
            let client = BybitClient::new(BASE_URL);
            let resp = client
                .create_order(
                    api_key.as_str(),
                    RECV_WINDOW,
                    sig.as_str(),
                    ts.as_str(),
                    &body,
                )
                .await
                .map_err(|e| format!("[bybit] create_order (market): {e}"))?
                .into_inner();
            ok(resp)
        })
    }
}

// ============================================================================
// Tool 10: cancel_order (signed)
// ============================================================================

pub(crate) struct CancelOrder;

#[derive(Debug, Deserialize, JsonSchema)]
pub(crate) struct CancelOrderArgs {
    #[serde(default)]
    pub api_key: Option<String>,
    #[serde(default)]
    pub secret_key: Option<String>,
    pub category: String,
    pub symbol: String,
    /// The orderId returned when the order was placed.
    pub order_id: String,
}

impl DynAomiTool for CancelOrder {
    type App = BybitApp;
    type Args = CancelOrderArgs;
    const NAME: &'static str = "bybit_cancel_order";
    const DESCRIPTION: &'static str = "Cancel a single open order by orderId. Idempotent on the server (re-cancelling an already-closed order returns an error). Use after place-order if the user changes their mind, or after get_open_orders to clean up.";

    fn run(_app: &BybitApp, args: Self::Args, _ctx: DynToolCallCtx) -> Result<Value, String> {
        let (api_key, secret) = resolve_creds(args.api_key.as_deref(), args.secret_key.as_deref())?;
        let body = CancelOrderRequest {
            category: args.category,
            order_id: args.order_id,
            order_link_id: None,
            symbol: args.symbol,
        };
        let (ts, sig) = sign_post(&api_key, &secret, &body)?;
        let runtime = rt()?;
        runtime.block_on(async move {
            let client = BybitClient::new(BASE_URL);
            let resp = client
                .cancel_order(
                    api_key.as_str(),
                    RECV_WINDOW,
                    sig.as_str(),
                    ts.as_str(),
                    &body,
                )
                .await
                .map_err(|e| format!("[bybit] cancel_order: {e}"))?
                .into_inner();
            ok(resp)
        })
    }
}

// ============================================================================
// Tool 11: amend_order (signed)
// ============================================================================

pub(crate) struct AmendOrder;

#[derive(Debug, Deserialize, JsonSchema)]
pub(crate) struct AmendOrderArgs {
    #[serde(default)]
    pub api_key: Option<String>,
    #[serde(default)]
    pub secret_key: Option<String>,
    pub category: String,
    pub symbol: String,
    pub order_id: String,
    /// New quantity. Optional — provide qty and/or price.
    #[serde(default)]
    pub qty: Option<String>,
    /// New price. Optional — provide qty and/or price.
    #[serde(default)]
    pub price: Option<String>,
}

impl DynAomiTool for AmendOrder {
    type App = BybitApp;
    type Args = AmendOrderArgs;
    const NAME: &'static str = "bybit_amend_order";
    const DESCRIPTION: &'static str = "Modify the quantity and/or price of an open order without cancelling. Provide at least one of qty/price. Cheaper than cancel+replace because it preserves queue position when the price doesn't change.";

    fn run(_app: &BybitApp, args: Self::Args, _ctx: DynToolCallCtx) -> Result<Value, String> {
        if args.qty.is_none() && args.price.is_none() {
            return Err("[bybit] amend_order: provide at least one of qty or price".to_string());
        }
        let (api_key, secret) = resolve_creds(args.api_key.as_deref(), args.secret_key.as_deref())?;
        let body = AmendOrderRequest {
            category: args.category,
            order_id: args.order_id,
            price: args.price,
            qty: args.qty,
            symbol: args.symbol,
        };
        let (ts, sig) = sign_post(&api_key, &secret, &body)?;
        let runtime = rt()?;
        runtime.block_on(async move {
            let client = BybitClient::new(BASE_URL);
            let resp = client
                .amend_order(
                    api_key.as_str(),
                    RECV_WINDOW,
                    sig.as_str(),
                    ts.as_str(),
                    &body,
                )
                .await
                .map_err(|e| format!("[bybit] amend_order: {e}"))?
                .into_inner();
            ok(resp)
        })
    }
}

// ============================================================================
// Tool 12: set_leverage (signed)
// ============================================================================

pub(crate) struct SetLeverage;

#[derive(Debug, Deserialize, JsonSchema)]
pub(crate) struct SetLeverageArgs {
    #[serde(default)]
    pub api_key: Option<String>,
    #[serde(default)]
    pub secret_key: Option<String>,
    /// `linear` or `inverse` — perps only.
    pub category: String,
    pub symbol: String,
    /// Leverage on the buy side (e.g. `"10"` for 10x).
    pub buy_leverage: String,
    /// Leverage on the sell side (e.g. `"10"` for 10x).
    pub sell_leverage: String,
}

impl DynAomiTool for SetLeverage {
    type App = BybitApp;
    type Args = SetLeverageArgs;
    const NAME: &'static str = "bybit_set_leverage";
    const DESCRIPTION: &'static str = "Set per-symbol leverage (separate buy and sell legs). Call this BEFORE opening a perp position when the user wants leverage other than the account default. category must be linear or inverse — has no meaning for spot.";

    fn run(_app: &BybitApp, args: Self::Args, _ctx: DynToolCallCtx) -> Result<Value, String> {
        let (api_key, secret) = resolve_creds(args.api_key.as_deref(), args.secret_key.as_deref())?;
        let body = SetLeverageRequest {
            buy_leverage: args.buy_leverage,
            category: args.category,
            sell_leverage: args.sell_leverage,
            symbol: args.symbol,
        };
        let (ts, sig) = sign_post(&api_key, &secret, &body)?;
        let runtime = rt()?;
        runtime.block_on(async move {
            let client = BybitClient::new(BASE_URL);
            let resp = client
                .set_leverage(
                    api_key.as_str(),
                    RECV_WINDOW,
                    sig.as_str(),
                    ts.as_str(),
                    &body,
                )
                .await
                .map_err(|e| format!("[bybit] set_leverage: {e}"))?
                .into_inner();
            ok(resp)
        })
    }
}
