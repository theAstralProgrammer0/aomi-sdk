//! Curated tool layer for Limitless prediction markets on Base. Public reads
//! go through the progenitor-generated client at `crate::client` (see
//! `apps/limitless/openapi.yaml`). Authenticated reads (portfolio, key info)
//! bypass the generated client and call reqwest directly so we can attach the
//! HMAC-signed `lmts-*` headers that the Limitless spec doesn't model as
//! per-operation parameters — see `crate::auth` for the prehash format.
//!
//! Designed for the user story: research active prediction markets on Base
//! via Limitless, look at orderbooks, view your positions/trades, and verify
//! your API key works. Signed-order placement (POST /orders) is intentionally
//! NOT exposed yet — it needs typed-body + EIP-712 signature material that
//! gen-tool can't synth from the spec; add a `limitless_place_order`
//! composite by hand when ready.
//!
//! 7 curated tools:
//!   * limitless_search_markets       — semantic text search across markets (public)
//!   * limitless_browse_active        — list active markets by category    (public)
//!   * limitless_get_market           — full detail for one market         (public)
//!   * limitless_get_orderbook        — bids/asks for a market             (public)
//!   * limitless_check_key            — verify the API key + secret work   (signed)
//!   * limitless_get_my_positions     — your open positions                (signed)
//!   * limitless_get_my_trades        — your trade history                 (signed)

use crate::auth::{iso_timestamp, sign};
// The progenitor-generated client is currently unused at runtime: every
// endpoint we exercise has spec drift (renamed query params, extra response
// fields, or 404 paths), so the curated layer hits the API via raw reqwest
// in `public_get` / `signed_get`. We keep the generated module compiled so
// regenerating the spec is friction-free; switch back to typed calls once
// the spec is realigned with the live server.
use aomi_sdk::schemars::JsonSchema;
use aomi_sdk::*;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

#[derive(Clone, Default)]
pub(crate) struct LimitlessApp;

const BASE_URL: &str = "https://api.limitless.exchange";

// ============================================================================
// Helpers
// ============================================================================

fn ok<T: Serialize>(value: T) -> Result<Value, String> {
    let value = serde_json::to_value(value).map_err(|e| format!("[limitless] serialize: {e}"))?;
    Ok(match value {
        Value::Object(mut m) => {
            m.insert("source".into(), Value::String("limitless".into()));
            Value::Object(m)
        }
        other => json!({ "source": "limitless", "data": other }),
    })
}

fn rt() -> Result<tokio::runtime::Runtime, String> {
    tokio::runtime::Runtime::new().map_err(|e| format!("[limitless] runtime: {e}"))
}

/// Minimal percent-encoder for URL path/query segments. Keeps unreserved
/// characters per RFC 3986 verbatim, percent-encodes everything else.
fn urlencode(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for b in s.as_bytes() {
        let c = *b as char;
        if c.is_ascii_alphanumeric() || matches!(c, '-' | '_' | '.' | '~') {
            out.push(c);
        } else {
            use std::fmt::Write;
            let _ = write!(out, "%{:02X}", b);
        }
    }
    out
}

/// Resolve `(api_key, api_secret)`, falling back to env vars.
fn resolve_creds(
    api_key: Option<&str>,
    api_secret: Option<&str>,
) -> Result<(String, String), String> {
    let key = resolve_secret_value(
        api_key,
        "LIMITLESS_API_KEY",
        "[limitless] missing api_key argument and LIMITLESS_API_KEY env var",
    )?;
    let sec = resolve_secret_value(
        api_secret,
        "LIMITLESS_API_SECRET",
        "[limitless] missing api_secret argument and LIMITLESS_API_SECRET env var",
    )?;
    Ok((key, sec))
}

