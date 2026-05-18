//! Curated tool layer for Kaito AI. Hand-written from the
//! progenitor-generated client in `aomi_ext::kaito` (see
//! `ext/specs/kaito.yaml`). Kaito is the InfoFi network behind the Yaps
//! Open Protocol — quantifying crypto-influencer ("yapper") attention
//! across Twitter/X, Discord, Telegram, governance forums, Farcaster,
//! podcasts, and conference transcripts.
//!
//! Designed for the user story: rank and explore crypto yappers via
//! Kaito — get a yapper's score, list trending narratives they're
//! talking about, search the corpus for what specific yappers are
//! saying, and check token mindshare alongside.
//!
//! 4 endpoints map to 4 user-centric tools:
//!
//!   * `kaito_get_yapper_score`     — per-yapper Yaps score (typed).
//!   * `kaito_search`               — semantic search the corpus for yapper
//!     activity on a topic.
//!   * `kaito_trending_narratives`  — currently trending narratives.
//!   * `kaito_get_token_mindshare`  — attention metrics for a token.
//!
//! `/search`, `/trending`, and `/mindshare/{token}` are
//! `additionalProperties: true` in the spec (Kaito does not publish
//! per-field response shapes); they pass through as opaque JSON.
//! `/yaps` has a documented response shape and is typed concretely.

use aomi_ext::kaito::Client as GenClient;
use aomi_sdk::schemars::JsonSchema;
use aomi_sdk::*;
use reqwest::header::{AUTHORIZATION, HeaderMap, HeaderValue};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::num::NonZeroU32;
use std::time::Duration;

#[derive(Clone, Default)]
pub(crate) struct KaitoApp;

const DEFAULT_BASE_URL: &str = "https://api.kaito.ai/api/v1";
const BASE_URL_ENV: &str = "KAITO_API_ENDPOINT";

// ============================================================================
// Helpers
// ============================================================================

fn ok<T: Serialize>(value: T) -> Result<Value, String> {
    let value = serde_json::to_value(value).map_err(|e| format!("[kaito] serialize: {e}"))?;
    Ok(match value {
        Value::Object(mut m) => {
            m.insert("source".into(), Value::String("kaito".into()));
            Value::Object(m)
        }
        other => json!({ "source": "kaito", "data": other }),
    })
}

fn rt() -> Result<tokio::runtime::Runtime, String> {
    tokio::runtime::Runtime::new().map_err(|e| format!("[kaito] runtime: {e}"))
}

fn resolve_key(ctx: &DynToolCallCtx, arg: Option<&str>) -> Result<String, String> {
    resolve_secret_value(
        ctx,
        arg,
        "KAITO_API_KEY",
        "[kaito] missing api_key argument and KAITO_API_KEY env var",
    )
}

fn base_url() -> String {
    std::env::var(BASE_URL_ENV).unwrap_or_else(|_| DEFAULT_BASE_URL.to_string())
}

/// Build a generated client with `Authorization: Bearer <api_key>` wired in
/// as a default header on every request. Mirrors the pattern in
/// `apps/oneinch/src/tool.rs`.
fn make_client(api_key: &str) -> Result<GenClient, String> {
    let mut headers = HeaderMap::new();
    let mut bearer = HeaderValue::from_str(&format!("Bearer {api_key}"))
        .map_err(|e| format!("[kaito] invalid api_key: {e}"))?;
    bearer.set_sensitive(true);
    headers.insert(AUTHORIZATION, bearer);

    let http = reqwest::ClientBuilder::new()
        .timeout(Duration::from_secs(30))
        .default_headers(headers)
        .build()
        .map_err(|e| format!("[kaito] failed to build HTTP client: {e}"))?;

    Ok(GenClient::new_with_client(&base_url(), http))
}

/// Convert a user-supplied limit (i32) into the client's expected
/// `Option<NonZeroU32>`. Negative or zero values are dropped.
fn nz_limit(limit: Option<i32>) -> Option<NonZeroU32> {
    limit
        .filter(|&n| n > 0)
        .and_then(|n| NonZeroU32::new(n as u32))
}

// ============================================================================
// Tool 1: kaito_get_yapper_score — GET /yaps
// ============================================================================

pub(crate) struct GetYapperScore;

#[derive(Debug, Deserialize, JsonSchema)]
pub(crate) struct GetYapperScoreArgs {
    /// Optional Kaito API key. Falls back to KAITO_API_KEY env var.
    #[serde(default)]
    pub api_key: Option<String>,
    /// X account numeric user id (preferred — stable across handle changes).
    #[serde(default)]
    pub user_id: Option<String>,
    /// X account handle (without @). Used when `user_id` is not provided.
    #[serde(default)]
    pub username: Option<String>,
}

impl DynAomiTool for GetYapperScore {
    type App = KaitoApp;
    type Args = GetYapperScoreArgs;
    const NAME: &'static str = "kaito_get_yapper_score";
    const DESCRIPTION: &'static str = "Use when the user asks 'what's <yapper>'s Kaito score' or 'how influential is <X handle>'. Returns the Yaps Open Protocol attention score for a single X/Twitter account across rolling windows (24h, 48h, 7d, 30d, 3m, 6m, 12m, all-time). Pass `user_id` (numeric X account id, preferred) or `username` (X handle without @) — at least one is required.";

