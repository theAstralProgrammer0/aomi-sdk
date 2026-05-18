//! Curated tool layer for Binance Spot. Hand-written from the
//! progenitor-generated client in `aomi_ext::binance` — see
//! `ext/specs/binance.yaml` for the underlying surface and
//! `ext/src/binance/auth.rs` for the HMAC-SHA256 signing helpers.
//!
//! Designed for the user story: trade BTC/ETH spot on Binance — check
//! prices/depth/klines/24h stats, place and cancel orders, view balances and
//! personal fill history.
//!
//! 8 curated tools (preserved from the prior hand-written client):
//!   * binance_get_price        — last price for one or all spot symbols
//!   * binance_get_depth        — order-book bids/asks
//!   * binance_get_klines       — OHLCV candles
//!   * binance_get_24hr_stats   — rolling 24h price-change stats
//!   * binance_place_order      — place a spot order (signed)
//!   * binance_cancel_order     — cancel an open order (signed)
//!   * binance_get_account      — balances and permissions (signed)
//!   * binance_get_trades       — personal fill history on one symbol (signed)

use aomi_ext::binance::{Client as BinanceClient, build_query, current_timestamp_ms, sign};
use aomi_sdk::schemars::JsonSchema;
use aomi_sdk::*;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

#[derive(Clone, Default)]
pub(crate) struct BinanceApp;

const BASE_URL: &str = "https://api.binance.com";

// ============================================================================
// Helpers
// ============================================================================

fn ok<T: Serialize>(value: T) -> Result<Value, String> {
    let value = serde_json::to_value(value)
        .map_err(|e| format!("[binance] failed to serialize response: {e}"))?;
    Ok(match value {
        Value::Object(mut map) => {
            map.insert("source".to_string(), Value::String("binance".to_string()));
            Value::Object(map)
        }
        other => json!({ "source": "binance", "data": other }),
    })
}

fn rt() -> Result<tokio::runtime::Runtime, String> {
    tokio::runtime::Runtime::new().map_err(|e| format!("[binance] runtime: {e}"))
}

fn resolve_creds(
    api_key: Option<&str>,
    secret_key: Option<&str>,
) -> Result<(String, String), String> {
    let api = resolve_secret_value(
        api_key,
        "BINANCE_API_KEY",
        "[binance] missing api_key argument and BINANCE_API_KEY environment variable",
    )?;
    let sec = resolve_secret_value(
        secret_key,
        "BINANCE_SECRET_KEY",
        "[binance] missing secret_key argument and BINANCE_SECRET_KEY environment variable",
    )?;
    Ok((api, sec))
}

/// Build the canonical query string Binance signs, then compute the HMAC.
/// `pairs` MUST be in the same order the generated client emits them on the
/// wire (excluding `signature`, which is computed here and appended).
fn sign_query(secret: &str, pairs: &[(&str, Option<String>)]) -> Result<(i64, String), String> {
    let timestamp = current_timestamp_ms()?;
    let mut all: Vec<(&str, Option<String>)> = pairs.to_vec();
    all.push(("timestamp", Some(timestamp.to_string())));
    let query = build_query(&all);
    let signature = sign(secret, &query)?;
    Ok((timestamp, signature))
}

// ============================================================================
// Tool 1: GetPrice — GET /api/v3/ticker/price (public)
// ============================================================================

pub(crate) struct GetPrice;

#[derive(Debug, Deserialize, JsonSchema)]
pub(crate) struct GetPriceArgs {
    /// Trading pair symbol (e.g., "BTCUSDT", "ETHUSDT"). If omitted, returns prices for all symbols.
    pub(crate) symbol: Option<String>,
}

impl DynAomiTool for GetPrice {
    type App = BinanceApp;
    type Args = GetPriceArgs;
    const NAME: &'static str = "binance_get_price";
    const DESCRIPTION: &'static str = "Use when the user asks the latest spot price of a pair. Returns the current price for one symbol (e.g. BTCUSDT), or every pair when symbol is omitted.";

