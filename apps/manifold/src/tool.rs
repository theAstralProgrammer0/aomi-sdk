//! Curated tool layer for Manifold Markets. Hand-written from the generated
//! client in `aomi_ext::manifold` — see ext/specs/manifold.yaml for the surface.
//!
//! Six user-centric tools: list/search/get markets, market positions, place_bet,
//! create_market.

use aomi_ext::manifold::Client as GenClient;
use aomi_ext::manifold::types::{CreateMarketRequest, PlaceBetRequest};
use aomi_sdk::schemars::JsonSchema;
use aomi_sdk::*;
use reqwest::header::{AUTHORIZATION, HeaderMap, HeaderValue};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::num::NonZeroU32;
use std::time::Duration;

#[derive(Clone, Default)]
pub(crate) struct ManifoldApp;

const BASE_URL: &str = "https://api.manifold.markets/v0";

// ============================================================================
// Helpers
// ============================================================================

fn ok<T: Serialize>(value: T) -> Result<Value, String> {
    let value = serde_json::to_value(value)
        .map_err(|e| format!("[manifold] failed to serialize response: {e}"))?;
    Ok(match value {
        Value::Object(mut map) => {
            map.insert("source".to_string(), Value::String("manifold".to_string()));
            Value::Object(map)
        }
        other => json!({ "source": "manifold", "data": other }),
    })
}

fn rt() -> Result<tokio::runtime::Runtime, String> {
    tokio::runtime::Runtime::new().map_err(|e| format!("[manifold] runtime: {e}"))
}

fn resolve_manifold_api_key(ctx: &DynToolCallCtx,
    api_key: Option<&str>) -> Result<String, String> {
    resolve_secret_value(ctx, api_key,
        "MANIFOLD_API_KEY",
        "[manifold] missing api_key argument and MANIFOLD_API_KEY environment variable",
    )
}

/// Build a generated client without auth (read endpoints).
fn public_client() -> Result<GenClient, String> {
    let http = reqwest::ClientBuilder::new()
        .timeout(Duration::from_secs(30))
        .build()
        .map_err(|e| format!("[manifold] failed to build HTTP client: {e}"))?;
    Ok(GenClient::new_with_client(BASE_URL, http))
}

/// Build a generated client carrying `Authorization: Key <api_key>` for write endpoints.
fn authed_client(api_key: &str) -> Result<GenClient, String> {
    let mut headers = HeaderMap::new();
    let mut value = HeaderValue::from_str(&format!("Key {api_key}"))
        .map_err(|e| format!("[manifold] invalid api_key: {e}"))?;
    value.set_sensitive(true);
    headers.insert(AUTHORIZATION, value);

    let http = reqwest::ClientBuilder::new()
        .timeout(Duration::from_secs(30))
        .default_headers(headers)
        .build()
        .map_err(|e| format!("[manifold] failed to build HTTP client: {e}"))?;
    Ok(GenClient::new_with_client(BASE_URL, http))
}

// ============================================================================
// ListMarkets
// ============================================================================

pub(crate) struct ListMarkets;

#[derive(Debug, Deserialize, JsonSchema)]
pub(crate) struct ListMarketsArgs {
    /// Maximum number of markets to return (default 20, max 1000).
    limit: Option<u32>,
    /// Sort order: "newest" (default) or "score" for hottest.
    sort: Option<String>,
    /// Filter by topic slug(s), comma-separated (e.g. "ai,politics").
    topics: Option<String>,
}

impl DynAomiTool for ListMarkets {
    type App = ManifoldApp;
    type Args = ListMarketsArgs;
    const NAME: &'static str = "list_markets";
    const DESCRIPTION: &'static str = "Use when the user wants to browse Manifold prediction markets without a specific keyword, e.g. \"what's hot on Manifold\" or \"newest markets\". Returns a compact list of {id, question, url, probability, volume, isResolved}. Use search_markets when there is a keyword.";

