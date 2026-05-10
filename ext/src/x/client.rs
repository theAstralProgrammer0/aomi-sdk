#[allow(unused_imports)]
pub use progenitor_client::{ByteStream, ClientInfo, Error, ResponseValue};
#[allow(unused_imports)]
use progenitor_client::{encode_path, ClientHooks, OperationInfo, RequestBuilderExt};
/// Types used as operation parameters and responses.
#[allow(clippy::all)]
pub mod types {
    /// Error types.
    pub mod error {
        /// Error from a `TryFrom` or `FromStr` implementation.
        pub struct ConversionError(::std::borrow::Cow<'static, str>);
        impl ::std::error::Error for ConversionError {}
        impl ::std::fmt::Display for ConversionError {
            fn fmt(
                &self,
                f: &mut ::std::fmt::Formatter<'_>,
            ) -> Result<(), ::std::fmt::Error> {
                ::std::fmt::Display::fmt(&self.0, f)
            }
        }
        impl ::std::fmt::Debug for ConversionError {
            fn fmt(
                &self,
                f: &mut ::std::fmt::Formatter<'_>,
            ) -> Result<(), ::std::fmt::Error> {
                ::std::fmt::Debug::fmt(&self.0, f)
            }
        }
        impl From<&'static str> for ConversionError {
            fn from(value: &'static str) -> Self {
                Self(value.into())
            }
        }
        impl From<String> for ConversionError {
            fn from(value: String) -> Self {
                Self(value.into())
            }
        }
    }
    ///`Mention`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "name": {
    ///      "type": "string"
    ///    },
    ///    "userName": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct Mention {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub name: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "userName",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub user_name: ::std::option::Option<::std::string::String>,
    }
    impl ::std::default::Default for Mention {
        fn default() -> Self {
            Self {
                name: Default::default(),
                user_name: Default::default(),
            }
        }
    }
    ///`Post`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "author": {
    ///      "$ref": "#/components/schemas/PostAuthor"
    ///    },
    ///    "conversationId": {
    ///      "type": "string"
    ///    },
    ///    "createdAt": {
    ///      "type": "string"
    ///    },
    ///    "favoriteCount": {
    ///      "type": "integer",
    ///      "format": "int64"
    ///    },
    ///    "fullText": {
    ///      "type": "string"
    ///    },
    ///    "hashtags": {
    ///      "type": "array",
    ///      "items": {
    ///        "type": "string"
    ///      }
    ///    },
    ///    "id": {
    ///      "type": "string"
    ///    },
    ///    "inReplyToStatusId": {
    ///      "type": "string"
    ///    },
    ///    "isQuote": {
    ///      "type": "boolean"
    ///    },
    ///    "isRetweet": {
    ///      "type": "boolean"
    ///    },
    ///    "lang": {
    ///      "type": "string"
    ///    },
    ///    "mentions": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/Mention"
    ///      }
    ///    },
    ///    "quoteCount": {
    ///      "type": "integer",
    ///      "format": "int64"
    ///    },
    ///    "replyCount": {
    ///      "type": "integer",
    ///      "format": "int64"
    ///    },
    ///    "retweetCount": {
    ///      "type": "integer",
    ///      "format": "int64"
    ///    },
    ///    "text": {
    ///      "type": "string"
    ///    },
    ///    "urls": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/UrlEntity"
    ///      }
    ///    },
    ///    "viewCount": {
    ///      "type": "integer",
    ///      "format": "int64"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct Post {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub author: ::std::option::Option<PostAuthor>,
        #[serde(
            rename = "conversationId",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub conversation_id: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "createdAt",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub created_at: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "favoriteCount",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub favorite_count: ::std::option::Option<i64>,
        #[serde(
            rename = "fullText",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub full_text: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub hashtags: ::std::vec::Vec<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub id: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "inReplyToStatusId",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub in_reply_to_status_id: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "isQuote",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub is_quote: ::std::option::Option<bool>,
        #[serde(
            rename = "isRetweet",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub is_retweet: ::std::option::Option<bool>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub lang: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub mentions: ::std::vec::Vec<Mention>,
        #[serde(
            rename = "quoteCount",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub quote_count: ::std::option::Option<i64>,
        #[serde(
            rename = "replyCount",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub reply_count: ::std::option::Option<i64>,
        #[serde(
            rename = "retweetCount",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub retweet_count: ::std::option::Option<i64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub text: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub urls: ::std::vec::Vec<UrlEntity>,
        #[serde(
            rename = "viewCount",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub view_count: ::std::option::Option<i64>,
    }
    impl ::std::default::Default for Post {
        fn default() -> Self {
            Self {
                author: Default::default(),
                conversation_id: Default::default(),
                created_at: Default::default(),
                favorite_count: Default::default(),
                full_text: Default::default(),
                hashtags: Default::default(),
                id: Default::default(),
                in_reply_to_status_id: Default::default(),
                is_quote: Default::default(),
                is_retweet: Default::default(),
                lang: Default::default(),
                mentions: Default::default(),
                quote_count: Default::default(),
                reply_count: Default::default(),
                retweet_count: Default::default(),
                text: Default::default(),
                urls: Default::default(),
                view_count: Default::default(),
            }
        }
    }
    ///`PostAuthor`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "id": {
    ///      "type": "string"
    ///    },
    ///    "name": {
    ///      "type": "string"
    ///    },
    ///    "userName": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct PostAuthor {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub id: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub name: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "userName",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub user_name: ::std::option::Option<::std::string::String>,
    }
    impl ::std::default::Default for PostAuthor {
        fn default() -> Self {
            Self {
                id: Default::default(),
                name: Default::default(),
                user_name: Default::default(),
            }
        }
    }
    ///`PostResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "data": {
    ///      "$ref": "#/components/schemas/Post"
    ///    },
    ///    "msg": {
    ///      "type": "string"
    ///    },
    ///    "success": {
    ///      "type": "boolean"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct PostResponse {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub data: ::std::option::Option<Post>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub msg: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub success: ::std::option::Option<bool>,
    }
    impl ::std::default::Default for PostResponse {
        fn default() -> Self {
            Self {
                data: Default::default(),
                msg: Default::default(),
                success: Default::default(),
            }
        }
    }
    ///`PostsData`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "has_next_page": {
    ///      "type": "boolean"
    ///    },
    ///    "next_cursor": {
    ///      "type": "string"
    ///    },
    ///    "tweets": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/Post"
    ///      }
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct PostsData {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub has_next_page: ::std::option::Option<bool>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub next_cursor: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub tweets: ::std::vec::Vec<Post>,
    }
    impl ::std::default::Default for PostsData {
        fn default() -> Self {
            Self {
                has_next_page: Default::default(),
                next_cursor: Default::default(),
                tweets: Default::default(),
            }
        }
    }
    ///`PostsResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "data": {
    ///      "$ref": "#/components/schemas/PostsData"
    ///    },
    ///    "msg": {
    ///      "type": "string"
    ///    },
    ///    "success": {
    ///      "type": "boolean"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct PostsResponse {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub data: ::std::option::Option<PostsData>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub msg: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub success: ::std::option::Option<bool>,
    }
    impl ::std::default::Default for PostsResponse {
        fn default() -> Self {
            Self {
                data: Default::default(),
                msg: Default::default(),
                success: Default::default(),
            }
        }
    }
    ///`Trend`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "description": {
    ///      "type": "string"
    ///    },
    ///    "domainContext": {
    ///      "type": "string"
    ///    },
    ///    "name": {
    ///      "type": "string"
    ///    },
    ///    "tweetCount": {
    ///      "type": "integer",
    ///      "format": "int64"
    ///    },
    ///    "url": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct Trend {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub description: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "domainContext",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub domain_context: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub name: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "tweetCount",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub tweet_count: ::std::option::Option<i64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub url: ::std::option::Option<::std::string::String>,
    }
    impl ::std::default::Default for Trend {
        fn default() -> Self {
            Self {
                description: Default::default(),
                domain_context: Default::default(),
                name: Default::default(),
                tweet_count: Default::default(),
                url: Default::default(),
            }
        }
    }
    ///`TrendsData`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "trends": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/Trend"
    ///      }
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct TrendsData {
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub trends: ::std::vec::Vec<Trend>,
    }
    impl ::std::default::Default for TrendsData {
        fn default() -> Self {
            Self { trends: Default::default() }
        }
    }
    ///`TrendsResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "data": {
    ///      "$ref": "#/components/schemas/TrendsData"
    ///    },
    ///    "msg": {
    ///      "type": "string"
    ///    },
    ///    "success": {
    ///      "type": "boolean"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct TrendsResponse {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub data: ::std::option::Option<TrendsData>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub msg: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub success: ::std::option::Option<bool>,
    }
    impl ::std::default::Default for TrendsResponse {
        fn default() -> Self {
            Self {
                data: Default::default(),
                msg: Default::default(),
                success: Default::default(),
            }
        }
    }
    ///`UrlEntity`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "displayUrl": {
    ///      "type": "string"
    ///    },
    ///    "expandedUrl": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct UrlEntity {
        #[serde(
            rename = "displayUrl",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub display_url: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "expandedUrl",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub expanded_url: ::std::option::Option<::std::string::String>,
    }
    impl ::std::default::Default for UrlEntity {
        fn default() -> Self {
            Self {
                display_url: Default::default(),
                expanded_url: Default::default(),
            }
        }
    }
    ///`UserPayload`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "createdAt": {
    ///      "type": "string"
    ///    },
    ///    "description": {
    ///      "type": "string"
    ///    },
    ///    "favouritesCount": {
    ///      "type": "integer",
    ///      "format": "int64"
    ///    },
    ///    "followersCount": {
    ///      "type": "integer",
    ///      "format": "int64"
    ///    },
    ///    "followingCount": {
    ///      "type": "integer",
    ///      "format": "int64"
    ///    },
    ///    "id": {
    ///      "type": "string"
    ///    },
    ///    "listedCount": {
    ///      "type": "integer",
    ///      "format": "int64"
    ///    },
    ///    "location": {
    ///      "type": "string"
    ///    },
    ///    "name": {
    ///      "type": "string"
    ///    },
    ///    "statusesCount": {
    ///      "type": "integer",
    ///      "format": "int64"
    ///    },
    ///    "url": {
    ///      "type": "string"
    ///    },
    ///    "userName": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct UserPayload {
        #[serde(
            rename = "createdAt",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub created_at: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub description: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "favouritesCount",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub favourites_count: ::std::option::Option<i64>,
        #[serde(
            rename = "followersCount",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub followers_count: ::std::option::Option<i64>,
        #[serde(
            rename = "followingCount",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub following_count: ::std::option::Option<i64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub id: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "listedCount",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub listed_count: ::std::option::Option<i64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub location: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub name: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "statusesCount",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub statuses_count: ::std::option::Option<i64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub url: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "userName",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub user_name: ::std::option::Option<::std::string::String>,
    }
    impl ::std::default::Default for UserPayload {
        fn default() -> Self {
            Self {
                created_at: Default::default(),
                description: Default::default(),
                favourites_count: Default::default(),
                followers_count: Default::default(),
                following_count: Default::default(),
                id: Default::default(),
                listed_count: Default::default(),
                location: Default::default(),
                name: Default::default(),
                statuses_count: Default::default(),
                url: Default::default(),
                user_name: Default::default(),
            }
        }
    }
    ///`UserResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "data": {
    ///      "$ref": "#/components/schemas/UserPayload"
    ///    },
    ///    "msg": {
    ///      "type": "string"
    ///    },
    ///    "success": {
    ///      "type": "boolean"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct UserResponse {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub data: ::std::option::Option<UserPayload>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub msg: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub success: ::std::option::Option<bool>,
    }
    impl ::std::default::Default for UserResponse {
        fn default() -> Self {
            Self {
                data: Default::default(),
                msg: Default::default(),
                success: Default::default(),
            }
        }
    }
}
#[derive(Clone, Debug)]
/**Client for twitterapi.io X Read API

Subset of the twitterapi.io read-only X (Twitter) API covering the
endpoints used by the Aomi `x` app. twitterapi.io is a third-party
reseller — NOT the official X v2 API.

## Auth
Every request requires the `X-API-Key` header. The codegen describes the
header via the `apiKeyAuth` security scheme; the per-request value is set
by the caller through a shared `reqwest::Client` with default headers.


Version: 0.1.0*/
pub struct Client {
    pub(crate) baseurl: String,
    pub(crate) client: reqwest::Client,
}
impl Client {
    /// Create a new client.
    ///
    /// `baseurl` is the base URL provided to the internal
    /// `reqwest::Client`, and should include a scheme and hostname,
    /// as well as port and a path stem if applicable.
    pub fn new(baseurl: &str) -> Self {
        #[cfg(not(target_arch = "wasm32"))]
        let client = {
            let dur = ::std::time::Duration::from_secs(15u64);
            reqwest::ClientBuilder::new().connect_timeout(dur).timeout(dur)
        };
        #[cfg(target_arch = "wasm32")]
        let client = reqwest::ClientBuilder::new();
        Self::new_with_client(baseurl, client.build().unwrap())
    }
    /// Construct a new client with an existing `reqwest::Client`,
    /// allowing more control over its configuration.
    ///
    /// `baseurl` is the base URL provided to the internal
    /// `reqwest::Client`, and should include a scheme and hostname,
    /// as well as port and a path stem if applicable.
    pub fn new_with_client(baseurl: &str, client: reqwest::Client) -> Self {
        Self {
            baseurl: baseurl.to_string(),
            client,
        }
    }
}
impl ClientInfo<()> for Client {
    fn api_version() -> &'static str {
        "0.1.0"
    }
    fn baseurl(&self) -> &str {
        self.baseurl.as_str()
    }
    fn client(&self) -> &reqwest::Client {
        &self.client
    }
    fn inner(&self) -> &() {
        &()
    }
}
impl ClientHooks<()> for &Client {}
#[allow(clippy::all)]
impl Client {
    /**Look up a user by handle

Sends a `GET` request to `/twitter/user/info`

*/
    pub async fn get_user_info<'a>(
        &'a self,
        user_name: &'a str,
    ) -> Result<ResponseValue<types::UserResponse>, Error<()>> {
        let url = format!("{}/twitter/user/info", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map
            .append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(Self::api_version()),
            );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new("userName", &user_name))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "get_user_info",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Recent posts from a single user