    fn run(_app: &BinanceApp, args: Self::Args, _ctx: DynToolCallCtx) -> Result<Value, String> {
        let runtime = rt()?;
        runtime.block_on(async move {
            let client = BinanceClient::new(BASE_URL);
            let resp = client
                .get_ticker_price(args.symbol.as_deref())
                .await
                .map_err(|e| format!("[binance] get_ticker_price: {e}"))?
                .into_inner();
            ok(resp)
        })
    }
}

// ============================================================================
// Tool 2: GetDepth — GET /api/v3/depth (public)
// ============================================================================

pub(crate) struct GetDepth;

#[derive(Debug, Deserialize, JsonSchema)]
pub(crate) struct GetDepthArgs {
    /// Trading pair symbol (e.g., "BTCUSDT")
    pub(crate) symbol: String,
    /// Number of price levels to return (5, 10, 20, 50, 100, 500, 1000, 5000). Default 100.
    pub(crate) limit: Option<u32>,
}

impl DynAomiTool for GetDepth {
    type App = BinanceApp;
    type Args = GetDepthArgs;
    const NAME: &'static str = "binance_get_depth";
    const DESCRIPTION: &'static str = "Use when the user wants order-book depth (top bids/asks and sizes) for a spot pair, e.g. before placing a limit order or to gauge liquidity. Default depth is 100 levels.";

    fn run(_app: &BinanceApp, args: Self::Args, _ctx: DynToolCallCtx) -> Result<Value, String> {
        let runtime = rt()?;
        runtime.block_on(async move {
            let client = BinanceClient::new(BASE_URL);
            let limit = args.limit.map(|l| l as i32);
            let resp = client
                .get_depth(limit, args.symbol.as_str())
                .await
                .map_err(|e| format!("[binance] get_depth: {e}"))?
                .into_inner();
            ok(resp)
        })
    }
}

// ============================================================================
// Tool 3: GetKlines — GET /api/v3/klines (public)
// ============================================================================

pub(crate) struct GetKlines;

#[derive(Debug, Deserialize, JsonSchema)]
pub(crate) struct GetKlinesArgs {
    /// Trading pair symbol (e.g., "BTCUSDT")
    pub(crate) symbol: String,
    /// Kline/candlestick interval (e.g., "1m", "5m", "15m", "1h", "4h", "1d", "1w", "1M")
    pub(crate) interval: String,
    /// Start time in milliseconds (optional)
    pub(crate) start_time: Option<u64>,
    /// End time in milliseconds (optional)
    pub(crate) end_time: Option<u64>,
    /// Number of candles to return (default 500, max 1000)
    pub(crate) limit: Option<u32>,
}

impl DynAomiTool for GetKlines {
    type App = BinanceApp;
    type Args = GetKlinesArgs;
    const NAME: &'static str = "binance_get_klines";
    const DESCRIPTION: &'static str = "Use when the user asks for price history, charts, or technical analysis of a pair. Returns OHLCV candles as arrays [open_time, open, high, low, close, volume, close_time, quote_volume, trades, taker_buy_base_vol, taker_buy_quote_vol, ignore]. Default 500 candles, max 1000.";

    fn run(_app: &BinanceApp, args: Self::Args, _ctx: DynToolCallCtx) -> Result<Value, String> {
        let runtime = rt()?;
        runtime.block_on(async move {
            let client = BinanceClient::new(BASE_URL);
            let limit = args.limit.map(|l| l as i32);
            let start = args.start_time.map(|t| t as i64);
            let end = args.end_time.map(|t| t as i64);
            let resp = client
                .get_klines(
                    end,
                    args.interval.as_str(),
                    limit,
                    start,
                    args.symbol.as_str(),
                )
                .await
                .map_err(|e| format!("[binance] get_klines: {e}"))?
                .into_inner();
            ok(resp)
        })
    }
}

// ============================================================================
// Tool 4: Get24hrStats — GET /api/v3/ticker/24hr (public)
// ============================================================================

pub(crate) struct Get24hrStats;

#[derive(Debug, Deserialize, JsonSchema)]
pub(crate) struct Get24hrStatsArgs {
    /// Trading pair symbol (e.g., "BTCUSDT"). If omitted, returns stats for all symbols.
    pub(crate) symbol: Option<String>,
}

