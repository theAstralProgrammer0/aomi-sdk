//! Curated tool layer for Zora creator/content coins on Base. Backed by the
//! progenitor-generated client at `crate::client` — see
//! `apps/zora/openapi.yaml` for the underlying surface.
//!
//! 6 curated read tools focused on discovery + analytics. Coin deployment +
//! quoted swaps (POST /quote, Uniswap V4 hook setup) are intentionally NOT
//! exposed — those need typed bodies + signing.

use crate::client::Client as ZoraClient;
use aomi_sdk::*;
use aomi_sdk::schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

#[derive(Clone, Default)]
pub(crate) struct ZoraApp;

const BASE_URL: &str = "https://api-sdk.zora.engineering";

fn ok<T: Serialize>(value: T) -> Result<Value, String> {
    let value = serde_json::to_value(value).map_err(|e| format!("[zora] serialize: {e}"))?;
    Ok(match value {
        Value::Object(mut m) => {
            m.insert("source".into(), Value::String("zora".into()));
            Value::Object(m)
        }
        other => json!({ "source": "zora", "data": other }),
    })
}

fn rt() -> Result<tokio::runtime::Runtime, String> {
    tokio::runtime::Runtime::new().map_err(|e| format!("[zora] runtime: {e}"))
}

/// Build a reqwest client; injects the optional Zora API key in `api-key`
/// header. Many endpoints work without auth (rate-limited); supplying a key
/// lifts the limit.
fn make_client(api_key: Option<&str>) -> Result<ZoraClient, String> {
    let key = api_key.map(str::to_string).or_else(|| std::env::var("ZORA_API_KEY").ok());
    if let Some(k) = key {
        let mut headers = reqwest::header::HeaderMap::new();
        let v = reqwest::header::HeaderValue::from_str(&k)
            .map_err(|e| format!("[zora] invalid api_key: {e}"))?;
        headers.insert("api-key", v);
        let inner = reqwest::Client::builder()
            .default_headers(headers)
            .build()
            .map_err(|e| format!("[zora] reqwest: {e}"))?;
        Ok(ZoraClient::new_with_client(BASE_URL, inner))
    } else {
        Ok(ZoraClient::new(BASE_URL))
    }
}

// ============================================================================
// Tool 1: get_coin
// ============================================================================
pub(crate) struct GetCoin;
#[derive(Debug, Deserialize, JsonSchema)]
pub(crate) struct GetCoinArgs {
    /// Coin contract address on Base (`0x...`).
    pub address: String,
    /// Optional chain ID (defaults to 8453, Base mainnet).
    #[serde(default)]
    pub chain: Option<i64>,
    pub api_key: Option<String>,
}
impl DynAomiTool for GetCoin {
    type App = ZoraApp;
    type Args = GetCoinArgs;
    const NAME: &'static str = "zora_get_coin";
    const DESCRIPTION: &'static str = "Get full detail for a Zora coin by address — name, symbol, creator, market data. Use after the user names a coin or one is discovered via `zora_get_trends_by_name`.";

    fn run(_app: &ZoraApp, args: Self::Args, _ctx: DynToolCallCtx) -> Result<Value, String> {
        rt()?.block_on(async move {
            let client = make_client(args.api_key.as_deref())?;
            let resp = client.get_coin(args.address.as_str(), args.chain.or(Some(8453))).await
                .map_err(|e| format!("[zora] get_coin: {e}"))?.into_inner();
            ok(resp)
        })
    }
}

// ============================================================================
// Tool 2: get_coin_holders
// ============================================================================
pub(crate) struct GetCoinHolders;
#[derive(Debug, Deserialize, JsonSchema)]
pub(crate) struct GetCoinHoldersArgs {
    pub address: String,
    #[serde(default)]
    pub chain_id: Option<i64>,
    #[serde(default)]
    pub count: Option<i64>,
    pub api_key: Option<String>,
}
impl DynAomiTool for GetCoinHolders {
    type App = ZoraApp;
    type Args = GetCoinHoldersArgs;
    const NAME: &'static str = "zora_get_coin_holders";
    const DESCRIPTION: &'static str = "Get top holders of a Zora coin. Useful to gauge concentration before trading.";

    fn run(_app: &ZoraApp, args: Self::Args, _ctx: DynToolCallCtx) -> Result<Value, String> {
        rt()?.block_on(async move {
            let client = make_client(args.api_key.as_deref())?;
            let resp = client.get_coin_holders(
                args.address.as_str(), None, args.chain_id.unwrap_or(8453), args.count
            ).await.map_err(|e| format!("[zora] get_coin_holders: {e}"))?.into_inner();
            ok(resp)
        })
    }
}

