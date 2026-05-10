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
    ///`CreateWalletRequest`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "type",
    ///    "userIdentifier",
    ///    "userIdentifierType"
    ///  ],
    ///  "properties": {
    ///    "cosmosPrefix": {
    ///      "description": "Optional bech32 prefix (Cosmos wallets only)",
    ///      "type": "string"
    ///    },
    ///    "scheme": {
    ///      "description": "Optional MPC signing scheme",
    ///      "type": "string",
    ///      "enum": [
    ///        "DKLS",
    ///        "CGGMP",
    ///        "ED25519"
    ///      ]
    ///    },
    ///    "type": {
    ///      "description": "Wallet chain family",
    ///      "type": "string",
    ///      "enum": [
    ///        "EVM",
    ///        "SOLANA",
    ///        "COSMOS"
    ///      ]
    ///    },
    ///    "userIdentifier": {
    ///      "description": "User handle (email, phone, custom ID, etc.) the wallet is anchored to",
    ///      "type": "string"
    ///    },
    ///    "userIdentifierType": {
    ///      "description": "How Para should validate / normalize the userIdentifier",
    ///      "type": "string",
    ///      "enum": [
    ///        "EMAIL",
    ///        "PHONE",
    ///        "CUSTOM_ID",
    ///        "GUEST_ID",
    ///        "TELEGRAM",
    ///        "DISCORD",
    ///        "TWITTER"
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct CreateWalletRequest {
        ///Optional bech32 prefix (Cosmos wallets only)
        #[serde(
            rename = "cosmosPrefix",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub cosmos_prefix: ::std::option::Option<::std::string::String>,
        ///Optional MPC signing scheme
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub scheme: ::std::option::Option<CreateWalletRequestScheme>,
        ///Wallet chain family
        #[serde(rename = "type")]
        pub type_: CreateWalletRequestType,
        ///User handle (email, phone, custom ID, etc.) the wallet is anchored to
        #[serde(rename = "userIdentifier")]
        pub user_identifier: ::std::string::String,
        ///How Para should validate / normalize the userIdentifier
        #[serde(rename = "userIdentifierType")]
        pub user_identifier_type: CreateWalletRequestUserIdentifierType,
    }
    ///Optional MPC signing scheme
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Optional MPC signing scheme",
    ///  "type": "string",
    ///  "enum": [
    ///    "DKLS",
    ///    "CGGMP",
    ///    "ED25519"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        ::serde::Deserialize,
        ::serde::Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd
    )]
    pub enum CreateWalletRequestScheme {
        #[serde(rename = "DKLS")]
        Dkls,
        #[serde(rename = "CGGMP")]
        Cggmp,
        #[serde(rename = "ED25519")]
        Ed25519,
    }
    impl ::std::fmt::Display for CreateWalletRequestScheme {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Dkls => f.write_str("DKLS"),
                Self::Cggmp => f.write_str("CGGMP"),
                Self::Ed25519 => f.write_str("ED25519"),
            }
        }
    }
    impl ::std::str::FromStr for CreateWalletRequestScheme {
        type Err = self::error::ConversionError;
        fn from_str(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "DKLS" => Ok(Self::Dkls),
                "CGGMP" => Ok(Self::Cggmp),
                "ED25519" => Ok(Self::Ed25519),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for CreateWalletRequestScheme {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String> for CreateWalletRequestScheme {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for CreateWalletRequestScheme {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///Wallet chain family
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Wallet chain family",
    ///  "type": "string",
    ///  "enum": [
    ///    "EVM",
    ///    "SOLANA",
    ///    "COSMOS"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        ::serde::Deserialize,
        ::serde::Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd
    )]
    pub enum CreateWalletRequestType {
        #[serde(rename = "EVM")]
        Evm,
        #[serde(rename = "SOLANA")]
        Solana,
        #[serde(rename = "COSMOS")]
        Cosmos,
    }
    impl ::std::fmt::Display for CreateWalletRequestType {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Evm => f.write_str("EVM"),
                Self::Solana => f.write_str("SOLANA"),
                Self::Cosmos => f.write_str("COSMOS"),
            }
        }
    }
    impl ::std::str::FromStr for CreateWalletRequestType {
        type Err = self::error::ConversionError;
        fn from_str(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "EVM" => Ok(Self::Evm),
                "SOLANA" => Ok(Self::Solana),
                "COSMOS" => Ok(Self::Cosmos),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for CreateWalletRequestType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String> for CreateWalletRequestType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for CreateWalletRequestType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///How Para should validate / normalize the userIdentifier
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "How Para should validate / normalize the userIdentifier",
    ///  "type": "string",
    ///  "enum": [
    ///    "EMAIL",
    ///    "PHONE",
    ///    "CUSTOM_ID",
    ///    "GUEST_ID",
    ///    "TELEGRAM",
    ///    "DISCORD",
    ///    "TWITTER"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        ::serde::Deserialize,
        ::serde::Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd
    )]
    pub enum CreateWalletRequestUserIdentifierType {
        #[serde(rename = "EMAIL")]
        Email,
        #[serde(rename = "PHONE")]
        Phone,
        #[serde(rename = "CUSTOM_ID")]
        CustomId,
        #[serde(rename = "GUEST_ID")]
        GuestId,
        #[serde(rename = "TELEGRAM")]
        Telegram,
        #[serde(rename = "DISCORD")]
        Discord,
        #[serde(rename = "TWITTER")]
        Twitter,
    }
    impl ::std::fmt::Display for CreateWalletRequestUserIdentifierType {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Email => f.write_str("EMAIL"),
                Self::Phone => f.write_str("PHONE"),
                Self::CustomId => f.write_str("CUSTOM_ID"),
                Self::GuestId => f.write_str("GUEST_ID"),
                Self::Telegram => f.write_str("TELEGRAM"),
                Self::Discord => f.write_str("DISCORD"),
                Self::Twitter => f.write_str("TWITTER"),
            }
        }
    }
    impl ::std::str::FromStr for CreateWalletRequestUserIdentifierType {
        type Err = self::error::ConversionError;
        fn from_str(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "EMAIL" => Ok(Self::Email),
                "PHONE" => Ok(Self::Phone),
                "CUSTOM_ID" => Ok(Self::CustomId),
                "GUEST_ID" => Ok(Self::GuestId),
                "TELEGRAM" => Ok(Self::Telegram),
                "DISCORD" => Ok(Self::Discord),
                "TWITTER" => Ok(Self::Twitter),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for CreateWalletRequestUserIdentifierType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String>
    for CreateWalletRequestUserIdentifierType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String>
    for CreateWalletRequestUserIdentifierType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///Para error envelope. Fields vary; permissive on purpose.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Para error envelope. Fields vary; permissive on purpose.",
    ///  "type": "object",
    ///  "properties": {
    ///    "code": {
    ///      "type": "string"
    ///    },
    ///    "error": {
    ///      "type": "string"
    ///    },
    ///    "message": {
    ///      "type": "string"
    ///    }
    ///  },
    ///  "additionalProperties": true
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ErrorResponse {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub code: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub error: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub message: ::std::option::Option<::std::string::String>,
    }
    impl ::std::default::Default for ErrorResponse {
        fn default() -> Self {
            Self {
                code: Default::default(),
                error: Default::default(),
                message: Default::default(),
            }
        }
    }
    ///`SignRawRequest`
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
    ///      "description": "0x-prefixed hex bytes to sign",
    ///      "type": "string",
    ///      "pattern": "^0x[0-9a-fA-F]+$"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct SignRawRequest {
        ///0x-prefixed hex bytes to sign
        pub data: SignRawRequestData,
    }
    ///0x-prefixed hex bytes to sign
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "0x-prefixed hex bytes to sign",
    ///  "type": "string",
    ///  "pattern": "^0x[0-9a-fA-F]+$"
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct SignRawRequestData(::std::string::String);
    impl ::std::ops::Deref for SignRawRequestData {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }
    impl ::std::convert::From<SignRawRequestData> for ::std::string::String {
        fn from(value: SignRawRequestData) -> Self {
            value.0
        }
    }
    impl ::std::str::FromStr for SignRawRequestData {
        type Err = self::error::ConversionError;
        fn from_str(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            static PATTERN: ::std::sync::LazyLock<::regress::Regex> = ::std::sync::LazyLock::new(||
            { ::regress::Regex::new("^0x[0-9a-fA-F]+$").unwrap() });
            if PATTERN.find(value).is_none() {
                return Err("doesn't match pattern \"^0x[0-9a-fA-F]+$\"".into());
            }
            Ok(Self(value.to_string()))
        }
    }
    impl ::std::convert::TryFrom<&str> for SignRawRequestData {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String> for SignRawRequestData {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for SignRawRequestData {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl<'de> ::serde::Deserialize<'de> for SignRawRequestData {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }
    /**MPC signature payload. Exact shape depends on scheme; spec uses a
permissive object so the generated client surfaces the raw JSON.
*/
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "MPC signature payload. Exact shape depends on scheme; spec uses a\npermissive object so the generated client surfaces the raw JSON.\n",
    ///  "type": "object",
    ///  "properties": {
    ///    "sig": {
    ///      "description": "Alternate signature field name observed in some responses",
    ///      "type": "string"
    ///    },
    ///    "signature": {
    ///      "description": "Hex-encoded signature (when present)",
    ///      "type": "string"
    ///    }
    ///  },
    ///  "additionalProperties": true
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct SignRawResponse {
        ///Alternate signature field name observed in some responses
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub sig: ::std::option::Option<::std::string::String>,
        ///Hex-encoded signature (when present)
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub signature: ::std::option::Option<::std::string::String>,
    }
    impl ::std::default::Default for SignRawResponse {
        fn default() -> Self {
            Self {
                sig: Default::default(),
                signature: Default::default(),
            }
        }
    }
    ///Para wallet record. Additional fields may be returned.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Para wallet record. Additional fields may be returned.",
    ///  "type": "object",
    ///  "properties": {
    ///    "address": {
    ///      "description": "On-chain address (present once status = ready)",
    ///      "type": "string"
    ///    },
    ///    "cosmosPrefix": {
    ///      "type": "string"
    ///    },
    ///    "id": {
    ///      "description": "Para wallet ID (typically a UUID)",
    ///      "type": "string"
    ///    },
    ///    "publicKey": {
    ///      "description": "Hex-encoded public key (present once status = ready)",
    ///      "type": "string"
    ///    },
    ///    "scheme": {
    ///      "description": "MPC signing scheme used by this wallet",
    ///      "type": "string"
    ///    },
    ///    "status": {
    ///      "description": "Wallet lifecycle status. Observed values include `creating`,\n`ready`, and `error`; other values may exist.\n",
    ///      "type": "string"
    ///    },
    ///    "type": {
    ///      "description": "Wallet chain family",
    ///      "type": "string"
    ///    },
    ///    "userIdentifier": {
    ///      "type": "string"
    ///    },
    ///    "userIdentifierType": {
    ///      "type": "string"
    ///    }
    ///  },
    ///  "additionalProperties": true
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct Wallet {
        ///On-chain address (present once status = ready)
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub address: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "cosmosPrefix",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub cosmos_prefix: ::std::option::Option<::std::string::String>,
        ///Para wallet ID (typically a UUID)
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub id: ::std::option::Option<::std::string::String>,
        ///Hex-encoded public key (present once status = ready)
        #[serde(
            rename = "publicKey",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub public_key: ::std::option::Option<::std::string::String>,
        ///MPC signing scheme used by this wallet
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub scheme: ::std::option::Option<::std::string::String>,
        /**Wallet lifecycle status. Observed values include `creating`,
`ready`, and `error`; other values may exist.
*/
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub status: ::std::option::Option<::std::string::String>,
        ///Wallet chain family
        #[serde(
            rename = "type",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub type_: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "userIdentifier",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub user_identifier: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "userIdentifierType",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub user_identifier_type: ::std::option::Option<::std::string::String>,
    }
    impl ::std::default::Default for Wallet {
        fn default() -> Self {
            Self {
                address: Default::default(),
                cosmos_prefix: Default::default(),
                id: Default::default(),
                public_key: Default::default(),
                scheme: Default::default(),
                status: Default::default(),
                type_: Default::default(),
                user_identifier: Default::default(),
                user_identifier_type: Default::default(),
            }
        }
    }
}
#[derive(Clone, Debug)]
/**Client for Para Wallet REST API

Minimal OpenAPI surface for the Para MPC wallet REST API, covering only the
endpoints used by the Aomi `para` app: wallet create, wallet get, and raw signing.

## Auth
Para uses a static API key passed in the `X-API-Key` header. An optional
`X-Request-Id` (UUID) header is supported for idempotency / tracing.
The API key is provided at runtime via host context (state attribute
`para_api_key` / `PARA_API_KEY`); the spec only describes the header for
codegen purposes.


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
    /**Create a new MPC wallet

Create a Para MPC wallet for the given user identifier. Wallet creation
is asynchronous: the response typically has `status = "creating"` and
the wallet must be polled until `status = "ready"` before signing.


Sends a `POST` request to `/v1/wallets`

Arguments:
- `x_request_id`: Optional UUID for request tracing / idempotency
- `body`
*/
    pub async fn create_wallet<'a>(
        &'a self,
        x_request_id: Option<&'a ::uuid::Uuid>,
        body: &'a types::CreateWalletRequest,
    ) -> Result<ResponseValue<types::Wallet>, Error<()>> {
        let url = format!("{}/v1/wallets", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(2usize);
        header_map
            .append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(Self::api_version()),
            );
        if let Some(value) = x_request_id {
            header_map.append("X-Request-Id", value.to_string().try_into()?);
        }
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
            operation_id: "create_wallet",
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
    /**Fetch a single wallet by ID

Sends a `GET` request to `/v1/wallets/{walletId}`

Arguments:
- `wallet_id`: Para wallet identifier
- `x_request_id`: Optional UUID for request tracing / idempotency
*/
    pub async fn get_wallet<'a>(
        &'a self,
        wallet_id: &'a str,
        x_request_id: Option<&'a ::uuid::Uuid>,
    ) -> Result<ResponseValue<types::Wallet>, Error<()>> {
        let url = format!(
            "{}/v1/wallets/{}", self.baseurl, encode_path(& wallet_id.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(2usize);
        header_map
            .append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(Self::api_version()),
            );
        if let Some(value) = x_request_id {
            header_map.append("X-Request-Id", value.to_string().try_into()?);
        }
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
            operation_id: "get_wallet",
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
    /**MPC-sign raw 0x-prefixed hex data

Produce a distributed MPC signature over the supplied raw bytes.
The wallet must have `status = "ready"` before signing. Response
shape varies slightly by signing scheme; minimally includes a
`signature` (or `sig`) field.


Sends a `POST` request to `/v1/wallets/{walletId}/sign-raw`

Arguments:
- `wallet_id`: Para wallet identifier
- `x_request_id`: Optional UUID for request tracing / idempotency
- `body`
*/
    pub async fn sign_raw<'a>(
        &'a self,
        wallet_id: &'a str,
        x_request_id: Option<&'a ::uuid::Uuid>,
        body: &'a types::SignRawRequest,
    ) -> Result<ResponseValue<types::SignRawResponse>, Error<()>> {
        let url = format!(
            "{}/v1/wallets/{}/sign-raw", self.baseurl, encode_path(& wallet_id
            .to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(2usize);
        header_map
            .append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(Self::api_version()),
            );
        if let Some(value) = x_request_id {
            header_map.append("X-Request-Id", value.to_string().try_into()?);
        }
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
            operation_id: "sign_raw",
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
