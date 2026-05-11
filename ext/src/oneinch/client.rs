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
    ///`AllowanceResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "allowance": {
    ///      "description": "Current allowance in token base units.",
    ///      "type": "string"
    ///    }
    ///  },
    ///  "additionalProperties": true
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct AllowanceResponse {
        ///Current allowance in token base units.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub allowance: ::std::option::Option<::std::string::String>,
    }
    impl ::std::default::Default for AllowanceResponse {
        fn default() -> Self {
            Self {
                allowance: Default::default(),
            }
        }
    }
    ///`Error`
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
    ///    "error": {
    ///      "type": "string"
    ///    },
    ///    "statusCode": {
    ///      "type": "integer"
    ///    }
    ///  },
    ///  "additionalProperties": true
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct Error {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub description: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub error: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "statusCode",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub status_code: ::std::option::Option<i64>,
    }
    impl ::std::default::Default for Error {
        fn default() -> Self {
            Self {
                description: Default::default(),
                error: Default::default(),
                status_code: Default::default(),
            }
        }
    }
    ///`LiquiditySource`
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
    ///    "img": {
    ///      "type": "string"
    ///    },
    ///    "title": {
    ///      "type": "string"
    ///    }
    ///  },
    ///  "additionalProperties": true
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct LiquiditySource {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub id: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub img: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub title: ::std::option::Option<::std::string::String>,
    }
    impl ::std::default::Default for LiquiditySource {
        fn default() -> Self {
            Self {
                id: Default::default(),
                img: Default::default(),
                title: Default::default(),
            }
        }
    }
    ///`LiquiditySourcesResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "protocols": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/LiquiditySource"
    ///      }
    ///    }
    ///  },
    ///  "additionalProperties": true
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct LiquiditySourcesResponse {
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub protocols: ::std::vec::Vec<LiquiditySource>,
    }
    impl ::std::default::Default for LiquiditySourcesResponse {
        fn default() -> Self {
            Self {
                protocols: Default::default(),
            }
        }
    }
    ///`QuoteResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "dstAmount": {
    ///      "type": "string"
    ///    },
    ///    "protocols": {
    ///      "description": "Routing details across DEX protocols.",
    ///      "type": "array",
    ///      "items": {
    ///        "type": "object",
    ///        "additionalProperties": true
    ///      }
    ///    },
    ///    "srcAmount": {
    ///      "type": "string"
    ///    }
    ///  },
    ///  "additionalProperties": true
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct QuoteResponse {
        #[serde(
            rename = "dstAmount",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub dst_amount: ::std::option::Option<::std::string::String>,
        ///Routing details across DEX protocols.
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub protocols:
            ::std::vec::Vec<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
        #[serde(
            rename = "srcAmount",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub src_amount: ::std::option::Option<::std::string::String>,
    }
    impl ::std::default::Default for QuoteResponse {
        fn default() -> Self {
            Self {
                dst_amount: Default::default(),
                protocols: Default::default(),
                src_amount: Default::default(),
            }
        }
    }
    ///`SwapResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "dstAmount": {
    ///      "type": "string"
    ///    },
    ///    "protocols": {
    ///      "type": "array",
    ///      "items": {
    ///        "type": "object",
    ///        "additionalProperties": true
    ///      }
    ///    },
    ///    "srcAmount": {
    ///      "type": "string"
    ///    },
    ///    "tx": {
    ///      "$ref": "#/components/schemas/Transaction"
    ///    }
    ///  },
    ///  "additionalProperties": true
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct SwapResponse {
        #[serde(
            rename = "dstAmount",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub dst_amount: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub protocols:
            ::std::vec::Vec<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
        #[serde(
            rename = "srcAmount",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub src_amount: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub tx: ::std::option::Option<Transaction>,
    }
    impl ::std::default::Default for SwapResponse {
        fn default() -> Self {
            Self {
                dst_amount: Default::default(),
                protocols: Default::default(),
                src_amount: Default::default(),
                tx: Default::default(),
            }
        }
    }
    ///`Token`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "address": {
    ///      "type": "string"
    ///    },
    ///    "decimals": {
    ///      "type": "integer"
    ///    },
    ///    "logoURI": {
    ///      "type": "string"
    ///    },
    ///    "name": {
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
    pub struct Token {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub address: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub decimals: ::std::option::Option<i64>,
        #[serde(
            rename = "logoURI",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub logo_uri: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub name: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub symbol: ::std::option::Option<::std::string::String>,
    }
    impl ::std::default::Default for Token {
        fn default() -> Self {
            Self {
                address: Default::default(),
                decimals: Default::default(),
                logo_uri: Default::default(),
                name: Default::default(),
                symbol: Default::default(),
            }
        }
    }
    ///`TokensResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "tokens": {
    ///      "type": "object",
    ///      "additionalProperties": {
    ///        "$ref": "#/components/schemas/Token"
    ///      }
    ///    }
    ///  },
    ///  "additionalProperties": true
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct TokensResponse {
        #[serde(
            default,
            skip_serializing_if = ":: std :: collections :: HashMap::is_empty"
        )]
        pub tokens: ::std::collections::HashMap<::std::string::String, Token>,
    }
    impl ::std::default::Default for TokensResponse {
        fn default() -> Self {
            Self {
                tokens: Default::default(),
            }
        }
    }
    ///An EVM transaction the caller can sign and submit.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "An EVM transaction the caller can sign and submit.",
    ///  "type": "object",
    ///  "properties": {
    ///    "data": {
    ///      "type": "string"
    ///    },
    ///    "from": {
    ///      "type": "string"
    ///    },
    ///    "gas": {
    ///      "type": "string"
    ///    },
    ///    "gasPrice": {
    ///      "type": "string"
    ///    },
    ///    "to": {
    ///      "type": "string"
    ///    },
    ///    "value": {
    ///      "type": "string"
    ///    }
    ///  },
    ///  "additionalProperties": true
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct Transaction {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub data: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub from: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub gas: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "gasPrice",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub gas_price: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub to: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub value: ::std::option::Option<::std::string::String>,
    }
    impl ::std::default::Default for Transaction {
        fn default() -> Self {
            Self {
                data: Default::default(),
                from: Default::default(),
                gas: Default::default(),
                gas_price: Default::default(),
                to: Default::default(),
                value: Default::default(),
            }
        }
    }
}
#[derive(Clone, Debug)]
/**Client for 1inch Swap API

1inch Aggregation Protocol Swap API v6.0. Provides best-execution token swaps
across DEX liquidity on supported EVM chains. The API returns quotes,
swap transactions ready for the user to sign, ERC-20 approval calldata,
and reference data such as the supported token list.

## Auth
All endpoints require a Bearer token in the `Authorization` header. Obtain a
free API key at https://portal.1inch.dev/.

## Supported chains
Pass the EVM chain id as the `{chain}` path parameter. Officially supported
chain ids include: 1 (Ethereum), 10 (Optimism), 56 (BNB Chain), 100 (Gnosis),
137 (Polygon), 8453 (Base), 42161 (Arbitrum), 43114 (Avalanche).

## Native asset
Use the sentinel address `0xeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee` to
represent the native asset of a chain (ETH, MATIC, BNB, AVAX, ...).


Version: 6.0.0*/
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
        "6.0.0"
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
    /**Get a swap quote

    Returns the best route across DEX liquidity for selling `amount` of `src`
    for `dst` on the given chain. Does not require a wallet address.


    Sends a `GET` request to `/{chain}/quote`

    Arguments:
    - `chain`: EVM chain id (1, 10, 56, 100, 137, 8453, 42161, 43114, ...).
    - `amount`: Sell amount in source-token base units (wei for 18-dec tokens).
    - `dst`: Destination token address.
    - `protocols`: Optional comma-separated list of liquidity protocols to restrict routing to.
    - `src`: Source token address (use sentinel for native asset).
    */
    pub async fn get_quote<'a>(
        &'a self,
        chain: i64,
        amount: &'a str,
        dst: &'a str,
        protocols: Option<&'a str>,
        src: &'a str,
    ) -> Result<ResponseValue<types::QuoteResponse>, Error<types::Error>> {
        let url = format!("{}/{}/quote", self.baseurl, encode_path(&chain.to_string()),);
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
            .query(&progenitor_client::QueryParam::new("amount", &amount))
            .query(&progenitor_client::QueryParam::new("dst", &dst))
            .query(&progenitor_client::QueryParam::new("protocols", &protocols))
            .query(&progenitor_client::QueryParam::new("src", &src))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "get_quote",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            401u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Build a swap transaction

    Returns an executable swap transaction (`to`, `data`, `value`, `gas`)
    the caller can submit on-chain. For ERC-20 sells the user must first
    grant allowance to the 1inch router (see `/approve/transaction`).


    Sends a `GET` request to `/{chain}/swap`

    Arguments:
    - `chain`: EVM chain id (1, 10, 56, 100, 137, 8453, 42161, 43114, ...).
    - `amount`
    - `dst`
    - `from`: Sender wallet address.
    - `protocols`
    - `slippage`: Max acceptable slippage as a percent (1 = 1%).
    - `src`
    */
    pub async fn get_swap<'a>(
        &'a self,
        chain: i64,
        amount: &'a str,
        dst: &'a str,
        from: &'a str,
        protocols: Option<&'a str>,
        slippage: f64,
        src: &'a str,
    ) -> Result<ResponseValue<types::SwapResponse>, Error<types::Error>> {
        let url = format!("{}/{}/swap", self.baseurl, encode_path(&chain.to_string()),);
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
            .query(&progenitor_client::QueryParam::new("amount", &amount))
            .query(&progenitor_client::QueryParam::new("dst", &dst))
            .query(&progenitor_client::QueryParam::new("from", &from))
            .query(&progenitor_client::QueryParam::new("protocols", &protocols))
            .query(&progenitor_client::QueryParam::new("slippage", &slippage))
            .query(&progenitor_client::QueryParam::new("src", &src))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "get_swap",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            401u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Build an ERC-20 approval transaction for the 1inch router

    Sends a `GET` request to `/{chain}/approve/transaction`

    Arguments:
    - `chain`: EVM chain id (1, 10, 56, 100, 137, 8453, 42161, 43114, ...).
    - `amount`: Approval amount in token base units. Omit for unlimited approval.
    - `token_address`
    */
    pub async fn get_approve_transaction<'a>(
        &'a self,
        chain: i64,
        amount: Option<&'a str>,
        token_address: &'a str,
    ) -> Result<ResponseValue<types::Transaction>, Error<types::Error>> {
        let url = format!(
            "{}/{}/approve/transaction",
            self.baseurl,
            encode_path(&chain.to_string()),
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
            .query(&progenitor_client::QueryParam::new("amount", &amount))
            .query(&progenitor_client::QueryParam::new(
                "tokenAddress",
                &token_address,
            ))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "get_approve_transaction",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            401u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Check current ERC-20 allowance for the 1inch router

    Sends a `GET` request to `/{chain}/approve/allowance`

    Arguments:
    - `chain`: EVM chain id (1, 10, 56, 100, 137, 8453, 42161, 43114, ...).
    - `token_address`
    - `wallet_address`
    */
    pub async fn get_allowance<'a>(
        &'a self,
        chain: i64,
        token_address: &'a str,
        wallet_address: &'a str,
    ) -> Result<ResponseValue<types::AllowanceResponse>, Error<types::Error>> {
        let url = format!(
            "{}/{}/approve/allowance",
            self.baseurl,
            encode_path(&chain.to_string()),
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
            .query(&progenitor_client::QueryParam::new(
                "tokenAddress",
                &token_address,
            ))
            .query(&progenitor_client::QueryParam::new(
                "walletAddress",
                &wallet_address,
            ))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "get_allowance",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            401u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**List tokens supported by 1inch on a chain

    Sends a `GET` request to `/{chain}/tokens`

    Arguments:
    - `chain`: EVM chain id (1, 10, 56, 100, 137, 8453, 42161, 43114, ...).
    */
    pub async fn get_tokens<'a>(
        &'a self,
        chain: i64,
    ) -> Result<ResponseValue<types::TokensResponse>, Error<types::Error>> {
        let url = format!(
            "{}/{}/tokens",
            self.baseurl,
            encode_path(&chain.to_string()),
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
            operation_id: "get_tokens",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            401u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**List liquidity sources (protocols) supported on a chain

    Sends a `GET` request to `/{chain}/liquidity-sources`

    Arguments:
    - `chain`: EVM chain id (1, 10, 56, 100, 137, 8453, 42161, 43114, ...).
    */
    pub async fn get_liquidity_sources<'a>(
        &'a self,
        chain: i64,
    ) -> Result<ResponseValue<types::LiquiditySourcesResponse>, Error<types::Error>> {
        let url = format!(
            "{}/{}/liquidity-sources",
            self.baseurl,
            encode_path(&chain.to_string()),
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
            operation_id: "get_liquidity_sources",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            401u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
}
/// Items consumers will typically use such as the Client.
pub mod prelude {
    #[allow(unused_imports)]
    pub use super::Client;
}
