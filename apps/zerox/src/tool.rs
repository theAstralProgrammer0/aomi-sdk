use aomi_ext::zerox::ZeroxClient;
use aomi_sdk::*;
use aomi_sdk::schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Clone, Default)]
pub(crate) struct ZeroxApp;

fn ok<T: Serialize>(value: T) -> Result<Value, String> {
    let value = serde_json::to_value(value)
        .map_err(|e| format!("[0x] failed to serialize response: {e}"))?;
    Ok(match value {
        Value::Object(mut map) => {
            map.insert("source".to_string(), Value::String("0x".to_string()));
            Value::Object(map)
        }
        other => serde_json::json!({ "source": "0x", "data": other }),
    })
}

fn make_client(api_key: Option<&str>) -> Result<ZeroxClient, String> {
    let api_key = resolve_secret_value(
        api_key,
        "ZEROX_API_KEY",
        "[0x] missing api_key argument and ZEROX_API_KEY environment variable",
    )?;
    ZeroxClient::new(api_key)
}

// ============================================================================
// Tool: GetZeroxSwapQuote
// ============================================================================

pub(crate) struct GetZeroxSwapQuote;

#[derive(Debug, Deserialize, JsonSchema)]
pub(crate) struct GetZeroxSwapQuoteArgs {
    /// Optional 0x API key. Falls back to ZEROX_API_KEY when omitted.
    pub(crate) api_key: Option<String>,
    /// Source chain
    pub(crate) chain: String,
    /// Sell token symbol or address
    pub(crate) sell_token: String,
    /// Buy token symbol or address
    pub(crate) buy_token: String,
    /// Amount to swap (human-readable units)
    pub(crate) amount: f64,
    /// Sender/taker address (optional for price quotes)
    pub(crate) sender_address: Option<String>,
    /// Slippage tolerance as decimal (0.005 = 0.5%)
    pub(crate) slippage: Option<f64>,
}

impl DynAomiTool for GetZeroxSwapQuote {
    type App = ZeroxApp;
    type Args = GetZeroxSwapQuoteArgs;
    const NAME: &'static str = "get_zerox_swap_quote";
    const DESCRIPTION: &'static str = "Get a 0x permit2/price swap quote for price discovery.";

    fn run(_app: &ZeroxApp, args: Self::Args, _ctx: DynToolCallCtx) -> Result<Value, String> {
        ok(make_client(args.api_key.as_deref())?.get_quote(
            &args.chain,
            &args.sell_token,
            &args.buy_token,
            args.amount,
            args.sender_address.as_deref(),
            args.slippage,
        )?)
    }
}

// ============================================================================
// Tool: PlaceZeroxOrder
// ============================================================================

pub(crate) struct PlaceZeroxOrder;

#[derive(Debug, Deserialize, JsonSchema)]
pub(crate) struct PlaceZeroxOrderArgs {
    /// Optional 0x API key. Falls back to ZEROX_API_KEY when omitted.
    pub(crate) api_key: Option<String>,
    /// Source chain
    pub(crate) chain: String,
    /// Sell token symbol or address
    pub(crate) sell_token: String,
    /// Buy token symbol or address
    pub(crate) buy_token: String,
    /// Sell amount (human-readable units)
    pub(crate) amount: f64,
    /// Sender/taker wallet address (required)
    pub(crate) sender_address: String,
    /// Slippage tolerance as decimal (0.005 = 0.5%)
    pub(crate) slippage: Option<f64>,
}

impl DynAomiTool for PlaceZeroxOrder {
    type App = ZeroxApp;
    type Args = PlaceZeroxOrderArgs;
    const NAME: &'static str = "place_zerox_order";
    const DESCRIPTION: &'static str = "Get executable tx data via 0x allowance-holder/quote. Returns a raw transaction payload (to, data, value) that the host should stage with `stage_tx` using `data.raw`, verify with `simulate_batch`, then finalize with `commit_tx`.";

    fn run(_app: &ZeroxApp, args: Self::Args, _ctx: DynToolCallCtx) -> Result<Value, String> {
        let quote = make_client(args.api_key.as_deref())?.place_order(
            &args.chain,
            &args.sell_token,
            &args.buy_token,
            args.amount,
            &args.sender_address,
            args.slippage,
        )?;

        let tx = serde_json::to_value(
            quote
                .transaction
                .as_ref()
                .ok_or_else(|| "0x response missing transaction payload".to_string())?,
        )
        .map_err(|e| format!("failed to encode 0x transaction payload: {e}"))?;

        ok(serde_json::json!({
            "quote": quote,
            "transaction": tx,
            "note": "Stage this raw 0x transaction with stage_tx using data.raw, verify the staged pending_tx_id with simulate_batch, then call commit_tx. Do not re-encode 0x calldata.",
        }))
    }
}

// ============================================================================
// High Priority tools
// ============================================================================

pub(crate) struct GetZeroxSwapChains;

