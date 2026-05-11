#[allow(unused_imports)]
pub use progenitor_client::{ByteStream, ClientInfo, Error, ResponseValue};
#[allow(unused_imports)]
use progenitor_client::{ClientHooks, OperationInfo, RequestBuilderExt, encode_path};
/// Types used as operation parameters and responses.
#[allow(clippy::all)]
pub mod types {
    /// Error types.
    pub mod error {
        /// Error from a `TryFrom` or `FromStr` implementation.
        pub struct ConversionError(::std::borrow::Cow<'static, str>);
        impl ::std::error::Error for ConversionError {}
        impl ::std::fmt::Display for ConversionError {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> Result<(), ::std::fmt::Error> {
                ::std::fmt::Display::fmt(&self.0, f)
            }
        }
        impl ::std::fmt::Debug for ConversionError {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> Result<(), ::std::fmt::Error> {
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
    /**Opaque JSON response. The Kaito public docs do not currently
    publish per-field response shapes for /search, /trending, and
    /mindshare, so this spec passes those responses through as
    untyped objects. Refine when official docs are released.
    */
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Opaque JSON response. The Kaito public docs do not currently\npublish per-field response shapes for /search, /trending, and\n/mindshare, so this spec passes those responses through as\nuntyped objects. Refine when official docs are released.\n",
    ///  "type": "object",
    ///  "additionalProperties": true
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    #[serde(transparent)]
    pub struct JsonObject(pub ::serde_json::Map<::std::string::String, ::serde_json::Value>);
    impl ::std::ops::Deref for JsonObject {
        type Target = ::serde_json::Map<::std::string::String, ::serde_json::Value>;
        fn deref(&self) -> &::serde_json::Map<::std::string::String, ::serde_json::Value> {
            &self.0
        }
    }
    impl ::std::convert::From<JsonObject>
        for ::serde_json::Map<::std::string::String, ::serde_json::Value>
    {
        fn from(value: JsonObject) -> Self {
            value.0
        }
    }
    impl ::std::convert::From<::serde_json::Map<::std::string::String, ::serde_json::Value>>
        for JsonObject
    {
        fn from(value: ::serde_json::Map<::std::string::String, ::serde_json::Value>) -> Self {
            Self(value)
        }
    }
    /**Per-yapper attention score across rolling time windows, as
    documented in the Kaito Yaps Open Protocol.
    */
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Per-yapper attention score across rolling time windows, as\ndocumented in the Kaito Yaps Open Protocol.\n",
    ///  "type": "object",
    ///  "required": [
    ///    "user_id",
    ///    "username"
    ///  ],
    ///  "properties": {
    ///    "user_id": {
    ///      "description": "X account numeric user id.",
    ///      "type": "string"
    ///    },
    ///    "username": {
    ///      "description": "X account handle (without @).",
    ///      "type": "string"
    ///    },
    ///    "yaps_all": {
    ///      "description": "All-time yaps score.",
    ///      "type": "number"
    ///    },
    ///    "yaps_l12m": {
    ///      "description": "Yaps score over the last 12 months.",
    ///      "type": "number"
    ///    },
    ///    "yaps_l24h": {
    ///      "description": "Yaps score over the last 24 hours.",
    ///      "type": "number"
    ///    },
    ///    "yaps_l30d": {
    ///      "description": "Yaps score over the last 30 days.",
    ///      "type": "number"
    ///    },
    ///    "yaps_l3m": {
    ///      "description": "Yaps score over the last 3 months.",
    ///      "type": "number"
    ///    },
    ///    "yaps_l48h": {
    ///      "description": "Yaps score over the last 48 hours.",
    ///      "type": "number"
    ///    },
    ///    "yaps_l6m": {
    ///      "description": "Yaps score over the last 6 months.",
    ///      "type": "number"
    ///    },
    ///    "yaps_l7d": {
    ///      "description": "Yaps score over the last 7 days.",
    ///      "type": "number"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct YapsScore {
        ///X account numeric user id.
        pub user_id: ::std::string::String,
        ///X account handle (without @).
        pub username: ::std::string::String,
        ///All-time yaps score.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub yaps_all: ::std::option::Option<f64>,
        ///Yaps score over the last 12 months.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub yaps_l12m: ::std::option::Option<f64>,
        ///Yaps score over the last 24 hours.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub yaps_l24h: ::std::option::Option<f64>,
        ///Yaps score over the last 30 days.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub yaps_l30d: ::std::option::Option<f64>,
        ///Yaps score over the last 3 months.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub yaps_l3m: ::std::option::Option<f64>,
        ///Yaps score over the last 48 hours.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub yaps_l48h: ::std::option::Option<f64>,
        ///Yaps score over the last 6 months.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub yaps_l6m: ::std::option::Option<f64>,
        ///Yaps score over the last 7 days.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub yaps_l7d: ::std::option::Option<f64>,
    }
}
#[derive(Clone, Debug)]
/**Client for Kaito AI Public API

HTTP API for Kaito AI — a vertical search engine and InfoFi network
indexing Web3 sources (Twitter/X yaps, Discord, Telegram, governance
forums, Farcaster, podcasts, conference transcripts) with attention
quantification ("mindshare" / "yaps").

This spec covers only the surface consumed by the `apps/kaito` Aomi
app crate. The `/yaps` endpoint is documented in the public Kaito
Yaps Open Protocol reference; the `/search`, `/trending`, and
`/mindshare/{token}` endpoints are part of the Kaito API used by the
crate (per-field response shapes are not publicly published, so those
responses pass through as opaque JSON).

## Auth
Bearer token in the `Authorization` header. The crate reads the key
from the `KAITO_API_KEY` environment variable (or per-call argument)
and sets `Authorization: Bearer <key>` on every request. The spec
declares this via `securitySchemes.bearerAuth` and applies it
globally; the auth wiring itself is plain HTTP (no signing).

## Loose schemas
`/search`, `/trending`, and `/mindshare/{token}` decode as
`additionalProperties: true` JSON because Kaito does not publish
per-field shapes for these. `/yaps` has a documented response shape
and is typed concretely.


Version: 0.2.0*/
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
            reqwest::ClientBuilder::new()
                .connect_timeout(dur)
                .timeout(dur)
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
        "0.2.0"
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
    /**Get a yapper's score (Yaps Open Protocol)

    Return the per-yapper attention score ("yaps") for an X/Twitter
    account. Pass either `user_id` (the X numeric account id —
    recommended) or `username` (the X handle without @). At least one
    is required. Default rate limit is 100 calls per 5 minutes.


    Sends a `GET` request to `/yaps`

    Arguments:
    - `user_id`: X account numeric user id (preferred over username).
    - `username`: X account handle (without @). Used when user_id is not provided.
    */
    pub async fn get_yapper_score<'a>(
        &'a self,
        user_id: Option<&'a str>,
        username: Option<&'a str>,
    ) -> Result<ResponseValue<types::YapsScore>, Error<()>> {
        let url = format!("{}/yaps", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
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
            .query(&progenitor_client::QueryParam::new("user_id", &user_id))
            .query(&progenitor_client::QueryParam::new("username", &username))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "get_yapper_score",
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
    /**Semantic search across Kaito's Web3 corpus

    Run a semantic search across indexed Web3 sources (Twitter/X
    yaps, Discord, Telegram, governance forums, Farcaster, podcasts,
    conference transcripts). Useful for finding what specific yappers
    or communities are saying about a topic. Results are
    AI-structured with attention quantification.


    Sends a `GET` request to `/search`

    Arguments:
    - `limit`: Maximum number of results to return.
    - `q`: Search query string.
    - `source_type`: Filter by source category (e.g. twitter, discord, telegram,
    farcaster, governance). Free-form string; not constrained to
    an enum because the upstream set may grow.

    */
    pub async fn search_corpus<'a>(
        &'a self,
        limit: Option<::std::num::NonZeroU32>,
        q: &'a str,
        source_type: Option<&'a str>,
    ) -> Result<ResponseValue<types::JsonObject>, Error<()>> {
        let url = format!("{}/search", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
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
            .query(&progenitor_client::QueryParam::new("limit", &limit))
            .query(&progenitor_client::QueryParam::new("q", &q))
            .query(&progenitor_client::QueryParam::new(
                "source_type",
                &source_type,
            ))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "search_corpus",
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
    /**Get trending narratives

    Return the topics and narratives currently trending across
    Kaito's indexed Web3 sources — what yappers are collectively
    focused on right now.


    Sends a `GET` request to `/trending`

    Arguments:
    - `limit`: Maximum number of trending entries to return.
    */
    pub async fn get_trending_narratives<'a>(
        &'a self,
        limit: Option<::std::num::NonZeroU32>,
    ) -> Result<ResponseValue<types::JsonObject>, Error<()>> {
        let url = format!("{}/trending", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
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
            .query(&progenitor_client::QueryParam::new("limit", &limit))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "get_trending_narratives",
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
    /**Get attention/mindshare metrics for a token

    Return attention and discussion-volume metrics for the given
    token symbol or name (e.g. BTC, ETH, SOL).


    Sends a `GET` request to `/mindshare/{token}`

    Arguments:
    - `token`: Token symbol or name to query mindshare for.
    */
    pub async fn get_token_mindshare<'a>(
        &'a self,
        token: &'a str,
    ) -> Result<ResponseValue<types::JsonObject>, Error<()>> {
        let url = format!(
            "{}/mindshare/{}",
            self.baseurl,
            encode_path(&token.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
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
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "get_token_mindshare",
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