impl DynAomiTool for Get24hrStats {
    type App = BinanceApp;
    type Args = Get24hrStatsArgs;
    const NAME: &'static str = "binance_get_24hr_stats";
    const DESCRIPTION: &'static str = "Use when the user asks how a pair has moved over the past 24 hours (price change %, high/low, volume). Returns rolling 24h stats for one symbol, or all pairs when symbol is omitted.";

    fn run(_app: &BinanceApp, args: Self::Args, _ctx: DynToolCallCtx) -> Result<Value, String> {
        let runtime = rt()?;
        runtime.block_on(async move {
            let client = BinanceClient::new(BASE_URL);
            let resp = client
                .get24hr_stats(args.symbol.as_deref())
                .await
                .map_err(|e| format!("[binance] get24hr_stats: {e}"))?
                .into_inner();
            ok(resp)
        })
    }
}

// ============================================================================
// Tool 5: PlaceOrder — POST /api/v3/order (signed)
// ============================================================================

pub(crate) struct PlaceOrder;

#[derive(Debug, Deserialize, JsonSchema)]
pub(crate) struct PlaceOrderArgs {
    /// Operator-injected Binance API key. Hidden from the LLM tool schema —
    /// resolved from the SDK secrets store or `BINANCE_API_KEY` env var.
    #[schemars(skip)]
    pub(crate) api_key: Option<String>,
    /// Operator-injected Binance secret key for HMAC signing. Hidden from
    /// the LLM tool schema — resolved from `BINANCE_SECRET_KEY` env var.
    #[schemars(skip)]
    pub(crate) secret_key: Option<String>,
    /// Trading pair symbol (e.g., "BTCUSDT")
    pub(crate) symbol: String,
    /// Order side: "BUY" or "SELL"
    pub(crate) side: String,
    /// Order type: "LIMIT", "MARKET", "STOP_LOSS_LIMIT", "TAKE_PROFIT_LIMIT"
    pub(crate) order_type: String,
    /// Time in force: "GTC", "IOC", or "FOK". Required for LIMIT orders.
    pub(crate) time_in_force: Option<String>,
    /// Order quantity
    pub(crate) quantity: Option<String>,
    /// Order price (required for LIMIT orders)
    pub(crate) price: Option<String>,
}

impl DynAomiTool for PlaceOrder {
    type App = BinanceApp;
    type Args = PlaceOrderArgs;
    const NAME: &'static str = "binance_place_order";
    const DESCRIPTION: &'static str = "Use when the user wants to place a spot order. Supports LIMIT (set price + GTC/IOC/FOK), MARKET (omit price/time_in_force), STOP_LOSS_LIMIT, TAKE_PROFIT_LIMIT. Reads BINANCE_API_KEY/BINANCE_SECRET_KEY from env if api_key/secret_key args are omitted.";

    fn run(_app: &BinanceApp, args: Self::Args, _ctx: DynToolCallCtx) -> Result<Value, String> {
        let (api_key, secret_key) =
            resolve_creds(args.api_key.as_deref(), args.secret_key.as_deref())?;
        // Order MUST match the order the generated client emits these on the
        // wire, EXCLUDING `signature` (computed here) and `timestamp` (appended
        // by sign_query). See client.rs::place_order for the canonical order:
        // price, quantity, side, [signature], symbol, timeInForce, timestamp, type.
        let pairs: Vec<(&str, Option<String>)> = vec![
            ("price", args.price.clone()),
            ("quantity", args.quantity.clone()),
            ("side", Some(args.side.clone())),
            ("symbol", Some(args.symbol.clone())),
            ("timeInForce", args.time_in_force.clone()),
            ("type", Some(args.order_type.clone())),
        ];
        let (timestamp, signature) = sign_query(&secret_key, &pairs)?;
        let runtime = rt()?;
        runtime.block_on(async move {
            let client = BinanceClient::new(BASE_URL);
            let resp = client
                .place_order(
                    args.price.as_deref(),
                    args.quantity.as_deref(),
                    args.side.as_str(),
                    signature.as_str(),
                    args.symbol.as_str(),
                    args.time_in_force.as_deref(),
                    timestamp,
                    args.order_type.as_str(),
                    api_key.as_str(),
                )
                .await
                .map_err(|e| format!("[binance] place_order: {e}"))?
                .into_inner();
            ok(resp)
        })
    }
}

