//! Curated tool layer for Neynar (Farcaster API). Hand-written from the
//! generated client in `aomi_ext::neynar` — see ext/specs/neynar.yaml for the
//! full surface.
//!
//! Designed for the user story: browse Farcaster — find users, view their
//! casts, browse channel feeds. The 140 mechanical tools from
//! `aomi-build gen-tool` were collapsed into 8 user-centric ones:
//!
//!   * `neynar_lookup_user`        — profile by handle
//!   * `neynar_search_users`       — fuzzy user search
//!   * `neynar_get_user_casts`     — recent casts authored by a user
//!   * `neynar_get_popular_casts`  — most popular casts authored by a user
//!   * `neynar_lookup_cast`        — single cast by hash or warpcast URL
//!   * `neynar_search_casts`       — keyword search across all casts
//!   * `neynar_lookup_channel`     — channel info by slug
//!   * `neynar_search_channels`    — channel keyword search

use aomi_ext::neynar::Client as NeynarClient;
use aomi_ext::neynar::types::LookupCastByHashOrUrlType;
use aomi_sdk::schemars::JsonSchema;
use aomi_sdk::*;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::num::{NonZeroU32, NonZeroU64};

#[derive(Clone, Default)]
pub(crate) struct NeynarApp;

const BASE_URL: &str = "https://api.neynar.com";

// ============================================================================
// Helpers
// ============================================================================

fn ok<T: Serialize>(value: T) -> Result<Value, String> {
    let value = serde_json::to_value(value).map_err(|e| format!("[neynar] serialize: {e}"))?;
    Ok(match value {
        Value::Object(mut m) => {
            m.insert("source".into(), Value::String("neynar".into()));
            Value::Object(m)
        }
        other => json!({ "source": "neynar", "data": other }),
    })
}

fn rt() -> Result<tokio::runtime::Runtime, String> {
    tokio::runtime::Runtime::new().map_err(|e| format!("[neynar] runtime: {e}"))
}

fn resolve_key(ctx: &DynToolCallCtx, arg: Option<&str>) -> Result<String, String> {
    resolve_secret_value(
        ctx,
        arg,
        "NEYNAR_API_KEY",
        "[neynar] missing api_key argument and NEYNAR_API_KEY env var",
    )
}

/// Build a reqwest client with the Neynar API key as a default header. The
/// generated progenitor client doesn't add the `x-api-key` header itself
/// (the spec declares it via `securitySchemes` rather than as an op param),
/// so we inject it here once per call.
fn build_client(api_key: &str) -> Result<NeynarClient, String> {
    use reqwest::header::{HeaderMap, HeaderValue};
    let mut headers = HeaderMap::new();
    let mut hv = HeaderValue::from_str(api_key)
        .map_err(|e| format!("[neynar] invalid api key header value: {e}"))?;
    hv.set_sensitive(true);
    headers.insert("x-api-key", hv);
    let client = reqwest::ClientBuilder::new()
        .default_headers(headers)
        .timeout(std::time::Duration::from_secs(15))
        .build()
        .map_err(|e| format!("[neynar] reqwest build: {e}"))?;
    Ok(NeynarClient::new_with_client(BASE_URL, client))
}

fn nz64(v: i64) -> Result<NonZeroU64, String> {
    if v <= 0 {
        return Err(format!("[neynar] expected positive integer, got {v}"));
    }
    NonZeroU64::new(v as u64).ok_or_else(|| "[neynar] zero not allowed".to_string())
}

fn nz32(v: u32) -> Option<NonZeroU32> {
    NonZeroU32::new(v)
}

// ============================================================================
// Tool 1: lookup_user — profile by handle
// ============================================================================

pub(crate) struct LookupUser;

#[derive(Debug, Deserialize, JsonSchema)]
pub(crate) struct LookupUserArgs {
    /// API credential (falls back to env var NEYNAR_API_KEY).
    pub api_key: Option<String>,
    /// Farcaster handle. Pass without leading `@` (e.g. "vitalik").
    pub username: String,
}

