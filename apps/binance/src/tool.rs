use aomi_ext::binance::{
    Binance24hrStatsResponse, BinanceAccountResponse, BinanceClient, BinanceDepthResponse,
    BinanceKlineResponse, BinanceOrderResponse, BinancePriceResponse, BinanceTradeList,
    SPOT_BASE_URL,
};
use aomi_sdk::*;
use aomi_sdk::schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Clone, Default)]
pub(crate) struct BinanceApp;

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
        other => serde_json::json!({ "source": "binance", "data": other }),
    })
}

fn resolve_binance_credentials(
    api_key: Option<&str>,
    secret_key: Option<&str>,
) -> Result<(String, String), String> {
    let api_key = resolve_secret_value(
        api_key,
        "BINANCE_API_KEY",
        "[binance] missing api_key argument and BINANCE_API_KEY environment variable",
    )?;
    let secret_key = resolve_secret_value(
        secret_key,
        "BINANCE_SECRET_KEY",
        "[binance] missing secret_key argument and BINANCE_SECRET_KEY environment variable",
    )?;
    Ok((api_key, secret_key))
}

// ============================================================================
// Tool 1: GetPrice — GET /ticker/price (public)
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
    const DESCRIPTION: &'static str =
        "Use when the user asks the latest spot price of a pair. Returns the current price for one symbol (e.g. BTCUSDT), or every pair when symbol is omitted.";

    fn run(_app: &BinanceApp, args: Self::Args, _ctx: DynToolCallCtx) -> Result<Value, String> {
        let client = BinanceClient::new()?;
        let query = match &args.symbol {
            Some(s) => format!("symbol={s}"),
            None => String::new(),
        };
        ok(client.public_get::<BinancePriceResponse>(SPOT_BASE_URL, "/ticker/price", &query)?)
    }
}

// ============================================================================
// Tool 2: GetDepth — GET /depth (public)
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
        let client = BinanceClient::new()?;
        let mut query = format!("symbol={}", args.symbol);
        if let Some(limit) = args.limit {
            query.push_str(&format!("&limit={limit}"));
        }
        ok(client.public_get::<BinanceDepthResponse>(SPOT_BASE_URL, "/depth", &query)?)
    }
}

// ============================================================================
// Tool 3: GetKlines — GET /klines (public)
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
        let client = BinanceClient::new()?;
        let mut query = format!("symbol={}&interval={}", args.symbol, args.interval);
        if let Some(start) = args.start_time {
            query.push_str(&format!("&startTime={start}"));
        }
        if let Some(end) = args.end_time {
            query.push_str(&format!("&endTime={end}"));
        }
        if let Some(limit) = args.limit {
            query.push_str(&format!("&limit={limit}"));
        }
        ok(client.public_get::<BinanceKlineResponse>(SPOT_BASE_URL, "/klines", &query)?)
    }
}

// ============================================================================
// Tool 4: Get24hrStats — GET /ticker/24hr (public)
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
        let client = BinanceClient::new()?;
        let query = match &args.symbol {
            Some(s) => format!("symbol={s}"),
            None => String::new(),
        };
        ok(client.public_get::<Binance24hrStatsResponse>(SPOT_BASE_URL, "/ticker/24hr", &query)?)
    }
}

// ============================================================================
// Tool 5: PlaceOrder — POST /order (signed)
// ============================================================================

pub(crate) struct PlaceOrder;

#[derive(Debug, Deserialize, JsonSchema)]
pub(crate) struct PlaceOrderArgs {
    /// Binance API key
    pub(crate) api_key: Option<String>,
    /// Binance secret key for request signing
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
        let client = BinanceClient::new()?;
        let (api_key, secret_key) =
            resolve_binance_credentials(args.api_key.as_deref(), args.secret_key.as_deref())?;
        let mut query = format!(
            "symbol={}&side={}&type={}",
            args.symbol, args.side, args.order_type
        );
        if let Some(ref tif) = args.time_in_force {
            query.push_str(&format!("&timeInForce={tif}"));
        }
        if let Some(ref qty) = args.quantity {
            query.push_str(&format!("&quantity={qty}"));
        }
        if let Some(ref price) = args.price {
            query.push_str(&format!("&price={price}"));
        }
        ok(client.signed_post::<BinanceOrderResponse>(
            SPOT_BASE_URL,
            "/order",
            &api_key,
            &secret_key,
            &query,
        )?)
    }
}

