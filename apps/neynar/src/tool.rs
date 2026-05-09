use aomi_ext::neynar::{
    CastLookupQuery, ChannelQuery, FeedQuery, NeynarClient, PublishCastRequest, SearchCastsQuery,
    SearchUsersQuery, TrendingFeedQuery, UrlEmbed, UserByUsernameQuery,
};
use aomi_sdk::schemars::JsonSchema;
use aomi_sdk::*;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Clone, Default)]
pub(crate) struct NeynarApp;

fn ok<T: Serialize>(value: T) -> Result<Value, String> {
    let value = serde_json::to_value(value)
        .map_err(|e| format!("[neynar] failed to serialize response: {e}"))?;
    Ok(match value {
        Value::Object(mut map) => {
            map.insert("source".to_string(), Value::String("neynar".to_string()));
            Value::Object(map)
        }
        other => serde_json::json!({ "source": "neynar", "data": other }),
    })
}

fn neynar_client(api_key: Option<&str>) -> Result<NeynarClient, String> {
    let api_key = resolve_secret_value(
        api_key,
        "NEYNAR_API_KEY",
        "[neynar] missing api_key argument and NEYNAR_API_KEY environment variable",
    )?;
    NeynarClient::new(api_key)
}

// ============================================================================
// Tool 1: GetUserByUsername
// ============================================================================

pub(crate) struct GetUserByUsername;

#[derive(Debug, Deserialize, JsonSchema)]
pub(crate) struct GetUserByUsernameArgs {
    /// Optional Neynar API key. Falls back to NEYNAR_API_KEY when omitted.
    #[serde(default)]
    pub(crate) api_key: Option<String>,
    /// Farcaster username to look up
    pub(crate) username: String,
}

impl DynAomiTool for GetUserByUsername {
    type App = NeynarApp;
    type Args = GetUserByUsernameArgs;
    const NAME: &'static str = "get_user_by_username";
    const DESCRIPTION: &'static str = "Look up a Farcaster user profile by username. Returns display name, bio, follower count, FID, and more.";

    fn run(_app: &NeynarApp, args: Self::Args, _ctx: DynToolCallCtx) -> Result<Value, String> {
        let client = neynar_client(args.api_key.as_deref())?;
        let username = args.username.trim_start_matches('@');
        ok(client.get(
            "/farcaster/user/by_username",
            &UserByUsernameQuery { username },
            "get_user_by_username",
        )?)
    }
}

// ============================================================================
// Tool 2: SearchUsers
// ============================================================================

pub(crate) struct SearchUsers;

#[derive(Debug, Deserialize, JsonSchema)]
pub(crate) struct SearchUsersArgs {
    /// Optional Neynar API key. Falls back to NEYNAR_API_KEY when omitted.
    #[serde(default)]
    pub(crate) api_key: Option<String>,
    /// Search query string to find users
    pub(crate) q: String,
}

impl DynAomiTool for SearchUsers {
    type App = NeynarApp;
    type Args = SearchUsersArgs;
    const NAME: &'static str = "search_users";
    const DESCRIPTION: &'static str =
        "Search for Farcaster users by name or keyword. Returns a list of matching user profiles.";

    fn run(_app: &NeynarApp, args: Self::Args, _ctx: DynToolCallCtx) -> Result<Value, String> {
        let client = neynar_client(args.api_key.as_deref())?;
        ok(client.get(
            "/farcaster/user/search",
            &SearchUsersQuery { q: args.q.as_str() },
            "search_users",
        )?)
    }
}

// ============================================================================
// Tool 3: GetFeed
// ============================================================================

pub(crate) struct GetFeed;

#[derive(Debug, Deserialize, JsonSchema)]
pub(crate) struct GetFeedArgs {
    /// Optional Neynar API key. Falls back to NEYNAR_API_KEY when omitted.
    #[serde(default)]
    pub(crate) api_key: Option<String>,
    /// Feed type, e.g. 'filter' or 'following'
    pub(crate) feed_type: String,
    /// Farcaster ID to filter the feed by (optional for some feed types)
    pub(crate) fid: Option<u64>,
    /// Maximum number of results to return (default 25, max 100)
    pub(crate) limit: Option<u32>,
}

