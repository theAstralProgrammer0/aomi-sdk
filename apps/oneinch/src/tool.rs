use aomi_ext::oneinch::OneInchClient;
use aomi_sdk::*;
use aomi_sdk::schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Clone, Default)]
pub(crate) struct OneInchApp;

fn ok<T: Serialize>(value: T) -> Result<Value, String> {
    let value = serde_json::to_value(value)
        .map_err(|e| format!("[1inch] failed to serialize response: {e}"))?;
    Ok(match value {
        Value::Object(mut map) => {
            map.insert("source".to_string(), Value::String("1inch".to_string()));
            Value::Object(map)
        }
        other => serde_json::json!({ "source": "1inch", "data": other }),
    })
}

fn make_client(api_key: Option<&str>) -> Result<OneInchClient, String> {
    let api_key = resolve_secret_value(
        api_key,
        "ONEINCH_API_KEY",
        "[1inch] missing api_key argument and ONEINCH_API_KEY environment variable",
    )?;
    OneInchClient::new(api_key)
}

// ============================================================================
// Tool: GetOneInchQuote
// ============================================================================

pub(crate) struct GetOneInchQuote;

#[derive(Debug, Deserialize, JsonSchema)]
pub(crate) struct GetOneInchQuoteArgs {
    /// Optional 1inch API key. Falls back to ONEINCH_API_KEY when omitted.
    #[serde(default)]
    pub(crate) api_key: Option<String>,
    /// Chain ID (default: 1 for Ethereum). Supported: 1, 10, 56, 100, 137, 8453, 42161, 43114.
    pub(crate) chain_id: Option<u64>,
    /// Source token address (e.g. "0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48" for USDC)
    pub(crate) src: String,
    /// Destination token address
    pub(crate) dst: String,
    /// Amount in minimal divisible units (wei for ETH, smallest unit for tokens)
    pub(crate) amount: String,
    /// Comma-separated list of protocols to use (optional, uses all if omitted)
    pub(crate) protocols: Option<String>,
}

impl DynAomiTool for GetOneInchQuote {
    type App = OneInchApp;
    type Args = GetOneInchQuoteArgs;
    const NAME: &'static str = "get_oneinch_quote";
    const DESCRIPTION: &'static str = "Get a 1inch swap quote for price discovery (no transaction data). Returns optimal routing across DEXs.";

    fn run(_app: &OneInchApp, args: Self::Args, _ctx: DynToolCallCtx) -> Result<Value, String> {
        ok(make_client(args.api_key.as_deref())?.get_quote(
            args.chain_id.unwrap_or(1),
            &args.src,
            &args.dst,
            &args.amount,
            args.protocols.as_deref(),
        )?)
    }
}

// ============================================================================
// Tool: GetOneInchSwap
// ============================================================================

pub(crate) struct GetOneInchSwap;

#[derive(Debug, Deserialize, JsonSchema)]
pub(crate) struct GetOneInchSwapArgs {
    /// Optional 1inch API key. Falls back to ONEINCH_API_KEY when omitted.
    #[serde(default)]
    pub(crate) api_key: Option<String>,
    /// Chain ID (default: 1 for Ethereum). Supported: 1, 10, 56, 100, 137, 8453, 42161, 43114.
    pub(crate) chain_id: Option<u64>,
    /// Source token address
    pub(crate) src: String,
    /// Destination token address
    pub(crate) dst: String,
    /// Amount in minimal divisible units (wei for ETH, smallest unit for tokens)
    pub(crate) amount: String,
    /// Sender wallet address (the address that will execute the swap)
    pub(crate) from: String,
    /// Maximum acceptable slippage percentage (e.g. 1 for 1%)
    pub(crate) slippage: f64,
    /// Comma-separated list of protocols to use (optional, uses all if omitted)
    pub(crate) protocols: Option<String>,
}

impl DynAomiTool for GetOneInchSwap {
    type App = OneInchApp;
    type Args = GetOneInchSwapArgs;
    const NAME: &'static str = "get_oneinch_swap";
    const DESCRIPTION: &'static str = "Get a 1inch swap quote with executable transaction calldata. Returns a raw tx object (to, data, value, gas) that the host should stage with `stage_tx` using `data.raw`, verify with `simulate_batch`, then finalize with `commit_tx`.";

    fn run(_app: &OneInchApp, args: Self::Args, _ctx: DynToolCallCtx) -> Result<Value, String> {
        ok(make_client(args.api_key.as_deref())?.get_swap(
            args.chain_id.unwrap_or(1),
            &args.src,
            &args.dst,
            &args.amount,
            &args.from,
            args.slippage,
            args.protocols.as_deref(),
        )?)
    }
}

// ============================================================================
// Tool: GetOneInchApproveTransaction
// ============================================================================

pub(crate) struct GetOneInchApproveTransaction;

