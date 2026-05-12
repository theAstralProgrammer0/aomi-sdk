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
    ///`AmmMarketResponseDto`
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
    pub struct AmmMarketResponseDto(
        pub ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    );
    impl ::std::ops::Deref for AmmMarketResponseDto {
        type Target = ::serde_json::Map<::std::string::String, ::serde_json::Value>;
        fn deref(&self) -> &::serde_json::Map<::std::string::String, ::serde_json::Value> {
            &self.0
        }
    }
    impl ::std::convert::From<AmmMarketResponseDto>
        for ::serde_json::Map<::std::string::String, ::serde_json::Value>
    {
        fn from(value: AmmMarketResponseDto) -> Self {
            value.0
        }
    }
    impl ::std::convert::From<::serde_json::Map<::std::string::String, ::serde_json::Value>>
        for AmmMarketResponseDto
    {
        fn from(value: ::serde_json::Map<::std::string::String, ::serde_json::Value>) -> Self {
            Self(value)
        }
    }
    ///`AmmPositionDto`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "market",
    ///    "outcomeIndex"
    ///  ],
    ///  "properties": {
    ///    "account": {
    ///      "description": "Wallet address that was used to create this position",
    ///      "examples": [
    ///        "0x1234567890123456789012345678901234567890"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "collateralAmount": {
    ///      "description": "Amount of collateral invested in token decimals",
    ///      "examples": [
    ///        "100500000"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "latestTrade": {
    ///      "description": "Latest trade associated with this position",
    ///      "type": "object"
    ///    },
    ///    "market": {
    ///      "description": "Market information for this position",
    ///      "type": "object"
    ///    },
    ///    "outcomeIndex": {
    ///      "description": "Index of the outcome token",
    ///      "examples": [
    ///        0
    ///      ],
    ///      "type": "number"
    ///    },
    ///    "outcomeTokenAmount": {
    ///      "description": "Amount of outcome tokens held in token decimals",
    ///      "examples": [
    ///        "50250000"
    ///      ],
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct AmmPositionDto {
        ///Wallet address that was used to create this position
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub account: ::std::option::Option<::std::string::String>,
        ///Amount of collateral invested in token decimals
        #[serde(
            rename = "collateralAmount",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub collateral_amount: ::std::option::Option<::std::string::String>,
        ///Latest trade associated with this position
        #[serde(
            rename = "latestTrade",
            default,
            skip_serializing_if = "::serde_json::Map::is_empty"
        )]
        pub latest_trade: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        ///Market information for this position
        pub market: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        ///Index of the outcome token
        #[serde(rename = "outcomeIndex")]
        pub outcome_index: f64,
        ///Amount of outcome tokens held in token decimals
        #[serde(
            rename = "outcomeTokenAmount",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub outcome_token_amount: ::std::option::Option<::std::string::String>,
    }
    ///`ApiKeyControllerCreateApiKeyResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "apiKey": {
    ///      "description": "The API key value. Only shown once at creation time.",
    ///      "examples": [
    ///        "lmts_sk_1234567890abcdef..."
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "createdAt": {
    ///      "description": "When the key was created",
    ///      "examples": [
    ///        "2025-06-15T10:30:00Z"
    ///      ],
    ///      "type": "string",
    ///      "format": "date-time"
    ///    },
    ///    "keyId": {
    ///      "description": "Unique identifier for the API key",
    ///      "examples": [
    ///        "key_abc123def456"
    ///      ],
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ApiKeyControllerCreateApiKeyResponse {
        ///The API key value. Only shown once at creation time.
        #[serde(
            rename = "apiKey",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub api_key: ::std::option::Option<::std::string::String>,
        ///When the key was created
        #[serde(
            rename = "createdAt",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub created_at: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
        ///Unique identifier for the API key
        #[serde(
            rename = "keyId",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub key_id: ::std::option::Option<::std::string::String>,
    }
    impl ::std::default::Default for ApiKeyControllerCreateApiKeyResponse {
        fn default() -> Self {
            Self {
                api_key: Default::default(),
                created_at: Default::default(),
                key_id: Default::default(),
            }
        }
    }
    ///`ApiKeyControllerGetActiveKeyResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "createdAt": {
    ///      "description": "When the key was created",
    ///      "examples": [
    ///        "2025-06-15T10:30:00Z"
    ///      ],
    ///      "type": "string",
    ///      "format": "date-time"
    ///    },
    ///    "keyId": {
    ///      "description": "Unique identifier for the API key",
    ///      "examples": [
    ///        "key_abc123def456"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "lastUsedAt": {
    ///      "description": "When the key was last used",
    ///      "examples": [
    ///        "2025-06-16T08:00:00Z"
    ///      ],
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ],
    ///      "format": "date-time"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ApiKeyControllerGetActiveKeyResponse {
        ///When the key was created
        #[serde(
            rename = "createdAt",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub created_at: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
        ///Unique identifier for the API key
        #[serde(
            rename = "keyId",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub key_id: ::std::option::Option<::std::string::String>,
        ///When the key was last used
        #[serde(
            rename = "lastUsedAt",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub last_used_at: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
    }
    impl ::std::default::Default for ApiKeyControllerGetActiveKeyResponse {
        fn default() -> Self {
            Self {
                created_at: Default::default(),
                key_id: Default::default(),
                last_used_at: Default::default(),
            }
        }
    }
    ///`ApiKeyControllerRevokeKeyResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "message": {
    ///      "examples": [
    ///        "API key revoked successfully"
    ///      ],
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ApiKeyControllerRevokeKeyResponse {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub message: ::std::option::Option<::std::string::String>,
    }
    impl ::std::default::Default for ApiKeyControllerRevokeKeyResponse {
        fn default() -> Self {
            Self {
                message: Default::default(),
            }
        }
    }
    ///`ApiTokenControllerRevokeTokenResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "message": {
    ///      "examples": [
    ///        "API token revoked successfully"
    ///      ],
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ApiTokenControllerRevokeTokenResponse {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub message: ::std::option::Option<::std::string::String>,
    }
    impl ::std::default::Default for ApiTokenControllerRevokeTokenResponse {
        fn default() -> Self {
            Self {
                message: Default::default(),
            }
        }
    }
    ///`ApiTokenListItem`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "createdAt",
    ///    "label",
    ///    "lastUsedAt",
    ///    "scopes",
    ///    "tokenId"
    ///  ],
    ///  "properties": {
    ///    "createdAt": {
    ///      "description": "Token creation timestamp",
    ///      "type": "string",
    ///      "format": "date-time"
    ///    },
    ///    "label": {
    ///      "description": "Human-readable label",
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "lastUsedAt": {
    ///      "description": "Last time the token was used for authentication",
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ],
    ///      "format": "date-time"
    ///    },
    ///    "scopes": {
    ///      "description": "Granted scopes",
    ///      "type": "array",
    ///      "items": {
    ///        "type": "string"
    ///      }
    ///    },
    ///    "tokenId": {
    ///      "description": "Unique token identifier",
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ApiTokenListItem {
        ///Token creation timestamp
        #[serde(rename = "createdAt")]
        pub created_at: ::chrono::DateTime<::chrono::offset::Utc>,
        ///Human-readable label
        pub label: ::std::option::Option<::std::string::String>,
        ///Last time the token was used for authentication
        #[serde(rename = "lastUsedAt")]
        pub last_used_at: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
        ///Granted scopes
        pub scopes: ::std::vec::Vec<::std::string::String>,
        ///Unique token identifier
        #[serde(rename = "tokenId")]
        pub token_id: ::std::string::String,
    }
    ///`AuthControllerLogoutResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "message": {
    ///      "examples": [
    ///        "Logged out successfully"
    ///      ],
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct AuthControllerLogoutResponse {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub message: ::std::option::Option<::std::string::String>,
    }
    impl ::std::default::Default for AuthControllerLogoutResponse {
        fn default() -> Self {
            Self {
                message: Default::default(),
            }
        }
    }
    ///`BatchOrderStatusItemDto`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "clientOrderId": {
    ///      "description": "Client-provided order ID. Provide either orderId or clientOrderId, not both.",
    ///      "examples": [
    ///        "client-order-001"
    ///      ],
    ///      "type": "string",
    ///      "maxLength": 128
    ///    },
    ///    "orderId": {
    ///      "description": "Internal order ID. Provide either orderId or clientOrderId, not both.",
    ///      "examples": [
    ///        "4aa706dd-6c57-4f3c-945a-99818dfd95f1"
    ///      ],
    ///      "type": "string",
    ///      "format": "uuid"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct BatchOrderStatusItemDto {
        ///Client-provided order ID. Provide either orderId or clientOrderId, not both.
        #[serde(
            rename = "clientOrderId",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub client_order_id: ::std::option::Option<BatchOrderStatusItemDtoClientOrderId>,
        ///Internal order ID. Provide either orderId or clientOrderId, not both.
        #[serde(
            rename = "orderId",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub order_id: ::std::option::Option<::uuid::Uuid>,
    }
    impl ::std::default::Default for BatchOrderStatusItemDto {
        fn default() -> Self {
            Self {
                client_order_id: Default::default(),
                order_id: Default::default(),
            }
        }
    }
    ///Client-provided order ID. Provide either orderId or clientOrderId, not both.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Client-provided order ID. Provide either orderId or clientOrderId, not both.",
    ///  "examples": [
    ///    "client-order-001"
    ///  ],
    ///  "type": "string",
    ///  "maxLength": 128
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct BatchOrderStatusItemDtoClientOrderId(::std::string::String);
    impl ::std::ops::Deref for BatchOrderStatusItemDtoClientOrderId {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }
    impl ::std::convert::From<BatchOrderStatusItemDtoClientOrderId> for ::std::string::String {
        fn from(value: BatchOrderStatusItemDtoClientOrderId) -> Self {
            value.0
        }
    }
    impl ::std::str::FromStr for BatchOrderStatusItemDtoClientOrderId {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            if value.chars().count() > 128usize {
                return Err("longer than 128 characters".into());
            }
            Ok(Self(value.to_string()))
        }
    }
    impl ::std::convert::TryFrom<&str> for BatchOrderStatusItemDtoClientOrderId {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String> for BatchOrderStatusItemDtoClientOrderId {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for BatchOrderStatusItemDtoClientOrderId {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl<'de> ::serde::Deserialize<'de> for BatchOrderStatusItemDtoClientOrderId {
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
    ///`BatchOrderStatusRequestDto`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "items"
    ///  ],
    ///  "properties": {
    ///    "items": {
    ///      "description": "List of status lookup queries (1-50 items)",
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/BatchOrderStatusItemDto"
    ///      },
    ///      "maxItems": 50,
    ///      "minItems": 1
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct BatchOrderStatusRequestDto {
        ///List of status lookup queries (1-50 items)
        pub items: ::std::vec::Vec<BatchOrderStatusItemDto>,
    }
    ///`BatchOrderStatusResponseDto`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "results"
    ///  ],
    ///  "properties": {
    ///    "results": {
    ///      "description": "Array of status results corresponding to request items",
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/BatchOrderStatusResultDto"
    ///      }
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct BatchOrderStatusResponseDto {
        ///Array of status results corresponding to request items
        pub results: ::std::vec::Vec<BatchOrderStatusResultDto>,
    }
    ///`BatchOrderStatusResultDto`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "index",
    ///    "status"
    ///  ],
    ///  "properties": {
    ///    "clientOrderId": {
    ///      "description": "Client-provided order ID",
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "data": {
    ///      "description": "Order data if found",
    ///      "type": [
    ///        "object",
    ///        "null"
    ///      ],
    ///      "properties": {
    ///        "execution": {
    ///          "$ref": "#/components/schemas/OrderExecutionSummary"
    ///        },
    ///        "makerMatches": {
    ///          "description": "Maker orders matched against this order",
    ///          "type": "array",
    ///          "items": {
    ///            "$ref": "#/components/schemas/MakerMatch"
    ///          }
    ///        },
    ///        "order": {
    ///          "$ref": "#/components/schemas/OrderResponseDto"
    ///        }
    ///      }
    ///    },
    ///    "error": {
    ///      "description": "Error message if status is 'invalid'",
    ///      "examples": [
    ///        "Exactly one of orderId or clientOrderId is required"
    ///      ],
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "index": {
    ///      "description": "Index of this item in the request array",
    ///      "examples": [
    ///        0
    ///      ],
    ///      "type": "number"
    ///    },
    ///    "orderId": {
    ///      "description": "Resolved internal order ID",
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "status": {
    ///      "description": "Lookup result status",
    ///      "examples": [
    ///        "found"
    ///      ],
    ///      "type": "string",
    ///      "enum": [
    ///        "found",
    ///        "not_found",
    ///        "invalid"
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct BatchOrderStatusResultDto {
        ///Client-provided order ID
        #[serde(
            rename = "clientOrderId",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub client_order_id: ::std::option::Option<::std::string::String>,
        ///Order data if found
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub data: ::std::option::Option<BatchOrderStatusResultDtoData>,
        ///Error message if status is 'invalid'
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub error: ::std::option::Option<::std::string::String>,
        ///Index of this item in the request array
        pub index: f64,
        ///Resolved internal order ID
        #[serde(
            rename = "orderId",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub order_id: ::std::option::Option<::std::string::String>,
        ///Lookup result status
        pub status: BatchOrderStatusResultDtoStatus,
    }
    ///Order data if found
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Order data if found",
    ///  "type": "object",
    ///  "properties": {
    ///    "execution": {
    ///      "$ref": "#/components/schemas/OrderExecutionSummary"
    ///    },
    ///    "makerMatches": {
    ///      "description": "Maker orders matched against this order",
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/MakerMatch"
    ///      }
    ///    },
    ///    "order": {
    ///      "$ref": "#/components/schemas/OrderResponseDto"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct BatchOrderStatusResultDtoData {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub execution: ::std::option::Option<OrderExecutionSummary>,
        ///Maker orders matched against this order
        #[serde(
            rename = "makerMatches",
            default,
            skip_serializing_if = "::std::vec::Vec::is_empty"
        )]
        pub maker_matches: ::std::vec::Vec<MakerMatch>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub order: ::std::option::Option<OrderResponseDto>,
    }
    impl ::std::default::Default for BatchOrderStatusResultDtoData {
        fn default() -> Self {
            Self {
                execution: Default::default(),
                maker_matches: Default::default(),
                order: Default::default(),
            }
        }
    }
    ///Lookup result status
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Lookup result status",
    ///  "examples": [
    ///    "found"
    ///  ],
    ///  "type": "string",
    ///  "enum": [
    ///    "found",
    ///    "not_found",
    ///    "invalid"
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
        PartialOrd,
    )]
    pub enum BatchOrderStatusResultDtoStatus {
        #[serde(rename = "found")]
        Found,
        #[serde(rename = "not_found")]
        NotFound,
        #[serde(rename = "invalid")]
        Invalid,
    }
    impl ::std::fmt::Display for BatchOrderStatusResultDtoStatus {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Found => f.write_str("found"),
                Self::NotFound => f.write_str("not_found"),
                Self::Invalid => f.write_str("invalid"),
            }
        }
    }
    impl ::std::str::FromStr for BatchOrderStatusResultDtoStatus {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "found" => Ok(Self::Found),
                "not_found" => Ok(Self::NotFound),
                "invalid" => Ok(Self::Invalid),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for BatchOrderStatusResultDtoStatus {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String> for BatchOrderStatusResultDtoStatus {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for BatchOrderStatusResultDtoStatus {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///`BreadcrumbItemDto`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "name",
    ///    "path",
    ///    "slug"
    ///  ],
    ///  "properties": {
    ///    "name": {
    ///      "description": "Display name of the breadcrumb item",
    ///      "type": "string"
    ///    },
    ///    "path": {
    ///      "description": "Full path to this breadcrumb item",
    ///      "type": "string"
    ///    },
    ///    "slug": {
    ///      "description": "URL slug of the breadcrumb item",
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct BreadcrumbItemDto {
        ///Display name of the breadcrumb item
        pub name: ::std::string::String,
        ///Full path to this breadcrumb item
        pub path: ::std::string::String,
        ///URL slug of the breadcrumb item
        pub slug: ::std::string::String,
    }
    ///`BrowseActiveMarketsResponseDto`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "data",
    ///    "totalMarketsCount"
    ///  ],
    ///  "properties": {
    ///    "data": {
    ///      "description": "Array of active markets with complete trading data including volume, liquidity, and recent feed events",
    ///      "examples": [
    ///        [
    ///          {
    ///            "address": "0x76d3e2098Be66Aa7E15138F467390f0Eb7349B9b",
    ///            "categories": [
    ///              "Hourly"
    ///            ],
    ///            "collateralToken": {
    ///              "address": "0x833589fCD6eDb6E08f4c7C32D4f71b54bdA02913",
    ///              "decimals": 6,
    ///              "symbol": "USDC"
    ///            },
    ///            "conditionId": "0x812f578437dc536def1412e4e593ef310884262c22868b30c1e58582e5f3e9bf",
    ///            "creator": {
    ///              "imageURI": "https://limitless.exchange/assets/images/logo.svg",
    ///              "link": "https://x.com/trylimitless",
    ///              "name": "Limitless"
    ///            },
    ///            "description": "This market will resolve to \"YES\" if the price of $DOGE is above $0.21652...",
    ///            "expirationDate": "Sep 1, 2025",
    ///            "expirationTimestamp": 1756728000000,
    ///            "expired": false,
    ///            "feedEvents": [
    ///              {
    ///                "data": {
    ///                  "address": "0x76d3e2098Be66Aa7E15138F467390f0Eb7349B9b",
    ///                  "contracts": "9.071313",
    ///                  "marketId": 7495,
    ///                  "outcome": "NO",
    ///                  "strategy": "Buy",
    ///                  "title": "$DOGE above $0.21652 on Sep 1, 12:00 UTC?",
    ///                  "tradeAmount": "5",
    ///                  "tradeAmountUSD": "4.999525"
    ///                },
    ///                "eventType": "NEW_TRADE",
    ///                "timestamp": "2025-09-01T11:30:31.000Z",
    ///                "user": {
    ///                  "account": "0xea27f6788F083e6070961d3E52A2e596367E04CC",
    ///                  "id": 7080,
    ///                  "name": "GG",
    ///                  "points": "0.00000000",
    ///                  "rankName": "Bronze"
    ///                }
    ///              }
    ///            ],
    ///            "id": 7495,
    ///            "liquidity": "50000000",
    ///            "liquidityFormatted": "50.000000",
    ///            "marketType": "single",
    ///            "openInterest": "48310707",
    ///            "openInterestFormatted": "48.310707",
    ///            "prices": [
    ///              42.8,
    ///              57.2
    ///            ],
    ///            "slug": "dollardoge-above-dollar021652-on-sep-1-1200-utc-1756724413009",
    ///            "status": "FUNDED",
    ///            "tags": [
    ///              "Lumy",
    ///              "Recurring",
    ///              "Hourly",
    ///              "Simple Mode"
    ///            ],
    ///            "title": "$DOGE above $0.21652 on Sep 1, 12:00 UTC?",
    ///            "tradeType": "amm",
    ///            "volume": "164109293",
    ///            "volumeFormatted": "164.109293"
    ///          }
    ///        ]
    ///      ],
    ///      "type": "array",
    ///      "items": {
    ///        "description": "Market object with complete trading data (FEMarket type)",
    ///        "type": "object",
    ///        "additionalProperties": true
    ///      }
    ///    },
    ///    "totalMarketsCount": {
    ///      "description": "Total number of active markets available for pagination",
    ///      "examples": [
    ///        150
    ///      ],
    ///      "type": "number"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct BrowseActiveMarketsResponseDto {
        ///Array of active markets with complete trading data including volume, liquidity, and recent feed events
        pub data: ::std::vec::Vec<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
        ///Total number of active markets available for pagination
        #[serde(rename = "totalMarketsCount")]
        pub total_markets_count: f64,
    }
    ///`CancelAllOrdersResponseDto`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "message"
    ///  ],
    ///  "properties": {
    ///    "canceled": {
    ///      "description": "Array of successfully cancelled order IDs",
    ///      "examples": [
    ///        [
    ///          "611badac-8dfc-48a0-b09e-59654adea1c5"
    ///        ]
    ///      ],
    ///      "type": "array",
    ///      "items": {
    ///        "type": "string"
    ///      }
    ///    },
    ///    "failed": {
    ///      "description": "Array of orders that failed to cancel with reasons",
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/CancelOrderFailure"
    ///      }
    ///    },
    ///    "message": {
    ///      "description": "Confirmation message for cancelling all orders",
    ///      "examples": [
    ///        "Orders canceled successfully"
    ///      ],
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct CancelAllOrdersResponseDto {
        ///Array of successfully cancelled order IDs
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub canceled: ::std::vec::Vec<::std::string::String>,
        ///Array of orders that failed to cancel with reasons
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub failed: ::std::vec::Vec<CancelOrderFailure>,
        ///Confirmation message for cancelling all orders
        pub message: ::std::string::String,
    }
    ///`CancelOrderBatchCombinedDto`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "oneOf": [
    ///    {
    ///      "required": [
    ///        "orderIds"
    ///      ]
    ///    },
    ///    {
    ///      "required": [
    ///        "clientOrderIds"
    ///      ]
    ///    }
    ///  ],
    ///  "properties": {
    ///    "clientOrderIds": {
    ///      "description": "Client-provided order IDs from order creation. Provide exactly one of orderIds or clientOrderIds.",
    ///      "examples": [
    ///        [
    ///          "partner-order-001",
    ///          "partner-order-002"
    ///        ]
    ///      ],
    ///      "type": "array",
    ///      "items": {
    ///        "type": "string",
    ///        "maxLength": 128
    ///      },
    ///      "maxItems": 50
    ///    },
    ///    "orderIds": {
    ///      "description": "Internal order IDs. Provide exactly one of orderIds or clientOrderIds.",
    ///      "examples": [
    ///        [
    ///          "6f52b6d2-6c9e-4a5c-8a4f-28ab4b7ff203"
    ///        ]
    ///      ],
    ///      "type": "array",
    ///      "items": {
    ///        "type": "string",
    ///        "format": "uuid"
    ///      },
    ///      "maxItems": 50
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    #[serde(untagged)]
    pub enum CancelOrderBatchCombinedDto {
        Variant0 {
            ///Internal order IDs. Provide exactly one of orderIds or clientOrderIds.
            #[serde(rename = "orderIds")]
            order_ids: ::std::vec::Vec<::uuid::Uuid>,
        },
        Variant1 {
            ///Client-provided order IDs from order creation. Provide exactly one of orderIds or clientOrderIds.
            #[serde(rename = "clientOrderIds")]
            client_order_ids:
                ::std::vec::Vec<CancelOrderBatchCombinedDtoVariant1ClientOrderIdsItem>,
        },
    }
    ///`CancelOrderBatchCombinedDtoVariant1ClientOrderIdsItem`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "maxLength": 128
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct CancelOrderBatchCombinedDtoVariant1ClientOrderIdsItem(::std::string::String);
    impl ::std::ops::Deref for CancelOrderBatchCombinedDtoVariant1ClientOrderIdsItem {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }
    impl ::std::convert::From<CancelOrderBatchCombinedDtoVariant1ClientOrderIdsItem>
        for ::std::string::String
    {
        fn from(value: CancelOrderBatchCombinedDtoVariant1ClientOrderIdsItem) -> Self {
            value.0
        }
    }
    impl ::std::str::FromStr for CancelOrderBatchCombinedDtoVariant1ClientOrderIdsItem {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            if value.chars().count() > 128usize {
                return Err("longer than 128 characters".into());
            }
            Ok(Self(value.to_string()))
        }
    }
    impl ::std::convert::TryFrom<&str> for CancelOrderBatchCombinedDtoVariant1ClientOrderIdsItem {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String>
        for CancelOrderBatchCombinedDtoVariant1ClientOrderIdsItem
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String>
        for CancelOrderBatchCombinedDtoVariant1ClientOrderIdsItem
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl<'de> ::serde::Deserialize<'de> for CancelOrderBatchCombinedDtoVariant1ClientOrderIdsItem {
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
    ///`CancelOrderBatchCombinedResponseDto`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "message"
    ///  ],
    ///  "properties": {
    ///    "canceled": {
    ///      "description": "Canceled client-order-id records",
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/ClientOrderCancelSuccess"
    ///      }
    ///    },
    ///    "failed": {
    ///      "description": "Client-order-id cancel failures",
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/ClientOrderCancelFailure"
    ///      }
    ///    },
    ///    "message": {
    ///      "examples": [
    ///        "Orders canceled successfully"
    ///      ],
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct CancelOrderBatchCombinedResponseDto {
        ///Canceled client-order-id records
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub canceled: ::std::vec::Vec<ClientOrderCancelSuccess>,
        ///Client-order-id cancel failures
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub failed: ::std::vec::Vec<ClientOrderCancelFailure>,
        pub message: ::std::string::String,
    }
    ///`CancelOrderBatchResponseDto`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "message"
    ///  ],
    ///  "properties": {
    ///    "canceled": {
    ///      "description": "Array of successfully cancelled order IDs",
    ///      "examples": [
    ///        [
    ///          "611badac-8dfc-48a0-b09e-59654adea1c5"
    ///        ]
    ///      ],
    ///      "type": "array",
    ///      "items": {
    ///        "type": "string"
    ///      }
    ///    },
    ///    "failed": {
    ///      "description": "Array of orders that failed to cancel with reasons",
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/CancelOrderFailure"
    ///      }
    ///    },
    ///    "message": {
    ///      "description": "Confirmation message for the cancelled orders",
    ///      "examples": [
    ///        "Orders canceled successfully"
    ///      ],
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct CancelOrderBatchResponseDto {
        ///Array of successfully cancelled order IDs
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub canceled: ::std::vec::Vec<::std::string::String>,
        ///Array of orders that failed to cancel with reasons
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub failed: ::std::vec::Vec<CancelOrderFailure>,
        ///Confirmation message for the cancelled orders
        pub message: ::std::string::String,
    }
    ///`CancelOrderCombinedDto`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "oneOf": [
    ///    {
    ///      "required": [
    ///        "orderId"
    ///      ]
    ///    },
    ///    {
    ///      "required": [
    ///        "clientOrderId"
    ///      ]
    ///    }
    ///  ],
    ///  "properties": {
    ///    "clientOrderId": {
    ///      "description": "Client-provided order ID from order creation. Provide exactly one of orderId or clientOrderId.",
    ///      "examples": [
    ///        "partner-order-001"
    ///      ],
    ///      "type": "string",
    ///      "maxLength": 128
    ///    },
    ///    "orderId": {
    ///      "description": "Internal order ID. Provide exactly one of orderId or clientOrderId.",
    ///      "examples": [
    ///        "6f52b6d2-6c9e-4a5c-8a4f-28ab4b7ff203"
    ///      ],
    ///      "type": "string",
    ///      "format": "uuid"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    #[serde(untagged)]
    pub enum CancelOrderCombinedDto {
        Variant0 {
            ///Internal order ID. Provide exactly one of orderId or clientOrderId.
            #[serde(rename = "orderId")]
            order_id: ::uuid::Uuid,
        },
        Variant1 {
            ///Client-provided order ID from order creation. Provide exactly one of orderId or clientOrderId.
            #[serde(rename = "clientOrderId")]
            client_order_id: CancelOrderCombinedDtoVariant1ClientOrderId,
        },
    }
    ///Client-provided order ID from order creation. Provide exactly one of orderId or clientOrderId.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Client-provided order ID from order creation. Provide exactly one of orderId or clientOrderId.",
    ///  "examples": [
    ///    "partner-order-001"
    ///  ],
    ///  "type": "string",
    ///  "maxLength": 128
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct CancelOrderCombinedDtoVariant1ClientOrderId(::std::string::String);
    impl ::std::ops::Deref for CancelOrderCombinedDtoVariant1ClientOrderId {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }
    impl ::std::convert::From<CancelOrderCombinedDtoVariant1ClientOrderId> for ::std::string::String {
        fn from(value: CancelOrderCombinedDtoVariant1ClientOrderId) -> Self {
            value.0
        }
    }
    impl ::std::str::FromStr for CancelOrderCombinedDtoVariant1ClientOrderId {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            if value.chars().count() > 128usize {
                return Err("longer than 128 characters".into());
            }
            Ok(Self(value.to_string()))
        }
    }
    impl ::std::convert::TryFrom<&str> for CancelOrderCombinedDtoVariant1ClientOrderId {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String>
        for CancelOrderCombinedDtoVariant1ClientOrderId
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String>
        for CancelOrderCombinedDtoVariant1ClientOrderId
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl<'de> ::serde::Deserialize<'de> for CancelOrderCombinedDtoVariant1ClientOrderId {
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
    ///`CancelOrderFailure`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "message",
    ///    "orderId",
    ///    "reason"
    ///  ],
    ///  "properties": {
    ///    "message": {
    ///      "description": "User-friendly error message",
    ///      "examples": [
    ///        "Order not found or already canceled"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "orderId": {
    ///      "description": "Order ID that failed to cancel",
    ///      "examples": [
    ///        "b53f0e4b-1529-45cc-ad39-e27f4c6eab5a"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "reason": {
    ///      "description": "Error code indicating the reason for failure",
    ///      "examples": [
    ///        "ORDER_NOT_FOUND"
    ///      ],
    ///      "type": "string",
    ///      "enum": [
    ///        "ORDER_NOT_FOUND",
    ///        "UNKNOWN_ERROR"
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct CancelOrderFailure {
        ///User-friendly error message
        pub message: ::std::string::String,
        ///Order ID that failed to cancel
        #[serde(rename = "orderId")]
        pub order_id: ::std::string::String,
        ///Error code indicating the reason for failure
        pub reason: CancelOrderFailureReason,
    }
    ///Error code indicating the reason for failure
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Error code indicating the reason for failure",
    ///  "examples": [
    ///    "ORDER_NOT_FOUND"
    ///  ],
    ///  "type": "string",
    ///  "enum": [
    ///    "ORDER_NOT_FOUND",
    ///    "UNKNOWN_ERROR"
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
        PartialOrd,
    )]
    pub enum CancelOrderFailureReason {
        #[serde(rename = "ORDER_NOT_FOUND")]
        OrderNotFound,
        #[serde(rename = "UNKNOWN_ERROR")]
        UnknownError,
    }
    impl ::std::fmt::Display for CancelOrderFailureReason {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::OrderNotFound => f.write_str("ORDER_NOT_FOUND"),
                Self::UnknownError => f.write_str("UNKNOWN_ERROR"),
            }
        }
    }
    impl ::std::str::FromStr for CancelOrderFailureReason {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "ORDER_NOT_FOUND" => Ok(Self::OrderNotFound),
                "UNKNOWN_ERROR" => Ok(Self::UnknownError),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for CancelOrderFailureReason {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String> for CancelOrderFailureReason {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for CancelOrderFailureReason {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///`CancelOrderResponseDto`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "message"
    ///  ],
    ///  "properties": {
    ///    "message": {
    ///      "description": "Confirmation message for the cancelled order",
    ///      "examples": [
    ///        "Order canceled successfully"
    ///      ],
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct CancelOrderResponseDto {
        ///Confirmation message for the cancelled order
        pub message: ::std::string::String,
    }
    ///`CategoryCountResponseDto`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "category",
    ///    "totalCount"
    ///  ],
    ///  "properties": {
    ///    "category": {
    ///      "description": "Number of active markets per category",
    ///      "examples": [
    ///        {
    ///          "1": 10,
    ///          "2": 5,
    ///          "3": 8
    ///        }
    ///      ],
    ///      "type": "object",
    ///      "additionalProperties": {
    ///        "type": "number"
    ///      }
    ///    },
    ///    "totalCount": {
    ///      "description": "Total number of active markets",
    ///      "examples": [
    ///        23
    ///      ],
    ///      "type": "number"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct CategoryCountResponseDto {
        ///Number of active markets per category
        pub category: ::std::collections::HashMap<::std::string::String, f64>,
        ///Total number of active markets
        #[serde(rename = "totalCount")]
        pub total_count: f64,
    }
    ///`ClientOrderCancelFailure`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "clientOrderId",
    ///    "message",
    ///    "reason"
    ///  ],
    ///  "properties": {
    ///    "clientOrderId": {
    ///      "description": "Client-provided order ID from order creation",
    ///      "examples": [
    ///        "partner-order-002"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "message": {
    ///      "description": "Human-readable failure message",
    ///      "examples": [
    ///        "Order not found"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "orderId": {
    ///      "description": "Internal order ID, when resolution succeeded before cancellation failed",
    ///      "examples": [
    ///        "6f52b6d2-6c9e-4a5c-8a4f-28ab4b7ff203"
    ///      ],
    ///      "type": "string",
    ///      "format": "uuid"
    ///    },
    ///    "reason": {
    ///      "description": "Failure reason",
    ///      "examples": [
    ///        "ORDER_NOT_FOUND"
    ///      ],
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ClientOrderCancelFailure {
        ///Client-provided order ID from order creation
        #[serde(rename = "clientOrderId")]
        pub client_order_id: ::std::string::String,
        ///Human-readable failure message
        pub message: ::std::string::String,
        ///Internal order ID, when resolution succeeded before cancellation failed
        #[serde(
            rename = "orderId",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub order_id: ::std::option::Option<::uuid::Uuid>,
        ///Failure reason
        pub reason: ::std::string::String,
    }
    ///`ClientOrderCancelSuccess`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "clientOrderId",
    ///    "orderId"
    ///  ],
    ///  "properties": {
    ///    "clientOrderId": {
    ///      "description": "Client-provided order ID from order creation",
    ///      "examples": [
    ///        "partner-order-001"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "orderId": {
    ///      "description": "Internal order ID",
    ///      "examples": [
    ///        "6f52b6d2-6c9e-4a5c-8a4f-28ab4b7ff203"
    ///      ],
    ///      "type": "string",
    ///      "format": "uuid"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ClientOrderCancelSuccess {
        ///Client-provided order ID from order creation
        #[serde(rename = "clientOrderId")]
        pub client_order_id: ::std::string::String,
        ///Internal order ID
        #[serde(rename = "orderId")]
        pub order_id: ::uuid::Uuid,
    }
    ///`ClobMarketResponseDto`
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
    pub struct ClobMarketResponseDto(
        pub ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    );
    impl ::std::ops::Deref for ClobMarketResponseDto {
        type Target = ::serde_json::Map<::std::string::String, ::serde_json::Value>;
        fn deref(&self) -> &::serde_json::Map<::std::string::String, ::serde_json::Value> {
            &self.0
        }
    }
    impl ::std::convert::From<ClobMarketResponseDto>
        for ::serde_json::Map<::std::string::String, ::serde_json::Value>
    {
        fn from(value: ClobMarketResponseDto) -> Self {
            value.0
        }
    }
    impl ::std::convert::From<::serde_json::Map<::std::string::String, ::serde_json::Value>>
        for ClobMarketResponseDto
    {
        fn from(value: ::serde_json::Map<::std::string::String, ::serde_json::Value>) -> Self {
            Self(value)
        }
    }
    ///`ClobPositionDto`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "market",
    ///    "orders",
    ///    "positions",
    ///    "tokensBalance"
    ///  ],
    ///  "properties": {
    ///    "latestTrade": {
    ///      "description": "Latest trade prices",
    ///      "type": "object"
    ///    },
    ///    "market": {
    ///      "description": "Market information",
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/Market"
    ///        }
    ///      ]
    ///    },
    ///    "orders": {
    ///      "description": "Order information",
    ///      "type": "object"
    ///    },
    ///    "positions": {
    ///      "description": "Position data for the market",
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/MarketPositionDataDto"
    ///        }
    ///      ]
    ///    },
    ///    "rewards": {
    ///      "description": "Rewards information for this position",
    ///      "type": "object"
    ///    },
    ///    "tokensBalance": {
    ///      "description": "Token balances in token decimals",
    ///      "type": "object"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ClobPositionDto {
        ///Latest trade prices
        #[serde(
            rename = "latestTrade",
            default,
            skip_serializing_if = "::serde_json::Map::is_empty"
        )]
        pub latest_trade: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        ///Market information
        pub market: Market,
        ///Order information
        pub orders: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        ///Position data for the market
        pub positions: MarketPositionDataDto,
        ///Rewards information for this position
        #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
        pub rewards: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        ///Token balances in token decimals
        #[serde(rename = "tokensBalance")]
        pub tokens_balance: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    }
    ///`CreateOrderDto`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "marketSlug",
    ///    "order",
    ///    "orderType",
    ///    "ownerId"
    ///  ],
    ///  "properties": {
    ///    "clientOrderId": {
    ///      "description": "Client-provided idempotency key for order placement. If a duplicate is submitted, the server returns 409 Conflict.",
    ///      "examples": [
    ///        "client-order-001"
    ///      ],
    ///      "type": "string",
    ///      "maxLength": 128
    ///    },
    ///    "marketSlug": {
    ///      "description": "Market identifier slug",
    ///      "examples": [
    ///        "biden-vs-trump-2024"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "onBehalfOf": {
    ///      "description": "Profile ID to place order on behalf of (partner flow). Requires an API token with trading scope and a partner relationship with the target profile.",
    ///      "examples": [
    ///        12345
    ///      ],
    ///      "type": "number"
    ///    },
    ///    "order": {
    ///      "description": "Order details including signature and amounts",
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/Order"
    ///        }
    ///      ]
    ///    },
    ///    "orderType": {
    ///      "description": "Order type (GTC=Good Till Cancelled, FOK=Fill Or Kill)",
    ///      "examples": [
    ///        "GTC"
    ///      ],
    ///      "type": "string",
    ///      "enum": [
    ///        "FOK",
    ///        "GTC"
    ///      ]
    ///    },
    ///    "ownerId": {
    ///      "description": "Profile ID of the order owner",
    ///      "examples": [
    ///        12345
    ///      ],
    ///      "type": "number"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct CreateOrderDto {
        ///Client-provided idempotency key for order placement. If a duplicate is submitted, the server returns 409 Conflict.
        #[serde(
            rename = "clientOrderId",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub client_order_id: ::std::option::Option<CreateOrderDtoClientOrderId>,
        ///Market identifier slug
        #[serde(rename = "marketSlug")]
        pub market_slug: ::std::string::String,
        ///Profile ID to place order on behalf of (partner flow). Requires an API token with trading scope and a partner relationship with the target profile.
        #[serde(
            rename = "onBehalfOf",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub on_behalf_of: ::std::option::Option<f64>,
        ///Order details including signature and amounts
        pub order: Order,
        ///Order type (GTC=Good Till Cancelled, FOK=Fill Or Kill)
        #[serde(rename = "orderType")]
        pub order_type: CreateOrderDtoOrderType,
        ///Profile ID of the order owner
        #[serde(rename = "ownerId")]
        pub owner_id: f64,
    }
    ///Client-provided idempotency key for order placement. If a duplicate is submitted, the server returns 409 Conflict.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Client-provided idempotency key for order placement. If a duplicate is submitted, the server returns 409 Conflict.",
    ///  "examples": [
    ///    "client-order-001"
    ///  ],
    ///  "type": "string",
    ///  "maxLength": 128
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct CreateOrderDtoClientOrderId(::std::string::String);
    impl ::std::ops::Deref for CreateOrderDtoClientOrderId {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }
    impl ::std::convert::From<CreateOrderDtoClientOrderId> for ::std::string::String {
        fn from(value: CreateOrderDtoClientOrderId) -> Self {
            value.0
        }
    }
    impl ::std::str::FromStr for CreateOrderDtoClientOrderId {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            if value.chars().count() > 128usize {
                return Err("longer than 128 characters".into());
            }
            Ok(Self(value.to_string()))
        }
    }
    impl ::std::convert::TryFrom<&str> for CreateOrderDtoClientOrderId {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String> for CreateOrderDtoClientOrderId {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for CreateOrderDtoClientOrderId {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl<'de> ::serde::Deserialize<'de> for CreateOrderDtoClientOrderId {
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
    ///Order type (GTC=Good Till Cancelled, FOK=Fill Or Kill)
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Order type (GTC=Good Till Cancelled, FOK=Fill Or Kill)",
    ///  "examples": [
    ///    "GTC"
    ///  ],
    ///  "type": "string",
    ///  "enum": [
    ///    "FOK",
    ///    "GTC"
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
        PartialOrd,
    )]
    pub enum CreateOrderDtoOrderType {
        #[serde(rename = "FOK")]
        Fok,
        #[serde(rename = "GTC")]
        Gtc,
    }
    impl ::std::fmt::Display for CreateOrderDtoOrderType {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Fok => f.write_str("FOK"),
                Self::Gtc => f.write_str("GTC"),
            }
        }
    }
    impl ::std::str::FromStr for CreateOrderDtoOrderType {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "FOK" => Ok(Self::Fok),
                "GTC" => Ok(Self::Gtc),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for CreateOrderDtoOrderType {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String> for CreateOrderDtoOrderType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for CreateOrderDtoOrderType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///`CreatePartnerAccountRequest`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "createServerWallet": {
    ///      "description": "If true, creates a Privy server wallet for the sub-account (enables delegated signing). If false or omitted, requires EOA wallet ownership headers.",
    ///      "default": false,
    ///      "type": "boolean"
    ///    },
    ///    "displayName": {
    ///      "description": "Public display name for the sub-account. Defaults to the wallet address if omitted.",
    ///      "examples": [
    ///        "user-alice"
    ///      ],
    ///      "type": "string",
    ///      "maxLength": 44
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct CreatePartnerAccountRequest {
        ///If true, creates a Privy server wallet for the sub-account (enables delegated signing). If false or omitted, requires EOA wallet ownership headers.
        #[serde(rename = "createServerWallet", default)]
        pub create_server_wallet: bool,
        ///Public display name for the sub-account. Defaults to the wallet address if omitted.
        #[serde(
            rename = "displayName",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub display_name: ::std::option::Option<CreatePartnerAccountRequestDisplayName>,
    }
    impl ::std::default::Default for CreatePartnerAccountRequest {
        fn default() -> Self {
            Self {
                create_server_wallet: Default::default(),
                display_name: Default::default(),
            }
        }
    }
    ///Public display name for the sub-account. Defaults to the wallet address if omitted.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Public display name for the sub-account. Defaults to the wallet address if omitted.",
    ///  "examples": [
    ///    "user-alice"
    ///  ],
    ///  "type": "string",
    ///  "maxLength": 44
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct CreatePartnerAccountRequestDisplayName(::std::string::String);
    impl ::std::ops::Deref for CreatePartnerAccountRequestDisplayName {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }
    impl ::std::convert::From<CreatePartnerAccountRequestDisplayName> for ::std::string::String {
        fn from(value: CreatePartnerAccountRequestDisplayName) -> Self {
            value.0
        }
    }
    impl ::std::str::FromStr for CreatePartnerAccountRequestDisplayName {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            if value.chars().count() > 44usize {
                return Err("longer than 44 characters".into());
            }
            Ok(Self(value.to_string()))
        }
    }
    impl ::std::convert::TryFrom<&str> for CreatePartnerAccountRequestDisplayName {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String> for CreatePartnerAccountRequestDisplayName {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for CreatePartnerAccountRequestDisplayName {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl<'de> ::serde::Deserialize<'de> for CreatePartnerAccountRequestDisplayName {
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
    ///`CreatePartnerAccountResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "account",
    ///    "profileId"
    ///  ],
    ///  "properties": {
    ///    "account": {
    ///      "description": "Wallet address of the created sub-account",
    ///      "examples": [
    ///        "0x5aAeb6053F3E94C9b9A09f33669435E7Ef1BeAed"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "profileId": {
    ///      "description": "Profile ID of the created sub-account",
    ///      "examples": [
    ///        789
    ///      ],
    ///      "type": "integer"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct CreatePartnerAccountResponse {
        ///Wallet address of the created sub-account
        pub account: ::std::string::String,
        ///Profile ID of the created sub-account
        #[serde(rename = "profileId")]
        pub profile_id: i64,
    }
    ///`CreateUserDto`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "client"
    ///  ],
    ///  "properties": {
    ///    "client": {
    ///      "description": "Client type for authentication",
    ///      "examples": [
    ///        "eoa"
    ///      ],
    ///      "type": "string",
    ///      "enum": [
    ///        "eoa",
    ///        "etherspot",
    ///        "base"
    ///      ]
    ///    },
    ///    "r": {
    ///      "description": "Referral code associated with the user who referred (invited) this user",
    ///      "type": "string"
    ///    },
    ///    "smartWallet": {
    ///      "description": "Smart wallet address (required for Smart Wallet client)",
    ///      "examples": [
    ///        "0x1234567890123456789012345678901234567890"
    ///      ],
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct CreateUserDto {
        ///Client type for authentication
        pub client: CreateUserDtoClient,
        ///Referral code associated with the user who referred (invited) this user
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub r: ::std::option::Option<::std::string::String>,
        ///Smart wallet address (required for Smart Wallet client)
        #[serde(
            rename = "smartWallet",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub smart_wallet: ::std::option::Option<::std::string::String>,
    }
    ///Client type for authentication
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Client type for authentication",
    ///  "examples": [
    ///    "eoa"
    ///  ],
    ///  "type": "string",
    ///  "enum": [
    ///    "eoa",
    ///    "etherspot",
    ///    "base"
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
        PartialOrd,
    )]
    pub enum CreateUserDtoClient {
        #[serde(rename = "eoa")]
        Eoa,
        #[serde(rename = "etherspot")]
        Etherspot,
        #[serde(rename = "base")]
        Base,
    }
    impl ::std::fmt::Display for CreateUserDtoClient {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Eoa => f.write_str("eoa"),
                Self::Etherspot => f.write_str("etherspot"),
                Self::Base => f.write_str("base"),
            }
        }
    }
    impl ::std::str::FromStr for CreateUserDtoClient {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "eoa" => Ok(Self::Eoa),
                "etherspot" => Ok(Self::Etherspot),
                "base" => Ok(Self::Base),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for CreateUserDtoClient {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String> for CreateUserDtoClient {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for CreateUserDtoClient {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///`CursorPaginationDto`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "nextCursor": {
    ///      "description": "Cursor for the next page of results",
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
    pub struct CursorPaginationDto {
        ///Cursor for the next page of results
        #[serde(
            rename = "nextCursor",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub next_cursor: ::std::option::Option<::std::string::String>,
    }
    impl ::std::default::Default for CursorPaginationDto {
        fn default() -> Self {
            Self {
                next_cursor: Default::default(),
            }
        }
    }
    ///`DeleteOrderBatchDto`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "orderIds"
    ///  ],
    ///  "properties": {
    ///    "orderIds": {
    ///      "description": "Array of order IDs to be cancelled in a single batch operation",
    ///      "examples": [
    ///        [
    ///          "6f52b6d2-6c9e-4a5c-8a4f-28ab4b7ff203",
    ///          "9e31c452-8a2b-42d1-b327-65f18d07dc96"
    ///        ]
    ///      ],
    ///      "type": "array",
    ///      "items": {
    ///        "type": "array"
    ///      }
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct DeleteOrderBatchDto {
        ///Array of order IDs to be cancelled in a single batch operation
        #[serde(rename = "orderIds")]
        pub order_ids: ::std::vec::Vec<::std::vec::Vec<::serde_json::Value>>,
    }
    ///`DeriveApiTokenRequest`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "label": {
    ///      "description": "Human-readable label for the token",
    ///      "examples": [
    ///        "production-trading-bot"
    ///      ],
    ///      "type": "string",
    ///      "maxLength": 128
    ///    },
    ///    "scopes": {
    ///      "description": "Scopes to grant. Defaults to [\"trading\"] if omitted. Must be a subset of the partner's allowed scopes. `delegated_signing` requires `trading`.",
    ///      "examples": [
    ///        [
    ///          "trading",
    ///          "account_creation"
    ///        ]
    ///      ],
    ///      "type": "array",
    ///      "items": {
    ///        "type": "string",
    ///        "enum": [
    ///          "trading",
    ///          "account_creation",
    ///          "delegated_signing"
    ///        ]
    ///      }
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct DeriveApiTokenRequest {
        ///Human-readable label for the token
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub label: ::std::option::Option<DeriveApiTokenRequestLabel>,
        ///Scopes to grant. Defaults to ["trading"] if omitted. Must be a subset of the partner's allowed scopes. `delegated_signing` requires `trading`.
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub scopes: ::std::vec::Vec<DeriveApiTokenRequestScopesItem>,
    }
    impl ::std::default::Default for DeriveApiTokenRequest {
        fn default() -> Self {
            Self {
                label: Default::default(),
                scopes: Default::default(),
            }
        }
    }
    ///Human-readable label for the token
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Human-readable label for the token",
    ///  "examples": [
    ///    "production-trading-bot"
    ///  ],
    ///  "type": "string",
    ///  "maxLength": 128
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct DeriveApiTokenRequestLabel(::std::string::String);
    impl ::std::ops::Deref for DeriveApiTokenRequestLabel {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }
    impl ::std::convert::From<DeriveApiTokenRequestLabel> for ::std::string::String {
        fn from(value: DeriveApiTokenRequestLabel) -> Self {
            value.0
        }
    }
    impl ::std::str::FromStr for DeriveApiTokenRequestLabel {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            if value.chars().count() > 128usize {
                return Err("longer than 128 characters".into());
            }
            Ok(Self(value.to_string()))
        }
    }
    impl ::std::convert::TryFrom<&str> for DeriveApiTokenRequestLabel {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String> for DeriveApiTokenRequestLabel {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for DeriveApiTokenRequestLabel {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl<'de> ::serde::Deserialize<'de> for DeriveApiTokenRequestLabel {
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
    ///`DeriveApiTokenRequestScopesItem`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "trading",
    ///    "account_creation",
    ///    "delegated_signing"
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
        PartialOrd,
    )]
    pub enum DeriveApiTokenRequestScopesItem {
        #[serde(rename = "trading")]
        Trading,
        #[serde(rename = "account_creation")]
        AccountCreation,
        #[serde(rename = "delegated_signing")]
        DelegatedSigning,
    }
    impl ::std::fmt::Display for DeriveApiTokenRequestScopesItem {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Trading => f.write_str("trading"),
                Self::AccountCreation => f.write_str("account_creation"),
                Self::DelegatedSigning => f.write_str("delegated_signing"),
            }
        }
    }
    impl ::std::str::FromStr for DeriveApiTokenRequestScopesItem {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "trading" => Ok(Self::Trading),
                "account_creation" => Ok(Self::AccountCreation),
                "delegated_signing" => Ok(Self::DelegatedSigning),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for DeriveApiTokenRequestScopesItem {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String> for DeriveApiTokenRequestScopesItem {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for DeriveApiTokenRequestScopesItem {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///`DeriveApiTokenResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "apiKey",
    ///    "createdAt",
    ///    "profile",
    ///    "scopes",
    ///    "secret",
    ///    "tokenId"
    ///  ],
    ///  "properties": {
    ///    "apiKey": {
    ///      "description": "The token ID, used as the `lmts-api-key` header value for HMAC requests",
    ///      "examples": [
    ///        "dGVzdC10b2tlbi0x"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "createdAt": {
    ///      "description": "Token creation timestamp",
    ///      "type": "string",
    ///      "format": "date-time"
    ///    },
    ///    "profile": {
    ///      "type": "object",
    ///      "properties": {
    ///        "account": {
    ///          "description": "Partner wallet address",
    ///          "examples": [
    ///            "0x27b4afBD88fE7c88c6897BB0b4ADE338D0401E37"
    ///          ],
    ///          "type": "string"
    ///        },
    ///        "id": {
    ///          "description": "Partner profile ID",
    ///          "examples": [
    ///            42
    ///          ],
    ///          "type": "integer"
    ///        }
    ///      }
    ///    },
    ///    "scopes": {
    ///      "description": "Granted scopes",
    ///      "examples": [
    ///        [
    ///          "trading",
    ///          "account_creation"
    ///        ]
    ///      ],
    ///      "type": "array",
    ///      "items": {
    ///        "type": "string"
    ///      }
    ///    },
    ///    "secret": {
    ///      "description": "Base64-encoded secret for HMAC signing. Returned once — store securely.",
    ///      "examples": [
    ///        "c2VjcmV0LWtleS1leGFtcGxlLWJhc2U2NC1lbmNvZGVk"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "tokenId": {
    ///      "description": "Same as apiKey. The unique token identifier.",
    ///      "examples": [
    ///        "dGVzdC10b2tlbi0x"
    ///      ],
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct DeriveApiTokenResponse {
        ///The token ID, used as the `lmts-api-key` header value for HMAC requests
        #[serde(rename = "apiKey")]
        pub api_key: ::std::string::String,
        ///Token creation timestamp
        #[serde(rename = "createdAt")]
        pub created_at: ::chrono::DateTime<::chrono::offset::Utc>,
        pub profile: DeriveApiTokenResponseProfile,
        ///Granted scopes
        pub scopes: ::std::vec::Vec<::std::string::String>,
        ///Base64-encoded secret for HMAC signing. Returned once — store securely.
        pub secret: ::std::string::String,
        ///Same as apiKey. The unique token identifier.
        #[serde(rename = "tokenId")]
        pub token_id: ::std::string::String,
    }
    ///`DeriveApiTokenResponseProfile`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "account": {
    ///      "description": "Partner wallet address",
    ///      "examples": [
    ///        "0x27b4afBD88fE7c88c6897BB0b4ADE338D0401E37"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "id": {
    ///      "description": "Partner profile ID",
    ///      "examples": [
    ///        42
    ///      ],
    ///      "type": "integer"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct DeriveApiTokenResponseProfile {
        ///Partner wallet address
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub account: ::std::option::Option<::std::string::String>,
        ///Partner profile ID
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub id: ::std::option::Option<i64>,
    }
    impl ::std::default::Default for DeriveApiTokenResponseProfile {
        fn default() -> Self {
            Self {
                account: Default::default(),
                id: Default::default(),
            }
        }
    }
    ///`EpochRewardDataDto`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "earnedPercent",
    ///    "epochId",
    ///    "timestamp",
    ///    "totalRewards",
    ///    "userRewards"
    ///  ],
    ///  "properties": {
    ///    "earnedPercent": {
    ///      "description": "Part of total rewards earned by the user - [0; 1]",
    ///      "examples": [
    ///        0.1
    ///      ],
    ///      "type": "number"
    ///    },
    ///    "epochId": {
    ///      "description": "Unique identifier of the epoch",
    ///      "examples": [
    ///        1
    ///      ],
    ///      "type": "number"
    ///    },
    ///    "timestamp": {
    ///      "description": "Timestamp when the epoch occurred",
    ///      "examples": [
    ///        "2024-01-01T00:00:00.000Z"
    ///      ],
    ///      "type": "string",
    ///      "format": "date-time"
    ///    },
    ///    "totalRewards": {
    ///      "description": "Total rewards distributed in this epoch in token decimals",
    ///      "examples": [
    ///        "1500000"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "userRewards": {
    ///      "description": "User rewards earned in this epoch in token decimals",
    ///      "examples": [
    ///        "150000"
    ///      ],
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct EpochRewardDataDto {
        ///Part of total rewards earned by the user - [0; 1]
        #[serde(rename = "earnedPercent")]
        pub earned_percent: f64,
        ///Unique identifier of the epoch
        #[serde(rename = "epochId")]
        pub epoch_id: f64,
        ///Timestamp when the epoch occurred
        pub timestamp: ::chrono::DateTime<::chrono::offset::Utc>,
        ///Total rewards distributed in this epoch in token decimals
        #[serde(rename = "totalRewards")]
        pub total_rewards: ::std::string::String,
        ///User rewards earned in this epoch in token decimals
        #[serde(rename = "userRewards")]
        pub user_rewards: ::std::string::String,
    }
    ///`ErrorResponseDto`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "message"
    ///  ],
    ///  "properties": {
    ///    "message": {
    ///      "description": "Error message",
    ///      "examples": [
    ///        "Invalid order data"
    ///      ],
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ErrorResponseDto {
        ///Error message
        pub message: ::std::string::String,
    }
    ///`FeedEventsResponseDto`
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
    ///      "description": "Array of feed events with pagination (FEFeedEvents type)",
    ///      "examples": [
    ///        {
    ///          "data": [
    ///            {
    ///              "data": {
    ///                "outcome": "YES",
    ///                "strategy": "Buy",
    ///                "title": "Market Title"
    ///              },
    ///              "eventType": "NEW_TRADE",
    ///              "timestamp": "2025-09-01T11:30:31.000Z",
    ///              "user": {
    ///                "name": "GG",
    ///                "rankName": "Bronze"
    ///              }
    ///            }
    ///          ],
    ///          "totalPages": 5
    ///        }
    ///      ],
    ///      "type": "object",
    ///      "additionalProperties": true
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct FeedEventsResponseDto {
        ///Array of feed events with pagination (FEFeedEvents type)
        pub data: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    }
    ///`FilterGroupDto`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "allowMultiple": {
    ///      "description": "Whether multiple options can be selected",
    ///      "type": "boolean"
    ///    },
    ///    "name": {
    ///      "description": "Display name of the filter group",
    ///      "type": "string"
    ///    },
    ///    "options": {
    ///      "description": "Available filter options",
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/FilterGroupOptionDto"
    ///      }
    ///    },
    ///    "presentation": {
    ///      "description": "Presentation style for the filter group",
    ///      "type": "string"
    ///    },
    ///    "slug": {
    ///      "description": "URL slug for the filter group",
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct FilterGroupDto {
        ///Whether multiple options can be selected
        #[serde(
            rename = "allowMultiple",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub allow_multiple: ::std::option::Option<bool>,
        ///Display name of the filter group
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub name: ::std::option::Option<::std::string::String>,
        ///Available filter options
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub options: ::std::vec::Vec<FilterGroupOptionDto>,
        ///Presentation style for the filter group
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub presentation: ::std::option::Option<::std::string::String>,
        ///URL slug for the filter group
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub slug: ::std::option::Option<::std::string::String>,
    }
    impl ::std::default::Default for FilterGroupDto {
        fn default() -> Self {
            Self {
                allow_multiple: Default::default(),
                name: Default::default(),
                options: Default::default(),
                presentation: Default::default(),
                slug: Default::default(),
            }
        }
    }
    ///`FilterGroupOptionDto`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "label",
    ///    "value"
    ///  ],
    ///  "properties": {
    ///    "label": {
    ///      "description": "Display label for the option",
    ///      "type": "string"
    ///    },
    ///    "metadata": {
    ///      "description": "Optional metadata for the option (color, icon)",
    ///      "type": "object"
    ///    },
    ///    "value": {
    ///      "description": "Value identifier for the option",
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct FilterGroupOptionDto {
        ///Display label for the option
        pub label: ::std::string::String,
        ///Optional metadata for the option (color, icon)
        #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
        pub metadata: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        ///Value identifier for the option
        pub value: ::std::string::String,
    }
    ///`HistoryEntryDto`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "blockTimestamp",
    ///    "collateralAmount",
    ///    "market",
    ///    "outcomeIndex",
    ///    "outcomeTokenAmount",
    ///    "outcomeTokenAmounts",
    ///    "outcomeTokenPrice",
    ///    "strategy"
    ///  ],
    ///  "properties": {
    ///    "blockTimestamp": {
    ///      "description": "Block timestamp of the operation (unix seconds)",
    ///      "examples": [
    ///        1744115608
    ///      ],
    ///      "type": "number"
    ///    },
    ///    "collateralAmount": {
    ///      "description": "Collateral amount involved (human-readable, after dividing by token decimals)",
    ///      "examples": [
    ///        "25.5"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "market": {
    ///      "description": "Market information",
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/HistoryMarketDto"
    ///        }
    ///      ]
    ///    },
    ///    "outcomeIndex": {
    ///      "description": "Index of the outcome (0 = YES, 1 = NO)",
    ///      "examples": [
    ///        0
    ///      ],
    ///      "type": "number"
    ///    },
    ///    "outcomeTokenAmount": {
    ///      "description": "Amount of outcome tokens involved (human-readable)",
    ///      "examples": [
    ///        "50"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "outcomeTokenAmounts": {
    ///      "description": "Amounts of outcome tokens for each outcome [YES, NO]",
    ///      "examples": [
    ///        [
    ///          "50",
    ///          "0"
    ///        ]
    ///      ],
    ///      "type": "array",
    ///      "items": {
    ///        "type": "string"
    ///      }
    ///    },
    ///    "outcomeTokenPrice": {
    ///      "description": "Effective price per outcome token for this fill",
    ///      "examples": [
    ///        0.51
    ///      ],
    ///      "type": "number"
    ///    },
    ///    "strategy": {
    ///      "description": "Type of operation. `Buy`/`Sell` = AMM trades, `Limit Buy`/`Limit Sell` = CLOB maker fills, `Market Buy`/`Market Sell` = CLOB taker fills, `Split`/`Merge` = position split/merge, `Convert` = NegRisk conversion, `Claim` = winnings redemption.",
    ///      "examples": [
    ///        "Limit Buy"
    ///      ],
    ///      "type": "string",
    ///      "enum": [
    ///        "Buy",
    ///        "Sell",
    ///        "Limit Buy",
    ///        "Limit Sell",
    ///        "Market Buy",
    ///        "Market Sell",
    ///        "Split",
    ///        "Merge",
    ///        "Convert",
    ///        "Claim"
    ///      ]
    ///    },
    ///    "transactionHash": {
    ///      "description": "On-chain transaction hash (present for settled trades, splits, merges, conversions, and claims)",
    ///      "examples": [
    ///        "0x992f36465f938b21a6a5fe3c417c98c3268a616a05479d2dc53870c6cd1a0761"
    ///      ],
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct HistoryEntryDto {
        ///Block timestamp of the operation (unix seconds)
        #[serde(rename = "blockTimestamp")]
        pub block_timestamp: f64,
        ///Collateral amount involved (human-readable, after dividing by token decimals)
        #[serde(rename = "collateralAmount")]
        pub collateral_amount: ::std::string::String,
        ///Market information
        pub market: HistoryMarketDto,
        ///Index of the outcome (0 = YES, 1 = NO)
        #[serde(rename = "outcomeIndex")]
        pub outcome_index: f64,
        ///Amount of outcome tokens involved (human-readable)
        #[serde(rename = "outcomeTokenAmount")]
        pub outcome_token_amount: ::std::string::String,
        ///Amounts of outcome tokens for each outcome [YES, NO]
        #[serde(rename = "outcomeTokenAmounts")]
        pub outcome_token_amounts: ::std::vec::Vec<::std::string::String>,
        ///Effective price per outcome token for this fill
        #[serde(rename = "outcomeTokenPrice")]
        pub outcome_token_price: f64,
        ///Type of operation. `Buy`/`Sell` = AMM trades, `Limit Buy`/`Limit Sell` = CLOB maker fills, `Market Buy`/`Market Sell` = CLOB taker fills, `Split`/`Merge` = position split/merge, `Convert` = NegRisk conversion, `Claim` = winnings redemption.
        pub strategy: HistoryEntryDtoStrategy,
        ///On-chain transaction hash (present for settled trades, splits, merges, conversions, and claims)
        #[serde(
            rename = "transactionHash",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub transaction_hash: ::std::option::Option<::std::string::String>,
    }
    ///Type of operation. `Buy`/`Sell` = AMM trades, `Limit Buy`/`Limit Sell` = CLOB maker fills, `Market Buy`/`Market Sell` = CLOB taker fills, `Split`/`Merge` = position split/merge, `Convert` = NegRisk conversion, `Claim` = winnings redemption.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Type of operation. `Buy`/`Sell` = AMM trades, `Limit Buy`/`Limit Sell` = CLOB maker fills, `Market Buy`/`Market Sell` = CLOB taker fills, `Split`/`Merge` = position split/merge, `Convert` = NegRisk conversion, `Claim` = winnings redemption.",
    ///  "examples": [
    ///    "Limit Buy"
    ///  ],
    ///  "type": "string",
    ///  "enum": [
    ///    "Buy",
    ///    "Sell",
    ///    "Limit Buy",
    ///    "Limit Sell",
    ///    "Market Buy",
    ///    "Market Sell",
    ///    "Split",
    ///    "Merge",
    ///    "Convert",
    ///    "Claim"
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
        PartialOrd,
    )]
    pub enum HistoryEntryDtoStrategy {
        Buy,
        Sell,
        #[serde(rename = "Limit Buy")]
        LimitBuy,
        #[serde(rename = "Limit Sell")]
        LimitSell,
        #[serde(rename = "Market Buy")]
        MarketBuy,
        #[serde(rename = "Market Sell")]
        MarketSell,
        Split,
        Merge,
        Convert,
        Claim,
    }
    impl ::std::fmt::Display for HistoryEntryDtoStrategy {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Buy => f.write_str("Buy"),
                Self::Sell => f.write_str("Sell"),
                Self::LimitBuy => f.write_str("Limit Buy"),
                Self::LimitSell => f.write_str("Limit Sell"),
                Self::MarketBuy => f.write_str("Market Buy"),
                Self::MarketSell => f.write_str("Market Sell"),
                Self::Split => f.write_str("Split"),
                Self::Merge => f.write_str("Merge"),
                Self::Convert => f.write_str("Convert"),
                Self::Claim => f.write_str("Claim"),
            }
        }
    }
    impl ::std::str::FromStr for HistoryEntryDtoStrategy {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "Buy" => Ok(Self::Buy),
                "Sell" => Ok(Self::Sell),
                "Limit Buy" => Ok(Self::LimitBuy),
                "Limit Sell" => Ok(Self::LimitSell),
                "Market Buy" => Ok(Self::MarketBuy),
                "Market Sell" => Ok(Self::MarketSell),
                "Split" => Ok(Self::Split),
                "Merge" => Ok(Self::Merge),
                "Convert" => Ok(Self::Convert),
                "Claim" => Ok(Self::Claim),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for HistoryEntryDtoStrategy {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String> for HistoryEntryDtoStrategy {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for HistoryEntryDtoStrategy {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///`HistoryMarketDto`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "closed",
    ///    "collateral",
    ///    "condition_id",
    ///    "deadline",
    ///    "funding",
    ///    "group",
    ///    "id",
    ///    "slug",
    ///    "title"
    ///  ],
    ///  "properties": {
    ///    "closed": {
    ///      "description": "Whether the market is closed",
    ///      "examples": [
    ///        false
    ///      ],
    ///      "type": "boolean"
    ///    },
    ///    "collateral": {
    ///      "description": "Collateral token information",
    ///      "examples": [
    ///        {
    ///          "decimals": 6,
    ///          "id": 7,
    ///          "symbol": "USDC"
    ///        }
    ///      ],
    ///      "type": "object"
    ///    },
    ///    "condition_id": {
    ///      "description": "Condition ID of the market",
    ///      "examples": [
    ///        "0x08518bc4bb8a3dbb89aac4425ace0876b94a5dfa77dee47a20737a19cf67e325"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "deadline": {
    ///      "description": "Market deadline",
    ///      "examples": [
    ///        "2025-07-04T23:59:59.000Z"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "funding": {
    ///      "description": "Market funding amount",
    ///      "examples": [
    ///        0
    ///      ],
    ///      "type": "number"
    ///    },
    ///    "group": {
    ///      "description": "Market group information",
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/HistoryMarketGroupDto"
    ///        }
    ///      ]
    ///    },
    ///    "id": {
    ///      "description": "Market ID",
    ///      "examples": [
    ///        980
    ///      ],
    ///      "type": "number"
    ///    },
    ///    "slug": {
    ///      "description": "Market slug",
    ///      "examples": [
    ///        "btc-above-100k-jul-4"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "title": {
    ///      "description": "Market title",
    ///      "examples": [
    ///        "$BTC above $100k on Jul 4?"
    ///      ],
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct HistoryMarketDto {
        ///Whether the market is closed
        pub closed: bool,
        ///Collateral token information
        pub collateral: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        ///Condition ID of the market
        pub condition_id: ::std::string::String,
        ///Market deadline
        pub deadline: ::std::string::String,
        ///Market funding amount
        pub funding: f64,
        ///Market group information
        pub group: HistoryMarketGroupDto,
        ///Market ID
        pub id: f64,
        ///Market slug
        pub slug: ::std::string::String,
        ///Market title
        pub title: ::std::string::String,
    }
    ///`HistoryMarketGroupDto`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "createdAt",
    ///    "deadline",
    ///    "hidden",
    ///    "id",
    ///    "metadata",
    ///    "negRiskMarketId",
    ///    "priorityIndex",
    ///    "slug",
    ///    "status",
    ///    "title",
    ///    "updatedAt"
    ///  ],
    ///  "properties": {
    ///    "createdAt": {
    ///      "description": "Creation timestamp",
    ///      "examples": [
    ///        "2025-04-07T17:20:22.135Z"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "deadline": {
    ///      "description": "Deadline for the group",
    ///      "examples": [
    ///        "2025-04-11T22:34:56.000Z"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "hidden": {
    ///      "description": "Whether the group is hidden",
    ///      "examples": [
    ///        false
    ///      ],
    ///      "type": "boolean"
    ///    },
    ///    "id": {
    ///      "description": "Unique identifier of the group",
    ///      "examples": [
    ///        10000037
    ///      ],
    ///      "type": "number"
    ///    },
    ///    "metadata": {
    ///      "description": "Group metadata",
    ///      "examples": [
    ///        {
    ///          "isBannered": false
    ///        }
    ///      ],
    ///      "type": "object"
    ///    },
    ///    "negRiskMarketId": {
    ///      "description": "Onchain NegRisk market ID as identified by the NegriskAdapter smart-contract",
    ///      "examples": [
    ///        "0xe103633b40e9b664f8acc89e8cf7b7916475961ae1708a249fa5d6c933168c00"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "priorityIndex": {
    ///      "description": "Priority index of the group",
    ///      "examples": [
    ///        0
    ///      ],
    ///      "type": "number"
    ///    },
    ///    "resolutionTxHash": {
    ///      "description": "Resolution transaction hash",
    ///      "type": "string"
    ///    },
    ///    "slug": {
    ///      "description": "Slug identifier of the group",
    ///      "examples": [
    ///        "positionconverted-test-1744046422596"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "status": {
    ///      "description": "Status of the group",
    ///      "examples": [
    ///        "FUNDED"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "title": {
    ///      "description": "Title of the group",
    ///      "examples": [
    ///        "PositionConverted test"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "txHash": {
    ///      "description": "Transaction hash for the group",
    ///      "type": "string"
    ///    },
    ///    "updatedAt": {
    ///      "description": "Last update timestamp",
    ///      "examples": [
    ///        "2025-04-07T17:22:08.464Z"
    ///      ],
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct HistoryMarketGroupDto {
        ///Creation timestamp
        #[serde(rename = "createdAt")]
        pub created_at: ::std::string::String,
        ///Deadline for the group
        pub deadline: ::std::string::String,
        ///Whether the group is hidden
        pub hidden: bool,
        ///Unique identifier of the group
        pub id: f64,
        ///Group metadata
        pub metadata: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        ///Onchain NegRisk market ID as identified by the NegriskAdapter smart-contract
        #[serde(rename = "negRiskMarketId")]
        pub neg_risk_market_id: ::std::string::String,
        ///Priority index of the group
        #[serde(rename = "priorityIndex")]
        pub priority_index: f64,
        ///Resolution transaction hash
        #[serde(
            rename = "resolutionTxHash",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub resolution_tx_hash: ::std::option::Option<::std::string::String>,
        ///Slug identifier of the group
        pub slug: ::std::string::String,
        ///Status of the group
        pub status: ::std::string::String,
        ///Title of the group
        pub title: ::std::string::String,
        ///Transaction hash for the group
        #[serde(
            rename = "txHash",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub tx_hash: ::std::option::Option<::std::string::String>,
        ///Last update timestamp
        #[serde(rename = "updatedAt")]
        pub updated_at: ::std::string::String,
    }
    ///`HistoryResponseDto`
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
    ///      "description": "List of history entries",
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/HistoryEntryDto"
    ///      }
    ///    },
    ///    "nextCursor": {
    ///      "description": "Opaque cursor for the next page. `null` when there are no more pages. Pass this value back as the `cursor` query parameter to fetch the next page.",
    ///      "examples": [
    ///        "eyJ0Ijoi..."
    ///      ],
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
    pub struct HistoryResponseDto {
        ///List of history entries
        pub data: ::std::vec::Vec<HistoryEntryDto>,
        ///Opaque cursor for the next page. `null` when there are no more pages. Pass this value back as the `cursor` query parameter to fetch the next page.
        #[serde(
            rename = "nextCursor",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub next_cursor: ::std::option::Option<::std::string::String>,
    }
    ///An AMM trade executed by the user
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "An AMM trade executed by the user",
    ///  "type": "object",
    ///  "required": [
    ///    "blockTimestamp",
    ///    "market",
    ///    "outcomeIndex",
    ///    "outcomeTokenAmounts",
    ///    "outcomeTokenNetCost",
    ///    "transactionHash"
    ///  ],
    ///  "properties": {
    ///    "blockTimestamp": {
    ///      "description": "Timestamp of the block containing the trade",
    ///      "examples": [
    ///        "2025-09-01T11:30:31.000Z"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "collateralAmount": {
    ///      "description": "Collateral amount traded (in token decimals)",
    ///      "examples": [
    ///        "5000000"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "market": {
    ///      "description": "Market details for the trade",
    ///      "type": "object",
    ///      "properties": {
    ///        "conditionId": {
    ///          "examples": [
    ///            "0x812f578437dc..."
    ///          ],
    ///          "type": "string"
    ///        },
    ///        "expirationDate": {
    ///          "examples": [
    ///            "Sep 1, 2025"
    ///          ],
    ///          "type": "string"
    ///        },
    ///        "id": {
    ///          "examples": [
    ///            "0x76d3e2098Be6..."
    ///          ],
    ///          "type": "string"
    ///        },
    ///        "slug": {
    ///          "examples": [
    ///            "doge-above-021-sep-1"
    ///          ],
    ///          "type": "string"
    ///        },
    ///        "title": {
    ///          "examples": [
    ///            "$DOGE above $0.21652 on Sep 1?"
    ///          ],
    ///          "type": "string"
    ///        }
    ///      }
    ///    },
    ///    "outcomeIndex": {
    ///      "description": "Index of the outcome traded (0 = YES, 1 = NO)",
    ///      "examples": [
    ///        0
    ///      ],
    ///      "type": "number"
    ///    },
    ///    "outcomeTokenAmount": {
    ///      "description": "Amount of outcome tokens traded",
    ///      "examples": [
    ///        "10000000"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "outcomeTokenAmounts": {
    ///      "description": "Collateral per outcome token",
    ///      "examples": [
    ///        [
    ///          "5000000",
    ///          "0"
    ///        ]
    ///      ],
    ///      "type": "array",
    ///      "items": {
    ///        "type": "string"
    ///      }
    ///    },
    ///    "outcomeTokenNetCost": {
    ///      "description": "Net cost of the outcome tokens",
    ///      "examples": [
    ///        "4950000"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "outcomeTokenPrice": {
    ///      "description": "Price of the outcome token at trade time",
    ///      "examples": [
    ///        "0.50"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "strategy": {
    ///      "description": "Trade direction",
    ///      "examples": [
    ///        "Buy"
    ///      ],
    ///      "type": "string",
    ///      "enum": [
    ///        "Buy",
    ///        "Sell"
    ///      ]
    ///    },
    ///    "transactionHash": {
    ///      "description": "On-chain transaction hash",
    ///      "examples": [
    ///        "0xabc123..."
    ///      ],
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct HistoryTradeDto {
        ///Timestamp of the block containing the trade
        #[serde(rename = "blockTimestamp")]
        pub block_timestamp: ::std::string::String,
        ///Collateral amount traded (in token decimals)
        #[serde(
            rename = "collateralAmount",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub collateral_amount: ::std::option::Option<::std::string::String>,
        pub market: HistoryTradeDtoMarket,
        ///Index of the outcome traded (0 = YES, 1 = NO)
        #[serde(rename = "outcomeIndex")]
        pub outcome_index: f64,
        ///Amount of outcome tokens traded
        #[serde(
            rename = "outcomeTokenAmount",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub outcome_token_amount: ::std::option::Option<::std::string::String>,
        ///Collateral per outcome token
        #[serde(rename = "outcomeTokenAmounts")]
        pub outcome_token_amounts: ::std::vec::Vec<::std::string::String>,
        ///Net cost of the outcome tokens
        #[serde(rename = "outcomeTokenNetCost")]
        pub outcome_token_net_cost: ::std::string::String,
        ///Price of the outcome token at trade time
        #[serde(
            rename = "outcomeTokenPrice",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub outcome_token_price: ::std::option::Option<::std::string::String>,
        ///Trade direction
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub strategy: ::std::option::Option<HistoryTradeDtoStrategy>,
        ///On-chain transaction hash
        #[serde(rename = "transactionHash")]
        pub transaction_hash: ::std::string::String,
    }
    ///Market details for the trade
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Market details for the trade",
    ///  "type": "object",
    ///  "properties": {
    ///    "conditionId": {
    ///      "examples": [
    ///        "0x812f578437dc..."
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "expirationDate": {
    ///      "examples": [
    ///        "Sep 1, 2025"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "id": {
    ///      "examples": [
    ///        "0x76d3e2098Be6..."
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "slug": {
    ///      "examples": [
    ///        "doge-above-021-sep-1"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "title": {
    ///      "examples": [
    ///        "$DOGE above $0.21652 on Sep 1?"
    ///      ],
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct HistoryTradeDtoMarket {
        #[serde(
            rename = "conditionId",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub condition_id: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "expirationDate",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub expiration_date: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub id: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub slug: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub title: ::std::option::Option<::std::string::String>,
    }
    impl ::std::default::Default for HistoryTradeDtoMarket {
        fn default() -> Self {
            Self {
                condition_id: Default::default(),
                expiration_date: Default::default(),
                id: Default::default(),
                slug: Default::default(),
                title: Default::default(),
            }
        }
    }
    ///Trade direction
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Trade direction",
    ///  "examples": [
    ///    "Buy"
    ///  ],
    ///  "type": "string",
    ///  "enum": [
    ///    "Buy",
    ///    "Sell"
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
        PartialOrd,
    )]
    pub enum HistoryTradeDtoStrategy {
        Buy,
        Sell,
    }
    impl ::std::fmt::Display for HistoryTradeDtoStrategy {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Buy => f.write_str("Buy"),
                Self::Sell => f.write_str("Sell"),
            }
        }
    }
    impl ::std::str::FromStr for HistoryTradeDtoStrategy {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "Buy" => Ok(Self::Buy),
                "Sell" => Ok(Self::Sell),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for HistoryTradeDtoStrategy {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String> for HistoryTradeDtoStrategy {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for HistoryTradeDtoStrategy {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///`ListMarketsResponseDto`
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
    ///    "cursor": {
    ///      "description": "Cursor pagination info (when using cursor parameter)",
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/CursorPaginationDto"
    ///        }
    ///      ]
    ///    },
    ///    "data": {
    ///      "description": "Array of market/group data",
    ///      "type": "array",
    ///      "items": {
    ///        "type": "string"
    ///      }
    ///    },
    ///    "pagination": {
    ///      "description": "Offset pagination info (when using page parameter)",
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/OffsetPaginationDto"
    ///        }
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ListMarketsResponseDto {
        ///Cursor pagination info (when using cursor parameter)
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub cursor: ::std::option::Option<CursorPaginationDto>,
        ///Array of market/group data
        pub data: ::std::vec::Vec<::std::string::String>,
        ///Offset pagination info (when using page parameter)
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub pagination: ::std::option::Option<OffsetPaginationDto>,
    }
    ///`MakerMatch`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object"
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    #[serde(transparent)]
    pub struct MakerMatch(pub ::serde_json::Map<::std::string::String, ::serde_json::Value>);
    impl ::std::ops::Deref for MakerMatch {
        type Target = ::serde_json::Map<::std::string::String, ::serde_json::Value>;
        fn deref(&self) -> &::serde_json::Map<::std::string::String, ::serde_json::Value> {
            &self.0
        }
    }
    impl ::std::convert::From<MakerMatch>
        for ::serde_json::Map<::std::string::String, ::serde_json::Value>
    {
        fn from(value: MakerMatch) -> Self {
            value.0
        }
    }
    impl ::std::convert::From<::serde_json::Map<::std::string::String, ::serde_json::Value>>
        for MakerMatch
    {
        fn from(value: ::serde_json::Map<::std::string::String, ::serde_json::Value>) -> Self {
            Self(value)
        }
    }
    ///`Market`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "address",
    ///    "condition_id",
    ///    "deadline",
    ///    "description",
    ///    "hidden",
    ///    "image_url",
    ///    "og_url",
    ///    "outcome_slot_count",
    ///    "payout_numerators",
    ///    "position_ids",
    ///    "priority_index",
    ///    "proxyTitle",
    ///    "question_id",
    ///    "resolutionTxHash",
    ///    "status",
    ///    "title",
    ///    "txHash",
    ///    "winning_index"
    ///  ],
    ///  "properties": {
    ///    "address": {
    ///      "description": "The address of the FixedProductMarketMaker contract.",
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ],
    ///      "maxLength": 42,
    ///      "minLength": 42,
    ///      "uniqueItems": true
    ///    },
    ///    "condition_id": {
    ///      "description": "The bytes32 conditionId of the market, representing the conditions defined on condition preparation.",
    ///      "type": [
    ///        "object",
    ///        "null"
    ///      ],
    ///      "maxLength": 66,
    ///      "minLength": 66,
    ///      "uniqueItems": true
    ///    },
    ///    "deadline": {
    ///      "description": "The deadline of the market.",
    ///      "type": "string"
    ///    },
    ///    "description": {
    ///      "description": "The description of the market.",
    ///      "type": "string",
    ///      "uniqueItems": false
    ///    },
    ///    "hidden": {
    ///      "default": false,
    ///      "type": "boolean"
    ///    },
    ///    "image_url": {
    ///      "description": "The URL of the market logo image for SEO.",
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "og_url": {
    ///      "description": "The URL of the market OG image for SEO.",
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "outcome_slot_count": {
    ///      "description": "The number of outcomes in the market.",
    ///      "default": 2,
    ///      "type": "number",
    ///      "maximum": 2.0,
    ///      "minimum": 2.0
    ///    },
    ///    "payout_numerators": {
    ///      "description": "The oracle payout numerators that the oracle reported.",
    ///      "type": [
    ///        "array",
    ///        "null"
    ///      ],
    ///      "items": {
    ///        "type": "string"
    ///      }
    ///    },
    ///    "position_ids": {
    ///      "description": "Array of position IDs (max 2 elements)",
    ///      "type": [
    ///        "array",
    ///        "null"
    ///      ],
    ///      "items": {
    ///        "type": "array"
    ///      },
    ///      "maxItems": 2
    ///    },
    ///    "priority_index": {
    ///      "type": "number"
    ///    },
    ///    "proxyTitle": {
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ],
    ///      "maxLength": 70,
    ///      "uniqueItems": true
    ///    },
    ///    "question_id": {
    ///      "description": "The bytes32 parsed title of the market.",
    ///      "type": [
    ///        "object",
    ///        "null"
    ///      ],
    ///      "maxLength": 66,
    ///      "minLength": 66,
    ///      "uniqueItems": true
    ///    },
    ///    "resolutionTxHash": {
    ///      "type": "string"
    ///    },
    ///    "status": {
    ///      "description": "Indicates the status of the market.",
    ///      "type": "string"
    ///    },
    ///    "title": {
    ///      "description": "Represents the question of the market, the title metadata of the market.",
    ///      "type": "string",
    ///      "maxLength": 70,
    ///      "uniqueItems": true
    ///    },
    ///    "txHash": {
    ///      "type": "string"
    ///    },
    ///    "winning_index": {
    ///      "description": "The outcome index that was reported by the oracle.",
    ///      "type": [
    ///        "number",
    ///        "null"
    ///      ],
    ///      "maximum": 1.0,
    ///      "minimum": 0.0
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct Market {
        ///The address of the FixedProductMarketMaker contract.
        pub address: ::std::option::Option<MarketAddress>,
        ///The bytes32 conditionId of the market, representing the conditions defined on condition preparation.
        pub condition_id:
            ::std::option::Option<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
        ///The deadline of the market.
        pub deadline: ::std::string::String,
        ///The description of the market.
        pub description: ::std::string::String,
        pub hidden: bool,
        ///The URL of the market logo image for SEO.
        pub image_url: ::std::option::Option<::std::string::String>,
        ///The URL of the market OG image for SEO.
        pub og_url: ::std::option::Option<::std::string::String>,
        ///The number of outcomes in the market.
        pub outcome_slot_count: f64,
        ///The oracle payout numerators that the oracle reported.
        pub payout_numerators: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        ///Array of position IDs (max 2 elements)
        pub position_ids:
            ::std::option::Option<::std::vec::Vec<::std::vec::Vec<::serde_json::Value>>>,
        pub priority_index: f64,
        #[serde(rename = "proxyTitle")]
        pub proxy_title: ::std::option::Option<MarketProxyTitle>,
        ///The bytes32 parsed title of the market.
        pub question_id:
            ::std::option::Option<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
        #[serde(rename = "resolutionTxHash")]
        pub resolution_tx_hash: ::std::string::String,
        ///Indicates the status of the market.
        pub status: ::std::string::String,
        ///Represents the question of the market, the title metadata of the market.
        pub title: MarketTitle,
        #[serde(rename = "txHash")]
        pub tx_hash: ::std::string::String,
        ///The outcome index that was reported by the oracle.
        pub winning_index: ::std::option::Option<f64>,
    }
    ///The address of the FixedProductMarketMaker contract.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "The address of the FixedProductMarketMaker contract.",
    ///  "type": "string",
    ///  "maxLength": 42,
    ///  "minLength": 42,
    ///  "uniqueItems": true
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct MarketAddress(::std::string::String);
    impl ::std::ops::Deref for MarketAddress {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }
    impl ::std::convert::From<MarketAddress> for ::std::string::String {
        fn from(value: MarketAddress) -> Self {
            value.0
        }
    }
    impl ::std::str::FromStr for MarketAddress {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            if value.chars().count() > 42usize {
                return Err("longer than 42 characters".into());
            }
            if value.chars().count() < 42usize {
                return Err("shorter than 42 characters".into());
            }
            Ok(Self(value.to_string()))
        }
    }
    impl ::std::convert::TryFrom<&str> for MarketAddress {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String> for MarketAddress {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for MarketAddress {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl<'de> ::serde::Deserialize<'de> for MarketAddress {
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
    ///`MarketControllerFindResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "oneOf": [
    ///    {
    ///      "$ref": "#/components/schemas/ClobMarketResponseDto"
    ///    },
    ///    {
    ///      "$ref": "#/components/schemas/NegRiskGroupResponseDto"
    ///    },
    ///    {
    ///      "$ref": "#/components/schemas/AmmMarketResponseDto"
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    #[serde(untagged)]
    pub enum MarketControllerFindResponse {
        ClobMarketResponseDto(ClobMarketResponseDto),
        NegRiskGroupResponseDto(NegRiskGroupResponseDto),
        AmmMarketResponseDto(AmmMarketResponseDto),
    }
    impl ::std::convert::From<ClobMarketResponseDto> for MarketControllerFindResponse {
        fn from(value: ClobMarketResponseDto) -> Self {
            Self::ClobMarketResponseDto(value)
        }
    }
    impl ::std::convert::From<NegRiskGroupResponseDto> for MarketControllerFindResponse {
        fn from(value: NegRiskGroupResponseDto) -> Self {
            Self::NegRiskGroupResponseDto(value)
        }
    }
    impl ::std::convert::From<AmmMarketResponseDto> for MarketControllerFindResponse {
        fn from(value: AmmMarketResponseDto) -> Self {
            Self::AmmMarketResponseDto(value)
        }
    }
    ///`MarketControllerGetActiveMarkets0AutomationType`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "manual",
    ///    "lumy",
    ///    "sports"
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
        PartialOrd,
    )]
    pub enum MarketControllerGetActiveMarkets0AutomationType {
        #[serde(rename = "manual")]
        Manual,
        #[serde(rename = "lumy")]
        Lumy,
        #[serde(rename = "sports")]
        Sports,
    }
    impl ::std::fmt::Display for MarketControllerGetActiveMarkets0AutomationType {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Manual => f.write_str("manual"),
                Self::Lumy => f.write_str("lumy"),
                Self::Sports => f.write_str("sports"),
            }
        }
    }
    impl ::std::str::FromStr for MarketControllerGetActiveMarkets0AutomationType {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "manual" => Ok(Self::Manual),
                "lumy" => Ok(Self::Lumy),
                "sports" => Ok(Self::Sports),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for MarketControllerGetActiveMarkets0AutomationType {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String>
        for MarketControllerGetActiveMarkets0AutomationType
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String>
        for MarketControllerGetActiveMarkets0AutomationType
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///`MarketControllerGetActiveMarkets0TradeType`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "amm",
    ///    "clob",
    ///    "group"
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
        PartialOrd,
    )]
    pub enum MarketControllerGetActiveMarkets0TradeType {
        #[serde(rename = "amm")]
        Amm,
        #[serde(rename = "clob")]
        Clob,
        #[serde(rename = "group")]
        Group,
    }
    impl ::std::fmt::Display for MarketControllerGetActiveMarkets0TradeType {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Amm => f.write_str("amm"),
                Self::Clob => f.write_str("clob"),
                Self::Group => f.write_str("group"),
            }
        }
    }
    impl ::std::str::FromStr for MarketControllerGetActiveMarkets0TradeType {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "amm" => Ok(Self::Amm),
                "clob" => Ok(Self::Clob),
                "group" => Ok(Self::Group),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for MarketControllerGetActiveMarkets0TradeType {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String>
        for MarketControllerGetActiveMarkets0TradeType
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for MarketControllerGetActiveMarkets0TradeType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///`MarketControllerGetActiveMarkets1AutomationType`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "manual",
    ///    "lumy",
    ///    "sports"
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
        PartialOrd,
    )]
    pub enum MarketControllerGetActiveMarkets1AutomationType {
        #[serde(rename = "manual")]
        Manual,
        #[serde(rename = "lumy")]
        Lumy,
        #[serde(rename = "sports")]
        Sports,
    }
    impl ::std::fmt::Display for MarketControllerGetActiveMarkets1AutomationType {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Manual => f.write_str("manual"),
                Self::Lumy => f.write_str("lumy"),
                Self::Sports => f.write_str("sports"),
            }
        }
    }
    impl ::std::str::FromStr for MarketControllerGetActiveMarkets1AutomationType {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "manual" => Ok(Self::Manual),
                "lumy" => Ok(Self::Lumy),
                "sports" => Ok(Self::Sports),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for MarketControllerGetActiveMarkets1AutomationType {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String>
        for MarketControllerGetActiveMarkets1AutomationType
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String>
        for MarketControllerGetActiveMarkets1AutomationType
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///`MarketControllerGetActiveMarkets1TradeType`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "amm",
    ///    "clob",
    ///    "group"
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
        PartialOrd,
    )]
    pub enum MarketControllerGetActiveMarkets1TradeType {
        #[serde(rename = "amm")]
        Amm,
        #[serde(rename = "clob")]
        Clob,
        #[serde(rename = "group")]
        Group,
    }
    impl ::std::fmt::Display for MarketControllerGetActiveMarkets1TradeType {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Amm => f.write_str("amm"),
                Self::Clob => f.write_str("clob"),
                Self::Group => f.write_str("group"),
            }
        }
    }
    impl ::std::str::FromStr for MarketControllerGetActiveMarkets1TradeType {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "amm" => Ok(Self::Amm),
                "clob" => Ok(Self::Clob),
                "group" => Ok(Self::Group),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for MarketControllerGetActiveMarkets1TradeType {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String>
        for MarketControllerGetActiveMarkets1TradeType
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for MarketControllerGetActiveMarkets1TradeType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///`MarketControllerGetActiveSlugsResponseItem`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "slug"
    ///  ],
    ///  "properties": {
    ///    "deadline": {
    ///      "description": "Expiration deadline",
    ///      "examples": [
    ///        "2024-12-31T23:59:59Z"
    ///      ],
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ],
    ///      "format": "date-time"
    ///    },
    ///    "markets": {
    ///      "description": "Nested markets (group entries only)",
    ///      "type": [
    ///        "array",
    ///        "null"
    ///      ],
    ///      "items": {
    ///        "type": "object",
    ///        "properties": {
    ///          "slug": {
    ///            "type": "string"
    ///          }
    ///        }
    ///      }
    ///    },
    ///    "slug": {
    ///      "description": "Market or group slug identifier",
    ///      "examples": [
    ///        "btc-price-prediction-2024"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "strikePrice": {
    ///      "description": "Strike price (null for groups)",
    ///      "examples": [
    ///        "50000"
    ///      ],
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "ticker": {
    ///      "description": "Asset ticker symbol",
    ///      "examples": [
    ///        "BTC"
    ///      ],
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
    pub struct MarketControllerGetActiveSlugsResponseItem {
        ///Expiration deadline
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub deadline: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
        ///Nested markets (group entries only)
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub markets: ::std::option::Option<
            ::std::vec::Vec<MarketControllerGetActiveSlugsResponseItemMarketsItem>,
        >,
        ///Market or group slug identifier
        pub slug: ::std::string::String,
        ///Strike price (null for groups)
        #[serde(
            rename = "strikePrice",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub strike_price: ::std::option::Option<::std::string::String>,
        ///Asset ticker symbol
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub ticker: ::std::option::Option<::std::string::String>,
    }
    ///`MarketControllerGetActiveSlugsResponseItemMarketsItem`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "slug": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct MarketControllerGetActiveSlugsResponseItemMarketsItem {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub slug: ::std::option::Option<::std::string::String>,
    }
    impl ::std::default::Default for MarketControllerGetActiveSlugsResponseItemMarketsItem {
        fn default() -> Self {
            Self {
                slug: Default::default(),
            }
        }
    }
    ///`MarketControllerGetOracleCandlesInterval`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "default": "1m",
    ///  "type": "string",
    ///  "enum": [
    ///    "1m",
    ///    "5m",
    ///    "15m",
    ///    "1h",
    ///    "4h",
    ///    "1d"
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
        PartialOrd,
    )]
    pub enum MarketControllerGetOracleCandlesInterval {
        #[serde(rename = "1m")]
        X1m,
        #[serde(rename = "5m")]
        X5m,
        #[serde(rename = "15m")]
        X15m,
        #[serde(rename = "1h")]
        X1h,
        #[serde(rename = "4h")]
        X4h,
        #[serde(rename = "1d")]
        X1d,
    }
    impl ::std::fmt::Display for MarketControllerGetOracleCandlesInterval {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::X1m => f.write_str("1m"),
                Self::X5m => f.write_str("5m"),
                Self::X15m => f.write_str("15m"),
                Self::X1h => f.write_str("1h"),
                Self::X4h => f.write_str("4h"),
                Self::X1d => f.write_str("1d"),
            }
        }
    }
    impl ::std::str::FromStr for MarketControllerGetOracleCandlesInterval {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "1m" => Ok(Self::X1m),
                "5m" => Ok(Self::X5m),
                "15m" => Ok(Self::X15m),
                "1h" => Ok(Self::X1h),
                "4h" => Ok(Self::X4h),
                "1d" => Ok(Self::X1d),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for MarketControllerGetOracleCandlesInterval {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String> for MarketControllerGetOracleCandlesInterval {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for MarketControllerGetOracleCandlesInterval {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::default::Default for MarketControllerGetOracleCandlesInterval {
        fn default() -> Self {
            MarketControllerGetOracleCandlesInterval::X1m
        }
    }
    ///`MarketControllerGetOracleCandlesResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "interval": {
    ///      "description": "The requested candlestick interval",
    ///      "examples": [
    ///        "1h"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "rows": {
    ///      "description": "Array of OHLCV candlestick rows",
    ///      "type": "array",
    ///      "items": {
    ///        "type": "object",
    ///        "properties": {
    ///          "close": {
    ///            "description": "Closing price",
    ///            "type": "number"
    ///          },
    ///          "high": {
    ///            "description": "Highest price",
    ///            "type": "number"
    ///          },
    ///          "low": {
    ///            "description": "Lowest price",
    ///            "type": "number"
    ///          },
    ///          "open": {
    ///            "description": "Opening price",
    ///            "type": "number"
    ///          },
    ///          "timestamp": {
    ///            "description": "Candle open time (UNIX seconds)",
    ///            "type": "number"
    ///          },
    ///          "volume": {
    ///            "description": "Trading volume",
    ///            "type": "number"
    ///          }
    ///        }
    ///      }
    ///    },
    ///    "source": {
    ///      "description": "Data source identifier",
    ///      "examples": [
    ///        "chainlink"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "symbol": {
    ///      "description": "The resolved Chainlink symbol",
    ///      "examples": [
    ///        "BTC/USD"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "timestampEnd": {
    ///      "description": "End of the data range (UNIX seconds)",
    ///      "examples": [
    ///        1719878400
    ///      ],
    ///      "type": "number"
    ///    },
    ///    "timestampStart": {
    ///      "description": "Start of the data range (UNIX seconds)",
    ///      "examples": [
    ///        1719792000
    ///      ],
    ///      "type": "number"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct MarketControllerGetOracleCandlesResponse {
        ///The requested candlestick interval
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub interval: ::std::option::Option<::std::string::String>,
        ///Array of OHLCV candlestick rows
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub rows: ::std::vec::Vec<MarketControllerGetOracleCandlesResponseRowsItem>,
        ///Data source identifier
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub source: ::std::option::Option<::std::string::String>,
        ///The resolved Chainlink symbol
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub symbol: ::std::option::Option<::std::string::String>,
        ///End of the data range (UNIX seconds)
        #[serde(
            rename = "timestampEnd",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub timestamp_end: ::std::option::Option<f64>,
        ///Start of the data range (UNIX seconds)
        #[serde(
            rename = "timestampStart",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub timestamp_start: ::std::option::Option<f64>,
    }
    impl ::std::default::Default for MarketControllerGetOracleCandlesResponse {
        fn default() -> Self {
            Self {
                interval: Default::default(),
                rows: Default::default(),
                source: Default::default(),
                symbol: Default::default(),
                timestamp_end: Default::default(),
                timestamp_start: Default::default(),
            }
        }
    }
    ///`MarketControllerGetOracleCandlesResponseRowsItem`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "close": {
    ///      "description": "Closing price",
    ///      "type": "number"
    ///    },
    ///    "high": {
    ///      "description": "Highest price",
    ///      "type": "number"
    ///    },
    ///    "low": {
    ///      "description": "Lowest price",
    ///      "type": "number"
    ///    },
    ///    "open": {
    ///      "description": "Opening price",
    ///      "type": "number"
    ///    },
    ///    "timestamp": {
    ///      "description": "Candle open time (UNIX seconds)",
    ///      "type": "number"
    ///    },
    ///    "volume": {
    ///      "description": "Trading volume",
    ///      "type": "number"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct MarketControllerGetOracleCandlesResponseRowsItem {
        ///Closing price
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub close: ::std::option::Option<f64>,
        ///Highest price
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub high: ::std::option::Option<f64>,
        ///Lowest price
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub low: ::std::option::Option<f64>,
        ///Opening price
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub open: ::std::option::Option<f64>,
        ///Candle open time (UNIX seconds)
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub timestamp: ::std::option::Option<f64>,
        ///Trading volume
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub volume: ::std::option::Option<f64>,
    }
    impl ::std::default::Default for MarketControllerGetOracleCandlesResponseRowsItem {
        fn default() -> Self {
            Self {
                close: Default::default(),
                high: Default::default(),
                low: Default::default(),
                open: Default::default(),
                timestamp: Default::default(),
                volume: Default::default(),
            }
        }
    }
    ///`MarketOrderbookControllerGetHistoricalPriceInterval`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "1h",
    ///    "6h",
    ///    "1d",
    ///    "1w",
    ///    "1m",
    ///    "all"
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
        PartialOrd,
    )]
    pub enum MarketOrderbookControllerGetHistoricalPriceInterval {
        #[serde(rename = "1h")]
        X1h,
        #[serde(rename = "6h")]
        X6h,
        #[serde(rename = "1d")]
        X1d,
        #[serde(rename = "1w")]
        X1w,
        #[serde(rename = "1m")]
        X1m,
        #[serde(rename = "all")]
        All,
    }
    impl ::std::fmt::Display for MarketOrderbookControllerGetHistoricalPriceInterval {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::X1h => f.write_str("1h"),
                Self::X6h => f.write_str("6h"),
                Self::X1d => f.write_str("1d"),
                Self::X1w => f.write_str("1w"),
                Self::X1m => f.write_str("1m"),
                Self::All => f.write_str("all"),
            }
        }
    }
    impl ::std::str::FromStr for MarketOrderbookControllerGetHistoricalPriceInterval {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "1h" => Ok(Self::X1h),
                "6h" => Ok(Self::X6h),
                "1d" => Ok(Self::X1d),
                "1w" => Ok(Self::X1w),
                "1m" => Ok(Self::X1m),
                "all" => Ok(Self::All),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for MarketOrderbookControllerGetHistoricalPriceInterval {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String>
        for MarketOrderbookControllerGetHistoricalPriceInterval
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String>
        for MarketOrderbookControllerGetHistoricalPriceInterval
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///`MarketOrderbookControllerGetHistoricalPriceResponseItem`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "prices": {
    ///      "type": "array",
    ///      "items": {
    ///        "type": "object",
    ///        "properties": {
    ///          "price": {
    ///            "examples": [
    ///              0.75
    ///            ],
    ///            "type": "number"
    ///          },
    ///          "timestamp": {
    ///            "examples": [
    ///              "2024-01-15T10:30:00Z"
    ///            ],
    ///            "type": "string"
    ///          }
    ///        }
    ///      }
    ///    },
    ///    "title": {
    ///      "examples": [
    ///        "YES Token"
    ///      ],
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct MarketOrderbookControllerGetHistoricalPriceResponseItem {
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub prices:
            ::std::vec::Vec<MarketOrderbookControllerGetHistoricalPriceResponseItemPricesItem>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub title: ::std::option::Option<::std::string::String>,
    }
    impl ::std::default::Default for MarketOrderbookControllerGetHistoricalPriceResponseItem {
        fn default() -> Self {
            Self {
                prices: Default::default(),
                title: Default::default(),
            }
        }
    }
    ///`MarketOrderbookControllerGetHistoricalPriceResponseItemPricesItem`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "price": {
    ///      "examples": [
    ///        0.75
    ///      ],
    ///      "type": "number"
    ///    },
    ///    "timestamp": {
    ///      "examples": [
    ///        "2024-01-15T10:30:00Z"
    ///      ],
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct MarketOrderbookControllerGetHistoricalPriceResponseItemPricesItem {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub price: ::std::option::Option<f64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub timestamp: ::std::option::Option<::std::string::String>,
    }
    impl ::std::default::Default for MarketOrderbookControllerGetHistoricalPriceResponseItemPricesItem {
        fn default() -> Self {
            Self {
                price: Default::default(),
                timestamp: Default::default(),
            }
        }
    }
    ///`MarketOrderbookControllerGetLockedBalanceResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "currency": {
    ///      "examples": [
    ///        "USDC"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "lockedBalance": {
    ///      "examples": [
    ///        "250.50"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "lockedBalanceFormatted": {
    ///      "examples": [
    ///        "250.50 USDC"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "orderCount": {
    ///      "examples": [
    ///        3
    ///      ],
    ///      "type": "number"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct MarketOrderbookControllerGetLockedBalanceResponse {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub currency: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "lockedBalance",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub locked_balance: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "lockedBalanceFormatted",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub locked_balance_formatted: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "orderCount",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub order_count: ::std::option::Option<f64>,
    }
    impl ::std::default::Default for MarketOrderbookControllerGetLockedBalanceResponse {
        fn default() -> Self {
            Self {
                currency: Default::default(),
                locked_balance: Default::default(),
                locked_balance_formatted: Default::default(),
                order_count: Default::default(),
            }
        }
    }
    ///`MarketOrderbookControllerGetMarketEventsResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "events": {
    ///      "type": "array",
    ///      "items": {
    ///        "type": "object",
    ///        "properties": {
    ///          "data": {
    ///            "type": "object",
    ///            "additionalProperties": true
    ///          },
    ///          "id": {
    ///            "examples": [
    ///              "event-123"
    ///            ],
    ///            "type": "string"
    ///          },
    ///          "timestamp": {
    ///            "examples": [
    ///              "2024-01-15T10:30:00Z"
    ///            ],
    ///            "type": "string"
    ///          },
    ///          "type": {
    ///            "examples": [
    ///              "ORDER_PLACED"
    ///            ],
    ///            "type": "string"
    ///          }
    ///        }
    ///      }
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct MarketOrderbookControllerGetMarketEventsResponse {
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub events: ::std::vec::Vec<MarketOrderbookControllerGetMarketEventsResponseEventsItem>,
    }
    impl ::std::default::Default for MarketOrderbookControllerGetMarketEventsResponse {
        fn default() -> Self {
            Self {
                events: Default::default(),
            }
        }
    }
    ///`MarketOrderbookControllerGetMarketEventsResponseEventsItem`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "data": {
    ///      "type": "object",
    ///      "additionalProperties": true
    ///    },
    ///    "id": {
    ///      "examples": [
    ///        "event-123"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "timestamp": {
    ///      "examples": [
    ///        "2024-01-15T10:30:00Z"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "type": {
    ///      "examples": [
    ///        "ORDER_PLACED"
    ///      ],
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct MarketOrderbookControllerGetMarketEventsResponseEventsItem {
        #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
        pub data: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub id: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub timestamp: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "type",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub type_: ::std::option::Option<::std::string::String>,
    }
    impl ::std::default::Default for MarketOrderbookControllerGetMarketEventsResponseEventsItem {
        fn default() -> Self {
            Self {
                data: Default::default(),
                id: Default::default(),
                timestamp: Default::default(),
                type_: Default::default(),
            }
        }
    }
    ///`MarketOrderbookControllerGetOrderbookResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "adjustedMidpoint": {
    ///      "examples": [
    ///        0.75
    ///      ],
    ///      "type": "number"
    ///    },
    ///    "asks": {
    ///      "type": "array",
    ///      "items": {
    ///        "type": "object",
    ///        "properties": {
    ///          "price": {
    ///            "examples": [
    ///              0.76
    ///            ],
    ///            "type": "number"
    ///          },
    ///          "size": {
    ///            "examples": [
    ///              100
    ///            ],
    ///            "type": "number"
    ///          }
    ///        }
    ///      }
    ///    },
    ///    "bids": {
    ///      "type": "array",
    ///      "items": {
    ///        "type": "object",
    ///        "properties": {
    ///          "price": {
    ///            "examples": [
    ///              0.74
    ///            ],
    ///            "type": "number"
    ///          },
    ///          "size": {
    ///            "examples": [
    ///              150
    ///            ],
    ///            "type": "number"
    ///          }
    ///        }
    ///      }
    ///    },
    ///    "lastTradePrice": {
    ///      "examples": [
    ///        0.75
    ///      ],
    ///      "type": "number"
    ///    },
    ///    "maxSpread": {
    ///      "examples": [
    ///        0.05
    ///      ],
    ///      "type": "number"
    ///    },
    ///    "minSize": {
    ///      "examples": [
    ///        1
    ///      ],
    ///      "type": "number"
    ///    },
    ///    "tokenId": {
    ///      "examples": [
    ///        "19633204485790857949828516737993423758628930235371629943999544859324645414627"
    ///      ],
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct MarketOrderbookControllerGetOrderbookResponse {
        #[serde(
            rename = "adjustedMidpoint",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub adjusted_midpoint: ::std::option::Option<f64>,
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub asks: ::std::vec::Vec<MarketOrderbookControllerGetOrderbookResponseAsksItem>,
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub bids: ::std::vec::Vec<MarketOrderbookControllerGetOrderbookResponseBidsItem>,
        #[serde(
            rename = "lastTradePrice",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub last_trade_price: ::std::option::Option<f64>,
        #[serde(
            rename = "maxSpread",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub max_spread: ::std::option::Option<f64>,
        #[serde(
            rename = "minSize",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub min_size: ::std::option::Option<f64>,
        #[serde(
            rename = "tokenId",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub token_id: ::std::option::Option<::std::string::String>,
    }
    impl ::std::default::Default for MarketOrderbookControllerGetOrderbookResponse {
        fn default() -> Self {
            Self {
                adjusted_midpoint: Default::default(),
                asks: Default::default(),
                bids: Default::default(),
                last_trade_price: Default::default(),
                max_spread: Default::default(),
                min_size: Default::default(),
                token_id: Default::default(),
            }
        }
    }
    ///`MarketOrderbookControllerGetOrderbookResponseAsksItem`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "price": {
    ///      "examples": [
    ///        0.76
    ///      ],
    ///      "type": "number"
    ///    },
    ///    "size": {
    ///      "examples": [
    ///        100
    ///      ],
    ///      "type": "number"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct MarketOrderbookControllerGetOrderbookResponseAsksItem {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub price: ::std::option::Option<f64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub size: ::std::option::Option<f64>,
    }
    impl ::std::default::Default for MarketOrderbookControllerGetOrderbookResponseAsksItem {
        fn default() -> Self {
            Self {
                price: Default::default(),
                size: Default::default(),
            }
        }
    }
    ///`MarketOrderbookControllerGetOrderbookResponseBidsItem`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "price": {
    ///      "examples": [
    ///        0.74
    ///      ],
    ///      "type": "number"
    ///    },
    ///    "size": {
    ///      "examples": [
    ///        150
    ///      ],
    ///      "type": "number"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct MarketOrderbookControllerGetOrderbookResponseBidsItem {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub price: ::std::option::Option<f64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub size: ::std::option::Option<f64>,
    }
    impl ::std::default::Default for MarketOrderbookControllerGetOrderbookResponseBidsItem {
        fn default() -> Self {
            Self {
                price: Default::default(),
                size: Default::default(),
            }
        }
    }
    ///`MarketOrderbookControllerGetUserOrdersResponse`
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
    ///        "properties": {
    ///          "id": {
    ///            "examples": [
    ///              "12345"
    ///            ],
    ///            "type": "string"
    ///          },
    ///          "price": {
    ///            "examples": [
    ///              "0.75"
    ///            ],
    ///            "type": "string"
    ///          },
    ///          "quantity": {
    ///            "examples": [
    ///              "100"
    ///            ],
    ///            "type": "string"
    ///          },
    ///          "side": {
    ///            "examples": [
    ///              "BUY"
    ///            ],
    ///            "type": "string"
    ///          },
    ///          "status": {
    ///            "examples": [
    ///              "LIVE"
    ///            ],
    ///            "type": "string"
    ///          }
    ///        }
    ///      }
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct MarketOrderbookControllerGetUserOrdersResponse {
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub orders: ::std::vec::Vec<MarketOrderbookControllerGetUserOrdersResponseOrdersItem>,
    }
    impl ::std::default::Default for MarketOrderbookControllerGetUserOrdersResponse {
        fn default() -> Self {
            Self {
                orders: Default::default(),
            }
        }
    }
    ///`MarketOrderbookControllerGetUserOrdersResponseOrdersItem`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "id": {
    ///      "examples": [
    ///        "12345"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "price": {
    ///      "examples": [
    ///        "0.75"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "quantity": {
    ///      "examples": [
    ///        "100"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "side": {
    ///      "examples": [
    ///        "BUY"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "status": {
    ///      "examples": [
    ///        "LIVE"
    ///      ],
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct MarketOrderbookControllerGetUserOrdersResponseOrdersItem {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub id: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub price: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub quantity: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub side: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub status: ::std::option::Option<::std::string::String>,
    }
    impl ::std::default::Default for MarketOrderbookControllerGetUserOrdersResponseOrdersItem {
        fn default() -> Self {
            Self {
                id: Default::default(),
                price: Default::default(),
                quantity: Default::default(),
                side: Default::default(),
                status: Default::default(),
            }
        }
    }
    ///`MarketOrderbookControllerGetUserOrdersStatusesItem`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "LIVE",
    ///    "MATCHED"
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
        PartialOrd,
    )]
    pub enum MarketOrderbookControllerGetUserOrdersStatusesItem {
        #[serde(rename = "LIVE")]
        Live,
        #[serde(rename = "MATCHED")]
        Matched,
    }
    impl ::std::fmt::Display for MarketOrderbookControllerGetUserOrdersStatusesItem {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Live => f.write_str("LIVE"),
                Self::Matched => f.write_str("MATCHED"),
            }
        }
    }
    impl ::std::str::FromStr for MarketOrderbookControllerGetUserOrdersStatusesItem {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "LIVE" => Ok(Self::Live),
                "MATCHED" => Ok(Self::Matched),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for MarketOrderbookControllerGetUserOrdersStatusesItem {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String>
        for MarketOrderbookControllerGetUserOrdersStatusesItem
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String>
        for MarketOrderbookControllerGetUserOrdersStatusesItem
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///`MarketPageByPathResponseDto`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "baseFilter",
    ///    "breadcrumb",
    ///    "filterGroups",
    ///    "fullPath",
    ///    "id",
    ///    "metadata",
    ///    "name",
    ///    "slug"
    ///  ],
    ///  "properties": {
    ///    "baseFilter": {
    ///      "description": "Base filter configuration for the page",
    ///      "type": "object"
    ///    },
    ///    "breadcrumb": {
    ///      "description": "Breadcrumb trail to this page",
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/BreadcrumbItemDto"
    ///      }
    ///    },
    ///    "description": {
    ///      "description": "Description of the market page",
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "filterGroups": {
    ///      "description": "Available filter groups for the page",
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/FilterGroupDto"
    ///      }
    ///    },
    ///    "fullPath": {
    ///      "description": "Full URL path to this market page",
    ///      "type": "string"
    ///    },
    ///    "id": {
    ///      "description": "Unique identifier of the market page",
    ///      "type": "string"
    ///    },
    ///    "metadata": {
    ///      "description": "Additional metadata for the page",
    ///      "type": "object"
    ///    },
    ///    "name": {
    ///      "description": "Display name of the market page",
    ///      "type": "string"
    ///    },
    ///    "slug": {
    ///      "description": "URL slug of the market page",
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct MarketPageByPathResponseDto {
        ///Base filter configuration for the page
        #[serde(rename = "baseFilter")]
        pub base_filter: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        ///Breadcrumb trail to this page
        pub breadcrumb: ::std::vec::Vec<BreadcrumbItemDto>,
        ///Description of the market page
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub description: ::std::option::Option<::std::string::String>,
        ///Available filter groups for the page
        #[serde(rename = "filterGroups")]
        pub filter_groups: ::std::vec::Vec<FilterGroupDto>,
        ///Full URL path to this market page
        #[serde(rename = "fullPath")]
        pub full_path: ::std::string::String,
        ///Unique identifier of the market page
        pub id: ::std::string::String,
        ///Additional metadata for the page
        pub metadata: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        ///Display name of the market page
        pub name: ::std::string::String,
        ///URL slug of the market page
        pub slug: ::std::string::String,
    }
    ///`MarketPositionDataDto`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "no",
    ///    "yes"
    ///  ],
    ///  "properties": {
    ///    "no": {
    ///      "description": "Position data for NO outcome",
    ///      "examples": [
    ///        {
    ///          "cost": "25000000",
    ///          "fillPrice": "250000",
    ///          "marketValue": "20000000",
    ///          "realisedPnl": "0",
    ///          "unrealizedPnl": "-5000000"
    ///        }
    ///      ],
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/PositionDataDto"
    ///        }
    ///      ]
    ///    },
    ///    "yes": {
    ///      "description": "Position data for YES outcome",
    ///      "examples": [
    ///        {
    ///          "cost": "75000000",
    ///          "fillPrice": "750000",
    ///          "marketValue": "100000000",
    ///          "realisedPnl": "0",
    ///          "unrealizedPnl": "25000000"
    ///        }
    ///      ],
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/PositionDataDto"
    ///        }
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct MarketPositionDataDto {
        ///Position data for NO outcome
        pub no: PositionDataDto,
        ///Position data for YES outcome
        pub yes: PositionDataDto,
    }
    ///`MarketProxyTitle`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "maxLength": 70,
    ///  "uniqueItems": true
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct MarketProxyTitle(::std::string::String);
    impl ::std::ops::Deref for MarketProxyTitle {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }
    impl ::std::convert::From<MarketProxyTitle> for ::std::string::String {
        fn from(value: MarketProxyTitle) -> Self {
            value.0
        }
    }
    impl ::std::str::FromStr for MarketProxyTitle {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            if value.chars().count() > 70usize {
                return Err("longer than 70 characters".into());
            }
            Ok(Self(value.to_string()))
        }
    }
    impl ::std::convert::TryFrom<&str> for MarketProxyTitle {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String> for MarketProxyTitle {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for MarketProxyTitle {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl<'de> ::serde::Deserialize<'de> for MarketProxyTitle {
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
    ///Represents the question of the market, the title metadata of the market.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Represents the question of the market, the title metadata of the market.",
    ///  "type": "string",
    ///  "maxLength": 70,
    ///  "uniqueItems": true
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct MarketTitle(::std::string::String);
    impl ::std::ops::Deref for MarketTitle {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }
    impl ::std::convert::From<MarketTitle> for ::std::string::String {
        fn from(value: MarketTitle) -> Self {
            value.0
        }
    }
    impl ::std::str::FromStr for MarketTitle {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            if value.chars().count() > 70usize {
                return Err("longer than 70 characters".into());
            }
            Ok(Self(value.to_string()))
        }
    }
    impl ::std::convert::TryFrom<&str> for MarketTitle {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String> for MarketTitle {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for MarketTitle {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl<'de> ::serde::Deserialize<'de> for MarketTitle {
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
    ///`NavigationNodeDto`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "children",
    ///    "id",
    ///    "name",
    ///    "path",
    ///    "slug"
    ///  ],
    ///  "properties": {
    ///    "children": {
    ///      "description": "Child navigation nodes",
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/NavigationNodeDto"
    ///      }
    ///    },
    ///    "icon": {
    ///      "description": "Icon identifier for the navigation item",
    ///      "type": "string"
    ///    },
    ///    "id": {
    ///      "description": "Unique identifier for the navigation node",
    ///      "type": "string"
    ///    },
    ///    "name": {
    ///      "description": "Display name of the navigation item",
    ///      "type": "string"
    ///    },
    ///    "path": {
    ///      "description": "Full path to this navigation item",
    ///      "examples": [
    ///        "/sports/football"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "slug": {
    ///      "description": "URL slug for the navigation item",
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct NavigationNodeDto {
        ///Child navigation nodes
        pub children: ::std::vec::Vec<NavigationNodeDto>,
        ///Icon identifier for the navigation item
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub icon: ::std::option::Option<::std::string::String>,
        ///Unique identifier for the navigation node
        pub id: ::std::string::String,
        ///Display name of the navigation item
        pub name: ::std::string::String,
        ///Full path to this navigation item
        pub path: ::std::string::String,
        ///URL slug for the navigation item
        pub slug: ::std::string::String,
    }
    ///`NegRiskGroupResponseDto`
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
    pub struct NegRiskGroupResponseDto(
        pub ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    );
    impl ::std::ops::Deref for NegRiskGroupResponseDto {
        type Target = ::serde_json::Map<::std::string::String, ::serde_json::Value>;
        fn deref(&self) -> &::serde_json::Map<::std::string::String, ::serde_json::Value> {
            &self.0
        }
    }
    impl ::std::convert::From<NegRiskGroupResponseDto>
        for ::serde_json::Map<::std::string::String, ::serde_json::Value>
    {
        fn from(value: NegRiskGroupResponseDto) -> Self {
            value.0
        }
    }
    impl ::std::convert::From<::serde_json::Map<::std::string::String, ::serde_json::Value>>
        for NegRiskGroupResponseDto
    {
        fn from(value: ::serde_json::Map<::std::string::String, ::serde_json::Value>) -> Self {
            Self(value)
        }
    }
    ///`OffsetPaginationDto`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "limit",
    ///    "page",
    ///    "total",
    ///    "totalPages"
    ///  ],
    ///  "properties": {
    ///    "limit": {
    ///      "description": "Number of items per page",
    ///      "type": "number"
    ///    },
    ///    "page": {
    ///      "description": "Current page number",
    ///      "type": "number"
    ///    },
    ///    "total": {
    ///      "description": "Total number of items",
    ///      "type": "number"
    ///    },
    ///    "totalPages": {
    ///      "description": "Total number of pages",
    ///      "type": "number"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct OffsetPaginationDto {
        ///Number of items per page
        pub limit: f64,
        ///Current page number
        pub page: f64,
        ///Total number of items
        pub total: f64,
        ///Total number of pages
        #[serde(rename = "totalPages")]
        pub total_pages: f64,
    }
    ///`Order`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "feeRateBps",
    ///    "maker",
    ///    "makerAmount",
    ///    "salt",
    ///    "side",
    ///    "signer",
    ///    "takerAmount",
    ///    "tokenId"
    ///  ],
    ///  "properties": {
    ///    "expiration": {
    ///      "description": "Order expiration timestamp",
    ///      "examples": [
    ///        "2025-04-30T23:59:59Z"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "feeRateBps": {
    ///      "description": "Fee rate in basis points (1% = 100)",
    ///      "examples": [
    ///        0
    ///      ],
    ///      "type": "number"
    ///    },
    ///    "maker": {
    ///      "description": "Ethereum address of the maker (order creator)",
    ///      "examples": [
    ///        "0x742d35Cc6634C0532925a3b844Bc454e4438f44e"
    ///      ],
    ///      "type": "string",
    ///      "pattern": "^0x[a-fA-F0-9]{40}$"
    ///    },
    ///    "makerAmount": {
    ///      "description": "Amount the maker is offering, scaled by 1e6. For GTC orders: price * size * 1e6 (BUY) or size * 1e6 (SELL). For FOK orders: USDC to spend * 1e6 (BUY) or shares to sell * 1e6 (SELL).",
    ///      "examples": [
    ///        5000000
    ///      ],
    ///      "type": "number",
    ///      "minimum": 0.0
    ///    },
    ///    "nonce": {
    ///      "description": "Order nonce for cancellation tracking",
    ///      "examples": [
    ///        42
    ///      ],
    ///      "type": "number"
    ///    },
    ///    "price": {
    ///      "description": "Order price as decimal (0.01-0.99, required for GTC orders)",
    ///      "examples": [
    ///        0.75
    ///      ],
    ///      "type": "number",
    ///      "maximum": 0.99,
    ///      "minimum": 0.01
    ///    },
    ///    "salt": {
    ///      "description": "Unique random value for signature uniqueness (prevents replay attacks)",
    ///      "examples": [
    ///        1234567890
    ///      ],
    ///      "type": "number"
    ///    },
    ///    "side": {
    ///      "description": "Order side: 0 = BUY, 1 = SELL",
    ///      "examples": [
    ///        0
    ///      ],
    ///      "type": "number",
    ///      "enum": [
    ///        0.0,
    ///        1.0
    ///      ]
    ///    },
    ///    "signature": {
    ///      "description": "EIP-712 signature of order details. Optional when using delegated signing.",
    ///      "examples": [
    ///        "0x123abc456def789ghi0123abc456def789ghi0123abc456def789ghi0123456789012345678901"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "signatureType": {
    ///      "description": "Signature type (0-3). Optional when using delegated signing.",
    ///      "examples": [
    ///        2
    ///      ],
    ///      "type": "number",
    ///      "enum": [
    ///        0.0,
    ///        1.0,
    ///        2.0,
    ///        3.0
    ///      ]
    ///    },
    ///    "signer": {
    ///      "description": "Address that signed the order",
    ///      "examples": [
    ///        "0x742d35Cc6634C0532925a3b844Bc454e4438f44e"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "taker": {
    ///      "description": "Specific taker address (optional for open orders)",
    ///      "examples": [
    ///        "0x0000000000000000000000000000000000000000"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "takerAmount": {
    ///      "description": "Amount the maker wants in return, scaled by 1e6. For GTC orders: size * 1e6 (BUY) or price * size * 1e6 (SELL). For FOK orders: always 1.",
    ///      "examples": [
    ///        10000000
    ///      ],
    ///      "type": "number",
    ///      "minimum": 0.0
    ///    },
    ///    "tokenId": {
    ///      "description": "Token ID being traded (YES or NO position ID from conditional token)",
    ///      "examples": [
    ///        "19633204485790857949828516737993423758628930235371629943999544859324645414627"
    ///      ],
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct Order {
        ///Order expiration timestamp
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub expiration: ::std::option::Option<::std::string::String>,
        ///Fee rate in basis points (1% = 100)
        #[serde(rename = "feeRateBps")]
        pub fee_rate_bps: f64,
        ///Ethereum address of the maker (order creator)
        pub maker: OrderMaker,
        ///Amount the maker is offering, scaled by 1e6. For GTC orders: price * size * 1e6 (BUY) or size * 1e6 (SELL). For FOK orders: USDC to spend * 1e6 (BUY) or shares to sell * 1e6 (SELL).
        #[serde(rename = "makerAmount")]
        pub maker_amount: f64,
        ///Order nonce for cancellation tracking
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub nonce: ::std::option::Option<f64>,
        ///Order price as decimal (0.01-0.99, required for GTC orders)
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub price: ::std::option::Option<f64>,
        ///Unique random value for signature uniqueness (prevents replay attacks)
        pub salt: f64,
        ///Order side: 0 = BUY, 1 = SELL
        pub side: OrderSide,
        ///EIP-712 signature of order details. Optional when using delegated signing.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub signature: ::std::option::Option<::std::string::String>,
        ///Signature type (0-3). Optional when using delegated signing.
        #[serde(
            rename = "signatureType",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub signature_type: ::std::option::Option<OrderSignatureType>,
        ///Address that signed the order
        pub signer: ::std::string::String,
        ///Specific taker address (optional for open orders)
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub taker: ::std::option::Option<::std::string::String>,
        ///Amount the maker wants in return, scaled by 1e6. For GTC orders: size * 1e6 (BUY) or price * size * 1e6 (SELL). For FOK orders: always 1.
        #[serde(rename = "takerAmount")]
        pub taker_amount: f64,
        ///Token ID being traded (YES or NO position ID from conditional token)
        #[serde(rename = "tokenId")]
        pub token_id: ::std::string::String,
    }
    ///`OrderControllerBatchCancelOrdersResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "oneOf": [
    ///    {
    ///      "$ref": "#/components/schemas/CancelOrderBatchResponseDto"
    ///    },
    ///    {
    ///      "$ref": "#/components/schemas/CancelOrderBatchCombinedResponseDto"
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    #[serde(untagged)]
    pub enum OrderControllerBatchCancelOrdersResponse {
        ResponseDto(CancelOrderBatchResponseDto),
        CombinedResponseDto(CancelOrderBatchCombinedResponseDto),
    }
    impl ::std::convert::From<CancelOrderBatchResponseDto>
        for OrderControllerBatchCancelOrdersResponse
    {
        fn from(value: CancelOrderBatchResponseDto) -> Self {
            Self::ResponseDto(value)
        }
    }
    impl ::std::convert::From<CancelOrderBatchCombinedResponseDto>
        for OrderControllerBatchCancelOrdersResponse
    {
        fn from(value: CancelOrderBatchCombinedResponseDto) -> Self {
            Self::CombinedResponseDto(value)
        }
    }
    ///Execution and settlement summary for a created order
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Execution and settlement summary for a created order",
    ///  "type": "object",
    ///  "required": [
    ///    "effectiveFeeBps",
    ///    "feeRateBps",
    ///    "matched",
    ///    "settlementStatus",
    ///    "totalsRaw"
    ///  ],
    ///  "properties": {
    ///    "clientOrderId": {
    ///      "description": "Echo of client-provided idempotency key",
    ///      "examples": [
    ///        "client-order-001"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "effectiveFeeBps": {
    ///      "description": "Effective fee rate in basis points after any rebates",
    ///      "examples": [
    ///        26
    ///      ],
    ///      "type": "number"
    ///    },
    ///    "feeRateBps": {
    ///      "description": "Fee rate in basis points applied to this order",
    ///      "examples": [
    ///        25
    ///      ],
    ///      "type": "number"
    ///    },
    ///    "matched": {
    ///      "description": "Whether the order was matched immediately",
    ///      "examples": [
    ///        true
    ///      ],
    ///      "type": "boolean"
    ///    },
    ///    "settlementStatus": {
    ///      "description": "Current settlement status of the order",
    ///      "examples": [
    ///        "MINED"
    ///      ],
    ///      "type": "string",
    ///      "enum": [
    ///        "UNMATCHED",
    ///        "MATCHED",
    ///        "MINED",
    ///        "CONFIRMED",
    ///        "RETRYING",
    ///        "FAILED"
    ///      ]
    ///    },
    ///    "totalsRaw": {
    ///      "$ref": "#/components/schemas/OrderExecutionTotalsRawDto"
    ///    },
    ///    "tradeEventId": {
    ///      "description": "Trade event ID (present when matched)",
    ///      "examples": [
    ///        "4aa706dd-6c57-4f3c-945a-99818dfd95f1"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "txHash": {
    ///      "description": "On-chain transaction hash (present when mined)",
    ///      "examples": [
    ///        "0xabc123"
    ///      ],
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
    pub struct OrderExecutionDto {
        ///Echo of client-provided idempotency key
        #[serde(
            rename = "clientOrderId",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub client_order_id: ::std::option::Option<::std::string::String>,
        ///Effective fee rate in basis points after any rebates
        #[serde(rename = "effectiveFeeBps")]
        pub effective_fee_bps: f64,
        ///Fee rate in basis points applied to this order
        #[serde(rename = "feeRateBps")]
        pub fee_rate_bps: f64,
        ///Whether the order was matched immediately
        pub matched: bool,
        ///Current settlement status of the order
        #[serde(rename = "settlementStatus")]
        pub settlement_status: OrderExecutionDtoSettlementStatus,
        #[serde(rename = "totalsRaw")]
        pub totals_raw: OrderExecutionTotalsRawDto,
        ///Trade event ID (present when matched)
        #[serde(
            rename = "tradeEventId",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub trade_event_id: ::std::option::Option<::std::string::String>,
        ///On-chain transaction hash (present when mined)
        #[serde(
            rename = "txHash",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub tx_hash: ::std::option::Option<::std::string::String>,
    }
    ///Current settlement status of the order
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Current settlement status of the order",
    ///  "examples": [
    ///    "MINED"
    ///  ],
    ///  "type": "string",
    ///  "enum": [
    ///    "UNMATCHED",
    ///    "MATCHED",
    ///    "MINED",
    ///    "CONFIRMED",
    ///    "RETRYING",
    ///    "FAILED"
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
        PartialOrd,
    )]
    pub enum OrderExecutionDtoSettlementStatus {
        #[serde(rename = "UNMATCHED")]
        Unmatched,
        #[serde(rename = "MATCHED")]
        Matched,
        #[serde(rename = "MINED")]
        Mined,
        #[serde(rename = "CONFIRMED")]
        Confirmed,
        #[serde(rename = "RETRYING")]
        Retrying,
        #[serde(rename = "FAILED")]
        Failed,
    }
    impl ::std::fmt::Display for OrderExecutionDtoSettlementStatus {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Unmatched => f.write_str("UNMATCHED"),
                Self::Matched => f.write_str("MATCHED"),
                Self::Mined => f.write_str("MINED"),
                Self::Confirmed => f.write_str("CONFIRMED"),
                Self::Retrying => f.write_str("RETRYING"),
                Self::Failed => f.write_str("FAILED"),
            }
        }
    }
    impl ::std::str::FromStr for OrderExecutionDtoSettlementStatus {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "UNMATCHED" => Ok(Self::Unmatched),
                "MATCHED" => Ok(Self::Matched),
                "MINED" => Ok(Self::Mined),
                "CONFIRMED" => Ok(Self::Confirmed),
                "RETRYING" => Ok(Self::Retrying),
                "FAILED" => Ok(Self::Failed),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for OrderExecutionDtoSettlementStatus {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String> for OrderExecutionDtoSettlementStatus {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for OrderExecutionDtoSettlementStatus {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///`OrderExecutionSummary`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "clientOrderId": {
    ///      "description": "Client-provided order ID if one was set",
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "effectiveFeeBps": {
    ///      "description": "Effective fee rate after rebates",
    ///      "examples": [
    ///        0
    ///      ],
    ///      "type": "number"
    ///    },
    ///    "feeRateBps": {
    ///      "description": "Fee rate in basis points",
    ///      "examples": [
    ///        0
    ///      ],
    ///      "type": "number"
    ///    },
    ///    "matched": {
    ///      "description": "Whether the order was matched with a counterparty",
    ///      "type": "boolean"
    ///    },
    ///    "settlementStatus": {
    ///      "description": "On-chain settlement status",
    ///      "type": "string",
    ///      "enum": [
    ///        "UNMATCHED",
    ///        "PENDING",
    ///        "MINED",
    ///        "CONFIRMED",
    ///        "FAILED"
    ///      ]
    ///    },
    ///    "totalsRaw": {
    ///      "description": "Raw execution totals as strings (to preserve precision)",
    ///      "type": "object",
    ///      "properties": {
    ///        "contractsFee": {
    ///          "type": "string"
    ///        },
    ///        "contractsGross": {
    ///          "type": "string"
    ///        },
    ///        "contractsNet": {
    ///          "type": "string"
    ///        },
    ///        "usdFee": {
    ///          "type": "string"
    ///        },
    ///        "usdGross": {
    ///          "type": "string"
    ///        },
    ///        "usdNet": {
    ///          "type": "string"
    ///        }
    ///      }
    ///    },
    ///    "tradeEventId": {
    ///      "description": "Trade event ID if matched",
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "txHash": {
    ///      "description": "Transaction hash on Base L2",
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
    pub struct OrderExecutionSummary {
        ///Client-provided order ID if one was set
        #[serde(
            rename = "clientOrderId",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub client_order_id: ::std::option::Option<::std::string::String>,
        ///Effective fee rate after rebates
        #[serde(
            rename = "effectiveFeeBps",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub effective_fee_bps: ::std::option::Option<f64>,
        ///Fee rate in basis points
        #[serde(
            rename = "feeRateBps",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub fee_rate_bps: ::std::option::Option<f64>,
        ///Whether the order was matched with a counterparty
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub matched: ::std::option::Option<bool>,
        ///On-chain settlement status
        #[serde(
            rename = "settlementStatus",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub settlement_status: ::std::option::Option<OrderExecutionSummarySettlementStatus>,
        #[serde(
            rename = "totalsRaw",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub totals_raw: ::std::option::Option<OrderExecutionSummaryTotalsRaw>,
        ///Trade event ID if matched
        #[serde(
            rename = "tradeEventId",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub trade_event_id: ::std::option::Option<::std::string::String>,
        ///Transaction hash on Base L2
        #[serde(
            rename = "txHash",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub tx_hash: ::std::option::Option<::std::string::String>,
    }
    impl ::std::default::Default for OrderExecutionSummary {
        fn default() -> Self {
            Self {
                client_order_id: Default::default(),
                effective_fee_bps: Default::default(),
                fee_rate_bps: Default::default(),
                matched: Default::default(),
                settlement_status: Default::default(),
                totals_raw: Default::default(),
                trade_event_id: Default::default(),
                tx_hash: Default::default(),
            }
        }
    }
    ///On-chain settlement status
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "On-chain settlement status",
    ///  "type": "string",
    ///  "enum": [
    ///    "UNMATCHED",
    ///    "PENDING",
    ///    "MINED",
    ///    "CONFIRMED",
    ///    "FAILED"
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
        PartialOrd,
    )]
    pub enum OrderExecutionSummarySettlementStatus {
        #[serde(rename = "UNMATCHED")]
        Unmatched,
        #[serde(rename = "PENDING")]
        Pending,
        #[serde(rename = "MINED")]
        Mined,
        #[serde(rename = "CONFIRMED")]
        Confirmed,
        #[serde(rename = "FAILED")]
        Failed,
    }
    impl ::std::fmt::Display for OrderExecutionSummarySettlementStatus {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Unmatched => f.write_str("UNMATCHED"),
                Self::Pending => f.write_str("PENDING"),
                Self::Mined => f.write_str("MINED"),
                Self::Confirmed => f.write_str("CONFIRMED"),
                Self::Failed => f.write_str("FAILED"),
            }
        }
    }
    impl ::std::str::FromStr for OrderExecutionSummarySettlementStatus {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "UNMATCHED" => Ok(Self::Unmatched),
                "PENDING" => Ok(Self::Pending),
                "MINED" => Ok(Self::Mined),
                "CONFIRMED" => Ok(Self::Confirmed),
                "FAILED" => Ok(Self::Failed),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for OrderExecutionSummarySettlementStatus {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String> for OrderExecutionSummarySettlementStatus {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for OrderExecutionSummarySettlementStatus {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///Raw execution totals as strings (to preserve precision)
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Raw execution totals as strings (to preserve precision)",
    ///  "type": "object",
    ///  "properties": {
    ///    "contractsFee": {
    ///      "type": "string"
    ///    },
    ///    "contractsGross": {
    ///      "type": "string"
    ///    },
    ///    "contractsNet": {
    ///      "type": "string"
    ///    },
    ///    "usdFee": {
    ///      "type": "string"
    ///    },
    ///    "usdGross": {
    ///      "type": "string"
    ///    },
    ///    "usdNet": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct OrderExecutionSummaryTotalsRaw {
        #[serde(
            rename = "contractsFee",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub contracts_fee: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "contractsGross",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub contracts_gross: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "contractsNet",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub contracts_net: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "usdFee",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub usd_fee: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "usdGross",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub usd_gross: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "usdNet",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub usd_net: ::std::option::Option<::std::string::String>,
    }
    impl ::std::default::Default for OrderExecutionSummaryTotalsRaw {
        fn default() -> Self {
            Self {
                contracts_fee: Default::default(),
                contracts_gross: Default::default(),
                contracts_net: Default::default(),
                usd_fee: Default::default(),
                usd_gross: Default::default(),
                usd_net: Default::default(),
            }
        }
    }
    ///Raw execution totals in contract units (strings to preserve precision)
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Raw execution totals in contract units (strings to preserve precision)",
    ///  "type": "object",
    ///  "required": [
    ///    "contractsFee",
    ///    "contractsGross",
    ///    "contractsNet",
    ///    "usdFee",
    ///    "usdGross",
    ///    "usdNet"
    ///  ],
    ///  "properties": {
    ///    "contractsFee": {
    ///      "examples": [
    ///        "1000"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "contractsGross": {
    ///      "examples": [
    ///        "1000000"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "contractsNet": {
    ///      "examples": [
    ///        "999000"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "usdFee": {
    ///      "examples": [
    ///        "500"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "usdGross": {
    ///      "examples": [
    ///        "500000"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "usdNet": {
    ///      "examples": [
    ///        "499500"
    ///      ],
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct OrderExecutionTotalsRawDto {
        #[serde(rename = "contractsFee")]
        pub contracts_fee: ::std::string::String,
        #[serde(rename = "contractsGross")]
        pub contracts_gross: ::std::string::String,
        #[serde(rename = "contractsNet")]
        pub contracts_net: ::std::string::String,
        #[serde(rename = "usdFee")]
        pub usd_fee: ::std::string::String,
        #[serde(rename = "usdGross")]
        pub usd_gross: ::std::string::String,
        #[serde(rename = "usdNet")]
        pub usd_net: ::std::string::String,
    }
    ///Ethereum address of the maker (order creator)
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Ethereum address of the maker (order creator)",
    ///  "examples": [
    ///    "0x742d35Cc6634C0532925a3b844Bc454e4438f44e"
    ///  ],
    ///  "type": "string",
    ///  "pattern": "^0x[a-fA-F0-9]{40}$"
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct OrderMaker(::std::string::String);
    impl ::std::ops::Deref for OrderMaker {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }
    impl ::std::convert::From<OrderMaker> for ::std::string::String {
        fn from(value: OrderMaker) -> Self {
            value.0
        }
    }
    impl ::std::str::FromStr for OrderMaker {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
                ::std::sync::LazyLock::new(|| {
                    ::regress::Regex::new("^0x[a-fA-F0-9]{40}$").unwrap()
                });
            if PATTERN.find(value).is_none() {
                return Err("doesn't match pattern \"^0x[a-fA-F0-9]{40}$\"".into());
            }
            Ok(Self(value.to_string()))
        }
    }
    impl ::std::convert::TryFrom<&str> for OrderMaker {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String> for OrderMaker {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for OrderMaker {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl<'de> ::serde::Deserialize<'de> for OrderMaker {
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
    ///`OrderResponseDto`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "execution",
    ///    "order"
    ///  ],
    ///  "properties": {
    ///    "execution": {
    ///      "$ref": "#/components/schemas/OrderExecutionDto"
    ///    },
    ///    "makerMatches": {
    ///      "description": "Maker matches if order was matched immediately",
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/MakerMatch"
    ///      }
    ///    },
    ///    "order": {
    ///      "description": "Order details including slim market and owner",
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/Order"
    ///        }
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct OrderResponseDto {
        pub execution: OrderExecutionDto,
        ///Maker matches if order was matched immediately
        #[serde(
            rename = "makerMatches",
            default,
            skip_serializing_if = "::std::vec::Vec::is_empty"
        )]
        pub maker_matches: ::std::vec::Vec<MakerMatch>,
        ///Order details including slim market and owner
        pub order: Order,
    }
    ///Order side: 0 = BUY, 1 = SELL
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Order side: 0 = BUY, 1 = SELL",
    ///  "examples": [
    ///    0
    ///  ],
    ///  "type": "number",
    ///  "enum": [
    ///    0.0,
    ///    1.0
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Serialize, Clone, Debug)]
    #[serde(transparent)]
    pub struct OrderSide(f64);
    impl ::std::ops::Deref for OrderSide {
        type Target = f64;
        fn deref(&self) -> &f64 {
            &self.0
        }
    }
    impl ::std::convert::From<OrderSide> for f64 {
        fn from(value: OrderSide) -> Self {
            value.0
        }
    }
    impl ::std::convert::TryFrom<f64> for OrderSide {
        type Error = self::error::ConversionError;
        fn try_from(value: f64) -> ::std::result::Result<Self, self::error::ConversionError> {
            if ![0.0_f64, 1.0_f64].contains(&value) {
                Err("invalid value".into())
            } else {
                Ok(Self(value))
            }
        }
    }
    impl<'de> ::serde::Deserialize<'de> for OrderSide {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            Self::try_from(<f64>::deserialize(deserializer)?)
                .map_err(|e| <D::Error as ::serde::de::Error>::custom(e.to_string()))
        }
    }
    ///Signature type (0-3). Optional when using delegated signing.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Signature type (0-3). Optional when using delegated signing.",
    ///  "examples": [
    ///    2
    ///  ],
    ///  "type": "number",
    ///  "enum": [
    ///    0.0,
    ///    1.0,
    ///    2.0,
    ///    3.0
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Serialize, Clone, Debug)]
    #[serde(transparent)]
    pub struct OrderSignatureType(f64);
    impl ::std::ops::Deref for OrderSignatureType {
        type Target = f64;
        fn deref(&self) -> &f64 {
            &self.0
        }
    }
    impl ::std::convert::From<OrderSignatureType> for f64 {
        fn from(value: OrderSignatureType) -> Self {
            value.0
        }
    }
    impl ::std::convert::TryFrom<f64> for OrderSignatureType {
        type Error = self::error::ConversionError;
        fn try_from(value: f64) -> ::std::result::Result<Self, self::error::ConversionError> {
            if ![0.0_f64, 1.0_f64, 2.0_f64, 3.0_f64].contains(&value) {
                Err("invalid value".into())
            } else {
                Ok(Self(value))
            }
        }
    }
    impl<'de> ::serde::Deserialize<'de> for OrderSignatureType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            Self::try_from(<f64>::deserialize(deserializer)?)
                .map_err(|e| <D::Error as ::serde::de::Error>::custom(e.to_string()))
        }
    }
    ///`PartnerAccountAllowanceResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "chainId",
    ///    "partnerProfileId",
    ///    "profileId",
    ///    "ready",
    ///    "summary",
    ///    "targets",
    ///    "walletAddress"
    ///  ],
    ///  "properties": {
    ///    "chainId": {
    ///      "description": "Chain where allowance targets were checked",
    ///      "examples": [
    ///        84532
    ///      ],
    ///      "type": "integer"
    ///    },
    ///    "partnerProfileId": {
    ///      "description": "Authenticated parent partner profile ID",
    ///      "examples": [
    ///        4430
    ///      ],
    ///      "type": "integer"
    ///    },
    ///    "profileId": {
    ///      "description": "Child/server-wallet profile ID being checked",
    ///      "examples": [
    ///        4543
    ///      ],
    ///      "type": "integer"
    ///    },
    ///    "ready": {
    ///      "description": "True when every target is confirmed on chain",
    ///      "examples": [
    ///        false
    ///      ],
    ///      "type": "boolean"
    ///    },
    ///    "summary": {
    ///      "$ref": "#/components/schemas/PartnerAccountAllowanceSummary"
    ///    },
    ///    "targets": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/PartnerAccountAllowanceTarget"
    ///      }
    ///    },
    ///    "walletAddress": {
    ///      "description": "Managed server-wallet address for the child profile",
    ///      "examples": [
    ///        "0x1a665817f063Ee15C6C2c05D4315982145825C3D"
    ///      ],
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct PartnerAccountAllowanceResponse {
        ///Chain where allowance targets were checked
        #[serde(rename = "chainId")]
        pub chain_id: i64,
        ///Authenticated parent partner profile ID
        #[serde(rename = "partnerProfileId")]
        pub partner_profile_id: i64,
        ///Child/server-wallet profile ID being checked
        #[serde(rename = "profileId")]
        pub profile_id: i64,
        ///True when every target is confirmed on chain
        pub ready: bool,
        pub summary: PartnerAccountAllowanceSummary,
        pub targets: ::std::vec::Vec<PartnerAccountAllowanceTarget>,
        ///Managed server-wallet address for the child profile
        #[serde(rename = "walletAddress")]
        pub wallet_address: ::std::string::String,
    }
    ///`PartnerAccountAllowanceSummary`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "confirmed",
    ///    "failed",
    ///    "missing",
    ///    "submitted",
    ///    "total"
    ///  ],
    ///  "properties": {
    ///    "confirmed": {
    ///      "description": "Targets confirmed on chain",
    ///      "examples": [
    ///        2
    ///      ],
    ///      "type": "integer"
    ///    },
    ///    "failed": {
    ///      "description": "Targets whose latest live read or retry state failed",
    ///      "examples": [
    ///        0
    ///      ],
    ///      "type": "integer"
    ///    },
    ///    "missing": {
    ///      "description": "Targets still missing",
    ///      "examples": [
    ///        0
    ///      ],
    ///      "type": "integer"
    ///    },
    ///    "submitted": {
    ///      "description": "Targets submitted by the current retry response",
    ///      "examples": [
    ///        8
    ///      ],
    ///      "type": "integer"
    ///    },
    ///    "total": {
    ///      "description": "Total allowance target count",
    ///      "examples": [
    ///        10
    ///      ],
    ///      "type": "integer"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct PartnerAccountAllowanceSummary {
        ///Targets confirmed on chain
        pub confirmed: i64,
        ///Targets whose latest live read or retry state failed
        pub failed: i64,
        ///Targets still missing
        pub missing: i64,
        ///Targets submitted by the current retry response
        pub submitted: i64,
        ///Total allowance target count
        pub total: i64,
    }
    ///`PartnerAccountAllowanceTarget`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "confirmed",
    ///    "label",
    ///    "requiredFor",
    ///    "retryable",
    ///    "spenderOrOperator",
    ///    "status",
    ///    "tokenAddress",
    ///    "type"
    ///  ],
    ///  "properties": {
    ///    "confirmed": {
    ///      "description": "Whether this target is confirmed on chain",
    ///      "examples": [
    ///        false
    ///      ],
    ///      "type": "boolean"
    ///    },
    ///    "errorCode": {
    ///      "description": "Machine-readable failure code",
    ///      "examples": [
    ///        "RPC_READ_FAILED"
    ///      ],
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "errorMessage": {
    ///      "description": "Human-readable failure message",
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "label": {
    ///      "description": "Human-readable target label",
    ///      "examples": [
    ///        "ctf-exchange"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "requiredFor": {
    ///      "description": "Trading side that requires this target",
    ///      "examples": [
    ///        "BUY"
    ///      ],
    ///      "type": "string",
    ///      "enum": [
    ///        "BUY",
    ///        "SELL"
    ///      ]
    ///    },
    ///    "retryable": {
    ///      "description": "Whether a retry request may attempt this target",
    ///      "examples": [
    ///        true
    ///      ],
    ///      "type": "boolean"
    ///    },
    ///    "spenderOrOperator": {
    ///      "description": "Allowance spender or operator address",
    ///      "examples": [
    ///        "0x54d696A602343063000B25a51734E3BbE0Ec80a2"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "status": {
    ///      "description": "Current target status",
    ///      "examples": [
    ///        "submitted"
    ///      ],
    ///      "type": "string",
    ///      "enum": [
    ///        "confirmed",
    ///        "missing",
    ///        "submitted",
    ///        "failed"
    ///      ]
    ///    },
    ///    "tokenAddress": {
    ///      "description": "ERC20 or conditional-token contract address",
    ///      "examples": [
    ///        "0x833589fCD6eDb6E08f4c7C32D4f71b54bdA02913"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "transactionId": {
    ///      "description": "Sponsored transaction ID when submitted",
    ///      "examples": [
    ///        "tx_abc123"
    ///      ],
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "txHash": {
    ///      "description": "Transaction hash when available",
    ///      "examples": [
    ///        "0xabc123..."
    ///      ],
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "type": {
    ///      "description": "Allowance target category",
    ///      "examples": [
    ///        "USDC_ALLOWANCE"
    ///      ],
    ///      "type": "string",
    ///      "enum": [
    ///        "USDC_ALLOWANCE",
    ///        "CTF_APPROVAL"
    ///      ]
    ///    },
    ///    "userOperationHash": {
    ///      "description": "User operation hash when available",
    ///      "examples": [
    ///        "0xdef456..."
    ///      ],
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
    pub struct PartnerAccountAllowanceTarget {
        ///Whether this target is confirmed on chain
        pub confirmed: bool,
        ///Machine-readable failure code
        #[serde(
            rename = "errorCode",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub error_code: ::std::option::Option<::std::string::String>,
        ///Human-readable failure message
        #[serde(
            rename = "errorMessage",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub error_message: ::std::option::Option<::std::string::String>,
        ///Human-readable target label
        pub label: ::std::string::String,
        ///Trading side that requires this target
        #[serde(rename = "requiredFor")]
        pub required_for: PartnerAccountAllowanceTargetRequiredFor,
        ///Whether a retry request may attempt this target
        pub retryable: bool,
        ///Allowance spender or operator address
        #[serde(rename = "spenderOrOperator")]
        pub spender_or_operator: ::std::string::String,
        ///Current target status
        pub status: PartnerAccountAllowanceTargetStatus,
        ///ERC20 or conditional-token contract address
        #[serde(rename = "tokenAddress")]
        pub token_address: ::std::string::String,
        ///Sponsored transaction ID when submitted
        #[serde(
            rename = "transactionId",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub transaction_id: ::std::option::Option<::std::string::String>,
        ///Transaction hash when available
        #[serde(
            rename = "txHash",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub tx_hash: ::std::option::Option<::std::string::String>,
        ///Allowance target category
        #[serde(rename = "type")]
        pub type_: PartnerAccountAllowanceTargetType,
        ///User operation hash when available
        #[serde(
            rename = "userOperationHash",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub user_operation_hash: ::std::option::Option<::std::string::String>,
    }
    ///Trading side that requires this target
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Trading side that requires this target",
    ///  "examples": [
    ///    "BUY"
    ///  ],
    ///  "type": "string",
    ///  "enum": [
    ///    "BUY",
    ///    "SELL"
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
        PartialOrd,
    )]
    pub enum PartnerAccountAllowanceTargetRequiredFor {
        #[serde(rename = "BUY")]
        Buy,
        #[serde(rename = "SELL")]
        Sell,
    }
    impl ::std::fmt::Display for PartnerAccountAllowanceTargetRequiredFor {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Buy => f.write_str("BUY"),
                Self::Sell => f.write_str("SELL"),
            }
        }
    }
    impl ::std::str::FromStr for PartnerAccountAllowanceTargetRequiredFor {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "BUY" => Ok(Self::Buy),
                "SELL" => Ok(Self::Sell),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for PartnerAccountAllowanceTargetRequiredFor {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String> for PartnerAccountAllowanceTargetRequiredFor {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for PartnerAccountAllowanceTargetRequiredFor {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///Current target status
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Current target status",
    ///  "examples": [
    ///    "submitted"
    ///  ],
    ///  "type": "string",
    ///  "enum": [
    ///    "confirmed",
    ///    "missing",
    ///    "submitted",
    ///    "failed"
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
        PartialOrd,
    )]
    pub enum PartnerAccountAllowanceTargetStatus {
        #[serde(rename = "confirmed")]
        Confirmed,
        #[serde(rename = "missing")]
        Missing,
        #[serde(rename = "submitted")]
        Submitted,
        #[serde(rename = "failed")]
        Failed,
    }
    impl ::std::fmt::Display for PartnerAccountAllowanceTargetStatus {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Confirmed => f.write_str("confirmed"),
                Self::Missing => f.write_str("missing"),
                Self::Submitted => f.write_str("submitted"),
                Self::Failed => f.write_str("failed"),
            }
        }
    }
    impl ::std::str::FromStr for PartnerAccountAllowanceTargetStatus {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "confirmed" => Ok(Self::Confirmed),
                "missing" => Ok(Self::Missing),
                "submitted" => Ok(Self::Submitted),
                "failed" => Ok(Self::Failed),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for PartnerAccountAllowanceTargetStatus {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String> for PartnerAccountAllowanceTargetStatus {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for PartnerAccountAllowanceTargetStatus {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///Allowance target category
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Allowance target category",
    ///  "examples": [
    ///    "USDC_ALLOWANCE"
    ///  ],
    ///  "type": "string",
    ///  "enum": [
    ///    "USDC_ALLOWANCE",
    ///    "CTF_APPROVAL"
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
        PartialOrd,
    )]
    pub enum PartnerAccountAllowanceTargetType {
        #[serde(rename = "USDC_ALLOWANCE")]
        UsdcAllowance,
        #[serde(rename = "CTF_APPROVAL")]
        CtfApproval,
    }
    impl ::std::fmt::Display for PartnerAccountAllowanceTargetType {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::UsdcAllowance => f.write_str("USDC_ALLOWANCE"),
                Self::CtfApproval => f.write_str("CTF_APPROVAL"),
            }
        }
    }
    impl ::std::str::FromStr for PartnerAccountAllowanceTargetType {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "USDC_ALLOWANCE" => Ok(Self::UsdcAllowance),
                "CTF_APPROVAL" => Ok(Self::CtfApproval),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for PartnerAccountAllowanceTargetType {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String> for PartnerAccountAllowanceTargetType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for PartnerAccountAllowanceTargetType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///`PartnerAllowanceRetryRateLimitError`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "message",
    ///    "retryAfterSeconds"
    ///  ],
    ///  "properties": {
    ///    "message": {
    ///      "examples": [
    ///        "Allowance retry rate limited"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "retryAfterSeconds": {
    ///      "description": "Seconds to wait before retrying",
    ///      "examples": [
    ///        30
    ///      ],
    ///      "type": "integer"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct PartnerAllowanceRetryRateLimitError {
        pub message: ::std::string::String,
        ///Seconds to wait before retrying
        #[serde(rename = "retryAfterSeconds")]
        pub retry_after_seconds: i64,
    }
    ///`PartnerCapabilitiesResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "allowedScopes",
    ///    "partnerProfileId",
    ///    "tokenManagementEnabled"
    ///  ],
    ///  "properties": {
    ///    "allowedScopes": {
    ///      "description": "Scopes the partner is allowed to request when deriving tokens",
    ///      "examples": [
    ///        [
    ///          "trading",
    ///          "account_creation",
    ///          "delegated_signing"
    ///        ]
    ///      ],
    ///      "type": "array",
    ///      "items": {
    ///        "type": "string",
    ///        "enum": [
    ///          "trading",
    ///          "account_creation",
    ///          "delegated_signing"
    ///        ]
    ///      }
    ///    },
    ///    "partnerProfileId": {
    ///      "description": "Partner profile ID",
    ///      "examples": [
    ///        42
    ///      ],
    ///      "type": "integer"
    ///    },
    ///    "tokenManagementEnabled": {
    ///      "description": "Whether the partner can manage tokens via self-service endpoints",
    ///      "examples": [
    ///        true
    ///      ],
    ///      "type": "boolean"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct PartnerCapabilitiesResponse {
        ///Scopes the partner is allowed to request when deriving tokens
        #[serde(rename = "allowedScopes")]
        pub allowed_scopes: ::std::vec::Vec<PartnerCapabilitiesResponseAllowedScopesItem>,
        ///Partner profile ID
        #[serde(rename = "partnerProfileId")]
        pub partner_profile_id: i64,
        ///Whether the partner can manage tokens via self-service endpoints
        #[serde(rename = "tokenManagementEnabled")]
        pub token_management_enabled: bool,
    }
    ///`PartnerCapabilitiesResponseAllowedScopesItem`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "trading",
    ///    "account_creation",
    ///    "delegated_signing"
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
        PartialOrd,
    )]
    pub enum PartnerCapabilitiesResponseAllowedScopesItem {
        #[serde(rename = "trading")]
        Trading,
        #[serde(rename = "account_creation")]
        AccountCreation,
        #[serde(rename = "delegated_signing")]
        DelegatedSigning,
    }
    impl ::std::fmt::Display for PartnerCapabilitiesResponseAllowedScopesItem {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Trading => f.write_str("trading"),
                Self::AccountCreation => f.write_str("account_creation"),
                Self::DelegatedSigning => f.write_str("delegated_signing"),
            }
        }
    }
    impl ::std::str::FromStr for PartnerCapabilitiesResponseAllowedScopesItem {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "trading" => Ok(Self::Trading),
                "account_creation" => Ok(Self::AccountCreation),
                "delegated_signing" => Ok(Self::DelegatedSigning),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for PartnerCapabilitiesResponseAllowedScopesItem {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String>
        for PartnerCapabilitiesResponseAllowedScopesItem
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String>
        for PartnerCapabilitiesResponseAllowedScopesItem
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///`PortfolioPnlChartDto`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "currentValue",
    ///    "data",
    ///    "percentChange",
    ///    "previousValue",
    ///    "timeframe"
    ///  ],
    ///  "properties": {
    ///    "current": {
    ///      "description": "Current PnL snapshot (hybrid: realised + unrealized + total)",
    ///      "type": [
    ///        "object",
    ///        "null"
    ///      ]
    ///    },
    ///    "currentValue": {
    ///      "description": "Current realised PnL (USD)",
    ///      "examples": [
    ///        -7.04
    ///      ],
    ///      "type": "number"
    ///    },
    ///    "data": {
    ///      "description": "Realised PnL series (USD) for the selected timeframe",
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/PortfolioPnlChartPointDto"
    ///      }
    ///    },
    ///    "percentChange": {
    ///      "description": "Percent change between previousValue and currentValue (realised PnL)",
    ///      "examples": [
    ///        8.24
    ///      ],
    ///      "type": "number"
    ///    },
    ///    "previousValue": {
    ///      "description": "Previous realised PnL (USD) at timeframe start",
    ///      "examples": [
    ///        -6.5
    ///      ],
    ///      "type": "number"
    ///    },
    ///    "timeframe": {
    ///      "description": "Timeframe window used for previous/current comparison",
    ///      "examples": [
    ///        "7d"
    ///      ],
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct PortfolioPnlChartDto {
        ///Current PnL snapshot (hybrid: realised + unrealized + total)
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub current:
            ::std::option::Option<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
        ///Current realised PnL (USD)
        #[serde(rename = "currentValue")]
        pub current_value: f64,
        ///Realised PnL series (USD) for the selected timeframe
        pub data: ::std::vec::Vec<PortfolioPnlChartPointDto>,
        ///Percent change between previousValue and currentValue (realised PnL)
        #[serde(rename = "percentChange")]
        pub percent_change: f64,
        ///Previous realised PnL (USD) at timeframe start
        #[serde(rename = "previousValue")]
        pub previous_value: f64,
        ///Timeframe window used for previous/current comparison
        pub timeframe: ::std::string::String,
    }
    ///`PortfolioPnlChartPointDto`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "timestamp",
    ///    "value"
    ///  ],
    ///  "properties": {
    ///    "timestamp": {
    ///      "description": "Point timestamp (ms)",
    ///      "examples": [
    ///        1700000000000
    ///      ],
    ///      "type": "number"
    ///    },
    ///    "value": {
    ///      "description": "Series value in USD (realised PnL)",
    ///      "examples": [
    ///        123.45
    ///      ],
    ///      "type": "number"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct PortfolioPnlChartPointDto {
        ///Point timestamp (ms)
        pub timestamp: f64,
        ///Series value in USD (realised PnL)
        pub value: f64,
    }
    ///`PortfolioPositionsDto`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "accumulativePoints",
    ///    "amm",
    ///    "clob",
    ///    "points",
    ///    "rewards"
    ///  ],
    ///  "properties": {
    ///    "accumulativePoints": {
    ///      "description": "User accumulative points",
    ///      "examples": [
    ///        456
    ///      ],
    ///      "type": "number"
    ///    },
    ///    "amm": {
    ///      "description": "List of AMM positions",
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/AmmPositionDto"
    ///      }
    ///    },
    ///    "clob": {
    ///      "description": "List of CLOB positions",
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/ClobPositionDto"
    ///      }
    ///    },
    ///    "group": {
    ///      "description": "Grouped CLOB positions (if enabled)",
    ///      "type": "array",
    ///      "items": {
    ///        "type": "object"
    ///      }
    ///    },
    ///    "points": {
    ///      "description": "User points",
    ///      "examples": [
    ///        123
    ///      ],
    ///      "type": "number"
    ///    },
    ///    "rewards": {
    ///      "description": "Rewards information for the portfolio",
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/PortfolioRewardsDto"
    ///        }
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct PortfolioPositionsDto {
        ///User accumulative points
        #[serde(rename = "accumulativePoints")]
        pub accumulative_points: f64,
        ///List of AMM positions
        pub amm: ::std::vec::Vec<AmmPositionDto>,
        ///List of CLOB positions
        pub clob: ::std::vec::Vec<ClobPositionDto>,
        ///Grouped CLOB positions (if enabled)
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub group: ::std::vec::Vec<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
        ///User points
        pub points: f64,
        ///Rewards information for the portfolio
        pub rewards: PortfolioRewardsDto,
    }
    ///`PortfolioRewardsChartEntryDto`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "timestamp",
    ///    "totalRewards",
    ///    "userRewards"
    ///  ],
    ///  "properties": {
    ///    "timestamp": {
    ///      "description": "Timestamp of the rewards entry",
    ///      "examples": [
    ///        1672531200000
    ///      ],
    ///      "type": "number"
    ///    },
    ///    "totalRewards": {
    ///      "description": "Total rewards for this timestamp in token decimals",
    ///      "examples": [
    ///        "60000000"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "userRewards": {
    ///      "description": "User rewards for this timestamp in token decimals",
    ///      "examples": [
    ///        "25000000"
    ///      ],
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct PortfolioRewardsChartEntryDto {
        ///Timestamp of the rewards entry
        pub timestamp: f64,
        ///Total rewards for this timestamp in token decimals
        #[serde(rename = "totalRewards")]
        pub total_rewards: ::std::string::String,
        ///User rewards for this timestamp in token decimals
        #[serde(rename = "userRewards")]
        pub user_rewards: ::std::string::String,
    }
    ///`PortfolioRewardsDto`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "rewardsByEpoch",
    ///    "todaysRewards",
    ///    "totalUnpaidRewards",
    ///    "totalUserRewardsLastEpoch"
    ///  ],
    ///  "properties": {
    ///    "rewardsByEpoch": {
    ///      "description": "Rewards data by epoch",
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/EpochRewardDataDto"
    ///      }
    ///    },
    ///    "rewardsChartData": {
    ///      "description": "Chart data showing rewards over time",
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/PortfolioRewardsChartEntryDto"
    ///      }
    ///    },
    ///    "todaysRewards": {
    ///      "description": "Rewards earned today in token decimals",
    ///      "examples": [
    ///        "50250000"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "totalUnpaidRewards": {
    ///      "description": "Total unpaid rewards in token decimals",
    ///      "examples": [
    ///        "200750000"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "totalUserRewardsLastEpoch": {
    ///      "description": "Total user rewards from the last epoch in token decimals",
    ///      "examples": [
    ///        "150500000"
    ///      ],
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct PortfolioRewardsDto {
        ///Rewards data by epoch
        #[serde(rename = "rewardsByEpoch")]
        pub rewards_by_epoch: ::std::vec::Vec<EpochRewardDataDto>,
        ///Chart data showing rewards over time
        #[serde(
            rename = "rewardsChartData",
            default,
            skip_serializing_if = "::std::vec::Vec::is_empty"
        )]
        pub rewards_chart_data: ::std::vec::Vec<PortfolioRewardsChartEntryDto>,
        ///Rewards earned today in token decimals
        #[serde(rename = "todaysRewards")]
        pub todays_rewards: ::std::string::String,
        ///Total unpaid rewards in token decimals
        #[serde(rename = "totalUnpaidRewards")]
        pub total_unpaid_rewards: ::std::string::String,
        ///Total user rewards from the last epoch in token decimals
        #[serde(rename = "totalUserRewardsLastEpoch")]
        pub total_user_rewards_last_epoch: ::std::string::String,
    }
    ///`PositionDataDto`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "cost",
    ///    "fillPrice",
    ///    "marketValue",
    ///    "realisedPnl",
    ///    "unrealizedPnl"
    ///  ],
    ///  "properties": {
    ///    "cost": {
    ///      "description": "Cost basis of the position in token decimals (e.g. 1 USDC = 1000000)",
    ///      "examples": [
    ///        "75000000"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "fillPrice": {
    ///      "description": "Average fill price of the position (price per share in token decimals)",
    ///      "examples": [
    ///        "750000"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "marketValue": {
    ///      "description": "Current market value of the position in token decimals",
    ///      "examples": [
    ///        "100000000"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "realisedPnl": {
    ///      "description": "Realized profit/loss from closed positions in token decimals",
    ///      "examples": [
    ///        "0"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "unrealizedPnl": {
    ///      "description": "Unrealized profit/loss based on current market price in token decimals",
    ///      "examples": [
    ///        "25000000"
    ///      ],
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct PositionDataDto {
        ///Cost basis of the position in token decimals (e.g. 1 USDC = 1000000)
        pub cost: ::std::string::String,
        ///Average fill price of the position (price per share in token decimals)
        #[serde(rename = "fillPrice")]
        pub fill_price: ::std::string::String,
        ///Current market value of the position in token decimals
        #[serde(rename = "marketValue")]
        pub market_value: ::std::string::String,
        ///Realized profit/loss from closed positions in token decimals
        #[serde(rename = "realisedPnl")]
        pub realised_pnl: ::std::string::String,
        ///Unrealized profit/loss based on current market price in token decimals
        #[serde(rename = "unrealizedPnl")]
        pub unrealized_pnl: ::std::string::String,
    }
    ///`ProfileControllerRetryPartnerAccountAllowancesBody`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "additionalProperties": false
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct ProfileControllerRetryPartnerAccountAllowancesBody {}
    impl ::std::default::Default for ProfileControllerRetryPartnerAccountAllowancesBody {
        fn default() -> Self {
            Self {}
        }
    }
    ///User profile including trading rank and fee rate
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "User profile including trading rank and fee rate",
    ///  "type": "object",
    ///  "properties": {
    ///    "account": {
    ///      "description": "Ethereum wallet address",
    ///      "examples": [
    ///        "0x27b4afBD88fE7c88c6897BB0b4ADE338D0401E37"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "accumulativePoints": {
    ///      "description": "Total accumulated points (all-time)",
    ///      "examples": [
    ///        50000
    ///      ],
    ///      "type": "number"
    ///    },
    ///    "bio": {
    ///      "description": "User biography",
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "client": {
    ///      "description": "Client type used during registration",
    ///      "examples": [
    ///        "eoa"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "displayName": {
    ///      "description": "Display name shown in the UI",
    ///      "examples": [
    ///        "Trader 123"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "embeddedAccount": {
    ///      "description": "Embedded wallet address (Privy)",
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "enrolledInPointsProgram": {
    ///      "description": "Whether the user is enrolled in the points program",
    ///      "type": [
    ///        "boolean",
    ///        "null"
    ///      ]
    ///    },
    ///    "hasTraded": {
    ///      "description": "Whether the user has executed at least one trade",
    ///      "type": "boolean"
    ///    },
    ///    "id": {
    ///      "description": "Internal user ID (used as ownerId in other API flows)",
    ///      "examples": [
    ///        42
    ///      ],
    ///      "type": "integer"
    ///    },
    ///    "isCaptain": {
    ///      "description": "Whether the user is a competition team captain",
    ///      "type": "boolean"
    ///    },
    ///    "isCreator": {
    ///      "description": "Whether the user is a market creator",
    ///      "type": "boolean"
    ///    },
    ///    "isOnboarded": {
    ///      "description": "Whether the user has completed onboarding",
    ///      "type": "boolean"
    ///    },
    ///    "isTop100": {
    ///      "description": "Whether the user is in the top 100 on the leaderboard",
    ///      "type": "boolean"
    ///    },
    ///    "leaderboardPosition": {
    ///      "description": "Position on the leaderboard",
    ///      "type": "integer"
    ///    },
    ///    "mode": {
    ///      "description": "Trading UI mode",
    ///      "type": "string",
    ///      "enum": [
    ///        "simple",
    ///        "advanced"
    ///      ]
    ///    },
    ///    "pfpUrl": {
    ///      "description": "Profile picture URL",
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "points": {
    ///      "description": "Current points balance",
    ///      "examples": [
    ///        15000
    ///      ],
    ///      "type": "number"
    ///    },
    ///    "rank": {
    ///      "description": "User's trading rank, which determines the fee rate applied to orders",
    ///      "type": "object",
    ///      "properties": {
    ///        "feeRateBps": {
    ///          "description": "Fee rate in basis points applied to this user's orders. Use this value when constructing signed orders.",
    ///          "examples": [
    ///            200
    ///          ],
    ///          "type": "integer"
    ///        },
    ///        "id": {
    ///          "description": "Rank ID",
    ///          "type": "integer"
    ///        },
    ///        "name": {
    ///          "description": "Rank tier name",
    ///          "examples": [
    ///            "Gold"
    ///          ],
    ///          "type": "string",
    ///          "enum": [
    ///            "Bronze",
    ///            "Silver",
    ///            "Gold",
    ///            "Platinum",
    ///            "Diamond"
    ///          ]
    ///        }
    ///      }
    ///    },
    ///    "referralCode": {
    ///      "description": "User's referral code",
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "referralData": {
    ///      "description": "Users referred by this user",
    ///      "type": "array",
    ///      "items": {
    ///        "type": "object",
    ///        "properties": {
    ///          "createdAt": {
    ///            "description": "When the referral was created",
    ///            "type": "string"
    ///          },
    ///          "displayName": {
    ///            "description": "Display name of the referred user",
    ///            "type": "string"
    ///          },
    ///          "id": {
    ///            "description": "Referral record ID",
    ///            "type": "integer"
    ///          },
    ///          "pfpUrl": {
    ///            "description": "Profile picture of the referred user",
    ///            "type": [
    ///              "string",
    ///              "null"
    ///            ]
    ///          },
    ///          "referredProfileId": {
    ///            "description": "Profile ID of the referred user",
    ///            "type": "integer"
    ///          }
    ///        }
    ///      }
    ///    },
    ///    "referredUsersCount": {
    ///      "description": "Number of users referred",
    ///      "type": "integer"
    ///    },
    ///    "smartWallet": {
    ///      "description": "Smart wallet address, if configured",
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "socialUrl": {
    ///      "description": "URL to the user's social media profile",
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "tradeWalletChoosen": {
    ///      "description": "Whether the user has selected a trade wallet",
    ///      "type": "boolean"
    ///    },
    ///    "tradeWalletOption": {
    ///      "description": "Which wallet is used for trading",
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ],
    ///      "enum": [
    ///        "eoa",
    ///        "smartWallet"
    ///      ]
    ///    },
    ///    "username": {
    ///      "description": "Unique username",
    ///      "examples": [
    ///        "trader123"
    ///      ],
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ProfileResponse {
        ///Ethereum wallet address
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub account: ::std::option::Option<::std::string::String>,
        ///Total accumulated points (all-time)
        #[serde(
            rename = "accumulativePoints",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub accumulative_points: ::std::option::Option<f64>,
        ///User biography
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub bio: ::std::option::Option<::std::string::String>,
        ///Client type used during registration
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub client: ::std::option::Option<::std::string::String>,
        ///Display name shown in the UI
        #[serde(
            rename = "displayName",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub display_name: ::std::option::Option<::std::string::String>,
        ///Embedded wallet address (Privy)
        #[serde(
            rename = "embeddedAccount",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub embedded_account: ::std::option::Option<::std::string::String>,
        ///Whether the user is enrolled in the points program
        #[serde(
            rename = "enrolledInPointsProgram",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub enrolled_in_points_program: ::std::option::Option<bool>,
        ///Whether the user has executed at least one trade
        #[serde(
            rename = "hasTraded",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub has_traded: ::std::option::Option<bool>,
        ///Internal user ID (used as ownerId in other API flows)
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub id: ::std::option::Option<i64>,
        ///Whether the user is a competition team captain
        #[serde(
            rename = "isCaptain",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub is_captain: ::std::option::Option<bool>,
        ///Whether the user is a market creator
        #[serde(
            rename = "isCreator",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub is_creator: ::std::option::Option<bool>,
        ///Whether the user has completed onboarding
        #[serde(
            rename = "isOnboarded",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub is_onboarded: ::std::option::Option<bool>,
        ///Whether the user is in the top 100 on the leaderboard
        #[serde(
            rename = "isTop100",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub is_top100: ::std::option::Option<bool>,
        ///Position on the leaderboard
        #[serde(
            rename = "leaderboardPosition",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub leaderboard_position: ::std::option::Option<i64>,
        ///Trading UI mode
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub mode: ::std::option::Option<ProfileResponseMode>,
        ///Profile picture URL
        #[serde(
            rename = "pfpUrl",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub pfp_url: ::std::option::Option<::std::string::String>,
        ///Current points balance
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub points: ::std::option::Option<f64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub rank: ::std::option::Option<ProfileResponseRank>,
        ///User's referral code
        #[serde(
            rename = "referralCode",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub referral_code: ::std::option::Option<::std::string::String>,
        ///Users referred by this user
        #[serde(
            rename = "referralData",
            default,
            skip_serializing_if = "::std::vec::Vec::is_empty"
        )]
        pub referral_data: ::std::vec::Vec<ProfileResponseReferralDataItem>,
        ///Number of users referred
        #[serde(
            rename = "referredUsersCount",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub referred_users_count: ::std::option::Option<i64>,
        ///Smart wallet address, if configured
        #[serde(
            rename = "smartWallet",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub smart_wallet: ::std::option::Option<::std::string::String>,
        ///URL to the user's social media profile
        #[serde(
            rename = "socialUrl",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub social_url: ::std::option::Option<::std::string::String>,
        ///Whether the user has selected a trade wallet
        #[serde(
            rename = "tradeWalletChoosen",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub trade_wallet_choosen: ::std::option::Option<bool>,
        ///Which wallet is used for trading
        #[serde(
            rename = "tradeWalletOption",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub trade_wallet_option: ::std::option::Option<ProfileResponseTradeWalletOption>,
        ///Unique username
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub username: ::std::option::Option<::std::string::String>,
    }
    impl ::std::default::Default for ProfileResponse {
        fn default() -> Self {
            Self {
                account: Default::default(),
                accumulative_points: Default::default(),
                bio: Default::default(),
                client: Default::default(),
                display_name: Default::default(),
                embedded_account: Default::default(),
                enrolled_in_points_program: Default::default(),
                has_traded: Default::default(),
                id: Default::default(),
                is_captain: Default::default(),
                is_creator: Default::default(),
                is_onboarded: Default::default(),
                is_top100: Default::default(),
                leaderboard_position: Default::default(),
                mode: Default::default(),
                pfp_url: Default::default(),
                points: Default::default(),
                rank: Default::default(),
                referral_code: Default::default(),
                referral_data: Default::default(),
                referred_users_count: Default::default(),
                smart_wallet: Default::default(),
                social_url: Default::default(),
                trade_wallet_choosen: Default::default(),
                trade_wallet_option: Default::default(),
                username: Default::default(),
            }
        }
    }
    ///Trading UI mode
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Trading UI mode",
    ///  "type": "string",
    ///  "enum": [
    ///    "simple",
    ///    "advanced"
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
        PartialOrd,
    )]
    pub enum ProfileResponseMode {
        #[serde(rename = "simple")]
        Simple,
        #[serde(rename = "advanced")]
        Advanced,
    }
    impl ::std::fmt::Display for ProfileResponseMode {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Simple => f.write_str("simple"),
                Self::Advanced => f.write_str("advanced"),
            }
        }
    }
    impl ::std::str::FromStr for ProfileResponseMode {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "simple" => Ok(Self::Simple),
                "advanced" => Ok(Self::Advanced),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for ProfileResponseMode {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String> for ProfileResponseMode {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for ProfileResponseMode {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///User's trading rank, which determines the fee rate applied to orders
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "User's trading rank, which determines the fee rate applied to orders",
    ///  "type": "object",
    ///  "properties": {
    ///    "feeRateBps": {
    ///      "description": "Fee rate in basis points applied to this user's orders. Use this value when constructing signed orders.",
    ///      "examples": [
    ///        200
    ///      ],
    ///      "type": "integer"
    ///    },
    ///    "id": {
    ///      "description": "Rank ID",
    ///      "type": "integer"
    ///    },
    ///    "name": {
    ///      "description": "Rank tier name",
    ///      "examples": [
    ///        "Gold"
    ///      ],
    ///      "type": "string",
    ///      "enum": [
    ///        "Bronze",
    ///        "Silver",
    ///        "Gold",
    ///        "Platinum",
    ///        "Diamond"
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ProfileResponseRank {
        ///Fee rate in basis points applied to this user's orders. Use this value when constructing signed orders.
        #[serde(
            rename = "feeRateBps",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub fee_rate_bps: ::std::option::Option<i64>,
        ///Rank ID
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub id: ::std::option::Option<i64>,
        ///Rank tier name
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub name: ::std::option::Option<ProfileResponseRankName>,
    }
    impl ::std::default::Default for ProfileResponseRank {
        fn default() -> Self {
            Self {
                fee_rate_bps: Default::default(),
                id: Default::default(),
                name: Default::default(),
            }
        }
    }
    ///Rank tier name
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Rank tier name",
    ///  "examples": [
    ///    "Gold"
    ///  ],
    ///  "type": "string",
    ///  "enum": [
    ///    "Bronze",
    ///    "Silver",
    ///    "Gold",
    ///    "Platinum",
    ///    "Diamond"
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
        PartialOrd,
    )]
    pub enum ProfileResponseRankName {
        Bronze,
        Silver,
        Gold,
        Platinum,
        Diamond,
    }
    impl ::std::fmt::Display for ProfileResponseRankName {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Bronze => f.write_str("Bronze"),
                Self::Silver => f.write_str("Silver"),
                Self::Gold => f.write_str("Gold"),
                Self::Platinum => f.write_str("Platinum"),
                Self::Diamond => f.write_str("Diamond"),
            }
        }
    }
    impl ::std::str::FromStr for ProfileResponseRankName {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "Bronze" => Ok(Self::Bronze),
                "Silver" => Ok(Self::Silver),
                "Gold" => Ok(Self::Gold),
                "Platinum" => Ok(Self::Platinum),
                "Diamond" => Ok(Self::Diamond),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for ProfileResponseRankName {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String> for ProfileResponseRankName {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for ProfileResponseRankName {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///`ProfileResponseReferralDataItem`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "createdAt": {
    ///      "description": "When the referral was created",
    ///      "type": "string"
    ///    },
    ///    "displayName": {
    ///      "description": "Display name of the referred user",
    ///      "type": "string"
    ///    },
    ///    "id": {
    ///      "description": "Referral record ID",
    ///      "type": "integer"
    ///    },
    ///    "pfpUrl": {
    ///      "description": "Profile picture of the referred user",
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "referredProfileId": {
    ///      "description": "Profile ID of the referred user",
    ///      "type": "integer"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ProfileResponseReferralDataItem {
        ///When the referral was created
        #[serde(
            rename = "createdAt",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub created_at: ::std::option::Option<::std::string::String>,
        ///Display name of the referred user
        #[serde(
            rename = "displayName",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub display_name: ::std::option::Option<::std::string::String>,
        ///Referral record ID
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub id: ::std::option::Option<i64>,
        ///Profile picture of the referred user
        #[serde(
            rename = "pfpUrl",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub pfp_url: ::std::option::Option<::std::string::String>,
        ///Profile ID of the referred user
        #[serde(
            rename = "referredProfileId",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub referred_profile_id: ::std::option::Option<i64>,
    }
    impl ::std::default::Default for ProfileResponseReferralDataItem {
        fn default() -> Self {
            Self {
                created_at: Default::default(),
                display_name: Default::default(),
                id: Default::default(),
                pfp_url: Default::default(),
                referred_profile_id: Default::default(),
            }
        }
    }
    ///Which wallet is used for trading
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Which wallet is used for trading",
    ///  "type": "string",
    ///  "enum": [
    ///    "eoa",
    ///    "smartWallet"
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
        PartialOrd,
    )]
    pub enum ProfileResponseTradeWalletOption {
        #[serde(rename = "eoa")]
        Eoa,
        #[serde(rename = "smartWallet")]
        SmartWallet,
    }
    impl ::std::fmt::Display for ProfileResponseTradeWalletOption {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Eoa => f.write_str("eoa"),
                Self::SmartWallet => f.write_str("smartWallet"),
            }
        }
    }
    impl ::std::str::FromStr for ProfileResponseTradeWalletOption {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "eoa" => Ok(Self::Eoa),
                "smartWallet" => Ok(Self::SmartWallet),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for ProfileResponseTradeWalletOption {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String> for ProfileResponseTradeWalletOption {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for ProfileResponseTradeWalletOption {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///`PropertyKeyResponseDto`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "createdAt",
    ///    "id",
    ///    "isSystem",
    ///    "metadata",
    ///    "name",
    ///    "slug",
    ///    "type",
    ///    "updatedAt"
    ///  ],
    ///  "properties": {
    ///    "createdAt": {
    ///      "description": "Creation timestamp",
    ///      "type": "string",
    ///      "format": "date-time"
    ///    },
    ///    "id": {
    ///      "description": "Unique identifier of the property key",
    ///      "type": "string"
    ///    },
    ///    "isSystem": {
    ///      "description": "Whether this is a system-defined property key",
    ///      "type": "boolean"
    ///    },
    ///    "metadata": {
    ///      "description": "Additional metadata for the property key",
    ///      "type": "object"
    ///    },
    ///    "name": {
    ///      "description": "Display name of the property key",
    ///      "type": "string"
    ///    },
    ///    "options": {
    ///      "description": "Available options for this property key",
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/PropertyOptionResponseDto"
    ///      }
    ///    },
    ///    "slug": {
    ///      "description": "URL slug of the property key",
    ///      "type": "string"
    ///    },
    ///    "type": {
    ///      "description": "Type of property key",
    ///      "type": "string",
    ///      "enum": [
    ///        "select",
    ///        "multi-select"
    ///      ]
    ///    },
    ///    "updatedAt": {
    ///      "description": "Last update timestamp",
    ///      "type": "string",
    ///      "format": "date-time"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct PropertyKeyResponseDto {
        ///Creation timestamp
        #[serde(rename = "createdAt")]
        pub created_at: ::chrono::DateTime<::chrono::offset::Utc>,
        ///Unique identifier of the property key
        pub id: ::std::string::String,
        ///Whether this is a system-defined property key
        #[serde(rename = "isSystem")]
        pub is_system: bool,
        ///Additional metadata for the property key
        pub metadata: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        ///Display name of the property key
        pub name: ::std::string::String,
        ///Available options for this property key
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub options: ::std::vec::Vec<PropertyOptionResponseDto>,
        ///URL slug of the property key
        pub slug: ::std::string::String,
        ///Type of property key
        #[serde(rename = "type")]
        pub type_: PropertyKeyResponseDtoType,
        ///Last update timestamp
        #[serde(rename = "updatedAt")]
        pub updated_at: ::chrono::DateTime<::chrono::offset::Utc>,
    }
    ///Type of property key
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Type of property key",
    ///  "type": "string",
    ///  "enum": [
    ///    "select",
    ///    "multi-select"
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
        PartialOrd,
    )]
    pub enum PropertyKeyResponseDtoType {
        #[serde(rename = "select")]
        Select,
        #[serde(rename = "multi-select")]
        MultiSelect,
    }
    impl ::std::fmt::Display for PropertyKeyResponseDtoType {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Select => f.write_str("select"),
                Self::MultiSelect => f.write_str("multi-select"),
            }
        }
    }
    impl ::std::str::FromStr for PropertyKeyResponseDtoType {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "select" => Ok(Self::Select),
                "multi-select" => Ok(Self::MultiSelect),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for PropertyKeyResponseDtoType {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String> for PropertyKeyResponseDtoType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for PropertyKeyResponseDtoType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///`PropertyOptionResponseDto`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "createdAt",
    ///    "id",
    ///    "label",
    ///    "metadata",
    ///    "propertyKeyId",
    ///    "sortOrder",
    ///    "updatedAt",
    ///    "value"
    ///  ],
    ///  "properties": {
    ///    "createdAt": {
    ///      "description": "Creation timestamp",
    ///      "type": "string",
    ///      "format": "date-time"
    ///    },
    ///    "id": {
    ///      "description": "Unique identifier of the property option",
    ///      "type": "string"
    ///    },
    ///    "label": {
    ///      "description": "Display label for the option",
    ///      "type": "string"
    ///    },
    ///    "metadata": {
    ///      "description": "Additional metadata for the option",
    ///      "type": "object"
    ///    },
    ///    "parentOptionId": {
    ///      "description": "Parent option ID for hierarchical options",
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "propertyKeyId": {
    ///      "description": "Property key ID this option belongs to",
    ///      "type": "string"
    ///    },
    ///    "sortOrder": {
    ///      "description": "Sort order for the option",
    ///      "type": "number"
    ///    },
    ///    "updatedAt": {
    ///      "description": "Last update timestamp",
    ///      "type": "string",
    ///      "format": "date-time"
    ///    },
    ///    "value": {
    ///      "description": "Value identifier for the option",
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct PropertyOptionResponseDto {
        ///Creation timestamp
        #[serde(rename = "createdAt")]
        pub created_at: ::chrono::DateTime<::chrono::offset::Utc>,
        ///Unique identifier of the property option
        pub id: ::std::string::String,
        ///Display label for the option
        pub label: ::std::string::String,
        ///Additional metadata for the option
        pub metadata: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        ///Parent option ID for hierarchical options
        #[serde(
            rename = "parentOptionId",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub parent_option_id: ::std::option::Option<::std::string::String>,
        ///Property key ID this option belongs to
        #[serde(rename = "propertyKeyId")]
        pub property_key_id: ::std::string::String,
        ///Sort order for the option
        #[serde(rename = "sortOrder")]
        pub sort_order: f64,
        ///Last update timestamp
        #[serde(rename = "updatedAt")]
        pub updated_at: ::chrono::DateTime<::chrono::offset::Utc>,
        ///Value identifier for the option
        pub value: ::std::string::String,
    }
    ///`PublicPortfolioControllerTradedVolumeResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "data": {
    ///      "description": "Total traded volume in whole USDC (no decimal divisor needed). Includes both taker and maker volume across CLOB and AMM trades.",
    ///      "examples": [
    ///        "9702853"
    ///      ],
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct PublicPortfolioControllerTradedVolumeResponse {
        ///Total traded volume in whole USDC (no decimal divisor needed). Includes both taker and maker volume across CLOB and AMM trades.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub data: ::std::option::Option<::std::string::String>,
    }
    impl ::std::default::Default for PublicPortfolioControllerTradedVolumeResponse {
        fn default() -> Self {
            Self {
                data: Default::default(),
            }
        }
    }
    ///`TradingPortfolioControllerGetAllowanceResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "allowance": {
    ///      "examples": [
    ///        "1000000000000"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "checkedAddress": {
    ///      "examples": [
    ///        "0x..."
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "hasMinimumAllowance": {
    ///      "examples": [
    ///        true
    ///      ],
    ///      "type": "boolean"
    ///    },
    ///    "spender": {
    ///      "examples": [
    ///        "0x..."
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "type": {
    ///      "examples": [
    ///        "clob"
    ///      ],
    ///      "type": "string",
    ///      "enum": [
    ///        "clob",
    ///        "negrisk"
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct TradingPortfolioControllerGetAllowanceResponse {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub allowance: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "checkedAddress",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub checked_address: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "hasMinimumAllowance",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub has_minimum_allowance: ::std::option::Option<bool>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub spender: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "type",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub type_: ::std::option::Option<TradingPortfolioControllerGetAllowanceResponseType>,
    }
    impl ::std::default::Default for TradingPortfolioControllerGetAllowanceResponse {
        fn default() -> Self {
            Self {
                allowance: Default::default(),
                checked_address: Default::default(),
                has_minimum_allowance: Default::default(),
                spender: Default::default(),
                type_: Default::default(),
            }
        }
    }
    ///`TradingPortfolioControllerGetAllowanceResponseType`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "examples": [
    ///    "clob"
    ///  ],
    ///  "type": "string",
    ///  "enum": [
    ///    "clob",
    ///    "negrisk"
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
        PartialOrd,
    )]
    pub enum TradingPortfolioControllerGetAllowanceResponseType {
        #[serde(rename = "clob")]
        Clob,
        #[serde(rename = "negrisk")]
        Negrisk,
    }
    impl ::std::fmt::Display for TradingPortfolioControllerGetAllowanceResponseType {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Clob => f.write_str("clob"),
                Self::Negrisk => f.write_str("negrisk"),
            }
        }
    }
    impl ::std::str::FromStr for TradingPortfolioControllerGetAllowanceResponseType {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "clob" => Ok(Self::Clob),
                "negrisk" => Ok(Self::Negrisk),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for TradingPortfolioControllerGetAllowanceResponseType {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String>
        for TradingPortfolioControllerGetAllowanceResponseType
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String>
        for TradingPortfolioControllerGetAllowanceResponseType
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///`TradingPortfolioControllerGetAllowanceType`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "clob",
    ///    "negrisk"
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
        PartialOrd,
    )]
    pub enum TradingPortfolioControllerGetAllowanceType {
        #[serde(rename = "clob")]
        Clob,
        #[serde(rename = "negrisk")]
        Negrisk,
    }
    impl ::std::fmt::Display for TradingPortfolioControllerGetAllowanceType {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Clob => f.write_str("clob"),
                Self::Negrisk => f.write_str("negrisk"),
            }
        }
    }
    impl ::std::str::FromStr for TradingPortfolioControllerGetAllowanceType {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "clob" => Ok(Self::Clob),
                "negrisk" => Ok(Self::Negrisk),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for TradingPortfolioControllerGetAllowanceType {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String>
        for TradingPortfolioControllerGetAllowanceType
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for TradingPortfolioControllerGetAllowanceType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
}
#[derive(Clone, Debug)]
/**Client for Limitless Exchange API

Production-ready REST API for prediction market trading, portfolio management, and market data on Limitless Exchange (Base L2).

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
    /**Get active API key

    Returns the currently active API key metadata for the authenticated user. Does not return the key value itself.

    Sends a `GET` request to `/auth/api-keys`

    */
    pub async fn api_key_controller_get_active_key<'a>(
        &'a self,
    ) -> Result<
        ResponseValue<::std::option::Option<types::ApiKeyControllerGetActiveKeyResponse>>,
        Error<()>,
    > {
        let url = format!("{}/auth/api-keys", self.baseurl,);
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
            operation_id: "api_key_controller_get_active_key",
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
    /**Create API key

    Creates a new API key for programmatic access. Only available for users authenticated via the UI (Privy). Previous active keys are automatically revoked.

    Sends a `POST` request to `/auth/api-keys`

    */
    pub async fn api_key_controller_create_api_key<'a>(
        &'a self,
    ) -> Result<ResponseValue<types::ApiKeyControllerCreateApiKeyResponse>, Error<()>> {
        let url = format!("{}/auth/api-keys", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
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
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "api_key_controller_create_api_key",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            201u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Revoke API key

    Revokes the currently active API key. The key will immediately stop working for authentication.

    Sends a `DELETE` request to `/auth/api-keys`

    */
    pub async fn api_key_controller_revoke_key<'a>(
        &'a self,
    ) -> Result<ResponseValue<types::ApiKeyControllerRevokeKeyResponse>, Error<()>> {
        let url = format!("{}/auth/api-keys", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .delete(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "api_key_controller_revoke_key",
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
    /**List active tokens

    Lists all active (non-revoked) API tokens for the authenticated partner. Requires token management to be enabled for the partner.

    Sends a `GET` request to `/auth/api-tokens`

    */
    pub async fn api_token_controller_list_tokens<'a>(
        &'a self,
    ) -> Result<ResponseValue<::std::vec::Vec<types::ApiTokenListItem>>, Error<()>> {
        let url = format!("{}/auth/api-tokens", self.baseurl,);
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
            operation_id: "api_token_controller_list_tokens",
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
    /**Get partner capabilities

    Returns the partner capability configuration for the authenticated user, including whether token management is enabled and which scopes are allowed for self-service token derivation. Requires Privy authentication (Bearer token).

    Sends a `GET` request to `/auth/api-tokens/capabilities`

    */
    pub async fn api_token_controller_get_capabilities<'a>(
        &'a self,
    ) -> Result<ResponseValue<types::PartnerCapabilitiesResponse>, Error<()>> {
        let url = format!("{}/auth/api-tokens/capabilities", self.baseurl,);
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
            operation_id: "api_token_controller_get_capabilities",
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
    /**Derive scoped API token

    Creates a new scoped API token for the authenticated partner. Requires Privy authentication (Bearer token). The token secret is returned once at creation — store it securely. Requested scopes must be a subset of the partner's allowed scopes.

    Sends a `POST` request to `/auth/api-tokens/derive`

    */
    pub async fn api_token_controller_derive_token<'a>(
        &'a self,
        body: &'a types::DeriveApiTokenRequest,
    ) -> Result<ResponseValue<types::DeriveApiTokenResponse>, Error<()>> {
        let url = format!("{}/auth/api-tokens/derive", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
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
            operation_id: "api_token_controller_derive_token",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            201u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Revoke token

    Revokes an active API token. The token becomes immediately unusable. Requires token management to be enabled for the partner.

    Sends a `DELETE` request to `/auth/api-tokens/{tokenId}`

    Arguments:
    - `token_id`: The token ID to revoke
    */
    pub async fn api_token_controller_revoke_token<'a>(
        &'a self,
        token_id: &'a str,
    ) -> Result<ResponseValue<types::ApiTokenControllerRevokeTokenResponse>, Error<()>> {
        let url = format!(
            "{}/auth/api-tokens/{}",
            self.baseurl,
            encode_path(&token_id.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .delete(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "api_token_controller_revoke_token",
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
    /**User login (removed)

    Deprecated. Cookie-based session login has been removed. Use API keys instead.

    Sends a `POST` request to `/auth/login`

    */
    pub async fn auth_controller_login<'a>(
        &'a self,
    ) -> Result<ResponseValue<ByteStream>, Error<()>> {
        let url = format!("{}/auth/login", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self.client.post(url).headers(header_map).build()?;
        let info = OperationInfo {
            operation_id: "auth_controller_login",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200..=299 => Ok(ResponseValue::stream(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**User logout

    Logs out the user

    Sends a `POST` request to `/auth/logout`

    */
    pub async fn auth_controller_logout<'a>(
        &'a self,
    ) -> Result<ResponseValue<types::AuthControllerLogoutResponse>, Error<()>> {
        let url = format!("{}/auth/logout", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
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
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "auth_controller_logout",
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
    /**Get signing message

    Returns a signing message with a randomly generated nonce for authentication purposes.

    Sends a `GET` request to `/auth/signing-message`

    */
    pub async fn auth_controller_get_signing_message<'a>(
        &'a self,
    ) -> Result<ResponseValue<::std::string::String>, Error<()>> {
        let url = format!("{}/auth/signing-message", self.baseurl,);
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
            operation_id: "auth_controller_get_signing_message",
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
    /**Verify authentication

    Verifies if the user is authenticated

    Sends a `GET` request to `/auth/verify-auth`

    */
    pub async fn auth_controller_verify_auth<'a>(&'a self) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!("{}/auth/verify-auth", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self.client.get(url).headers(header_map).build()?;
        let info = OperationInfo {
            operation_id: "auth_controller_verify_auth",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Get market page by path

    Resolves a URL path to a market page with its configuration, filters, and breadcrumb

    Sends a `GET` request to `/market-pages/by-path`

    Arguments:
    - `path`: URL path to resolve (e.g., "/sports/football")
    */
    pub async fn market_navigation_controller_by_path<'a>(
        &'a self,
        path: &'a str,
    ) -> Result<ResponseValue<types::MarketPageByPathResponseDto>, Error<()>> {
        let url = format!("{}/market-pages/by-path", self.baseurl,);
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
            .query(&progenitor_client::QueryParam::new("path", &path))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "market_navigation_controller_by_path",
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
    /**List markets for a page

    Returns paginated list of markets for a specific market page with filtering and sorting

    Sends a `GET` request to `/market-pages/{id}/markets`

    Arguments:
    - `id`: Market page ID
    - `cursor`: Cursor token for cursor-based pagination (cannot be used with page)
    - `limit`: Number of items per page (max 100)
    - `page`: Page number for offset pagination (cannot be used with cursor)
    - `sort`: Sort field with optional - prefix for descending (e.g., -updatedAt, createdAt, deadline)
    */
    pub async fn market_navigation_controller_list_markets<'a>(
        &'a self,
        id: &'a str,
        cursor: Option<&'a ::serde_json::Value>,
        limit: Option<&'a ::serde_json::Value>,
        page: Option<&'a ::serde_json::Value>,
        sort: Option<&'a ::serde_json::Value>,
    ) -> Result<ResponseValue<types::ListMarketsResponseDto>, Error<()>> {
        let url = format!(
            "{}/market-pages/{}/markets",
            self.baseurl,
            encode_path(&id.to_string()),
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
            .query(&progenitor_client::QueryParam::new("cursor", &cursor))
            .query(&progenitor_client::QueryParam::new("limit", &limit))
            .query(&progenitor_client::QueryParam::new("page", &page))
            .query(&progenitor_client::QueryParam::new("sort", &sort))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "market_navigation_controller_list_markets",
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
    /**Browse Active Markets

    Retrieves markets and groups that are active and not yet resolved, with optional category filtering

    Sends a `GET` request to `/markets/active`

    Arguments:
    - `category_id`: Filter markets by category ID
    - `automation_type`: Filter by automation type (manual, lumy, or sports)
    - `limit`: Number of items per page
    - `page`: Page number for pagination
    - `sort_by`: Sort by query parameter
    - `trade_type`: Filter by trade type (amm, clob, or group)
    */
    pub async fn market_controller_get_active_markets_1<'a>(
        &'a self,
        category_id: f64,
        automation_type: Option<types::MarketControllerGetActiveMarkets1AutomationType>,
        limit: Option<i64>,
        page: Option<i64>,
        sort_by: Option<&'a str>,
        trade_type: Option<types::MarketControllerGetActiveMarkets1TradeType>,
    ) -> Result<ResponseValue<types::BrowseActiveMarketsResponseDto>, Error<()>> {
        let url = format!("{}/markets/active", self.baseurl,);
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
                "automationType",
                &automation_type,
            ))
            .query(&progenitor_client::QueryParam::new("limit", &limit))
            .query(&progenitor_client::QueryParam::new("page", &page))
            .query(&progenitor_client::QueryParam::new("sortBy", &sort_by))
            .query(&progenitor_client::QueryParam::new(
                "tradeType",
                &trade_type,
            ))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "market_controller_get_active_markets_1",
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
    /**Get active market slugs with metadata

    Retrieves slugs, strike prices, tickers, and deadlines for all active markets and groups. Group markets are nested under their parent group.

    Sends a `GET` request to `/markets/active/slugs`

    */
    pub async fn market_controller_get_active_slugs<'a>(
        &'a self,
    ) -> Result<
        ResponseValue<::std::vec::Vec<types::MarketControllerGetActiveSlugsResponseItem>>,
        Error<()>,
    > {
        let url = format!("{}/markets/active/slugs", self.baseurl,);
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
            operation_id: "market_controller_get_active_slugs",
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
    /**Browse Active Markets

    Retrieves markets and groups that are active and not yet resolved, with optional category filtering

    Sends a `GET` request to `/markets/active/{categoryId}`

    Arguments:
    - `category_id`: Filter markets by category ID
    - `automation_type`: Filter by automation type (manual, lumy, or sports)
    - `limit`: Number of items per page
    - `page`: Page number for pagination
    - `sort_by`: Sort by query parameter
    - `trade_type`: Filter by trade type (amm, clob, or group)
    */
    pub async fn market_controller_get_active_markets_0<'a>(
        &'a self,
        category_id: f64,
        automation_type: Option<types::MarketControllerGetActiveMarkets0AutomationType>,
        limit: Option<i64>,
        page: Option<i64>,
        sort_by: Option<&'a str>,
        trade_type: Option<types::MarketControllerGetActiveMarkets0TradeType>,
    ) -> Result<ResponseValue<types::BrowseActiveMarketsResponseDto>, Error<()>> {
        let url = format!(
            "{}/markets/active/{}",
            self.baseurl,
            encode_path(&category_id.to_string()),
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
                "automationType",
                &automation_type,
            ))
            .query(&progenitor_client::QueryParam::new("limit", &limit))
            .query(&progenitor_client::QueryParam::new("page", &page))
            .query(&progenitor_client::QueryParam::new("sortBy", &sort_by))
            .query(&progenitor_client::QueryParam::new(
                "tradeType",
                &trade_type,
            ))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "market_controller_get_active_markets_0",
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
    /**Get active market count per category

    Returns the number of active markets for each category and the total market count

    Sends a `GET` request to `/markets/categories/count`

    */
    pub async fn market_controller_get_active_market_count_per_category<'a>(
        &'a self,
    ) -> Result<ResponseValue<types::CategoryCountResponseDto>, Error<()>> {
        let url = format!("{}/markets/categories/count", self.baseurl,);
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
            operation_id: "market_controller_get_active_market_count_per_category",
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
    /**Search for markets based on semantic similarity

    Sends a `GET` request to `/markets/search`

    Arguments:
    - `limit`: Maximum number of results to return
    - `page`: Number of page
    - `query`: Search query text
    - `similarity_threshold`: Minimum similarity score (0-1)
    */
    pub async fn market_search_controller_search<'a>(
        &'a self,
        limit: Option<i64>,
        page: Option<i64>,
        query: &'a str,
        similarity_threshold: Option<f64>,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!("{}/markets/search", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .query(&progenitor_client::QueryParam::new("limit", &limit))
            .query(&progenitor_client::QueryParam::new("page", &page))
            .query(&progenitor_client::QueryParam::new("query", &query))
            .query(&progenitor_client::QueryParam::new(
                "similarityThreshold",
                &similarity_threshold,
            ))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "market_search_controller_search",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Get Market Details

    Retrieves market or group data using either an Ethereum address or a slug identifier

    Sends a `GET` request to `/markets/{addressOrSlug}`

    Arguments:
    - `address_or_slug`: Market/group address (0x...) or slug identifier (my-market-name)
    */
    pub async fn market_controller_find<'a>(
        &'a self,
        address_or_slug: &'a str,
    ) -> Result<ResponseValue<types::MarketControllerFindResponse>, Error<()>> {
        let url = format!(
            "{}/markets/{}",
            self.baseurl,
            encode_path(&address_or_slug.to_string()),
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
            operation_id: "market_controller_find",
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
    /**Get oracle candlesticks

    Returns Chainlink candlestick data for markets configured with Chainlink Data Streams chart metadata. Useful for charting the underlying oracle price alongside prediction market prices.

    Sends a `GET` request to `/markets/{addressOrSlug}/oracle-candles`

    Arguments:
    - `address_or_slug`: Market address (0x...) or slug identifier
    - `from`: Start timestamp in UNIX seconds. Defaults to market creation time minus 15 minutes.
    - `interval`: Candlestick interval
    - `to`: End timestamp in UNIX seconds. Defaults to now.
    */
    pub async fn market_controller_get_oracle_candles<'a>(
        &'a self,
        address_or_slug: &'a str,
        from: Option<f64>,
        interval: Option<types::MarketControllerGetOracleCandlesInterval>,
        to: Option<f64>,
    ) -> Result<ResponseValue<types::MarketControllerGetOracleCandlesResponse>, Error<()>> {
        let url = format!(
            "{}/markets/{}/oracle-candles",
            self.baseurl,
            encode_path(&address_or_slug.to_string()),
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
            .query(&progenitor_client::QueryParam::new("from", &from))
            .query(&progenitor_client::QueryParam::new("interval", &interval))
            .query(&progenitor_client::QueryParam::new("to", &to))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "market_controller_get_oracle_candles",
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
    /**Market Events

    Get recent events for a specific market including trades, orders, and liquidity changes

    Sends a `GET` request to `/markets/{slug}/events`

    Arguments:
    - `slug`: Market slug identifier
    - `limit`: Number of events per page
    - `page`: Page number for pagination
    */
    pub async fn market_orderbook_controller_get_market_events<'a>(
        &'a self,
        slug: &'a str,
        limit: Option<i64>,
        page: Option<i64>,
    ) -> Result<ResponseValue<types::MarketOrderbookControllerGetMarketEventsResponse>, Error<()>>
    {
        let url = format!(
            "{}/markets/{}/events",
            self.baseurl,
            encode_path(&slug.to_string()),
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
            .query(&progenitor_client::QueryParam::new("limit", &limit))
            .query(&progenitor_client::QueryParam::new("page", &page))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "market_orderbook_controller_get_market_events",
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
    /**Get feed events for a market

    Retrieves the latest feed events related to a specific market with pagination support

    Sends a `GET` request to `/markets/{slug}/get-feed-events`

    Arguments:
    - `slug`: Slug of the market
    - `limit`: Number of events per page
    - `page`: Page number for pagination
    */
    pub async fn market_controller_get_feed_event<'a>(
        &'a self,
        slug: &'a str,
        limit: Option<i64>,
        page: Option<i64>,
    ) -> Result<ResponseValue<types::FeedEventsResponseDto>, Error<()>> {
        let url = format!(
            "{}/markets/{}/get-feed-events",
            self.baseurl,
            encode_path(&slug.to_string()),
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
            .query(&progenitor_client::QueryParam::new("limit", &limit))
            .query(&progenitor_client::QueryParam::new("page", &page))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "market_controller_get_feed_event",
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
    /**Get Historical Prices

    Retrieve historical price data for a specific market with configurable time intervals

    Sends a `GET` request to `/markets/{slug}/historical-price`

    Arguments:
    - `slug`: Market slug identifier
    - `from`: Start date for historical data
    - `interval`: Time interval for data points
    - `to`: End date for historical data
    */
    pub async fn market_orderbook_controller_get_historical_price<'a>(
        &'a self,
        slug: &'a str,
        from: Option<&'a ::serde_json::Value>,
        interval: Option<types::MarketOrderbookControllerGetHistoricalPriceInterval>,
        to: Option<&'a ::serde_json::Value>,
    ) -> Result<
        ResponseValue<
            ::std::vec::Vec<types::MarketOrderbookControllerGetHistoricalPriceResponseItem>,
        >,
        Error<()>,
    > {
        let url = format!(
            "{}/markets/{}/historical-price",
            self.baseurl,
            encode_path(&slug.to_string()),
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
            .query(&progenitor_client::QueryParam::new("from", &from))
            .query(&progenitor_client::QueryParam::new("interval", &interval))
            .query(&progenitor_client::QueryParam::new("to", &to))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "market_orderbook_controller_get_historical_price",
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
    /**Get Locked Balance

    Get the amount of funds locked in open orders for the authenticated user

    Sends a `GET` request to `/markets/{slug}/locked-balance`

    Arguments:
    - `slug`: Market slug identifier
    */
    pub async fn market_orderbook_controller_get_locked_balance<'a>(
        &'a self,
        slug: &'a str,
    ) -> Result<ResponseValue<types::MarketOrderbookControllerGetLockedBalanceResponse>, Error<()>>
    {
        let url = format!(
            "{}/markets/{}/locked-balance",
            self.baseurl,
            encode_path(&slug.to_string()),
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
            operation_id: "market_orderbook_controller_get_locked_balance",
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
    /**Get Orderbook

    Retrieve the current orderbook for a market showing all open buy and sell orders

    Sends a `GET` request to `/markets/{slug}/orderbook`

    Arguments:
    - `slug`: Market slug identifier
    */
    pub async fn market_orderbook_controller_get_orderbook<'a>(
        &'a self,
        slug: &'a str,
    ) -> Result<ResponseValue<types::MarketOrderbookControllerGetOrderbookResponse>, Error<()>>
    {
        let url = format!(
            "{}/markets/{}/orderbook",
            self.baseurl,
            encode_path(&slug.to_string()),
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
            operation_id: "market_orderbook_controller_get_orderbook",
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
    /**User Orders

    Get all orders placed by the authenticated user for a specific market

    Sends a `GET` request to `/markets/{slug}/user-orders`

    Arguments:
    - `slug`: Market slug identifier
    - `limit`: Maximum number of orders to return
    - `statuses`: Order status(es) to filter by. Defaults to [LIVE] if not provided
    */
    pub async fn market_orderbook_controller_get_user_orders<'a>(
        &'a self,
        slug: &'a str,
        limit: Option<i64>,
        statuses: Option<
            &'a ::std::vec::Vec<types::MarketOrderbookControllerGetUserOrdersStatusesItem>,
        >,
    ) -> Result<ResponseValue<types::MarketOrderbookControllerGetUserOrdersResponse>, Error<()>>
    {
        let url = format!(
            "{}/markets/{}/user-orders",
            self.baseurl,
            encode_path(&slug.to_string()),
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
            .query(&progenitor_client::QueryParam::new("limit", &limit))
            .query(&progenitor_client::QueryParam::new("statuses", &statuses))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "market_orderbook_controller_get_user_orders",
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
    /**Get navigation tree

    Returns the hierarchical navigation structure for market pages

    Sends a `GET` request to `/navigation`

    */
    pub async fn market_navigation_controller_get_navigation_tree<'a>(
        &'a self,
    ) -> Result<ResponseValue<::std::vec::Vec<types::NavigationNodeDto>>, Error<()>> {
        let url = format!("{}/navigation", self.baseurl,);
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
            operation_id: "market_navigation_controller_get_navigation_tree",
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
    /**Create Order

    Creates a buy/sell order for prediction market positions. Requires signed order data.

    Sends a `POST` request to `/orders`

    Arguments:
    - `body`: Order creation data including signature and order parameters
    */
    pub async fn order_controller_create_order<'a>(
        &'a self,
        body: &'a types::CreateOrderDto,
    ) -> Result<ResponseValue<types::OrderResponseDto>, Error<()>> {
        let url = format!("{}/orders", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
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
            operation_id: "order_controller_create_order",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            201u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Cancel all of a user's orders in a specific market

    Sends a `DELETE` request to `/orders/all/{slug}`

    */
    pub async fn order_controller_cancel_all_orders<'a>(
        &'a self,
        slug: &'a str,
    ) -> Result<ResponseValue<types::CancelAllOrdersResponseDto>, Error<()>> {
        let url = format!(
            "{}/orders/all/{}",
            self.baseurl,
            encode_path(&slug.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .delete(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "order_controller_cancel_all_orders",
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
    /**Batch Cancel Orders (Combined)

    Cancel open orders by either internal orderIds or client-provided clientOrderIds.

    Sends a `POST` request to `/orders/batch-cancel`

    */
    pub async fn order_controller_batch_cancel_orders<'a>(
        &'a self,
        body: &'a types::CancelOrderBatchCombinedDto,
    ) -> Result<ResponseValue<types::OrderControllerBatchCancelOrdersResponse>, Error<()>> {
        let url = format!("{}/orders/batch-cancel", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
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
            operation_id: "order_controller_batch_cancel_orders",
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
    /**Cancel Order (Combined)

    Cancel one open order by either internal orderId or client-provided clientOrderId.

    Sends a `POST` request to `/orders/cancel`

    */
    pub async fn order_controller_cancel_order_combined<'a>(
        &'a self,
        body: &'a types::CancelOrderCombinedDto,
    ) -> Result<ResponseValue<types::CancelOrderResponseDto>, Error<()>> {
        let url = format!("{}/orders/cancel", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
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
            operation_id: "order_controller_cancel_order_combined",
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
    /**Cancel multiple orders in batch

    Sends a `POST` request to `/orders/cancel-batch`

    */
    pub async fn order_controller_cancel_order_batch<'a>(
        &'a self,
        body: &'a types::DeleteOrderBatchDto,
    ) -> Result<ResponseValue<types::CancelOrderBatchResponseDto>, Error<()>> {
        let url = format!("{}/orders/cancel-batch", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
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
            operation_id: "order_controller_cancel_order_batch",
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
    /**Get order statuses in batch

    Fetches historical order statuses for multiple orders by internal order IDs and/or client-provided order IDs. Returns execution details, settlement status, and maker match data for each order.

    Sends a `POST` request to `/orders/status/batch`

    */
    pub async fn order_controller_get_order_status_batch<'a>(
        &'a self,
        body: &'a types::BatchOrderStatusRequestDto,
    ) -> Result<ResponseValue<types::BatchOrderStatusResponseDto>, Error<()>> {
        let url = format!("{}/orders/status/batch", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
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
            operation_id: "order_controller_get_order_status_batch",
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
    /**Cancel Order

    Cancel an open order and return locked funds

    Sends a `DELETE` request to `/orders/{orderId}`

    Arguments:
    - `order_id`: Unique identifier of the order to be cancelled
    */
    pub async fn order_controller_cancel_order<'a>(
        &'a self,
        order_id: &'a str,
    ) -> Result<ResponseValue<types::CancelOrderResponseDto>, Error<()>> {
        let url = format!(
            "{}/orders/{}",
            self.baseurl,
            encode_path(&order_id.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .delete(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "order_controller_cancel_order",
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
    /**Get History

    Paginated history including AMM, CLOB trades, splits/merges, NegRisk conversions. Partner API tokens with `delegated_signing` scope may read a sub-account by sending the `x-on-behalf-of: <profileId>` header.

    Sends a `GET` request to `/portfolio/history`

    Arguments:
    - `cursor`: Opaque cursor for cursor-based pagination. Omit for first page. For subsequent pages, pass the `nextCursor` value from the previous response.
    - `limit`: Number of items per page (1-100).
    */
    pub async fn portfolio_controller_get_history<'a>(
        &'a self,
        cursor: Option<&'a str>,
        limit: Option<i64>,
    ) -> Result<ResponseValue<types::HistoryResponseDto>, Error<()>> {
        let url = format!("{}/portfolio/history", self.baseurl,);
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
            .query(&progenitor_client::QueryParam::new("cursor", &cursor))
            .query(&progenitor_client::QueryParam::new("limit", &limit))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "portfolio_controller_get_history",
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
    /**Get portfolio PnL chart

    Hybrid PnL: realised series + current total snapshot

    Sends a `GET` request to `/portfolio/pnl-chart`

    Arguments:
    - `timeframe`: Timeframe window for percent change and chart series
    */
    pub async fn portfolio_controller_get_pnl_chart<'a>(
        &'a self,
        timeframe: Option<&'a str>,
    ) -> Result<ResponseValue<types::PortfolioPnlChartDto>, Error<()>> {
        let url = format!("{}/portfolio/pnl-chart", self.baseurl,);
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
            .query(&progenitor_client::QueryParam::new("timeframe", &timeframe))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "portfolio_controller_get_pnl_chart",
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
    /**Get points breakdown

    Sends a `GET` request to `/portfolio/points`

    */
    pub async fn portfolio_controller_get_points_breakdown<'a>(
        &'a self,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!("{}/portfolio/points", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self.client.get(url).headers(header_map).build()?;
        let info = OperationInfo {
            operation_id: "portfolio_controller_get_points_breakdown",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Get Positions

    Retrieve all active positions with P&L calculations and market values

    Sends a `GET` request to `/portfolio/positions`

    */
    pub async fn portfolio_controller_get_positions<'a>(
        &'a self,
    ) -> Result<ResponseValue<types::PortfolioPositionsDto>, Error<()>> {
        let url = format!("{}/portfolio/positions", self.baseurl,);
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
            operation_id: "portfolio_controller_get_positions",
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
    /**Get Trades

    Retrieve all AMM trades executed by the authenticated user

    Sends a `GET` request to `/portfolio/trades`

    */
    pub async fn portfolio_controller_get_trades<'a>(
        &'a self,
    ) -> Result<ResponseValue<::std::vec::Vec<types::HistoryTradeDto>>, Error<()>> {
        let url = format!("{}/portfolio/trades", self.baseurl,);
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
            operation_id: "portfolio_controller_get_trades",
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
    /**Get User Trading Allowance

    Check USDC allowance for CLOB or NegRisk trading contracts

    Sends a `GET` request to `/portfolio/trading/allowance`

    Arguments:
    - `spender`: Optional spender address override (e.g., venue exchange address)
    - `type_`: Trading type: CLOB or NegRisk
    */
    pub async fn trading_portfolio_controller_get_allowance<'a>(
        &'a self,
        spender: Option<&'a str>,
        type_: types::TradingPortfolioControllerGetAllowanceType,
    ) -> Result<ResponseValue<types::TradingPortfolioControllerGetAllowanceResponse>, Error<()>>
    {
        let url = format!("{}/portfolio/trading/allowance", self.baseurl,);
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
            .query(&progenitor_client::QueryParam::new("spender", &spender))
            .query(&progenitor_client::QueryParam::new("type", &type_))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "trading_portfolio_controller_get_allowance",
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
    /**Get portfolio PnL chart (public)

    Hybrid PnL: realised series + current total snapshot

    Sends a `GET` request to `/portfolio/{account}/pnl-chart`

    Arguments:
    - `account`: User Ethereum address
    - `timeframe`: Timeframe window for percent change and chart series
    */
    pub async fn public_portfolio_controller_get_pnl_chart<'a>(
        &'a self,
        account: &'a str,
        timeframe: Option<&'a str>,
    ) -> Result<ResponseValue<types::PortfolioPnlChartDto>, Error<()>> {
        let url = format!(
            "{}/portfolio/{}/pnl-chart",
            self.baseurl,
            encode_path(&account.to_string()),
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
            .query(&progenitor_client::QueryParam::new("timeframe", &timeframe))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "public_portfolio_controller_get_pnl_chart",
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
    /**Get All User Positions

    Retrieve all positions for a specific user address

    Sends a `GET` request to `/portfolio/{account}/positions`

    Arguments:
    - `account`: User Ethereum address
    */
    pub async fn public_portfolio_controller_get_positions<'a>(
        &'a self,
        account: &'a str,
    ) -> Result<ResponseValue<types::PortfolioPositionsDto>, Error<()>> {
        let url = format!(
            "{}/portfolio/{}/positions",
            self.baseurl,
            encode_path(&account.to_string()),
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
            operation_id: "public_portfolio_controller_get_positions",
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
    /**User Total Volume

    Get total traded volume for a specific user. Returns combined CLOB and AMM volume (both taker and maker sides) in whole USDC. This is the all-time cumulative trading volume, not the current portfolio value.

    Sends a `GET` request to `/portfolio/{account}/traded-volume`

    Arguments:
    - `account`: User Ethereum address
    */
    pub async fn public_portfolio_controller_traded_volume<'a>(
        &'a self,
        account: &'a str,
    ) -> Result<ResponseValue<types::PublicPortfolioControllerTradedVolumeResponse>, Error<()>>
    {
        let url = format!(
            "{}/portfolio/{}/traded-volume",
            self.baseurl,
            encode_path(&account.to_string()),
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
            operation_id: "public_portfolio_controller_traded_volume",
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
    /**Create partner sub-account

    Creates a new sub-account linked to the authenticated partner. Requires HMAC authentication with the `account_creation` scope.

    **Server wallet mode** (`createServerWallet: true`): Creates a Privy server wallet and profile. The partner can then submit orders on behalf of this account using delegated signing.

    **EOA mode** (default): Requires wallet ownership verification via `x-account`, `x-signing-message`, and `x-signature` headers. The end user signs their own orders.

    Sends a `POST` request to `/profiles/partner-accounts`

    Arguments:
    - `x_account`: EOA mode only. Checksummed Ethereum address of the sub-account wallet.
    - `x_signature`: EOA mode only. Hex-encoded signature from the sub-account wallet.
    - `x_signing_message`: EOA mode only. Hex-encoded signing message.
    - `body`
    */
    pub async fn profile_controller_create_partner_account<'a>(
        &'a self,
        x_account: Option<&'a str>,
        x_signature: Option<&'a str>,
        x_signing_message: Option<&'a str>,
        body: &'a types::CreatePartnerAccountRequest,
    ) -> Result<ResponseValue<types::CreatePartnerAccountResponse>, Error<()>> {
        let url = format!("{}/profiles/partner-accounts", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(4usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        if let Some(value) = x_account {
            header_map.append("x-account", value.to_string().try_into()?);
        }
        if let Some(value) = x_signature {
            header_map.append("x-signature", value.to_string().try_into()?);
        }
        if let Some(value) = x_signing_message {
            header_map.append("x-signing-message", value.to_string().try_into()?);
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
            operation_id: "profile_controller_create_partner_account",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            201u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Check partner account allowances

    Checks delegated-trading allowance readiness for a partner-created server-wallet sub-account. Requires HMAC authentication with `account_creation` and `delegated_signing` scopes. Status is based on live chain reads.

    Sends a `GET` request to `/profiles/partner-accounts/{profileId}/allowances`

    Arguments:
    - `profile_id`: Partner sub-account profile ID for the server-wallet child account.
    */
    pub async fn profile_controller_check_partner_account_allowances<'a>(
        &'a self,
        profile_id: i64,
    ) -> Result<ResponseValue<types::PartnerAccountAllowanceResponse>, Error<()>> {
        let url = format!(
            "{}/profiles/partner-accounts/{}/allowances",
            self.baseurl,
            encode_path(&profile_id.to_string()),
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
            operation_id: "profile_controller_check_partner_account_allowances",
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
    /**Retry partner account allowances

    Retries delegated-trading allowance recovery for a partner-created server-wallet sub-account. The retry takes a short wallet lock, re-checks live chain state, and submits only targets still missing. A `submitted` target means this retry request submitted a sponsored transaction or user operation.

    Sends a `POST` request to `/profiles/partner-accounts/{profileId}/allowances/retry`

    Arguments:
    - `profile_id`: Partner sub-account profile ID for the server-wallet child account.
    - `body`
    */
    pub async fn profile_controller_retry_partner_account_allowances<'a>(
        &'a self,
        profile_id: i64,
        body: &'a types::ProfileControllerRetryPartnerAccountAllowancesBody,
    ) -> Result<ResponseValue<types::PartnerAccountAllowanceResponse>, Error<()>> {
        let url = format!(
            "{}/profiles/partner-accounts/{}/allowances/retry",
            self.baseurl,
            encode_path(&profile_id.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
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
            operation_id: "profile_controller_retry_partner_account_allowances",
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
    /**Get your profile

    Retrieve the authenticated user's profile, including their internal user ID and fee rate. This is useful for API users who need their `feeRateBps` (for order signing) or numeric `id` (used as `ownerId` in other flows). You can only access your own profile — requesting another user's address returns 403.

    Sends a `GET` request to `/profiles/{account}`

    Arguments:
    - `account`: Your wallet address (the address associated with your API key)
    */
    pub async fn profile_controller_find_one<'a>(
        &'a self,
        account: &'a str,
    ) -> Result<ResponseValue<types::ProfileResponse>, Error<()>> {
        let url = format!(
            "{}/profiles/{}",
            self.baseurl,
            encode_path(&account.to_string()),
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
            operation_id: "profile_controller_find_one",
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
    /**List property keys

    Returns all property keys with their options, sorted by slug

    Sends a `GET` request to `/property-keys`

    */
    pub async fn market_navigation_controller_list_property_keys<'a>(
        &'a self,
    ) -> Result<ResponseValue<::std::vec::Vec<types::PropertyKeyResponseDto>>, Error<()>> {
        let url = format!("{}/property-keys", self.baseurl,);
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
            operation_id: "market_navigation_controller_list_property_keys",
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
    /**Get property key by ID

    Returns a specific property key with its options

    Sends a `GET` request to `/property-keys/{id}`

    Arguments:
    - `id`: Property key ID
    */
    pub async fn market_navigation_controller_get_property_key<'a>(
        &'a self,
        id: &'a str,
    ) -> Result<ResponseValue<types::PropertyKeyResponseDto>, Error<()>> {
        let url = format!(
            "{}/property-keys/{}",
            self.baseurl,
            encode_path(&id.to_string()),
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
            operation_id: "market_navigation_controller_get_property_key",
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
    /**List property options

    Returns options for a specific property key, optionally filtered by parent option

    Sends a `GET` request to `/property-keys/{id}/options`

    Arguments:
    - `id`: Property key ID
    - `parent_id`: Parent option ID for hierarchical filtering
    */
    pub async fn market_navigation_controller_list_property_options<'a>(
        &'a self,
        id: &'a str,
        parent_id: Option<&'a str>,
    ) -> Result<ResponseValue<::std::vec::Vec<types::PropertyOptionResponseDto>>, Error<()>> {
        let url = format!(
            "{}/property-keys/{}/options",
            self.baseurl,
            encode_path(&id.to_string()),
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
            .query(&progenitor_client::QueryParam::new("parentId", &parent_id))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "market_navigation_controller_list_property_options",
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
