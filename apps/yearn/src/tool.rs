//! Curated tool layer for Yearn yDaemon. Hand-written from the
//! progenitor-generated client at `aomi_ext::yearn::Client` — see
//! ext/specs/yearn.yaml for the surface.
//!
//! No auth; the client just takes a base URL.

use aomi_ext::yearn::Client as YearnClient;
use aomi_ext::yearn::types::YearnVault;
use aomi_sdk::schemars::JsonSchema;
use aomi_sdk::*;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

#[derive(Clone, Default)]
pub(crate) struct YearnApp;

const BASE_URL: &str = "https://ydaemon.yearn.fi";

fn default_chain_id() -> i64 {
    1
}

// ============================================================================
// Helpers
// ============================================================================

fn ok<T: Serialize>(value: T) -> Result<Value, String> {
    let value = serde_json::to_value(value).map_err(|e| format!("[yearn] serialize: {e}"))?;
    Ok(match value {
        Value::Object(mut m) => {
            m.insert("source".into(), Value::String("yearn".into()));
            Value::Object(m)
        }
        other => json!({ "source": "yearn", "data": other }),
    })
}

fn rt() -> Result<tokio::runtime::Runtime, String> {
    tokio::runtime::Runtime::new().map_err(|e| format!("[yearn] runtime: {e}"))
}

fn client() -> YearnClient {
    let base = std::env::var("YEARN_API_ENDPOINT").unwrap_or_else(|_| BASE_URL.to_string());
    YearnClient::new(&base)
}

/// Pick the realised net APY when present, falling back to the forward APY.
fn vault_net_apy(v: &YearnVault) -> Option<f64> {
    let apr = v.apr.as_ref()?;
    apr.net_apr
        .or_else(|| apr.forward_apr.as_ref().and_then(|f| f.net_apr))
}

fn vault_tvl_usd(v: &YearnVault) -> Option<f64> {
    v.tvl.as_ref().and_then(|t| t.tvl)
}

fn vault_token_symbol(v: &YearnVault) -> Option<&str> {
    v.token.as_ref().and_then(|t| t.symbol.as_deref())
}

// ============================================================================
// yearn_list_vaults — every vault on a chain
// ============================================================================

pub(crate) struct ListVaults;

#[derive(Debug, Deserialize, JsonSchema)]
pub(crate) struct ListVaultsArgs {
    /// Chain ID. Supported: 1 (Ethereum), 10 (Optimism), 137 (Polygon),
    /// 250 (Fantom), 8453 (Base), 42161 (Arbitrum). Default: 1.
    #[serde(default = "default_chain_id")]
    pub chain_id: i64,
}

impl DynAomiTool for ListVaults {
    type App = YearnApp;
    type Args = ListVaultsArgs;
    const NAME: &'static str = "yearn_list_vaults";
    const DESCRIPTION: &'static str = "List every Yearn vault on a chain with TVL, APY, strategies, and fees. Use when the user wants the full catalogue. For a ranked best-yield shortlist, prefer `yearn_top_vaults_by_apy`.";

    fn run(_app: &YearnApp, args: Self::Args, _ctx: DynToolCallCtx) -> Result<Value, String> {
        let runtime = rt()?;
        runtime.block_on(async move {
            let vaults = client()
                .get_all_vaults(args.chain_id)
                .await
                .map_err(|e| format!("[yearn] get_all_vaults: {e}"))?
                .into_inner();
            ok(json!({ "chain_id": args.chain_id, "data": vaults }))
        })
    }
}

// ============================================================================
// yearn_get_vault_detail — deep-dive on one vault by address
// ============================================================================

pub(crate) struct GetVaultDetail;

#[derive(Debug, Deserialize, JsonSchema)]
pub(crate) struct GetVaultDetailArgs {
    /// Chain ID. Supported: 1, 10, 137, 250, 8453, 42161. Default: 1.
    #[serde(default = "default_chain_id")]
    pub chain_id: i64,
    /// Vault contract address (0x-prefixed).
    pub address: String,
}

impl DynAomiTool for GetVaultDetail {
    type App = YearnApp;
    type Args = GetVaultDetailArgs;
    const NAME: &'static str = "yearn_get_vault_detail";
    const DESCRIPTION: &'static str = "Deep-dive a single Yearn vault by address: APY breakdown (net, gross, weekly, monthly, inception), strategies allocated to it, fees, and TVL. Use when the user has a specific vault in mind.";

    fn run(_app: &YearnApp, args: Self::Args, _ctx: DynToolCallCtx) -> Result<Value, String> {
        let runtime = rt()?;
        runtime.block_on(async move {
            let detail = client()
                .get_vault_detail(args.chain_id, args.address.as_str())
                .await
                .map_err(|e| {
                    format!(
                        "[yearn] get_vault_detail {} {}: {e}",
                        args.chain_id, args.address
                    )
                })?
                .into_inner();
            ok(json!({ "chain_id": args.chain_id, "data": detail }))
        })
    }
}

