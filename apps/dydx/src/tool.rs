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
    /// Optional ticker filter (e.g., "BTC-USD"). Omit to list every perpetual market.
    pub(crate) ticker: Option<String>,
}

impl DynAomiTool for GetMarkets {
    type App = DydxApp;
    type Args = GetMarketsArgs;
    const NAME: &'static str = "dydx_get_markets";
    const DESCRIPTION: &'static str = "Use when the user asks what's tradable on dYdX, market parameters, or oracle price. Returns per-market tick size, step size, initial/maintenance margin fractions, current oracle price, 24h volume, open interest, and next funding rate. Pass a ticker to scope to one market.";

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
    const DESCRIPTION: &'static str = "Use when the user asks about depth, spread, or current best bid/ask on a dYdX market. Returns the L2 orderbook snapshot as bid and ask arrays of {price, size} levels.";

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
    /// Candle resolution: "1MIN", "5MINS", "15MINS", "30MINS", "1HOUR", "4HOURS", or "1DAY"
    pub(crate) resolution: String,
    /// Maximum number of candles to return (default 100, max 100).
    pub(crate) limit: Option<u32>,
}

impl DynAomiTool for GetCandles {
    type App = DydxApp;
    type Args = GetCandlesArgs;
    const NAME: &'static str = "dydx_get_candles";
    const DESCRIPTION: &'static str = "Use when the user asks for price history, charts, or technical analysis on a dYdX market. Returns OHLCV candles (open/high/low/close as decimal strings, plus baseTokenVolume and usdVolume). Resolutions: 1MIN, 5MINS, 15MINS, 30MINS, 1HOUR, 4HOURS, 1DAY.";

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
    /// Maximum number of trades to return (default 100).
    pub(crate) limit: Option<u32>,
}

impl DynAomiTool for GetTrades {
    type App = DydxApp;
    type Args = GetTradesArgs;
    const NAME: &'static str = "dydx_get_trades";
    const DESCRIPTION: &'static str = "Use when the user asks for the latest tape, recent fills, or volume on a dYdX market (this is the public market tape, not the user's own fills — use dydx_get_fills for that). Returns recent trades with price, size, side, and timestamp.";

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
    /// dYdX bech32 address (e.g., "dydx1...")
    pub(crate) address: String,
    /// Subaccount number. Defaults to 0 (the default subaccount most users have).
    pub(crate) subaccount_number: Option<u32>,
}

impl DynAomiTool for GetAccount {
    type App = DydxApp;
    type Args = GetAccountArgs;
    const NAME: &'static str = "dydx_get_account";
    const DESCRIPTION: &'static str = "Use when the user asks about their dYdX account, equity, free collateral, margin usage, or open perp positions. Returns the subaccount snapshot. If subaccount_number is omitted it defaults to 0.";

    fn run(_app: &DydxApp, args: Self::Args, _ctx: DynToolCallCtx) -> Result<Value, String> {
        let client = DydxClient::new()?;
        let sub = args.subaccount_number.unwrap_or(0);
        client.get(&format!(
            "/addresses/{}/subaccountNumber/{}",
            args.address, sub
        ))
    }
}

// ============================================================================
// Tool 6: GetOrders
// ============================================================================

pub(crate) struct GetOrders;

#[derive(Debug, Deserialize, JsonSchema)]
pub(crate) struct GetOrdersArgs {
    /// dYdX bech32 address (e.g., "dydx1...")
    pub(crate) address: String,
    /// Subaccount number. Defaults to 0.
    pub(crate) subaccount_number: Option<u32>,
    /// Order status filter: "OPEN", "FILLED", "CANCELED", "BEST_EFFORT_CANCELED", or "UNTRIGGERED". Omit for all.
    pub(crate) status: Option<String>,
    /// Market ticker filter (e.g., "BTC-USD"). Omit for all markets.
    pub(crate) ticker: Option<String>,
}

impl DynAomiTool for GetOrders {
    type App = DydxApp;
    type Args = GetOrdersArgs;
    const NAME: &'static str = "dydx_get_orders";
    const DESCRIPTION: &'static str = "Use when the user wants to see their resting limit orders, conditional orders, or order history on dYdX. Returns orders for the subaccount with side, size, price, status, and timestamps. Filter by status or ticker to narrow.";

    fn run(_app: &DydxApp, args: Self::Args, _ctx: DynToolCallCtx) -> Result<Value, String> {
        let client = DydxClient::new()?;
        let sub = args.subaccount_number.unwrap_or(0);
        let mut path = format!(
            "/orders?address={}&subaccountNumber={}",
            args.address, sub
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
    /// dYdX bech32 address (e.g., "dydx1...")
    pub(crate) address: String,
    /// Subaccount number. Defaults to 0.
    pub(crate) subaccount_number: Option<u32>,
    /// Market ticker filter (e.g., "BTC-USD"). Omit for all markets.
    pub(crate) market: Option<String>,
    /// Maximum number of fills to return (default 100).
    pub(crate) limit: Option<u32>,
}

impl DynAomiTool for GetFills {
    type App = DydxApp;
    type Args = GetFillsArgs;
    const NAME: &'static str = "dydx_get_fills";
    const DESCRIPTION: &'static str =
        "Use when the user asks for their executed trades, fill history, fees paid, or realized PnL inputs on dYdX. Returns fills for the subaccount with price, size, side, fee, and timestamp.";

    fn run(_app: &DydxApp, args: Self::Args, _ctx: DynToolCallCtx) -> Result<Value, String> {
        let client = DydxClient::new()?;
        let sub = args.subaccount_number.unwrap_or(0);
        let mut path = format!(
            "/fills?address={}&subaccountNumber={}",
            args.address, sub
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
    /// Maximum number of funding rate entries to return (default 100).
    pub(crate) limit: Option<u32>,
}

impl DynAomiTool for GetHistoricalFunding {
    type App = DydxApp;
    type Args = GetHistoricalFundingArgs;
    const NAME: &'static str = "dydx_get_historical_funding";
    const DESCRIPTION: &'static str = "Use when the user asks about funding rate history, basis trends, or whether a perp has been paying longs or shorts. Returns the per-hour funding rate series with rate, oracle price, and effective time. Rates are fractional per funding interval (typically 1 hour).";

    fn run(_app: &DydxApp, args: Self::Args, _ctx: DynToolCallCtx) -> Result<Value, String> {
        let client = DydxClient::new()?;
        let mut path = format!("/historical-funding/{}", args.ticker);
        if let Some(limit) = args.limit {
            path.push_str(&format!("?limit={limit}"));
        }
        client.get(&path)
    }
}
