use aomi_ext::defillama::DefiLamaClient;
use aomi_sdk::schemars::JsonSchema;
use aomi_sdk::*;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

#[derive(Clone, Default)]
pub(crate) struct DefiLlamaApp;

fn ok<T: Serialize>(value: T) -> Result<Value, String> {
    let value = serde_json::to_value(value)
        .map_err(|e| format!("[defillama] failed to serialize response: {e}"))?;
    Ok(match value {
        Value::Object(mut map) => {
            map.insert("source".to_string(), Value::String("defillama".to_string()));
            Value::Object(map)
        }
        other => json!({ "source": "defillama", "data": other }),
    })
}

// ============================================================================
// GetLammaTokenPrice
// ============================================================================

pub(crate) struct GetLammaTokenPrice;

#[derive(Debug, Deserialize, JsonSchema)]
pub(crate) struct GetLammaTokenPriceArgs {
    /// Token symbol or name (e.g., "ETH", "bitcoin", "USDC")
    pub(crate) token: String,
}

impl DynAomiTool for GetLammaTokenPrice {
    type App = DefiLlamaApp;
    type Args = GetLammaTokenPriceArgs;
    const NAME: &'static str = "get_token_price";
    const DESCRIPTION: &'static str = "Get overall token price estimation from DefiLlama (informational, not an executable trade quote).";

    fn run(_app: &DefiLlamaApp, args: Self::Args, _ctx: DynToolCallCtx) -> Result<Value, String> {
        let response = DefiLamaClient::new()?.get_token_price(&args.token)?;
        let coin = response
            .get("coins")
            .and_then(Value::as_object)
            .and_then(|coins| coins.values().next())
            .ok_or_else(|| format!("Token not found: {}", args.token))?;
        ok(json!({
            "symbol": coin.get("symbol").and_then(Value::as_str).unwrap_or("N/A"),
            "price_usd": format!("${:.2}", coin.get("price").and_then(Value::as_f64).unwrap_or(0.0)),
            "confidence": coin.get("confidence").cloned().unwrap_or(Value::Null),
        }))
    }
}

// ============================================================================
// GetLammaYieldOpportunities
// ============================================================================

pub(crate) struct GetLammaYieldOpportunities;

#[derive(Debug, Deserialize, JsonSchema)]
pub(crate) struct GetLammaYieldOpportunitiesArgs {
    /// Filter by chain (optional): ethereum, arbitrum, optimism, polygon, base, bsc, solana
    pub(crate) chain: Option<String>,
    /// Filter by project name (optional): aave, compound, lido, etc.
    pub(crate) project: Option<String>,
    /// Only show stablecoin pools
    pub(crate) stablecoin_only: Option<bool>,
    /// Maximum results (default: 20)
    pub(crate) limit: Option<u32>,
}

impl DynAomiTool for GetLammaYieldOpportunities {
    type App = DefiLlamaApp;
    type Args = GetLammaYieldOpportunitiesArgs;
    const NAME: &'static str = "get_yield_opportunities";
    const DESCRIPTION: &'static str = "Get overall yield estimation from DefiLlama and list pools sorted by APY (informational, not trade execution).";