impl DynAomiTool for GetFeed {
    type App = NeynarApp;
    type Args = GetFeedArgs;
    const NAME: &'static str = "get_feed";
    const DESCRIPTION: &'static str =
        "Get casts from a Farcaster feed. Supports filtering by feed type, FID, and result limit.";

    fn run(_app: &NeynarApp, args: Self::Args, _ctx: DynToolCallCtx) -> Result<Value, String> {
        let client = neynar_client(args.api_key.as_deref())?;
        ok(client.get(
            "/farcaster/feed",
            &FeedQuery {
                feed_type: args.feed_type.as_str(),
                fid: args.fid,
                limit: args.limit.unwrap_or(25),
            },
            "get_feed",
        )?)
    }
}

// ============================================================================
// Tool 4: GetCast
// ============================================================================

pub(crate) struct GetCast;

#[derive(Debug, Deserialize, JsonSchema)]
pub(crate) struct GetCastArgs {
    /// Optional Neynar API key. Falls back to NEYNAR_API_KEY when omitted.
    #[serde(default)]
    pub(crate) api_key: Option<String>,
    /// Cast identifier: a cast hash (0x...) or a Warpcast URL
    pub(crate) identifier: String,
    /// Type of the identifier: 'hash' or 'url'
    #[serde(rename = "type")]
    pub(crate) id_type: String,
}

impl DynAomiTool for GetCast {
    type App = NeynarApp;
    type Args = GetCastArgs;
    const NAME: &'static str = "get_cast";
    const DESCRIPTION: &'static str = "Get a single Farcaster cast by its hash or Warpcast URL. Returns cast content, author, reactions, and replies.";

    fn run(_app: &NeynarApp, args: Self::Args, _ctx: DynToolCallCtx) -> Result<Value, String> {
        let client = neynar_client(args.api_key.as_deref())?;
        ok(client.get(
            "/farcaster/cast",
            &CastLookupQuery {
                identifier: args.identifier.as_str(),
                id_type: args.id_type.as_str(),
            },
            "get_cast",
        )?)
    }
}

// ============================================================================
// Tool 5: SearchCasts
// ============================================================================

pub(crate) struct SearchCasts;

#[derive(Debug, Deserialize, JsonSchema)]
pub(crate) struct SearchCastsArgs {
    /// Optional Neynar API key. Falls back to NEYNAR_API_KEY when omitted.
    #[serde(default)]
    pub(crate) api_key: Option<String>,
    /// Search query string to find casts
    pub(crate) q: String,
    /// Maximum number of results to return (default 25, max 100)
    pub(crate) limit: Option<u32>,
}

impl DynAomiTool for SearchCasts {
    type App = NeynarApp;
    type Args = SearchCastsArgs;
    const NAME: &'static str = "search_casts";
    const DESCRIPTION: &'static str = "Search for Farcaster casts by keyword. Returns matching casts with content, author info, and engagement metrics.";

    fn run(_app: &NeynarApp, args: Self::Args, _ctx: DynToolCallCtx) -> Result<Value, String> {
        let client = neynar_client(args.api_key.as_deref())?;
        ok(client.get(
            "/farcaster/cast/search",
            &SearchCastsQuery {
                q: args.q.as_str(),
                limit: args.limit.unwrap_or(25),
            },
            "search_casts",
        )?)
    }
}

// ============================================================================
// Tool 6: PublishCast
// ============================================================================

pub(crate) struct PublishCast;

