//! Curated tool layer for 0x Swap API v2. Wraps the generated progenitor
//! client in `aomi_ext::zerox` (see `ext/specs/zerox.yaml`).
//!
//! Designed for the user story: "swap on EVM via 0x — quote, execute on-chain
//! through AllowanceHolder, or sign EIP-712 for a gasless relayer trade."
//!
//! Tool surface (5 user-centric tools):
//!   * `zerox_get_price`            — AllowanceHolder price quote (no signing)
//!   * `zerox_build_swap`           — AllowanceHolder firm quote + executable tx
//!   * `zerox_get_gasless_quote`    — gasless quote with EIP-712 typed data
//!   * `zerox_submit_gasless_swap`  — submit signed gasless trade to relayer
//!   * `zerox_get_gasless_status`   — poll a submitted gasless trade

use aomi_ext::zerox::Client as GenClient;
use aomi_ext::zerox::types::{GaslessSubmitRequest, SwapQuote};
use aomi_sdk::*;
use aomi_sdk::schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::time::Duration;

#[derive(Clone, Default)]
pub(crate) struct ZeroxApp;

const BASE_URL: &str = "https://api.0x.org";

// ============================================================================
// Helpers
// ============================================================================

fn ok<T: Serialize>(value: T) -> Result<Value, String> {
    let value = serde_json::to_value(value)
        .map_err(|e| format!("[0x] failed to serialize response: {e}"))?;
    Ok(match value {
        Value::Object(mut map) => {
            map.insert("source".to_string(), Value::String("0x".to_string()));
            Value::Object(map)
        }
        other => json!({ "source": "0x", "data": other }),
    })
}

fn rt() -> Result<tokio::runtime::Runtime, String> {
    tokio::runtime::Runtime::new().map_err(|e| format!("[0x] runtime: {e}"))
}

fn resolve_key(arg: Option<&str>) -> Result<String, String> {
    resolve_secret_value(
        arg,
        "ZEROX_API_KEY",
        "[0x] missing api_key argument and ZEROX_API_KEY environment variable",
    )
}

/// Build a generated client carrying the `0x-api-key` and `0x-version: v2`
/// headers that production 0x demands.
fn make_client(api_key: &str) -> Result<GenClient, String> {
    let mut headers = reqwest::header::HeaderMap::new();
    let mut key = reqwest::header::HeaderValue::from_str(api_key)
        .map_err(|e| format!("[0x] invalid api_key: {e}"))?;
    key.set_sensitive(true);
    headers.insert(
        reqwest::header::HeaderName::from_static("0x-api-key"),
        key,
    );
    headers.insert(
        reqwest::header::HeaderName::from_static("0x-version"),
        reqwest::header::HeaderValue::from_static("v2"),
    );

    let endpoint =
        std::env::var("ZEROX_API_ENDPOINT").unwrap_or_else(|_| BASE_URL.to_string());
    let http = reqwest::ClientBuilder::new()
        .timeout(Duration::from_secs(30))
        .default_headers(headers)
        .build()
        .map_err(|e| format!("[0x] failed to build HTTP client: {e}"))?;
    Ok(GenClient::new_with_client(&endpoint, http))
}

// ============================================================================
// Token / chain shorthand resolution (preserved from the handwritten client)
// ============================================================================

fn get_chain_info(chain: &str) -> Result<(&'static str, i64), String> {
    match chain.to_lowercase().as_str() {
        "ethereum" | "eth" | "mainnet" => Ok(("ethereum", 1)),
        "polygon" | "matic" => Ok(("polygon", 137)),
        "arbitrum" | "arb" => Ok(("arbitrum", 42161)),
        "optimism" | "op" => Ok(("optimism", 10)),
        "base" => Ok(("base", 8453)),
        "bsc" | "binance" => Ok(("bsc", 56)),
        "avalanche" | "avax" => Ok(("avalanche", 43114)),
        _ => Err(format!("[0x] unsupported chain: {chain}")),
    }
}

fn is_hex_address(token: &str) -> bool {
    token.len() == 42
        && token.starts_with("0x")
        && token[2..].chars().all(|c| c.is_ascii_hexdigit())
}

