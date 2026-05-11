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
    ///`CandlesResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "candles": {
    ///      "type": "array",
    ///      "items": {
    ///        "type": "object",
    ///        "additionalProperties": true
    ///      }
    ///    }
    ///  },
    ///  "additionalProperties": true
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct CandlesResponse {
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub candles: ::std::vec::Vec<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
    }
    impl ::std::default::Default for CandlesResponse {
        fn default() -> Self {
            Self {
                candles: Default::default(),
            }
        }
    }
    ///`OrderbookLevel`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "price": {
    ///      "type": "string"
    ///    },
    ///    "size": {
    ///      "type": "string"
    ///    }
    ///  },
    ///  "additionalProperties": true
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct OrderbookLevel {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub price: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub size: ::std::option::Option<::std::string::String>,
    }
    impl ::std::default::Default for OrderbookLevel {
        fn default() -> Self {
            Self {
                price: Default::default(),
                size: Default::default(),
            }
        }
    }
    ///`OrderbookResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "asks": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/OrderbookLevel"
    ///      }
    ///    },
    ///    "bids": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/OrderbookLevel"
    ///      }
    ///    }
    ///  },
    ///  "additionalProperties": true
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct OrderbookResponse {
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub asks: ::std::vec::Vec<OrderbookLevel>,
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub bids: ::std::vec::Vec<OrderbookLevel>,
    }
    impl ::std::default::Default for OrderbookResponse {
        fn default() -> Self {
            Self {
                asks: Default::default(),
                bids: Default::default(),
            }
        }
    }
    ///`PerpetualMarketsResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "markets": {
    ///      "type": "object",
    ///      "additionalProperties": true
    ///    }
    ///  },
    ///  "additionalProperties": true
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct PerpetualMarketsResponse {
        #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
        pub markets: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    }
    impl ::std::default::Default for PerpetualMarketsResponse {
        fn default() -> Self {
            Self {
                markets: Default::default(),
            }
        }
    }
}
#[derive(Clone, Debug)]
/**Client for dYdX v4 Indexer API

Read-only public Indexer API for dYdX v4 (Cosmos-based perpetual futures DEX).

## Auth
All endpoints in this spec are unauthenticated. Trades / orders are placed
via signed Cosmos transactions and are explicitly out of scope.

## Notes
Response bodies are typed loosely (`additionalProperties: true`) for fields
that the upstream docs do not strictly schematize — the curated tool layer
surfaces the JSON to callers verbatim, so partial typing is intentional.


Version: 1.0*/
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
        "1.0"
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
    /**List all perpetual markets, optionally filtered by ticker

    Sends a `GET` request to `/perpetualMarkets`

    Arguments:
    - `ticker`: Optional ticker filter (e.g. "BTC-USD").
    */
    pub async fn get_perpetual_markets<'a>(
        &'a self,
        ticker: Option<&'a str>,
    ) -> Result<ResponseValue<types::PerpetualMarketsResponse>, Error<()>> {
        let url = format!("{}/perpetualMarkets", self.baseurl,);
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
            .query(&progenitor_client::QueryParam::new("ticker", &ticker))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "get_perpetual_markets",
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
    /**L2 orderbook snapshot for a perpetual market

    Sends a `GET` request to `/orderbooks/perpetualMarket/{ticker}`

    Arguments:
    - `ticker`: Market ticker (e.g. "BTC-USD").
    */
    pub async fn get_orderbook<'a>(
        &'a self,
        ticker: &'a str,
    ) -> Result<ResponseValue<types::OrderbookResponse>, Error<()>> {
        let url = format!(
            "{}/orderbooks/perpetualMarket/{}",
            self.baseurl,
            encode_path(&ticker.to_string()),
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
            operation_id: "get_orderbook",
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
    /**OHLCV candles for a perpetual market

    Sends a `GET` request to `/candles/perpetualMarkets/{ticker}`

    Arguments:
    - `ticker`: Market ticker (e.g. "BTC-USD").
    - `limit`
    - `resolution`: One of 1MIN, 5MINS, 15MINS, 30MINS, 1HOUR, 4HOURS, 1DAY.
    */
    pub async fn get_candles<'a>(
        &'a self,
        ticker: &'a str,
        limit: Option<::std::num::NonZeroU32>,
        resolution: &'a str,
    ) -> Result<ResponseValue<types::CandlesResponse>, Error<()>> {
        let url = format!(
            "{}/candles/perpetualMarkets/{}",
            self.baseurl,
            encode_path(&ticker.to_string()),
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
            .query(&progenitor_client::QueryParam::new("limit", &limit))
            .query(&progenitor_client::QueryParam::new(
                "resolution",
                &resolution,
            ))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "get_candles",
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
    /**Recent public trades for a perpetual market

    Sends a `GET` request to `/trades/perpetualMarket/{ticker}`

    Arguments:
    - `ticker`: Market ticker (e.g. "BTC-USD").
    - `limit`
    */
    pub async fn get_trades<'a>(
        &'a self,
        ticker: &'a str,
        limit: Option<::std::num::NonZeroU32>,
    ) -> Result<
        ResponseValue<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
        Error<()>,
    > {
        let url = format!(
            "{}/trades/perpetualMarket/{}",
            self.baseurl,
            encode_path(&ticker.to_string()),
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
            .query(&progenitor_client::QueryParam::new("limit", &limit))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "get_trades",
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
    /**Get a subaccount snapshot for an address

    Sends a `GET` request to `/addresses/{address}/subaccountNumber/{subaccountNumber}`

    Arguments:
    - `address`: dYdX bech32 address (e.g. dydx1...).
    - `subaccount_number`
    */
    pub async fn get_subaccount<'a>(
        &'a self,
        address: &'a str,
        subaccount_number: u32,
    ) -> Result<
        ResponseValue<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
        Error<()>,
    > {
        let url = format!(
            "{}/addresses/{}/subaccountNumber/{}",
            self.baseurl,
            encode_path(&address.to_string()),
            encode_path(&subaccount_number.to_string()),
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
            operation_id: "get_subaccount",
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
    /**Orders for a subaccount, optionally filtered by status / ticker

    Sends a `GET` request to `/orders`

    Arguments:
    - `address`
    - `status`: One of OPEN, FILLED, CANCELED, BEST_EFFORT_CANCELED, UNTRIGGERED.
    - `subaccount_number`
    - `ticker`
    */
    pub async fn get_orders<'a>(
        &'a self,
        address: &'a str,
        status: Option<&'a str>,
        subaccount_number: u32,
        ticker: Option<&'a str>,
    ) -> Result<
        ResponseValue<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
        Error<()>,
    > {
        let url = format!("{}/orders", self.baseurl,);
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
            .query(&progenitor_client::QueryParam::new("address", &address))
            .query(&progenitor_client::QueryParam::new("status", &status))
            .query(&progenitor_client::QueryParam::new(
                "subaccountNumber",
                &subaccount_number,
            ))
            .query(&progenitor_client::QueryParam::new("ticker", &ticker))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "get_orders",
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
    /**Fills (executed trades) for a subaccount

    Sends a `GET` request to `/fills`

    */
    pub async fn get_fills<'a>(
        &'a self,
        address: &'a str,
        limit: Option<::std::num::NonZeroU32>,
        market: Option<&'a str>,
        subaccount_number: u32,
    ) -> Result<
        ResponseValue<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
        Error<()>,
    > {
        let url = format!("{}/fills", self.baseurl,);
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
            .query(&progenitor_client::QueryParam::new("address", &address))
            .query(&progenitor_client::QueryParam::new("limit", &limit))
            .query(&progenitor_client::QueryParam::new("market", &market))
            .query(&progenitor_client::QueryParam::new(
                "subaccountNumber",
                &subaccount_number,
            ))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "get_fills",
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
    /**Per-hour historical funding rate series for a market

    Sends a `GET` request to `/historicalFunding/{ticker}`

    Arguments:
    - `ticker`: Market ticker (e.g. "BTC-USD").
    - `limit`
    */
    pub async fn get_historical_funding<'a>(
        &'a self,
        ticker: &'a str,
        limit: Option<::std::num::NonZeroU32>,
    ) -> Result<
        ResponseValue<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
        Error<()>,
    > {
        let url = format!(
            "{}/historicalFunding/{}",
            self.baseurl,
            encode_path(&ticker.to_string()),
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
            .query(&progenitor_client::QueryParam::new("limit", &limit))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "get_historical_funding",
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
