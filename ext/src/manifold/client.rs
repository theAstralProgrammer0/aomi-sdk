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
    ///`CreateMarketRequest`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "initialProb",
    ///    "outcomeType",
    ///    "question"
    ///  ],
    ///  "properties": {
    ///    "closeTime": {
    ///      "type": "integer",
    ///      "format": "int64"
    ///    },
    ///    "initialProb": {
    ///      "type": "integer",
    ///      "format": "int32",
    ///      "maximum": 99.0,
    ///      "minimum": 1.0
    ///    },
    ///    "outcomeType": {
    ///      "description": "Currently only BINARY is supported by this client.",
    ///      "type": "string"
    ///    },
    ///    "question": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct CreateMarketRequest {
        #[serde(
            rename = "closeTime",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub close_time: ::std::option::Option<i64>,
        #[serde(rename = "initialProb")]
        pub initial_prob: ::std::num::NonZeroU32,
        ///Currently only BINARY is supported by this client.
        #[serde(rename = "outcomeType")]
        pub outcome_type: ::std::string::String,
        pub question: ::std::string::String,
    }
    ///`CreateMarketResponse`
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
    ///    "question": {
    ///      "type": "string"
    ///    },
    ///    "slug": {
    ///      "type": "string"
    ///    },
    ///    "url": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct CreateMarketResponse {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub id: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub question: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub slug: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub url: ::std::option::Option<::std::string::String>,
    }
    impl ::std::default::Default for CreateMarketResponse {
        fn default() -> Self {
            Self {
                id: Default::default(),
                question: Default::default(),
                slug: Default::default(),
                url: Default::default(),
            }
        }
    }
    ///`FullMarket`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "closeTime": {
    ///      "type": "integer",
    ///      "format": "int64"
    ///    },
    ///    "createdTime": {
    ///      "type": "integer",
    ///      "format": "int64"
    ///    },
    ///    "creatorName": {
    ///      "type": "string"
    ///    },
    ///    "id": {
    ///      "type": "string"
    ///    },
    ///    "isResolved": {
    ///      "type": "boolean"
    ///    },
    ///    "mechanism": {
    ///      "type": "string"
    ///    },
    ///    "outcomeType": {
    ///      "type": "string"
    ///    },
    ///    "probability": {
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "question": {
    ///      "type": "string"
    ///    },
    ///    "resolution": {
    ///      "type": "string"
    ///    },
    ///    "textDescription": {
    ///      "type": "string"
    ///    },
    ///    "totalLiquidity": {
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "url": {
    ///      "type": "string"
    ///    },
    ///    "volume": {
    ///      "type": "number",
    ///      "format": "double"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct FullMarket {
        #[serde(
            rename = "closeTime",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub close_time: ::std::option::Option<i64>,
        #[serde(
            rename = "createdTime",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub created_time: ::std::option::Option<i64>,
        #[serde(
            rename = "creatorName",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub creator_name: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub id: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "isResolved",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub is_resolved: ::std::option::Option<bool>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub mechanism: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "outcomeType",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub outcome_type: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub probability: ::std::option::Option<f64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub question: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub resolution: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "textDescription",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub text_description: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "totalLiquidity",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub total_liquidity: ::std::option::Option<f64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub url: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub volume: ::std::option::Option<f64>,
    }
    impl ::std::default::Default for FullMarket {
        fn default() -> Self {
            Self {
                close_time: Default::default(),
                created_time: Default::default(),
                creator_name: Default::default(),
                id: Default::default(),
                is_resolved: Default::default(),
                mechanism: Default::default(),
                outcome_type: Default::default(),
                probability: Default::default(),
                question: Default::default(),
                resolution: Default::default(),
                text_description: Default::default(),
                total_liquidity: Default::default(),
                url: Default::default(),
                volume: Default::default(),
            }
        }
    }
    ///`LiteMarket`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "closeTime": {
    ///      "type": "integer",
    ///      "format": "int64"
    ///    },
    ///    "createdTime": {
    ///      "type": "integer",
    ///      "format": "int64"
    ///    },
    ///    "id": {
    ///      "type": "string"
    ///    },
    ///    "isResolved": {
    ///      "type": "boolean"
    ///    },
    ///    "probability": {
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "question": {
    ///      "type": "string"
    ///    },
    ///    "url": {
    ///      "type": "string"
    ///    },
    ///    "volume": {
    ///      "type": "number",
    ///      "format": "double"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct LiteMarket {
        #[serde(
            rename = "closeTime",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub close_time: ::std::option::Option<i64>,
        #[serde(
            rename = "createdTime",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub created_time: ::std::option::Option<i64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub id: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "isResolved",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub is_resolved: ::std::option::Option<bool>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub probability: ::std::option::Option<f64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub question: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub url: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub volume: ::std::option::Option<f64>,
    }
    impl ::std::default::Default for LiteMarket {
        fn default() -> Self {
            Self {
                close_time: Default::default(),
                created_time: Default::default(),
                id: Default::default(),
                is_resolved: Default::default(),
                probability: Default::default(),
                question: Default::default(),
                url: Default::default(),
                volume: Default::default(),
            }
        }
    }
    ///`PlaceBetRequest`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "amount",
    ///    "contractId",
    ///    "outcome"
    ///  ],
    ///  "properties": {
    ///    "amount": {
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "contractId": {
    ///      "type": "string"
    ///    },
    ///    "outcome": {
    ///      "description": "YES or NO for binary markets.",
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct PlaceBetRequest {
        pub amount: f64,
        #[serde(rename = "contractId")]
        pub contract_id: ::std::string::String,
        ///YES or NO for binary markets.
        pub outcome: ::std::string::String,
    }
    ///`PlaceBetResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "amount": {
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "betId": {
    ///      "type": "string"
    ///    },
    ///    "contractId": {
    ///      "type": "string"
    ///    },
    ///    "outcome": {
    ///      "type": "string"
    ///    },
    ///    "probAfter": {
    ///      "type": "number",
    ///      "format": "double"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct PlaceBetResponse {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub amount: ::std::option::Option<f64>,
        #[serde(
            rename = "betId",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub bet_id: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "contractId",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub contract_id: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub outcome: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "probAfter",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub prob_after: ::std::option::Option<f64>,
    }
    impl ::std::default::Default for PlaceBetResponse {
        fn default() -> Self {
            Self {
                amount: Default::default(),
                bet_id: Default::default(),
                contract_id: Default::default(),
                outcome: Default::default(),
                prob_after: Default::default(),
            }
        }
    }
}
#[derive(Clone, Debug)]
/**Client for Manifold Markets API

Subset of the Manifold Markets v0 REST API covering the endpoints used by
the Aomi Manifold app.

## Auth
Read endpoints (markets, search, positions) are public. Write endpoints
(POST /bet, POST /market) require an API key sent as
`Authorization: Key <api_key>`. The codegen describes the header via the
`apiKeyAuth` security scheme.


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
    /**List markets (newest / hottest)

    Sends a `GET` request to `/markets`

    */
    pub async fn list_markets<'a>(
        &'a self,
        limit: Option<i32>,
        sort: Option<&'a str>,
        topics: Option<&'a str>,
    ) -> Result<ResponseValue<::std::vec::Vec<types::LiteMarket>>, Error<()>> {
        let url = format!("{}/markets", self.baseurl,);
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
            .query(&progenitor_client::QueryParam::new("sort", &sort))
            .query(&progenitor_client::QueryParam::new("topics", &topics))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "list_markets",
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
    /**Keyword search across markets

    Sends a `GET` request to `/search-markets`

    */
    pub async fn search_markets<'a>(
        &'a self,
        filter: Option<&'a str>,
        sort: Option<&'a str>,
        term: &'a str,
    ) -> Result<ResponseValue<::std::vec::Vec<types::LiteMarket>>, Error<()>> {
        let url = format!("{}/search-markets", self.baseurl,);
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
            .query(&progenitor_client::QueryParam::new("filter", &filter))
            .query(&progenitor_client::QueryParam::new("sort", &sort))
            .query(&progenitor_client::QueryParam::new("term", &term))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "search_markets",
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
    /**Get a market by id or slug

    Sends a `GET` request to `/market/{idOrSlug}`

    */
    pub async fn get_market<'a>(
        &'a self,
        id_or_slug: &'a str,
    ) -> Result<ResponseValue<types::FullMarket>, Error<()>> {
        let url = format!(
            "{}/market/{}",
            self.baseurl,
            encode_path(&id_or_slug.to_string()),
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
            operation_id: "get_market",
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
    /**Get top positions for a market

    Sends a `GET` request to `/market/{idOrSlug}/positions`

    */
    pub async fn get_market_positions<'a>(
        &'a self,
        id_or_slug: &'a str,
    ) -> Result<
        ResponseValue<
            ::std::vec::Vec<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
        >,
        Error<()>,
    > {
        let url = format!(
            "{}/market/{}/positions",
            self.baseurl,
            encode_path(&id_or_slug.to_string()),
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
            operation_id: "get_market_positions",
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
    /**Place a bet on a binary market

    Sends a `POST` request to `/bet`

    */
    pub async fn place_bet<'a>(
        &'a self,
        body: &'a types::PlaceBetRequest,
    ) -> Result<ResponseValue<types::PlaceBetResponse>, Error<()>> {
        let url = format!("{}/bet", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "place_bet",
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
    /**Create a new market

    Sends a `POST` request to `/market`

    */
    pub async fn create_market<'a>(
        &'a self,
        body: &'a types::CreateMarketRequest,
    ) -> Result<ResponseValue<types::CreateMarketResponse>, Error<()>> {
        let url = format!("{}/market", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "create_market",
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