fn get_token_address(chain: &str, token: &str) -> Result<String, String> {
    let native = "0xEeeeeEeeeEeEeeEeEeEeeEEEeeeeEeeeeeeeEEeE";
    let token_lower = token.to_lowercase();
    if token_lower == native.to_lowercase() {
        return Ok(native.to_string());
    }
    if is_hex_address(token) {
        return Ok(token.to_string());
    }
    match (chain, token_lower.as_str()) {
        (_, "eth") | (_, "matic") | (_, "bnb") | (_, "avax") => Ok(native.to_string()),
        ("ethereum", "usdc") => Ok("0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48".to_string()),
        ("ethereum", "usdt") => Ok("0xdAC17F958D2ee523a2206206994597C13D831ec7".to_string()),
        ("ethereum", "dai") => Ok("0x6B175474E89094C44Da98b954EedeAC495271d0F".to_string()),
        ("ethereum", "weth") => Ok("0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2".to_string()),
        ("ethereum", "wbtc") => Ok("0x2260FAC5E5542a773Aa44fBCfeDf7C193bc2C599".to_string()),
        ("ethereum", "uni") => Ok("0x1f9840a85d5aF5bf1D1762F925BDADdC4201F984".to_string()),
        ("ethereum", "aave") => Ok("0x7Fc66500c84A76Ad7e9c93437bFc5Ac33E2DdAE9".to_string()),
        ("ethereum", "link") => Ok("0x514910771AF9Ca656af840dff83E8264EcF986CA".to_string()),
        ("ethereum", "mkr") => Ok("0x9f8F72aA9304c8B593d555F12ef6589cC3A579A2".to_string()),
        ("ethereum", "crv") => Ok("0xD533a949740bb3306d119CC777fa900bA034cd52".to_string()),
        ("ethereum", "ldo") => Ok("0x5A98FcBEA516Cf06857215779Fd812CA3beF1B32".to_string()),
        ("arbitrum", "usdc") => Ok("0xaf88d065e77c8cC2239327C5EDb3A432268e5831".to_string()),
        ("arbitrum", "usdt") => Ok("0xFd086bC7CD5C481DCC9C85ebE478A1C0b69FCbb9".to_string()),
        ("arbitrum", "weth") => Ok("0x82aF49447D8a07e3bd95BD0d56f35241523fBab1".to_string()),
        ("base", "usdc") => Ok("0x833589fCD6eDb6E08f4c7C32D4f71b54bdA02913".to_string()),
        ("base", "weth") => Ok("0x4200000000000000000000000000000000000006".to_string()),
        ("polygon", "usdc") => Ok("0x3c499c542cEF5E3811e1192ce70d8cC03d5c3359".to_string()),
        ("polygon", "usdt") => Ok("0xc2132D05D31c914a87C6611C10748AEb04B58e8F".to_string()),
        ("polygon", "weth") => Ok("0x7ceB23fD6bC0adD59E62ac25578270cFf1b9f619".to_string()),
        _ => Err(format!("[0x] unknown token {token} on chain {chain}")),
    }
}

fn get_token_decimals(chain: &str, token: &str) -> u8 {
    let token_lower = token.to_lowercase();
    if is_hex_address(token) {
        return match (chain, token_lower.as_str()) {
            ("ethereum", "0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48") => 6,
            ("ethereum", "0xdac17f958d2ee523a2206206994597c13d831ec7") => 6,
            ("arbitrum", "0xaf88d065e77c8cc2239327c5edb3a432268e5831") => 6,
            ("arbitrum", "0xfd086bc7cd5c481dcc9c85ebe478a1c0b69fcbb9") => 6,
            ("polygon", "0x3c499c542cef5e3811e1192ce70d8cc03d5c3359") => 6,
            ("polygon", "0xc2132d05d31c914a87c6611c10748aeb04b58e8f") => 6,
            ("base", "0x833589fcd6edb6e08f4c7c32d4f71b54bda02913") => 6,
            ("ethereum", "0x2260fac5e5542a773aa44fbcfedf7c193bc2c599") => 8,
            _ => 18,
        };
    }
    match token_lower.as_str() {
        "usdc" | "usdt" => 6,
        "wbtc" => 8,
        _ => 18,
    }
}

