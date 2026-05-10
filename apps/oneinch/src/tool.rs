//! Curated tool layer for 1inch Swap API v6.0.
//!
//! Wraps the generated client in `aomi_ext::oneinch` (see `ext/specs/oneinch.yaml`).
//! Designed for the user story: "swap tokens on EVM chains via 1inch — get a
//! quote, build the swap transaction the user signs, manage approvals."
//!
//! The 6 mechanical stubs from `aomi-build gen-tool` collapse into 5
//! user-centric tools:
//!
//!   * `oneinch_get_quote`        — price-only quote for a swap
//!   * `oneinch_build_swap_tx`    — composite quote + allowance check + swap tx
//!   * `oneinch_check_allowance`  — current router allowance for an ERC-20
//!   * `oneinch_get_approve_tx`   — raw ERC-20 approval calldata
//!   * `oneinch_list_tokens`      — supported token list for a chain

use aomi_ext::oneinch::Client as GenClient;
use aomi_sdk::*;
use aomi_sdk::schemars::JsonSchema;
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::time::Duration;

#[derive(Clone, Default)]
pub(crate) struct OneinchApp;

const BASE_URL: &str = "https://api.1inch.dev/swap/v6.0";

/// EVM chain ids 1inch officially supports for Swap API v6.0.
const SUPPORTED_CHAINS: &[i64] = &[1, 10, 56, 100, 137, 8453, 42161, 43114];

// ============================================================================
// Helpers
// ============================================================================

fn ok<T: Serialize>(value: T) -> Result<Value, String> {
    let value = serde_json::to_value(value).map_err(|e| format!("[1inch] serialize: {e}"))?;
    Ok(match value {
        Value::Object(mut m) => {
            m.insert("source".into(), Value::String("1inch".into()));
            Value::Object(m)
        }
        other => json!({ "source": "1inch", "data": other }),
    })
}

fn rt() -> Result<tokio::runtime::Runtime, String> {
    tokio::runtime::Runtime::new().map_err(|e| format!("[1inch] runtime: {e}"))
}

fn resolve_key(arg: Option<&str>) -> Result<String, String> {
    resolve_secret_value(
        arg,
        "ONEINCH_API_KEY",
        "[1inch] missing api_key argument and ONEINCH_API_KEY env var",
    )
}

fn validate_chain(chain_id: i64) -> Result<(), String> {
    if SUPPORTED_CHAINS.contains(&chain_id) {
        Ok(())
    } else {
        Err(format!(
            "[1inch] unsupported chain_id {chain_id}. Supported: {SUPPORTED_CHAINS:?}"
        ))
    }
}

/// Build a generated client carrying a `Bearer <api_key>` default header.
/// 1inch requires the bearer token on every request; the spec's `securitySchemes`
/// only declares the header name, so we wire it in here.
fn make_client(api_key: &str) -> Result<GenClient, String> {
    let mut headers = HeaderMap::new();
    let mut bearer = HeaderValue::from_str(&format!("Bearer {api_key}"))
        .map_err(|e| format!("[1inch] invalid api_key: {e}"))?;
    bearer.set_sensitive(true);
    headers.insert(AUTHORIZATION, bearer);

    let http = reqwest::ClientBuilder::new()
        .timeout(Duration::from_secs(30))
        .default_headers(headers)
        .build()
        .map_err(|e| format!("[1inch] failed to build HTTP client: {e}"))?;

    Ok(GenClient::new_with_client(BASE_URL, http))
}

/// Parse a u128 token amount; surfaces a friendly error rather than letting the
/// API reject the request later.
fn parse_amount(s: &str) -> Result<u128, String> {
    s.parse::<u128>()
        .map_err(|_| format!("[1inch] amount must be a non-negative integer string, got {s:?}"))
}

// ============================================================================
// Tool 1: oneinch_get_quote — price-only quote (no wallet, no tx)
// ============================================================================

pub(crate) struct GetQuote;

