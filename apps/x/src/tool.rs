//! Curated tool layer for the twitterapi.io read-only X (Twitter) API.
//!
//! Wraps the generated client in `aomi_ext::x` (see `ext/specs/x.yaml`).
//! Five user-centric read tools, all behind a single `X-API-Key` header that
//! is wired through the shared reqwest client.

use aomi_ext::x::Client as GenClient;
use aomi_sdk::schemars::JsonSchema;
use aomi_sdk::*;
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::time::Duration;

#[derive(Clone, Default)]
pub(crate) struct XApp;

const BASE_URL: &str = "https://api.twitterapi.io";

// ============================================================================
// Helpers
// ============================================================================

fn ok<T: Serialize>(value: T) -> Result<Value, String> {
    let value = serde_json::to_value(value)
        .map_err(|e| format!("[x] failed to serialize response: {e}"))?;
    Ok(match value {
        Value::Object(mut map) => {
            map.insert("source".to_string(), Value::String("x".to_string()));
            Value::Object(map)
        }
        other => serde_json::json!({ "source": "x", "data": other }),
    })
}

fn rt() -> Result<tokio::runtime::Runtime, String> {
    tokio::runtime::Runtime::new().map_err(|e| format!("[x] runtime: {e}"))
}

fn make_client(ctx: &DynToolCallCtx,
    api_key: Option<&str>) -> Result<GenClient, String> {
    let api_key = resolve_secret_value(ctx, api_key,
        "X_API_KEY",
        "[x] missing api_key argument and X_API_KEY environment variable",
    )?;
    let mut headers = HeaderMap::new();
    let mut value =
        HeaderValue::from_str(&api_key).map_err(|e| format!("[x] invalid api_key: {e}"))?;
    value.set_sensitive(true);
    headers.insert(HeaderName::from_static("x-api-key"), value);

    let http = reqwest::ClientBuilder::new()
        .timeout(Duration::from_secs(30))
        .default_headers(headers)
        .build()
        .map_err(|e| format!("[x] failed to build HTTP client: {e}"))?;
    Ok(GenClient::new_with_client(BASE_URL, http))
}

// ============================================================================
// Tool 1: GetXUser
// ============================================================================

pub(crate) struct GetXUser;

#[derive(Debug, Deserialize, JsonSchema)]
pub(crate) struct GetXUserArgs {
    /// Optional X API key. Falls back to X_API_KEY when omitted.
    #[serde(default)]
    pub(crate) api_key: Option<String>,
    /// X username without the @ symbol (e.g., 'elonmusk')
    pub(crate) username: String,
}

impl DynAomiTool for GetXUser {
    type App = XApp;
    type Args = GetXUserArgs;
    const NAME: &'static str = "get_x_user";
    const DESCRIPTION: &'static str = "Use when the user asks about an X account (followers, bio, account age, post count). Pass the handle without the @ — e.g. \"elonmusk\". Returns the profile object.";

    fn run(_app: &XApp, args: Self::Args, ctx: DynToolCallCtx) -> Result<Value, String> {
        let client = make_client(&ctx, args.api_key.as_deref())?;
        let username = args.username.trim_start_matches('@').to_string();
        let runtime = rt()?;
        let response = runtime.block_on(async move {
            client
                .get_user_info(username.as_str())
                .await
                .map_err(|e| format!("[x] get_user_info: {e}"))
                .map(|r| r.into_inner())
        })?;
        ok(response)
    }
}

// ============================================================================
// Tool 2: GetXUserPosts
// ============================================================================

pub(crate) struct GetXUserPosts;

#[derive(Debug, Deserialize, JsonSchema)]
pub(crate) struct GetXUserPostsArgs {
    /// Optional X API key. Falls back to X_API_KEY when omitted.
    #[serde(default)]
    api_key: Option<String>,
    /// X username without the @ symbol
    username: String,
    /// Pagination cursor for fetching more results
    cursor: Option<String>,
}

impl DynAomiTool for GetXUserPosts {
    type App = XApp;
    type Args = GetXUserPostsArgs;
    const NAME: &'static str = "get_x_user_posts";
    const DESCRIPTION: &'static str = "Use when the user asks \"what has @handle been posting recently\". Returns one page of recent posts (text + engagement) plus a `next_cursor` for pagination. Pass the handle without the @.";

    fn run(_app: &XApp, args: Self::Args, ctx: DynToolCallCtx) -> Result<Value, String> {
        let client = make_client(&ctx, args.api_key.as_deref())?;
        let username = args.username.trim_start_matches('@').to_string();
        let cursor = args
            .cursor
            .as_deref()
            .filter(|c| !c.is_empty())
            .map(str::to_string);
        let runtime = rt()?;
        let response = runtime.block_on(async move {
            client
                .get_user_last_tweets(cursor.as_deref(), username.as_str())
                .await
                .map_err(|e| format!("[x] get_user_last_tweets: {e}"))
                .map(|r| r.into_inner())
        })?;
        ok(response)
    }
}