// ============================================================================
// yearn_top_vaults_by_apy — composite: filter and rank vaults
// ============================================================================

pub(crate) struct TopVaultsByApy;

#[derive(Debug, Deserialize, JsonSchema)]
pub(crate) struct TopVaultsByApyArgs {
    /// Chain ID. Supported: 1, 10, 137, 250, 8453, 42161. Default: 1.
    #[serde(default = "default_chain_id")]
    pub chain_id: i64,
    /// Filter by underlying token symbol (e.g. `USDC`, `WETH`, `DAI`).
    /// Case-insensitive substring match.
    #[serde(default)]
    pub token_symbol: Option<String>,
    /// Minimum TVL (USD) to include. Default 100_000.
    #[serde(default)]
    pub min_tvl_usd: Option<f64>,
    /// Max vaults returned, sorted by net APY descending. Default 15.
    #[serde(default)]
    pub limit: Option<usize>,
}

impl DynAomiTool for TopVaultsByApy {
    type App = YearnApp;
    type Args = TopVaultsByApyArgs;
    const NAME: &'static str = "yearn_top_vaults_by_apy";
    const DESCRIPTION: &'static str = "Top Yearn vaults on a chain ranked by net APY, optionally filtered by underlying token symbol and minimum TVL. Use when the user asks 'best Yearn vault for USDC' or 'highest yield on Yearn Arbitrum'. Returns a compact summary (address, symbol, net APY, TVL) — pass the address to `yearn_get_vault_detail` for the full breakdown.";

    fn run(_app: &YearnApp, args: Self::Args, _ctx: DynToolCallCtx) -> Result<Value, String> {
        let runtime = rt()?;
        let token_filter = args.token_symbol.as_ref().map(|s| s.to_lowercase());
        let min_tvl = args.min_tvl_usd.unwrap_or(100_000.0);
        let limit = args.limit.unwrap_or(15);
        let chain_id = args.chain_id;
        let token_symbol_orig = args.token_symbol.clone();
        runtime.block_on(async move {
            let raw = client()
                .get_all_vaults(chain_id)
                .await
                .map_err(|e| format!("[yearn] get_all_vaults: {e}"))?
                .into_inner();

            let mut filtered: Vec<&YearnVault> = raw
                .iter()
                .filter(|v| {
                    if vault_net_apy(v).is_none() {
                        return false;
                    }
                    if vault_tvl_usd(v).unwrap_or(0.0) < min_tvl {
                        return false;
                    }
                    if let Some(ref filt) = token_filter {
                        let sym = vault_token_symbol(v).unwrap_or("").to_lowercase();
                        if !sym.contains(filt) {
                            return false;
                        }
                    }
                    true
                })
                .collect();

            filtered.sort_by(|a, b| {
                let aa = vault_net_apy(a).unwrap_or(0.0);
                let bb = vault_net_apy(b).unwrap_or(0.0);
                bb.partial_cmp(&aa).unwrap_or(std::cmp::Ordering::Equal)
            });
            filtered.truncate(limit);

            let vaults: Vec<&YearnVault> = filtered;
            Ok(json!({
                "source": "yearn",
                "chain_id": chain_id,
                "filters": {
                    "token_symbol": token_symbol_orig,
                    "min_tvl_usd": min_tvl,
                },
                "vaults_found": vaults.len(),
                "vaults": vaults,
            }))
        })
    }
}

// ============================================================================
// yearn_blacklisted_vaults — blacklist check
// ============================================================================

pub(crate) struct GetBlacklistedVaults;

#[derive(Debug, Deserialize, JsonSchema)]
pub(crate) struct GetBlacklistedVaultsArgs {}

impl DynAomiTool for GetBlacklistedVaults {
    type App = YearnApp;
    type Args = GetBlacklistedVaultsArgs;
    const NAME: &'static str = "yearn_blacklisted_vaults";
    const DESCRIPTION: &'static str = "Get the cross-chain list of vaults Yearn has blacklisted (deprecated, exploited, or otherwise removed from the official UI). Use when the user wants to verify that a vault address is still in good standing.";

    fn run(_app: &YearnApp, _args: Self::Args, _ctx: DynToolCallCtx) -> Result<Value, String> {
        let runtime = rt()?;
        runtime.block_on(async move {
            let list = client()
                .get_blacklisted_vaults()
                .await
                .map_err(|e| format!("[yearn] get_blacklisted_vaults: {e}"))?
                .into_inner();
            ok(json!({ "data": list }))
        })
    }
}
