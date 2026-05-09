use aomi_ext::bybit::{
    AmendOrderRequest, BybitActionResult, BybitClient, BybitKlineResult, BybitOrderbookResult,
    BybitPositionListResult, BybitTickerResult, BybitWalletBalanceResult, CancelOrderRequest,
    CreateOrderRequest, SetLeverageRequest,
};
use aomi_sdk::*;
use aomi_sdk::schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Clone, Default)]
pub(crate) struct BybitApp;

fn ok<T: Serialize>(value: T) -> Result<Value, String> {
    let value = serde_json::to_value(value)
        .map_err(|e| format!("[bybit] failed to serialize response: {e}"))?;
    Ok(match value {
        Value::Object(mut map) => {
            map.insert("source".to_string(), Value::String("bybit".to_string()));
            Value::Object(map)
        }
        other => serde_json::json!({ "source": "bybit", "data": other }),
    })
}

fn resolve_bybit_credentials(
    api_key: Option<&str>,
    secret_key: Option<&str>,
) -> Result<(String, String), String> {
    let api_key = resolve_secret_value(
        api_key,
        "BYBIT_API_KEY",
        "[bybit] missing api_key argument and BYBIT_API_KEY environment variable",
    )?;
    let secret_key = resolve_secret_value(
        secret_key,
        "BYBIT_SECRET_KEY",
        "[bybit] missing secret_key argument and BYBIT_SECRET_KEY environment variable",
    )?;
    Ok((api_key, secret_key))
}

// ============================================================================
// Arg structs
// ============================================================================

