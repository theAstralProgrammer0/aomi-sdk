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
    ///`AccountResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "balances": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/Balance"
    ///      }
    ///    }
    ///  },
    ///  "additionalProperties": true
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct AccountResponse {
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub balances: ::std::vec::Vec<Balance>,
    }
    impl ::std::default::Default for AccountResponse {
        fn default() -> Self {
            Self {
                balances: Default::default(),
            }
        }
    }
    ///`Balance`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "asset": {
    ///      "type": "string"
    ///    },
    ///    "free": {
    ///      "type": "string"
    ///    },
    ///    "locked": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct Balance {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub asset: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub free: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub locked: ::std::option::Option<::std::string::String>,
    }
    impl ::std::default::Default for Balance {
        fn default() -> Self {
            Self {
                asset: Default::default(),
                free: Default::default(),
                locked: Default::default(),
            }
        }
    }
    ///`DepthResponse`
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
    ///        "type": "array",
    ///        "items": {
    ///          "type": "string"
    ///        }
    ///      }
    ///    },
    ///    "bids": {
    ///      "type": "array",
    ///      "items": {
    ///        "type": "array",
    ///        "items": {
    ///          "type": "string"
    ///        }
    ///      }
    ///    },
    ///    "lastUpdateId": {
    ///      "type": "integer",
    ///      "format": "int64"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct DepthResponse {
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub asks: ::std::vec::Vec<::std::vec::Vec<::std::string::String>>,
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub bids: ::std::vec::Vec<::std::vec::Vec<::std::string::String>>,
        #[serde(
            rename = "lastUpdateId",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub last_update_id: ::std::option::Option<i64>,
    }
    impl ::std::default::Default for DepthResponse {
        fn default() -> Self {
            Self {
                asks: Default::default(),
                bids: Default::default(),
                last_update_id: Default::default(),
            }
        }
    }
    ///`OrderResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "orderId": {
    ///      "type": "integer",
    ///      "format": "int64"
    ///    },
    ///    "status": {
    ///      "type": "string"
    ///    },
    ///    "symbol": {
    ///      "type": "string"
    ///    }
    ///  },
    ///  "additionalProperties": true
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct OrderResponse {
        #[serde(
            rename = "orderId",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub order_id: ::std::option::Option<i64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub status: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub symbol: ::std::option::Option<::std::string::String>,
    }
    impl ::std::default::Default for OrderResponse {
        fn default() -> Self {
            Self {
                order_id: Default::default(),
                status: Default::default(),
                symbol: Default::default(),
            }
        }
    }
    ///`Trade`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "id": {
    ///      "type": "integer",
    ///      "format": "int64"
    ///    },
    ///    "price": {
    ///      "type": "string"
    ///    },
    ///    "qty": {
    ///      "type": "string"
    ///    },
    ///    "quoteQty": {
    ///      "type": "string"
    ///    },
    ///    "time": {
    ///      "type": "integer",
    ///      "format": "int64"
    ///    }
    ///  },
    ///  "additionalProperties": true
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct Trade {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub id: ::std::option::Option<i64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub price: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub qty: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "quoteQty",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub quote_qty: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub time: ::std::option::Option<i64>,
    }
    impl ::std::default::Default for Trade {
        fn default() -> Self {
            Self {
                id: Default::default(),
                price: Default::default(),
                qty: Default::default(),
                quote_qty: Default::default(),
                time: Default::default(),
            }
        }
    }
}
#[derive(Clone, Debug)]
/**Client for Binance Spot API

Binance Spot REST API surface used by the Aomi `binance` app.

Only the endpoints actually called by `apps/binance/src/tool.rs` are
described here — public market data (price/depth/klines/24h stats) plus
signed account/order/trade endpoints.

## Auth
Signed endpoints require:
  - `X-MBX-APIKEY` header — your API key
  - `timestamp` query parameter (millis since epoch)
  - `signature` query parameter — HMAC-SHA256(secret_key, query_string)

The HMAC signing is hand-written in `ext/src/binance/auth.rs` — this spec
only describes the resulting `X-MBX-APIKEY` header and the `timestamp` /
`signature` query parameters for codegen purposes. The curated tool layer
in `apps/binance/src/tool.rs` computes the signature before calling the
generated method.


Version: 3.0*/
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
        "3.0"
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
    /**Latest spot price for one or all symbols

    Public endpoint. Returns the latest price for a single symbol when
    `symbol` is provided, or for every spot pair when omitted.


    Sends a `GET` request to `/api/v3/ticker/price`

    Arguments:
    - `symbol`: Symbol like `BTCUSDT`. Omit for all symbols.
    */
    pub async fn get_ticker_price<'a>(
        &'a self,
        symbol: Option<&'a str>,
    ) -> Result<
        ResponseValue<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
        Error<()>,
    > {
        let url = format!("{}/api/v3/ticker/price", self.baseurl,);
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
            .query(&progenitor_client::QueryParam::new("symbol", &symbol))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "get_ticker_price",
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
    /**Order-book bids and asks for a symbol

    Public endpoint. Returns the top N price levels for the given symbol.


    Sends a `GET` request to `/api/v3/depth`

    Arguments:
    - `limit`: Number of levels — 5, 10, 20, 50, 100 (default), 500, 1000, 5000.
    - `symbol`
    */
    pub async fn get_depth<'a>(
        &'a self,
        limit: Option<i32>,
        symbol: &'a str,
    ) -> Result<ResponseValue<types::DepthResponse>, Error<()>> {
        let url = format!("{}/api/v3/depth", self.baseurl,);
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
            .query(&progenitor_client::QueryParam::new("symbol", &symbol))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "get_depth",
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
    /**OHLCV candles for a symbol at a given interval

    Public endpoint. Each candle is a 12-element array
    `[open_time, open, high, low, close, volume, close_time, quote_volume,
    trades, taker_buy_base_vol, taker_buy_quote_vol, ignore]`.


    Sends a `GET` request to `/api/v3/klines`

    Arguments:
    - `end_time`: End time in millis since epoch.
    - `interval`: Candle interval — `1m`, `5m`, `15m`, `1h`, `4h`, `1d`, `1w`, `1M`, etc.
    - `limit`: Default 500, max 1000.
    - `start_time`: Start time in millis since epoch.
    - `symbol`
    */
    pub async fn get_klines<'a>(
        &'a self,
        end_time: Option<i64>,
        interval: &'a str,
        limit: Option<i32>,
        start_time: Option<i64>,
        symbol: &'a str,
    ) -> Result<
        ResponseValue<
            ::std::vec::Vec<
                ::std::vec::Vec<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
            >,
        >,
        Error<()>,
    > {
        let url = format!("{}/api/v3/klines", self.baseurl,);
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
            .query(&progenitor_client::QueryParam::new("endTime", &end_time))
            .query(&progenitor_client::QueryParam::new("interval", &interval))
            .query(&progenitor_client::QueryParam::new("limit", &limit))
            .query(&progenitor_client::QueryParam::new(
                "startTime",
                &start_time,
            ))
            .query(&progenitor_client::QueryParam::new("symbol", &symbol))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "get_klines",
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
    /**Rolling 24h price-change stats

    Public endpoint. Returns 24h volume, high/low, and percent change for
    a single symbol or every spot pair.


    Sends a `GET` request to `/api/v3/ticker/24hr`

    */
    pub async fn get24hr_stats<'a>(
        &'a self,
        symbol: Option<&'a str>,
    ) -> Result<
        ResponseValue<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
        Error<()>,
    > {
        let url = format!("{}/api/v3/ticker/24hr", self.baseurl,);
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
            .query(&progenitor_client::QueryParam::new("symbol", &symbol))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "get24hr_stats",
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
    /**Place a new spot order (signed)

    Signed endpoint. The body is sent as URL-encoded query parameters
    (Binance's convention even for POST). The curated tool builds the query
    string, computes the HMAC signature, and passes both `signature` and
    `timestamp` as query params.


    Sends a `POST` request to `/api/v3/order`

    Arguments:
    - `price`
    - `quantity`
    - `side`: `BUY` or `SELL`.
    - `signature`: HMAC-SHA256 hex of the encoded query string (without `signature`).
    - `symbol`
    - `time_in_force`: `GTC`, `IOC`, `FOK` (required for LIMIT orders).
    - `timestamp`: Millis since epoch (server-side validity window applies).
    - `type_`: `LIMIT`, `MARKET`, `STOP_LOSS_LIMIT`, `TAKE_PROFIT_LIMIT`.
    - `x_mbx_apikey`: Binance API key.
    */
    pub async fn place_order<'a>(
        &'a self,
        price: Option<&'a str>,
        quantity: Option<&'a str>,
        side: &'a str,
        signature: &'a str,
        symbol: &'a str,
        time_in_force: Option<&'a str>,
        timestamp: i64,
        type_: &'a str,
        x_mbx_apikey: &'a str,
    ) -> Result<ResponseValue<types::OrderResponse>, Error<()>> {
        let url = format!("{}/api/v3/order", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(2usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        header_map.append("X-MBX-APIKEY", x_mbx_apikey.to_string().try_into()?);
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new("price", &price))
            .query(&progenitor_client::QueryParam::new("quantity", &quantity))
            .query(&progenitor_client::QueryParam::new("side", &side))
            .query(&progenitor_client::QueryParam::new("signature", &signature))
            .query(&progenitor_client::QueryParam::new("symbol", &symbol))
            .query(&progenitor_client::QueryParam::new(
                "timeInForce",
                &time_in_force,
            ))
            .query(&progenitor_client::QueryParam::new("timestamp", &timestamp))
            .query(&progenitor_client::QueryParam::new("type", &type_))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "place_order",
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
    /**Cancel an open spot order (signed)

    Signed endpoint. Pass either `orderId` (preferred) or
    `origClientOrderId`.


    Sends a `DELETE` request to `/api/v3/order`

    Arguments:
    - `order_id`
    - `orig_client_order_id`
    - `signature`
    - `symbol`
    - `timestamp`
    - `x_mbx_apikey`: Binance API key.
    */
    pub async fn cancel_order<'a>(
        &'a self,
        order_id: Option<i64>,
        orig_client_order_id: Option<&'a str>,
        signature: &'a str,
        symbol: &'a str,
        timestamp: i64,
        x_mbx_apikey: &'a str,
    ) -> Result<ResponseValue<types::OrderResponse>, Error<()>> {
        let url = format!("{}/api/v3/order", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(2usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        header_map.append("X-MBX-APIKEY", x_mbx_apikey.to_string().try_into()?);
        #[allow(unused_mut)]
        let mut request = self
            .client
            .delete(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new("orderId", &order_id))
            .query(&progenitor_client::QueryParam::new(
                "origClientOrderId",
                &orig_client_order_id,
            ))
            .query(&progenitor_client::QueryParam::new("signature", &signature))
            .query(&progenitor_client::QueryParam::new("symbol", &symbol))
            .query(&progenitor_client::QueryParam::new("timestamp", &timestamp))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "cancel_order",
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
    /**Account balances and permissions (signed)

    Signed endpoint. Returns free / locked balances per asset and account
    permissions.


    Sends a `GET` request to `/api/v3/account`

    Arguments:
    - `signature`
    - `timestamp`
    - `x_mbx_apikey`: Binance API key.
    */
    pub async fn get_account<'a>(
        &'a self,
        signature: &'a str,
        timestamp: i64,
        x_mbx_apikey: &'a str,
    ) -> Result<ResponseValue<types::AccountResponse>, Error<()>> {
        let url = format!("{}/api/v3/account", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(2usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        header_map.append("X-MBX-APIKEY", x_mbx_apikey.to_string().try_into()?);
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new("signature", &signature))
            .query(&progenitor_client::QueryParam::new("timestamp", &timestamp))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "get_account",
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
    /**Personal fill history for one symbol (signed)

    Signed endpoint. Returns the user's recent trades on the given symbol.


    Sends a `GET` request to `/api/v3/myTrades`

    Arguments:
    - `end_time`
    - `from_id`
    - `limit`: Default 500, max 1000.
    - `signature`
    - `start_time`
    - `symbol`
    - `timestamp`
    - `x_mbx_apikey`: Binance API key.
    */
    pub async fn get_my_trades<'a>(
        &'a self,
        end_time: Option<i64>,
        from_id: Option<i64>,
        limit: Option<i32>,
        signature: &'a str,
        start_time: Option<i64>,
        symbol: &'a str,
        timestamp: i64,
        x_mbx_apikey: &'a str,
    ) -> Result<ResponseValue<::std::vec::Vec<types::Trade>>, Error<()>> {
        let url = format!("{}/api/v3/myTrades", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(2usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        header_map.append("X-MBX-APIKEY", x_mbx_apikey.to_string().try_into()?);
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new("endTime", &end_time))
            .query(&progenitor_client::QueryParam::new("fromId", &from_id))
            .query(&progenitor_client::QueryParam::new("limit", &limit))
            .query(&progenitor_client::QueryParam::new("signature", &signature))
            .query(&progenitor_client::QueryParam::new(
                "startTime",
                &start_time,
            ))
            .query(&progenitor_client::QueryParam::new("symbol", &symbol))
            .query(&progenitor_client::QueryParam::new("timestamp", &timestamp))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "get_my_trades",
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