    fn run(_app: &ManifoldApp, args: Self::Args, _ctx: DynToolCallCtx) -> Result<Value, String> {
        let limit = args.limit.unwrap_or(20).min(1000) as i32;
        let sort = args.sort.clone().unwrap_or_else(|| "newest".to_string());
        let topics = args.topics.clone();

        let runtime = rt()?;
        let markets = runtime.block_on(async move {
            let client = public_client()?;
            client
                .list_markets(Some(limit), Some(sort.as_str()), topics.as_deref())
                .await
                .map_err(|e| format!("[manifold] list_markets: {e}"))
                .map(|r| r.into_inner())
        })?;

        ok(json!({ "markets_count": markets.len(), "markets": markets }))
    }
}

// ============================================================================
// GetMarket
// ============================================================================

pub(crate) struct GetMarket;

#[derive(Debug, Deserialize, JsonSchema)]
pub(crate) struct GetMarketArgs {
    /// The market ID or slug to look up
    id: String,
}

impl DynAomiTool for GetMarket {
    type App = ManifoldApp;
    type Args = GetMarketArgs;
    const NAME: &'static str = "get_market";
    const DESCRIPTION: &'static str = "Use when the user wants full detail on one Manifold market (probability, volume, liquidity, close time, resolution). Accepts an `id` (24-char hex) or a slug from the market URL.";

    fn run(_app: &ManifoldApp, args: Self::Args, _ctx: DynToolCallCtx) -> Result<Value, String> {
        let runtime = rt()?;
        let market = runtime.block_on(async move {
            let client = public_client()?;
            client
                .get_market(args.id.as_str())
                .await
                .map_err(|e| format!("[manifold] get_market: {e}"))
                .map(|r| r.into_inner())
        })?;

        ok(market)
    }
}

// ============================================================================
// GetMarketPositions
// ============================================================================

pub(crate) struct GetMarketPositions;

#[derive(Debug, Deserialize, JsonSchema)]
pub(crate) struct GetMarketPositionsArgs {
    /// The market ID to get positions for
    id: String,
}

impl DynAomiTool for GetMarketPositions {
    type App = ManifoldApp;
    type Args = GetMarketPositionsArgs;
    const NAME: &'static str = "get_market_positions";
    const DESCRIPTION: &'static str = "Use when the user asks who is holding what on a market (top traders, position concentration). Returns the list of user positions for a given market id.";

    fn run(_app: &ManifoldApp, args: Self::Args, _ctx: DynToolCallCtx) -> Result<Value, String> {
        let id = args.id.clone();
        let runtime = rt()?;
        let positions = runtime.block_on(async move {
            let client = public_client()?;
            client
                .get_market_positions(id.as_str())
                .await
                .map_err(|e| format!("[manifold] get_market_positions: {e}"))
                .map(|r| r.into_inner())
        })?;
        ok(json!({ "market_id": args.id, "positions": positions }))
    }
}

// ============================================================================
// SearchMarkets
// ============================================================================

pub(crate) struct SearchMarkets;

#[derive(Debug, Deserialize, JsonSchema)]
pub(crate) struct SearchMarketsArgs {
    /// Search query term
    term: String,
    /// Sort order: "newest", "score", "liquidity", etc.
    sort: Option<String>,
    /// Filter: "open", "closed", "resolved", "all"
    filter: Option<String>,
}

impl DynAomiTool for SearchMarkets {
    type App = ManifoldApp;
    type Args = SearchMarketsArgs;
    const NAME: &'static str = "search_markets";
    const DESCRIPTION: &'static str = "Use when the user asks about a topic and you need to find Manifold markets — e.g. \"is there a market on the next Fed cut?\". `term` is a keyword search; `filter` defaults to \"open\" so unresolved markets surface first.";

    fn run(_app: &ManifoldApp, args: Self::Args, _ctx: DynToolCallCtx) -> Result<Value, String> {
        // Default to open markets so resolved/closed don't dominate results.
        let filter = args.filter.clone().unwrap_or_else(|| "open".to_string());
        let sort = args.sort.clone();
        let term = args.term.clone();

        let runtime = rt()?;
        let markets = runtime.block_on(async move {
            let client = public_client()?;
            client
                .search_markets(Some(filter.as_str()), sort.as_deref(), term.as_str())
                .await
                .map_err(|e| format!("[manifold] search_markets: {e}"))
                .map(|r| r.into_inner())
        })?;

        ok(json!({
            "query": args.term,
            "results_count": markets.len(),
            "markets": markets,
        }))
    }
}