/// Run an HMAC-signed GET against `path_with_query` (no body) and decode the
/// JSON response into `serde_json::Value`. Used by every signed tool below.
async fn signed_get(
    api_key: &str,
    api_secret: &str,
    path_with_query: &str,
) -> Result<Value, String> {
    let timestamp = iso_timestamp();
    let signature = sign(api_secret, &timestamp, "GET", path_with_query, "")?;
    let url = format!("{BASE_URL}{path_with_query}");
    let resp = reqwest::Client::new()
        .get(&url)
        .header("lmts-api-key", api_key)
        .header("lmts-timestamp", &timestamp)
        .header("lmts-signature", &signature)
        .send()
        .await
        .map_err(|e| format!("[limitless] HTTP error on {path_with_query}: {e}"))?;
    let status = resp.status();
    let body = resp
        .text()
        .await
        .map_err(|e| format!("[limitless] failed to read response body: {e}"))?;
    if !status.is_success() {
        return Err(format!(
            "[limitless] {path_with_query} returned {status}: {body}"
        ));
    }
    serde_json::from_str(&body).map_err(|e| {
        format!(
            "[limitless] response was not JSON ({e}); first 200 chars: {}",
            body.chars().take(200).collect::<String>()
        )
    })
}

/// Public GET (no auth) returning raw JSON. Used for endpoints where the
/// generated client's typed response disagrees with the live API (spec drift)
/// — we forward the body unchanged and let the LLM read what's actually there.
async fn public_get(path_with_query: &str) -> Result<Value, String> {
    let url = format!("{BASE_URL}{path_with_query}");
    let resp = reqwest::Client::new()
        .get(&url)
        .send()
        .await
        .map_err(|e| format!("[limitless] HTTP error on {path_with_query}: {e}"))?;
    let status = resp.status();
    let body = resp
        .text()
        .await
        .map_err(|e| format!("[limitless] failed to read response body: {e}"))?;
    if !status.is_success() {
        return Err(format!(
            "[limitless] {path_with_query} returned {status}: {body}"
        ));
    }
    serde_json::from_str(&body).map_err(|e| {
        format!(
            "[limitless] response was not JSON ({e}); first 200 chars: {}",
            body.chars().take(200).collect::<String>()
        )
    })
}

// ============================================================================
// Tool 1: search_markets — semantic search (public)
// ============================================================================

pub(crate) struct SearchMarkets;

#[derive(Debug, Deserialize, JsonSchema)]
pub(crate) struct SearchMarketsArgs {
    /// Free-text query (e.g., "election", "ETH price by year-end", "fed rate cut").
    pub query: String,
    /// Max markets to return (default 20).
    #[serde(default)]
    pub limit: Option<i64>,
}

impl DynAomiTool for SearchMarkets {
    type App = LimitlessApp;
    type Args = SearchMarketsArgs;
    const NAME: &'static str = "limitless_search_markets";
    const DESCRIPTION: &'static str = "Semantic search across active Limitless prediction markets. Use when the user names a topic ('election', 'ETH price', 'fed cuts') and wants matching markets. Returns market summaries (slug, title, current YES/NO prices). Public — no API key needed.";

    fn run(_app: &LimitlessApp, args: Self::Args, _ctx: DynToolCallCtx) -> Result<Value, String> {
        let runtime = rt()?;
        runtime.block_on(async move {
            // Live API expects ?query= (the spec called it `q`, which the
            // server rejects). Use raw public_get so we control the wire.
            let mut path = format!("/markets/search?query={}", urlencode(&args.query));
            if let Some(limit) = args.limit {
                path.push_str(&format!("&limit={limit}"));
            }
            let resp = public_get(&path).await?;
            ok(resp)
        })
    }
}

// ============================================================================
// Tool 2: browse_active — list active markets, optionally by category (public)
// ============================================================================

pub(crate) struct BrowseActive;

#[derive(Debug, Deserialize, JsonSchema)]
pub(crate) struct BrowseActiveArgs {
    /// Optional category ID to filter by. Omit to browse all categories.
    #[serde(default)]
    pub category_id: Option<f64>,
    /// Max results per page (default 50).
    #[serde(default)]
    pub limit: Option<i64>,
    /// Page index (1-based, default 1).
    #[serde(default)]
    pub page: Option<i64>,
}