// ============================================================================
// Tool 6: CancelOrder — DELETE /order (signed)
// ============================================================================

pub(crate) struct CancelOrder;

#[derive(Debug, Deserialize, JsonSchema)]
pub(crate) struct CancelOrderArgs {
    /// Binance API key
    pub(crate) api_key: Option<String>,
    /// Binance secret key for request signing
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
        let client = BinanceClient::new()?;
        let (api_key, secret_key) =
            resolve_binance_credentials(args.api_key.as_deref(), args.secret_key.as_deref())?;
        let mut query = format!("symbol={}", args.symbol);
        if let Some(oid) = args.order_id {
            query.push_str(&format!("&orderId={oid}"));
        }
        if let Some(ref cid) = args.orig_client_order_id {
            query.push_str(&format!("&origClientOrderId={cid}"));
        }
        ok(client.signed_delete::<BinanceOrderResponse>(
            SPOT_BASE_URL,
            "/order",
            &api_key,
            &secret_key,
            &query,
        )?)
    }
}

// ============================================================================
// Tool 7: GetAccount — GET /account (signed)
// ============================================================================

pub(crate) struct GetAccount;

#[derive(Debug, Deserialize, JsonSchema)]
pub(crate) struct GetAccountArgs {
    /// Binance API key
    pub(crate) api_key: Option<String>,
    /// Binance secret key for request signing
    pub(crate) secret_key: Option<String>,
}

impl DynAomiTool for GetAccount {
    type App = BinanceApp;
    type Args = GetAccountArgs;
    const NAME: &'static str = "binance_get_account";
    const DESCRIPTION: &'static str = "Use when the user asks about their Binance balances or account state. Returns free/locked balances for every asset and account-level permissions. Reads BINANCE_API_KEY/BINANCE_SECRET_KEY from env if not passed.";

    fn run(_app: &BinanceApp, args: Self::Args, _ctx: DynToolCallCtx) -> Result<Value, String> {
        let client = BinanceClient::new()?;
        let (api_key, secret_key) =
            resolve_binance_credentials(args.api_key.as_deref(), args.secret_key.as_deref())?;
        ok(client.signed_get::<BinanceAccountResponse>(
            SPOT_BASE_URL,
            "/account",
            &api_key,
            &secret_key,
            "",
        )?)
    }
}

// ============================================================================
// Tool 8: GetTrades — GET /myTrades (signed)
// ============================================================================

pub(crate) struct GetTrades;

#[derive(Debug, Deserialize, JsonSchema)]
pub(crate) struct GetTradesArgs {
    /// Binance API key
    pub(crate) api_key: Option<String>,
    /// Binance secret key for request signing
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
    const DESCRIPTION: &'static str =
        "Use when the user asks for their personal fill history on a pair (price, qty, fee, timestamp). Pair-scoped — must specify symbol. Default 500 trades, max 1000. Reads credentials from BINANCE_API_KEY/BINANCE_SECRET_KEY if not passed.";

    fn run(_app: &BinanceApp, args: Self::Args, _ctx: DynToolCallCtx) -> Result<Value, String> {
        let client = BinanceClient::new()?;
        let (api_key, secret_key) =
            resolve_binance_credentials(args.api_key.as_deref(), args.secret_key.as_deref())?;
        let mut query = format!("symbol={}", args.symbol);
        if let Some(from_id) = args.from_id {
            query.push_str(&format!("&fromId={from_id}"));
        }
        if let Some(start) = args.start_time {
            query.push_str(&format!("&startTime={start}"));
        }
        if let Some(end) = args.end_time {
            query.push_str(&format!("&endTime={end}"));
        }
        if let Some(limit) = args.limit {
            query.push_str(&format!("&limit={limit}"));
        }
        ok(client.signed_get::<BinanceTradeList>(
            SPOT_BASE_URL,
            "/myTrades",
            &api_key,
            &secret_key,
            &query,
        )?)
    }
}
