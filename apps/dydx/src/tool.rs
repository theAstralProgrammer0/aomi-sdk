//! Curated tool layer for dYdX v4 Indexer. Hand-written from the
//! progenitor-generated client at `aomi_ext::dydx::Client` — see
//! ext/specs/dydx.yaml.
//!
//! Read-only public Indexer endpoints; no auth. Trades / orders are placed
//! via signed Cosmos transactions and are explicitly out of scope.
//!
//! Curated 8 tools (matching the original surface):
//!   * `dydx_get_markets`             — list markets / one ticker
//!   * `dydx_get_orderbook`           — L2 snapshot
//!   * `dydx_get_candles`             — OHLCV history
//!   * `dydx_get_trades`              — public trade tape
//!   * `dydx_get_account`             — subaccount snapshot
//!   * `dydx_get_orders`              — open / historical orders
//!   * `dydx_get_fills`               — user's executed trades
//!   * `dydx_get_historical_funding`  — per-hour funding series

use aomi_ext::dydx::Client as DydxClient;
use aomi_sdk::*;
use aomi_sdk::schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::num::NonZeroU32;

#[derive(Clone, Default)]
pub(crate) struct DydxApp;

const BASE_URL: &str = "https://indexer.dydx.trade/v4";

// ============================================================================
// Helpers
// ============================================================================

fn ok<T: Serialize>(value: T) -> Result<Value, String> {
    let value = serde_json::to_value(value).map_err(|e| format!("[dydx] serialize: {e}"))?;
    Ok(match value {
        Value::Object(mut m) => {
            m.insert("source".into(), Value::String("dydx".into()));
            Value::Object(m)
        }
        other => json!({ "source": "dydx", "data": other }),
    })
}

fn rt() -> Result<tokio::runtime::Runtime, String> {
    tokio::runtime::Runtime::new().map_err(|e| format!("[dydx] runtime: {e}"))
}

fn base_url() -> String {
    std::env::var("DYDX_INDEXER_URL").unwrap_or_else(|_| BASE_URL.to_string())
}

fn nz(limit: Option<u32>) -> Option<NonZeroU32> {
    limit.and_then(NonZeroU32::new)
}

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
        rt()?.block_on(async move {
            let client = DydxClient::new(&base_url());
            let r = client
                .get_perpetual_markets(args.ticker.as_deref())
                .await
                .map_err(|e| format!("[dydx] get markets: {e}"))?
                .into_inner();
            ok(r)
        })
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
        rt()?.block_on(async move {
            let client = DydxClient::new(&base_url());
            let r = client
                .get_orderbook(&args.ticker)
                .await
                .map_err(|e| format!("[dydx] orderbook {}: {e}", args.ticker))?
                .into_inner();
            ok(r)
        })
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
        rt()?.block_on(async move {
            let client = DydxClient::new(&base_url());
            let r = client
                .get_candles(&args.ticker, nz(args.limit), &args.resolution)
                .await
                .map_err(|e| format!("[dydx] candles {}: {e}", args.ticker))?
                .into_inner();
            ok(r)
        })
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
        rt()?.block_on(async move {
            let client = DydxClient::new(&base_url());
            let r = client
                .get_trades(&args.ticker, nz(args.limit))
                .await
                .map_err(|e| format!("[dydx] trades {}: {e}", args.ticker))?
                .into_inner();
            ok(r)
        })
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
        rt()?.block_on(async move {
            let client = DydxClient::new(&base_url());
            let sub = args.subaccount_number.unwrap_or(0);
            let r = client
                .get_subaccount(&args.address, sub)
                .await
                .map_err(|e| format!("[dydx] subaccount {}/{sub}: {e}", args.address))?
                .into_inner();
            ok(r)
        })
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
        rt()?.block_on(async move {
            let client = DydxClient::new(&base_url());
            let sub = args.subaccount_number.unwrap_or(0);
            let r = client
                .get_orders(
                    &args.address,
                    args.status.as_deref(),
                    sub,
                    args.ticker.as_deref(),
                )
                .await
                .map_err(|e| format!("[dydx] orders {}: {e}", args.address))?
                .into_inner();
            ok(r)
        })
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
        rt()?.block_on(async move {
            let client = DydxClient::new(&base_url());
            let sub = args.subaccount_number.unwrap_or(0);
            let r = client
                .get_fills(&args.address, nz(args.limit), args.market.as_deref(), sub)
                .await
                .map_err(|e| format!("[dydx] fills {}: {e}", args.address))?
                .into_inner();
            ok(r)
        })
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
        rt()?.block_on(async move {
            let client = DydxClient::new(&base_url());
            let r = client
                .get_historical_funding(&args.ticker, nz(args.limit))
                .await
                .map_err(|e| format!("[dydx] historical funding {}: {e}", args.ticker))?
                .into_inner();
            ok(r)
        })
    }
}
