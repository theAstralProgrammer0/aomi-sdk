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
    ///`YearnApr`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "fees": {
    ///      "$ref": "#/components/schemas/YearnAprFees"
    ///    },
    ///    "forwardAPR": {
    ///      "$ref": "#/components/schemas/YearnForwardApr"
    ///    },
    ///    "netAPR": {
    ///      "description": "Realised net APY (post-fee). Decimal: 0.05 = 5%.",
    ///      "type": [
    ///        "number",
    ///        "null"
    ///      ],
    ///      "format": "double"
    ///    },
    ///    "type": {
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
    pub struct YearnApr {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub fees: ::std::option::Option<YearnAprFees>,
        #[serde(
            rename = "forwardAPR",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub forward_apr: ::std::option::Option<YearnForwardApr>,
        ///Realised net APY (post-fee). Decimal: 0.05 = 5%.
        #[serde(
            rename = "netAPR",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub net_apr: ::std::option::Option<f64>,
        #[serde(
            rename = "type",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub type_: ::std::option::Option<::std::string::String>,
    }
    impl ::std::default::Default for YearnApr {
        fn default() -> Self {
            Self {
                fees: Default::default(),
                forward_apr: Default::default(),
                net_apr: Default::default(),
                type_: Default::default(),
            }
        }
    }
    ///`YearnAprFees`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "management": {
    ///      "type": [
    ///        "number",
    ///        "null"
    ///      ],
    ///      "format": "double"
    ///    },
    ///    "performance": {
    ///      "type": [
    ///        "number",
    ///        "null"
    ///      ],
    ///      "format": "double"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct YearnAprFees {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub management: ::std::option::Option<f64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub performance: ::std::option::Option<f64>,
    }
    impl ::std::default::Default for YearnAprFees {
        fn default() -> Self {
            Self {
                management: Default::default(),
                performance: Default::default(),
            }
        }
    }
    ///`YearnForwardApr`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "netAPR": {
    ///      "description": "Forward-looking net APY (post-fee). Decimal.",
    ///      "type": [
    ///        "number",
    ///        "null"
    ///      ],
    ///      "format": "double"
    ///    },
    ///    "type": {
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
    pub struct YearnForwardApr {
        ///Forward-looking net APY (post-fee). Decimal.
        #[serde(
            rename = "netAPR",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub net_apr: ::std::option::Option<f64>,
        #[serde(
            rename = "type",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub type_: ::std::option::Option<::std::string::String>,
    }
    impl ::std::default::Default for YearnForwardApr {
        fn default() -> Self {
            Self {
                net_apr: Default::default(),
                type_: Default::default(),
            }
        }
    }
    ///`YearnTvl`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "price": {
    ///      "description": "Underlying token price in USD",
    ///      "type": [
    ///        "number",
    ///        "null"
    ///      ],
    ///      "format": "double"
    ///    },
    ///    "totalAssets": {
    ///      "description": "Raw token amount as decimal string",
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "tvl": {
    ///      "description": "USD TVL",
    ///      "type": [
    ///        "number",
    ///        "null"
    ///      ],
    ///      "format": "double"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct YearnTvl {
        ///Underlying token price in USD
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub price: ::std::option::Option<f64>,
        ///Raw token amount as decimal string
        #[serde(
            rename = "totalAssets",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub total_assets: ::std::option::Option<::std::string::String>,
        ///USD TVL
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub tvl: ::std::option::Option<f64>,
    }
    impl ::std::default::Default for YearnTvl {
        fn default() -> Self {
            Self {
                price: Default::default(),
                total_assets: Default::default(),
                tvl: Default::default(),
            }
        }
    }
    ///`YearnUnderlyingToken`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "address": {
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "decimals": {
    ///      "type": [
    ///        "integer",
    ///        "null"
    ///      ],
    ///      "format": "int64"
    ///    },
    ///    "name": {
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "symbol": {
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
    pub struct YearnUnderlyingToken {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub address: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub decimals: ::std::option::Option<i64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub name: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub symbol: ::std::option::Option<::std::string::String>,
    }
    impl ::std::default::Default for YearnUnderlyingToken {
        fn default() -> Self {
            Self {
                address: Default::default(),
                decimals: Default::default(),
                name: Default::default(),
                symbol: Default::default(),
            }
        }
    }
    ///`YearnVault`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "address",
    ///    "chainID"
    ///  ],
    ///  "properties": {
    ///    "address": {
    ///      "description": "Vault contract address (0x...)",
    ///      "type": "string"
    ///    },
    ///    "apr": {
    ///      "$ref": "#/components/schemas/YearnApr"
    ///    },
    ///    "category": {
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "chainID": {
    ///      "type": "integer",
    ///      "format": "int64"
    ///    },
    ///    "kind": {
    ///      "description": "e.g. 'Single Strategy', 'Multi Strategy', 'Legacy'",
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "name": {
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "symbol": {
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "token": {
    ///      "$ref": "#/components/schemas/YearnUnderlyingToken"
    ///    },
    ///    "tvl": {
    ///      "$ref": "#/components/schemas/YearnTvl"
    ///    },
    ///    "version": {
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
    pub struct YearnVault {
        ///Vault contract address (0x...)
        pub address: ::std::string::String,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub apr: ::std::option::Option<YearnApr>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub category: ::std::option::Option<::std::string::String>,
        #[serde(rename = "chainID")]
        pub chain_id: i64,
        ///e.g. 'Single Strategy', 'Multi Strategy', 'Legacy'
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub kind: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub name: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub symbol: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub token: ::std::option::Option<YearnUnderlyingToken>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub tvl: ::std::option::Option<YearnTvl>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub version: ::std::option::Option<::std::string::String>,
    }
}
#[derive(Clone, Debug)]
/**Client for Yearn yDaemon API

Yearn yDaemon REST API — public, no auth required.

Covers the three endpoints used by the Aomi `yearn` app:
  - GET /{chainId}/vaults/all
  - GET /{chainId}/vaults/{address}
  - GET /info/vaults/blacklisted

Default host is `https://ydaemon.yearn.fi`. Response schemas are
intentionally narrowed to the subset of fields that the assistant
actually reasons about (address, symbol, TVL, net APY, fees, kind,
version, underlying token). The live API returns many more fields
(icons, descriptions, risk vectors, debts, migration metadata, ...);
those are dropped by serde at deserialization time.


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
    /**List every vault on a chain (TVL, APY/APR, strategies, fees, etc.)

Sends a `GET` request to `/{chainId}/vaults/all`

Arguments:
- `chain_id`: Chain ID (e.g. 1, 10, 137, 250, 8453, 42161).
*/
    pub async fn get_all_vaults<'a>(
        &'a self,
        chain_id: i64,
    ) -> Result<ResponseValue<::std::vec::Vec<types::YearnVault>>, Error<()>> {
        let url = format!(
            "{}/{}/vaults/all", self.baseurl, encode_path(& chain_id.to_string()),
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
            operation_id: "get_all_vaults",
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
    /**Deep-dive a single vault by address

Sends a `GET` request to `/{chainId}/vaults/{address}`

Arguments:
- `chain_id`
- `address`: Vault contract address (0x...).
*/
    pub async fn get_vault_detail<'a>(
        &'a self,
        chain_id: i64,
        address: &'a str,
    ) -> Result<ResponseValue<types::YearnVault>, Error<()>> {
        let url = format!(
            "{}/{}/vaults/{}", self.baseurl, encode_path(& chain_id.to_string()),
            encode_path(& address.to_string()),
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
            operation_id: "get_vault_detail",
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
    /**Cross-chain list of vaults removed from the official Yearn UI

Sends a `GET` request to `/info/vaults/blacklisted`

*/
    pub async fn get_blacklisted_vaults<'a>(
        &'a self,
    ) -> Result<ResponseValue<::std::vec::Vec<::std::string::String>>, Error<()>> {
        let url = format!("{}/info/vaults/blacklisted", self.baseurl,);
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
            operation_id: "get_blacklisted_vaults",
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