// ============================================================================
// Tool 3: get_coin_price_history
// ============================================================================
pub(crate) struct GetCoinPriceHistory;
#[derive(Debug, Deserialize, JsonSchema)]
pub(crate) struct GetCoinPriceHistoryArgs {
    pub address: String,
    #[serde(default)]
    pub chain: Option<i64>,
    pub api_key: Option<String>,
}
impl DynAomiTool for GetCoinPriceHistory {
    type App = ZoraApp;
    type Args = GetCoinPriceHistoryArgs;
    const NAME: &'static str = "zora_get_coin_price_history";
    const DESCRIPTION: &'static str = "Price + volume history for a Zora coin. Use when the user asks for a chart or wants momentum.";

    fn run(_app: &ZoraApp, args: Self::Args, _ctx: DynToolCallCtx) -> Result<Value, String> {
        rt()?.block_on(async move {
            let client = make_client(args.api_key.as_deref())?;
            let resp = client.get_coin_price_history(
                args.address.as_str(), args.chain.or(Some(8453))
            ).await.map_err(|e| format!("[zora] get_coin_price_history: {e}"))?.into_inner();
            ok(resp)
        })
    }
}

// ============================================================================
// Tool 4: get_trends_by_name
// ============================================================================
pub(crate) struct GetTrendsByName;
#[derive(Debug, Deserialize, JsonSchema)]
pub(crate) struct GetTrendsByNameArgs {
    pub name: String,
    #[serde(default)]
    pub first: Option<i64>,
    pub api_key: Option<String>,
}
impl DynAomiTool for GetTrendsByName {
    type App = ZoraApp;
    type Args = GetTrendsByNameArgs;
    const NAME: &'static str = "zora_get_trends_by_name";
    const DESCRIPTION: &'static str = "Search trending Zora creator/content coins by name. Use to discover what's hot or find a coin matching a term.";

    fn run(_app: &ZoraApp, args: Self::Args, _ctx: DynToolCallCtx) -> Result<Value, String> {
        rt()?.block_on(async move {
            let client = make_client(args.api_key.as_deref())?;
            let resp = client.get_trends_by_name(None, args.first, args.name.as_str()).await
                .map_err(|e| format!("[zora] get_trends_by_name: {e}"))?.into_inner();
            ok(resp)
        })
    }
}

// ============================================================================
// Tool 5: get_featured_creators
// ============================================================================
pub(crate) struct GetFeaturedCreators;
#[derive(Debug, Deserialize, JsonSchema)]
pub(crate) struct GetFeaturedCreatorsArgs {
    #[serde(default)]
    pub first: Option<i64>,
    pub api_key: Option<String>,
}
impl DynAomiTool for GetFeaturedCreators {
    type App = ZoraApp;
    type Args = GetFeaturedCreatorsArgs;
    const NAME: &'static str = "zora_get_featured_creators";
    const DESCRIPTION: &'static str = "Current featured/top creators on Zora. Use when the user wants to discover noteworthy creator-coin issuers.";

    fn run(_app: &ZoraApp, args: Self::Args, _ctx: DynToolCallCtx) -> Result<Value, String> {
        rt()?.block_on(async move {
            let client = make_client(args.api_key.as_deref())?;
            let resp = client.get_featured_creators(None, args.first, None, None).await
                .map_err(|e| format!("[zora] get_featured_creators: {e}"))?.into_inner();
            ok(resp)
        })
    }
}

// ============================================================================
// Tool 6: get_profile
// ============================================================================
pub(crate) struct GetProfile;
#[derive(Debug, Deserialize, JsonSchema)]
pub(crate) struct GetProfileArgs {
    /// Handle, address, or profile ID.
    pub identifier: String,
    pub api_key: Option<String>,
}
impl DynAomiTool for GetProfile {
    type App = ZoraApp;
    type Args = GetProfileArgs;
    const NAME: &'static str = "zora_get_profile";
    const DESCRIPTION: &'static str = "Get a full Zora profile by handle, address, or profile ID — bio, stats, deployed coins.";

    fn run(_app: &ZoraApp, args: Self::Args, _ctx: DynToolCallCtx) -> Result<Value, String> {
        rt()?.block_on(async move {
            let client = make_client(args.api_key.as_deref())?;
            let resp = client.get_profile(args.identifier.as_str()).await
                .map_err(|e| format!("[zora] get_profile: {e}"))?.into_inner();
            ok(resp)
        })
    }
}
