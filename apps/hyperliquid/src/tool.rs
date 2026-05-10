//! Curated tool layer for Hyperliquid public info API. Built on top of the
//! progenitor-generated client at `aomi_ext::hyperliquid::Client` — see
//! ext/specs/hyperliquid.yaml.
//!
//! Hyperliquid's whole read surface is a single `POST /info` URL discriminated
//! by a `type` field in the body. Each tool here builds the appropriate JSON
//! body and forwards it to `client.post_info(&body)`.

use aomi_ext::hyperliquid::Client as HyperliquidClient;
use aomi_sdk::*;
use aomi_sdk::schemars::JsonSchema;
use serde::Deserialize;
use serde_json::{Map, Value, json};

#[derive(Clone, Default)]
pub(crate) struct HyperliquidApp;

const DEFAULT_API_URL: &str = "https://api.hyperliquid.xyz";

// ============================================================================
// Helpers
// ============================================================================

fn rt() -> Result<tokio::runtime::Runtime, String> {
    tokio::runtime::Runtime::new().map_err(|e| format!("[hyperliquid] runtime: {e}"))
}

fn base_url() -> String {
    std::env::var("HYPERLIQUID_API_URL").unwrap_or_else(|_| DEFAULT_API_URL.to_string())
}

fn with_source(value: Value) -> Value {
    match value {
        Value::Object(mut map) => {
            map.insert(
                "source".to_string(),
                Value::String("hyperliquid".to_string()),
            );
            Value::Object(map)
        }
        other => json!({
            "source": "hyperliquid",
            "data": other,
        }),
    }
}

/// Build a body Map from a JSON object literal and post it.
fn post_info(body: Value) -> Result<Value, String> {
    let map: Map<String, Value> = match body {
        Value::Object(m) => m,
        _ => return Err("[hyperliquid] body must be an object".to_string()),
    };
    rt()?.block_on(async move {
        let client = HyperliquidClient::new(&base_url());
        let resp = client
            .post_info(&map)
            .await
            .map_err(|e| format!("[hyperliquid] post /info: {e}"))?
            .into_inner();
        Ok(with_source(resp))
    })
}

// ============================================================================
// Tool structs + Args
// ============================================================================

pub(crate) struct GetMeta;

#[derive(Debug, Deserialize, JsonSchema)]
pub(crate) struct GetMetaArgs {}

pub(crate) struct GetAllMids;

#[derive(Debug, Deserialize, JsonSchema)]
pub(crate) struct GetAllMidsArgs {}

pub(crate) struct GetL2Book;

#[derive(Debug, Deserialize, JsonSchema)]
pub(crate) struct GetL2BookArgs {
    /// Asset ticker (e.g., "BTC", "ETH", "SOL")
    pub(crate) coin: String,
}

pub(crate) struct GetClearinghouseState;

#[derive(Debug, Deserialize, JsonSchema)]
pub(crate) struct GetClearinghouseStateArgs {
    /// Ethereum-style address (0x...)
    pub(crate) user: String,
}

pub(crate) struct GetOpenOrders;

#[derive(Debug, Deserialize, JsonSchema)]
pub(crate) struct GetOpenOrdersArgs {
    /// Ethereum-style address (0x...)
    pub(crate) user: String,
}

pub(crate) struct GetUserFills;

#[derive(Debug, Deserialize, JsonSchema)]
pub(crate) struct GetUserFillsArgs {
    /// Ethereum-style address (0x...)
    pub(crate) user: String,
}

pub(crate) struct GetFundingHistory;

#[derive(Debug, Deserialize, JsonSchema)]
pub(crate) struct GetFundingHistoryArgs {
    /// Asset ticker (e.g., "BTC", "ETH")
    pub(crate) coin: String,
    /// Start time in milliseconds (Unix epoch)
    pub(crate) start_time: u64,
    /// End time in milliseconds (Unix epoch). Optional -- defaults to now.
    #[serde(default)]
    pub(crate) end_time: Option<u64>,
}

pub(crate) struct GetCandleSnapshot;

#[derive(Debug, Deserialize, JsonSchema)]
pub(crate) struct GetCandleSnapshotArgs {
    /// Asset ticker (e.g., "BTC", "ETH")
    pub(crate) coin: String,
    /// Candle interval: "1m", "5m", "15m", "1h", "4h", "1d"
    pub(crate) interval: String,
    /// Start time in milliseconds (Unix epoch)
    pub(crate) start_time: u64,
    /// End time in milliseconds (Unix epoch)
    pub(crate) end_time: u64,
}