impl DynAomiTool for LookupUser {
    type App = NeynarApp;
    type Args = LookupUserArgs;
    const NAME: &'static str = "neynar_lookup_user";
    const DESCRIPTION: &'static str = "Look up a Farcaster user by their handle (username). Use when the user mentions an @handle and you need their FID, profile, follower counts, or bio. Returns the full user object.";

    fn run(_app: &NeynarApp, args: Self::Args, ctx: DynToolCallCtx) -> Result<Value, String> {
        let api_key = resolve_key(&ctx, args.api_key.as_deref())?;
        let username = args.username.trim().trim_start_matches('@').to_string();
        let runtime = rt()?;
        runtime.block_on(async move {
            let client = build_client(&api_key)?;
            let resp = client
                .lookup_user_by_username(&username, None, None)
                .await
                .map_err(|e| format!("[neynar] lookup_user_by_username: {e}"))?;
            ok(resp.into_inner())
        })
    }
}

// ============================================================================
// Tool 2: search_users — fuzzy user search
// ============================================================================

pub(crate) struct SearchUsers;

#[derive(Debug, Deserialize, JsonSchema)]
pub(crate) struct SearchUsersArgs {
    /// API credential (falls back to env var NEYNAR_API_KEY).
    pub api_key: Option<String>,
    /// Free-text query — matches handles, display names and bios.
    pub query: String,
    /// Max results, 1–100. Defaults to 10.
    #[serde(default)]
    pub limit: Option<u32>,
}

impl DynAomiTool for SearchUsers {
    type App = NeynarApp;
    type Args = SearchUsersArgs;
    const NAME: &'static str = "neynar_search_users";
    const DESCRIPTION: &'static str = "Search Farcaster users by name/handle/bio keyword. Use when the user describes a person but doesn't know their exact handle. Returns up to `limit` matching profiles ranked by relevance.";

    fn run(_app: &NeynarApp, args: Self::Args, ctx: DynToolCallCtx) -> Result<Value, String> {
        let api_key = resolve_key(&ctx, args.api_key.as_deref())?;
        let query = args.query;
        let limit = nz32(args.limit.unwrap_or(10).clamp(1, 100));
        let runtime = rt()?;
        runtime.block_on(async move {
            let client = build_client(&api_key)?;
            let resp = client
                .search_user(None, limit, &query, None, None)
                .await
                .map_err(|e| format!("[neynar] search_user: {e}"))?;
            ok(resp.into_inner())
        })
    }
}

// ============================================================================
// Tool 3: get_user_casts — recent casts authored by a user
// ============================================================================

pub(crate) struct GetUserCasts;

#[derive(Debug, Deserialize, JsonSchema)]
pub(crate) struct GetUserCastsArgs {
    /// API credential (falls back to env var NEYNAR_API_KEY).
    pub api_key: Option<String>,
    /// Farcaster ID (FID) of the author. Get from `neynar_lookup_user` if you only have a handle.
    pub fid: i64,
    /// If true, include the user's reply casts; if false (default), only top-level casts.
    #[serde(default)]
    pub include_replies: Option<bool>,
    /// Max casts to return, 1–150. Defaults to 25.
    #[serde(default)]
    pub limit: Option<u32>,
}

impl DynAomiTool for GetUserCasts {
    type App = NeynarApp;
    type Args = GetUserCastsArgs;
    const NAME: &'static str = "neynar_get_user_casts";
    const DESCRIPTION: &'static str = "Fetch a user's recent casts in reverse-chronological order. Use when the user asks 'what has @handle been posting' or 'show me X's recent casts'. Pass `include_replies=true` to also include their replies.";

    fn run(_app: &NeynarApp, args: Self::Args, ctx: DynToolCallCtx) -> Result<Value, String> {
        let api_key = resolve_key(&ctx, args.api_key.as_deref())?;
        let fid = nz64(args.fid)?;
        let include_replies = args.include_replies;
        let limit = nz32(args.limit.unwrap_or(25).clamp(1, 150));
        let runtime = rt()?;
        runtime.block_on(async move {
            let client = build_client(&api_key)?;
            let resp = client
                .fetch_casts_for_user(
                    None,
                    None,
                    None,
                    fid,
                    include_replies,
                    limit,
                    None,
                    None,
                    None,
                )
                .await
                .map_err(|e| format!("[neynar] fetch_casts_for_user: {e}"))?;
            ok(resp.into_inner())
        })
    }
}

