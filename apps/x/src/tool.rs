use aomi_ext::x::{
    AdvancedSearchQuery, Post, PostsData, TrendsData, TrendsQuery, TweetInfoQuery, User,
    UserInfoQuery, UserLastTweetsQuery, XClient, XPostsView, XSearchResultsView, XTrendsView,
    format_post, format_trend, format_user,
};
use aomi_sdk::schemars::JsonSchema;
use aomi_sdk::*;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Clone, Default)]
pub(crate) struct XApp;

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

fn x_client(api_key: Option<&str>) -> Result<XClient, String> {
    let api_key = resolve_secret_value(
        api_key,
        "X_API_KEY",
        "[x] missing api_key argument and X_API_KEY environment variable",
    )?;
    XClient::new(api_key)
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
    const DESCRIPTION: &'static str = "Use when the user asks about an X account (followers, bio, verification, account age). Pass the handle without the @ — e.g. \"elonmusk\". Returns the profile object.";

    fn run(_app: &XApp, args: Self::Args, _ctx: DynToolCallCtx) -> Result<Value, String> {
        let client = x_client(args.api_key.as_deref())?;
        let username = args.username.trim_start_matches('@');
        let user: User = client.get(
            "/twitter/user/info",
            &UserInfoQuery {
                user_name: username,
            },
        )?;
        ok(format_user(&user))
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
    const DESCRIPTION: &'static str = "Use when the user asks \"what has @handle been posting recently\". Returns one page of recent posts (text + engagement) plus a `cursor` for pagination. Pass the handle without the @.";

    fn run(_app: &XApp, args: Self::Args, _ctx: DynToolCallCtx) -> Result<Value, String> {
        let client = x_client(args.api_key.as_deref())?;
        let username = args.username.trim_start_matches('@');
        let data: PostsData = client.get(
            "/twitter/user/last_tweets",
            &UserLastTweetsQuery {
                user_name: username,
                cursor: args.cursor.as_deref().filter(|cursor| !cursor.is_empty()),
            },
        )?;
        let posts = data.tweets.unwrap_or_default();
        let formatted = posts.iter().map(format_post).collect::<Vec<_>>();
        ok(XPostsView {
            posts_count: formatted.len(),
            posts: formatted,
            cursor: data.next_cursor,
            has_more: data.has_next_page.unwrap_or(false),
        })
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

    fn run(_app: &XApp, args: Self::Args, _ctx: DynToolCallCtx) -> Result<Value, String> {
        let client = x_client(args.api_key.as_deref())?;
        let query_type = args.query_type.as_deref().unwrap_or("Latest");
        let data: PostsData = client.get(
            "/twitter/tweet/advanced_search",
            &AdvancedSearchQuery {
                query: args.query.as_str(),
                query_type,
                cursor: args.cursor.as_deref().filter(|cursor| !cursor.is_empty()),
            },
        )?;
        let posts = data.tweets.unwrap_or_default();
        let formatted = posts.iter().map(format_post).collect::<Vec<_>>();
        ok(XSearchResultsView {
            query: args.query,
            query_type: query_type.to_string(),
            results_count: formatted.len(),
            posts: formatted,
            cursor: data.next_cursor,
            has_more: data.has_next_page.unwrap_or(false),
        })
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

    fn run(_app: &XApp, args: Self::Args, _ctx: DynToolCallCtx) -> Result<Value, String> {
        let client = x_client(args.api_key.as_deref())?;
        let data: TrendsData = client.get(
            "/twitter/trends",
            &TrendsQuery {
                woeid: args.woeid,
                count: args.count,
            },
        )?;
        let trends = data.trends.unwrap_or_default();
        let formatted = trends.iter().map(format_trend).collect::<Vec<_>>();
        ok(XTrendsView {
            trends_count: formatted.len(),
            trends: formatted,
        })
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

    fn run(_app: &XApp, args: Self::Args, _ctx: DynToolCallCtx) -> Result<Value, String> {
        let client = x_client(args.api_key.as_deref())?;
        let post: Post = client.get(
            "/twitter/tweet/info",
            &TweetInfoQuery {
                tweet_id: args.post_id.as_str(),
            },
        )?;
        ok(format_post(&post))
    }
}
