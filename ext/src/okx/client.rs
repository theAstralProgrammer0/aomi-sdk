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
    ///`CancelOrderRequest`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "instId",
    ///    "ordId"
    ///  ],
    ///  "properties": {
    ///    "instId": {
    ///      "type": "string"
    ///    },
    ///    "ordId": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct CancelOrderRequest {
        #[serde(rename = "instId")]
        pub inst_id: ::std::string::String,
        #[serde(rename = "ordId")]
        pub ord_id: ::std::string::String,
    }
    ///`EnvelopeResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "code": {
    ///      "type": "string"
    ///    },
    ///    "data": {
    ///      "type": "array",
    ///      "items": {
    ///        "type": "object",
    ///        "additionalProperties": true
    ///      }
    ///    },
    ///    "msg": {
    ///      "type": "string"
    ///    }
    ///  },
    ///  "additionalProperties": true
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct EnvelopeResponse {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub code: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub data: ::std::vec::Vec<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub msg: ::std::option::Option<::std::string::String>,
    }
    impl ::std::default::Default for EnvelopeResponse {
        fn default() -> Self {
            Self {
                code: Default::default(),
                data: Default::default(),
                msg: Default::default(),
            }
        }
    }
    ///`PlaceOrderRequest`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "instId",
    ///    "ordType",
    ///    "side",
    ///    "sz",
    ///    "tdMode"
    ///  ],
    ///  "properties": {
    ///    "instId": {
    ///      "type": "string"
    ///    },
    ///    "ordType": {
    ///      "description": "`market`, `limit`, `post_only`, `fok`, `ioc`.",
    ///      "type": "string"
    ///    },
    ///    "px": {
    ///      "description": "Price (required for limit orders).",
    ///      "type": "string"
    ///    },
    ///    "side": {
    ///      "description": "`buy` or `sell`.",
    ///      "type": "string"
    ///    },
    ///    "sz": {
    ///      "type": "string"
    ///    },
    ///    "tdMode": {
    ///      "description": "`cash` (spot) / `cross` / `isolated`.",
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct PlaceOrderRequest {
        #[serde(rename = "instId")]
        pub inst_id: ::std::string::String,
        ///`market`, `limit`, `post_only`, `fok`, `ioc`.
        #[serde(rename = "ordType")]
        pub ord_type: ::std::string::String,
        ///Price (required for limit orders).
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub px: ::std::option::Option<::std::string::String>,
        ///`buy` or `sell`.
        pub side: ::std::string::String,
        pub sz: ::std::string::String,
        ///`cash` (spot) / `cross` / `isolated`.
        #[serde(rename = "tdMode")]
        pub td_mode: ::std::string::String,
    }
    ///`SetLeverageRequest`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "instId",
    ///    "lever",
    ///    "mgnMode"
    ///  ],
    ///  "properties": {
    ///    "instId": {
    ///      "type": "string"
    ///    },
    ///    "lever": {
    ///      "type": "string"
    ///    },
    ///    "mgnMode": {
    ///      "description": "`cross` or `isolated`.",
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct SetLeverageRequest {
        #[serde(rename = "instId")]
        pub inst_id: ::std::string::String,
        pub lever: ::std::string::String,
        ///`cross` or `isolated`.
        #[serde(rename = "mgnMode")]
        pub mgn_mode: ::std::string::String,
    }
}
#[derive(Clone, Debug)]
/**Client for OKX V5 API

OKX V5 REST API surface used by the Aomi `okx` app.

Only the endpoints actually called by `apps/okx/src/tool.rs` are
described here — public market data (tickers/books/candles) plus signed
account/order endpoints.

## Auth
Signed endpoints require these request headers, sent on every signed call:
  - `OK-ACCESS-KEY`        — your API key
  - `OK-ACCESS-SIGN`       — base64(HMAC-SHA256(secret, prehash))
  - `OK-ACCESS-TIMESTAMP`  — ISO-8601 timestamp e.g. 2024-01-01T00:00:00.000Z
  - `OK-ACCESS-PASSPHRASE` — the passphrase set when the API key was created

The prehash is `timestamp + method + requestPath + body` where `requestPath`
includes the query string. The HMAC key is the API secret. The signing logic
is hand-written in `ext/src/okx/auth.rs` — this spec only describes the
resulting headers for codegen purposes. The curated tool layer in
`apps/okx/src/tool.rs` computes the signature before calling the generated
method.

All responses share an envelope: `{ code: "0", msg: "", data: [...] }`.
Non-zero `code` indicates a logical error even on HTTP 200.


Version: 5.0*/
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
        "5.0"
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
    /**Latest ticker snapshot for instruments of a given type

    Public endpoint. Returns last price, 24h volume, best bid/ask, etc. for
    every instrument of the given type (SPOT, SWAP, FUTURES, OPTION).


    Sends a `GET` request to `/api/v5/market/tickers`

    Arguments:
    - `inst_type`: `SPOT`, `SWAP`, `FUTURES`, or `OPTION`.
    */
    pub async fn get_tickers<'a>(
        &'a self,
        inst_type: &'a str,
    ) -> Result<ResponseValue<types::EnvelopeResponse>, Error<()>> {
        let url = format!("{}/api/v5/market/tickers", self.baseurl,);
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
            .query(&progenitor_client::QueryParam::new("instType", &inst_type))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "get_tickers",
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
    /**Order-book bids and asks for an instrument

    Public endpoint. Returns bids/asks at the requested depth.


    Sends a `GET` request to `/api/v5/market/books`

    Arguments:
    - `inst_id`: Instrument ID e.g. `BTC-USDT`, `BTC-USDT-SWAP`.
    - `sz`: Depth — max 400; default 1.
    */
    pub async fn get_order_book<'a>(
        &'a self,
        inst_id: &'a str,
        sz: Option<&'a str>,
    ) -> Result<ResponseValue<types::EnvelopeResponse>, Error<()>> {
        let url = format!("{}/api/v5/market/books", self.baseurl,);
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
            .query(&progenitor_client::QueryParam::new("instId", &inst_id))
            .query(&progenitor_client::QueryParam::new("sz", &sz))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "get_order_book",
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
    /**OHLCV candles for an instrument

    Public endpoint. Returns candle arrays for the given instrument.


    Sends a `GET` request to `/api/v5/market/candles`

    Arguments:
    - `after`: Pagination — return records newer than this ms timestamp.
    - `bar`: Bar size — `1m`, `3m`, `5m`, `15m`, `30m`, `1H`, `4H`, `1D`, `1W`, `1M`.
    - `before`: Pagination — return records older than this ms timestamp.
    - `inst_id`
    - `limit`: Max 300, default 100.
    */
    pub async fn get_candles<'a>(
        &'a self,
        after: Option<&'a str>,
        bar: Option<&'a str>,
        before: Option<&'a str>,
        inst_id: &'a str,
        limit: Option<&'a str>,
    ) -> Result<ResponseValue<types::EnvelopeResponse>, Error<()>> {
        let url = format!("{}/api/v5/market/candles", self.baseurl,);
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
            .query(&progenitor_client::QueryParam::new("after", &after))
            .query(&progenitor_client::QueryParam::new("bar", &bar))
            .query(&progenitor_client::QueryParam::new("before", &before))
            .query(&progenitor_client::QueryParam::new("instId", &inst_id))
            .query(&progenitor_client::QueryParam::new("limit", &limit))
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
    /**Place a new order (signed)

    Signed endpoint. JSON body. The curated tool computes the HMAC over
    `timestamp + "POST" + "/api/v5/trade/order" + body_json` and passes the
    signature/timestamp/api-key/passphrase as headers.


    Sends a `POST` request to `/api/v5/trade/order`

    */
    pub async fn place_order<'a>(
        &'a self,
        ok_access_key: &'a str,
        ok_access_passphrase: &'a str,
        ok_access_sign: &'a str,
        ok_access_timestamp: &'a str,
        body: &'a types::PlaceOrderRequest,
    ) -> Result<ResponseValue<types::EnvelopeResponse>, Error<()>> {
        let url = format!("{}/api/v5/trade/order", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(5usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        header_map.append("OK-ACCESS-KEY", ok_access_key.to_string().try_into()?);
        header_map.append(
            "OK-ACCESS-PASSPHRASE",
            ok_access_passphrase.to_string().try_into()?,
        );
        header_map.append("OK-ACCESS-SIGN", ok_access_sign.to_string().try_into()?);
        header_map.append(
            "OK-ACCESS-TIMESTAMP",
            ok_access_timestamp.to_string().try_into()?,
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
    /**Cancel an open order (signed)

    Signed endpoint. JSON body.

    Sends a `POST` request to `/api/v5/trade/cancel-order`

    */
    pub async fn cancel_order<'a>(
        &'a self,
        ok_access_key: &'a str,
        ok_access_passphrase: &'a str,
        ok_access_sign: &'a str,
        ok_access_timestamp: &'a str,
        body: &'a types::CancelOrderRequest,
    ) -> Result<ResponseValue<types::EnvelopeResponse>, Error<()>> {
        let url = format!("{}/api/v5/trade/cancel-order", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(5usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        header_map.append("OK-ACCESS-KEY", ok_access_key.to_string().try_into()?);
        header_map.append(
            "OK-ACCESS-PASSPHRASE",
            ok_access_passphrase.to_string().try_into()?,
        );
        header_map.append("OK-ACCESS-SIGN", ok_access_sign.to_string().try_into()?);
        header_map.append(
            "OK-ACCESS-TIMESTAMP",
            ok_access_timestamp.to_string().try_into()?,
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
    /**Account balances (signed)

    Signed endpoint. Returns unified-account balances; optional `ccy` is a
    comma-separated currency list.


    Sends a `GET` request to `/api/v5/account/balance`

    Arguments:
    - `ccy`: Comma-separated currency list, e.g. `BTC,USDT`.
    - `ok_access_key`
    - `ok_access_passphrase`
    - `ok_access_sign`
    - `ok_access_timestamp`
    */
    pub async fn get_balance<'a>(
        &'a self,
        ccy: Option<&'a str>,
        ok_access_key: &'a str,
        ok_access_passphrase: &'a str,
        ok_access_sign: &'a str,
        ok_access_timestamp: &'a str,
    ) -> Result<ResponseValue<types::EnvelopeResponse>, Error<()>> {
        let url = format!("{}/api/v5/account/balance", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(5usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        header_map.append("OK-ACCESS-KEY", ok_access_key.to_string().try_into()?);
        header_map.append(
            "OK-ACCESS-PASSPHRASE",
            ok_access_passphrase.to_string().try_into()?,
        );
        header_map.append("OK-ACCESS-SIGN", ok_access_sign.to_string().try_into()?);
        header_map.append(
            "OK-ACCESS-TIMESTAMP",
            ok_access_timestamp.to_string().try_into()?,
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new("ccy", &ccy))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "get_balance",
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
    /**Open derivative positions (signed)

    Signed endpoint.

    Sends a `GET` request to `/api/v5/account/positions`

    Arguments:
    - `inst_id`
    - `inst_type`: `SPOT`, `SWAP`, `FUTURES`, or `OPTION`.
    - `ok_access_key`
    - `ok_access_passphrase`
    - `ok_access_sign`
    - `ok_access_timestamp`
    */
    pub async fn get_positions<'a>(
        &'a self,
        inst_id: Option<&'a str>,
        inst_type: Option<&'a str>,
        ok_access_key: &'a str,
        ok_access_passphrase: &'a str,
        ok_access_sign: &'a str,
        ok_access_timestamp: &'a str,
    ) -> Result<ResponseValue<types::EnvelopeResponse>, Error<()>> {
        let url = format!("{}/api/v5/account/positions", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(5usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        header_map.append("OK-ACCESS-KEY", ok_access_key.to_string().try_into()?);
        header_map.append(
            "OK-ACCESS-PASSPHRASE",
            ok_access_passphrase.to_string().try_into()?,
        );
        header_map.append("OK-ACCESS-SIGN", ok_access_sign.to_string().try_into()?);
        header_map.append(
            "OK-ACCESS-TIMESTAMP",
            ok_access_timestamp.to_string().try_into()?,
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new("instId", &inst_id))
            .query(&progenitor_client::QueryParam::new("instType", &inst_type))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "get_positions",
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
    /**Set per-instrument leverage (signed)

    Signed endpoint. JSON body.

    Sends a `POST` request to `/api/v5/account/set-leverage`

    */
    pub async fn set_leverage<'a>(
        &'a self,
        ok_access_key: &'a str,
        ok_access_passphrase: &'a str,
        ok_access_sign: &'a str,
        ok_access_timestamp: &'a str,
        body: &'a types::SetLeverageRequest,
    ) -> Result<ResponseValue<types::EnvelopeResponse>, Error<()>> {
        let url = format!("{}/api/v5/account/set-leverage", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(5usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        header_map.append("OK-ACCESS-KEY", ok_access_key.to_string().try_into()?);
        header_map.append(
            "OK-ACCESS-PASSPHRASE",
            ok_access_passphrase.to_string().try_into()?,
        );
        header_map.append("OK-ACCESS-SIGN", ok_access_sign.to_string().try_into()?);
        header_map.append(
            "OK-ACCESS-TIMESTAMP",
            ok_access_timestamp.to_string().try_into()?,
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
            operation_id: "set_leverage",
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