impl DynAomiTool for BrowseActive {
    type App = LimitlessApp;
    type Args = BrowseActiveArgs;
    const NAME: &'static str = "limitless_browse_active";
    const DESCRIPTION: &'static str = "Browse active Limitless markets. Use when the user wants to discover what's currently tradeable. Optional `category_id` narrows the list. Public — no API key needed.";

    fn run(_app: &LimitlessApp, args: Self::Args, _ctx: DynToolCallCtx) -> Result<Value, String> {
        let runtime = rt()?;
        runtime.block_on(async move {
            // Live API rejects unknown query params, so build the URL by hand
            // (the generated client sends a few extras that aren't accepted).
            let limit = args.limit.unwrap_or(50);
            let page = args.page.unwrap_or(1);
            let mut path = format!("/markets/active?limit={limit}&page={page}");
            if let Some(cat) = args.category_id {
                path.push_str(&format!("&category_id={cat}"));
            }
            let resp = public_get(&path).await?;
            ok(resp)
        })
    }
}

// ============================================================================
// Tool 3: get_market — full detail for a single market (public)
// ============================================================================

pub(crate) struct GetMarket;

#[derive(Debug, Deserialize, JsonSchema)]
pub(crate) struct GetMarketArgs {
    /// Market slug or contract address.
    pub address_or_slug: String,
}

impl DynAomiTool for GetMarket {
    type App = LimitlessApp;
    type Args = GetMarketArgs;
    const NAME: &'static str = "limitless_get_market";
    const DESCRIPTION: &'static str = "Fetch full detail for one market by slug or contract address — title, description, outcomes, current prices, volume, expiration. Use after `limitless_search_markets` or `limitless_browse_active` returns a slug. Public — no API key needed.";

    fn run(_app: &LimitlessApp, args: Self::Args, _ctx: DynToolCallCtx) -> Result<Value, String> {
        let runtime = rt()?;
        runtime.block_on(async move {
            // Path is `/markets/{slug}`. Forward raw JSON — the typed schema
            // misses several fields the live API returns.
            let path = format!("/markets/{}", urlencode(&args.address_or_slug));
            let resp = public_get(&path).await?;
            ok(resp)
        })
    }
}

// ============================================================================
// Tool 4: get_orderbook — bids/asks for a market (public)
// ============================================================================

pub(crate) struct GetOrderbook;

#[derive(Debug, Deserialize, JsonSchema)]
pub(crate) struct GetOrderbookArgs {
    /// Market slug.
    pub slug: String,
}

impl DynAomiTool for GetOrderbook {
    type App = LimitlessApp;
    type Args = GetOrderbookArgs;
    const NAME: &'static str = "limitless_get_orderbook";
    const DESCRIPTION: &'static str = "Get the L2 orderbook (bids and asks) for a Limitless market. Use to gauge liquidity before placing an order. Public — no API key needed.";

    fn run(_app: &LimitlessApp, args: Self::Args, _ctx: DynToolCallCtx) -> Result<Value, String> {
        let runtime = rt()?;
        runtime.block_on(async move {
            // Live response carries an extra `side` field per level (and may
            // omit some optional fields), so the generated typed response
            // fails to deserialize. Forward the raw JSON instead.
            let path = format!("/markets/{}/orderbook", urlencode(&args.slug));
            let resp = public_get(&path).await?;
            ok(resp)
        })
    }
}

// ============================================================================
// Tool 5: check_key — verify the active key works (signed)
// ============================================================================

pub(crate) struct CheckKey;

