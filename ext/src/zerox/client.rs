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
    ///Free-form error payload returned by 0x.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Free-form error payload returned by 0x.",
    ///  "type": "object",
    ///  "additionalProperties": true
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    #[serde(transparent)]
    pub struct ErrorResponseBody(
        pub ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    );
    impl ::std::ops::Deref for ErrorResponseBody {
        type Target = ::serde_json::Map<::std::string::String, ::serde_json::Value>;
        fn deref(
            &self,
        ) -> &::serde_json::Map<::std::string::String, ::serde_json::Value> {
            &self.0
        }
    }
    impl ::std::convert::From<ErrorResponseBody>
    for ::serde_json::Map<::std::string::String, ::serde_json::Value> {
        fn from(value: ErrorResponseBody) -> Self {
            value.0
        }
    }
    impl ::std::convert::From<
        ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    > for ErrorResponseBody {
        fn from(
            value: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        ) -> Self {
            Self(value)
        }
    }
    ///`GaslessSubmitRequest`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "chainId",
    ///    "trade"
    ///  ],
    ///  "properties": {
    ///    "approval": {
    ///      "description": "Optional signed `approval` object returned by `getGaslessQuote`.",
    ///      "type": "object",
    ///      "additionalProperties": true
    ///    },
    ///    "chainId": {
    ///      "type": "integer",
    ///      "format": "int64"
    ///    },
    ///    "trade": {
    ///      "description": "Signed `trade` object returned by `getGaslessQuote`.",
    ///      "type": "object",
    ///      "additionalProperties": true
    ///    }
    ///  },
    ///  "additionalProperties": false
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct GaslessSubmitRequest {
        ///Optional signed `approval` object returned by `getGaslessQuote`.
        #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
        pub approval: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        #[serde(rename = "chainId")]
        pub chain_id: i64,
        ///Signed `trade` object returned by `getGaslessQuote`.
        pub trade: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    }
    /**Loosely typed quote response. Fields vary across price/quote/gasless
endpoints; consumers should pick what they need from the returned JSON.
Common fields include `buyAmount`, `sellAmount`, `transaction`, `issues`,
`route`, `trade`, `approval`, `tradeHash`, `status`.
*/
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Loosely typed quote response. Fields vary across price/quote/gasless\nendpoints; consumers should pick what they need from the returned JSON.\nCommon fields include `buyAmount`, `sellAmount`, `transaction`, `issues`,\n`route`, `trade`, `approval`, `tradeHash`, `status`.\n",
    ///  "type": "object",
    ///  "additionalProperties": true
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    #[serde(transparent)]
    pub struct SwapQuote(
        pub ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    );
    impl ::std::ops::Deref for SwapQuote {
        type Target = ::serde_json::Map<::std::string::String, ::serde_json::Value>;
        fn deref(
            &self,
        ) -> &::serde_json::Map<::std::string::String, ::serde_json::Value> {
            &self.0
        }
    }
    impl ::std::convert::From<SwapQuote>
    for ::serde_json::Map<::std::string::String, ::serde_json::Value> {
        fn from(value: SwapQuote) -> Self {
            value.0
        }
    }
    impl ::std::convert::From<
        ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    > for SwapQuote {
        fn from(
            value: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        ) -> Self {
            Self(value)
        }
    }
}
#[derive(Clone, Debug)]
/**Client for 0x Swap API

0x Swap API v2 — quote and execute same-chain ERC-20 / native swaps either
via the AllowanceHolder on-chain path or via the gasless relayer (sign
EIP-712, relayer pays gas).

## Auth
All endpoints require an API key in the `0x-api-key` request header. The
`0x-version: v2` header is also required by the production API.

## Chain selection
Chain is selected via the `chainId` query parameter (numeric).

## Native asset
Use the sentinel address `0xeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee`
to represent the native asset of a chain (ETH, MATIC, BNB, AVAX, ...).

Endpoints covered: AllowanceHolder price/quote, gasless quote/submit/status.


Version: 2.0.0*/
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
        "2.0.0"
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
    /**Indicative AllowanceHolder price (no signing required)

Sends a `GET` request to `/swap/allowance-holder/price`

Arguments:
- `buy_token`: Buy token contract address (or sentinel `0xEeee...EEeE` for native).
- `chain_id`: Numeric EVM chain id (1, 10, 56, 137, 8453, 42161, 43114, ...).
- `sell_amount`: Sell amount in base units (decimal string).
- `sell_token`: Sell token contract address (or sentinel `0xEeee...EEeE` for native).
- `slippage_percentage`: Slippage tolerance as a decimal (e.g. 0.01 = 1%). Default 0.01.
- `taker`: Optional taker wallet address (improves quote accuracy).
*/
    pub async fn get_allowance_holder_price<'a>(
        &'a self,
        buy_token: &'a str,
        chain_id: i64,
        sell_amount: &'a str,
        sell_token: &'a str,
        slippage_percentage: Option<f64>,
        taker: Option<&'a str>,
    ) -> Result<ResponseValue<types::SwapQuote>, Error<()>> {
        let url = format!("{}/swap/allowance-holder/price", self.baseurl,);
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
            .query(&progenitor_client::QueryParam::new("buyToken", &buy_token))
            .query(&progenitor_client::QueryParam::new("chainId", &chain_id))
            .query(&progenitor_client::QueryParam::new("sellAmount", &sell_amount))
            .query(&progenitor_client::QueryParam::new("sellToken", &sell_token))
            .query(
                &progenitor_client::QueryParam::new(
                    "slippagePercentage",
                    &slippage_percentage,
                ),
            )
            .query(&progenitor_client::QueryParam::new("taker", &taker))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "get_allowance_holder_price",
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
    /**Firm AllowanceHolder quote with executable transaction

Sends a `GET` request to `/swap/allowance-holder/quote`

Arguments:
- `buy_token`: Buy token contract address (or sentinel `0xEeee...EEeE` for native).
- `chain_id`: Numeric EVM chain id (1, 10, 56, 137, 8453, 42161, 43114, ...).
- `sell_amount`: Sell amount in base units (decimal string).
- `sell_token`: Sell token contract address (or sentinel `0xEeee...EEeE` for native).
- `slippage_percentage`: Slippage tolerance as a decimal (e.g. 0.01 = 1%). Default 0.01.
- `taker`: Optional taker wallet address (improves quote accuracy).
*/
    pub async fn get_allowance_holder_quote<'a>(
        &'a self,
        buy_token: &'a str,
        chain_id: i64,
        sell_amount: &'a str,
        sell_token: &'a str,
        slippage_percentage: Option<f64>,
        taker: Option<&'a str>,
    ) -> Result<ResponseValue<types::SwapQuote>, Error<()>> {
        let url = format!("{}/swap/allowance-holder/quote", self.baseurl,);
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
            .query(&progenitor_client::QueryParam::new("buyToken", &buy_token))
            .query(&progenitor_client::QueryParam::new("chainId", &chain_id))
            .query(&progenitor_client::QueryParam::new("sellAmount", &sell_amount))
            .query(&progenitor_client::QueryParam::new("sellToken", &sell_token))
            .query(
                &progenitor_client::QueryParam::new(
                    "slippagePercentage",
                    &slippage_percentage,
                ),
            )
            .query(&progenitor_client::QueryParam::new("taker", &taker))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "get_allowance_holder_quote",
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
    /**Firm gasless quote (returns EIP-712 typed data to sign)

Sends a `GET` request to `/gasless/quote`

Arguments:
- `buy_token`: Buy token contract address (or sentinel `0xEeee...EEeE` for native).
- `chain_id`: Numeric EVM chain id (1, 10, 56, 137, 8453, 42161, 43114, ...).
- `sell_amount`: Sell amount in base units (decimal string).
- `sell_token`: Sell token contract address (or sentinel `0xEeee...EEeE` for native).
- `slippage_percentage`: Slippage tolerance as a decimal (e.g. 0.01 = 1%). Default 0.01.
- `taker`: Optional taker wallet address (improves quote accuracy).
*/
    pub async fn get_gasless_quote<'a>(
        &'a self,
        buy_token: &'a str,
        chain_id: i64,
        sell_amount: &'a str,
        sell_token: &'a str,
        slippage_percentage: Option<f64>,
        taker: Option<&'a str>,
    ) -> Result<ResponseValue<types::SwapQuote>, Error<()>> {
        let url = format!("{}/gasless/quote", self.baseurl,);
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
            .query(&progenitor_client::QueryParam::new("buyToken", &buy_token))
            .query(&progenitor_client::QueryParam::new("chainId", &chain_id))
            .query(&progenitor_client::QueryParam::new("sellAmount", &sell_amount))
            .query(&progenitor_client::QueryParam::new("sellToken", &sell_token))
            .query(
                &progenitor_client::QueryParam::new(
                    "slippagePercentage",
                    &slippage_percentage,
                ),
            )
            .query(&progenitor_client::QueryParam::new("taker", &taker))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "get_gasless_quote",
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
    /**Submit signed gasless trade (and approval) to the 0x relayer

Sends a `POST` request to `/gasless/submit`

*/
    pub async fn submit_gasless<'a>(
        &'a self,
        body: &'a types::GaslessSubmitRequest,
    ) -> Result<ResponseValue<types::SwapQuote>, Error<()>> {
        let url = format!("{}/gasless/submit", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map
            .append(
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
            operation_id: "submit_gasless",
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
    /**Poll the lifecycle of a submitted gasless trade

Sends a `GET` request to `/gasless/status/{tradeHash}`

Arguments:
- `trade_hash`: Hash returned by `submitGasless`.
- `chain_id`
*/
    pub async fn get_gasless_status<'a>(
        &'a self,
        trade_hash: &'a str,
        chain_id: i64,
    ) -> Result<ResponseValue<types::SwapQuote>, Error<()>> {
        let url = format!(
            "{}/gasless/status/{}", self.baseurl, encode_path(& trade_hash.to_string()),
        );
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
            .query(&progenitor_client::QueryParam::new("chainId", &chain_id))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "get_gasless_status",
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
