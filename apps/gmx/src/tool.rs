use aomi_ext::gmx::{GmxClient, resolve_chain_label};
use aomi_sdk::*;
use aomi_sdk::schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

#[derive(Clone, Default)]
pub(crate) struct GmxApp;

fn ok<T: Serialize>(value: T, chain: &str) -> Result<Value, String> {
    let value = serde_json::to_value(value)
        .map_err(|e| format!("[gmx] failed to serialize response: {e}"))?;
    Ok(match value {
        Value::Object(mut map) => {
            map.insert("source".to_string(), Value::String("gmx".to_string()));
            map.insert("chain".to_string(), Value::String(chain.to_string()));
            Value::Object(map)
        }
        other => json!({ "source": "gmx", "chain": chain, "data": other }),
    })
}

// ============================================================================
// Tool: GetGmxPrices
// ============================================================================

pub(crate) struct GetGmxPrices;

#[derive(Debug, Deserialize, JsonSchema)]
pub(crate) struct GetGmxPricesArgs {
    /// Chain to query: "arbitrum" (default) or "avalanche"
    #[serde(default)]
    pub(crate) chain: Option<String>,
}

impl DynAomiTool for GetGmxPrices {
    type App = GmxApp;
    type Args = GetGmxPricesArgs;
    const NAME: &'static str = "get_gmx_prices";
    const DESCRIPTION: &'static str = "Use when the user asks the current price of any token tradable on GMX (BTC, ETH, ARB, AVAX, etc.) or wants to compare GMX's oracle to other venues. Returns min/max oracle prices and the address+symbol for every listed token on the chosen chain. Defaults to Arbitrum.";

    fn run(_app: &GmxApp, args: Self::Args, _ctx: DynToolCallCtx) -> Result<Value, String> {
        let chain = resolve_chain_label(args.chain.as_deref());
        ok(
            json!({ "tickers": GmxClient::new(args.chain.as_deref())?.get_prices()? }),
            chain,
        )
    }
}

// ============================================================================
// Tool: GetGmxSignedPrices
// ============================================================================

pub(crate) struct GetGmxSignedPrices;

#[derive(Debug, Deserialize, JsonSchema)]
pub(crate) struct GetGmxSignedPricesArgs {
    /// Chain to query: "arbitrum" (default) or "avalanche"
    #[serde(default)]
    pub(crate) chain: Option<String>,
}

impl DynAomiTool for GetGmxSignedPrices {
    type App = GmxApp;
    type Args = GetGmxSignedPricesArgs;
    const NAME: &'static str = "get_gmx_signed_prices";
    const DESCRIPTION: &'static str = "Use only when the user specifically asks about keeper-signed oracle prices, EIP-712 price payloads, or what prices GMX will use for on-chain settlement. Returns the latest signed price packets from GMX keepers (with signatures and validity windows). For ordinary 'what is the price' questions use get_gmx_prices instead.";

    fn run(_app: &GmxApp, args: Self::Args, _ctx: DynToolCallCtx) -> Result<Value, String> {
        let chain = resolve_chain_label(args.chain.as_deref());
        ok(
            GmxClient::new(args.chain.as_deref())?.get_signed_prices()?,
            chain,
        )
    }
}

// ============================================================================
// Tool: GetGmxMarkets
// ============================================================================

pub(crate) struct GetGmxMarkets;

#[derive(Debug, Deserialize, JsonSchema)]
pub(crate) struct GetGmxMarketsArgs {
    /// Chain to query: "arbitrum" (default) or "avalanche"
    #[serde(default)]
    pub(crate) chain: Option<String>,
}

impl DynAomiTool for GetGmxMarkets {
    type App = GmxApp;
    type Args = GetGmxMarketsArgs;
    const NAME: &'static str = "get_gmx_markets";
    const DESCRIPTION: &'static str = "Use when the user asks what markets exist on GMX v2, GM pool composition, current funding rates, open interest skew, borrow rates, or pool TVL. Returns every GM market on the chosen chain with market address, index/long/short tokens, funding and borrow rates, and long/short OI. Defaults to Arbitrum.";

    fn run(_app: &GmxApp, args: Self::Args, _ctx: DynToolCallCtx) -> Result<Value, String> {
        let chain = resolve_chain_label(args.chain.as_deref());
        ok(
            json!({ "markets": GmxClient::new(args.chain.as_deref())?.get_markets()? }),
            chain,
        )
    }
}

// ============================================================================
// Tool: GetGmxPositions
// ============================================================================

pub(crate) struct GetGmxPositions;

#[derive(Debug, Deserialize, JsonSchema)]
pub(crate) struct GetGmxPositionsArgs {
    /// Ethereum address of the account to query positions for (e.g. "0x1234...")
    pub(crate) account: String,
    /// Chain to query: "arbitrum" (default) or "avalanche"
    #[serde(default)]
    pub(crate) chain: Option<String>,
}

impl DynAomiTool for GetGmxPositions {
    type App = GmxApp;
    type Args = GetGmxPositionsArgs;
    const NAME: &'static str = "get_gmx_positions";
    const DESCRIPTION: &'static str = "Use when the user asks about their (or another address's) open GMX v2 perp positions, size, leverage, collateral, entry price, or unrealized PnL. Returns one entry per open position with market, side, size, collateral, entry price, and PnL. Pass the EVM address (0x...). Defaults to Arbitrum.";

    fn run(_app: &GmxApp, args: Self::Args, _ctx: DynToolCallCtx) -> Result<Value, String> {
        let chain = resolve_chain_label(args.chain.as_deref());
        let positions = GmxClient::new(args.chain.as_deref())?.get_positions(&args.account)?;
        ok(
            json!({ "account": args.account, "positions": positions }),
            chain,
        )
    }
}

// ============================================================================
// Tool: GetGmxOrders
// ============================================================================

pub(crate) struct GetGmxOrders;

#[derive(Debug, Deserialize, JsonSchema)]
pub(crate) struct GetGmxOrdersArgs {
    /// Ethereum address of the account to query orders for (e.g. "0x1234...")
    pub(crate) account: String,
    /// Chain to query: "arbitrum" (default) or "avalanche"
    #[serde(default)]
    pub(crate) chain: Option<String>,
}

impl DynAomiTool for GetGmxOrders {
    type App = GmxApp;
    type Args = GetGmxOrdersArgs;
    const NAME: &'static str = "get_gmx_orders";
    const DESCRIPTION: &'static str = "Use when the user asks about their pending GMX v2 orders -- limit entries, take-profit, stop-loss, or any trigger order awaiting keeper execution. Returns one entry per pending order with order type, market, size delta, trigger price, and acceptable price. Pass the EVM address (0x...). Defaults to Arbitrum.";

    fn run(_app: &GmxApp, args: Self::Args, _ctx: DynToolCallCtx) -> Result<Value, String> {
        let chain = resolve_chain_label(args.chain.as_deref());
        let orders = GmxClient::new(args.chain.as_deref())?.get_orders(&args.account)?;
        ok(json!({ "account": args.account, "orders": orders }), chain)
    }
}
