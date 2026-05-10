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
    ///`LifiChainsResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "additionalProperties": true
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    #[serde(transparent)]
    pub struct LifiChainsResponse(
        pub ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    );
    impl ::std::ops::Deref for LifiChainsResponse {
        type Target = ::serde_json::Map<::std::string::String, ::serde_json::Value>;
        fn deref(
            &self,
        ) -> &::serde_json::Map<::std::string::String, ::serde_json::Value> {
            &self.0
        }
    }
    impl ::std::convert::From<LifiChainsResponse>
    for ::serde_json::Map<::std::string::String, ::serde_json::Value> {
        fn from(value: LifiChainsResponse) -> Self {
            value.0
        }
    }
    impl ::std::convert::From<
        ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    > for LifiChainsResponse {
        fn from(
            value: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        ) -> Self {
            Self(value)
        }
    }
    ///`LifiError`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "additionalProperties": true
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    #[serde(transparent)]
    pub struct LifiError(
        pub ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    );
    impl ::std::ops::Deref for LifiError {
        type Target = ::serde_json::Map<::std::string::String, ::serde_json::Value>;
        fn deref(
            &self,
        ) -> &::serde_json::Map<::std::string::String, ::serde_json::Value> {
            &self.0
        }
    }
    impl ::std::convert::From<LifiError>
    for ::serde_json::Map<::std::string::String, ::serde_json::Value> {
        fn from(value: LifiError) -> Self {
            value.0
        }
    }
    impl ::std::convert::From<
        ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    > for LifiError {
        fn from(
            value: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        ) -> Self {
            Self(value)
        }
    }
    /**LI.FI quote response. Contains `id`, `tool`, `action`, `estimate`,
`transactionRequest` (when an executable route was found), and
`includedSteps`. The shape varies per route so we expose it as a
loose object — the curated tool layer extracts what it needs.
*/
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "LI.FI quote response. Contains `id`, `tool`, `action`, `estimate`,\n`transactionRequest` (when an executable route was found), and\n`includedSteps`. The shape varies per route so we expose it as a\nloose object — the curated tool layer extracts what it needs.\n",
    ///  "type": "object",
    ///  "additionalProperties": true
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    #[serde(transparent)]
    pub struct LifiQuote(
        pub ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    );
    impl ::std::ops::Deref for LifiQuote {
        type Target = ::serde_json::Map<::std::string::String, ::serde_json::Value>;
        fn deref(
            &self,
        ) -> &::serde_json::Map<::std::string::String, ::serde_json::Value> {
            &self.0
        }
    }
    impl ::std::convert::From<LifiQuote>
    for ::serde_json::Map<::std::string::String, ::serde_json::Value> {
        fn from(value: LifiQuote) -> Self {
            value.0
        }
    }
    impl ::std::convert::From<
        ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    > for LifiQuote {
        fn from(
            value: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        ) -> Self {
            Self(value)
        }
    }
    ///`LifiStatus`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "additionalProperties": true
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    #[serde(transparent)]
    pub struct LifiStatus(
        pub ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    );
    impl ::std::ops::Deref for LifiStatus {
        type Target = ::serde_json::Map<::std::string::String, ::serde_json::Value>;
        fn deref(
            &self,
        ) -> &::serde_json::Map<::std::string::String, ::serde_json::Value> {
            &self.0
        }
    }
    impl ::std::convert::From<LifiStatus>
    for ::serde_json::Map<::std::string::String, ::serde_json::Value> {
        fn from(value: LifiStatus) -> Self {
            value.0
        }
    }
    impl ::std::convert::From<
        ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    > for LifiStatus {
        fn from(
            value: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        ) -> Self {
            Self(value)
        }
    }
    ///`LifiTokensResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "additionalProperties": true
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    #[serde(transparent)]
    pub struct LifiTokensResponse(
        pub ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    );
    impl ::std::ops::Deref for LifiTokensResponse {
        type Target = ::serde_json::Map<::std::string::String, ::serde_json::Value>;
        fn deref(
            &self,
        ) -> &::serde_json::Map<::std::string::String, ::serde_json::Value> {
            &self.0
        }
    }
    impl ::std::convert::From<LifiTokensResponse>
    for ::serde_json::Map<::std::string::String, ::serde_json::Value> {
        fn from(value: LifiTokensResponse) -> Self {
            value.0
        }
    }
    impl ::std::convert::From<
        ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    > for LifiTokensResponse {
        fn from(
            value: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        ) -> Self {
            Self(value)
        }
    }
}
#[derive(Clone, Debug)]
/**Client for LI.FI API

LI.FI cross-chain swap and bridge aggregator API.

## Auth
Optional Bearer token in the `x-lifi-api-key` header. Quoting and status
work without an API key.

Endpoints covered: `/v1/quote`, `/v1/status`, `/v1/chains`, `/v1/tokens`.
Composite tools (build_swap_tx, build_bridge_tx) layer on top in the
curated tool layer.


Version: 1.0.0*/
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
        "1.0.0"
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
    /**Get a same-chain or cross-chain swap quote with executable transaction

Sends a `GET` request to `/v1/quote`

Arguments:
- `from_address`: Sender wallet address.
- `from_amount`: Sell amount in source-token base units (decimal string).
- `from_chain`: Source chain id or canonical name (e.g. "1" or "ETH").
- `from_token`: Source token contract address (or sentinel for native).
- `slippage`: Slippage tolerance as a decimal (0.005 = 0.5%).
- `to_address`: Receiver wallet address. Defaults to fromAddress.
- `to_chain`: Destination chain id or canonical name.
- `to_token`: Destination token contract address (or sentinel for native).
*/
    pub async fn get_quote<'a>(
        &'a self,
        from_address: &'a str,
        from_amount: &'a str,
        from_chain: &'a str,
        from_token: &'a str,
        slippage: Option<f64>,
        to_address: Option<&'a str>,
        to_chain: &'a str,
        to_token: &'a str,
    ) -> Result<ResponseValue<types::LifiQuote>, Error<()>> {
        let url = format!("{}/v1/quote", self.baseurl,);
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
            .query(&progenitor_client::QueryParam::new("fromAddress", &from_address))
            .query(&progenitor_client::QueryParam::new("fromAmount", &from_amount))
            .query(&progenitor_client::QueryParam::new("fromChain", &from_chain))
            .query(&progenitor_client::QueryParam::new("fromToken", &from_token))
            .query(&progenitor_client::QueryParam::new("slippage", &slippage))
            .query(&progenitor_client::QueryParam::new("toAddress", &to_address))
            .query(&progenitor_client::QueryParam::new("toChain", &to_chain))
            .query(&progenitor_client::QueryParam::new("toToken", &to_token))
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
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Track the lifecycle of a cross-chain transfer by source-chain tx hash

Sends a `GET` request to `/v1/status`

*/
    pub async fn get_status<'a>(
        &'a self,
        bridge: Option<&'a str>,
        from_chain: Option<&'a str>,
        to_chain: Option<&'a str>,
        tx_hash: &'a str,
    ) -> Result<ResponseValue<types::LifiStatus>, Error<()>> {
        let url = format!("{}/v1/status", self.baseurl,);
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
            .query(&progenitor_client::QueryParam::new("bridge", &bridge))
            .query(&progenitor_client::QueryParam::new("fromChain", &from_chain))
            .query(&progenitor_client::QueryParam::new("toChain", &to_chain))
            .query(&progenitor_client::QueryParam::new("txHash", &tx_hash))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "get_status",
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
    /**List supported chains

Sends a `GET` request to `/v1/chains`

Arguments:
- `chain_types`: Comma-separated chain types (EVM, SVM).
*/
    pub async fn get_chains<'a>(
        &'a self,
        chain_types: Option<&'a str>,
    ) -> Result<ResponseValue<types::LifiChainsResponse>, Error<()>> {
        let url = format!("{}/v1/chains", self.baseurl,);
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
            .query(&progenitor_client::QueryParam::new("chainTypes", &chain_types))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "get_chains",
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
    /**List supported tokens, optionally filtered by chain

Sends a `GET` request to `/v1/tokens`

Arguments:
- `chain_types`: Comma-separated chain types (EVM, SVM).
- `chains`: Comma-separated chain IDs (e.g. "1,137,8453").
*/
    pub async fn get_tokens<'a>(
        &'a self,
        chain_types: Option<&'a str>,
        chains: Option<&'a str>,
    ) -> Result<ResponseValue<types::LifiTokensResponse>, Error<()>> {
        let url = format!("{}/v1/tokens", self.baseurl,);
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
            .query(&progenitor_client::QueryParam::new("chainTypes", &chain_types))
            .query(&progenitor_client::QueryParam::new("chains", &chains))
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
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
}
/// Items consumers will typically use such as the Client.
pub mod prelude {
    #[allow(unused_imports)]
    pub use super::Client;
}
