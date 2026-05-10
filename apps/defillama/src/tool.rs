//! Curated tool layer for DefiLlama. Hand-written from the progenitor-generated
//! client at `aomi_ext::defillama::Client` — see ext/specs/defillama.yaml.
//!
//! DefiLlama splits its surface across four hosts (no auth on any of them):
//!   * api.llama.fi          — protocols, TVL, fees, dex/options/perps
//!   * coins.llama.fi        — token prices (current, historical, %)
//!   * yields.llama.fi       — yield pools and per-pool charts
//!   * stablecoins.llama.fi  — stablecoin mcap, chain distribution
//!
//! The progenitor client takes a single base URL, so each tool constructs its
//! own `Client` against the right host. 17 mechanical tools were curated down
//! to 6 user-centric tools focused on token prices, protocol/chain TVL, and
//! yields.

use aomi_ext::defillama::Client as DefiLlamaClient;
use aomi_sdk::*;
use aomi_sdk::schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

#[derive(Clone, Default)]
pub(crate) struct DefiLlamaApp;

const API_HOST: &str = "https://api.llama.fi";
const COINS_HOST: &str = "https://coins.llama.fi";
const YIELDS_HOST: &str = "https://yields.llama.fi";

// ============================================================================
// Helpers
// ============================================================================

fn ok<T: Serialize>(value: T) -> Result<Value, String> {
    let value = serde_json::to_value(value)
        .map_err(|e| format!("[defillama] serialize: {e}"))?;
    Ok(match value {
        Value::Object(mut m) => {
            m.insert("source".into(), Value::String("defillama".into()));
            Value::Object(m)
        }
        other => json!({ "source": "defillama", "data": other }),
    })
}

fn rt() -> Result<tokio::runtime::Runtime, String> {
    tokio::runtime::Runtime::new().map_err(|e| format!("[defillama] runtime: {e}"))
}

// ============================================================================
// Tool 1: get_token_price — current price for one or more tokens
// ============================================================================

pub(crate) struct GetTokenPrice;

#[derive(Debug, Deserialize, JsonSchema)]
pub(crate) struct GetTokenPriceArgs {
    /// Comma-separated coin identifiers. Use `coingecko:<id>` for well-known
    /// tokens (e.g. `coingecko:ethereum,coingecko:bitcoin`) or
    /// `<chain>:<address>` for arbitrary on-chain tokens
    /// (e.g. `ethereum:0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48` for USDC).
    pub coins: String,
    /// How far (in seconds) to look back for a tracked price if the exact
    /// timestamp has none. Optional — DefiLlama default is 4 hours.
    #[serde(default)]
    pub search_width: Option<String>,
}

impl DynAomiTool for GetTokenPrice {
    type App = DefiLlamaApp;
    type Args = GetTokenPriceArgs;
    const NAME: &'static str = "defillama_get_token_price";
    const DESCRIPTION: &'static str = "Get the current USD price of one or more tokens. Use when the user asks 'what's X worth right now'. Accepts coingecko slugs (`coingecko:bitcoin`) or on-chain identifiers (`ethereum:0x...`). Returns price, decimals, and confidence per coin.";

    fn run(_app: &DefiLlamaApp, args: Self::Args, _ctx: DynToolCallCtx) -> Result<Value, String> {
        let runtime = rt()?;
        runtime.block_on(async move {
            let client = DefiLlamaClient::new(COINS_HOST);
            let r = client
                .get_current_prices(args.coins.as_str(), args.search_width.as_deref())
                .await
                .map_err(|e| format!("[defillama] get_token_price: {e}"))?
                .into_inner();
            ok(r)
        })
    }
}

// ============================================================================
// Tool 2: get_price_history — historical price + % change since
// ============================================================================

pub(crate) struct GetPriceHistory;