// ============================================================================
// PlaceBet
// ============================================================================

pub(crate) struct PlaceBet;

#[derive(Debug, Deserialize, JsonSchema)]
pub(crate) struct PlaceBetArgs {
    /// Manifold API key for authentication
    api_key: Option<String>,
    /// The contract/market ID to bet on
    contract_id: String,
    /// Amount of mana (M$) to bet
    amount: f64,
    /// Outcome to bet on: "YES" or "NO"
    outcome: String,
}

impl DynAomiTool for PlaceBet {
    type App = ManifoldApp;
    type Args = PlaceBetArgs;
    const NAME: &'static str = "place_bet";
    const DESCRIPTION: &'static str = "Use when the user wants to bet mana on a Manifold binary (YES/NO) market. `contract_id` is the market `id` from get_market or search_markets. `amount` is in mana (M$). Requires MANIFOLD_API_KEY.";

    fn run(_app: &ManifoldApp, args: Self::Args, ctx: DynToolCallCtx) -> Result<Value, String> {
        let outcome = args.outcome.to_uppercase();
        if outcome != "YES" && outcome != "NO" {
            return Err("outcome must be YES or NO".to_string());
        }
        if args.amount <= 0.0 {
            return Err("amount must be greater than 0".to_string());
        }

        let api_key = resolve_manifold_api_key(&ctx, args.api_key.as_deref())?;
        let body = PlaceBetRequest {
            contract_id: args.contract_id.clone(),
            amount: args.amount,
            outcome: outcome.clone(),
        };

        let runtime = rt()?;
        let response = runtime.block_on(async move {
            let client = authed_client(&api_key)?;
            client
                .place_bet(&body)
                .await
                .map_err(|e| format!("[manifold] place_bet: {e}"))
                .map(|r| r.into_inner())
        })?;

        ok(response)
    }
}

// ============================================================================
// CreateMarket
// ============================================================================

pub(crate) struct CreateMarket;

#[derive(Debug, Deserialize, JsonSchema)]
pub(crate) struct CreateMarketArgs {
    /// Manifold API key for authentication
    api_key: Option<String>,
    /// The question for the market (e.g., "Will X happen by Y date?")
    question: String,
    /// Market type (use "BINARY" for yes/no markets)
    #[serde(rename = "type")]
    market_type: Option<String>,
    /// Close time as a Unix timestamp in milliseconds
    close_time: Option<u64>,
    /// Initial probability for binary markets (1-99)
    initial_prob: Option<u32>,
}

impl DynAomiTool for CreateMarket {
    type App = ManifoldApp;
    type Args = CreateMarketArgs;
    const NAME: &'static str = "create_market";
    const DESCRIPTION: &'static str = "Use when the user wants to launch their own prediction market. Defaults to a BINARY (YES/NO) market with 50% initial probability. Requires MANIFOLD_API_KEY.";

    fn run(_app: &ManifoldApp, args: Self::Args, ctx: DynToolCallCtx) -> Result<Value, String> {
        let market_type = args
            .market_type
            .clone()
            .unwrap_or_else(|| "BINARY".to_string());
        let initial_prob = args.initial_prob.unwrap_or(50);

        if !(1..=99).contains(&initial_prob) {
            return Err("initial_prob must be between 1 and 99".to_string());
        }
        let initial_prob = NonZeroU32::new(initial_prob)
            .ok_or_else(|| "initial_prob must be greater than 0".to_string())?;

        let api_key = resolve_manifold_api_key(&ctx, args.api_key.as_deref())?;
        let close_time = args.close_time.map(|v| v as i64);
        let body = CreateMarketRequest {
            close_time,
            initial_prob,
            outcome_type: market_type,
            question: args.question.clone(),
        };

        let runtime = rt()?;
        let response = runtime.block_on(async move {
            let client = authed_client(&api_key)?;
            client
                .create_market(&body)
                .await
                .map_err(|e| format!("[manifold] create_market: {e}"))
                .map(|r| r.into_inner())
        })?;

        ok(response)
    }
}