    fn run(_app: &DefiLlamaApp, args: Self::Args, _ctx: DynToolCallCtx) -> Result<Value, String> {
        let response = DefiLamaClient::new()?
            .get_yield_pools(args.chain.as_deref(), args.project.as_deref())?;

        let mut pools = response
            .get("data")
            .and_then(Value::as_array)
            .cloned()
            .unwrap_or_default();

        if args.stablecoin_only.unwrap_or(false) {
            pools.retain(|p| {
                p.get("stablecoin")
                    .and_then(Value::as_bool)
                    .unwrap_or(false)
            });
        }

        pools.sort_by(|a, b| {
            let aa = a.get("apy").and_then(Value::as_f64).unwrap_or(0.0);
            let bb = b.get("apy").and_then(Value::as_f64).unwrap_or(0.0);
            bb.partial_cmp(&aa).unwrap_or(std::cmp::Ordering::Equal)
        });
        pools.truncate(args.limit.unwrap_or(20) as usize);

        let formatted: Vec<Value> = pools
            .into_iter()
            .map(|p| {
                let tvl_str = p
                    .get("tvlUsd")
                    .and_then(Value::as_f64)
                    .map(|t| format!("${:.0}M", t / 1_000_000.0));
                json!({
                    "pool": p.get("symbol").and_then(Value::as_str).unwrap_or("N/A"),
                    "project": p.get("project").and_then(Value::as_str).unwrap_or("N/A"),
                    "chain": p.get("chain").and_then(Value::as_str).unwrap_or("N/A"),
                    "apy": format!("{:.2}%", p.get("apy").and_then(Value::as_f64).unwrap_or(0.0)),
                    "tvl": tvl_str,
                    "stablecoin": p.get("stablecoin").cloned().unwrap_or(Value::Null),
                    "il_risk": p.get("ilRisk").cloned().unwrap_or(Value::Null),
                })
            })
            .collect();

        ok(json!({ "pools_found": formatted.len(), "pools": formatted }))
    }
}

// ============================================================================
// GetLammaProtocols
// ============================================================================

pub(crate) struct GetLammaProtocols;

#[derive(Debug, Deserialize, JsonSchema)]
pub(crate) struct GetLammaProtocolsArgs {
    /// Filter by category: dexes, lending, yield, liquid-staking, bridge, derivatives
    pub(crate) category: Option<String>,
    /// Maximum results (default: 20)
    pub(crate) limit: Option<u32>,
}

impl DynAomiTool for GetLammaProtocols {
    type App = DefiLlamaApp;
    type Args = GetLammaProtocolsArgs;
    const NAME: &'static str = "get_defi_protocols";
    const DESCRIPTION: &'static str = "Get overall protocol TVL estimation from DefiLlama (informational, not executable trading data).";

    fn run(_app: &DefiLlamaApp, args: Self::Args, _ctx: DynToolCallCtx) -> Result<Value, String> {
        let response = DefiLamaClient::new()?.get_protocols(args.category.as_deref())?;

        let protocols = response
            .as_array()
            .cloned()
            .or_else(|| response.get("data").and_then(Value::as_array).cloned())
            .unwrap_or_default();

        let formatted: Vec<Value> = protocols
            .into_iter()
            .take(args.limit.unwrap_or(20) as usize)
            .map(|p| {
                json!({
                    "name": p.get("name").and_then(Value::as_str).unwrap_or("N/A"),
                    "tvl": format!("${:.2}B", p.get("tvl").and_then(Value::as_f64).unwrap_or(0.0) / 1_000_000_000.0),
                    "category": p.get("category").cloned().unwrap_or(Value::Null),
                    "chains": p.get("chains").cloned().unwrap_or(Value::Null),
                    "change_1d": p.get("change_1d").and_then(Value::as_f64).map(|c| format!("{c:+.1}%")),
                })
            })
            .collect();

        ok(json!({ "protocols_count": formatted.len(), "protocols": formatted }))
    }
}

// ============================================================================
// GetLammaChainTvl
// ============================================================================

pub(crate) struct GetLammaChainTvl;

#[derive(Debug, Deserialize, JsonSchema)]
pub(crate) struct GetLammaChainTvlArgs {
    /// Maximum results (default: 15)
    pub(crate) limit: Option<u32>,
}

impl DynAomiTool for GetLammaChainTvl {
    type App = DefiLlamaApp;
    type Args = GetLammaChainTvlArgs;
    const NAME: &'static str = "get_chain_tvl";
    const DESCRIPTION: &'static str = "Get overall chain TVL estimation from DefiLlama (informational, not executable trading data).";

