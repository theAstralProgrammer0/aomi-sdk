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
    ///`GetMarketsResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "markets": {
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
    pub struct GetMarketsResponse {
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub markets: ::std::vec::Vec<
            ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        >,
    }
    impl ::std::default::Default for GetMarketsResponse {
        fn default() -> Self {
            Self {
                markets: Default::default(),
            }
        }
    }
    ///`GetOrdersResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "orders": {
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
    pub struct GetOrdersResponse {
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub orders: ::std::vec::Vec<
            ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        >,
    }
    impl ::std::default::Default for GetOrdersResponse {
        fn default() -> Self {
            Self { orders: Default::default() }
        }
    }
    ///`GetPositionsResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "positions": {
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
    pub struct GetPositionsResponse {
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub positions: ::std::vec::Vec<
            ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        >,
    }
    impl ::std::default::Default for GetPositionsResponse {
        fn default() -> Self {
            Self {
                positions: Default::default(),
            }
        }
    }
    ///`GetSignedPricesResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "signedPrices": {
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
    pub struct GetSignedPricesResponse {
        #[serde(
            rename = "signedPrices",
            default,
            skip_serializing_if = "::std::vec::Vec::is_empty"
        )]
        pub signed_prices: ::std::vec::Vec<
            ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        >,
    }
    impl ::std::default::Default for GetSignedPricesResponse {
        fn default() -> Self {
            Self {
                signed_prices: Default::default(),
            }
        }
    }
}
#[derive(Clone, Debug)]
/**Client for GMX v2 Public API

GMX v2 public read-only REST API for synthetics on Arbitrum and Avalanche.

## Hosts
GMX exposes the same surface on two hosts (one per chain). Each operation
here uses the relative path; callers select the host by passing the
appropriate `baseurl` to the client constructor:
  - `https://arbitrum-api.gmxinfra.io`  (default — Arbitrum deployment)
  - `https://avalanche-api.gmxinfra.io` (Avalanche deployment)

## Auth
Unauthenticated. All endpoints are public read-only.

## Response shapes
Most response payloads are loosely typed (`additionalProperties: true`)
because GMX's docs do not publish strict JSON schemas. The curated tool
layer hands the raw JSON back to callers.


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
    /**Current oracle min/max prices for every listed token

Sends a `GET` request to `/prices/tickers`

*/
    pub async fn get_prices<'a>(
        &'a self,
    ) -> Result<
        ResponseValue<
            ::std::vec::Vec<
                ::serde_json::Map<::std::string::String, ::serde_json::Value>,
            >,
        >,
        Error<()>,
    > {
        let url = format!("{}/prices/tickers", self.baseurl,);
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
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "get_prices",
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
    /**Latest keeper-signed price packets (advanced)

Sends a `GET` request to `/signed_prices/latest`

*/
    pub async fn get_signed_prices<'a>(
        &'a self,
    ) -> Result<ResponseValue<types::GetSignedPricesResponse>, Error<()>> {
        let url = format!("{}/signed_prices/latest", self.baseurl,);
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
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "get_signed_prices",
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
    /**All GM markets with funding/borrow rates, OI, pool composition

Sends a `GET` request to `/markets/info`

*/
    pub async fn get_markets<'a>(
        &'a self,
    ) -> Result<ResponseValue<types::GetMarketsResponse>, Error<()>> {
        let url = format!("{}/markets/info", self.baseurl,);
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
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "get_markets",
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
    /**Open leveraged positions for an account

Sends a `GET` request to `/positions`

Arguments:
- `account`: Ethereum address (0x...) of the account.
*/
    pub async fn get_positions<'a>(
        &'a self,
        account: &'a str,
    ) -> Result<ResponseValue<types::GetPositionsResponse>, Error<()>> {
        let url = format!("{}/positions", self.baseurl,);
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
            .query(&progenitor_client::QueryParam::new("account", &account))
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
    /**Pending limit/trigger/stop orders for an account

Sends a `GET` request to `/orders`

Arguments:
- `account`: Ethereum address (0x...) of the account.
*/
    pub async fn get_orders<'a>(
        &'a self,
        account: &'a str,
    ) -> Result<ResponseValue<types::GetOrdersResponse>, Error<()>> {
        let url = format!("{}/orders", self.baseurl,);
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
            .query(&progenitor_client::QueryParam::new("account", &account))
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
}
/// Items consumers will typically use such as the Client.
pub mod prelude {
    #[allow(unused_imports)]
    pub use super::Client;
}