// ============================================================================
// Tool 6: CancelOrder — DELETE /api/v3/order (signed)
// ============================================================================

pub(crate) struct CancelOrder;

#[derive(Debug, Deserialize, JsonSchema)]
pub(crate) struct CancelOrderArgs {
    /// Operator-injected Binance API key. Hidden from the LLM tool schema —
    /// resolved from the SDK secrets store or `BINANCE_API_KEY` env var.
    #[schemars(skip)]
    pub(crate) api_key: Option<String>,
    /// Operator-injected Binance secret key for HMAC signing. Hidden from
    /// the LLM tool schema — resolved from `BINANCE_SECRET_KEY` env var.
    #[schemars(skip)]
    pub(crate) secret_key: Option<String>,
    /// Trading pair symbol (e.g., "BTCUSDT")
    pub(crate) symbol: String,
    /// Order ID to cancel
    pub(crate) order_id: Option<u64>,
    /// Original client order ID to cancel (alternative to order_id)
    pub(crate) orig_client_order_id: Option<String>,
}

impl DynAomiTool for CancelOrder {
    type App = BinanceApp;
    type Args = CancelOrderArgs;
    const NAME: &'static str = "binance_cancel_order";
    const DESCRIPTION: &'static str = "Use when the user wants to cancel an open spot order. Provide either order_id (preferred — returned by place_order) or orig_client_order_id. Reads credentials from BINANCE_API_KEY/BINANCE_SECRET_KEY if not passed.";

    fn run(_app: &BinanceApp, args: Self::Args, _ctx: DynToolCallCtx) -> Result<Value, String> {
        let (api_key, secret_key) =
            resolve_creds(args.api_key.as_deref(), args.secret_key.as_deref())?;
        // wire order from cancel_order: orderId, origClientOrderId, [signature], symbol, timestamp.
        let pairs: Vec<(&str, Option<String>)> = vec![
            ("orderId", args.order_id.map(|v| v.to_string())),
            ("origClientOrderId", args.orig_client_order_id.clone()),
            ("symbol", Some(args.symbol.clone())),
        ];
        let (timestamp, signature) = sign_query(&secret_key, &pairs)?;
        let order_id_i64 = args.order_id.map(|v| v as i64);
        let runtime = rt()?;
        runtime.block_on(async move {
            let client = BinanceClient::new(BASE_URL);
            let resp = client
                .cancel_order(
                    order_id_i64,
                    args.orig_client_order_id.as_deref(),
                    signature.as_str(),
                    args.symbol.as_str(),
                    timestamp,
                    api_key.as_str(),
                )
                .await
                .map_err(|e| format!("[binance] cancel_order: {e}"))?
                .into_inner();
            ok(resp)
        })
    }
}

// ============================================================================
// Tool 7: GetAccount — GET /api/v3/account (signed)
// ============================================================================

pub(crate) struct GetAccount;

#[derive(Debug, Deserialize, JsonSchema)]
pub(crate) struct GetAccountArgs {
    /// Operator-injected Binance API key. Hidden from the LLM tool schema —
    /// resolved from the SDK secrets store or `BINANCE_API_KEY` env var.
    #[schemars(skip)]
    pub(crate) api_key: Option<String>,
    /// Operator-injected Binance secret key for HMAC signing. Hidden from
    /// the LLM tool schema — resolved from `BINANCE_SECRET_KEY` env var.
    #[schemars(skip)]
    pub(crate) secret_key: Option<String>,
}

impl DynAomiTool for GetAccount {
    type App = BinanceApp;
    type Args = GetAccountArgs;
    const NAME: &'static str = "binance_get_account";
    const DESCRIPTION: &'static str = "Use when the user asks about their Binance balances or account state. Returns free/locked balances for every asset and account-level permissions. Reads BINANCE_API_KEY/BINANCE_SECRET_KEY from env if not passed.";