    fn run(_app: &DefiLlamaApp, args: Self::Args, _ctx: DynToolCallCtx) -> Result<Value, String> {
        let response = DefiLamaClient::new()?.get_chains_tvl()?;
        let chains = response
            .as_array()
            .cloned()
            .or_else(|| response.get("data").and_then(Value::as_array).cloned())
            .unwrap_or_default();

        let formatted: Vec<Value> = chains
            .into_iter()
            .take(args.limit.unwrap_or(15) as usize)
            .enumerate()
            .map(|(i, c)| {
                json!({
                    "rank": i + 1,
                    "chain": c.get("name").and_then(Value::as_str).unwrap_or("N/A"),
                    "tvl": format!("${:.2}B", c.get("tvl").and_then(Value::as_f64).unwrap_or(0.0) / 1_000_000_000.0),
                    "native_token": c.get("tokenSymbol").cloned().unwrap_or(Value::Null),
                })
            })
            .collect();

        ok(json!({ "chains": formatted }))
    }
}

// ============================================================================
// GetLammaProtocolDetail
// ============================================================================

pub(crate) struct GetLammaProtocolDetail;

#[derive(Debug, Clone, Deserialize, JsonSchema)]
pub(crate) struct GetLammaProtocolDetailArgs {
    #[schemars(description = "Protocol slug (e.g. \"aave\", \"uniswap\", \"lido\")")]
    pub protocol: String,
}

impl DynAomiTool for GetLammaProtocolDetail {
    type App = DefiLlamaApp;
    type Args = GetLammaProtocolDetailArgs;
    const NAME: &'static str = "get_protocol_detail";
    const DESCRIPTION: &'static str =
        "Get deep-dive data for a single protocol: historical TVL, chain breakdown, metadata.";

    fn run(_app: &DefiLlamaApp, args: Self::Args, _ctx: DynToolCallCtx) -> Result<Value, String> {
        ok(DefiLamaClient::new()?.get_protocol_detail(&args.protocol)?)
    }
}

// ============================================================================
// GetLammaDexVolumes
// ============================================================================

pub(crate) struct GetLammaDexVolumes;

#[derive(Debug, Clone, Deserialize, JsonSchema)]
pub(crate) struct GetLammaDexVolumesArgs {
    #[schemars(description = "Filter by chain name (e.g. \"Ethereum\", \"Arbitrum\"). Optional.")]
    #[serde(default)]
    pub chain: Option<String>,
    #[schemars(description = "Exclude total data chart from response (default: true)")]
    #[serde(default)]
    pub exclude_total_data_chart: Option<bool>,
    #[schemars(description = "Exclude total data chart breakdown from response (default: true)")]
    #[serde(default)]
    pub exclude_total_data_chart_breakdown: Option<bool>,
}

impl DynAomiTool for GetLammaDexVolumes {
    type App = DefiLlamaApp;
    type Args = GetLammaDexVolumesArgs;
    const NAME: &'static str = "get_dex_volumes";
    const DESCRIPTION: &'static str =
        "Get DEX volume rankings across all chains or for a specific chain.";

    fn run(_app: &DefiLlamaApp, args: Self::Args, _ctx: DynToolCallCtx) -> Result<Value, String> {
        ok(DefiLamaClient::new()?.get_dex_volumes(
            args.chain.as_deref(),
            args.exclude_total_data_chart,
            args.exclude_total_data_chart_breakdown,
        )?)
    }
}

// ============================================================================
// GetLammaFeesOverview
// ============================================================================

pub(crate) struct GetLammaFeesOverview;

#[derive(Debug, Clone, Deserialize, JsonSchema)]
pub(crate) struct GetLammaFeesOverviewArgs {
    #[schemars(description = "Filter by chain name (e.g. \"Ethereum\"). Optional.")]
    #[serde(default)]
    pub chain: Option<String>,
    #[schemars(description = "Exclude total data chart from response (default: true)")]
    #[serde(default)]
    pub exclude_total_data_chart: Option<bool>,
    #[schemars(description = "Exclude total data chart breakdown from response (default: true)")]
    #[serde(default)]
    pub exclude_total_data_chart_breakdown: Option<bool>,
    #[schemars(description = "Data type filter (e.g. \"dailyFees\", \"dailyRevenue\"). Optional.")]
    #[serde(default)]
    pub data_type: Option<String>,
}

