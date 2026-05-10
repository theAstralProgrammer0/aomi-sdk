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
    ///`BalanceParams`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "token",
    ///    "user"
    ///  ],
    ///  "properties": {
    ///    "token": {
    ///      "description": "Token symbol (e.g. \"USDT\").",
    ///      "type": "string"
    ///    },
    ///    "user": {
    ///      "description": "Account name or address (e.g. \"alice\", \"0x...\").",
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct BalanceParams {
        ///Token symbol (e.g. "USDT").
        pub token: ::std::string::String,
        ///Account name or address (e.g. "alice", "0x...").
        pub user: ::std::string::String,
    }
    /**Token balance value. Schema is loose because the underlying response
format is not publicly documented.
*/
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Token balance value. Schema is loose because the underlying response\nformat is not publicly documented.\n",
    ///  "type": "object",
    ///  "properties": {
    ///    "balance": {
    ///      "oneOf": [
    ///        {
    ///          "type": "string"
    ///        },
    ///        {
    ///          "type": "integer",
    ///          "format": "int64"
    ///        }
    ///      ]
    ///    },
    ///    "token": {
    ///      "type": "string"
    ///    },
    ///    "user": {
    ///      "type": "string"
    ///    }
    ///  },
    ///  "additionalProperties": true
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct BalanceResult {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub balance: ::std::option::Option<BalanceResultBalance>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub token: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub user: ::std::option::Option<::std::string::String>,
    }
    impl ::std::default::Default for BalanceResult {
        fn default() -> Self {
            Self {
                balance: Default::default(),
                token: Default::default(),
                user: Default::default(),
            }
        }
    }
    ///`BalanceResultBalance`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "oneOf": [
    ///    {
    ///      "type": "string"
    ///    },
    ///    {
    ///      "type": "integer",
    ///      "format": "int64"
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    #[serde(untagged)]
    pub enum BalanceResultBalance {
        String(::std::string::String),
        Int64(i64),
    }
    impl ::std::fmt::Display for BalanceResultBalance {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                Self::String(x) => x.fmt(f),
                Self::Int64(x) => x.fmt(f),
            }
        }
    }
    impl ::std::convert::From<i64> for BalanceResultBalance {
        fn from(value: i64) -> Self {
            Self::Int64(value)
        }
    }
    ///`GetBalanceBody`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "allOf": [
    ///    {
    ///      "$ref": "#/components/schemas/RpcRequestEnvelope"
    ///    },
    ///    {
    ///      "type": "object",
    ///      "properties": {
    ///        "method": {
    ///          "type": "string",
    ///          "enum": [
    ///            "getBalance"
    ///          ]
    ///        },
    ///        "params": {
    ///          "type": "array",
    ///          "items": {
    ///            "$ref": "#/components/schemas/BalanceParams"
    ///          },
    ///          "maxItems": 1,
    ///          "minItems": 1
    ///        }
    ///      }
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct GetBalanceBody {
        pub id: GetBalanceBodyId,
        pub jsonrpc: GetBalanceBodyJsonrpc,
        pub method: GetBalanceBodyMethod,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub params: ::std::option::Option<[BalanceParams; 1usize]>,
    }
    ///`GetBalanceBodyId`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "oneOf": [
    ///    {
    ///      "type": "integer",
    ///      "format": "int64"
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
    pub enum GetBalanceBodyId {
        Int64(i64),
        String(::std::string::String),
    }
    impl ::std::fmt::Display for GetBalanceBodyId {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                Self::Int64(x) => x.fmt(f),
                Self::String(x) => x.fmt(f),
            }
        }
    }
    impl ::std::convert::From<i64> for GetBalanceBodyId {
        fn from(value: i64) -> Self {
            Self::Int64(value)
        }
    }
    ///`GetBalanceBodyJsonrpc`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "2.0"
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
    pub enum GetBalanceBodyJsonrpc {
        #[serde(rename = "2.0")]
        X20,
    }
    impl ::std::fmt::Display for GetBalanceBodyJsonrpc {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::X20 => f.write_str("2.0"),
            }
        }
    }
    impl ::std::str::FromStr for GetBalanceBodyJsonrpc {
        type Err = self::error::ConversionError;
        fn from_str(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "2.0" => Ok(Self::X20),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for GetBalanceBodyJsonrpc {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String> for GetBalanceBodyJsonrpc {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for GetBalanceBodyJsonrpc {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///`GetBalanceBodyMethod`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "getBalance"
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
    pub enum GetBalanceBodyMethod {
        #[serde(rename = "getBalance")]
        GetBalance,
    }
    impl ::std::fmt::Display for GetBalanceBodyMethod {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::GetBalance => f.write_str("getBalance"),
            }
        }
    }
    impl ::std::str::FromStr for GetBalanceBodyMethod {
        type Err = self::error::ConversionError;
        fn from_str(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "getBalance" => Ok(Self::GetBalance),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for GetBalanceBodyMethod {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String> for GetBalanceBodyMethod {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for GetBalanceBodyMethod {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///`GetBalanceResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "allOf": [
    ///    {
    ///      "$ref": "#/components/schemas/RpcResponseEnvelope"
    ///    },
    ///    {
    ///      "type": "object",
    ///      "properties": {
    ///        "result": {
    ///          "$ref": "#/components/schemas/BalanceResult"
    ///        }
    ///      }
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct GetBalanceResponse {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub error: ::std::option::Option<RpcError>,
        pub id: GetBalanceResponseId,
        pub jsonrpc: GetBalanceResponseJsonrpc,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub result: ::std::option::Option<BalanceResult>,
    }
    ///`GetBalanceResponseId`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "oneOf": [
    ///    {
    ///      "type": "integer",
    ///      "format": "int64"
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
    pub enum GetBalanceResponseId {
        Int64(i64),
        String(::std::string::String),
    }
    impl ::std::fmt::Display for GetBalanceResponseId {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                Self::Int64(x) => x.fmt(f),
                Self::String(x) => x.fmt(f),
            }
        }
    }
    impl ::std::convert::From<i64> for GetBalanceResponseId {
        fn from(value: i64) -> Self {
            Self::Int64(value)
        }
    }
    ///`GetBalanceResponseJsonrpc`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "2.0"
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
    pub enum GetBalanceResponseJsonrpc {
        #[serde(rename = "2.0")]
        X20,
    }
    impl ::std::fmt::Display for GetBalanceResponseJsonrpc {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::X20 => f.write_str("2.0"),
            }
        }
    }
    impl ::std::str::FromStr for GetBalanceResponseJsonrpc {
        type Err = self::error::ConversionError;
        fn from_str(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "2.0" => Ok(Self::X20),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for GetBalanceResponseJsonrpc {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String> for GetBalanceResponseJsonrpc {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for GetBalanceResponseJsonrpc {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///`GetTransactionReceiptBody`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "allOf": [
    ///    {
    ///      "$ref": "#/components/schemas/RpcRequestEnvelope"
    ///    },
    ///    {
    ///      "type": "object",
    ///      "properties": {
    ///        "method": {
    ///          "type": "string",
    ///          "enum": [
    ///            "getTransactionReceipt"
    ///          ]
    ///        },
    ///        "params": {
    ///          "type": "array",
    ///          "items": {
    ///            "description": "Transaction hash",
    ///            "type": "string"
    ///          },
    ///          "maxItems": 1,
    ///          "minItems": 1
    ///        }
    ///      }
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct GetTransactionReceiptBody {
        pub id: GetTransactionReceiptBodyId,
        pub jsonrpc: GetTransactionReceiptBodyJsonrpc,
        pub method: GetTransactionReceiptBodyMethod,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub params: ::std::option::Option<[::std::string::String; 1usize]>,
    }
    ///`GetTransactionReceiptBodyId`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "oneOf": [
    ///    {
    ///      "type": "integer",
    ///      "format": "int64"
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
    pub enum GetTransactionReceiptBodyId {
        Int64(i64),
        String(::std::string::String),
    }
    impl ::std::fmt::Display for GetTransactionReceiptBodyId {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                Self::Int64(x) => x.fmt(f),
                Self::String(x) => x.fmt(f),
            }
        }
    }
    impl ::std::convert::From<i64> for GetTransactionReceiptBodyId {
        fn from(value: i64) -> Self {
            Self::Int64(value)
        }
    }
    ///`GetTransactionReceiptBodyJsonrpc`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "2.0"
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
    pub enum GetTransactionReceiptBodyJsonrpc {
        #[serde(rename = "2.0")]
        X20,
    }
    impl ::std::fmt::Display for GetTransactionReceiptBodyJsonrpc {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::X20 => f.write_str("2.0"),
            }
        }
    }
    impl ::std::str::FromStr for GetTransactionReceiptBodyJsonrpc {
        type Err = self::error::ConversionError;
        fn from_str(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "2.0" => Ok(Self::X20),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for GetTransactionReceiptBodyJsonrpc {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String>
    for GetTransactionReceiptBodyJsonrpc {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String>
    for GetTransactionReceiptBodyJsonrpc {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///`GetTransactionReceiptBodyMethod`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "getTransactionReceipt"
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
    pub enum GetTransactionReceiptBodyMethod {
        #[serde(rename = "getTransactionReceipt")]
        GetTransactionReceipt,
    }
    impl ::std::fmt::Display for GetTransactionReceiptBodyMethod {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::GetTransactionReceipt => f.write_str("getTransactionReceipt"),
            }
        }
    }
    impl ::std::str::FromStr for GetTransactionReceiptBodyMethod {
        type Err = self::error::ConversionError;
        fn from_str(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "getTransactionReceipt" => Ok(Self::GetTransactionReceipt),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for GetTransactionReceiptBodyMethod {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String>
    for GetTransactionReceiptBodyMethod {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String>
    for GetTransactionReceiptBodyMethod {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///`GetTransactionReceiptResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "allOf": [
    ///    {
    ///      "$ref": "#/components/schemas/RpcResponseEnvelope"
    ///    },
    ///    {
    ///      "type": "object",
    ///      "properties": {
    ///        "result": {
    ///          "$ref": "#/components/schemas/TransactionReceipt"
    ///        }
    ///      }
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct GetTransactionReceiptResponse {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub error: ::std::option::Option<RpcError>,
        pub id: GetTransactionReceiptResponseId,
        pub jsonrpc: GetTransactionReceiptResponseJsonrpc,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub result: ::std::option::Option<TransactionReceipt>,
    }
    ///`GetTransactionReceiptResponseId`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "oneOf": [
    ///    {
    ///      "type": "integer",
    ///      "format": "int64"
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
    pub enum GetTransactionReceiptResponseId {
        Int64(i64),
        String(::std::string::String),
    }
    impl ::std::fmt::Display for GetTransactionReceiptResponseId {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                Self::Int64(x) => x.fmt(f),
                Self::String(x) => x.fmt(f),
            }
        }
    }
    impl ::std::convert::From<i64> for GetTransactionReceiptResponseId {
        fn from(value: i64) -> Self {
            Self::Int64(value)
        }
    }
    ///`GetTransactionReceiptResponseJsonrpc`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "2.0"
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
    pub enum GetTransactionReceiptResponseJsonrpc {
        #[serde(rename = "2.0")]
        X20,
    }
    impl ::std::fmt::Display for GetTransactionReceiptResponseJsonrpc {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::X20 => f.write_str("2.0"),
            }
        }
    }
    impl ::std::str::FromStr for GetTransactionReceiptResponseJsonrpc {
        type Err = self::error::ConversionError;
        fn from_str(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "2.0" => Ok(Self::X20),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for GetTransactionReceiptResponseJsonrpc {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String>
    for GetTransactionReceiptResponseJsonrpc {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String>
    for GetTransactionReceiptResponseJsonrpc {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///`GetTransactionStatusBody`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "allOf": [
    ///    {
    ///      "$ref": "#/components/schemas/RpcRequestEnvelope"
    ///    },
    ///    {
    ///      "type": "object",
    ///      "properties": {
    ///        "method": {
    ///          "type": "string",
    ///          "enum": [
    ///            "getTransactionStatus"
    ///          ]
    ///        },
    ///        "params": {
    ///          "type": "array",
    ///          "items": {
    ///            "description": "Transaction hash (hex string, 0x-prefixed)",
    ///            "type": "string"
    ///          },
    ///          "maxItems": 1,
    ///          "minItems": 1
    ///        }
    ///      }
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct GetTransactionStatusBody {
        pub id: GetTransactionStatusBodyId,
        pub jsonrpc: GetTransactionStatusBodyJsonrpc,
        pub method: GetTransactionStatusBodyMethod,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub params: ::std::option::Option<[::std::string::String; 1usize]>,
    }
    ///`GetTransactionStatusBodyId`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "oneOf": [
    ///    {
    ///      "type": "integer",
    ///      "format": "int64"
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
    pub enum GetTransactionStatusBodyId {
        Int64(i64),
        String(::std::string::String),
    }
    impl ::std::fmt::Display for GetTransactionStatusBodyId {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                Self::Int64(x) => x.fmt(f),
                Self::String(x) => x.fmt(f),
            }
        }
    }
    impl ::std::convert::From<i64> for GetTransactionStatusBodyId {
        fn from(value: i64) -> Self {
            Self::Int64(value)
        }
    }
    ///`GetTransactionStatusBodyJsonrpc`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "2.0"
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
    pub enum GetTransactionStatusBodyJsonrpc {
        #[serde(rename = "2.0")]
        X20,
    }
    impl ::std::fmt::Display for GetTransactionStatusBodyJsonrpc {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::X20 => f.write_str("2.0"),
            }
        }
    }
    impl ::std::str::FromStr for GetTransactionStatusBodyJsonrpc {
        type Err = self::error::ConversionError;
        fn from_str(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "2.0" => Ok(Self::X20),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for GetTransactionStatusBodyJsonrpc {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String>
    for GetTransactionStatusBodyJsonrpc {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String>
    for GetTransactionStatusBodyJsonrpc {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///`GetTransactionStatusBodyMethod`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "getTransactionStatus"
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
    pub enum GetTransactionStatusBodyMethod {
        #[serde(rename = "getTransactionStatus")]
        GetTransactionStatus,
    }
    impl ::std::fmt::Display for GetTransactionStatusBodyMethod {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::GetTransactionStatus => f.write_str("getTransactionStatus"),
            }
        }
    }
    impl ::std::str::FromStr for GetTransactionStatusBodyMethod {
        type Err = self::error::ConversionError;
        fn from_str(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "getTransactionStatus" => Ok(Self::GetTransactionStatus),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for GetTransactionStatusBodyMethod {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String>
    for GetTransactionStatusBodyMethod {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String>
    for GetTransactionStatusBodyMethod {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///`GetTransactionStatusResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "allOf": [
    ///    {
    ///      "$ref": "#/components/schemas/RpcResponseEnvelope"
    ///    },
    ///    {
    ///      "type": "object",
    ///      "properties": {
    ///        "result": {
    ///          "$ref": "#/components/schemas/TransactionStatus"
    ///        }
    ///      }
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct GetTransactionStatusResponse {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub error: ::std::option::Option<RpcError>,
        pub id: GetTransactionStatusResponseId,
        pub jsonrpc: GetTransactionStatusResponseJsonrpc,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub result: ::std::option::Option<TransactionStatus>,
    }
    ///`GetTransactionStatusResponseId`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "oneOf": [
    ///    {
    ///      "type": "integer",
    ///      "format": "int64"
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
    pub enum GetTransactionStatusResponseId {
        Int64(i64),
        String(::std::string::String),
    }
    impl ::std::fmt::Display for GetTransactionStatusResponseId {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                Self::Int64(x) => x.fmt(f),
                Self::String(x) => x.fmt(f),
            }
        }
    }
    impl ::std::convert::From<i64> for GetTransactionStatusResponseId {
        fn from(value: i64) -> Self {
            Self::Int64(value)
        }
    }
    ///`GetTransactionStatusResponseJsonrpc`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "2.0"
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
    pub enum GetTransactionStatusResponseJsonrpc {
        #[serde(rename = "2.0")]
        X20,
    }
    impl ::std::fmt::Display for GetTransactionStatusResponseJsonrpc {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::X20 => f.write_str("2.0"),
            }
        }
    }
    impl ::std::str::FromStr for GetTransactionStatusResponseJsonrpc {
        type Err = self::error::ConversionError;
        fn from_str(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "2.0" => Ok(Self::X20),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for GetTransactionStatusResponseJsonrpc {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String>
    for GetTransactionStatusResponseJsonrpc {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String>
    for GetTransactionStatusResponseJsonrpc {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    /**Health check response. Shape is not formally specified; the Aomi
client falls back to wrapping non-JSON bodies in `{ raw: "..." }`.
*/
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Health check response. Shape is not formally specified; the Aomi\nclient falls back to wrapping non-JSON bodies in `{ raw: \"...\" }`.\n",
    ///  "type": "object",
    ///  "properties": {
    ///    "ok": {
    ///      "type": "boolean"
    ///    },
    ///    "status": {
    ///      "type": "string"
    ///    }
    ///  },
    ///  "additionalProperties": true
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct HealthResponse {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub ok: ::std::option::Option<bool>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub status: ::std::option::Option<::std::string::String>,
    }
    impl ::std::default::Default for HealthResponse {
        fn default() -> Self {
            Self {
                ok: Default::default(),
                status: Default::default(),
            }
        }
    }
    ///`RpcError`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "code": {
    ///      "type": "integer",
    ///      "format": "int32"
    ///    },
    ///    "data": {
    ///      "description": "Optional method-specific error payload."
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
    pub struct RpcError {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub code: ::std::option::Option<i32>,
        ///Optional method-specific error payload.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub data: ::std::option::Option<::serde_json::Value>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub message: ::std::option::Option<::std::string::String>,
    }
    impl ::std::default::Default for RpcError {
        fn default() -> Self {
            Self {
                code: Default::default(),
                data: Default::default(),
                message: Default::default(),
            }
        }
    }
    ///`RpcRequestEnvelope`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "id",
    ///    "jsonrpc",
    ///    "method"
    ///  ],
    ///  "properties": {
    ///    "id": {
    ///      "oneOf": [
    ///        {
    ///          "type": "integer",
    ///          "format": "int64"
    ///        },
    ///        {
    ///          "type": "string"
    ///        }
    ///      ]
    ///    },
    ///    "jsonrpc": {
    ///      "type": "string",
    ///      "enum": [
    ///        "2.0"
    ///      ]
    ///    },
    ///    "method": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct RpcRequestEnvelope {
        pub id: RpcRequestEnvelopeId,
        pub jsonrpc: RpcRequestEnvelopeJsonrpc,
        pub method: ::std::string::String,
    }
    ///`RpcRequestEnvelopeId`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "oneOf": [
    ///    {
    ///      "type": "integer",
    ///      "format": "int64"
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
    pub enum RpcRequestEnvelopeId {
        Int64(i64),
        String(::std::string::String),
    }
    impl ::std::fmt::Display for RpcRequestEnvelopeId {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                Self::Int64(x) => x.fmt(f),
                Self::String(x) => x.fmt(f),
            }
        }
    }
    impl ::std::convert::From<i64> for RpcRequestEnvelopeId {
        fn from(value: i64) -> Self {
            Self::Int64(value)
        }
    }
    ///`RpcRequestEnvelopeJsonrpc`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "2.0"
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
    pub enum RpcRequestEnvelopeJsonrpc {
        #[serde(rename = "2.0")]
        X20,
    }
    impl ::std::fmt::Display for RpcRequestEnvelopeJsonrpc {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::X20 => f.write_str("2.0"),
            }
        }
    }
    impl ::std::str::FromStr for RpcRequestEnvelopeJsonrpc {
        type Err = self::error::ConversionError;
        fn from_str(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "2.0" => Ok(Self::X20),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for RpcRequestEnvelopeJsonrpc {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String> for RpcRequestEnvelopeJsonrpc {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for RpcRequestEnvelopeJsonrpc {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///`RpcRequestGeneric`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "allOf": [
    ///    {
    ///      "$ref": "#/components/schemas/RpcRequestEnvelope"
    ///    },
    ///    {
    ///      "type": "object",
    ///      "properties": {
    ///        "params": {
    ///          "description": "Positional params array (JSON-RPC 2.0)",
    ///          "type": "array",
    ///          "items": {}
    ///        }
    ///      }
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct RpcRequestGeneric {
        pub id: RpcRequestGenericId,
        pub jsonrpc: RpcRequestGenericJsonrpc,
        pub method: ::std::string::String,
        ///Positional params array (JSON-RPC 2.0)
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub params: ::std::vec::Vec<::serde_json::Value>,
    }
    ///`RpcRequestGenericId`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "oneOf": [
    ///    {
    ///      "type": "integer",
    ///      "format": "int64"
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
    pub enum RpcRequestGenericId {
        Int64(i64),
        String(::std::string::String),
    }
    impl ::std::fmt::Display for RpcRequestGenericId {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                Self::Int64(x) => x.fmt(f),
                Self::String(x) => x.fmt(f),
            }
        }
    }
    impl ::std::convert::From<i64> for RpcRequestGenericId {
        fn from(value: i64) -> Self {
            Self::Int64(value)
        }
    }
    ///`RpcRequestGenericJsonrpc`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "2.0"
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
    pub enum RpcRequestGenericJsonrpc {
        #[serde(rename = "2.0")]
        X20,
    }
    impl ::std::fmt::Display for RpcRequestGenericJsonrpc {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::X20 => f.write_str("2.0"),
            }
        }
    }
    impl ::std::str::FromStr for RpcRequestGenericJsonrpc {
        type Err = self::error::ConversionError;
        fn from_str(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "2.0" => Ok(Self::X20),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for RpcRequestGenericJsonrpc {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String> for RpcRequestGenericJsonrpc {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for RpcRequestGenericJsonrpc {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///`RpcResponseEnvelope`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "id",
    ///    "jsonrpc"
    ///  ],
    ///  "properties": {
    ///    "error": {
    ///      "$ref": "#/components/schemas/RpcError"
    ///    },
    ///    "id": {
    ///      "oneOf": [
    ///        {
    ///          "type": "integer",
    ///          "format": "int64"
    ///        },
    ///        {
    ///          "type": "string"
    ///        }
    ///      ]
    ///    },
    ///    "jsonrpc": {
    ///      "type": "string",
    ///      "enum": [
    ///        "2.0"
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct RpcResponseEnvelope {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub error: ::std::option::Option<RpcError>,
        pub id: RpcResponseEnvelopeId,
        pub jsonrpc: RpcResponseEnvelopeJsonrpc,
    }
    ///`RpcResponseEnvelopeId`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "oneOf": [
    ///    {
    ///      "type": "integer",
    ///      "format": "int64"
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
    pub enum RpcResponseEnvelopeId {
        Int64(i64),
        String(::std::string::String),
    }
    impl ::std::fmt::Display for RpcResponseEnvelopeId {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                Self::Int64(x) => x.fmt(f),
                Self::String(x) => x.fmt(f),
            }
        }
    }
    impl ::std::convert::From<i64> for RpcResponseEnvelopeId {
        fn from(value: i64) -> Self {
            Self::Int64(value)
        }
    }
    ///`RpcResponseEnvelopeJsonrpc`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "2.0"
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
    pub enum RpcResponseEnvelopeJsonrpc {
        #[serde(rename = "2.0")]
        X20,
    }
    impl ::std::fmt::Display for RpcResponseEnvelopeJsonrpc {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::X20 => f.write_str("2.0"),
            }
        }
    }
    impl ::std::str::FromStr for RpcResponseEnvelopeJsonrpc {
        type Err = self::error::ConversionError;
        fn from_str(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "2.0" => Ok(Self::X20),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for RpcResponseEnvelopeJsonrpc {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String> for RpcResponseEnvelopeJsonrpc {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for RpcResponseEnvelopeJsonrpc {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///`RpcResponseGeneric`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "allOf": [
    ///    {
    ///      "$ref": "#/components/schemas/RpcResponseEnvelope"
    ///    },
    ///    {
    ///      "type": "object",
    ///      "properties": {
    ///        "result": {
    ///          "description": "Method-specific result; shape varies."
    ///        }
    ///      }
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct RpcResponseGeneric {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub error: ::std::option::Option<RpcError>,
        pub id: RpcResponseGenericId,
        pub jsonrpc: RpcResponseGenericJsonrpc,
        ///Method-specific result; shape varies.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub result: ::std::option::Option<::serde_json::Value>,
    }
    ///`RpcResponseGenericId`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "oneOf": [
    ///    {
    ///      "type": "integer",
    ///      "format": "int64"
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
    pub enum RpcResponseGenericId {
        Int64(i64),
        String(::std::string::String),
    }
    impl ::std::fmt::Display for RpcResponseGenericId {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                Self::Int64(x) => x.fmt(f),
                Self::String(x) => x.fmt(f),
            }
        }
    }
    impl ::std::convert::From<i64> for RpcResponseGenericId {
        fn from(value: i64) -> Self {
            Self::Int64(value)
        }
    }
    ///`RpcResponseGenericJsonrpc`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "2.0"
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
    pub enum RpcResponseGenericJsonrpc {
        #[serde(rename = "2.0")]
        X20,
    }
    impl ::std::fmt::Display for RpcResponseGenericJsonrpc {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::X20 => f.write_str("2.0"),
            }
        }
    }
    impl ::std::str::FromStr for RpcResponseGenericJsonrpc {
        type Err = self::error::ConversionError;
        fn from_str(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "2.0" => Ok(Self::X20),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for RpcResponseGenericJsonrpc {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String> for RpcResponseGenericJsonrpc {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for RpcResponseGenericJsonrpc {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///`SendTransactionBody`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "allOf": [
    ///    {
    ///      "$ref": "#/components/schemas/RpcRequestEnvelope"
    ///    },
    ///    {
    ///      "type": "object",
    ///      "properties": {
    ///        "method": {
    ///          "type": "string",
    ///          "enum": [
    ///            "sendTransaction"
    ///          ]
    ///        },
    ///        "params": {
    ///          "type": "array",
    ///          "items": {
    ///            "$ref": "#/components/schemas/TransferTransaction"
    ///          },
    ///          "maxItems": 1,
    ///          "minItems": 1
    ///        }
    ///      }
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct SendTransactionBody {
        pub id: SendTransactionBodyId,
        pub jsonrpc: SendTransactionBodyJsonrpc,
        pub method: SendTransactionBodyMethod,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub params: ::std::option::Option<[TransferTransaction; 1usize]>,
    }
    ///`SendTransactionBodyId`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "oneOf": [
    ///    {
    ///      "type": "integer",
    ///      "format": "int64"
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
    pub enum SendTransactionBodyId {
        Int64(i64),
        String(::std::string::String),
    }
    impl ::std::fmt::Display for SendTransactionBodyId {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                Self::Int64(x) => x.fmt(f),
                Self::String(x) => x.fmt(f),
            }
        }
    }
    impl ::std::convert::From<i64> for SendTransactionBodyId {
        fn from(value: i64) -> Self {
            Self::Int64(value)
        }
    }
    ///`SendTransactionBodyJsonrpc`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "2.0"
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
    pub enum SendTransactionBodyJsonrpc {
        #[serde(rename = "2.0")]
        X20,
    }
    impl ::std::fmt::Display for SendTransactionBodyJsonrpc {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::X20 => f.write_str("2.0"),
            }
        }
    }
    impl ::std::str::FromStr for SendTransactionBodyJsonrpc {
        type Err = self::error::ConversionError;
        fn from_str(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "2.0" => Ok(Self::X20),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for SendTransactionBodyJsonrpc {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String> for SendTransactionBodyJsonrpc {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for SendTransactionBodyJsonrpc {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///`SendTransactionBodyMethod`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "sendTransaction"
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
    pub enum SendTransactionBodyMethod {
        #[serde(rename = "sendTransaction")]
        SendTransaction,
    }
    impl ::std::fmt::Display for SendTransactionBodyMethod {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::SendTransaction => f.write_str("sendTransaction"),
            }
        }
    }
    impl ::std::str::FromStr for SendTransactionBodyMethod {
        type Err = self::error::ConversionError;
        fn from_str(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "sendTransaction" => Ok(Self::SendTransaction),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for SendTransactionBodyMethod {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String> for SendTransactionBodyMethod {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for SendTransactionBodyMethod {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///`SendTransactionResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "allOf": [
    ///    {
    ///      "$ref": "#/components/schemas/RpcResponseEnvelope"
    ///    },
    ///    {
    ///      "type": "object",
    ///      "properties": {
    ///        "result": {
    ///          "$ref": "#/components/schemas/SendTransactionResult"
    ///        }
    ///      }
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct SendTransactionResponse {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub error: ::std::option::Option<RpcError>,
        pub id: SendTransactionResponseId,
        pub jsonrpc: SendTransactionResponseJsonrpc,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub result: ::std::option::Option<SendTransactionResult>,
    }
    ///`SendTransactionResponseId`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "oneOf": [
    ///    {
    ///      "type": "integer",
    ///      "format": "int64"
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
    pub enum SendTransactionResponseId {
        Int64(i64),
        String(::std::string::String),
    }
    impl ::std::fmt::Display for SendTransactionResponseId {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                Self::Int64(x) => x.fmt(f),
                Self::String(x) => x.fmt(f),
            }
        }
    }
    impl ::std::convert::From<i64> for SendTransactionResponseId {
        fn from(value: i64) -> Self {
            Self::Int64(value)
        }
    }
    ///`SendTransactionResponseJsonrpc`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "2.0"
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
    pub enum SendTransactionResponseJsonrpc {
        #[serde(rename = "2.0")]
        X20,
    }
    impl ::std::fmt::Display for SendTransactionResponseJsonrpc {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::X20 => f.write_str("2.0"),
            }
        }
    }
    impl ::std::str::FromStr for SendTransactionResponseJsonrpc {
        type Err = self::error::ConversionError;
        fn from_str(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "2.0" => Ok(Self::X20),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for SendTransactionResponseJsonrpc {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String>
    for SendTransactionResponseJsonrpc {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String>
    for SendTransactionResponseJsonrpc {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    /**Result of submitting a transaction. Shape is not publicly documented;
treated as a permissive object.
*/
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Result of submitting a transaction. Shape is not publicly documented;\ntreated as a permissive object.\n",
    ///  "type": "object",
    ///  "properties": {
    ///    "accepted": {
    ///      "type": "boolean"
    ///    },
    ///    "hash": {
    ///      "type": "string"
    ///    }
    ///  },
    ///  "additionalProperties": true
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct SendTransactionResult {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub accepted: ::std::option::Option<bool>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub hash: ::std::option::Option<::std::string::String>,
    }
    impl ::std::default::Default for SendTransactionResult {
        fn default() -> Self {
            Self {
                accepted: Default::default(),
                hash: Default::default(),
            }
        }
    }
    /**Finalized transaction receipt. Shape is not publicly documented;
permissive object until a real spec is available.
*/
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Finalized transaction receipt. Shape is not publicly documented;\npermissive object until a real spec is available.\n",
    ///  "type": [
    ///    "object",
    ///    "null"
    ///  ],
    ///  "properties": {
    ///    "hash": {
    ///      "type": "string"
    ///    },
    ///    "status": {
    ///      "type": "string"
    ///    }
    ///  },
    ///  "additionalProperties": true
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    #[serde(transparent)]
    pub struct TransactionReceipt(pub ::std::option::Option<TransactionReceiptInner>);
    impl ::std::ops::Deref for TransactionReceipt {
        type Target = ::std::option::Option<TransactionReceiptInner>;
        fn deref(&self) -> &::std::option::Option<TransactionReceiptInner> {
            &self.0
        }
    }
    impl ::std::convert::From<TransactionReceipt>
    for ::std::option::Option<TransactionReceiptInner> {
        fn from(value: TransactionReceipt) -> Self {
            value.0
        }
    }
    impl ::std::convert::From<::std::option::Option<TransactionReceiptInner>>
    for TransactionReceipt {
        fn from(value: ::std::option::Option<TransactionReceiptInner>) -> Self {
            Self(value)
        }
    }
    /**Finalized transaction receipt. Shape is not publicly documented;
permissive object until a real spec is available.
*/
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Finalized transaction receipt. Shape is not publicly documented;\npermissive object until a real spec is available.\n",
    ///  "type": "object",
    ///  "properties": {
    ///    "hash": {
    ///      "type": "string"
    ///    },
    ///    "status": {
    ///      "type": "string"
    ///    }
    ///  },
    ///  "additionalProperties": true
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct TransactionReceiptInner {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub hash: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub status: ::std::option::Option<::std::string::String>,
    }
    impl ::std::default::Default for TransactionReceiptInner {
        fn default() -> Self {
            Self {
                hash: Default::default(),
                status: Default::default(),
            }
        }
    }
    /**Transaction lifecycle state. Observed values include "pending",
"batched", "processed", "failed".
*/
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Transaction lifecycle state. Observed values include \"pending\",\n\"batched\", \"processed\", \"failed\".\n",
    ///  "type": "object",
    ///  "properties": {
    ///    "hash": {
    ///      "type": "string"
    ///    },
    ///    "status": {
    ///      "type": "string"
    ///    }
    ///  },
    ///  "additionalProperties": true
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct TransactionStatus {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub hash: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub status: ::std::option::Option<::std::string::String>,
    }
    impl ::std::default::Default for TransactionStatus {
        fn default() -> Self {
            Self {
                hash: Default::default(),
                status: Default::default(),
            }
        }
    }
    ///`TransferTransaction`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "hash",
    ///    "receiver",
    ///    "sender",
    ///    "token",
    ///    "value"
    ///  ],
    ///  "properties": {
    ///    "hash": {
    ///      "description": "Caller-supplied unique transaction hash (hex).",
    ///      "type": "string"
    ///    },
    ///    "receiver": {
    ///      "description": "Recipient account name or address.",
    ///      "type": "string"
    ///    },
    ///    "sender": {
    ///      "description": "Sender account name or address.",
    ///      "type": "string"
    ///    },
    ///    "token": {
    ///      "description": "Token symbol.",
    ///      "type": "string"
    ///    },
    ///    "value": {
    ///      "description": "Amount in the smallest token denomination.",
    ///      "type": "integer",
    ///      "format": "int64",
    ///      "minimum": 0.0
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct TransferTransaction {
        ///Caller-supplied unique transaction hash (hex).
        pub hash: ::std::string::String,
        ///Recipient account name or address.
        pub receiver: ::std::string::String,
        ///Sender account name or address.
        pub sender: ::std::string::String,
        ///Token symbol.
        pub token: ::std::string::String,
        ///Amount in the smallest token denomination.
        pub value: i64,
    }
}
#[derive(Clone, Debug)]
/**Client for Pelagos Appchain JSON-RPC API

Stub OpenAPI surface for a self-hosted Pelagos appchain node, modelled
from the Aomi `pelagos` app's existing client (`apps/pelagos/src/client.rs`).

The Pelagos appchain exposes a single JSON-RPC POST endpoint at `/rpc`
plus a `GET /health` liveness check. Because JSON-RPC multiplexes many
methods over one path, this spec models each known method as a separate
operation under a synthetic path (`/rpc#<method>`). Codegen will produce
one Rust method per operation; the underlying HTTP request body always
targets `POST /rpc` with `{ jsonrpc: "2.0", id, method, params }`.

NOTE: This is a stub. There is no public Pelagos JSON-RPC reference
discoverable as of the fetched_at date. Schemas are permissive
(`additionalProperties: true`) and reflect only what the existing
Aomi client sends/expects. Needs review against a real node spec
once available.

## Auth
No authentication is described. The appchain is assumed to be
self-hosted (default `http://localhost:8080`); access control is
expected to be handled at the network layer.


Version: 0.1.0*/
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
        "0.1.0"
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
    /**Liveness check for a Pelagos appchain node

Returns a 2xx response when the node is reachable. Body shape is
not formally specified; clients should treat any 2xx as healthy.


Sends a `GET` request to `/health`

*/
    pub async fn health<'a>(
        &'a self,
    ) -> Result<ResponseValue<types::HealthResponse>, Error<()>> {
        let url = format!("{}/health", self.baseurl,);
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
            operation_id: "health",
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
    /**Generic JSON-RPC entrypoint

Catch-all JSON-RPC 2.0 endpoint. Send `{ jsonrpc: "2.0", id, method,
params }`. The Aomi client also models known methods as dedicated
operations below for ergonomic codegen; both routes hit this same
path on the wire.


Sends a `POST` request to `/rpc`

*/
    pub async fn rpc<'a>(
        &'a self,
        body: &'a types::RpcRequestGeneric,
    ) -> Result<ResponseValue<types::RpcResponseGeneric>, Error<()>> {
        let url = format!("{}/rpc", self.baseurl,);
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
            operation_id: "rpc",
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
    /**Get a user's token balance on the appchain

Wire-level: POST /rpc with method = "getBalance" and
params = [{ user, token }].


Sends a `POST` request to `/rpc#getBalance`

*/
    pub async fn get_balance<'a>(
        &'a self,
        body: &'a types::GetBalanceBody,
    ) -> Result<ResponseValue<types::GetBalanceResponse>, Error<()>> {
        let url = format!("{}/rpc#getBalance", self.baseurl,);
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
    /**Look up the lifecycle state of a transaction by hash

Wire-level: POST /rpc with method = "getTransactionStatus" and
params = [hash].


Sends a `POST` request to `/rpc#getTransactionStatus`

*/
    pub async fn get_transaction_status<'a>(
        &'a self,
        body: &'a types::GetTransactionStatusBody,
    ) -> Result<ResponseValue<types::GetTransactionStatusResponse>, Error<()>> {
        let url = format!("{}/rpc#getTransactionStatus", self.baseurl,);
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
            operation_id: "get_transaction_status",
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
    /**Fetch the finalized execution receipt for a settled transaction

Wire-level: POST /rpc with method = "getTransactionReceipt" and
params = [hash]. Returns null for transactions not yet processed.


Sends a `POST` request to `/rpc#getTransactionReceipt`

*/
    pub async fn get_transaction_receipt<'a>(
        &'a self,
        body: &'a types::GetTransactionReceiptBody,
    ) -> Result<ResponseValue<types::GetTransactionReceiptResponse>, Error<()>> {
        let url = format!("{}/rpc#getTransactionReceipt", self.baseurl,);
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
            operation_id: "get_transaction_receipt",
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
    /**Submit a token transfer transaction

Wire-level: POST /rpc with method = "sendTransaction" and
params = [TransferTransaction]. Caller supplies a unique `hash`.


Sends a `POST` request to `/rpc#sendTransaction`

*/
    pub async fn send_transaction<'a>(
        &'a self,
        body: &'a types::SendTransactionBody,
    ) -> Result<ResponseValue<types::SendTransactionResponse>, Error<()>> {
        let url = format!("{}/rpc#sendTransaction", self.baseurl,);
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
            operation_id: "send_transaction",
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