#[derive(Debug, Deserialize, JsonSchema)]
pub(crate) struct GetPriceHistoryArgs {
    /// Comma-separated coin identifiers (`coingecko:bitcoin` or
    /// `ethereum:0x...`).
    pub coins: String,
    /// Past unix timestamp (seconds) to fetch the price at.
    pub timestamp: i64,
    /// Optional period for a complementary % change reading (e.g. `1d`,
    /// `7d`, `30d`). Defaults to `24h` when omitted.
    #[serde(default)]
    pub period: Option<String>,
}

impl DynAomiTool for GetPriceHistory {
    type App = DefiLlamaApp;
    type Args = GetPriceHistoryArgs;
    const NAME: &'static str = "defillama_get_price_history";
    const DESCRIPTION: &'static str = "Get a token's price at a past timestamp together with the % change over a window. Use when the user asks 'how much has X moved since N' or 'what was X worth at time T'. Combines /prices/historical and /percentage in one call.";

    fn run(_app: &DefiLlamaApp, args: Self::Args, _ctx: DynToolCallCtx) -> Result<Value, String> {
        let runtime = rt()?;
        let coins = args.coins.clone();
        let timestamp = args.timestamp;
        let period = args.period.clone();
        runtime.block_on(async move {
            let client = DefiLlamaClient::new(COINS_HOST);

            let historical = client
                .get_historical_prices(timestamp, coins.as_str(), None)
                .await
                .map_err(|e| format!("[defillama] historical price: {e}"))?
                .into_inner();

            let pct = client
                .get_price_change_percentage(
                    coins.as_str(),
                    None,
                    period.as_deref(),
                    Some(timestamp),
                )
                .await
                .map_err(|e| format!("[defillama] price change %: {e}"))?
                .into_inner();

            ok(json!({
                "at_timestamp": timestamp,
                "period": period.unwrap_or_else(|| "24h".to_string()),
                "historical": historical,
                "change_percent": pct,
            }))
        })
    }
}

// ============================================================================
// Tool 3: list_protocols — top DeFi protocols by current TVL
// ============================================================================

pub(crate) struct ListProtocols;

#[derive(Debug, Deserialize, JsonSchema)]
pub(crate) struct ListProtocolsArgs {
    /// Optional category filter (e.g. `Lending`, `Dexes`, `Liquid Staking`,
    /// `Yield`, `Bridge`, `Derivatives`).
    #[serde(default)]
    pub category: Option<String>,
    /// Max protocols returned, sorted by current TVL descending. Default 25.
    #[serde(default)]
    pub limit: Option<usize>,
}

impl DynAomiTool for ListProtocols {
    type App = DefiLlamaApp;
    type Args = ListProtocolsArgs;
    const NAME: &'static str = "defillama_list_protocols";
    const DESCRIPTION: &'static str = "List DeFi protocols ranked by current TVL, optionally filtered by category. Use when the user asks 'what are the biggest lending protocols' or 'show me top DEXes by TVL'. Returns name, slug, category, chains, TVL, and 1d change.";

    fn run(_app: &DefiLlamaApp, args: Self::Args, _ctx: DynToolCallCtx) -> Result<Value, String> {
        let runtime = rt()?;
        let category = args.category.clone();
        let limit = args.limit.unwrap_or(25);
        runtime.block_on(async move {
            let client = DefiLlamaClient::new(API_HOST);
            let mut protocols = client
                .get_protocols(category.as_deref())
                .await
                .map_err(|e| format!("[defillama] list protocols: {e}"))?
                .into_inner();
            protocols.sort_by(|a, b| {
                b.tvl.partial_cmp(&a.tvl).unwrap_or(std::cmp::Ordering::Equal)
            });
            protocols.truncate(limit);
            ok(json!({ "protocols": protocols }))
        })
    }
}

// ============================================================================
// Tool 4: get_protocol_tvl — current + historical TVL for one protocol
// ============================================================================

pub(crate) struct GetProtocolTvl;

#[derive(Debug, Deserialize, JsonSchema)]
pub(crate) struct GetProtocolTvlArgs {
    /// Protocol slug as used in DefiLlama URLs (e.g. `aave-v3`, `uniswap`,
    /// `lido`, `makerdao`).
    pub protocol: String,
}