#[derive(Debug, Deserialize, JsonSchema)]
pub(crate) struct CheckKeyArgs {
    /// Limitless API key id (falls back to LIMITLESS_API_KEY env var).
    #[serde(default)]
    pub api_key: Option<String>,
    /// Limitless API secret, base64-encoded as shown in the dashboard
    /// (falls back to LIMITLESS_API_SECRET env var).
    #[serde(default)]
    pub api_secret: Option<String>,
}

impl DynAomiTool for CheckKey {
    type App = LimitlessApp;
    type Args = CheckKeyArgs;
    const NAME: &'static str = "limitless_check_key";
    const DESCRIPTION: &'static str = "Verify that the configured Limitless API key + secret work. Calls GET /auth/api-keys/active and returns metadata about the active key (id, scopes, created_at). Run this first if any signed tool starts failing.";

    fn run(_app: &LimitlessApp, args: Self::Args, _ctx: DynToolCallCtx) -> Result<Value, String> {
        let (key, sec) = resolve_creds(args.api_key.as_deref(), args.api_secret.as_deref())?;
        let runtime = rt()?;
        runtime.block_on(async move {
            // Live path is `/auth/api-keys` (no `/active` suffix). Returns
            // the metadata for the currently-authenticated key.
            let resp = signed_get(&key, &sec, "/auth/api-keys").await?;
            ok(resp)
        })
    }
}

// ============================================================================
// Tool 6: get_my_positions — open positions for the authenticated user (signed)
// ============================================================================

pub(crate) struct GetMyPositions;

#[derive(Debug, Deserialize, JsonSchema)]
pub(crate) struct GetMyPositionsArgs {
    /// Limitless API key id (falls back to LIMITLESS_API_KEY env var).
    #[serde(default)]
    pub api_key: Option<String>,
    /// Limitless API secret (falls back to LIMITLESS_API_SECRET env var).
    #[serde(default)]
    pub api_secret: Option<String>,
}

impl DynAomiTool for GetMyPositions {
    type App = LimitlessApp;
    type Args = GetMyPositionsArgs;
    const NAME: &'static str = "limitless_get_my_positions";
    const DESCRIPTION: &'static str = "Get the authenticated user's open positions across all Limitless markets. Requires a Limitless API key + secret (HMAC-signed). Returns size, entry price, current PnL per position.";

    fn run(_app: &LimitlessApp, args: Self::Args, _ctx: DynToolCallCtx) -> Result<Value, String> {
        let (key, sec) = resolve_creds(args.api_key.as_deref(), args.api_secret.as_deref())?;
        let runtime = rt()?;
        runtime.block_on(async move {
            let resp = signed_get(&key, &sec, "/portfolio/positions").await?;
            ok(resp)
        })
    }
}

// ============================================================================
// Tool 7: get_my_trades — trade history for the authenticated user (signed)
// ============================================================================

pub(crate) struct GetMyTrades;

#[derive(Debug, Deserialize, JsonSchema)]
pub(crate) struct GetMyTradesArgs {
    /// Limitless API key id (falls back to LIMITLESS_API_KEY env var).
    #[serde(default)]
    pub api_key: Option<String>,
    /// Limitless API secret (falls back to LIMITLESS_API_SECRET env var).
    #[serde(default)]
    pub api_secret: Option<String>,
}

impl DynAomiTool for GetMyTrades {
    type App = LimitlessApp;
    type Args = GetMyTradesArgs;
    const NAME: &'static str = "limitless_get_my_trades";
    const DESCRIPTION: &'static str = "Get the authenticated user's recent trade fills on Limitless. Requires a Limitless API key + secret (HMAC-signed). Useful for PnL review and position reconciliation.";

    fn run(_app: &LimitlessApp, args: Self::Args, _ctx: DynToolCallCtx) -> Result<Value, String> {
        let (key, sec) = resolve_creds(args.api_key.as_deref(), args.api_secret.as_deref())?;
        let runtime = rt()?;
        runtime.block_on(async move {
            let resp = signed_get(&key, &sec, "/portfolio/trades").await?;
            ok(resp)
        })
    }
}