impl DynAomiTool for GetLammaFeesOverview {
    type App = DefiLlamaApp;
    type Args = GetLammaFeesOverviewArgs;
    const NAME: &'static str = "get_fees_overview";
    const DESCRIPTION: &'static str =
        "Get protocol fee and revenue rankings across all chains or for a specific chain.";

    fn run(_app: &DefiLlamaApp, args: Self::Args, _ctx: DynToolCallCtx) -> Result<Value, String> {
        ok(DefiLamaClient::new()?.get_fees_overview(
            args.chain.as_deref(),
            args.exclude_total_data_chart,
            args.exclude_total_data_chart_breakdown,
            args.data_type.as_deref(),
        )?)
    }
}

// ============================================================================
// GetLammaProtocolFees
// ============================================================================

pub(crate) struct GetLammaProtocolFees;

#[derive(Debug, Clone, Deserialize, JsonSchema)]
pub(crate) struct GetLammaProtocolFeesArgs {
    #[schemars(description = "Protocol slug (e.g. \"aave\", \"uniswap\")")]
    pub protocol: String,
    #[schemars(description = "Data type filter (e.g. \"dailyFees\", \"dailyRevenue\"). Optional.")]
    #[serde(default)]
    pub data_type: Option<String>,
}

impl DynAomiTool for GetLammaProtocolFees {
    type App = DefiLlamaApp;
    type Args = GetLammaProtocolFeesArgs;
    const NAME: &'static str = "get_protocol_fees";
    const DESCRIPTION: &'static str = "Get fee and revenue detail for a single protocol.";

    fn run(_app: &DefiLlamaApp, args: Self::Args, _ctx: DynToolCallCtx) -> Result<Value, String> {
        ok(DefiLamaClient::new()?.get_protocol_fees(&args.protocol, args.data_type.as_deref())?)
    }
}

// ============================================================================
// GetLammaStablecoins
// ============================================================================

pub(crate) struct GetLammaStablecoins;

#[derive(Debug, Clone, Deserialize, JsonSchema)]
pub(crate) struct GetLammaStablecoinsArgs {
    #[schemars(description = "Include price data in response (default: true)")]
    #[serde(default)]
    pub include_prices: Option<bool>,
}

impl DynAomiTool for GetLammaStablecoins {
    type App = DefiLlamaApp;
    type Args = GetLammaStablecoinsArgs;
    const NAME: &'static str = "get_stablecoins";
    const DESCRIPTION: &'static str = "List all stablecoins with their circulating supply data.";

    fn run(_app: &DefiLlamaApp, args: Self::Args, _ctx: DynToolCallCtx) -> Result<Value, String> {
        ok(DefiLamaClient::new()?.get_stablecoins(args.include_prices)?)
    }
}

// ============================================================================
// GetLammaStablecoinChains
// ============================================================================

pub(crate) struct GetLammaStablecoinChains;

#[derive(Debug, Clone, Deserialize, JsonSchema)]
pub(crate) struct GetLammaStablecoinChainsArgs {}

impl DynAomiTool for GetLammaStablecoinChains {
    type App = DefiLlamaApp;
    type Args = GetLammaStablecoinChainsArgs;
    const NAME: &'static str = "get_stablecoin_chains";
    const DESCRIPTION: &'static str = "Get stablecoin market cap per chain.";

    fn run(_app: &DefiLlamaApp, _args: Self::Args, _ctx: DynToolCallCtx) -> Result<Value, String> {
        ok(DefiLamaClient::new()?.get_stablecoin_chains()?)
    }
}