// ============================================================================
// Tool 4: get_popular_casts — most popular casts authored by a user
// ============================================================================

pub(crate) struct GetPopularCasts;

#[derive(Debug, Deserialize, JsonSchema)]
pub(crate) struct GetPopularCastsArgs {
    /// API credential (falls back to env var NEYNAR_API_KEY).
    pub api_key: Option<String>,
    /// Farcaster ID (FID) of the author.
    pub fid: i64,
}

impl DynAomiTool for GetPopularCasts {
    type App = NeynarApp;
    type Args = GetPopularCastsArgs;
    const NAME: &'static str = "neynar_get_popular_casts";
    const DESCRIPTION: &'static str = "Fetch the most popular (highest-engagement) casts authored by a user. Use when the user asks 'what are X's best casts' or 'show me top posts from @handle' — distinct from `get_user_casts` which is reverse-chronological.";

    fn run(_app: &NeynarApp, args: Self::Args, ctx: DynToolCallCtx) -> Result<Value, String> {
        let api_key = resolve_key(&ctx, args.api_key.as_deref())?;
        let fid = nz64(args.fid)?;
        let runtime = rt()?;
        runtime.block_on(async move {
            let client = build_client(&api_key)?;
            let resp = client
                .fetch_popular_casts_by_user(fid, None)
                .await
                .map_err(|e| format!("[neynar] fetch_popular_casts_by_user: {e}"))?;
            ok(resp.into_inner())
        })
    }
}

// ============================================================================
// Tool 5: lookup_cast — single cast by hash or warpcast URL
// ============================================================================

pub(crate) struct LookupCast;

#[derive(Debug, Deserialize, JsonSchema)]
pub(crate) struct LookupCastArgs {
    /// API credential (falls back to env var NEYNAR_API_KEY).
    pub api_key: Option<String>,
    /// Either a 0x... cast hash (40 hex chars) or a warpcast.com URL. The kind
    /// is auto-detected from the input shape.
    pub identifier: String,
}

impl DynAomiTool for LookupCast {
    type App = NeynarApp;
    type Args = LookupCastArgs;
    const NAME: &'static str = "neynar_lookup_cast";
    const DESCRIPTION: &'static str = "Fetch a single cast by its hash (0x...) or by a warpcast.com URL. Use when the user shares a link to a cast or quotes a hash. The identifier kind (hash vs url) is detected automatically.";

    fn run(_app: &NeynarApp, args: Self::Args, ctx: DynToolCallCtx) -> Result<Value, String> {
        let api_key = resolve_key(&ctx, args.api_key.as_deref())?;
        let id = args.identifier.trim().to_string();
        let kind = if id.starts_with("http://") || id.starts_with("https://") {
            LookupCastByHashOrUrlType::Url
        } else {
            LookupCastByHashOrUrlType::Hash
        };
        let runtime = rt()?;
        runtime.block_on(async move {
            let client = build_client(&api_key)?;
            let resp = client
                .lookup_cast_by_hash_or_url(&id, kind, None, None)
                .await
                .map_err(|e| format!("[neynar] lookup_cast_by_hash_or_url: {e}"))?;
            ok(resp.into_inner())
        })
    }
}

// ============================================================================
// Tool 6: search_casts — keyword search across all casts
// ============================================================================

pub(crate) struct SearchCasts;

#[derive(Debug, Deserialize, JsonSchema)]
pub(crate) struct SearchCastsArgs {
    /// API credential (falls back to env var NEYNAR_API_KEY).
    pub api_key: Option<String>,
    /// Search keyword(s). Matches cast text.
    pub query: String,
    /// Optional channel slug to scope the search to one channel (e.g. "ethereum").
    #[serde(default)]
    pub channel_id: Option<String>,
    /// Optional FID to limit search to one author.
    #[serde(default)]
    pub author_fid: Option<i64>,
    /// Max results, 1–100. Defaults to 25.
    #[serde(default)]
    pub limit: Option<u32>,
}