#[derive(Debug, Deserialize, JsonSchema)]
pub(crate) struct GetZeroxSwapChainsArgs {
    /// Optional 0x API key. Falls back to ZEROX_API_KEY when omitted.
    pub(crate) api_key: Option<String>,
}

impl DynAomiTool for GetZeroxSwapChains {
    type App = ZeroxApp;
    type Args = GetZeroxSwapChainsArgs;
    const NAME: &'static str = "get_zerox_swap_chains";
    const DESCRIPTION: &'static str =
        "List all chains supported by the 0x Swap API. Returns an array of { chainName, chainId }.";

    fn run(_app: &ZeroxApp, args: Self::Args, _ctx: DynToolCallCtx) -> Result<Value, String> {
        ok(make_client(args.api_key.as_deref())?.get_swap_chains()?)
    }
}

pub(crate) struct GetZeroxAllowanceHolderPrice;

#[derive(Debug, Deserialize, JsonSchema)]
pub(crate) struct GetZeroxAllowanceHolderPriceArgs {
    /// Optional 0x API key. Falls back to ZEROX_API_KEY when omitted.
    pub(crate) api_key: Option<String>,
    /// Source chain (e.g. "ethereum", "polygon")
    pub(crate) chain: String,
    /// Sell token symbol or address
    pub(crate) sell_token: String,
    /// Buy token symbol or address
    pub(crate) buy_token: String,
    /// Amount to sell (human-readable units)
    pub(crate) amount: f64,
    /// Sender/taker address (optional for price discovery)
    pub(crate) sender_address: Option<String>,
    /// Slippage tolerance as decimal (0.005 = 0.5%)
    pub(crate) slippage: Option<f64>,
}

impl DynAomiTool for GetZeroxAllowanceHolderPrice {
    type App = ZeroxApp;
    type Args = GetZeroxAllowanceHolderPriceArgs;
    const NAME: &'static str = "get_zerox_allowance_holder_price";
    const DESCRIPTION: &'static str = "Get a 0x allowance-holder/price quote for price discovery. Matches the AllowanceHolder execution path so the price reflects actual execution costs.";

    fn run(_app: &ZeroxApp, args: Self::Args, _ctx: DynToolCallCtx) -> Result<Value, String> {
        ok(make_client(args.api_key.as_deref())?.get_allowance_holder_price(
            &args.chain,
            &args.sell_token,
            &args.buy_token,
            args.amount,
            args.sender_address.as_deref(),
            args.slippage,
        )?)
    }
}

pub(crate) struct GetZeroxLiquiditySources;

#[derive(Debug, Deserialize, JsonSchema)]
pub(crate) struct GetZeroxLiquiditySourcesArgs {
    /// Optional 0x API key. Falls back to ZEROX_API_KEY when omitted.
    pub(crate) api_key: Option<String>,
    /// Chain name (e.g. "ethereum", "polygon")
    pub(crate) chain: String,
}

impl DynAomiTool for GetZeroxLiquiditySources {
    type App = ZeroxApp;
    type Args = GetZeroxLiquiditySourcesArgs;
    const NAME: &'static str = "get_zerox_liquidity_sources";
    const DESCRIPTION: &'static str =
        "List available DEXs and AMMs (liquidity sources) on a given chain.";

    fn run(_app: &ZeroxApp, args: Self::Args, _ctx: DynToolCallCtx) -> Result<Value, String> {
        ok(make_client(args.api_key.as_deref())?.get_liquidity_sources(&args.chain)?)
    }
}

// ============================================================================
// Gasless tools
// ============================================================================

pub(crate) struct GetZeroxGaslessPrice;

#[derive(Debug, Deserialize, JsonSchema)]
pub(crate) struct GetZeroxGaslessPriceArgs {
    /// Optional 0x API key. Falls back to ZEROX_API_KEY when omitted.
    pub(crate) api_key: Option<String>,
    /// Source chain (e.g. "ethereum", "polygon")
    pub(crate) chain: String,
    /// Sell token symbol or address (must be ERC-20, not native)
    pub(crate) sell_token: String,
    /// Buy token symbol or address
    pub(crate) buy_token: String,
    /// Amount to sell (human-readable units)
    pub(crate) amount: f64,
    /// Sender/taker address (optional)
    pub(crate) sender_address: Option<String>,
    /// Slippage tolerance as decimal (0.005 = 0.5%)
    pub(crate) slippage: Option<f64>,
}

impl DynAomiTool for GetZeroxGaslessPrice {
    type App = ZeroxApp;
    type Args = GetZeroxGaslessPriceArgs;
    const NAME: &'static str = "get_zerox_gasless_price";
    const DESCRIPTION: &'static str = "Get a gasless swap price quote from 0x. The sell token must be an ERC-20 token (not native ETH/MATIC/etc.).";

    fn run(_app: &ZeroxApp, args: Self::Args, _ctx: DynToolCallCtx) -> Result<Value, String> {
        ok(make_client(args.api_key.as_deref())?.get_gasless_price(
            &args.chain,
            &args.sell_token,
            &args.buy_token,
            args.amount,
            args.sender_address.as_deref(),
            args.slippage,
        )?)
    }
}