#[derive(Debug, Deserialize, JsonSchema)]
pub(crate) struct GetQuoteArgs {
    /// Optional 1inch developer API key. Falls back to ONEINCH_API_KEY env var.
    #[serde(default)]
    pub api_key: Option<String>,
    /// EVM chain id. Defaults to 1 (Ethereum). Supported: 1, 10, 56, 100, 137, 8453, 42161, 43114.
    #[serde(default)]
    pub chain_id: Option<i64>,
    /// Source token address (0x...). Use 0xEeeE...EEeE for native ETH/MATIC/BNB/AVAX.
    pub src: String,
    /// Destination token address (0x...). Use 0xEeeE...EEeE for native asset.
    pub dst: String,
    /// Sell amount in source-token base units (wei for 18-dec tokens; "1000000" = 1 USDC).
    pub amount: String,
}

impl DynAomiTool for GetQuote {
    type App = OneinchApp;
    type Args = GetQuoteArgs;
    const NAME: &'static str = "oneinch_get_quote";
    const DESCRIPTION: &'static str = "Use when the user asks for a 1inch swap price (no transaction). Returns the optimal route across DEXs and the expected `dstAmount` for selling `amount` of `src` for `dst` on the given chain. No wallet address required.";

    fn run(_app: &OneinchApp, args: Self::Args, _ctx: DynToolCallCtx) -> Result<Value, String> {
        let api_key = resolve_key(args.api_key.as_deref())?;
        let chain_id = args.chain_id.unwrap_or(1);
        validate_chain(chain_id)?;
        parse_amount(&args.amount)?;

        let client = make_client(&api_key)?;
        let runtime = rt()?;
        let result = runtime
            .block_on(async move {
                client
                    .get_quote(
                        chain_id,
                        args.amount.as_str(),
                        args.dst.as_str(),
                        None,
                        args.src.as_str(),
                    )
                    .await
            })
            .map_err(|e| format!("[1inch] get_quote: {e}"))?;
        ok(result.into_inner())
    }
}

// ============================================================================
// Tool 2: oneinch_build_swap_tx — composite quote + allowance + swap tx
// ============================================================================

pub(crate) struct BuildSwapTx;

#[derive(Debug, Deserialize, JsonSchema)]
pub(crate) struct BuildSwapTxArgs {
    /// Optional 1inch developer API key. Falls back to ONEINCH_API_KEY env var.
    #[serde(default)]
    pub api_key: Option<String>,
    /// EVM chain id. Defaults to 1 (Ethereum).
    #[serde(default)]
    pub chain_id: Option<i64>,
    /// Source token address. Use 0xEeeE...EEeE for native asset.
    pub src: String,
    /// Destination token address.
    pub dst: String,
    /// Sell amount in source-token base units.
    pub amount: String,
    /// Sender wallet address (the address that will sign and execute the swap).
    pub from: String,
    /// Maximum acceptable slippage as a percent (1 = 1%, 0.5 = 0.5%). Defaults to 1.0.
    #[serde(default)]
    pub slippage: Option<f64>,
}

const NATIVE_SENTINEL: &str = "0xeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee";

fn is_native(addr: &str) -> bool {
    addr.eq_ignore_ascii_case(NATIVE_SENTINEL)
}

impl DynAomiTool for BuildSwapTx {
    type App = OneinchApp;
    type Args = BuildSwapTxArgs;
    const NAME: &'static str = "oneinch_build_swap_tx";
    const DESCRIPTION: &'static str = "Use when the user is ready to execute a 1inch swap. Composite tool: fetches a quote, checks ERC-20 allowance for the 1inch router (skipped for native asset sells), and returns the executable swap tx (`to`, `data`, `value`, `gas`). If allowance is insufficient, also returns an `approve_tx` the user must sign and submit BEFORE the swap. Stage transactions via `stage_tx` with `data: { raw }`, simulate, then `commit_tx` — do not re-encode.";