    fn run(_app: &BinanceApp, args: Self::Args, _ctx: DynToolCallCtx) -> Result<Value, String> {
        let (api_key, secret_key) =
            resolve_creds(args.api_key.as_deref(), args.secret_key.as_deref())?;
        let pairs: Vec<(&str, Option<String>)> = vec![];
        let (timestamp, signature) = sign_query(&secret_key, &pairs)?;
        let runtime = rt()?;
        runtime.block_on(async move {
            let client = BinanceClient::new(BASE_URL);
            let resp = client
                .get_account(signature.as_str(), timestamp, api_key.as_str())
                .await
                .map_err(|e| format!("[binance] get_account: {e}"))?
                .into_inner();
            ok(resp)
        })
    }
}

// ============================================================================
// Tool 8: GetTrades — GET /api/v3/myTrades (signed)
// ============================================================================

pub(crate) struct GetTrades;

#[derive(Debug, Deserialize, JsonSchema)]
pub(crate) struct GetTradesArgs {
    /// Operator-injected Binance API key. Hidden from the LLM tool schema —
    /// resolved from the SDK secrets store or `BINANCE_API_KEY` env var.
    #[schemars(skip)]
    pub(crate) api_key: Option<String>,
    /// Operator-injected Binance secret key for HMAC signing. Hidden from
    /// the LLM tool schema — resolved from `BINANCE_SECRET_KEY` env var.
    #[schemars(skip)]
    pub(crate) secret_key: Option<String>,
    /// Trading pair symbol (e.g., "BTCUSDT")
    pub(crate) symbol: String,
    /// Trade ID to fetch from (optional)
    pub(crate) from_id: Option<u64>,
    /// Start time in milliseconds (optional)
    pub(crate) start_time: Option<u64>,
    /// End time in milliseconds (optional)
    pub(crate) end_time: Option<u64>,
    /// Number of trades to return (default 500, max 1000)
    pub(crate) limit: Option<u32>,
}

impl DynAomiTool for GetTrades {
    type App = BinanceApp;
    type Args = GetTradesArgs;
    const NAME: &'static str = "binance_get_trades";
    const DESCRIPTION: &'static str = "Use when the user asks for their personal fill history on a pair (price, qty, fee, timestamp). Pair-scoped — must specify symbol. Default 500 trades, max 1000. Reads credentials from BINANCE_API_KEY/BINANCE_SECRET_KEY if not passed.";

    fn run(_app: &BinanceApp, args: Self::Args, _ctx: DynToolCallCtx) -> Result<Value, String> {
        let (api_key, secret_key) =
            resolve_creds(args.api_key.as_deref(), args.secret_key.as_deref())?;
        // wire order from get_my_trades: endTime, fromId, limit, [signature], startTime, symbol, timestamp.
        let pairs: Vec<(&str, Option<String>)> = vec![
            ("endTime", args.end_time.map(|v| v.to_string())),
            ("fromId", args.from_id.map(|v| v.to_string())),
            ("limit", args.limit.map(|v| v.to_string())),
            ("startTime", args.start_time.map(|v| v.to_string())),
            ("symbol", Some(args.symbol.clone())),
        ];
        let (timestamp, signature) = sign_query(&secret_key, &pairs)?;
        let end = args.end_time.map(|v| v as i64);
        let from = args.from_id.map(|v| v as i64);
        let limit = args.limit.map(|v| v as i32);
        let start = args.start_time.map(|v| v as i64);
        let runtime = rt()?;
        runtime.block_on(async move {
            let client = BinanceClient::new(BASE_URL);
            let resp = client
                .get_my_trades(
                    end,
                    from,
                    limit,
                    signature.as_str(),
                    start,
                    args.symbol.as_str(),
                    timestamp,
                    api_key.as_str(),
                )
                .await
                .map_err(|e| format!("[binance] get_my_trades: {e}"))?
                .into_inner();
            ok(resp)
        })
    }
}