// ============================================================================
// Impls
// ============================================================================

impl DynAomiTool for GetMeta {
    type App = HyperliquidApp;
    type Args = GetMetaArgs;
    const NAME: &'static str = "get_meta";
    const DESCRIPTION: &'static str = "Get exchange metadata including the universe of tradeable perpetual assets and their size decimals.";

    fn run(_app: &HyperliquidApp, _args: Self::Args, _ctx: DynToolCallCtx) -> Result<Value, String> {
        post_info(json!({"type": "meta"}))
    }
}

impl DynAomiTool for GetAllMids {
    type App = HyperliquidApp;
    type Args = GetAllMidsArgs;
    const NAME: &'static str = "get_all_mids";
    const DESCRIPTION: &'static str =
        "Get current mid-prices for all listed assets on Hyperliquid.";

    fn run(_app: &HyperliquidApp, _args: Self::Args, _ctx: DynToolCallCtx) -> Result<Value, String> {
        post_info(json!({"type": "allMids"}))
    }
}

impl DynAomiTool for GetL2Book {
    type App = HyperliquidApp;
    type Args = GetL2BookArgs;
    const NAME: &'static str = "get_l2_book";
    const DESCRIPTION: &'static str =
        "Get L2 order book snapshot (bid and ask levels) for a specific asset.";

    fn run(_app: &HyperliquidApp, args: Self::Args, _ctx: DynToolCallCtx) -> Result<Value, String> {
        post_info(json!({"type": "l2Book", "coin": args.coin}))
    }
}

impl DynAomiTool for GetClearinghouseState {
    type App = HyperliquidApp;
    type Args = GetClearinghouseStateArgs;
    const NAME: &'static str = "get_clearinghouse_state";
    const DESCRIPTION: &'static str = "Get account state including positions, margin summary, and account value for a user address.";

    fn run(_app: &HyperliquidApp, args: Self::Args, _ctx: DynToolCallCtx) -> Result<Value, String> {
        post_info(json!({"type": "clearinghouseState", "user": args.user}))
    }
}

impl DynAomiTool for GetOpenOrders {
    type App = HyperliquidApp;
    type Args = GetOpenOrdersArgs;
    const NAME: &'static str = "get_open_orders";
    const DESCRIPTION: &'static str = "Get all open (pending) orders for a user address, including coin, side, size, limit price, and order ID.";

    fn run(_app: &HyperliquidApp, args: Self::Args, _ctx: DynToolCallCtx) -> Result<Value, String> {
        post_info(json!({"type": "openOrders", "user": args.user}))
    }
}

impl DynAomiTool for GetUserFills {
    type App = HyperliquidApp;
    type Args = GetUserFillsArgs;
    const NAME: &'static str = "get_user_fills";
    const DESCRIPTION: &'static str = "Get trade fill history for a user address, including fees, timestamps, and transaction hashes.";

    fn run(_app: &HyperliquidApp, args: Self::Args, _ctx: DynToolCallCtx) -> Result<Value, String> {
        post_info(json!({"type": "userFills", "user": args.user}))
    }
}

impl DynAomiTool for GetFundingHistory {
    type App = HyperliquidApp;
    type Args = GetFundingHistoryArgs;
    const NAME: &'static str = "get_funding_history";
    const DESCRIPTION: &'static str =
        "Get historical funding rate snapshots for a specific asset over a time range.";

    fn run(_app: &HyperliquidApp, args: Self::Args, _ctx: DynToolCallCtx) -> Result<Value, String> {
        let mut body = json!({
            "type": "fundingHistory",
            "coin": args.coin,
            "startTime": args.start_time,
        });
        if let Some(et) = args.end_time {
            body.as_object_mut()
                .unwrap()
                .insert("endTime".to_string(), json!(et));
        }
        post_info(body)
    }
}

impl DynAomiTool for GetCandleSnapshot {
    type App = HyperliquidApp;
    type Args = GetCandleSnapshotArgs;
    const NAME: &'static str = "get_candle_snapshot";
    const DESCRIPTION: &'static str =
        "Get OHLCV candlestick data for a specific asset at a given interval and time range.";

    fn run(_app: &HyperliquidApp, args: Self::Args, _ctx: DynToolCallCtx) -> Result<Value, String> {
        post_info(json!({
            "type": "candleSnapshot",
            "req": {
                "coin": args.coin,
                "interval": args.interval,
                "startTime": args.start_time,
                "endTime": args.end_time,
            }
        }))
    }
}