impl DynAomiTool for GetProtocolTvl {
    type App = DefiLlamaApp;
    type Args = GetProtocolTvlArgs;
    const NAME: &'static str = "defillama_get_protocol_tvl";
    const DESCRIPTION: &'static str = "Deep-dive into a single protocol: current TVL plus the historical TVL series and per-chain breakdown. Use when the user names a specific protocol and wants its size or trend. Combines /tvl/{slug} and /protocol/{slug}.";

    fn run(_app: &DefiLlamaApp, args: Self::Args, _ctx: DynToolCallCtx) -> Result<Value, String> {
        let runtime = rt()?;
        let protocol = args.protocol.clone();
        runtime.block_on(async move {
            let client = DefiLlamaClient::new(API_HOST);

            let current_tvl = client
                .get_protocol_current_tvl(protocol.as_str())
                .await
                .map_err(|e| format!("[defillama] current tvl {protocol}: {e}"))?
                .into_inner();

            let detail = client
                .get_protocol_detail(protocol.as_str())
                .await
                .map_err(|e| format!("[defillama] protocol detail {protocol}: {e}"))?
                .into_inner();

            ok(json!({
                "protocol": protocol,
                "current_tvl_usd": current_tvl,
                "detail": detail,
            }))
        })
    }
}

// ============================================================================
// Tool 5: top_yield_pools — best yield pools, filterable
// ============================================================================

pub(crate) struct TopYieldPools;

#[derive(Debug, Deserialize, JsonSchema)]
pub(crate) struct TopYieldPoolsArgs {
    /// Filter by chain slug (e.g. `Ethereum`, `Arbitrum`, `Base`, `Solana`).
    #[serde(default)]
    pub chain: Option<String>,
    /// Filter by project slug (e.g. `aave-v3`, `lido`, `compound-v3`).
    #[serde(default)]
    pub project: Option<String>,
    /// Only return pools whose underlying asset is a stablecoin.
    #[serde(default)]
    pub stablecoin_only: Option<bool>,
    /// Minimum TVL (USD) — drops dust pools. Default 1_000_000.
    #[serde(default)]
    pub min_tvl_usd: Option<f64>,
    /// Max pools returned, sorted by APY descending. Default 20.
    #[serde(default)]
    pub limit: Option<usize>,
}

impl DynAomiTool for TopYieldPools {
    type App = DefiLlamaApp;
    type Args = TopYieldPoolsArgs;
    const NAME: &'static str = "defillama_top_yield_pools";
    const DESCRIPTION: &'static str = "Find the highest-APY yield pools, filterable by chain, project, stablecoin-only, and minimum TVL. Use when the user asks 'where can I earn yield' or 'top stablecoin yields on Arbitrum'. Returns symbol, project, chain, APY, TVL, and IL/stable flags. Pool UUIDs in the result feed `defillama_get_yield_pool_history` for time-series APY.";