#[derive(Debug, Deserialize, JsonSchema)]
pub(crate) struct GetTickersArgs {
    /// Product category: "spot", "linear", "inverse", or "option"
    pub(crate) category: String,
    /// Trading pair symbol (e.g. "BTCUSDT"). Optional — omit to get all tickers for the category.
    pub(crate) symbol: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub(crate) struct GetOrderbookArgs {
    /// Product category: "spot", "linear", "inverse", or "option"
    pub(crate) category: String,
    /// Trading pair symbol (e.g. "BTCUSDT")
    pub(crate) symbol: String,
    /// Depth limit (e.g. 1, 25, 50, 200). Optional.
    pub(crate) limit: Option<u32>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub(crate) struct GetKlineArgs {
    /// Product category: "spot", "linear", "inverse", or "option"
    pub(crate) category: String,
    /// Trading pair symbol (e.g. "BTCUSDT")
    pub(crate) symbol: String,
    /// Kline interval: 1,3,5,15,30,60,120,240,360,720,D,M,W
    pub(crate) interval: String,
    /// Start timestamp in milliseconds. Optional.
    pub(crate) start: Option<u64>,
    /// End timestamp in milliseconds. Optional.
    pub(crate) end: Option<u64>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub(crate) struct CreateOrderArgs {
    /// Bybit API key
    pub(crate) api_key: Option<String>,
    /// Bybit API secret
    pub(crate) secret_key: Option<String>,
    /// Product category: "spot", "linear", "inverse", or "option"
    pub(crate) category: String,
    /// Trading pair symbol (e.g. "BTCUSDT")
    pub(crate) symbol: String,
    /// Order side: "Buy" or "Sell"
    pub(crate) side: String,
    /// Order type: "Limit" or "Market"
    pub(crate) order_type: String,
    /// Order quantity (string)
    pub(crate) qty: String,
    /// Order price (string). Required for Limit orders.
    pub(crate) price: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub(crate) struct CancelOrderArgs {
    /// Bybit API key
    pub(crate) api_key: Option<String>,
    /// Bybit API secret
    pub(crate) secret_key: Option<String>,
    /// Product category: "spot", "linear", "inverse", or "option"
    pub(crate) category: String,
    /// Trading pair symbol (e.g. "BTCUSDT")
    pub(crate) symbol: String,
    /// Order ID to cancel
    pub(crate) order_id: String,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub(crate) struct AmendOrderArgs {
    /// Bybit API key
    pub(crate) api_key: Option<String>,
    /// Bybit API secret
    pub(crate) secret_key: Option<String>,
    /// Product category: "spot", "linear", "inverse", or "option"
    pub(crate) category: String,
    /// Trading pair symbol (e.g. "BTCUSDT")
    pub(crate) symbol: String,
    /// Order ID to amend
    pub(crate) order_id: String,
    /// New quantity (string). Optional.
    pub(crate) qty: Option<String>,
    /// New price (string). Optional.
    pub(crate) price: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub(crate) struct GetPositionsArgs {
    /// Bybit API key
    pub(crate) api_key: Option<String>,
    /// Bybit API secret
    pub(crate) secret_key: Option<String>,
    /// Product category: "linear" or "inverse"
    pub(crate) category: String,
    /// Trading pair symbol (e.g. "BTCUSDT"). Optional.
    pub(crate) symbol: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub(crate) struct GetWalletBalanceArgs {
    /// Bybit API key
    pub(crate) api_key: Option<String>,
    /// Bybit API secret
    pub(crate) secret_key: Option<String>,
    /// Account type: "UNIFIED" or "CONTRACT"
    pub(crate) account_type: String,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub(crate) struct SetLeverageArgs {
    /// Bybit API key
    pub(crate) api_key: Option<String>,
    /// Bybit API secret
    pub(crate) secret_key: Option<String>,
    /// Product category: "linear" or "inverse"
    pub(crate) category: String,
    /// Trading pair symbol (e.g. "BTCUSDT")
    pub(crate) symbol: String,
    /// Buy leverage (string, e.g. "10")
    pub(crate) buy_leverage: String,
    /// Sell leverage (string, e.g. "10")
    pub(crate) sell_leverage: String,
}

// ============================================================================
// Tool structs
// ============================================================================

pub(crate) struct GetTickers;
pub(crate) struct GetOrderbook;
pub(crate) struct GetKline;
pub(crate) struct CreateOrder;
pub(crate) struct CancelOrder;
pub(crate) struct AmendOrder;
pub(crate) struct GetPositions;
pub(crate) struct GetWalletBalance;
pub(crate) struct SetLeverage;

// ============================================================================
// Tool 1: GetTickers (public)
// ============================================================================

impl DynAomiTool for GetTickers {
    type App = BybitApp;
    type Args = GetTickersArgs;
    const NAME: &'static str = "bybit_get_tickers";
    const DESCRIPTION: &'static str =
        "Get price tickers for a given category. Optionally filter by symbol.";

    fn run(_app: &BybitApp, args: Self::Args, _ctx: DynToolCallCtx) -> Result<Value, String> {
        let client = BybitClient::new()?;
        let mut params = vec![format!("category={}", args.category)];
        if let Some(ref sym) = args.symbol {
            params.push(format!("symbol={sym}"));
        }
        let query = params.join("&");
        ok(client.public_get::<BybitTickerResult>("/market/tickers", &query)?)
    }
}

// ============================================================================
// Tool 2: GetOrderbook (public)
// ============================================================================

impl DynAomiTool for GetOrderbook {
    type App = BybitApp;
    type Args = GetOrderbookArgs;
    const NAME: &'static str = "bybit_get_orderbook";
    const DESCRIPTION: &'static str = "Get order book snapshot for a symbol.";

    fn run(_app: &BybitApp, args: Self::Args, _ctx: DynToolCallCtx) -> Result<Value, String> {
        let client = BybitClient::new()?;
        let mut params = vec![
            format!("category={}", args.category),
            format!("symbol={}", args.symbol),
        ];
        if let Some(limit) = args.limit {
            params.push(format!("limit={limit}"));
        }
        let query = params.join("&");
        ok(client.public_get::<BybitOrderbookResult>("/market/orderbook", &query)?)
    }
}

// ============================================================================
// Tool 3: GetKline (public)
// ============================================================================

impl DynAomiTool for GetKline {
    type App = BybitApp;
    type Args = GetKlineArgs;
    const NAME: &'static str = "bybit_get_kline";
    const DESCRIPTION: &'static str =
        "Get candlestick/kline data for a symbol at a given interval.";

    fn run(_app: &BybitApp, args: Self::Args, _ctx: DynToolCallCtx) -> Result<Value, String> {
        let client = BybitClient::new()?;
        let mut params = vec![
            format!("category={}", args.category),
            format!("symbol={}", args.symbol),
            format!("interval={}", args.interval),
        ];
        if let Some(start) = args.start {
            params.push(format!("start={start}"));
        }
        if let Some(end) = args.end {
            params.push(format!("end={end}"));
        }
        let query = params.join("&");
        ok(client.public_get::<BybitKlineResult>("/market/kline", &query)?)
    }
}

// ============================================================================
// Tool 4: CreateOrder (authenticated POST)
// ============================================================================

impl DynAomiTool for CreateOrder {
    type App = BybitApp;
    type Args = CreateOrderArgs;
    const NAME: &'static str = "bybit_create_order";
    const DESCRIPTION: &'static str =
        "Place a new order. Requires api_key and secret_key for authentication.";

    fn run(_app: &BybitApp, args: Self::Args, _ctx: DynToolCallCtx) -> Result<Value, String> {
        let client = BybitClient::new()?;
        let (api_key, secret_key) =
            resolve_bybit_credentials(args.api_key.as_deref(), args.secret_key.as_deref())?;
        let body = CreateOrderRequest {
            category: &args.category,
            symbol: &args.symbol,
            side: &args.side,
            order_type: &args.order_type,
            qty: &args.qty,
            price: args.price.as_deref(),
        };
        ok(client.auth_post::<_, BybitActionResult>(
            "/order/create",
            &body,
            &api_key,
            &secret_key,
        )?)
    }
}

// ============================================================================
// Tool 5: CancelOrder (authenticated POST)
// ============================================================================

impl DynAomiTool for CancelOrder {
    type App = BybitApp;
    type Args = CancelOrderArgs;
    const NAME: &'static str = "bybit_cancel_order";
    const DESCRIPTION: &'static str =
        "Cancel an existing order. Requires api_key and secret_key for authentication.";

    fn run(_app: &BybitApp, args: Self::Args, _ctx: DynToolCallCtx) -> Result<Value, String> {
        let client = BybitClient::new()?;
        let (api_key, secret_key) =
            resolve_bybit_credentials(args.api_key.as_deref(), args.secret_key.as_deref())?;
        let body = CancelOrderRequest {
            category: &args.category,
            symbol: &args.symbol,
            order_id: &args.order_id,
        };
        ok(client.auth_post::<_, BybitActionResult>(
            "/order/cancel",
            &body,
            &api_key,
            &secret_key,
        )?)
    }
}

// ============================================================================
// Tool 6: AmendOrder (authenticated POST)
// ============================================================================

impl DynAomiTool for AmendOrder {
    type App = BybitApp;
    type Args = AmendOrderArgs;
    const NAME: &'static str = "bybit_amend_order";
    const DESCRIPTION: &'static str = "Modify an existing order (quantity and/or price). Requires api_key and secret_key for authentication.";

    fn run(_app: &BybitApp, args: Self::Args, _ctx: DynToolCallCtx) -> Result<Value, String> {
        let client = BybitClient::new()?;
        let (api_key, secret_key) =
            resolve_bybit_credentials(args.api_key.as_deref(), args.secret_key.as_deref())?;
        let body = AmendOrderRequest {
            category: &args.category,
            symbol: &args.symbol,
            order_id: &args.order_id,
            qty: args.qty.as_deref(),
            price: args.price.as_deref(),
        };
        ok(client.auth_post::<_, BybitActionResult>(
            "/order/amend",
            &body,
            &api_key,
            &secret_key,
        )?)
    }
}

// ============================================================================
// Tool 7: GetPositions (authenticated GET)
// ============================================================================

impl DynAomiTool for GetPositions {
    type App = BybitApp;
    type Args = GetPositionsArgs;
    const NAME: &'static str = "bybit_get_positions";
    const DESCRIPTION: &'static str =
        "Get open positions. Requires api_key and secret_key for authentication.";

    fn run(_app: &BybitApp, args: Self::Args, _ctx: DynToolCallCtx) -> Result<Value, String> {
        let client = BybitClient::new()?;
        let (api_key, secret_key) =
            resolve_bybit_credentials(args.api_key.as_deref(), args.secret_key.as_deref())?;
        let mut params = vec![format!("category={}", args.category)];
        if let Some(ref sym) = args.symbol {
            params.push(format!("symbol={sym}"));
        }
        let query = params.join("&");
        ok(client.auth_get::<BybitPositionListResult>(
            "/position/list",
            &query,
            &api_key,
            &secret_key,
        )?)
    }
}

// ============================================================================
// Tool 8: GetWalletBalance (authenticated GET)
// ============================================================================

impl DynAomiTool for GetWalletBalance {
    type App = BybitApp;
    type Args = GetWalletBalanceArgs;
    const NAME: &'static str = "bybit_get_wallet_balance";
    const DESCRIPTION: &'static str =
        "Get account wallet balance. Requires api_key and secret_key for authentication.";

    fn run(_app: &BybitApp, args: Self::Args, _ctx: DynToolCallCtx) -> Result<Value, String> {
        let client = BybitClient::new()?;
        let (api_key, secret_key) =
            resolve_bybit_credentials(args.api_key.as_deref(), args.secret_key.as_deref())?;
        let query = format!("accountType={}", args.account_type);
        ok(client.auth_get::<BybitWalletBalanceResult>(
            "/account/wallet-balance",
            &query,
            &api_key,
            &secret_key,
        )?)
    }
}

// ============================================================================
// Tool 9: SetLeverage (authenticated POST)
// ============================================================================

impl DynAomiTool for SetLeverage {
    type App = BybitApp;
    type Args = SetLeverageArgs;
    const NAME: &'static str = "bybit_set_leverage";
    const DESCRIPTION: &'static str =
        "Set leverage for a symbol. Requires api_key and secret_key for authentication.";

    fn run(_app: &BybitApp, args: Self::Args, _ctx: DynToolCallCtx) -> Result<Value, String> {
        let client = BybitClient::new()?;
        let (api_key, secret_key) =
            resolve_bybit_credentials(args.api_key.as_deref(), args.secret_key.as_deref())?;
        let body = SetLeverageRequest {
            category: &args.category,
            symbol: &args.symbol,
            buy_leverage: &args.buy_leverage,
            sell_leverage: &args.sell_leverage,
        };
        ok(client.auth_post::<_, BybitActionResult>(
            "/position/set-leverage",
            &body,
            &api_key,
            &secret_key,
        )?)
    }
}
