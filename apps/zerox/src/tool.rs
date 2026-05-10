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
// Tool: ZeroxGetPrice (price discovery, no taker required)
// ============================================================================

pub(crate) struct ZeroxGetPrice;

#[derive(Debug, Deserialize, JsonSchema)]
pub(crate) struct ZeroxGetPriceArgs {
    /// Optional 0x API key. Falls back to ZEROX_API_KEY when omitted.
    #[serde(default)]
    pub(crate) api_key: Option<String>,
    /// Chain name (ethereum, polygon, arbitrum, optimism, base, bsc, avalanche).
    pub(crate) chain: String,
    /// Sell token symbol (e.g. "USDC", "ETH") or 0x... contract address.
    pub(crate) sell_token: String,
    /// Buy token symbol or 0x... contract address.
    pub(crate) buy_token: String,
    /// Sell amount in human-readable units (e.g. 100.0 for 100 USDC).
    pub(crate) amount: f64,
    /// Optional taker address (improves quote accuracy when included).
    #[serde(default)]
    pub(crate) sender_address: Option<String>,
    /// Slippage tolerance as a decimal (0.005 = 0.5%). Default: 0.01 (1%).
    #[serde(default)]
    pub(crate) slippage: Option<f64>,
}

impl DynAomiTool for ZeroxGetPrice {
    type App = ZeroxApp;
    type Args = ZeroxGetPriceArgs;
    const NAME: &'static str = "zerox_get_price";
    const DESCRIPTION: &'static str = "Use when the user asks for a 0x swap price (no signing). Returns expected `buyAmount` and routing for selling `amount` of `sell_token` for `buy_token`. Uses the AllowanceHolder pricing path so the quote reflects actual execution cost. No wallet signature required.";

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

// ============================================================================
// Tool: ZeroxBuildSwap (executable AllowanceHolder quote)
// ============================================================================

pub(crate) struct ZeroxBuildSwap;

#[derive(Debug, Deserialize, JsonSchema)]
pub(crate) struct ZeroxBuildSwapArgs {
    /// Optional 0x API key. Falls back to ZEROX_API_KEY when omitted.
    #[serde(default)]
    pub(crate) api_key: Option<String>,
    /// Chain name (ethereum, polygon, arbitrum, optimism, base, bsc, avalanche).
    pub(crate) chain: String,
    /// Sell token symbol or 0x... address.
    pub(crate) sell_token: String,
    /// Buy token symbol or 0x... address.
    pub(crate) buy_token: String,
    /// Sell amount in human-readable units.
    pub(crate) amount: f64,
    /// Taker wallet address (the address that will execute the swap).
    pub(crate) sender_address: String,
    /// Slippage tolerance as a decimal (0.005 = 0.5%). Default: 0.01.
    #[serde(default)]
    pub(crate) slippage: Option<f64>,
}

impl DynAomiTool for ZeroxBuildSwap {
    type App = ZeroxApp;
    type Args = ZeroxBuildSwapArgs;
    const NAME: &'static str = "zerox_build_swap";
    const DESCRIPTION: &'static str = "Use when the user is ready to execute a 0x swap. Returns a raw swap tx (`transaction.to`, `transaction.data`, `transaction.value`) plus an `issues.allowance` block. If `issues.allowance` indicates insufficient allowance, the host must first approve the returned `spender` (the 0x AllowanceHolder) for the sell token, then stage and commit the swap tx via `stage_tx` / `simulate_batch` / `commit_tx`. Do not re-encode 0x calldata.";

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
            "note": "If quote.issues.allowance is non-null, approve the returned `spender` (the AllowanceHolder, NOT the Exchange Proxy or Permit2) for the sell token before staging this swap. Stage with stage_tx using { raw: <tx hex> } or by passing transaction.to/data/value directly; never re-encode.",
        }))
    }
}

// ============================================================================
// Tool: ZeroxGetGaslessQuote (EIP-712 quote for signing)
// ============================================================================

pub(crate) struct ZeroxGetGaslessQuote;