fn amount_to_base_units(amount: f64, decimals: u8) -> Result<String, String> {
    if !amount.is_finite() || amount < 0.0 {
        return Err("[0x] amount must be a finite non-negative number".to_string());
    }
    let scaled = amount * 10f64.powi(decimals as i32);
    if scaled > (u128::MAX as f64) {
        return Err("[0x] amount is too large to convert to base units".to_string());
    }
    Ok((scaled.round() as u128).to_string())
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
        let api_key = resolve_key(args.api_key.as_deref())?;
        let (chain_name, chain_id) = get_chain_info(&args.chain)?;
        let sell_addr = get_token_address(chain_name, &args.sell_token)?;
        let buy_addr = get_token_address(chain_name, &args.buy_token)?;
        let decimals = get_token_decimals(chain_name, &args.sell_token);
        let amount_wei = amount_to_base_units(args.amount, decimals)?;
        let slippage = args.slippage.unwrap_or(0.01);
        let taker = args.sender_address.clone();

        let client = make_client(&api_key)?;
        let runtime = rt()?;
        runtime.block_on(async move {
            let resp = client
                .get_allowance_holder_price(
                    &buy_addr,
                    chain_id,
                    &amount_wei,
                    &sell_addr,
                    Some(slippage),
                    taker.as_deref(),
                )
                .await
                .map_err(|e| format!("[0x] get_allowance_holder_price: {e}"))?
                .into_inner();
            ok::<SwapQuote>(resp)
        })
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
        let api_key = resolve_key(args.api_key.as_deref())?;
        let (chain_name, chain_id) = get_chain_info(&args.chain)?;
        let sell_addr = get_token_address(chain_name, &args.sell_token)?;
        let buy_addr = get_token_address(chain_name, &args.buy_token)?;
        let decimals = get_token_decimals(chain_name, &args.sell_token);
        let amount_wei = amount_to_base_units(args.amount, decimals)?;
        let slippage = args.slippage.unwrap_or(0.01);
        let taker = args.sender_address.clone();

        let client = make_client(&api_key)?;
        let runtime = rt()?;
        runtime.block_on(async move {
            let resp = client
                .get_allowance_holder_quote(
                    &buy_addr,
                    chain_id,
                    &amount_wei,
                    &sell_addr,
                    Some(slippage),
                    Some(&taker),
                )
                .await
                .map_err(|e| format!("[0x] get_allowance_holder_quote: {e}"))?
                .into_inner();
            ok::<SwapQuote>(resp)
        })
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
        let api_key = resolve_key(args.api_key.as_deref())?;
        let (chain_name, chain_id) = get_chain_info(&args.chain)?;
        let sell_addr = get_token_address(chain_name, &args.sell_token)?;
        let buy_addr = get_token_address(chain_name, &args.buy_token)?;
        let decimals = get_token_decimals(chain_name, &args.sell_token);
        let amount_wei = amount_to_base_units(args.amount, decimals)?;
        let slippage = args.slippage.unwrap_or(0.01);
        let taker = args.sender_address.clone();

        let client = make_client(&api_key)?;
        let runtime = rt()?;
        runtime.block_on(async move {
            let resp = client
                .get_gasless_quote(
                    &buy_addr,
                    chain_id,
                    &amount_wei,
                    &sell_addr,
                    Some(slippage),
                    Some(&taker),
                )
                .await
                .map_err(|e| format!("[0x] get_gasless_quote: {e}"))?
                .into_inner();
            ok::<SwapQuote>(resp)
        })
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
        let api_key = resolve_key(args.api_key.as_deref())?;
        let trade_map = match args.trade {
            Value::Object(m) => m,
            _ => return Err("[0x] `trade` must be a JSON object".to_string()),
        };
        let approval_map = match args.approval {
            Some(Value::Object(m)) => m,
            Some(_) => return Err("[0x] `approval` must be a JSON object when provided".to_string()),
            None => serde_json::Map::new(),
        };
        let body = GaslessSubmitRequest {
            chain_id: args.chain_id as i64,
            trade: trade_map,
            approval: approval_map,
        };
        let client = make_client(&api_key)?;
        let runtime = rt()?;
        runtime.block_on(async move {
            let resp = client
                .submit_gasless(&body)
                .await
                .map_err(|e| format!("[0x] submit_gasless: {e}"))?
                .into_inner();
            ok::<SwapQuote>(resp)
        })
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
        let api_key = resolve_key(args.api_key.as_deref())?;
        let client = make_client(&api_key)?;
        let runtime = rt()?;
        let trade_hash = args.trade_hash.clone();
        let chain_id = args.chain_id as i64;
        runtime.block_on(async move {
            let resp = client
                .get_gasless_status(&trade_hash, chain_id)
                .await
                .map_err(|e| format!("[0x] get_gasless_status: {e}"))?
                .into_inner();
            ok::<SwapQuote>(resp)
        })
    }
}