#[derive(Debug, Deserialize, JsonSchema)]
pub(crate) struct GetOneInchApproveTransactionArgs {
    /// Optional 1inch API key. Falls back to ONEINCH_API_KEY when omitted.
    #[serde(default)]
    pub(crate) api_key: Option<String>,
    /// Chain ID (default: 1 for Ethereum). Supported: 1, 10, 56, 100, 137, 8453, 42161, 43114.
    pub(crate) chain_id: Option<u64>,
    /// Token contract address to approve
    pub(crate) token_address: String,
    /// Approval amount in minimal divisible units (optional; omit for unlimited approval)
    pub(crate) amount: Option<String>,
}

impl DynAomiTool for GetOneInchApproveTransaction {
    type App = OneInchApp;
    type Args = GetOneInchApproveTransactionArgs;
    const NAME: &'static str = "get_oneinch_approve_transaction";
    const DESCRIPTION: &'static str = "Get transaction data to approve the 1inch router to spend a token. Returns a raw approval tx object (to, data, value). Omit amount for unlimited approval. Stage it directly with `stage_tx` using `data.raw`; do not re-encode calldata.";

    fn run(_app: &OneInchApp, args: Self::Args, _ctx: DynToolCallCtx) -> Result<Value, String> {
        ok(make_client(args.api_key.as_deref())?.get_approve_transaction(
            args.chain_id.unwrap_or(1),
            &args.token_address,
            args.amount.as_deref(),
        )?)
    }
}

// ============================================================================
// Tool: GetOneInchAllowance
// ============================================================================

pub(crate) struct GetOneInchAllowance;

#[derive(Debug, Deserialize, JsonSchema)]
pub(crate) struct GetOneInchAllowanceArgs {
    /// Optional 1inch API key. Falls back to ONEINCH_API_KEY when omitted.
    #[serde(default)]
    pub(crate) api_key: Option<String>,
    /// Chain ID (default: 1 for Ethereum). Supported: 1, 10, 56, 100, 137, 8453, 42161, 43114.
    pub(crate) chain_id: Option<u64>,
    /// Token contract address to check
    pub(crate) token_address: String,
    /// Wallet address to check allowance for
    pub(crate) wallet_address: String,
}

impl DynAomiTool for GetOneInchAllowance {
    type App = OneInchApp;
    type Args = GetOneInchAllowanceArgs;
    const NAME: &'static str = "get_oneinch_allowance";
    const DESCRIPTION: &'static str = "Check the current allowance the 1inch router has for a token from a given wallet. Returns the allowance amount.";

    fn run(_app: &OneInchApp, args: Self::Args, _ctx: DynToolCallCtx) -> Result<Value, String> {
        ok(make_client(args.api_key.as_deref())?.get_allowance(
            args.chain_id.unwrap_or(1),
            &args.token_address,
            &args.wallet_address,
        )?)
    }
}

// ============================================================================
// Tool: GetOneInchLiquiditySources
// ============================================================================

pub(crate) struct GetOneInchLiquiditySources;

#[derive(Debug, Deserialize, JsonSchema)]
pub(crate) struct GetOneInchLiquiditySourcesArgs {
    /// Optional 1inch API key. Falls back to ONEINCH_API_KEY when omitted.
    #[serde(default)]
    pub(crate) api_key: Option<String>,
    /// Chain ID (default: 1 for Ethereum). Supported: 1, 10, 56, 100, 137, 8453, 42161, 43114.
    pub(crate) chain_id: Option<u64>,
}

impl DynAomiTool for GetOneInchLiquiditySources {
    type App = OneInchApp;
    type Args = GetOneInchLiquiditySourcesArgs;
    const NAME: &'static str = "get_oneinch_liquidity_sources";
    const DESCRIPTION: &'static str =
        "List available DEXs and AMMs (liquidity sources) on a given chain for 1inch routing.";

    fn run(_app: &OneInchApp, args: Self::Args, _ctx: DynToolCallCtx) -> Result<Value, String> {
        ok(make_client(args.api_key.as_deref())?
            .get_liquidity_sources(args.chain_id.unwrap_or(1))?)
    }
}

// ============================================================================
// Tool: GetOneInchTokens
// ============================================================================

pub(crate) struct GetOneInchTokens;

#[derive(Debug, Deserialize, JsonSchema)]
pub(crate) struct GetOneInchTokensArgs {
    /// Optional 1inch API key. Falls back to ONEINCH_API_KEY when omitted.
    #[serde(default)]
    pub(crate) api_key: Option<String>,
    /// Chain ID (default: 1 for Ethereum). Supported: 1, 10, 56, 100, 137, 8453, 42161, 43114.
    pub(crate) chain_id: Option<u64>,
}

impl DynAomiTool for GetOneInchTokens {
    type App = OneInchApp;
    type Args = GetOneInchTokensArgs;
    const NAME: &'static str = "get_oneinch_tokens";
    const DESCRIPTION: &'static str = "List all supported tokens on a given chain. Returns token addresses, symbols, decimals, and logos.";

    fn run(_app: &OneInchApp, args: Self::Args, _ctx: DynToolCallCtx) -> Result<Value, String> {
        ok(make_client(args.api_key.as_deref())?.get_tokens(args.chain_id.unwrap_or(1))?)
    }
}