    fn run(_app: &DefiLlamaApp, args: Self::Args, _ctx: DynToolCallCtx) -> Result<Value, String> {
        let runtime = rt()?;
        let chain = args.chain.clone();
        let project = args.project.clone();
        let stables_only = args.stablecoin_only.unwrap_or(false);
        let min_tvl = args.min_tvl_usd.unwrap_or(1_000_000.0);
        let limit = args.limit.unwrap_or(20);
        runtime.block_on(async move {
            let client = DefiLlamaClient::new(YIELDS_HOST);
            let resp = client
                .get_yield_pools(chain.as_deref(), project.as_deref())
                .await
                .map_err(|e| format!("[defillama] yield pools: {e}"))?
                .into_inner();

            let mut pools: Vec<Value> = resp
                .get("data")
                .and_then(Value::as_array)
                .cloned()
                .unwrap_or_default();

            pools.retain(|p: &Value| {
                let tvl = p.get("tvlUsd").and_then(Value::as_f64).unwrap_or(0.0);
                if tvl < min_tvl {
                    return false;
                }
                if stables_only
                    && !p.get("stablecoin").and_then(Value::as_bool).unwrap_or(false)
                {
                    return false;
                }
                true
            });

            pools.sort_by(|a: &Value, b: &Value| {
                let aa = a.get("apy").and_then(Value::as_f64).unwrap_or(0.0);
                let bb = b.get("apy").and_then(Value::as_f64).unwrap_or(0.0);
                bb.partial_cmp(&aa).unwrap_or(std::cmp::Ordering::Equal)
            });
            pools.truncate(limit);

            let summarised: Vec<Value> = pools
                .into_iter()
                .map(|p| {
                    json!({
                        "pool": p.get("pool").cloned().unwrap_or(Value::Null),
                        "symbol": p.get("symbol").cloned().unwrap_or(Value::Null),
                        "project": p.get("project").cloned().unwrap_or(Value::Null),
                        "chain": p.get("chain").cloned().unwrap_or(Value::Null),
                        "apy": p.get("apy").cloned().unwrap_or(Value::Null),
                        "apy_base": p.get("apyBase").cloned().unwrap_or(Value::Null),
                        "apy_reward": p.get("apyReward").cloned().unwrap_or(Value::Null),
                        "tvl_usd": p.get("tvlUsd").cloned().unwrap_or(Value::Null),
                        "stablecoin": p.get("stablecoin").cloned().unwrap_or(Value::Null),
                        "il_risk": p.get("ilRisk").cloned().unwrap_or(Value::Null),
                        "exposure": p.get("exposure").cloned().unwrap_or(Value::Null),
                    })
                })
                .collect();

            ok(json!({
                "filters": {
                    "chain": chain,
                    "project": project,
                    "stablecoin_only": stables_only,
                    "min_tvl_usd": min_tvl,
                },
                "pools_found": summarised.len(),
                "pools": summarised,
            }))
        })
    }
}

// ============================================================================
// Tool 6: get_chain_tvl — current + historical TVL for a chain
// ============================================================================

pub(crate) struct GetChainTvl;

#[derive(Debug, Deserialize, JsonSchema)]
pub(crate) struct GetChainTvlArgs {
    /// Chain slug (e.g. `Ethereum`, `Arbitrum`, `Solana`, `Base`). Omit to
    /// get the ranked list of every chain by current TVL.
    #[serde(default)]
    pub chain: Option<String>,
    /// When listing all chains, max results returned. Default 25.
    #[serde(default)]
    pub limit: Option<usize>,
}

impl DynAomiTool for GetChainTvl {
    type App = DefiLlamaApp;
    type Args = GetChainTvlArgs;
    const NAME: &'static str = "defillama_get_chain_tvl";
    const DESCRIPTION: &'static str = "Get TVL data for blockchains. With no chain, returns every chain ranked by current TVL. With a chain slug, returns that chain's full historical TVL series. Use when the user asks 'which chains have the most DeFi activity' or 'show TVL trend for Solana'.";

    fn run(_app: &DefiLlamaApp, args: Self::Args, _ctx: DynToolCallCtx) -> Result<Value, String> {
        let runtime = rt()?;
        let chain = args.chain.clone();
        let limit = args.limit.unwrap_or(25);
        runtime.block_on(async move {
            let client = DefiLlamaClient::new(API_HOST);
            match chain {
                Some(name) => {
                    let series = client
                        .get_historical_chain_tvl(name.as_str())
                        .await
                        .map_err(|e| format!("[defillama] historical chain tvl {name}: {e}"))?
                        .into_inner();
                    ok(json!({ "chain": name, "history": series }))
                }
                None => {
                    let mut chains = client
                        .get_chains_tvl()
                        .await
                        .map_err(|e| format!("[defillama] chains tvl: {e}"))?
                        .into_inner();
                    chains.sort_by(|a, b| {
                        b.tvl.partial_cmp(&a.tvl).unwrap_or(std::cmp::Ordering::Equal)
                    });
                    chains.truncate(limit);
                    ok(json!({ "chains": chains }))
                }
            }
        })
    }
}