// ============================================================================
// GetLammaHistoricalTokenPrice
// ============================================================================

pub(crate) struct GetLammaHistoricalTokenPrice;

#[derive(Debug, Clone, Deserialize, JsonSchema)]
pub(crate) struct GetLammaHistoricalTokenPriceArgs {
    #[schemars(
        description = "Comma-separated coin identifiers (e.g. \"coingecko:ethereum,coingecko:bitcoin\")"
    )]
    pub coins: String,
    #[schemars(description = "Start unix timestamp. Optional.")]
    #[serde(default)]
    pub start: Option<u64>,
    #[schemars(description = "End unix timestamp. Optional.")]
    #[serde(default)]
    pub end: Option<u64>,
    #[schemars(description = "Number of data points to return. Optional.")]
    #[serde(default)]
    pub span: Option<u64>,
    #[schemars(
        description = "Time period between data points (e.g. \"1d\", \"1h\", \"4h\"). Optional."
    )]
    #[serde(default)]
    pub period: Option<String>,
}

impl DynAomiTool for GetLammaHistoricalTokenPrice {
    type App = DefiLlamaApp;
    type Args = GetLammaHistoricalTokenPriceArgs;
    const NAME: &'static str = "get_historical_token_price";
    const DESCRIPTION: &'static str = "Get historical price chart for one or more tokens.";

    fn run(_app: &DefiLlamaApp, args: Self::Args, _ctx: DynToolCallCtx) -> Result<Value, String> {
        ok(DefiLamaClient::new()?.get_historical_token_price(
            &args.coins,
            args.start,
            args.end,
            args.span,
            args.period.as_deref(),
        )?)
    }
}

// ============================================================================
// GetLammaTokenPriceChange
// ============================================================================

pub(crate) struct GetLammaTokenPriceChange;

#[derive(Debug, Clone, Deserialize, JsonSchema)]
pub(crate) struct GetLammaTokenPriceChangeArgs {
    #[schemars(
        description = "Comma-separated coin identifiers (e.g. \"coingecko:ethereum,coingecko:bitcoin\")"
    )]
    pub coins: String,
    #[schemars(description = "Unix timestamp to calculate change from. Optional.")]
    #[serde(default)]
    pub timestamp: Option<u64>,
    #[schemars(
        description = "If true, calculate change looking forward from timestamp. Optional."
    )]
    #[serde(default)]
    pub look_forward: Option<bool>,
    #[schemars(description = "Period for price change (e.g. \"1d\", \"7d\", \"30d\"). Optional.")]
    #[serde(default)]
    pub period: Option<String>,
}

impl DynAomiTool for GetLammaTokenPriceChange {
    type App = DefiLlamaApp;
    type Args = GetLammaTokenPriceChangeArgs;
    const NAME: &'static str = "get_token_price_change";
    const DESCRIPTION: &'static str =
        "Get percentage price change for one or more tokens over a given period.";

    fn run(_app: &DefiLlamaApp, args: Self::Args, _ctx: DynToolCallCtx) -> Result<Value, String> {
        ok(DefiLamaClient::new()?.get_token_price_change(
            &args.coins,
            args.timestamp,
            args.look_forward,
            args.period.as_deref(),
        )?)
    }
}

// ============================================================================
// GetLammaHistoricalChainTvl
// ============================================================================

pub(crate) struct GetLammaHistoricalChainTvl;

#[derive(Debug, Clone, Deserialize, JsonSchema)]
pub(crate) struct GetLammaHistoricalChainTvlArgs {
    #[schemars(description = "Chain name (e.g. \"Ethereum\", \"Arbitrum\", \"Solana\")")]
    pub chain: String,
}

impl DynAomiTool for GetLammaHistoricalChainTvl {
    type App = DefiLlamaApp;
    type Args = GetLammaHistoricalChainTvlArgs;
    const NAME: &'static str = "get_historical_chain_tvl";
    const DESCRIPTION: &'static str = "Get daily historical TVL for a specific chain.";