    fn run(_app: &OneinchApp, args: Self::Args, _ctx: DynToolCallCtx) -> Result<Value, String> {
        let api_key = resolve_key(args.api_key.as_deref())?;
        let chain_id = args.chain_id.unwrap_or(1);
        validate_chain(chain_id)?;
        let amount_u = parse_amount(&args.amount)?;
        let slippage = args.slippage.unwrap_or(1.0);

        let client = make_client(&api_key)?;
        let runtime = rt()?;
        let src = args.src.clone();
        let dst = args.dst.clone();
        let amount = args.amount.clone();
        let from = args.from.clone();

        let result = runtime
            .block_on(async move {
                // 1. Quote.
                let quote = client
                    .get_quote(chain_id, amount.as_str(), dst.as_str(), None, src.as_str())
                    .await
                    .map_err(|e| format!("[1inch] quote step: {e}"))?
                    .into_inner();

                // 2. Allowance + optional approve_tx (skip for native sells).
                let mut approve_tx: Option<Value> = None;
                let mut allowance_value: Option<String> = None;
                if !is_native(&src) {
                    let allow = client
                        .get_allowance(chain_id, src.as_str(), from.as_str())
                        .await
                        .map_err(|e| format!("[1inch] allowance step: {e}"))?
                        .into_inner();
                    let current = allow.allowance.clone().unwrap_or_default();
                    allowance_value = Some(current.clone());

                    let needs_approve = match current.parse::<u128>() {
                        Ok(v) => v < amount_u,
                        Err(_) => true, // unparseable -> assume insufficient
                    };
                    if needs_approve {
                        let approve = client
                            .get_approve_transaction(chain_id, Some(amount.as_str()), src.as_str())
                            .await
                            .map_err(|e| format!("[1inch] approve_tx step: {e}"))?
                            .into_inner();
                        approve_tx = Some(serde_json::to_value(approve).map_err(|e| {
                            format!("[1inch] approve_tx serialize: {e}")
                        })?);
                    }
                }

                // 3. Swap tx.
                let swap = client
                    .get_swap(
                        chain_id,
                        amount.as_str(),
                        dst.as_str(),
                        from.as_str(),
                        None,
                        slippage,
                        src.as_str(),
                    )
                    .await
                    .map_err(|e| format!("[1inch] swap step: {e}"))?
                    .into_inner();

                Ok::<_, String>(json!({
                    "chain_id": chain_id,
                    "slippage": slippage,
                    "quote": quote,
                    "allowance": allowance_value,
                    "approve_tx": approve_tx,
                    "swap": swap,
                }))
            })?;

        ok(result)
    }
}

// ============================================================================
// Tool 3: oneinch_check_allowance
// ============================================================================

pub(crate) struct CheckAllowance;

#[derive(Debug, Deserialize, JsonSchema)]
pub(crate) struct CheckAllowanceArgs {
    /// Optional 1inch developer API key. Falls back to ONEINCH_API_KEY env var.
    #[serde(default)]
    pub api_key: Option<String>,
    /// EVM chain id. Defaults to 1 (Ethereum).
    #[serde(default)]
    pub chain_id: Option<i64>,
    /// ERC-20 token contract address.
    pub token_address: String,
    /// Wallet address whose allowance is being checked.
    pub wallet_address: String,
}

impl DynAomiTool for CheckAllowance {
    type App = OneinchApp;
    type Args = CheckAllowanceArgs;
    const NAME: &'static str = "oneinch_check_allowance";
    const DESCRIPTION: &'static str = "Use before any ERC-20 swap to confirm the wallet has granted the 1inch router enough allowance. Returns the current allowance in token base units. If less than the swap amount, build an approval with `oneinch_get_approve_tx` (or use `oneinch_build_swap_tx` which handles this for you).";