    fn run(_app: &KaitoApp, args: Self::Args, ctx: DynToolCallCtx) -> Result<Value, String> {
        if args.user_id.is_none() && args.username.is_none() {
            return Err("[kaito] get_yapper_score: provide `user_id` or `username`".into());
        }
        let api_key = resolve_key(&ctx, args.api_key.as_deref())?;
        let client = make_client(&api_key)?;
        let runtime = rt()?;
        let resp = runtime
            .block_on(async move {
                client
                    .get_yapper_score(args.user_id.as_deref(), args.username.as_deref())
                    .await
            })
            .map_err(|e| format!("[kaito] get_yapper_score: {e}"))?
            .into_inner();
        ok(resp)
    }
}

// ============================================================================
// Tool 2: kaito_search — GET /search
// ============================================================================

pub(crate) struct Search;

#[derive(Debug, Deserialize, JsonSchema)]
pub(crate) struct SearchArgs {
    /// Optional Kaito API key. Falls back to KAITO_API_KEY env var.
    #[serde(default)]
    pub api_key: Option<String>,
    /// Free-text semantic query (e.g. "EigenLayer restaking yields",
    /// "Solana memecoin sentiment last week", "@vitalik on rollups").
    pub query: String,
    /// Optional source filter: "twitter", "discord", "telegram",
    /// "farcaster", "governance", "podcasts". Free-form; upstream may add more.
    #[serde(default)]
    pub source: Option<String>,
    /// Max results to return (default 20).
    #[serde(default)]
    pub limit: Option<i32>,
}

impl DynAomiTool for Search {
    type App = KaitoApp;
    type Args = SearchArgs;
    const NAME: &'static str = "kaito_search";
    const DESCRIPTION: &'static str = "Use when the user wants to search the Web3 conversation corpus — what yappers are saying about a topic across X/Twitter, Discord, Telegram, Farcaster, governance forums, podcasts, and conference transcripts. Returns AI-structured results with attention metrics. Optionally filter by `source`. Response shape is opaque (Kaito does not publish per-field docs for /search).";

    fn run(_app: &KaitoApp, args: Self::Args, ctx: DynToolCallCtx) -> Result<Value, String> {
        let api_key = resolve_key(&ctx, args.api_key.as_deref())?;
        let limit = nz_limit(args.limit.or(Some(20)));
        let client = make_client(&api_key)?;
        let runtime = rt()?;
        let resp = runtime
            .block_on(async move {
                client
                    .search_corpus(limit, args.query.as_str(), args.source.as_deref())
                    .await
            })
            .map_err(|e| format!("[kaito] search: {e}"))?
            .into_inner();
        // JsonObject is `additionalProperties: true` — pass through.
        ok(Value::Object(resp.into()))
    }
}

// ============================================================================
// Tool 3: kaito_trending_narratives — GET /trending
// ============================================================================

pub(crate) struct TrendingNarratives;

#[derive(Debug, Deserialize, JsonSchema)]
pub(crate) struct TrendingNarrativesArgs {
    /// Optional Kaito API key. Falls back to KAITO_API_KEY env var.
    #[serde(default)]
    pub api_key: Option<String>,
    /// Max trending entries to return (default 20).
    #[serde(default)]
    pub limit: Option<i32>,
}

impl DynAomiTool for TrendingNarratives {
    type App = KaitoApp;
    type Args = TrendingNarrativesArgs;
    const NAME: &'static str = "kaito_trending_narratives";
    const DESCRIPTION: &'static str = "Use when the user asks 'what narratives are trending in crypto right now' or 'what are yappers focused on'. Returns the top trending topics/narratives across Kaito's indexed Web3 sources, ordered by current attention. Response shape is opaque.";

    fn run(_app: &KaitoApp, args: Self::Args, ctx: DynToolCallCtx) -> Result<Value, String> {
        let api_key = resolve_key(&ctx, args.api_key.as_deref())?;
        let limit = nz_limit(args.limit.or(Some(20)));
        let client = make_client(&api_key)?;
        let runtime = rt()?;
        let resp = runtime
            .block_on(async move { client.get_trending_narratives(limit).await })
            .map_err(|e| format!("[kaito] trending_narratives: {e}"))?
            .into_inner();
        ok(Value::Object(resp.into()))
    }
}

// ============================================================================
// Tool 4: kaito_get_token_mindshare — GET /mindshare/{token}
// ============================================================================

pub(crate) struct GetTokenMindshare;

#[derive(Debug, Deserialize, JsonSchema)]
pub(crate) struct GetTokenMindshareArgs {
    /// Optional Kaito API key. Falls back to KAITO_API_KEY env var.
    #[serde(default)]
    pub api_key: Option<String>,
    /// Token symbol or name (e.g. "BTC", "ETH", "SOL", "EIGEN").
    pub token: String,
}

impl DynAomiTool for GetTokenMindshare {
    type App = KaitoApp;
    type Args = GetTokenMindshareArgs;
    const NAME: &'static str = "kaito_get_token_mindshare";
    const DESCRIPTION: &'static str = "Use when the user asks 'how much attention is token X getting' or wants quantitative mindshare metrics (discussion volume, share-of-voice, attention trend) for a specific token. Pass the token symbol or name. Response shape is opaque.";

    fn run(_app: &KaitoApp, args: Self::Args, ctx: DynToolCallCtx) -> Result<Value, String> {
        let api_key = resolve_key(&ctx, args.api_key.as_deref())?;
        let client = make_client(&api_key)?;
        let runtime = rt()?;
        let resp = runtime
            .block_on(async move { client.get_token_mindshare(args.token.as_str()).await })
            .map_err(|e| format!("[kaito] get_token_mindshare: {e}"))?
            .into_inner();
        ok(Value::Object(resp.into()))
    }
}