#[derive(Debug, Deserialize, JsonSchema)]
pub(crate) struct ZeroxGetGaslessQuoteArgs {
    /// Optional 0x API key. Falls back to ZEROX_API_KEY when omitted.
    #[serde(default)]
    pub(crate) api_key: Option<String>,
    /// Chain name (ethereum, polygon, arbitrum, optimism, base, bsc, avalanche).
    pub(crate) chain: String,
    /// Sell token (must be ERC-20, not native ETH/MATIC/BNB/AVAX).
    pub(crate) sell_token: String,
    /// Buy token symbol or 0x... address.
    pub(crate) buy_token: String,
    /// Sell amount in human-readable units.
    pub(crate) amount: f64,
    /// Taker wallet address (the address that signs the EIP-712 payloads).
    pub(crate) sender_address: String,
    /// Slippage tolerance as a decimal (0.005 = 0.5%). Default: 0.01.
    #[serde(default)]
    pub(crate) slippage: Option<f64>,
}

impl DynAomiTool for ZeroxGetGaslessQuote {
    type App = ZeroxApp;
    type Args = ZeroxGetGaslessQuoteArgs;
    const NAME: &'static str = "zerox_get_gasless_quote";
    const DESCRIPTION: &'static str = "Use when the user wants a gasless 0x swap (relayer pays gas). Returns EIP-712 typed-data for `trade` and (sometimes) `approval` that the user must sign with the host wallet's `sign_typed_data`. After signing, call `zerox_submit_gasless_swap`. Sell token must be ERC-20 (not native).";

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

// ============================================================================
// Tool: ZeroxSubmitGaslessSwap
// ============================================================================

pub(crate) struct ZeroxSubmitGaslessSwap;

#[derive(Debug, Deserialize, JsonSchema)]
pub(crate) struct ZeroxSubmitGaslessSwapArgs {
    /// Optional 0x API key. Falls back to ZEROX_API_KEY when omitted.
    #[serde(default)]
    pub(crate) api_key: Option<String>,
    /// Numeric chain ID (e.g. 1 for Ethereum, 137 for Polygon).
    pub(crate) chain_id: u64,
    /// Signed `trade` object (signature attached) from `zerox_get_gasless_quote`.
    pub(crate) trade: Value,
    /// Signed `approval` object, if the gasless quote required one.
    #[serde(default)]
    pub(crate) approval: Option<Value>,
}

impl DynAomiTool for ZeroxSubmitGaslessSwap {
    type App = ZeroxApp;
    type Args = ZeroxSubmitGaslessSwapArgs;
    const NAME: &'static str = "zerox_submit_gasless_swap";
    const DESCRIPTION: &'static str = "Use after the user has signed the EIP-712 trade (and approval) returned by `zerox_get_gasless_quote`. Submits the signed payloads to the 0x relayer. Returns a `tradeHash` to poll with `zerox_get_gasless_status`.";

    fn run(_app: &ZeroxApp, args: Self::Args, _ctx: DynToolCallCtx) -> Result<Value, String> {
        ok(make_client(args.api_key.as_deref())?.submit_gasless_swap(
            args.chain_id,
            &args.trade,
            args.approval.as_ref(),
        )?)
    }
}

// ============================================================================
// Tool: ZeroxGetGaslessStatus
// ============================================================================

pub(crate) struct ZeroxGetGaslessStatus;

#[derive(Debug, Deserialize, JsonSchema)]
pub(crate) struct ZeroxGetGaslessStatusArgs {
    /// Optional 0x API key. Falls back to ZEROX_API_KEY when omitted.
    #[serde(default)]
    pub(crate) api_key: Option<String>,
    /// `tradeHash` returned by `zerox_submit_gasless_swap`.
    pub(crate) trade_hash: String,
    /// Numeric chain ID where the trade was submitted.
    pub(crate) chain_id: u64,
}

impl DynAomiTool for ZeroxGetGaslessStatus {
    type App = ZeroxApp;
    type Args = ZeroxGetGaslessStatusArgs;
    const NAME: &'static str = "zerox_get_gasless_status";
    const DESCRIPTION: &'static str = "Use to track a submitted gasless trade. Returns status that progresses pending -> succeeded -> confirmed. Confirmed means the on-chain settlement is finalized.";

    fn run(_app: &ZeroxApp, args: Self::Args, _ctx: DynToolCallCtx) -> Result<Value, String> {
        ok(make_client(args.api_key.as_deref())?
            .get_gasless_status(&args.trade_hash, args.chain_id)?)
    }
}
