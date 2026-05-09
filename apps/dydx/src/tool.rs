use aomi_ext::dydx::DydxClient;
use aomi_sdk::*;
use aomi_sdk::schemars::JsonSchema;
use serde::Deserialize;
use serde_json::Value;

#[derive(Clone, Default)]
pub(crate) struct DydxApp;

// ============================================================================
// Tool 1: GetMarkets
// ============================================================================

pub(crate) struct GetMarkets;

#[derive(Debug, Deserialize, JsonSchema)]
pub(crate) struct GetMarketsArgs {
    /// Optional ticker to filter by (e.g., "BTC-USD"). If omitted, returns all perpetual markets.
    pub(crate) ticker: Option<String>,
}

impl DynAomiTool for GetMarkets {
    type App = DydxApp;
    type Args = GetMarketsArgs;
    const NAME: &'static str = "dydx_get_markets";
    const DESCRIPTION: &'static str = "Get perpetual market metadata from dYdX v4. Returns tick size, step size, margin requirements, and current oracle price. Optionally filter by ticker.";

    fn run(_app: &DydxApp, args: Self::Args, _ctx: DynToolCallCtx) -> Result<Value, String> {
        let client = DydxClient::new()?;
        let path = match &args.ticker {
            Some(ticker) => format!("/perpetualMarkets?ticker={ticker}"),
            None => "/perpetualMarkets".to_string(),
        };
        client.get(&path)
    }
}

// ============================================================================
// Tool 2: GetOrderbook
// ============================================================================

pub(crate) struct GetOrderbook;

#[derive(Debug, Deserialize, JsonSchema)]
pub(crate) struct GetOrderbookArgs {
    /// Market ticker (e.g., "BTC-USD")
    pub(crate) ticker: String,
}

impl DynAomiTool for GetOrderbook {
    type App = DydxApp;
    type Args = GetOrderbookArgs;
    const NAME: &'static str = "dydx_get_orderbook";
    const DESCRIPTION: &'static str = "Get the L2 orderbook snapshot for a dYdX v4 perpetual market. Returns arrays of bid and ask price levels.";

    fn run(_app: &DydxApp, args: Self::Args, _ctx: DynToolCallCtx) -> Result<Value, String> {
        let client = DydxClient::new()?;
        client.get(&format!("/orderbooks/perpetualMarket/{}", args.ticker))
    }
}

// ============================================================================
// Tool 3: GetCandles
// ============================================================================

pub(crate) struct GetCandles;

#[derive(Debug, Deserialize, JsonSchema)]
pub(crate) struct GetCandlesArgs {
    /// Market ticker (e.g., "ETH-USD")
    pub(crate) ticker: String,
    /// Candle resolution: 1MIN, 5MINS, 15MINS, 30MINS, 1HOUR, 4HOURS, or 1DAY
    pub(crate) resolution: String,
    /// Maximum number of candles to return (optional)
    pub(crate) limit: Option<u32>,
}

impl DynAomiTool for GetCandles {
    type App = DydxApp;
    type Args = GetCandlesArgs;
    const NAME: &'static str = "dydx_get_candles";
    const DESCRIPTION: &'static str = "Get OHLCV candle data for a dYdX v4 perpetual market. Supports resolutions: 1MIN, 5MINS, 15MINS, 30MINS, 1HOUR, 4HOURS, 1DAY.";

    fn run(_app: &DydxApp, args: Self::Args, _ctx: DynToolCallCtx) -> Result<Value, String> {
        let client = DydxClient::new()?;
        let mut path = format!(
            "/candles/perpetualMarket/{}?resolution={}",
            args.ticker, args.resolution
        );
        if let Some(limit) = args.limit {
            path.push_str(&format!("&limit={limit}"));
        }
        client.get(&path)
    }
}

// ============================================================================
// Tool 4: GetTrades
// ============================================================================

pub(crate) struct GetTrades;

#[derive(Debug, Deserialize, JsonSchema)]
pub(crate) struct GetTradesArgs {
    /// Market ticker (e.g., "BTC-USD")
    pub(crate) ticker: String,
    /// Maximum number of trades to return (optional)
    pub(crate) limit: Option<u32>,
}

impl DynAomiTool for GetTrades {
    type App = DydxApp;
    type Args = GetTradesArgs;
    const NAME: &'static str = "dydx_get_trades";
    const DESCRIPTION: &'static str = "Get recent trades for a dYdX v4 perpetual market. Returns trade price, size, side, and timestamp.";

    fn run(_app: &DydxApp, args: Self::Args, _ctx: DynToolCallCtx) -> Result<Value, String> {
        let client = DydxClient::new()?;
        let mut path = format!("/trades/perpetualMarket/{}", args.ticker);
        if let Some(limit) = args.limit {
            path.push_str(&format!("?limit={limit}"));
        }
        client.get(&path)
    }
}

// ============================================================================
// Tool 5: GetAccount
// ============================================================================

