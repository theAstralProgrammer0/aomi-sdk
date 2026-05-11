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
    ///`AcrossDepositStatus`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "status"
    ///  ],
    ///  "properties": {
    ///    "status": {
    ///      "type": "string"
    ///    }
    ///  },
    ///  "additionalProperties": true
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct AcrossDepositStatus {
        pub status: ::std::string::String,
    }
    ///`AcrossFeeComponent`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "pct",
    ///    "total"
    ///  ],
    ///  "properties": {
    ///    "pct": {
    ///      "type": "string"
    ///    },
    ///    "total": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct AcrossFeeComponent {
        pub pct: ::std::string::String,
        pub total: ::std::string::String,
    }
    ///`AcrossLimits`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "maxDeposit",
    ///    "maxDepositInstant",
    ///    "maxDepositShortDelay",
    ///    "minDeposit",
    ///    "recommendedDepositInstant"
    ///  ],
    ///  "properties": {
    ///    "maxDeposit": {
    ///      "type": "string"
    ///    },
    ///    "maxDepositInstant": {
    ///      "type": "string"
    ///    },
    ///    "maxDepositShortDelay": {
    ///      "type": "string"
    ///    },
    ///    "minDeposit": {
    ///      "type": "string"
    ///    },
    ///    "recommendedDepositInstant": {
    ///      "type": "string"
    ///    }
    ///  },
    ///  "additionalProperties": true
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct AcrossLimits {
        #[serde(rename = "maxDeposit")]
        pub max_deposit: ::std::string::String,
        #[serde(rename = "maxDepositInstant")]
        pub max_deposit_instant: ::std::string::String,
        #[serde(rename = "maxDepositShortDelay")]
        pub max_deposit_short_delay: ::std::string::String,
        #[serde(rename = "minDeposit")]
        pub min_deposit: ::std::string::String,
        #[serde(rename = "recommendedDepositInstant")]
        pub recommended_deposit_instant: ::std::string::String,
    }
    ///`AcrossRoute`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "destinationChainId",
    ///    "destinationToken",
    ///    "originChainId",
    ///    "originToken"
    ///  ],
    ///  "properties": {
    ///    "destinationChainId": {
    ///      "type": "integer",
    ///      "format": "int64"
    ///    },
    ///    "destinationToken": {
    ///      "type": "string"
    ///    },
    ///    "destinationTokenSymbol": {
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "isNative": {
    ///      "type": [
    ///        "boolean",
    ///        "null"
    ///      ]
    ///    },
    ///    "originChainId": {
    ///      "type": "integer",
    ///      "format": "int64"
    ///    },
    ///    "originToken": {
    ///      "type": "string"
    ///    },
    ///    "originTokenSymbol": {
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    }
    ///  },
    ///  "additionalProperties": true
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct AcrossRoute {
        #[serde(rename = "destinationChainId")]
        pub destination_chain_id: i64,
        #[serde(rename = "destinationToken")]
        pub destination_token: ::std::string::String,
        #[serde(
            rename = "destinationTokenSymbol",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub destination_token_symbol: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "isNative",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub is_native: ::std::option::Option<bool>,
        #[serde(rename = "originChainId")]
        pub origin_chain_id: i64,
        #[serde(rename = "originToken")]
        pub origin_token: ::std::string::String,
        #[serde(
            rename = "originTokenSymbol",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub origin_token_symbol: ::std::option::Option<::std::string::String>,
    }
    ///`AcrossSuggestedFees`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "inputToken",
    ///    "lpFee",
    ///    "outputAmount",
    ///    "outputToken",
    ///    "relayerCapitalFee",
    ///    "relayerGasFee",
    ///    "totalRelayFee"
    ///  ],
    ///  "properties": {
    ///    "estimatedFillTimeSec": {
    ///      "type": [
    ///        "integer",
    ///        "null"
    ///      ],
    ///      "format": "int64"
    ///    },
    ///    "inputToken": {
    ///      "$ref": "#/components/schemas/AcrossTokenRef"
    ///    },
    ///    "limits": {
    ///      "oneOf": [
    ///        {
    ///          "type": "null"
    ///        },
    ///        {
    ///          "allOf": [
    ///            {
    ///              "$ref": "#/components/schemas/AcrossLimits"
    ///            }
    ///          ]
    ///        }
    ///      ]
    ///    },
    ///    "lpFee": {
    ///      "$ref": "#/components/schemas/AcrossFeeComponent"
    ///    },
    ///    "outputAmount": {
    ///      "type": "string"
    ///    },
    ///    "outputToken": {
    ///      "$ref": "#/components/schemas/AcrossTokenRef"
    ///    },
    ///    "relayerCapitalFee": {
    ///      "$ref": "#/components/schemas/AcrossFeeComponent"
    ///    },
    ///    "relayerGasFee": {
    ///      "$ref": "#/components/schemas/AcrossFeeComponent"
    ///    },
    ///    "totalRelayFee": {
    ///      "$ref": "#/components/schemas/AcrossFeeComponent"
    ///    }
    ///  },
    ///  "additionalProperties": true
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct AcrossSuggestedFees {
        #[serde(
            rename = "estimatedFillTimeSec",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub estimated_fill_time_sec: ::std::option::Option<i64>,
        #[serde(rename = "inputToken")]
        pub input_token: AcrossTokenRef,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub limits: ::std::option::Option<AcrossLimits>,
        #[serde(rename = "lpFee")]
        pub lp_fee: AcrossFeeComponent,
        #[serde(rename = "outputAmount")]
        pub output_amount: ::std::string::String,
        #[serde(rename = "outputToken")]
        pub output_token: AcrossTokenRef,
        #[serde(rename = "relayerCapitalFee")]
        pub relayer_capital_fee: AcrossFeeComponent,
        #[serde(rename = "relayerGasFee")]
        pub relayer_gas_fee: AcrossFeeComponent,
        #[serde(rename = "totalRelayFee")]
        pub total_relay_fee: AcrossFeeComponent,
    }
    ///`AcrossTokenRef`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "address",
    ///    "chainId",
    ///    "decimals",
    ///    "symbol"
    ///  ],
    ///  "properties": {
    ///    "address": {
    ///      "type": "string"
    ///    },
    ///    "chainId": {
    ///      "type": "integer",
    ///      "format": "int64"
    ///    },
    ///    "decimals": {
    ///      "type": "integer",
    ///      "format": "int64"
    ///    },
    ///    "symbol": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct AcrossTokenRef {
        pub address: ::std::string::String,
        #[serde(rename = "chainId")]
        pub chain_id: i64,
        pub decimals: i64,
        pub symbol: ::std::string::String,
    }
}
#[derive(Clone, Debug)]
/**Client for Across Protocol API

Across Protocol bridge REST API — public, no auth required.

Covers the four endpoints used by the Aomi `across` app:
  - GET /available-routes
  - GET /limits
  - GET /suggested-fees
  - GET /deposit/status

Default host is `https://app.across.to/api`. All endpoints are unauthenticated.


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
    /**List supported bridge routes (origin → destination token pairs)

    Sends a `GET` request to `/available-routes`

    */
    pub async fn get_available_routes<'a>(
        &'a self,
        destination_chain_id: Option<i64>,
        destination_token: Option<&'a str>,
        origin_chain_id: Option<i64>,
        origin_token: Option<&'a str>,
    ) -> Result<ResponseValue<::std::vec::Vec<types::AcrossRoute>>, Error<()>> {
        let url = format!("{}/available-routes", self.baseurl,);
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
                "destinationChainId",
                &destination_chain_id,
            ))
            .query(&progenitor_client::QueryParam::new(
                "destinationToken",
                &destination_token,
            ))
            .query(&progenitor_client::QueryParam::new(
                "originChainId",
                &origin_chain_id,
            ))
            .query(&progenitor_client::QueryParam::new(
                "originToken",
                &origin_token,
            ))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "get_available_routes",
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
    /**Min/max bridge amounts and instant-fill caps for a route

    Sends a `GET` request to `/limits`

    */
    pub async fn get_limits<'a>(
        &'a self,
        destination_chain_id: i64,
        input_token: &'a str,
        origin_chain_id: i64,
        output_token: &'a str,
    ) -> Result<ResponseValue<types::AcrossLimits>, Error<()>> {
        let url = format!("{}/limits", self.baseurl,);
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
                "destinationChainId",
                &destination_chain_id,
            ))
            .query(&progenitor_client::QueryParam::new(
                "inputToken",
                &input_token,
            ))
            .query(&progenitor_client::QueryParam::new(
                "originChainId",
                &origin_chain_id,
            ))
            .query(&progenitor_client::QueryParam::new(
                "outputToken",
                &output_token,
            ))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "get_limits",
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
    /**Fee quote and relayer parameters for a bridge

    Sends a `GET` request to `/suggested-fees`

    */
    pub async fn get_suggested_fees<'a>(
        &'a self,
        amount: &'a str,
        destination_chain_id: i64,
        input_token: &'a str,
        message: Option<&'a str>,
        origin_chain_id: i64,
        output_token: &'a str,
        recipient: Option<&'a str>,
    ) -> Result<ResponseValue<types::AcrossSuggestedFees>, Error<()>> {
        let url = format!("{}/suggested-fees", self.baseurl,);
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
                "destinationChainId",
                &destination_chain_id,
            ))
            .query(&progenitor_client::QueryParam::new(
                "inputToken",
                &input_token,
            ))
            .query(&progenitor_client::QueryParam::new("message", &message))
            .query(&progenitor_client::QueryParam::new(
                "originChainId",
                &origin_chain_id,
            ))
            .query(&progenitor_client::QueryParam::new(
                "outputToken",
                &output_token,
            ))
            .query(&progenitor_client::QueryParam::new("recipient", &recipient))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "get_suggested_fees",
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
    /**Current fill status for a submitted deposit

    Sends a `GET` request to `/deposit/status`

    */
    pub async fn get_deposit_status<'a>(
        &'a self,
        deposit_id: i64,
        origin_chain_id: i64,
    ) -> Result<ResponseValue<types::AcrossDepositStatus>, Error<()>> {
        let url = format!("{}/deposit/status", self.baseurl,);
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
                "depositId",
                &deposit_id,
            ))
            .query(&progenitor_client::QueryParam::new(
                "originChainId",
                &origin_chain_id,
            ))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "get_deposit_status",
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
