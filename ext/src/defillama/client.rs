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
    ///`ChainTvl`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "name",
    ///    "tvl"
    ///  ],
    ///  "properties": {
    ///    "chainId": {
    ///      "oneOf": [
    ///        {
    ///          "type": "null"
    ///        },
    ///        {
    ///          "oneOf": [
    ///            {
    ///              "type": "integer"
    ///            },
    ///            {
    ///              "type": "string"
    ///            }
    ///          ]
    ///        }
    ///      ]
    ///    },
    ///    "name": {
    ///      "type": "string"
    ///    },
    ///    "tokenSymbol": {
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "tvl": {
    ///      "type": "number"
    ///    }
    ///  },
    ///  "additionalProperties": true
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ChainTvl {
        #[serde(
            rename = "chainId",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub chain_id: ::std::option::Option<ChainTvlChainId>,
        pub name: ::std::string::String,
        #[serde(
            rename = "tokenSymbol",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub token_symbol: ::std::option::Option<::std::string::String>,
        pub tvl: f64,
    }
    ///`ChainTvlChainId`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "oneOf": [
    ///    {
    ///      "type": "integer"
    ///    },
    ///    {
    ///      "type": "string"
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    #[serde(untagged)]
    pub enum ChainTvlChainId {
        Integer(i64),
        String(::std::string::String),
    }
    impl ::std::fmt::Display for ChainTvlChainId {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                Self::Integer(x) => x.fmt(f),
                Self::String(x) => x.fmt(f),
            }
        }
    }
    impl ::std::convert::From<i64> for ChainTvlChainId {
        fn from(value: i64) -> Self {
            Self::Integer(value)
        }
    }
    ///`CoinPrice`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "confidence": {
    ///      "type": [
    ///        "number",
    ///        "null"
    ///      ]
    ///    },
    ///    "decimals": {
    ///      "type": "integer"
    ///    },
    ///    "price": {
    ///      "type": "number"
    ///    },
    ///    "symbol": {
    ///      "type": "string"
    ///    },
    ///    "timestamp": {
    ///      "type": "integer",
    ///      "format": "int64"
    ///    }
    ///  },
    ///  "additionalProperties": true
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct CoinPrice {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub confidence: ::std::option::Option<f64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub decimals: ::std::option::Option<i64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub price: ::std::option::Option<f64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub symbol: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub timestamp: ::std::option::Option<i64>,
    }
    impl ::std::default::Default for CoinPrice {
        fn default() -> Self {
            Self {
                confidence: Default::default(),
                decimals: Default::default(),
                price: Default::default(),
                symbol: Default::default(),
                timestamp: Default::default(),
            }
        }
    }
    ///`CoinPriceMap`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "coins": {
    ///      "type": "object",
    ///      "additionalProperties": {
    ///        "$ref": "#/components/schemas/CoinPrice"
    ///      }
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct CoinPriceMap {
        #[serde(
            default,
            skip_serializing_if = ":: std :: collections :: HashMap::is_empty"
        )]
        pub coins: ::std::collections::HashMap<::std::string::String, CoinPrice>,
    }
    impl ::std::default::Default for CoinPriceMap {
        fn default() -> Self {
            Self { coins: Default::default() }
        }
    }
    ///`ProtocolSummary`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "name"
    ///  ],
    ///  "properties": {
    ///    "category": {
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "chainTvls": {
    ///      "type": "object",
    ///      "additionalProperties": {
    ///        "type": "number"
    ///      }
    ///    },
    ///    "chains": {
    ///      "type": "array",
    ///      "items": {
    ///        "type": "string"
    ///      }
    ///    },
    ///    "id": {
    ///      "type": "string"
    ///    },
    ///    "name": {
    ///      "type": "string"
    ///    },
    ///    "symbol": {
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "tvl": {
    ///      "type": [
    ///        "number",
    ///        "null"
    ///      ]
    ///    }
    ///  },
    ///  "additionalProperties": true
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ProtocolSummary {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub category: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "chainTvls",
            default,
            skip_serializing_if = ":: std :: collections :: HashMap::is_empty"
        )]
        pub chain_tvls: ::std::collections::HashMap<::std::string::String, f64>,
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub chains: ::std::vec::Vec<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub id: ::std::option::Option<::std::string::String>,
        pub name: ::std::string::String,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub symbol: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub tvl: ::std::option::Option<f64>,
    }
    ///`TvlPoint`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "date",
    ///    "tvl"
    ///  ],
    ///  "properties": {
    ///    "date": {
    ///      "description": "Unix epoch seconds.",
    ///      "type": "integer",
    ///      "format": "int64"
    ///    },
    ///    "tvl": {
    ///      "type": "number"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct TvlPoint {
        ///Unix epoch seconds.
        pub date: i64,
        pub tvl: f64,
    }
    ///`YieldPool`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "chain",
    ///    "pool",
    ///    "project",
    ///    "symbol"
    ///  ],
    ///  "properties": {
    ///    "apy": {
    ///      "type": [
    ///        "number",
    ///        "null"
    ///      ]
    ///    },
    ///    "apyBase": {
    ///      "type": [
    ///        "number",
    ///        "null"
    ///      ]
    ///    },
    ///    "apyReward": {
    ///      "type": [
    ///        "number",
    ///        "null"
    ///      ]
    ///    },
    ///    "chain": {
    ///      "type": "string"
    ///    },
    ///    "exposure": {
    ///      "description": "\"single\" or \"multi\".",
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "ilRisk": {
    ///      "description": "\"yes\" or \"no\".",
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "pool": {
    ///      "description": "Pool UUID; feed to /chart/{pool}.",
    ///      "type": "string"
    ///    },
    ///    "project": {
    ///      "type": "string"
    ///    },
    ///    "stablecoin": {
    ///      "type": [
    ///        "boolean",
    ///        "null"
    ///      ]
    ///    },
    ///    "symbol": {
    ///      "type": "string"
    ///    },
    ///    "tvlUsd": {
    ///      "type": [
    ///        "number",
    ///        "null"
    ///      ]
    ///    }
    ///  },
    ///  "additionalProperties": true
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct YieldPool {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub apy: ::std::option::Option<f64>,
        #[serde(
            rename = "apyBase",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub apy_base: ::std::option::Option<f64>,
        #[serde(
            rename = "apyReward",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub apy_reward: ::std::option::Option<f64>,
        pub chain: ::std::string::String,
        ///"single" or "multi".
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub exposure: ::std::option::Option<::std::string::String>,
        ///"yes" or "no".
        #[serde(
            rename = "ilRisk",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub il_risk: ::std::option::Option<::std::string::String>,
        ///Pool UUID; feed to /chart/{pool}.
        pub pool: ::std::string::String,
        pub project: ::std::string::String,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub stablecoin: ::std::option::Option<bool>,
        pub symbol: ::std::string::String,
        #[serde(
            rename = "tvlUsd",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub tvl_usd: ::std::option::Option<f64>,
    }
    ///`YieldPoolList`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "data"
    ///  ],
    ///  "properties": {
    ///    "data": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/YieldPool"
    ///      }
    ///    },
    ///    "status": {
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct YieldPoolList {
        pub data: ::std::vec::Vec<YieldPool>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub status: ::std::option::Option<::std::string::String>,
    }
}
#[derive(Clone, Debug)]
/**Client for DefiLlama API

Free public DefiLlama REST API.

## Hosts
DefiLlama splits its surface across four hosts:
  - `https://api.llama.fi`         — protocols, TVL, fees, dex/options/perps overviews, blocks
  - `https://coins.llama.fi`       — token prices (current, historical, batch, chart, %, first)
  - `https://yields.llama.fi`      — yield pools and per-pool charts
  - `https://stablecoins.llama.fi` — stablecoin mcap, chain distribution, prices

Each operation declares its own `servers` entry to pin the host.

## Auth
All endpoints in this spec are unauthenticated. DefiLlama also exposes a
Pro API at `https://pro-api.llama.fi/{key}/...` which is intentionally NOT
covered here.


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
    /**List all protocols with current TVL

Sends a `GET` request to `/protocols`

Arguments:
- `category`: Optional category filter (e.g. "Lending", "Dexes").
*/
    pub async fn get_protocols<'a>(
        &'a self,
        category: Option<&'a str>,
    ) -> Result<ResponseValue<::std::vec::Vec<types::ProtocolSummary>>, Error<()>> {
        let url = format!("{}/protocols", self.baseurl,);
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
            .query(&progenitor_client::QueryParam::new("category", &category))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "get_protocols",
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
    /**Historical TVL of a protocol with token and chain breakdowns

Sends a `GET` request to `/protocol/{protocol}`

Arguments:
- `protocol`: Protocol slug (e.g. "uniswap", "aave-v3").
*/
    pub async fn get_protocol_detail<'a>(
        &'a self,
        protocol: &'a str,
    ) -> Result<
        ResponseValue<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
        Error<()>,
    > {
        let url = format!(
            "{}/protocol/{}", self.baseurl, encode_path(& protocol.to_string()),
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
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "get_protocol_detail",
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
    /**Current TVL of a protocol (single number)

Sends a `GET` request to `/tvl/{protocol}`

Arguments:
- `protocol`: Protocol slug (e.g. "uniswap", "aave-v3").
*/
    pub async fn get_protocol_current_tvl<'a>(
        &'a self,
        protocol: &'a str,
    ) -> Result<ResponseValue<f64>, Error<()>> {
        let url = format!(
            "{}/tvl/{}", self.baseurl, encode_path(& protocol.to_string()),
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
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "get_protocol_current_tvl",
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
    /**Current TVL of all chains

Sends a `GET` request to `/v2/chains`

*/
    pub async fn get_chains_tvl<'a>(
        &'a self,
    ) -> Result<ResponseValue<::std::vec::Vec<types::ChainTvl>>, Error<()>> {
        let url = format!("{}/v2/chains", self.baseurl,);
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
            operation_id: "get_chains_tvl",
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
    /**Historical TVL across all chains

Sends a `GET` request to `/v2/historicalChainTvl`

*/
    pub async fn get_historical_chain_tvl_all<'a>(
        &'a self,
    ) -> Result<ResponseValue<::std::vec::Vec<types::TvlPoint>>, Error<()>> {
        let url = format!("{}/v2/historicalChainTvl", self.baseurl,);
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
            operation_id: "get_historical_chain_tvl_all",
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
    /**Historical TVL of a single chain

Sends a `GET` request to `/v2/historicalChainTvl/{chain}`

Arguments:
- `chain`: Chain slug (e.g. "ethereum", "arbitrum").
*/
    pub async fn get_historical_chain_tvl<'a>(
        &'a self,
        chain: &'a str,
    ) -> Result<ResponseValue<::std::vec::Vec<types::TvlPoint>>, Error<()>> {
        let url = format!(
            "{}/v2/historicalChainTvl/{}", self.baseurl, encode_path(& chain
            .to_string()),
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
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "get_historical_chain_tvl",
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
    /**All DEXs with volume summaries

Sends a `GET` request to `/overview/dexs`

*/
    pub async fn get_dex_overview<'a>(
        &'a self,
        data_type: Option<&'a str>,
        exclude_total_data_chart: Option<bool>,
        exclude_total_data_chart_breakdown: Option<bool>,
    ) -> Result<
        ResponseValue<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
        Error<()>,
    > {
        let url = format!("{}/overview/dexs", self.baseurl,);
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
            .query(&progenitor_client::QueryParam::new("dataType", &data_type))
            .query(
                &progenitor_client::QueryParam::new(
                    "excludeTotalDataChart",
                    &exclude_total_data_chart,
                ),
            )
            .query(
                &progenitor_client::QueryParam::new(
                    "excludeTotalDataChartBreakdown",
                    &exclude_total_data_chart_breakdown,
                ),
            )
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "get_dex_overview",
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
    /**All DEXs on a chain

Sends a `GET` request to `/overview/dexs/{chain}`

Arguments:
- `chain`: Chain slug (e.g. "ethereum", "arbitrum").
- `exclude_total_data_chart`
- `exclude_total_data_chart_breakdown`
*/
    pub async fn get_dex_overview_by_chain<'a>(
        &'a self,
        chain: &'a str,
        exclude_total_data_chart: Option<bool>,
        exclude_total_data_chart_breakdown: Option<bool>,
    ) -> Result<
        ResponseValue<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
        Error<()>,
    > {
        let url = format!(
            "{}/overview/dexs/{}", self.baseurl, encode_path(& chain.to_string()),
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
            .query(
                &progenitor_client::QueryParam::new(
                    "excludeTotalDataChart",
                    &exclude_total_data_chart,
                ),
            )
            .query(
                &progenitor_client::QueryParam::new(
                    "excludeTotalDataChartBreakdown",
                    &exclude_total_data_chart_breakdown,
                ),
            )
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "get_dex_overview_by_chain",
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
    /**Volume summary for a single DEX protocol

Sends a `GET` request to `/summary/dexs/{protocol}`

Arguments:
- `protocol`: Protocol slug (e.g. "uniswap", "aave-v3").
- `data_type`
- `exclude_total_data_chart`
- `exclude_total_data_chart_breakdown`
*/
    pub async fn get_dex_protocol_volume<'a>(
        &'a self,
        protocol: &'a str,
        data_type: Option<&'a str>,
        exclude_total_data_chart: Option<bool>,
        exclude_total_data_chart_breakdown: Option<bool>,
    ) -> Result<
        ResponseValue<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
        Error<()>,
    > {
        let url = format!(
            "{}/summary/dexs/{}", self.baseurl, encode_path(& protocol.to_string()),
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
            .query(&progenitor_client::QueryParam::new("dataType", &data_type))
            .query(
                &progenitor_client::QueryParam::new(
                    "excludeTotalDataChart",
                    &exclude_total_data_chart,
                ),
            )
            .query(
                &progenitor_client::QueryParam::new(
                    "excludeTotalDataChartBreakdown",
                    &exclude_total_data_chart_breakdown,
                ),
            )
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "get_dex_protocol_volume",
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
    /**All protocols with fee/revenue summaries

Sends a `GET` request to `/overview/fees`

*/
    pub async fn get_fees_overview<'a>(
        &'a self,
        data_type: Option<&'a str>,
    ) -> Result<
        ResponseValue<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
        Error<()>,
    > {
        let url = format!("{}/overview/fees", self.baseurl,);
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
            .query(&progenitor_client::QueryParam::new("dataType", &data_type))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "get_fees_overview",
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
    /**Fee/revenue summary for a single protocol

Sends a `GET` request to `/summary/fees/{protocol}`

Arguments:
- `protocol`: Protocol slug (e.g. "uniswap", "aave-v3").
- `data_type`
*/
    pub async fn get_protocol_fees<'a>(
        &'a self,
        protocol: &'a str,
        data_type: Option<&'a str>,
    ) -> Result<
        ResponseValue<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
        Error<()>,
    > {
        let url = format!(
            "{}/summary/fees/{}", self.baseurl, encode_path(& protocol.to_string()),
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
            .query(&progenitor_client::QueryParam::new("dataType", &data_type))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "get_protocol_fees",
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
    /**Current prices of tokens by `chain:address` (or coingecko id)

Sends a `GET` request to `/prices/current/{coins}`

Arguments:
- `coins`: Comma-separated list of coin IDs in `chain:address` format
(e.g. `ethereum:0x...`) or coingecko ID (`coingecko:bitcoin`).

- `search_width`
*/
    pub async fn get_current_prices<'a>(
        &'a self,
        coins: &'a str,
        search_width: Option<&'a str>,
    ) -> Result<ResponseValue<types::CoinPriceMap>, Error<()>> {
        let url = format!(
            "{}/prices/current/{}", self.baseurl, encode_path(& coins.to_string()),
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
            .query(&progenitor_client::QueryParam::new("searchWidth", &search_width))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "get_current_prices",
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
    /**Prices at a single past timestamp

Sends a `GET` request to `/prices/historical/{timestamp}/{coins}`

Arguments:
- `timestamp`
- `coins`: Comma-separated list of coin IDs in `chain:address` format
(e.g. `ethereum:0x...`) or coingecko ID (`coingecko:bitcoin`).

- `search_width`
*/
    pub async fn get_historical_prices<'a>(
        &'a self,
        timestamp: i64,
        coins: &'a str,
        search_width: Option<&'a str>,
    ) -> Result<ResponseValue<types::CoinPriceMap>, Error<()>> {
        let url = format!(
            "{}/prices/historical/{}/{}", self.baseurl, encode_path(& timestamp
            .to_string()), encode_path(& coins.to_string()),
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
            .query(&progenitor_client::QueryParam::new("searchWidth", &search_width))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "get_historical_prices",
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
    /**Percentage change in price over a window

Sends a `GET` request to `/percentage/{coins}`

Arguments:
- `coins`: Comma-separated list of coin IDs in `chain:address` format
(e.g. `ethereum:0x...`) or coingecko ID (`coingecko:bitcoin`).

- `look_forward`
- `period`
- `timestamp`
*/
    pub async fn get_price_change_percentage<'a>(
        &'a self,
        coins: &'a str,
        look_forward: Option<bool>,
        period: Option<&'a str>,
        timestamp: Option<i64>,
    ) -> Result<
        ResponseValue<::std::collections::HashMap<::std::string::String, f64>>,
        Error<()>,
    > {
        let url = format!(
            "{}/percentage/{}", self.baseurl, encode_path(& coins.to_string()),
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
            .query(&progenitor_client::QueryParam::new("lookForward", &look_forward))
            .query(&progenitor_client::QueryParam::new("period", &period))
            .query(&progenitor_client::QueryParam::new("timestamp", &timestamp))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "get_price_change_percentage",
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
    /**All yield pools enriched with predictions and stats

Sends a `GET` request to `/pools`

*/
    pub async fn get_yield_pools<'a>(
        &'a self,
        chain: Option<&'a str>,
        project: Option<&'a str>,
    ) -> Result<ResponseValue<types::YieldPoolList>, Error<()>> {
        let url = format!("{}/pools", self.baseurl,);
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
            .query(&progenitor_client::QueryParam::new("chain", &chain))
            .query(&progenitor_client::QueryParam::new("project", &project))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "get_yield_pools",
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
    /**Historical APY and TVL of a single pool

Sends a `GET` request to `/chart/{pool}`

*/
    pub async fn get_yield_pool_history<'a>(
        &'a self,
        pool: &'a str,
    ) -> Result<
        ResponseValue<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
        Error<()>,
    > {
        let url = format!("{}/chart/{}", self.baseurl, encode_path(& pool.to_string()),);
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
            operation_id: "get_yield_pool_history",
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
    /**All stablecoins with circulating amounts

Sends a `GET` request to `/stablecoins`

*/
    pub async fn get_stablecoins<'a>(
        &'a self,
        include_prices: Option<bool>,
    ) -> Result<
        ResponseValue<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
        Error<()>,
    > {
        let url = format!("{}/stablecoins", self.baseurl,);
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
            .query(&progenitor_client::QueryParam::new("includePrices", &include_prices))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "get_stablecoins",
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
    /**Current mcap sum of all stablecoins per chain

Sends a `GET` request to `/stablecoinchains`

*/
    pub async fn get_stablecoin_chains<'a>(
        &'a self,
    ) -> Result<
        ResponseValue<
            ::std::vec::Vec<
                ::serde_json::Map<::std::string::String, ::serde_json::Value>,
            >,
        >,
        Error<()>,
    > {
        let url = format!("{}/stablecoinchains", self.baseurl,);
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
            operation_id: "get_stablecoin_chains",
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
    /**Historical mcap and chain distribution of a single stablecoin

Sends a `GET` request to `/stablecoin/{asset}`

Arguments:
- `asset`: Stablecoin id (e.g. "1" for USDT).
*/
    pub async fn get_stablecoin_history<'a>(
        &'a self,
        asset: &'a str,
    ) -> Result<
        ResponseValue<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
        Error<()>,
    > {
        let url = format!(
            "{}/stablecoin/{}", self.baseurl, encode_path(& asset.to_string()),
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
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "get_stablecoin_history",
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