Sends a `GET` request to `/twitter/user/last_tweets`

*/
    pub async fn get_user_last_tweets<'a>(
        &'a self,
        cursor: Option<&'a str>,
        user_name: &'a str,
    ) -> Result<ResponseValue<types::PostsResponse>, Error<()>> {
        let url = format!("{}/twitter/user/last_tweets", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map
            .append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(Self::api_version()),
            );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new("cursor", &cursor))
            .query(&progenitor_client::QueryParam::new("userName", &user_name))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "get_user_last_tweets",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Advanced search across posts

Sends a `GET` request to `/twitter/tweet/advanced_search`

*/
    pub async fn search_tweets<'a>(
        &'a self,
        cursor: Option<&'a str>,
        query: &'a str,
        query_type: &'a str,
    ) -> Result<ResponseValue<types::PostsResponse>, Error<()>> {
        let url = format!("{}/twitter/tweet/advanced_search", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map
            .append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(Self::api_version()),
            );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new("cursor", &cursor))
            .query(&progenitor_client::QueryParam::new("query", &query))
            .query(&progenitor_client::QueryParam::new("queryType", &query_type))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "search_tweets",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Trending topics

Sends a `GET` request to `/twitter/trends`

*/
    pub async fn get_trends<'a>(
        &'a self,
        count: Option<i64>,
        woeid: Option<i64>,
    ) -> Result<ResponseValue<types::TrendsResponse>, Error<()>> {
        let url = format!("{}/twitter/trends", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map
            .append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(Self::api_version()),
            );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new("count", &count))
            .query(&progenitor_client::QueryParam::new("woeid", &woeid))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "get_trends",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**One post by id

Sends a `GET` request to `/twitter/tweet/info`

*/
    pub async fn get_tweet_info<'a>(
        &'a self,
        tweet_id: &'a str,
    ) -> Result<ResponseValue<types::PostResponse>, Error<()>> {
        let url = format!("{}/twitter/tweet/info", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map
            .append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(Self::api_version()),
            );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new("tweetId", &tweet_id))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "get_tweet_info",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
}
/// Items consumers will typically use such as the Client.
pub mod prelude {
    #[allow(unused_imports)]
    pub use super::Client;
}