    fn run(_app: &DefiLlamaApp, args: Self::Args, _ctx: DynToolCallCtx) -> Result<Value, String> {
        ok(DefiLamaClient::new()?.get_historical_chain_tvl(&args.chain)?)
    }
}

// ============================================================================
// GetLammaDexProtocolVolume
// ============================================================================

pub(crate) struct GetLammaDexProtocolVolume;

#[derive(Debug, Clone, Deserialize, JsonSchema)]
pub(crate) struct GetLammaDexProtocolVolumeArgs {
    #[schemars(description = "Protocol slug (e.g. \"uniswap\", \"curve\")")]
    pub protocol: String,
    #[schemars(description = "Exclude total data chart from response (default: true)")]
    #[serde(default)]
    pub exclude_total_data_chart: Option<bool>,
    #[schemars(description = "Exclude total data chart breakdown from response (default: true)")]
    #[serde(default)]
    pub exclude_total_data_chart_breakdown: Option<bool>,
}

impl DynAomiTool for GetLammaDexProtocolVolume {
    type App = DefiLlamaApp;
    type Args = GetLammaDexProtocolVolumeArgs;
    const NAME: &'static str = "get_dex_protocol_volume";
    const DESCRIPTION: &'static str = "Get volume detail for a single DEX protocol.";

    fn run(_app: &DefiLlamaApp, args: Self::Args, _ctx: DynToolCallCtx) -> Result<Value, String> {
        ok(DefiLamaClient::new()?.get_dex_protocol_volume(
            &args.protocol,
            args.exclude_total_data_chart,
            args.exclude_total_data_chart_breakdown,
        )?)
    }
}

// ============================================================================
// GetLammaStablecoinHistory
// ============================================================================

pub(crate) struct GetLammaStablecoinHistory;

#[derive(Debug, Clone, Deserialize, JsonSchema)]
pub(crate) struct GetLammaStablecoinHistoryArgs {
    #[schemars(
        description = "Chain name to filter (e.g. \"Ethereum\"). Optional -- omit for all chains."
    )]
    #[serde(default)]
    pub chain: Option<String>,
    #[schemars(description = "Stablecoin ID filter. Optional.")]
    #[serde(default)]
    pub stablecoin: Option<u64>,
}

impl DynAomiTool for GetLammaStablecoinHistory {
    type App = DefiLlamaApp;
    type Args = GetLammaStablecoinHistoryArgs;
    const NAME: &'static str = "get_stablecoin_history";
    const DESCRIPTION: &'static str =
        "Get historical stablecoin market cap data, optionally filtered by chain or stablecoin ID.";

    fn run(_app: &DefiLlamaApp, args: Self::Args, _ctx: DynToolCallCtx) -> Result<Value, String> {
        ok(
            DefiLamaClient::new()?
                .get_stablecoin_history(args.chain.as_deref(), args.stablecoin)?,
        )
    }
}

// ============================================================================
// GetLammaYieldPoolHistory
// ============================================================================

pub(crate) struct GetLammaYieldPoolHistory;

#[derive(Debug, Clone, Deserialize, JsonSchema)]
pub(crate) struct GetLammaYieldPoolHistoryArgs {
    #[schemars(description = "Pool UUID (e.g. from get_yield_opportunities results)")]
    pub pool: String,
}

impl DynAomiTool for GetLammaYieldPoolHistory {
    type App = DefiLlamaApp;
    type Args = GetLammaYieldPoolHistoryArgs;
    const NAME: &'static str = "get_yield_pool_history";
    const DESCRIPTION: &'static str = "Get historical APY and TVL data for a specific yield pool.";

    fn run(_app: &DefiLlamaApp, args: Self::Args, _ctx: DynToolCallCtx) -> Result<Value, String> {
        ok(DefiLamaClient::new()?.get_yield_pool_history(&args.pool)?)
    }
}
