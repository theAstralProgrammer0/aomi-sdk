use aomi_ext::yearn::YearnClient;
use aomi_sdk::*;
use aomi_sdk::schemars::JsonSchema;
use serde::Deserialize;
use serde_json::{Value, json};

#[derive(Clone, Default)]
pub(crate) struct YearnApp;

fn default_chain_id() -> u64 {
    1
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
    pub chain_id: u64,
}

impl DynAomiTool for ListVaults {
    type App = YearnApp;
    type Args = ListVaultsArgs;
    const NAME: &'static str = "yearn_list_vaults";
    const DESCRIPTION: &'static str = "List every Yearn vault on a chain with TVL, APY, strategies, and fees. Use when the user wants the full catalogue. For a ranked best-yield shortlist, prefer `yearn_top_vaults_by_apy`.";

    fn run(_app: &YearnApp, args: Self::Args, _ctx: DynToolCallCtx) -> Result<Value, String> {
        YearnClient::new()?.get_all_vaults(args.chain_id)
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
    pub chain_id: u64,
    /// Vault contract address (0x-prefixed).
    pub address: String,
}

impl DynAomiTool for GetVaultDetail {
    type App = YearnApp;
    type Args = GetVaultDetailArgs;
    const NAME: &'static str = "yearn_get_vault_detail";
    const DESCRIPTION: &'static str = "Deep-dive a single Yearn vault by address: APY breakdown (net, gross, weekly, monthly, inception), strategies allocated to it, fees, and TVL. Use when the user has a specific vault in mind.";

    fn run(_app: &YearnApp, args: Self::Args, _ctx: DynToolCallCtx) -> Result<Value, String> {
        YearnClient::new()?.get_vault_detail(args.chain_id, &args.address)
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
    pub chain_id: u64,
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

fn vault_summary(v: &Value) -> Value {
    let net_apy = v
        .get("apr")
        .and_then(|a| a.get("netAPR"))
        .and_then(Value::as_f64)
        .or_else(|| {
            v.get("apr")
                .and_then(|a| a.get("forwardAPR"))
                .and_then(|f| f.get("netAPR"))
                .and_then(Value::as_f64)
        });
    let tvl_usd = v
        .get("tvl")
        .and_then(|t| t.get("tvl"))
        .and_then(Value::as_f64);
    json!({
        "address": v.get("address").cloned().unwrap_or(Value::Null),
        "name": v.get("name").cloned().unwrap_or(Value::Null),
        "symbol": v.get("symbol").cloned().unwrap_or(Value::Null),
        "token": v.get("token").and_then(|t| t.get("symbol")).cloned().unwrap_or(Value::Null),
        "net_apy": net_apy,
        "tvl_usd": tvl_usd,
        "version": v.get("version").cloned().unwrap_or(Value::Null),
        "kind": v.get("kind").cloned().unwrap_or(Value::Null),
    })
}

impl DynAomiTool for TopVaultsByApy {
    type App = YearnApp;
    type Args = TopVaultsByApyArgs;
    const NAME: &'static str = "yearn_top_vaults_by_apy";
    const DESCRIPTION: &'static str = "Top Yearn vaults on a chain ranked by net APY, optionally filtered by underlying token symbol and minimum TVL. Use when the user asks 'best Yearn vault for USDC' or 'highest yield on Yearn Arbitrum'. Returns a compact summary (address, symbol, net APY, TVL) — pass the address to `yearn_get_vault_detail` for the full breakdown.";

    fn run(_app: &YearnApp, args: Self::Args, _ctx: DynToolCallCtx) -> Result<Value, String> {
        let raw = YearnClient::new()?.get_all_vaults(args.chain_id)?;
        let vaults = raw
            .get("data")
            .and_then(Value::as_array)
            .cloned()
            .or_else(|| raw.as_array().cloned())
            .unwrap_or_default();

        let token_filter = args.token_symbol.as_deref().map(|s| s.to_lowercase());
        let min_tvl = args.min_tvl_usd.unwrap_or(100_000.0);
        let limit = args.limit.unwrap_or(15);

        let mut filtered: Vec<Value> = vaults
            .into_iter()
            .map(|v| vault_summary(&v))
            .filter(|s| {
                if s.get("net_apy").and_then(Value::as_f64).is_none() {
                    return false;
                }
                if s.get("tvl_usd").and_then(Value::as_f64).unwrap_or(0.0) < min_tvl {
                    return false;
                }
                if let Some(ref filt) = token_filter {
                    let sym = s
                        .get("token")
                        .and_then(Value::as_str)
                        .unwrap_or("")
                        .to_lowercase();
                    if !sym.contains(filt) {
                        return false;
                    }
                }
                true
            })
            .collect();

        filtered.sort_by(|a, b| {
            let aa = a.get("net_apy").and_then(Value::as_f64).unwrap_or(0.0);
            let bb = b.get("net_apy").and_then(Value::as_f64).unwrap_or(0.0);
            bb.partial_cmp(&aa).unwrap_or(std::cmp::Ordering::Equal)
        });
        filtered.truncate(limit);

        Ok(json!({
            "source": "yearn",
            "chain_id": args.chain_id,
            "filters": {
                "token_symbol": args.token_symbol,
                "min_tvl_usd": min_tvl,
            },
            "vaults_found": filtered.len(),
            "vaults": filtered,
        }))
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
        YearnClient::new()?.get_blacklisted_vaults()
    }
}