// ============================================================================
// Tool 3: SearchX
// ============================================================================

pub(crate) struct SearchX;

#[derive(Debug, Deserialize, JsonSchema)]
pub(crate) struct SearchXArgs {
    /// Optional X API key. Falls back to X_API_KEY when omitted.
    #[serde(default)]
    api_key: Option<String>,
    /// Search query. Supports operators: from:user, #hashtag, @mention, lang:en, since:2026-01-01, until:2026-02-01, min_faves:100
    query: String,
    /// Sort order: 'Latest' for recent posts, 'Top' for popular posts (default: Latest)
    query_type: Option<String>,
    /// Pagination cursor for fetching more results
    cursor: Option<String>,
}

impl DynAomiTool for SearchX {
    type App = XApp;
    type Args = SearchXArgs;
    const NAME: &'static str = "search_x";
    const DESCRIPTION: &'static str = "Use when the user wants posts about a topic, hashtag, or from a specific account combined with filters. Supports advanced operators (from:, #tag, lang:en, since:YYYY-MM-DD, min_faves:N, filter:media). `query_type=Latest` for chronological, `Top` for most-engaged. Returns posts + cursor.";

    fn run(_app: &XApp, args: Self::Args, ctx: DynToolCallCtx) -> Result<Value, String> {
        let client = make_client(&ctx, args.api_key.as_deref())?;
        let query_type = args
            .query_type
            .clone()
            .unwrap_or_else(|| "Latest".to_string());
        let cursor = args
            .cursor
            .as_deref()
            .filter(|c| !c.is_empty())
            .map(str::to_string);
        let query = args.query.clone();
        let qt = query_type.clone();
        let runtime = rt()?;
        let response = runtime.block_on(async move {
            client
                .search_tweets(cursor.as_deref(), query.as_str(), qt.as_str())
                .await
                .map_err(|e| format!("[x] search_tweets: {e}"))
                .map(|r| r.into_inner())
        })?;
        ok(response)
    }
}

// ============================================================================
// Tool 4: GetXTrends
// ============================================================================

pub(crate) struct GetXTrends;

#[derive(Debug, Deserialize, JsonSchema)]
pub(crate) struct GetXTrendsArgs {
    /// Optional X API key. Falls back to X_API_KEY when omitted.
    #[serde(default)]
    api_key: Option<String>,
    /// Optional Yahoo WOEID location ID. The twitterapi.io trends docs document this parameter.
    #[serde(default)]
    woeid: Option<u64>,
    /// Optional number of trends to return.
    #[serde(default)]
    count: Option<u64>,
}

impl DynAomiTool for GetXTrends {
    type App = XApp;
    type Args = GetXTrendsArgs;
    const NAME: &'static str = "get_x_trends";
    const DESCRIPTION: &'static str = "Use when the user asks \"what's trending on X\". `woeid` is the Yahoo location id (1=worldwide, 23424977=US, 23424975=UK). Defaults to worldwide when omitted.";

    fn run(_app: &XApp, args: Self::Args, ctx: DynToolCallCtx) -> Result<Value, String> {
        let client = make_client(&ctx, args.api_key.as_deref())?;
        let count = args.count.map(|v| v as i64);
        let woeid = args.woeid.map(|v| v as i64);
        let runtime = rt()?;
        let response = runtime.block_on(async move {
            client
                .get_trends(count, woeid)
                .await
                .map_err(|e| format!("[x] get_trends: {e}"))
                .map(|r| r.into_inner())
        })?;
        ok(response)
    }
}

// ============================================================================
// Tool 5: GetXPost
// ============================================================================

pub(crate) struct GetXPost;

#[derive(Debug, Deserialize, JsonSchema)]
pub(crate) struct GetXPostArgs {
    /// Optional X API key. Falls back to X_API_KEY when omitted.
    #[serde(default)]
    api_key: Option<String>,
    /// The ID of the post to retrieve
    post_id: String,
}

impl DynAomiTool for GetXPost {
    type App = XApp;
    type Args = GetXPostArgs;
    const NAME: &'static str = "get_x_post";
    const DESCRIPTION: &'static str = "Use when the user shares an X post URL or numeric post id and wants full content + engagement. The id is the trailing number in `https://x.com/<user>/status/<id>`.";

    fn run(_app: &XApp, args: Self::Args, ctx: DynToolCallCtx) -> Result<Value, String> {
        let client = make_client(&ctx, args.api_key.as_deref())?;
        let post_id = args.post_id.clone();
        let runtime = rt()?;
        let response = runtime.block_on(async move {
            client
                .get_tweet_info(post_id.as_str())
                .await
                .map_err(|e| format!("[x] get_tweet_info: {e}"))
                .map(|r| r.into_inner())
        })?;
        ok(response)
    }
}
