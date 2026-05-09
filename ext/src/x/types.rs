use serde::{Deserialize, Serialize};
use serde_json::Value;

// ============================================================================
// Query types
// ============================================================================

#[derive(Debug, Clone, Serialize)]
pub struct UserInfoQuery<'a> {
    #[serde(rename = "userName")]
    pub user_name: &'a str,
}

#[derive(Debug, Clone, Serialize)]
pub struct UserLastTweetsQuery<'a> {
    #[serde(rename = "userName")]
    pub user_name: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<&'a str>,
}

#[derive(Debug, Clone, Serialize)]
pub struct AdvancedSearchQuery<'a> {
    pub query: &'a str,
    #[serde(rename = "queryType")]
    pub query_type: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<&'a str>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct TrendsQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub woeid: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<u64>,
}

#[derive(Debug, Clone, Serialize)]
pub struct TweetInfoQuery<'a> {
    #[serde(rename = "tweetId")]
    pub tweet_id: &'a str,
}

// ============================================================================
// API response models
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    #[serde(default, deserialize_with = "de_opt_string")]
    pub id: Option<String>,
    pub user_name: Option<String>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub location: Option<String>,
    pub url: Option<String>,
    pub profile_image_url: Option<String>,
    pub profile_banner_url: Option<String>,
    #[serde(default, deserialize_with = "de_opt_u64")]
    pub followers_count: Option<u64>,
    #[serde(default, deserialize_with = "de_opt_u64")]
    pub following_count: Option<u64>,
    #[serde(default, deserialize_with = "de_opt_u64")]
    pub favourites_count: Option<u64>,
    #[serde(default, deserialize_with = "de_opt_u64")]
    pub statuses_count: Option<u64>,
    #[serde(default, deserialize_with = "de_opt_u64")]
    pub listed_count: Option<u64>,
    pub created_at: Option<String>,
    pub verified: Option<bool>,
    pub is_blue_verified: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Post {
    #[serde(default, deserialize_with = "de_opt_string")]
    pub id: Option<String>,
    pub text: Option<String>,
    pub full_text: Option<String>,
    pub created_at: Option<String>,
    pub author: Option<PostAuthor>,
    #[serde(default, deserialize_with = "de_opt_u64")]
    pub retweet_count: Option<u64>,
    #[serde(default, deserialize_with = "de_opt_u64")]
    pub favorite_count: Option<u64>,
    #[serde(default, deserialize_with = "de_opt_u64")]
    pub reply_count: Option<u64>,
    #[serde(default, deserialize_with = "de_opt_u64")]
    pub quote_count: Option<u64>,
    #[serde(default, deserialize_with = "de_opt_u64")]
    pub view_count: Option<u64>,
    pub lang: Option<String>,
    pub is_retweet: Option<bool>,
    pub is_quote: Option<bool>,
    pub in_reply_to_status_id: Option<String>,
    pub conversation_id: Option<String>,
    pub hashtags: Option<Vec<String>>,
    pub mentions: Option<Vec<Mention>>,
    pub urls: Option<Vec<UrlEntity>>,
    pub media: Option<Vec<Media>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PostAuthor {
    #[serde(default, deserialize_with = "de_opt_string")]
    pub id: Option<String>,
    pub user_name: Option<String>,
    pub name: Option<String>,
    pub profile_image_url: Option<String>,
    pub is_blue_verified: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Mention {
    pub user_name: Option<String>,
    pub name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UrlEntity {
    pub expanded_url: Option<String>,
    pub display_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Media {
    pub media_url_https: Option<String>,
    #[serde(rename = "type")]
    pub media_type: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Trend {
    pub name: Option<String>,
    pub url: Option<String>,
    #[serde(default, deserialize_with = "de_opt_u64")]
    pub tweet_count: Option<u64>,
    pub description: Option<String>,
    pub domain_context: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct PostsData {
    pub tweets: Option<Vec<Post>>,
    pub next_cursor: Option<String>,
    pub has_next_page: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct TrendsData {
    pub trends: Option<Vec<Trend>>,
}

// ============================================================================
// View types (returned to LLM)
// ============================================================================

#[derive(Debug, Clone, Serialize)]
pub struct XUserView {
    pub id: Option<String>,
    pub username: Option<String>,
    pub name: Option<String>,
    pub bio: Option<String>,
    pub location: Option<String>,
    pub url: Option<String>,
    pub profile_image: Option<String>,
    pub banner_image: Option<String>,
    pub followers: Option<u64>,
    pub following: Option<u64>,
    pub posts_count: Option<u64>,
    pub likes_count: Option<u64>,
    pub listed_count: Option<u64>,
    pub created_at: Option<String>,
    pub verified: Option<bool>,
    pub blue_verified: Option<bool>,
}

#[derive(Debug, Clone, Serialize)]
pub struct XPostAuthorView {
    pub id: Option<String>,
    pub username: Option<String>,
    pub name: Option<String>,
    pub profile_image: Option<String>,
    pub blue_verified: Option<bool>,
}

#[derive(Debug, Clone, Serialize)]
pub struct XPostMentionView {
    pub username: Option<String>,
    pub name: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct XPostUrlView {
    pub url: Option<String>,
    pub display: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct XPostMediaView {
    pub url: Option<String>,
    #[serde(rename = "type")]
    pub media_type: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct XPostView {
    pub id: Option<String>,
    pub text: Option<String>,
    pub created_at: Option<String>,
    pub author: Option<XPostAuthorView>,
    pub reposts: Option<u64>,
    pub likes: Option<u64>,
    pub replies: Option<u64>,
    pub quotes: Option<u64>,
    pub views: Option<u64>,
    pub language: Option<String>,
    pub is_repost: Option<bool>,
    pub is_quote: Option<bool>,
    pub reply_to: Option<String>,
    pub conversation_id: Option<String>,
    pub hashtags: Option<Vec<String>>,
    pub mentions: Option<Vec<XPostMentionView>>,
    pub urls: Option<Vec<XPostUrlView>>,
    pub media: Option<Vec<XPostMediaView>>,
}

#[derive(Debug, Clone, Serialize)]
pub struct XPostsView {
    pub posts_count: usize,
    pub posts: Vec<XPostView>,
    pub cursor: Option<String>,
    pub has_more: bool,
}

#[derive(Debug, Clone, Serialize)]
pub struct XSearchResultsView {
    pub query: String,
    pub query_type: String,
    pub results_count: usize,
    pub posts: Vec<XPostView>,
    pub cursor: Option<String>,
    pub has_more: bool,
}

#[derive(Debug, Clone, Serialize)]
pub struct XTrendView {
    pub name: Option<String>,
    pub url: Option<String>,
    pub post_count: Option<u64>,
    pub description: Option<String>,
    pub category: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct XTrendsView {
    pub trends_count: usize,
    pub trends: Vec<XTrendView>,
}

// ============================================================================
// Formatters
// ============================================================================

pub fn format_user(user: &User) -> XUserView {
    XUserView {
        id: user.id.clone(),
        username: user.user_name.clone(),
        name: user.name.clone(),
        bio: user.description.clone(),
        location: user.location.clone(),
        url: user.url.clone(),
        profile_image: user.profile_image_url.clone(),
        banner_image: user.profile_banner_url.clone(),
        followers: user.followers_count,
        following: user.following_count,
        posts_count: user.statuses_count,
        likes_count: user.favourites_count,
        listed_count: user.listed_count,
        created_at: user.created_at.clone(),
        verified: user.verified,
        blue_verified: user.is_blue_verified,
    }
}

pub fn format_post(p: &Post) -> XPostView {
    XPostView {
        id: p.id.clone(),
        text: p.full_text.clone().or_else(|| p.text.clone()),
        created_at: p.created_at.clone(),
        author: p.author.as_ref().map(|a| XPostAuthorView {
            id: a.id.clone(),
            username: a.user_name.clone(),
            name: a.name.clone(),
            profile_image: a.profile_image_url.clone(),
            blue_verified: a.is_blue_verified,
        }),
        reposts: p.retweet_count,
        likes: p.favorite_count,
        replies: p.reply_count,
        quotes: p.quote_count,
        views: p.view_count,
        language: p.lang.clone(),
        is_repost: p.is_retweet,
        is_quote: p.is_quote,
        reply_to: p.in_reply_to_status_id.clone(),
        conversation_id: p.conversation_id.clone(),
        hashtags: p.hashtags.clone(),
        mentions: p.mentions.as_ref().map(|mentions| {
            mentions
                .iter()
                .map(|mention| XPostMentionView {
                    username: mention.user_name.clone(),
                    name: mention.name.clone(),
                })
                .collect()
        }),
        urls: p.urls.as_ref().map(|urls| {
            urls.iter()
                .map(|url| XPostUrlView {
                    url: url.expanded_url.clone(),
                    display: url.display_url.clone(),
                })
                .collect()
        }),
        media: p.media.as_ref().map(|media| {
            media
                .iter()
                .map(|item| XPostMediaView {
                    url: item.media_url_https.clone(),
                    media_type: item.media_type.clone(),
                })
                .collect()
        }),
    }
}

pub fn format_trend(trend: &Trend) -> XTrendView {
    XTrendView {
        name: trend.name.clone(),
        url: trend.url.clone(),
        post_count: trend.tweet_count,
        description: trend.description.clone(),
        category: trend.domain_context.clone(),
    }
}

// ============================================================================
// Custom deserializers
// ============================================================================

pub fn de_opt_string<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let value = Option::<Value>::deserialize(deserializer)?;
    Ok(match value {
        None | Some(Value::Null) => None,
        Some(Value::String(s)) => Some(s),
        Some(Value::Number(n)) => Some(n.to_string()),
        Some(Value::Bool(b)) => Some(b.to_string()),
        Some(other) => Some(other.to_string()),
    })
}

pub fn de_opt_u64<'de, D>(deserializer: D) -> Result<Option<u64>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let value = Option::<Value>::deserialize(deserializer)?;
    Ok(match value {
        None | Some(Value::Null) => None,
        Some(Value::Number(n)) => n.as_u64().or_else(|| {
            n.as_i64()
                .and_then(|v| if v >= 0 { Some(v as u64) } else { None })
        }),
        Some(Value::String(s)) => s.parse::<u64>().ok(),
        Some(Value::Bool(b)) => Some(if b { 1 } else { 0 }),
        _ => None,
    })
}
