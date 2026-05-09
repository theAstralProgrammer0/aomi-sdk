use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct UserByUsernameQuery<'a> {
    pub username: &'a str,
}

#[derive(Debug, Clone, Serialize)]
pub struct SearchUsersQuery<'a> {
    pub q: &'a str,
}

#[derive(Debug, Clone, Serialize)]
pub struct FeedQuery<'a> {
    #[serde(rename = "feed_type")]
    pub feed_type: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fid: Option<u64>,
    pub limit: u32,
}

#[derive(Debug, Clone, Serialize)]
pub struct CastLookupQuery<'a> {
    pub identifier: &'a str,
    #[serde(rename = "type")]
    pub id_type: &'a str,
}

#[derive(Debug, Clone, Serialize)]
pub struct SearchCastsQuery<'a> {
    pub q: &'a str,
    pub limit: u32,
}

#[derive(Debug, Clone, Serialize)]
pub struct PublishCastRequest {
    pub signer_uuid: String,
    pub text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub embeds: Option<Vec<UrlEmbed>>,
}

#[derive(Debug, Clone, Serialize)]
pub struct UrlEmbed {
    pub url: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct ChannelQuery<'a> {
    pub id: &'a str,
}

#[derive(Debug, Clone, Serialize)]
pub struct TrendingFeedQuery<'a> {
    pub limit: u32,
    pub time_window: &'a str,
}