#[derive(Debug, Deserialize, JsonSchema)]
pub(crate) struct PublishCastArgs {
    /// Optional Neynar API key. Falls back to NEYNAR_API_KEY when omitted.
    #[serde(default)]
    pub(crate) api_key: Option<String>,
    /// UUID of the signer authorized to publish on behalf of the user
    pub(crate) signer_uuid: String,
    /// Text content of the cast (up to 1024 bytes)
    pub(crate) text: String,
    /// Optional list of embed URLs to attach to the cast
    pub(crate) embeds: Option<Vec<EmbedArg>>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub(crate) struct EmbedArg {
    /// URL to embed in the cast
    pub(crate) url: String,
}

impl DynAomiTool for PublishCast {
    type App = NeynarApp;
    type Args = PublishCastArgs;
    const NAME: &'static str = "publish_cast";
    const DESCRIPTION: &'static str = "Publish a new cast to Farcaster. Requires a signer_uuid authorized to act on behalf of the user.";

    fn run(_app: &NeynarApp, args: Self::Args, _ctx: DynToolCallCtx) -> Result<Value, String> {
        let client = neynar_client(args.api_key.as_deref())?;
        let body = PublishCastRequest {
            signer_uuid: args.signer_uuid,
            text: args.text,
            embeds: args.embeds.map(|embeds| {
                embeds
                    .into_iter()
                    .map(|embed| UrlEmbed { url: embed.url })
                    .collect()
            }),
        };
        ok(client.post_json("/farcaster/cast", &body, "publish_cast")?)
    }
}

// ============================================================================
// Tool 7: GetChannel
// ============================================================================

pub(crate) struct GetChannel;

#[derive(Debug, Deserialize, JsonSchema)]
pub(crate) struct GetChannelArgs {
    /// Optional Neynar API key. Falls back to NEYNAR_API_KEY when omitted.
    #[serde(default)]
    pub(crate) api_key: Option<String>,
    /// Channel ID (e.g. 'ethereum', 'farcaster', 'memes')
    pub(crate) id: String,
}

impl DynAomiTool for GetChannel {
    type App = NeynarApp;
    type Args = GetChannelArgs;
    const NAME: &'static str = "get_channel";
    const DESCRIPTION: &'static str = "Get information about a Farcaster channel by its ID. Returns channel name, description, follower count, and image.";

    fn run(_app: &NeynarApp, args: Self::Args, _ctx: DynToolCallCtx) -> Result<Value, String> {
        let client = neynar_client(args.api_key.as_deref())?;
        ok(client.get(
            "/farcaster/channel",
            &ChannelQuery {
                id: args.id.as_str(),
            },
            "get_channel",
        )?)
    }
}

// ============================================================================
// Tool 8: GetTrendingFeed
// ============================================================================

pub(crate) struct GetTrendingFeed;

#[derive(Debug, Deserialize, JsonSchema)]
pub(crate) struct GetTrendingFeedArgs {
    /// Optional Neynar API key. Falls back to NEYNAR_API_KEY when omitted.
    #[serde(default)]
    pub(crate) api_key: Option<String>,
    /// Maximum number of trending casts to return (default 10, max 100)
    pub(crate) limit: Option<u32>,
    /// Time window for trending calculation, e.g. '6h', '12h', '24h', '7d'
    pub(crate) time_window: Option<String>,
}

impl DynAomiTool for GetTrendingFeed {
    type App = NeynarApp;
    type Args = GetTrendingFeedArgs;
    const NAME: &'static str = "get_trending_feed";
    const DESCRIPTION: &'static str =
        "Get trending casts on Farcaster. Returns popular casts within a configurable time window.";

    fn run(_app: &NeynarApp, args: Self::Args, _ctx: DynToolCallCtx) -> Result<Value, String> {
        let client = neynar_client(args.api_key.as_deref())?;
        let time_window = args.time_window.unwrap_or_else(|| "24h".to_string());
        ok(client.get(
            "/farcaster/trending/feed",
            &TrendingFeedQuery {
                limit: args.limit.unwrap_or(10),
                time_window: time_window.as_str(),
            },
            "get_trending_feed",
        )?)
    }
}