    fn run(_app: &OneinchApp, args: Self::Args, _ctx: DynToolCallCtx) -> Result<Value, String> {
        let api_key = resolve_key(args.api_key.as_deref())?;
        let chain_id = args.chain_id.unwrap_or(1);
        validate_chain(chain_id)?;

        let client = make_client(&api_key)?;
        let runtime = rt()?;
        let result = runtime
            .block_on(async move {
                client
                    .get_allowance(
                        chain_id,
                        args.token_address.as_str(),
                        args.wallet_address.as_str(),
                    )
                    .await
            })
            .map_err(|e| format!("[1inch] check_allowance: {e}"))?;
        ok(result.into_inner())
    }
}

// ============================================================================
// Tool 4: oneinch_get_approve_tx
// ============================================================================

pub(crate) struct GetApproveTx;

#[derive(Debug, Deserialize, JsonSchema)]
pub(crate) struct GetApproveTxArgs {
    /// Optional 1inch developer API key. Falls back to ONEINCH_API_KEY env var.
    #[serde(default)]
    pub api_key: Option<String>,
    /// EVM chain id. Defaults to 1 (Ethereum).
    #[serde(default)]
    pub chain_id: Option<i64>,
    /// ERC-20 token contract address to approve for the 1inch router.
    pub token_address: String,
    /// Approval amount in token base units. Omit for an unlimited approval.
    #[serde(default)]
    pub amount: Option<String>,
}

impl DynAomiTool for GetApproveTx {
    type App = OneinchApp;
    type Args = GetApproveTxArgs;
    const NAME: &'static str = "oneinch_get_approve_tx";
    const DESCRIPTION: &'static str = "Use when `oneinch_check_allowance` shows insufficient allowance. Returns a raw ERC-20 approval tx (to=token, data=approve calldata, value=0) targeting the 1inch router. Stage via `stage_tx` with `data: { raw }`; do not re-encode. Omit `amount` for unlimited approval.";

    fn run(_app: &OneinchApp, args: Self::Args, _ctx: DynToolCallCtx) -> Result<Value, String> {
        let api_key = resolve_key(args.api_key.as_deref())?;
        let chain_id = args.chain_id.unwrap_or(1);
        validate_chain(chain_id)?;

        let client = make_client(&api_key)?;
        let runtime = rt()?;
        let result = runtime
            .block_on(async move {
                client
                    .get_approve_transaction(
                        chain_id,
                        args.amount.as_deref(),
                        args.token_address.as_str(),
                    )
                    .await
            })
            .map_err(|e| format!("[1inch] get_approve_tx: {e}"))?;
        ok(result.into_inner())
    }
}

// ============================================================================
// Tool 5: oneinch_list_tokens
// ============================================================================

pub(crate) struct ListTokens;

#[derive(Debug, Deserialize, JsonSchema)]
pub(crate) struct ListTokensArgs {
    /// Optional 1inch developer API key. Falls back to ONEINCH_API_KEY env var.
    #[serde(default)]
    pub api_key: Option<String>,
    /// EVM chain id. Defaults to 1 (Ethereum).
    #[serde(default)]
    pub chain_id: Option<i64>,
}

impl DynAomiTool for ListTokens {
    type App = OneinchApp;
    type Args = ListTokensArgs;
    const NAME: &'static str = "oneinch_list_tokens";
    const DESCRIPTION: &'static str = "Use when the user asks 'what's the address of <symbol> on <chain>?' or wants to discover swappable tokens. Returns the full token map (address -> symbol/name/decimals/logo) supported by 1inch on the chain.";

    fn run(_app: &OneinchApp, args: Self::Args, _ctx: DynToolCallCtx) -> Result<Value, String> {
        let api_key = resolve_key(args.api_key.as_deref())?;
        let chain_id = args.chain_id.unwrap_or(1);
        validate_chain(chain_id)?;

        let client = make_client(&api_key)?;
        let runtime = rt()?;
        let result = runtime
            .block_on(async move { client.get_tokens(chain_id).await })
            .map_err(|e| format!("[1inch] list_tokens: {e}"))?;
        ok(result.into_inner())
    }
}