impl DynAomiTool for SearchCasts {
    type App = NeynarApp;
    type Args = SearchCastsArgs;
    const NAME: &'static str = "neynar_search_casts";
    const DESCRIPTION: &'static str = "Keyword search across all Farcaster casts. Use when the user asks 'what's being said about X' or wants to find casts containing specific terms. Optionally scope to one channel or one author.";

    fn run(_app: &NeynarApp, args: Self::Args, ctx: DynToolCallCtx) -> Result<Value, String> {
        let api_key = resolve_key(&ctx, args.api_key.as_deref())?;
        let query = args.query;
        let channel_id = args.channel_id;
        let author_fid = args.author_fid.map(nz64).transpose()?;
        let limit = nz32(args.limit.unwrap_or(25).clamp(1, 100));
        let runtime = rt()?;
        runtime.block_on(async move {
            let client = build_client(&api_key)?;
            let resp = client
                .search_casts(
                    author_fid,
                    channel_id.as_deref(),
                    None,
                    limit,
                    None,
                    None,
                    &query,
                    None,
                    None,
                    None,
                )
                .await
                .map_err(|e| format!("[neynar] search_casts: {e}"))?;
            ok(resp.into_inner())
        })
    }
}

// ============================================================================
// Tool 7: lookup_channel — channel info by slug
// ============================================================================

pub(crate) struct LookupChannel;

#[derive(Debug, Deserialize, JsonSchema)]
pub(crate) struct LookupChannelArgs {
    /// API credential (falls back to env var NEYNAR_API_KEY).
    pub api_key: Option<String>,
    /// Channel slug from the URL: `warpcast.com/~/channel/ethereum` → `ethereum`.
    pub channel_id: String,
}

impl DynAomiTool for LookupChannel {
    type App = NeynarApp;
    type Args = LookupChannelArgs;
    const NAME: &'static str = "neynar_lookup_channel";
    const DESCRIPTION: &'static str = "Fetch metadata about a Farcaster channel by its slug. Use when the user references a /channel and you need its description, member count, hosts, or parent URL.";

    fn run(_app: &NeynarApp, args: Self::Args, ctx: DynToolCallCtx) -> Result<Value, String> {
        let api_key = resolve_key(&ctx, args.api_key.as_deref())?;
        let id = args.channel_id;
        let runtime = rt()?;
        runtime.block_on(async move {
            let client = build_client(&api_key)?;
            let resp = client
                .lookup_channel(&id, None, None)
                .await
                .map_err(|e| format!("[neynar] lookup_channel: {e}"))?;
            ok(resp.into_inner())
        })
    }
}

// ============================================================================
// Tool 8: search_channels — channel keyword search
// ============================================================================

pub(crate) struct SearchChannels;

#[derive(Debug, Deserialize, JsonSchema)]
pub(crate) struct SearchChannelsArgs {
    /// API credential (falls back to env var NEYNAR_API_KEY).
    pub api_key: Option<String>,
    /// Free-text channel name or topic to search for.
    pub query: String,
    /// Max results, 1–100. Defaults to 20.
    #[serde(default)]
    pub limit: Option<u64>,
}

impl DynAomiTool for SearchChannels {
    type App = NeynarApp;
    type Args = SearchChannelsArgs;
    const NAME: &'static str = "neynar_search_channels";
    const DESCRIPTION: &'static str = "Search Farcaster channels by name or topic keyword. Use when the user wants to discover a channel about a topic (e.g. 'is there a channel about base?'). Returns matching channels with their slugs.";

    fn run(_app: &NeynarApp, args: Self::Args, ctx: DynToolCallCtx) -> Result<Value, String> {
        let api_key = resolve_key(&ctx, args.api_key.as_deref())?;
        let query = args.query;
        let limit = args.limit.unwrap_or(20).clamp(1, 100);
        let limit = NonZeroU64::new(limit);
        let runtime = rt()?;
        runtime.block_on(async move {
            let client = build_client(&api_key)?;
            let resp = client
                .search_channels(None, limit, &query, None)
                .await
                .map_err(|e| format!("[neynar] search_channels: {e}"))?;
            ok(resp.into_inner())
        })
    }
}