pub(crate) struct GetAccount;

#[derive(Debug, Deserialize, JsonSchema)]
pub(crate) struct GetAccountArgs {
    /// dYdX address (e.g., "dydx1...")
    pub(crate) address: String,
    /// Subaccount number (typically 0 for default)
    pub(crate) subaccount_number: u32,
}

impl DynAomiTool for GetAccount {
    type App = DydxApp;
    type Args = GetAccountArgs;
    const NAME: &'static str = "dydx_get_account";
    const DESCRIPTION: &'static str = "Get account state for a dYdX v4 subaccount. Returns equity, free collateral, open positions, and margin usage.";

    fn run(_app: &DydxApp, args: Self::Args, _ctx: DynToolCallCtx) -> Result<Value, String> {
        let client = DydxClient::new()?;
        client.get(&format!(
            "/addresses/{}/subaccountNumber/{}",
            args.address, args.subaccount_number
        ))
    }
}

// ============================================================================
// Tool 6: GetOrders
// ============================================================================

pub(crate) struct GetOrders;

#[derive(Debug, Deserialize, JsonSchema)]
pub(crate) struct GetOrdersArgs {
    /// dYdX address (e.g., "dydx1...")
    pub(crate) address: String,
    /// Subaccount number (typically 0 for default)
    pub(crate) subaccount_number: u32,
    /// Optional order status filter (e.g., "OPEN", "FILLED", "CANCELED")
    pub(crate) status: Option<String>,
    /// Optional ticker filter (e.g., "BTC-USD")
    pub(crate) ticker: Option<String>,
}

impl DynAomiTool for GetOrders {
    type App = DydxApp;
    type Args = GetOrdersArgs;
    const NAME: &'static str = "dydx_get_orders";
    const DESCRIPTION: &'static str = "Get open or historical orders for a dYdX v4 subaccount. Optionally filter by status or ticker.";

    fn run(_app: &DydxApp, args: Self::Args, _ctx: DynToolCallCtx) -> Result<Value, String> {
        let client = DydxClient::new()?;
        let mut path = format!(
            "/orders?address={}&subaccountNumber={}",
            args.address, args.subaccount_number
        );
        if let Some(status) = &args.status {
            path.push_str(&format!("&status={status}"));
        }
        if let Some(ticker) = &args.ticker {
            path.push_str(&format!("&ticker={ticker}"));
        }
        client.get(&path)
    }
}

// ============================================================================
// Tool 7: GetFills
// ============================================================================

pub(crate) struct GetFills;

#[derive(Debug, Deserialize, JsonSchema)]
pub(crate) struct GetFillsArgs {
    /// dYdX address (e.g., "dydx1...")
    pub(crate) address: String,
    /// Subaccount number (typically 0 for default)
    pub(crate) subaccount_number: u32,
    /// Optional market ticker filter (e.g., "BTC-USD")
    pub(crate) market: Option<String>,
    /// Maximum number of fills to return (optional)
    pub(crate) limit: Option<u32>,
}

impl DynAomiTool for GetFills {
    type App = DydxApp;
    type Args = GetFillsArgs;
    const NAME: &'static str = "dydx_get_fills";
    const DESCRIPTION: &'static str =
        "Get fill history for a dYdX v4 subaccount. Optionally filter by market and limit results.";

    fn run(_app: &DydxApp, args: Self::Args, _ctx: DynToolCallCtx) -> Result<Value, String> {
        let client = DydxClient::new()?;
        let mut path = format!(
            "/fills?address={}&subaccountNumber={}",
            args.address, args.subaccount_number
        );
        if let Some(market) = &args.market {
            path.push_str(&format!("&market={market}"));
        }
        if let Some(limit) = args.limit {
            path.push_str(&format!("&limit={limit}"));
        }
        client.get(&path)
    }
}

// ============================================================================
// Tool 8: GetHistoricalFunding
// ============================================================================

pub(crate) struct GetHistoricalFunding;

#[derive(Debug, Deserialize, JsonSchema)]
pub(crate) struct GetHistoricalFundingArgs {
    /// Market ticker (e.g., "BTC-USD")
    pub(crate) ticker: String,
    /// Maximum number of funding rate entries to return (optional)
    pub(crate) limit: Option<u32>,
}

impl DynAomiTool for GetHistoricalFunding {
    type App = DydxApp;
    type Args = GetHistoricalFundingArgs;
    const NAME: &'static str = "dydx_get_historical_funding";
    const DESCRIPTION: &'static str = "Get historical funding rates for a dYdX v4 perpetual market. Returns funding rate, price, and timestamp per interval.";

    fn run(_app: &DydxApp, args: Self::Args, _ctx: DynToolCallCtx) -> Result<Value, String> {
        let client = DydxClient::new()?;
        let mut path = format!("/historical-funding/{}", args.ticker);
        if let Some(limit) = args.limit {
            path.push_str(&format!("?limit={limit}"));
        }
        client.get(&path)
    }
}