pub(crate) struct GetZeroxGaslessQuote;

#[derive(Debug, Deserialize, JsonSchema)]
pub(crate) struct GetZeroxGaslessQuoteArgs {
    /// Optional 0x API key. Falls back to ZEROX_API_KEY when omitted.
    pub(crate) api_key: Option<String>,
    /// Source chain (e.g. "ethereum", "polygon")
    pub(crate) chain: String,
    /// Sell token symbol or address (must be ERC-20, not native)
    pub(crate) sell_token: String,
    /// Buy token symbol or address
    pub(crate) buy_token: String,
    /// Amount to sell (human-readable units)
    pub(crate) amount: f64,
    /// Sender/taker wallet address (required)
    pub(crate) sender_address: String,
    /// Slippage tolerance as decimal (0.005 = 0.5%)
    pub(crate) slippage: Option<f64>,
}

impl DynAomiTool for GetZeroxGaslessQuote {
    type App = ZeroxApp;
    type Args = GetZeroxGaslessQuoteArgs;
    const NAME: &'static str = "get_zerox_gasless_quote";
    const DESCRIPTION: &'static str = "Get a gasless swap quote with EIP-712 typed data for signing. Returns approval (optional) and trade objects that the user must sign before submitting.";

    fn run(_app: &ZeroxApp, args: Self::Args, _ctx: DynToolCallCtx) -> Result<Value, String> {
        ok(make_client(args.api_key.as_deref())?.get_gasless_quote(
            &args.chain,
            &args.sell_token,
            &args.buy_token,
            args.amount,
            &args.sender_address,
            args.slippage,
        )?)
    }
}

pub(crate) struct SubmitZeroxGaslessSwap;

#[derive(Debug, Deserialize, JsonSchema)]
pub(crate) struct SubmitZeroxGaslessSwapArgs {
    /// Optional 0x API key. Falls back to ZEROX_API_KEY when omitted.
    pub(crate) api_key: Option<String>,
    /// Chain ID (numeric, e.g. 1 for Ethereum)
    pub(crate) chain_id: u64,
    /// Signed trade object from gasless quote
    pub(crate) trade: Value,
    /// Signed approval object (if the gasless quote required one)
    pub(crate) approval: Option<Value>,
}

impl DynAomiTool for SubmitZeroxGaslessSwap {
    type App = ZeroxApp;
    type Args = SubmitZeroxGaslessSwapArgs;
    const NAME: &'static str = "submit_zerox_gasless_swap";
    const DESCRIPTION: &'static str = "Submit a signed gasless trade (and optional approval) to the 0x relayer. Returns a tradeHash for status polling.";

    fn run(_app: &ZeroxApp, args: Self::Args, _ctx: DynToolCallCtx) -> Result<Value, String> {
        ok(make_client(args.api_key.as_deref())?.submit_gasless_swap(
            args.chain_id,
            &args.trade,
            args.approval.as_ref(),
        )?)
    }
}

pub(crate) struct GetZeroxGaslessStatus;

#[derive(Debug, Deserialize, JsonSchema)]
pub(crate) struct GetZeroxGaslessStatusArgs {
    /// Optional 0x API key. Falls back to ZEROX_API_KEY when omitted.
    pub(crate) api_key: Option<String>,
    /// Trade hash returned by submit_zerox_gasless_swap
    pub(crate) trade_hash: String,
    /// Chain ID (numeric, e.g. 1 for Ethereum)
    pub(crate) chain_id: u64,
}

impl DynAomiTool for GetZeroxGaslessStatus {
    type App = ZeroxApp;
    type Args = GetZeroxGaslessStatusArgs;
    const NAME: &'static str = "get_zerox_gasless_status";
    const DESCRIPTION: &'static str = "Poll the status of a gasless trade by tradeHash. Status progresses: pending -> succeeded -> confirmed.";

    fn run(_app: &ZeroxApp, args: Self::Args, _ctx: DynToolCallCtx) -> Result<Value, String> {
        ok(make_client(args.api_key.as_deref())?
            .get_gasless_status(&args.trade_hash, args.chain_id)?)
    }
}

pub(crate) struct GetZeroxGaslessChains;

#[derive(Debug, Deserialize, JsonSchema)]
pub(crate) struct GetZeroxGaslessChainsArgs {
    /// Optional 0x API key. Falls back to ZEROX_API_KEY when omitted.
    pub(crate) api_key: Option<String>,
}

impl DynAomiTool for GetZeroxGaslessChains {
    type App = ZeroxApp;
    type Args = GetZeroxGaslessChainsArgs;
    const NAME: &'static str = "get_zerox_gasless_chains";
    const DESCRIPTION: &'static str = "List all chains that support gasless swaps via the 0x API.";

    fn run(_app: &ZeroxApp, args: Self::Args, _ctx: DynToolCallCtx) -> Result<Value, String> {
        ok(make_client(args.api_key.as_deref())?.get_gasless_chains()?)
    }
}
