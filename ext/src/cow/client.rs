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
    ///`AccessListItem`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "address",
    ///    "storage_keys"
    ///  ],
    ///  "properties": {
    ///    "address": {
    ///      "$ref": "#/components/schemas/Address"
    ///    },
    ///    "storage_keys": {
    ///      "type": "array",
    ///      "items": {
    ///        "description": "32 byte storage key encoded as hex with `0x` prefix.",
    ///        "examples": [
    ///          "0x0000000000000000000000000000000000000000000000000000000000000000"
    ///        ],
    ///        "type": "string"
    ///      }
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct AccessListItem {
        pub address: Address,
        pub storage_keys: ::std::vec::Vec<::std::string::String>,
    }
    ///20 byte Ethereum address encoded as a hex with `0x` prefix.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "20 byte Ethereum address encoded as a hex with `0x` prefix.",
    ///  "examples": [
    ///    "0x6810e776880c02933d47db1b9fc05908e5386b96"
    ///  ],
    ///  "type": "string"
    ///}
    /// ```
    /// </details>
    #[derive(
        ::serde::Deserialize, ::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd,
    )]
    #[serde(transparent)]
    pub struct Address(pub ::std::string::String);
    impl ::std::ops::Deref for Address {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }
    impl ::std::convert::From<Address> for ::std::string::String {
        fn from(value: Address) -> Self {
            value.0
        }
    }
    impl ::std::convert::From<::std::string::String> for Address {
        fn from(value: ::std::string::String) -> Self {
            Self(value)
        }
    }
    impl ::std::str::FromStr for Address {
        type Err = ::std::convert::Infallible;
        fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
            Ok(Self(value.to_string()))
        }
    }
    impl ::std::fmt::Display for Address {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            self.0.fmt(f)
        }
    }
    /**The string encoding of a JSON object representing some `appData`. The
    format of the JSON expected in the `appData` field is defined
    [here](https://github.com/cowprotocol/app-data).
    */
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "The string encoding of a JSON object representing some `appData`. The\nformat of the JSON expected in the `appData` field is defined\n[here](https://github.com/cowprotocol/app-data).\n",
    ///  "examples": [
    ///    "{\"version\":\"0.9.0\",\"metadata\":{}}"
    ///  ],
    ///  "type": "string"
    ///}
    /// ```
    /// </details>
    #[derive(
        ::serde::Deserialize, ::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd,
    )]
    #[serde(transparent)]
    pub struct AppData(pub ::std::string::String);
    impl ::std::ops::Deref for AppData {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }
    impl ::std::convert::From<AppData> for ::std::string::String {
        fn from(value: AppData) -> Self {
            value.0
        }
    }
    impl ::std::convert::From<::std::string::String> for AppData {
        fn from(value: ::std::string::String) -> Self {
            Self(value)
        }
    }
    impl ::std::str::FromStr for AppData {
        type Err = ::std::convert::Infallible;
        fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
            Ok(Self(value.to_string()))
        }
    }
    impl ::std::fmt::Display for AppData {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            self.0.fmt(f)
        }
    }
    /**32 bytes encoded as hex with `0x` prefix.
    It's expected to be the hash of the stringified JSON object representing the `appData`.
    */
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "32 bytes encoded as hex with `0x` prefix.\nIt's expected to be the hash of the stringified JSON object representing the `appData`.\n",
    ///  "examples": [
    ///    "0x0000000000000000000000000000000000000000000000000000000000000000"
    ///  ],
    ///  "type": "string"
    ///}
    /// ```
    /// </details>
    #[derive(
        ::serde::Deserialize, ::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd,
    )]
    #[serde(transparent)]
    pub struct AppDataHash(pub ::std::string::String);
    impl ::std::ops::Deref for AppDataHash {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }
    impl ::std::convert::From<AppDataHash> for ::std::string::String {
        fn from(value: AppDataHash) -> Self {
            value.0
        }
    }
    impl ::std::convert::From<::std::string::String> for AppDataHash {
        fn from(value: ::std::string::String) -> Self {
            Self(value)
        }
    }
    impl ::std::str::FromStr for AppDataHash {
        type Err = ::std::convert::Infallible;
        fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
            Ok(Self(value.to_string()))
        }
    }
    impl ::std::fmt::Display for AppDataHash {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            self.0.fmt(f)
        }
    }
    ///An `appData` document that is registered with the API.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "An `appData` document that is registered with the API.",
    ///  "type": "object",
    ///  "required": [
    ///    "appData"
    ///  ],
    ///  "properties": {
    ///    "fullAppData": {
    ///      "$ref": "#/components/schemas/AppData"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct AppDataObject {
        #[serde(rename = "appData")]
        pub app_data: ::serde_json::Value,
        #[serde(
            rename = "fullAppData",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub full_app_data: ::std::option::Option<AppData>,
    }
    /**A batch auction for solving.
     */
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "A batch auction for solving.\n",
    ///  "type": "object",
    ///  "properties": {
    ///    "block": {
    ///      "description": "The block number for the auction. Orders and prices are guaranteed to be valid on this block. Proposed settlements should be valid for this block as well.\n",
    ///      "type": "integer"
    ///    },
    ///    "id": {
    ///      "description": "The unique identifier of the auction. Increment whenever the backend creates a new auction.\n",
    ///      "type": "integer"
    ///    },
    ///    "orders": {
    ///      "description": "The solvable orders included in the auction.\n",
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/AuctionOrder"
    ///      }
    ///    },
    ///    "prices": {
    ///      "$ref": "#/components/schemas/AuctionPrices"
    ///    },
    ///    "surplusCapturingJitOrderOwners": {
    ///      "description": "List of addresses on whose surplus will count towards the objective value of their solution (unlike other orders that were created by the solver).\n",
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/Address"
    ///      }
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct Auction {
        /**The block number for the auction. Orders and prices are guaranteed to be valid on this block. Proposed settlements should be valid for this block as well.
         */
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub block: ::std::option::Option<i64>,
        /**The unique identifier of the auction. Increment whenever the backend creates a new auction.
         */
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub id: ::std::option::Option<i64>,
        /**The solvable orders included in the auction.
         */
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub orders: ::std::vec::Vec<AuctionOrder>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub prices: ::std::option::Option<AuctionPrices>,
        /**List of addresses on whose surplus will count towards the objective value of their solution (unlike other orders that were created by the solver).
         */
        #[serde(
            rename = "surplusCapturingJitOrderOwners",
            default,
            skip_serializing_if = "::std::vec::Vec::is_empty"
        )]
        pub surplus_capturing_jit_order_owners: ::std::vec::Vec<Address>,
    }
    impl ::std::default::Default for Auction {
        fn default() -> Self {
            Self {
                block: Default::default(),
                id: Default::default(),
                orders: Default::default(),
                prices: Default::default(),
                surplus_capturing_jit_order_owners: Default::default(),
            }
        }
    }
    /**A solvable order included in the current batch auction. Contains the data forwarded to solvers for solving.
     */
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "A solvable order included in the current batch auction. Contains the data forwarded to solvers for solving.\n",
    ///  "type": "object",
    ///  "required": [
    ///    "appData",
    ///    "buyAmount",
    ///    "buyToken",
    ///    "buyTokenBalance",
    ///    "class",
    ///    "created",
    ///    "executed",
    ///    "kind",
    ///    "owner",
    ///    "partiallyFillable",
    ///    "postInteractions",
    ///    "preInteractions",
    ///    "protocolFees",
    ///    "receiver",
    ///    "sellAmount",
    ///    "sellToken",
    ///    "sellTokenBalance",
    ///    "signature",
    ///    "uid",
    ///    "validTo"
    ///  ],
    ///  "properties": {
    ///    "appData": {
    ///      "$ref": "#/components/schemas/AppDataHash"
    ///    },
    ///    "buyAmount": {
    ///      "description": "see `OrderParameters::buyAmount`",
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/TokenAmount"
    ///        }
    ///      ]
    ///    },
    ///    "buyToken": {
    ///      "description": "see `OrderParameters::buyToken`",
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/Address"
    ///        }
    ///      ]
    ///    },
    ///    "buyTokenBalance": {
    ///      "description": "see `OrderParameters::buyTokenBalance`",
    ///      "default": "erc20",
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/BuyTokenDestination"
    ///        }
    ///      ]
    ///    },
    ///    "class": {
    ///      "$ref": "#/components/schemas/OrderClass"
    ///    },
    ///    "created": {
    ///      "description": "Creation time of the order. Denominated in epoch seconds.",
    ///      "examples": [
    ///        "123456"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "executed": {
    ///      "description": "Currently executed amount of sell/buy token, depending on the order kind.\n",
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/TokenAmount"
    ///        }
    ///      ]
    ///    },
    ///    "kind": {
    ///      "description": "see `OrderParameters::kind`",
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/OrderKind"
    ///        }
    ///      ]
    ///    },
    ///    "owner": {
    ///      "$ref": "#/components/schemas/Address"
    ///    },
    ///    "partiallyFillable": {
    ///      "description": "see `OrderParameters::partiallyFillable`",
    ///      "type": "boolean"
    ///    },
    ///    "postInteractions": {
    ///      "description": "The post-interactions that need to be executed after the execution of the order.\n",
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/InteractionData"
    ///      }
    ///    },
    ///    "preInteractions": {
    ///      "description": "The pre-interactions that need to be executed before the first execution of the order.\n",
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/InteractionData"
    ///      }
    ///    },
    ///    "protocolFees": {
    ///      "description": "The fee policies that are used to compute the protocol fees for this order.\n",
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/FeePolicy"
    ///      }
    ///    },
    ///    "quote": {
    ///      "description": "A winning quote.\n",
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/Quote"
    ///        }
    ///      ]
    ///    },
    ///    "receiver": {
    ///      "description": "see `OrderParameters::receiver`",
    ///      "oneOf": [
    ///        {
    ///          "type": "null"
    ///        },
    ///        {
    ///          "allOf": [
    ///            {
    ///              "$ref": "#/components/schemas/Address"
    ///            }
    ///          ]
    ///        }
    ///      ]
    ///    },
    ///    "sellAmount": {
    ///      "description": "see `OrderParameters::sellAmount`",
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/TokenAmount"
    ///        }
    ///      ]
    ///    },
    ///    "sellToken": {
    ///      "description": "see `OrderParameters::sellToken`",
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/Address"
    ///        }
    ///      ]
    ///    },
    ///    "sellTokenBalance": {
    ///      "description": "see `OrderParameters::sellTokenBalance`",
    ///      "default": "erc20",
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/SellTokenSource"
    ///        }
    ///      ]
    ///    },
    ///    "signature": {
    ///      "$ref": "#/components/schemas/Signature"
    ///    },
    ///    "uid": {
    ///      "$ref": "#/components/schemas/UID"
    ///    },
    ///    "validTo": {
    ///      "description": "see `OrderParameters::validTo`",
    ///      "type": "integer"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct AuctionOrder {
        #[serde(rename = "appData")]
        pub app_data: AppDataHash,
        ///see `OrderParameters::buyAmount`
        #[serde(rename = "buyAmount")]
        pub buy_amount: TokenAmount,
        ///see `OrderParameters::buyToken`
        #[serde(rename = "buyToken")]
        pub buy_token: Address,
        ///see `OrderParameters::buyTokenBalance`
        #[serde(rename = "buyTokenBalance")]
        pub buy_token_balance: BuyTokenDestination,
        pub class: OrderClass,
        ///Creation time of the order. Denominated in epoch seconds.
        pub created: ::std::string::String,
        /**Currently executed amount of sell/buy token, depending on the order kind.
         */
        pub executed: TokenAmount,
        ///see `OrderParameters::kind`
        pub kind: OrderKind,
        pub owner: Address,
        ///see `OrderParameters::partiallyFillable`
        #[serde(rename = "partiallyFillable")]
        pub partially_fillable: bool,
        /**The post-interactions that need to be executed after the execution of the order.
         */
        #[serde(rename = "postInteractions")]
        pub post_interactions: ::std::vec::Vec<InteractionData>,
        /**The pre-interactions that need to be executed before the first execution of the order.
         */
        #[serde(rename = "preInteractions")]
        pub pre_interactions: ::std::vec::Vec<InteractionData>,
        /**The fee policies that are used to compute the protocol fees for this order.
         */
        #[serde(rename = "protocolFees")]
        pub protocol_fees: ::std::vec::Vec<FeePolicy>,
        /**A winning quote.
         */
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub quote: ::std::option::Option<Quote>,
        ///see `OrderParameters::receiver`
        pub receiver: ::std::option::Option<Address>,
        ///see `OrderParameters::sellAmount`
        #[serde(rename = "sellAmount")]
        pub sell_amount: TokenAmount,
        ///see `OrderParameters::sellToken`
        #[serde(rename = "sellToken")]
        pub sell_token: Address,
        ///see `OrderParameters::sellTokenBalance`
        #[serde(rename = "sellTokenBalance")]
        pub sell_token_balance: SellTokenSource,
        pub signature: Signature,
        pub uid: Uid,
        ///see `OrderParameters::validTo`
        #[serde(rename = "validTo")]
        pub valid_to: i64,
    }
    /**The reference prices for all traded tokens in the auction as a mapping from token addresses to a price denominated in native token (i.e. 1e18 represents a token that trades one to one with the native token). These prices are used for solution competition for computing surplus and converting fees to native token.
     */
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "The reference prices for all traded tokens in the auction as a mapping from token addresses to a price denominated in native token (i.e. 1e18 represents a token that trades one to one with the native token). These prices are used for solution competition for computing surplus and converting fees to native token.\n",
    ///  "type": "object",
    ///  "additionalProperties": {
    ///    "$ref": "#/components/schemas/BigUint"
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    #[serde(transparent)]
    pub struct AuctionPrices(pub ::std::collections::HashMap<::std::string::String, BigUint>);
    impl ::std::ops::Deref for AuctionPrices {
        type Target = ::std::collections::HashMap<::std::string::String, BigUint>;
        fn deref(&self) -> &::std::collections::HashMap<::std::string::String, BigUint> {
            &self.0
        }
    }
    impl ::std::convert::From<AuctionPrices>
        for ::std::collections::HashMap<::std::string::String, BigUint>
    {
        fn from(value: AuctionPrices) -> Self {
            value.0
        }
    }
    impl ::std::convert::From<::std::collections::HashMap<::std::string::String, BigUint>>
        for AuctionPrices
    {
        fn from(value: ::std::collections::HashMap<::std::string::String, BigUint>) -> Self {
            Self(value)
        }
    }
    ///A big unsigned integer encoded in decimal.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "A big unsigned integer encoded in decimal.",
    ///  "examples": [
    ///    "1234567890"
    ///  ],
    ///  "type": "string"
    ///}
    /// ```
    /// </details>
    #[derive(
        ::serde::Deserialize, ::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd,
    )]
    #[serde(transparent)]
    pub struct BigUint(pub ::std::string::String);
    impl ::std::ops::Deref for BigUint {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }
    impl ::std::convert::From<BigUint> for ::std::string::String {
        fn from(value: BigUint) -> Self {
            value.0
        }
    }
    impl ::std::convert::From<::std::string::String> for BigUint {
        fn from(value: ::std::string::String) -> Self {
            Self(value)
        }
    }
    impl ::std::str::FromStr for BigUint {
        type Err = ::std::convert::Infallible;
        fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
            Ok(Self(value.to_string()))
        }
    }
    impl ::std::fmt::Display for BigUint {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            self.0.fmt(f)
        }
    }
    ///Where should the `buyToken` be transferred to?
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Where should the `buyToken` be transferred to?",
    ///  "type": "string",
    ///  "enum": [
    ///    "erc20",
    ///    "internal"
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
    pub enum BuyTokenDestination {
        #[serde(rename = "erc20")]
        Erc20,
        #[serde(rename = "internal")]
        Internal,
    }
    impl ::std::fmt::Display for BuyTokenDestination {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Erc20 => f.write_str("erc20"),
                Self::Internal => f.write_str("internal"),
            }
        }
    }
    impl ::std::str::FromStr for BuyTokenDestination {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "erc20" => Ok(Self::Erc20),
                "internal" => Ok(Self::Internal),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for BuyTokenDestination {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String> for BuyTokenDestination {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for BuyTokenDestination {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///Some `calldata` sent to a contract in a transaction encoded as a hex with `0x` prefix.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Some `calldata` sent to a contract in a transaction encoded as a hex with `0x` prefix.",
    ///  "examples": [
    ///    "0xca11da7a"
    ///  ],
    ///  "type": "string"
    ///}
    /// ```
    /// </details>
    #[derive(
        ::serde::Deserialize, ::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd,
    )]
    #[serde(transparent)]
    pub struct CallData(pub ::std::string::String);
    impl ::std::ops::Deref for CallData {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }
    impl ::std::convert::From<CallData> for ::std::string::String {
        fn from(value: CallData) -> Self {
            value.0
        }
    }
    impl ::std::convert::From<::std::string::String> for CallData {
        fn from(value: ::std::string::String) -> Self {
            Self(value)
        }
    }
    impl ::std::str::FromStr for CallData {
        type Err = ::std::convert::Infallible;
        fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
            Ok(Self(value.to_string()))
        }
    }
    impl ::std::fmt::Display for CallData {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            self.0.fmt(f)
        }
    }
    /**The components that describe a batch auction for the solver competition.
     */
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "The components that describe a batch auction for the solver competition.\n",
    ///  "type": "object",
    ///  "properties": {
    ///    "orders": {
    ///      "description": "The UIDs of the orders included in the auction.\n",
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/UID"
    ///      }
    ///    },
    ///    "prices": {
    ///      "$ref": "#/components/schemas/AuctionPrices"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct CompetitionAuction {
        /**The UIDs of the orders included in the auction.
         */
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub orders: ::std::vec::Vec<Uid>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub prices: ::std::option::Option<AuctionPrices>,
    }
    impl ::std::default::Default for CompetitionAuction {
        fn default() -> Self {
            Self {
                orders: Default::default(),
                prices: Default::default(),
            }
        }
    }
    ///`CompetitionOrderStatus`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "type"
    ///  ],
    ///  "properties": {
    ///    "type": {
    ///      "type": "string",
    ///      "enum": [
    ///        "open",
    ///        "scheduled",
    ///        "active",
    ///        "solved",
    ///        "executing",
    ///        "traded",
    ///        "cancelled"
    ///      ]
    ///    },
    ///    "value": {
    ///      "description": "A list of solvers who participated in the latest competition, sorted\nby score in ascending order, where the last element is the winner.\n\nThe presence of executed amounts defines whether the solver provided\na solution for the desired order.",
    ///      "type": "array",
    ///      "items": {
    ///        "type": "object",
    ///        "required": [
    ///          "solver"
    ///        ],
    ///        "properties": {
    ///          "executedAmounts": {
    ///            "$ref": "#/components/schemas/ExecutedAmounts"
    ///          },
    ///          "solver": {
    ///            "description": "Name of the solver.",
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
    pub struct CompetitionOrderStatus {
        #[serde(rename = "type")]
        pub type_: CompetitionOrderStatusType,
        /**A list of solvers who participated in the latest competition, sorted
        by score in ascending order, where the last element is the winner.

        The presence of executed amounts defines whether the solver provided
        a solution for the desired order.*/
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub value: ::std::vec::Vec<CompetitionOrderStatusValueItem>,
    }
    ///`CompetitionOrderStatusType`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "open",
    ///    "scheduled",
    ///    "active",
    ///    "solved",
    ///    "executing",
    ///    "traded",
    ///    "cancelled"
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
    pub enum CompetitionOrderStatusType {
        #[serde(rename = "open")]
        Open,
        #[serde(rename = "scheduled")]
        Scheduled,
        #[serde(rename = "active")]
        Active,
        #[serde(rename = "solved")]
        Solved,
        #[serde(rename = "executing")]
        Executing,
        #[serde(rename = "traded")]
        Traded,
        #[serde(rename = "cancelled")]
        Cancelled,
    }
    impl ::std::fmt::Display for CompetitionOrderStatusType {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Open => f.write_str("open"),
                Self::Scheduled => f.write_str("scheduled"),
                Self::Active => f.write_str("active"),
                Self::Solved => f.write_str("solved"),
                Self::Executing => f.write_str("executing"),
                Self::Traded => f.write_str("traded"),
                Self::Cancelled => f.write_str("cancelled"),
            }
        }
    }
    impl ::std::str::FromStr for CompetitionOrderStatusType {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "open" => Ok(Self::Open),
                "scheduled" => Ok(Self::Scheduled),
                "active" => Ok(Self::Active),
                "solved" => Ok(Self::Solved),
                "executing" => Ok(Self::Executing),
                "traded" => Ok(Self::Traded),
                "cancelled" => Ok(Self::Cancelled),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for CompetitionOrderStatusType {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String> for CompetitionOrderStatusType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for CompetitionOrderStatusType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///`CompetitionOrderStatusValueItem`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "solver"
    ///  ],
    ///  "properties": {
    ///    "executedAmounts": {
    ///      "$ref": "#/components/schemas/ExecutedAmounts"
    ///    },
    ///    "solver": {
    ///      "description": "Name of the solver.",
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct CompetitionOrderStatusValueItem {
        #[serde(
            rename = "executedAmounts",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub executed_amounts: ::std::option::Option<ExecutedAmounts>,
        ///Name of the solver.
        pub solver: ::std::string::String,
    }
    ///`DebugAuction`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "block",
    ///    "deadline",
    ///    "executions",
    ///    "feePolicies",
    ///    "id",
    ///    "nativePrices",
    ///    "proposedSolutions",
    ///    "settlementAttempts"
    ///  ],
    ///  "properties": {
    ///    "block": {
    ///      "description": "Block number of the auction.",
    ///      "type": "integer"
    ///    },
    ///    "deadline": {
    ///      "description": "Deadline block for the auction.",
    ///      "type": "integer"
    ///    },
    ///    "executions": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/DebugExecution"
    ///      }
    ///    },
    ///    "feePolicies": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/DebugFeePolicy"
    ///      }
    ///    },
    ///    "id": {
    ///      "description": "Auction ID.",
    ///      "type": "integer"
    ///    },
    ///    "nativePrices": {
    ///      "description": "Native prices for the order's sell and buy tokens in this auction. Keys are hex-encoded token addresses, values are decimal price strings.\n",
    ///      "type": "object",
    ///      "additionalProperties": {
    ///        "type": "string"
    ///      }
    ///    },
    ///    "proposedSolutions": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/DebugProposedSolution"
    ///      }
    ///    },
    ///    "settlementAttempts": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/DebugSettlementAttempt"
    ///      }
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct DebugAuction {
        ///Block number of the auction.
        pub block: i64,
        ///Deadline block for the auction.
        pub deadline: i64,
        pub executions: ::std::vec::Vec<DebugExecution>,
        #[serde(rename = "feePolicies")]
        pub fee_policies: ::std::vec::Vec<DebugFeePolicy>,
        ///Auction ID.
        pub id: i64,
        /**Native prices for the order's sell and buy tokens in this auction. Keys are hex-encoded token addresses, values are decimal price strings.
         */
        #[serde(rename = "nativePrices")]
        pub native_prices:
            ::std::collections::HashMap<::std::string::String, ::std::string::String>,
        #[serde(rename = "proposedSolutions")]
        pub proposed_solutions: ::std::vec::Vec<DebugProposedSolution>,
        #[serde(rename = "settlementAttempts")]
        pub settlement_attempts: ::std::vec::Vec<DebugSettlementAttempt>,
    }
    ///`DebugEvent`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "label",
    ///    "timestamp"
    ///  ],
    ///  "properties": {
    ///    "label": {
    ///      "description": "Event type (e.g. created, ready, filtered, traded).",
    ///      "type": "string"
    ///    },
    ///    "reason": {
    ///      "description": "Why the order was filtered or marked invalid. Only present for \"filtered\" and \"invalid\" events.\n",
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "timestamp": {
    ///      "type": "string",
    ///      "format": "date-time"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct DebugEvent {
        ///Event type (e.g. created, ready, filtered, traded).
        pub label: ::std::string::String,
        /**Why the order was filtered or marked invalid. Only present for "filtered" and "invalid" events.
         */
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub reason: ::std::option::Option<::std::string::String>,
        pub timestamp: ::chrono::DateTime<::chrono::offset::Utc>,
    }
    ///`DebugExecution`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "blockNumber",
    ///    "executedFee",
    ///    "executedFeeToken",
    ///    "protocolFees"
    ///  ],
    ///  "properties": {
    ///    "blockNumber": {
    ///      "type": "integer"
    ///    },
    ///    "executedFee": {
    ///      "$ref": "#/components/schemas/TokenAmount"
    ///    },
    ///    "executedFeeToken": {
    ///      "$ref": "#/components/schemas/Address"
    ///    },
    ///    "protocolFees": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/DebugProtocolFee"
    ///      }
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct DebugExecution {
        #[serde(rename = "blockNumber")]
        pub block_number: i64,
        #[serde(rename = "executedFee")]
        pub executed_fee: TokenAmount,
        #[serde(rename = "executedFeeToken")]
        pub executed_fee_token: Address,
        #[serde(rename = "protocolFees")]
        pub protocol_fees: ::std::vec::Vec<DebugProtocolFee>,
    }
    ///Fee policy applied to this order in this auction.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Fee policy applied to this order in this auction.",
    ///  "type": "object",
    ///  "required": [
    ///    "kind"
    ///  ],
    ///  "properties": {
    ///    "kind": {
    ///      "type": "string",
    ///      "enum": [
    ///        "surplus",
    ///        "volume",
    ///        "priceImprovement"
    ///      ]
    ///    },
    ///    "priceImprovementFactor": {
    ///      "type": "number"
    ///    },
    ///    "priceImprovementMaxVolumeFactor": {
    ///      "type": "number"
    ///    },
    ///    "surplusFactor": {
    ///      "type": "number"
    ///    },
    ///    "surplusMaxVolumeFactor": {
    ///      "type": "number"
    ///    },
    ///    "volumeFactor": {
    ///      "type": "number"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct DebugFeePolicy {
        pub kind: DebugFeePolicyKind,
        #[serde(
            rename = "priceImprovementFactor",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub price_improvement_factor: ::std::option::Option<f64>,
        #[serde(
            rename = "priceImprovementMaxVolumeFactor",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub price_improvement_max_volume_factor: ::std::option::Option<f64>,
        #[serde(
            rename = "surplusFactor",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub surplus_factor: ::std::option::Option<f64>,
        #[serde(
            rename = "surplusMaxVolumeFactor",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub surplus_max_volume_factor: ::std::option::Option<f64>,
        #[serde(
            rename = "volumeFactor",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub volume_factor: ::std::option::Option<f64>,
    }
    ///`DebugFeePolicyKind`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "surplus",
    ///    "volume",
    ///    "priceImprovement"
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
    pub enum DebugFeePolicyKind {
        #[serde(rename = "surplus")]
        Surplus,
        #[serde(rename = "volume")]
        Volume,
        #[serde(rename = "priceImprovement")]
        PriceImprovement,
    }
    impl ::std::fmt::Display for DebugFeePolicyKind {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Surplus => f.write_str("surplus"),
                Self::Volume => f.write_str("volume"),
                Self::PriceImprovement => f.write_str("priceImprovement"),
            }
        }
    }
    impl ::std::str::FromStr for DebugFeePolicyKind {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "surplus" => Ok(Self::Surplus),
                "volume" => Ok(Self::Volume),
                "priceImprovement" => Ok(Self::PriceImprovement),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for DebugFeePolicyKind {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String> for DebugFeePolicyKind {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for DebugFeePolicyKind {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///`DebugOrderResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "auctions",
    ///    "events",
    ///    "order",
    ///    "orderUid",
    ///    "trades"
    ///  ],
    ///  "properties": {
    ///    "auctions": {
    ///      "description": "Auctions this order participated in, sorted by ID. Each auction groups all related data: native prices, proposed solutions, executions, settlement attempts, and fee policies.\n",
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/DebugAuction"
    ///      }
    ///    },
    ///    "events": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/DebugEvent"
    ///      }
    ///    },
    ///    "order": {
    ///      "$ref": "#/components/schemas/Order"
    ///    },
    ///    "orderUid": {
    ///      "description": "The UID of the order being debugged.",
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/UID"
    ///        }
    ///      ]
    ///    },
    ///    "trades": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/DebugTrade"
    ///      }
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct DebugOrderResponse {
        /**Auctions this order participated in, sorted by ID. Each auction groups all related data: native prices, proposed solutions, executions, settlement attempts, and fee policies.
         */
        pub auctions: ::std::vec::Vec<DebugAuction>,
        pub events: ::std::vec::Vec<DebugEvent>,
        pub order: Order,
        ///The UID of the order being debugged.
        #[serde(rename = "orderUid")]
        pub order_uid: Uid,
        pub trades: ::std::vec::Vec<DebugTrade>,
    }
    ///`DebugProposedSolution`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "executedBuy",
    ///    "executedSell",
    ///    "filteredOut",
    ///    "isWinner",
    ///    "ranking",
    ///    "score",
    ///    "solutionUid",
    ///    "solver"
    ///  ],
    ///  "properties": {
    ///    "executedBuy": {
    ///      "$ref": "#/components/schemas/TokenAmount"
    ///    },
    ///    "executedSell": {
    ///      "$ref": "#/components/schemas/TokenAmount"
    ///    },
    ///    "filteredOut": {
    ///      "type": "boolean"
    ///    },
    ///    "isWinner": {
    ///      "type": "boolean"
    ///    },
    ///    "ranking": {
    ///      "type": "integer"
    ///    },
    ///    "score": {
    ///      "description": "Decimal-encoded score.",
    ///      "type": "string"
    ///    },
    ///    "solutionUid": {
    ///      "type": "integer"
    ///    },
    ///    "solver": {
    ///      "$ref": "#/components/schemas/Address"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct DebugProposedSolution {
        #[serde(rename = "executedBuy")]
        pub executed_buy: TokenAmount,
        #[serde(rename = "executedSell")]
        pub executed_sell: TokenAmount,
        #[serde(rename = "filteredOut")]
        pub filtered_out: bool,
        #[serde(rename = "isWinner")]
        pub is_winner: bool,
        pub ranking: i64,
        ///Decimal-encoded score.
        pub score: ::std::string::String,
        #[serde(rename = "solutionUid")]
        pub solution_uid: i64,
        pub solver: Address,
    }
    ///`DebugProtocolFee`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "amount",
    ///    "token"
    ///  ],
    ///  "properties": {
    ///    "amount": {
    ///      "$ref": "#/components/schemas/TokenAmount"
    ///    },
    ///    "token": {
    ///      "$ref": "#/components/schemas/Address"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct DebugProtocolFee {
        pub amount: TokenAmount,
        pub token: Address,
    }
    ///`DebugSettlementAttempt`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "deadlineBlock",
    ///    "solutionUid",
    ///    "solver",
    ///    "startBlock",
    ///    "startTimestamp"
    ///  ],
    ///  "properties": {
    ///    "deadlineBlock": {
    ///      "type": "integer"
    ///    },
    ///    "endBlock": {
    ///      "type": "integer"
    ///    },
    ///    "endTimestamp": {
    ///      "type": "string",
    ///      "format": "date-time"
    ///    },
    ///    "outcome": {
    ///      "description": "Settlement outcome (e.g. \"success\", \"revert\").",
    ///      "type": "string"
    ///    },
    ///    "solutionUid": {
    ///      "type": "integer"
    ///    },
    ///    "solver": {
    ///      "$ref": "#/components/schemas/Address"
    ///    },
    ///    "startBlock": {
    ///      "type": "integer"
    ///    },
    ///    "startTimestamp": {
    ///      "type": "string",
    ///      "format": "date-time"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct DebugSettlementAttempt {
        #[serde(rename = "deadlineBlock")]
        pub deadline_block: i64,
        #[serde(
            rename = "endBlock",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub end_block: ::std::option::Option<i64>,
        #[serde(
            rename = "endTimestamp",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub end_timestamp: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
        ///Settlement outcome (e.g. "success", "revert").
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub outcome: ::std::option::Option<::std::string::String>,
        #[serde(rename = "solutionUid")]
        pub solution_uid: i64,
        pub solver: Address,
        #[serde(rename = "startBlock")]
        pub start_block: i64,
        #[serde(rename = "startTimestamp")]
        pub start_timestamp: ::chrono::DateTime<::chrono::offset::Utc>,
    }
    ///`DebugTrade`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "blockNumber",
    ///    "buyAmount",
    ///    "logIndex",
    ///    "sellAmount",
    ///    "sellAmountBeforeFees"
    ///  ],
    ///  "properties": {
    ///    "auctionId": {
    ///      "type": "integer"
    ///    },
    ///    "blockNumber": {
    ///      "type": "integer"
    ///    },
    ///    "buyAmount": {
    ///      "$ref": "#/components/schemas/TokenAmount"
    ///    },
    ///    "logIndex": {
    ///      "type": "integer"
    ///    },
    ///    "sellAmount": {
    ///      "$ref": "#/components/schemas/TokenAmount"
    ///    },
    ///    "sellAmountBeforeFees": {
    ///      "$ref": "#/components/schemas/TokenAmount"
    ///    },
    ///    "txHash": {
    ///      "$ref": "#/components/schemas/TransactionHash"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct DebugTrade {
        #[serde(
            rename = "auctionId",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub auction_id: ::std::option::Option<i64>,
        #[serde(rename = "blockNumber")]
        pub block_number: i64,
        #[serde(rename = "buyAmount")]
        pub buy_amount: TokenAmount,
        #[serde(rename = "logIndex")]
        pub log_index: i64,
        #[serde(rename = "sellAmount")]
        pub sell_amount: TokenAmount,
        #[serde(rename = "sellAmountBeforeFees")]
        pub sell_amount_before_fees: TokenAmount,
        #[serde(
            rename = "txHash",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub tx_hash: ::std::option::Option<TransactionHash>,
    }
    ///65 bytes encoded as hex with `0x` prefix. `r || s || v` from the spec.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "65 bytes encoded as hex with `0x` prefix. `r || s || v` from the spec.",
    ///  "examples": [
    ///    "0x0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000"
    ///  ],
    ///  "type": "string"
    ///}
    /// ```
    /// </details>
    #[derive(
        ::serde::Deserialize, ::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd,
    )]
    #[serde(transparent)]
    pub struct EcdsaSignature(pub ::std::string::String);
    impl ::std::ops::Deref for EcdsaSignature {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }
    impl ::std::convert::From<EcdsaSignature> for ::std::string::String {
        fn from(value: EcdsaSignature) -> Self {
            value.0
        }
    }
    impl ::std::convert::From<::std::string::String> for EcdsaSignature {
        fn from(value: ::std::string::String) -> Self {
            Self(value)
        }
    }
    impl ::std::str::FromStr for EcdsaSignature {
        type Err = ::std::convert::Infallible;
        fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
            Ok(Self(value.to_string()))
        }
    }
    impl ::std::fmt::Display for EcdsaSignature {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            self.0.fmt(f)
        }
    }
    ///How was the order signed?
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "How was the order signed?",
    ///  "type": "string",
    ///  "enum": [
    ///    "eip712",
    ///    "ethsign"
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
    pub enum EcdsaSigningScheme {
        #[serde(rename = "eip712")]
        Eip712,
        #[serde(rename = "ethsign")]
        Ethsign,
    }
    impl ::std::fmt::Display for EcdsaSigningScheme {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Eip712 => f.write_str("eip712"),
                Self::Ethsign => f.write_str("ethsign"),
            }
        }
    }
    impl ::std::str::FromStr for EcdsaSigningScheme {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "eip712" => Ok(Self::Eip712),
                "ethsign" => Ok(Self::Ethsign),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for EcdsaSigningScheme {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String> for EcdsaSigningScheme {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for EcdsaSigningScheme {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///Provides the additional data for ethflow orders.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Provides the additional data for ethflow orders.",
    ///  "type": "object",
    ///  "required": [
    ///    "refundTxHash",
    ///    "userValidTo"
    ///  ],
    ///  "properties": {
    ///    "refundTxHash": {
    ///      "description": "Specifies in which transaction the order was refunded. If\nthis field is null the order was not yet refunded.\n",
    ///      "oneOf": [
    ///        {
    ///          "type": "null"
    ///        },
    ///        {
    ///          "allOf": [
    ///            {
    ///              "$ref": "#/components/schemas/TransactionHash"
    ///            }
    ///          ]
    ///        }
    ///      ]
    ///    },
    ///    "userValidTo": {
    ///      "description": "Describes the `validTo` of an order ethflow order.\n\n**NOTE**: For ethflow orders, the `validTo` encoded in the smart\ncontract is `type(uint256).max`.\n",
    ///      "type": "integer"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct EthflowData {
        /**Specifies in which transaction the order was refunded. If
        this field is null the order was not yet refunded.
        */
        #[serde(rename = "refundTxHash")]
        pub refund_tx_hash: ::std::option::Option<TransactionHash>,
        /**Describes the `validTo` of an order ethflow order.

        **NOTE**: For ethflow orders, the `validTo` encoded in the smart
        contract is `type(uint256).max`.
        */
        #[serde(rename = "userValidTo")]
        pub user_valid_to: i64,
    }
    ///`ExecutedAmounts`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "buy",
    ///    "sell"
    ///  ],
    ///  "properties": {
    ///    "buy": {
    ///      "$ref": "#/components/schemas/BigUint"
    ///    },
    ///    "sell": {
    ///      "$ref": "#/components/schemas/BigUint"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ExecutedAmounts {
        pub buy: BigUint,
        pub sell: BigUint,
    }
    ///`ExecutedProtocolFee`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "amount": {
    ///      "allOf": [
    ///        {
    ///          "description": "Fee amount taken"
    ///        },
    ///        {
    ///          "$ref": "#/components/schemas/TokenAmount"
    ///        }
    ///      ]
    ///    },
    ///    "policy": {
    ///      "$ref": "#/components/schemas/FeePolicy"
    ///    },
    ///    "token": {
    ///      "allOf": [
    ///        {
    ///          "description": "The token in which the fee is taken"
    ///        },
    ///        {
    ///          "$ref": "#/components/schemas/Address"
    ///        }
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ExecutedProtocolFee {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub amount: ::std::option::Option<TokenAmount>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub policy: ::std::option::Option<FeePolicy>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub token: ::std::option::Option<Address>,
    }
    impl ::std::default::Default for ExecutedProtocolFee {
        fn default() -> Self {
            Self {
                amount: Default::default(),
                policy: Default::default(),
                token: Default::default(),
            }
        }
    }
    ///Defines the ways to calculate the protocol fee.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Defines the ways to calculate the protocol fee.",
    ///  "oneOf": [
    ///    {
    ///      "$ref": "#/components/schemas/Surplus"
    ///    },
    ///    {
    ///      "$ref": "#/components/schemas/Volume"
    ///    },
    ///    {
    ///      "$ref": "#/components/schemas/PriceImprovement"
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    #[serde(untagged)]
    pub enum FeePolicy {
        Surplus(Surplus),
        Volume(Volume),
        PriceImprovement(PriceImprovement),
    }
    impl ::std::convert::From<Surplus> for FeePolicy {
        fn from(value: Surplus) -> Self {
            Self::Surplus(value)
        }
    }
    impl ::std::convert::From<Volume> for FeePolicy {
        fn from(value: Volume) -> Self {
            Self::Volume(value)
        }
    }
    impl ::std::convert::From<PriceImprovement> for FeePolicy {
        fn from(value: PriceImprovement) -> Self {
            Self::PriceImprovement(value)
        }
    }
    ///`GetOrdersResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "oneOf": [
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "order"
    ///      ],
    ///      "properties": {
    ///        "order": {
    ///          "$ref": "#/components/schemas/Order"
    ///        }
    ///      }
    ///    },
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "error"
    ///      ],
    ///      "properties": {
    ///        "error": {
    ///          "type": "object",
    ///          "required": [
    ///            "description",
    ///            "uid"
    ///          ],
    ///          "properties": {
    ///            "description": {
    ///              "type": "string"
    ///            },
    ///            "uid": {
    ///              "$ref": "#/components/schemas/UID"
    ///            }
    ///          }
    ///        }
    ///      }
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub enum GetOrdersResponse {
        #[serde(rename = "order")]
        Order(Order),
        #[serde(rename = "error")]
        Error {
            description: ::std::string::String,
            uid: Uid,
        },
    }
    impl ::std::convert::From<Order> for GetOrdersResponse {
        fn from(value: Order) -> Self {
            Self::Order(value)
        }
    }
    /**Represents a smart contract interaction that can be executed as part of an order's pre or post hooks.
     */
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Represents a smart contract interaction that can be executed as part of an order's pre or post hooks.\n",
    ///  "type": "object",
    ///  "required": [
    ///    "callData",
    ///    "target",
    ///    "value"
    ///  ],
    ///  "properties": {
    ///    "callData": {
    ///      "description": "The calldata to be sent to the target contract. Encoded as a hex string with `0x` prefix.\n",
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/CallData"
    ///        }
    ///      ]
    ///    },
    ///    "target": {
    ///      "description": "The address of the contract to call.",
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/Address"
    ///        }
    ///      ]
    ///    },
    ///    "value": {
    ///      "description": "The amount of native token (ETH, xDAI, etc.) in Wei to send with the interaction call.\n",
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/TokenAmount"
    ///        }
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct InteractionData {
        /**The calldata to be sent to the target contract. Encoded as a hex string with `0x` prefix.
         */
        #[serde(rename = "callData")]
        pub call_data: CallData,
        ///The address of the contract to call.
        pub target: Address,
        /**The amount of native token (ETH, xDAI, etc.) in Wei to send with the interaction call.
         */
        pub value: TokenAmount,
    }
    /**The estimated native price for the token
     */
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "The estimated native price for the token\n",
    ///  "type": "object",
    ///  "properties": {
    ///    "price": {
    ///      "description": "Estimated price of the token.",
    ///      "type": "number"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct NativePriceResponse {
        ///Estimated price of the token.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub price: ::std::option::Option<f64>,
    }
    impl ::std::default::Default for NativePriceResponse {
        fn default() -> Self {
            Self {
                price: Default::default(),
            }
        }
    }
    ///`OnchainOrderData`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "sender"
    ///  ],
    ///  "properties": {
    ///    "placementError": {
    ///      "description": "Describes the error, if the order placement was not successful. This could happen, for example, if the `validTo` is too high, or no valid quote was found or generated.\n",
    ///      "type": "string",
    ///      "enum": [
    ///        "QuoteNotFound",
    ///        "ValidToTooFarInFuture",
    ///        "PreValidationError"
    ///      ]
    ///    },
    ///    "sender": {
    ///      "description": "If orders are placed as on-chain orders, the owner of the order might be a smart contract, but not the user placing the order. The actual user will be provided in this field.\n",
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/Address"
    ///        }
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct OnchainOrderData {
        /**Describes the error, if the order placement was not successful. This could happen, for example, if the `validTo` is too high, or no valid quote was found or generated.
         */
        #[serde(
            rename = "placementError",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub placement_error: ::std::option::Option<OnchainOrderDataPlacementError>,
        /**If orders are placed as on-chain orders, the owner of the order might be a smart contract, but not the user placing the order. The actual user will be provided in this field.
         */
        pub sender: Address,
    }
    /**Describes the error, if the order placement was not successful. This could happen, for example, if the `validTo` is too high, or no valid quote was found or generated.
     */
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Describes the error, if the order placement was not successful. This could happen, for example, if the `validTo` is too high, or no valid quote was found or generated.\n",
    ///  "type": "string",
    ///  "enum": [
    ///    "QuoteNotFound",
    ///    "ValidToTooFarInFuture",
    ///    "PreValidationError"
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
    pub enum OnchainOrderDataPlacementError {
        QuoteNotFound,
        ValidToTooFarInFuture,
        PreValidationError,
    }
    impl ::std::fmt::Display for OnchainOrderDataPlacementError {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::QuoteNotFound => f.write_str("QuoteNotFound"),
                Self::ValidToTooFarInFuture => f.write_str("ValidToTooFarInFuture"),
                Self::PreValidationError => f.write_str("PreValidationError"),
            }
        }
    }
    impl ::std::str::FromStr for OnchainOrderDataPlacementError {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "QuoteNotFound" => Ok(Self::QuoteNotFound),
                "ValidToTooFarInFuture" => Ok(Self::ValidToTooFarInFuture),
                "PreValidationError" => Ok(Self::PreValidationError),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for OnchainOrderDataPlacementError {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String> for OnchainOrderDataPlacementError {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for OnchainOrderDataPlacementError {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    /**An order as returned by the API. Combines the order creation data, order metadata, and any associated interactions.
     */
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "An order as returned by the API. Combines the order creation data, order metadata, and any associated interactions.\n",
    ///  "allOf": [
    ///    {
    ///      "$ref": "#/components/schemas/OrderCreation"
    ///    },
    ///    {
    ///      "$ref": "#/components/schemas/OrderMetaData"
    ///    },
    ///    {
    ///      "type": "object",
    ///      "properties": {
    ///        "interactions": {
    ///          "description": "Optional pre and post interactions associated with the order. Pre-interactions are executed before the order's trade, and post-interactions are executed after.\n",
    ///          "type": "object",
    ///          "properties": {
    ///            "post": {
    ///              "description": "Interactions to be executed after the order's trade. These can be used for cleanup or follow-up operations.\n",
    ///              "type": "array",
    ///              "items": {
    ///                "$ref": "#/components/schemas/InteractionData"
    ///              }
    ///            },
    ///            "pre": {
    ///              "description": "Interactions to be executed before the order's trade. These can be used for setup operations like token approvals.\n",
    ///              "type": "array",
    ///              "items": {
    ///                "$ref": "#/components/schemas/InteractionData"
    ///              }
    ///            }
    ///          }
    ///        }
    ///      }
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct Order {
        /**This field comes in two forms for backward compatibility. The hash form will eventually stop being accepted.
         */
        #[serde(rename = "appData")]
        pub app_data: OrderAppData,
        /**May be set for debugging purposes. If set, this field is compared to what the backend internally calculates as the app data hash based on the contents of `appData`. If the hash does not match, an error is returned. If this field is set, then `appData` **MUST** be a string encoding of a JSON object.
         */
        #[serde(
            rename = "appDataHash",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub app_data_hash: ::std::option::Option<AppDataHash>,
        /**Unused field that is currently always set to `null` and will be removed in the future.
         */
        #[serde(
            rename = "availableBalance",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub available_balance: ::std::option::Option<TokenAmount>,
        ///see `OrderParameters::buyAmount`
        #[serde(rename = "buyAmount")]
        pub buy_amount: TokenAmount,
        ///see `OrderParameters::buyToken`
        #[serde(rename = "buyToken")]
        pub buy_token: Address,
        ///see `OrderParameters::buyTokenBalance`
        #[serde(
            rename = "buyTokenBalance",
            default = "defaults::order_buy_token_balance"
        )]
        pub buy_token_balance: BuyTokenDestination,
        /**The class of the order (market, limit, or liquidity). Determines how fees are handled.
         */
        pub class: OrderClass,
        ///Creation time of the order. Encoded as ISO 8601 UTC.
        #[serde(rename = "creationDate")]
        pub creation_date: ::std::string::String,
        /**Additional data specific to ethflow orders. Only present for orders placed through the EthFlow contract, which allows trading native ETH directly without wrapping to WETH first.
         */
        #[serde(
            rename = "ethflowData",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub ethflow_data: ::std::option::Option<EthflowData>,
        /**The total amount of `buyToken` that has been executed for this order.
         */
        #[serde(rename = "executedBuyAmount")]
        pub executed_buy_amount: BigUint,
        /**Total fee charged for execution of the order. Contains network fee and protocol fees. This takes into account the historic static fee signed by the user and the new dynamic fee computed by solvers.
         */
        #[serde(
            rename = "executedFee",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub executed_fee: ::std::option::Option<BigUint>,
        /**[DEPRECATED] The total amount of the user signed `fee` that have been executed for this order. This value is only non-negative for very old orders.
         */
        #[serde(rename = "executedFeeAmount")]
        pub executed_fee_amount: BigUint,
        ///Token the executed fee was captured in.
        #[serde(
            rename = "executedFeeToken",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub executed_fee_token: ::std::option::Option<Address>,
        /**The total amount of `sellToken` that has been transferred from the user for this order so far.
         */
        #[serde(rename = "executedSellAmount")]
        pub executed_sell_amount: BigUint,
        /**The total amount of `sellToken` that has been transferred from the user for this order so far minus tokens that were transferred as part of the signed `fee` of the order. This is only relevant for old orders because now all orders have a signed `fee` of 0 and solvers compute an appropriate fee dynamically at the time of the order execution.
         */
        #[serde(rename = "executedSellAmountBeforeFees")]
        pub executed_sell_amount_before_fees: BigUint,
        ///see `OrderParameters::feeAmount`
        #[serde(rename = "feeAmount")]
        pub fee_amount: TokenAmount,
        /**If set, the backend enforces that this address matches what is decoded as the *signer* of the signature. This helps catch errors with invalid signature encodings as the backend might otherwise silently work with an unexpected address that for example does not have any balance.
         */
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub from: ::std::option::Option<Address>,
        /**Full `appData`, which the contract-level `appData` is a hash of. See `OrderCreation` for more information.
         */
        #[serde(
            rename = "fullAppData",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub full_app_data: ::std::option::Option<::std::string::String>,
        /**If set to true, full sell amount will be checked during allowance and balance checking. This will ensure the account has correct allowance and available balance for the order to be created.
         */
        #[serde(rename = "fullBalanceCheck", default)]
        pub full_balance_check: bool,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub interactions: ::std::option::Option<OrderInteractions>,
        ///Has this order been invalidated?
        pub invalidated: bool,
        /**Liquidity orders are functionally the same as normal smart contract
        orders but are not placed with the intent of actively getting
        traded. Instead they facilitate the trade of normal orders by
        allowing them to be matched against liquidity orders which uses less
        gas and can have better prices than external liquidity.

        As such liquidity orders will only be used in order to improve
        settlement of normal orders. They should not be expected to be
        traded otherwise and should not expect to get surplus.*/
        #[serde(
            rename = "isLiquidityOrder",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub is_liquidity_order: ::std::option::Option<bool>,
        ///see `OrderParameters::kind`
        pub kind: OrderKind,
        /**There is some data only available for orders that are placed on-chain. This data can be found in this object.
         */
        #[serde(
            rename = "onchainOrderData",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub onchain_order_data: ::std::option::Option<OnchainOrderData>,
        /**This represents the actual trader of an on-chain order.
        ### ethflow orders
        In this case, the `owner` would be the `EthFlow` contract and *not* the actual trader.
        */
        #[serde(
            rename = "onchainUser",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub onchain_user: ::std::option::Option<Address>,
        /**The address that signed the order and owns it. For regular orders, this is the trader. For EIP 1271 orders, it's the respective contract (see `onchainUser` for the actual trader).
         */
        pub owner: Address,
        ///see `OrderParameters::partiallyFillable`
        #[serde(rename = "partiallyFillable")]
        pub partially_fillable: bool,
        /**If the order was created with a quote, this field contains the original quote data for reference. Includes gas estimation and pricing information captured at the time of quoting, which can be used to analyze order execution and calculate fees.
         */
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub quote: ::std::option::Option<StoredOrderQuote>,
        /**Orders can optionally include a quote ID. This way the order can be linked to a quote and enable providing more metadata when analysing order slippage.
         */
        #[serde(
            rename = "quoteId",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub quote_id: ::std::option::Option<i64>,
        ///see `OrderParameters::receiver`
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub receiver: ::std::option::Option<Address>,
        ///see `OrderParameters::sellAmount`
        #[serde(rename = "sellAmount")]
        pub sell_amount: TokenAmount,
        ///see `OrderParameters::sellToken`
        #[serde(rename = "sellToken")]
        pub sell_token: Address,
        ///see `OrderParameters::sellTokenBalance`
        #[serde(
            rename = "sellTokenBalance",
            default = "defaults::order_sell_token_balance"
        )]
        pub sell_token_balance: SellTokenSource,
        /**The address of the CoW Protocol settlement contract that this order is valid for. Orders are only valid on the settlement contract they were signed for.
         */
        #[serde(rename = "settlementContract")]
        pub settlement_contract: Address,
        pub signature: Signature,
        #[serde(rename = "signingScheme")]
        pub signing_scheme: SigningScheme,
        ///Order status.
        pub status: OrderStatus,
        /**Unique identifier of the order. Computed as the EIP-712 hash of the order data combined with the owner address and valid_to timestamp.
         */
        pub uid: Uid,
        ///see `OrderParameters::validTo`
        #[serde(rename = "validTo")]
        pub valid_to: i64,
    }
    /**This field comes in two forms for backward compatibility. The hash form will eventually stop being accepted.
     */
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "This field comes in two forms for backward compatibility. The hash form will eventually stop being accepted.\n",
    ///  "anyOf": [
    ///    {
    ///      "title": "Full App Data",
    ///      "description": "**Short**:\nIf you do not care about `appData`, set this field to `\"{}\"` and make sure that the order you signed for this request had its `appData` field set to `0xb48d38f93eaa084033fc5970bf96e559c33c4cdc07d889ab00b4d63f9590739d`.\n**Long**:\nA string encoding a JSON object like `\"{\"hello\":\"world\"}\"`.\nThis field determines the smart contract order's `appData` field, which is assumed to be set to the `keccak256` hash of the UTF-8 encoded bytes of this string. You must ensure that the signature that is part of this request indeed signed a smart contract order with the `appData` field set appropriately. If this isn't the case, signature verification will fail. For easier debugging it is recommended to additionally set the `appDataHash` field.\nThe field must be the encoding of a valid JSON object. The JSON object can contain arbitrary application specific data (JSON key values). The optional key `backend` is special. It **MUST** conform to the schema documented in `ProtocolAppData`.\nThe intended use of the other keys of the object is follow the standardized format defined [here](https://github.com/cowprotocol/app-data). Example:\n```json {\n  \"version\": \"0.7.0\",\n  \"appCode\": \"YOUR_APP_CODE\",\n  \"metadata\": {}\n} ```\nThe total byte size of this field's UTF-8 encoded bytes is limited to 1000.",
    ///      "type": "string",
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/AppData"
    ///        }
    ///      ]
    ///    },
    ///    {
    ///      "$ref": "#/components/schemas/AppDataHash"
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct OrderAppData {
        #[serde(
            flatten,
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub subtype_0: ::std::option::Option<AppData>,
        #[serde(
            flatten,
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub subtype_1: ::std::option::Option<AppDataHash>,
    }
    impl ::std::default::Default for OrderAppData {
        fn default() -> Self {
            Self {
                subtype_0: Default::default(),
                subtype_1: Default::default(),
            }
        }
    }
    /**[EIP-712](https://eips.ethereum.org/EIPS/eip-712) signature of struct
    `OrderCancellation(bytes orderUid)` from the order's owner.
    */
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "[EIP-712](https://eips.ethereum.org/EIPS/eip-712) signature of struct\n`OrderCancellation(bytes orderUid)` from the order's owner.\n",
    ///  "type": "object",
    ///  "required": [
    ///    "signature",
    ///    "signingScheme"
    ///  ],
    ///  "properties": {
    ///    "signature": {
    ///      "description": "OrderCancellation signed by owner",
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/EcdsaSignature"
    ///        }
    ///      ]
    ///    },
    ///    "signingScheme": {
    ///      "$ref": "#/components/schemas/EcdsaSigningScheme"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct OrderCancellation {
        ///OrderCancellation signed by owner
        pub signature: EcdsaSignature,
        #[serde(rename = "signingScheme")]
        pub signing_scheme: EcdsaSigningScheme,
    }
    ///`OrderCancellationError`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "description",
    ///    "errorType"
    ///  ],
    ///  "properties": {
    ///    "description": {
    ///      "type": "string"
    ///    },
    ///    "errorType": {
    ///      "type": "string",
    ///      "enum": [
    ///        "InvalidSignature",
    ///        "WrongOwner",
    ///        "OrderNotFound",
    ///        "AlreadyCancelled",
    ///        "OrderFullyExecuted",
    ///        "OrderExpired",
    ///        "OnChainOrder"
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct OrderCancellationError {
        pub description: ::std::string::String,
        #[serde(rename = "errorType")]
        pub error_type: OrderCancellationErrorErrorType,
    }
    ///`OrderCancellationErrorErrorType`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "InvalidSignature",
    ///    "WrongOwner",
    ///    "OrderNotFound",
    ///    "AlreadyCancelled",
    ///    "OrderFullyExecuted",
    ///    "OrderExpired",
    ///    "OnChainOrder"
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
    pub enum OrderCancellationErrorErrorType {
        InvalidSignature,
        WrongOwner,
        OrderNotFound,
        AlreadyCancelled,
        OrderFullyExecuted,
        OrderExpired,
        OnChainOrder,
    }
    impl ::std::fmt::Display for OrderCancellationErrorErrorType {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::InvalidSignature => f.write_str("InvalidSignature"),
                Self::WrongOwner => f.write_str("WrongOwner"),
                Self::OrderNotFound => f.write_str("OrderNotFound"),
                Self::AlreadyCancelled => f.write_str("AlreadyCancelled"),
                Self::OrderFullyExecuted => f.write_str("OrderFullyExecuted"),
                Self::OrderExpired => f.write_str("OrderExpired"),
                Self::OnChainOrder => f.write_str("OnChainOrder"),
            }
        }
    }
    impl ::std::str::FromStr for OrderCancellationErrorErrorType {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "InvalidSignature" => Ok(Self::InvalidSignature),
                "WrongOwner" => Ok(Self::WrongOwner),
                "OrderNotFound" => Ok(Self::OrderNotFound),
                "AlreadyCancelled" => Ok(Self::AlreadyCancelled),
                "OrderFullyExecuted" => Ok(Self::OrderFullyExecuted),
                "OrderExpired" => Ok(Self::OrderExpired),
                "OnChainOrder" => Ok(Self::OnChainOrder),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for OrderCancellationErrorErrorType {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String> for OrderCancellationErrorErrorType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for OrderCancellationErrorErrorType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    /**EIP-712 signature of struct OrderCancellations { orderUid: bytes[] } from the order's owner.
     */
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "EIP-712 signature of struct OrderCancellations { orderUid: bytes[] } from the order's owner.\n",
    ///  "type": "object",
    ///  "required": [
    ///    "signature",
    ///    "signingScheme"
    ///  ],
    ///  "properties": {
    ///    "orderUids": {
    ///      "description": "Up to 128 UIDs of orders to cancel.",
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/UID"
    ///      }
    ///    },
    ///    "signature": {
    ///      "description": "`OrderCancellation` signed by the owner.",
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/EcdsaSignature"
    ///        }
    ///      ]
    ///    },
    ///    "signingScheme": {
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/EcdsaSigningScheme"
    ///        }
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct OrderCancellations {
        ///Up to 128 UIDs of orders to cancel.
        #[serde(
            rename = "orderUids",
            default,
            skip_serializing_if = "::std::vec::Vec::is_empty"
        )]
        pub order_uids: ::std::vec::Vec<Uid>,
        ///`OrderCancellation` signed by the owner.
        pub signature: EcdsaSignature,
        #[serde(rename = "signingScheme")]
        pub signing_scheme: EcdsaSigningScheme,
    }
    ///Order class.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Order class.",
    ///  "type": "string",
    ///  "enum": [
    ///    "market",
    ///    "limit",
    ///    "liquidity"
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
    pub enum OrderClass {
        #[serde(rename = "market")]
        Market,
        #[serde(rename = "limit")]
        Limit,
        #[serde(rename = "liquidity")]
        Liquidity,
    }
    impl ::std::fmt::Display for OrderClass {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Market => f.write_str("market"),
                Self::Limit => f.write_str("limit"),
                Self::Liquidity => f.write_str("liquidity"),
            }
        }
    }
    impl ::std::str::FromStr for OrderClass {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "market" => Ok(Self::Market),
                "limit" => Ok(Self::Limit),
                "liquidity" => Ok(Self::Liquidity),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for OrderClass {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String> for OrderClass {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for OrderClass {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///Data a user provides when creating a new order.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Data a user provides when creating a new order.",
    ///  "type": "object",
    ///  "required": [
    ///    "appData",
    ///    "buyAmount",
    ///    "buyToken",
    ///    "feeAmount",
    ///    "kind",
    ///    "partiallyFillable",
    ///    "sellAmount",
    ///    "sellToken",
    ///    "signature",
    ///    "signingScheme",
    ///    "validTo"
    ///  ],
    ///  "properties": {
    ///    "appData": {
    ///      "description": "This field comes in two forms for backward compatibility. The hash form will eventually stop being accepted.\n",
    ///      "anyOf": [
    ///        {
    ///          "title": "Full App Data",
    ///          "description": "**Short**:\nIf you do not care about `appData`, set this field to `\"{}\"` and make sure that the order you signed for this request had its `appData` field set to `0xb48d38f93eaa084033fc5970bf96e559c33c4cdc07d889ab00b4d63f9590739d`.\n**Long**:\nA string encoding a JSON object like `\"{\"hello\":\"world\"}\"`.\nThis field determines the smart contract order's `appData` field, which is assumed to be set to the `keccak256` hash of the UTF-8 encoded bytes of this string. You must ensure that the signature that is part of this request indeed signed a smart contract order with the `appData` field set appropriately. If this isn't the case, signature verification will fail. For easier debugging it is recommended to additionally set the `appDataHash` field.\nThe field must be the encoding of a valid JSON object. The JSON object can contain arbitrary application specific data (JSON key values). The optional key `backend` is special. It **MUST** conform to the schema documented in `ProtocolAppData`.\nThe intended use of the other keys of the object is follow the standardized format defined [here](https://github.com/cowprotocol/app-data). Example:\n```json {\n  \"version\": \"0.7.0\",\n  \"appCode\": \"YOUR_APP_CODE\",\n  \"metadata\": {}\n} ```\nThe total byte size of this field's UTF-8 encoded bytes is limited to 1000.",
    ///          "type": "string",
    ///          "allOf": [
    ///            {
    ///              "$ref": "#/components/schemas/AppData"
    ///            }
    ///          ]
    ///        },
    ///        {
    ///          "$ref": "#/components/schemas/AppDataHash"
    ///        }
    ///      ]
    ///    },
    ///    "appDataHash": {
    ///      "description": "May be set for debugging purposes. If set, this field is compared to what the backend internally calculates as the app data hash based on the contents of `appData`. If the hash does not match, an error is returned. If this field is set, then `appData` **MUST** be a string encoding of a JSON object.\n",
    ///      "oneOf": [
    ///        {
    ///          "type": "null"
    ///        },
    ///        {
    ///          "allOf": [
    ///            {
    ///              "$ref": "#/components/schemas/AppDataHash"
    ///            }
    ///          ]
    ///        }
    ///      ]
    ///    },
    ///    "buyAmount": {
    ///      "description": "see `OrderParameters::buyAmount`",
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/TokenAmount"
    ///        }
    ///      ]
    ///    },
    ///    "buyToken": {
    ///      "description": "see `OrderParameters::buyToken`",
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/Address"
    ///        }
    ///      ]
    ///    },
    ///    "buyTokenBalance": {
    ///      "description": "see `OrderParameters::buyTokenBalance`",
    ///      "default": "erc20",
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/BuyTokenDestination"
    ///        }
    ///      ]
    ///    },
    ///    "feeAmount": {
    ///      "description": "see `OrderParameters::feeAmount`",
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/TokenAmount"
    ///        }
    ///      ]
    ///    },
    ///    "from": {
    ///      "description": "If set, the backend enforces that this address matches what is decoded as the *signer* of the signature. This helps catch errors with invalid signature encodings as the backend might otherwise silently work with an unexpected address that for example does not have any balance.\n",
    ///      "oneOf": [
    ///        {
    ///          "type": "null"
    ///        },
    ///        {
    ///          "allOf": [
    ///            {
    ///              "$ref": "#/components/schemas/Address"
    ///            }
    ///          ]
    ///        }
    ///      ]
    ///    },
    ///    "fullBalanceCheck": {
    ///      "description": "If set to true, full sell amount will be checked during allowance and balance checking. This will ensure the account has correct allowance and available balance for the order to be created.\n",
    ///      "default": false,
    ///      "type": "boolean"
    ///    },
    ///    "kind": {
    ///      "description": "see `OrderParameters::kind`",
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/OrderKind"
    ///        }
    ///      ]
    ///    },
    ///    "partiallyFillable": {
    ///      "description": "see `OrderParameters::partiallyFillable`",
    ///      "type": "boolean"
    ///    },
    ///    "quoteId": {
    ///      "description": "Orders can optionally include a quote ID. This way the order can be linked to a quote and enable providing more metadata when analysing order slippage.\n",
    ///      "type": [
    ///        "integer",
    ///        "null"
    ///      ]
    ///    },
    ///    "receiver": {
    ///      "description": "see `OrderParameters::receiver`",
    ///      "oneOf": [
    ///        {
    ///          "type": "null"
    ///        },
    ///        {
    ///          "allOf": [
    ///            {
    ///              "$ref": "#/components/schemas/Address"
    ///            }
    ///          ]
    ///        }
    ///      ]
    ///    },
    ///    "sellAmount": {
    ///      "description": "see `OrderParameters::sellAmount`",
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/TokenAmount"
    ///        }
    ///      ]
    ///    },
    ///    "sellToken": {
    ///      "description": "see `OrderParameters::sellToken`",
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/Address"
    ///        }
    ///      ]
    ///    },
    ///    "sellTokenBalance": {
    ///      "description": "see `OrderParameters::sellTokenBalance`",
    ///      "default": "erc20",
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/SellTokenSource"
    ///        }
    ///      ]
    ///    },
    ///    "signature": {
    ///      "$ref": "#/components/schemas/Signature"
    ///    },
    ///    "signingScheme": {
    ///      "$ref": "#/components/schemas/SigningScheme"
    ///    },
    ///    "validTo": {
    ///      "description": "see `OrderParameters::validTo`",
    ///      "type": "integer"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct OrderCreation {
        /**This field comes in two forms for backward compatibility. The hash form will eventually stop being accepted.
         */
        #[serde(rename = "appData")]
        pub app_data: OrderCreationAppData,
        /**May be set for debugging purposes. If set, this field is compared to what the backend internally calculates as the app data hash based on the contents of `appData`. If the hash does not match, an error is returned. If this field is set, then `appData` **MUST** be a string encoding of a JSON object.
         */
        #[serde(
            rename = "appDataHash",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub app_data_hash: ::std::option::Option<AppDataHash>,
        ///see `OrderParameters::buyAmount`
        #[serde(rename = "buyAmount")]
        pub buy_amount: TokenAmount,
        ///see `OrderParameters::buyToken`
        #[serde(rename = "buyToken")]
        pub buy_token: Address,
        ///see `OrderParameters::buyTokenBalance`
        #[serde(
            rename = "buyTokenBalance",
            default = "defaults::order_creation_buy_token_balance"
        )]
        pub buy_token_balance: BuyTokenDestination,
        ///see `OrderParameters::feeAmount`
        #[serde(rename = "feeAmount")]
        pub fee_amount: TokenAmount,
        /**If set, the backend enforces that this address matches what is decoded as the *signer* of the signature. This helps catch errors with invalid signature encodings as the backend might otherwise silently work with an unexpected address that for example does not have any balance.
         */
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub from: ::std::option::Option<Address>,
        /**If set to true, full sell amount will be checked during allowance and balance checking. This will ensure the account has correct allowance and available balance for the order to be created.
         */
        #[serde(rename = "fullBalanceCheck", default)]
        pub full_balance_check: bool,
        ///see `OrderParameters::kind`
        pub kind: OrderKind,
        ///see `OrderParameters::partiallyFillable`
        #[serde(rename = "partiallyFillable")]
        pub partially_fillable: bool,
        /**Orders can optionally include a quote ID. This way the order can be linked to a quote and enable providing more metadata when analysing order slippage.
         */
        #[serde(
            rename = "quoteId",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub quote_id: ::std::option::Option<i64>,
        ///see `OrderParameters::receiver`
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub receiver: ::std::option::Option<Address>,
        ///see `OrderParameters::sellAmount`
        #[serde(rename = "sellAmount")]
        pub sell_amount: TokenAmount,
        ///see `OrderParameters::sellToken`
        #[serde(rename = "sellToken")]
        pub sell_token: Address,
        ///see `OrderParameters::sellTokenBalance`
        #[serde(
            rename = "sellTokenBalance",
            default = "defaults::order_creation_sell_token_balance"
        )]
        pub sell_token_balance: SellTokenSource,
        pub signature: Signature,
        #[serde(rename = "signingScheme")]
        pub signing_scheme: SigningScheme,
        ///see `OrderParameters::validTo`
        #[serde(rename = "validTo")]
        pub valid_to: i64,
    }
    /**This field comes in two forms for backward compatibility. The hash form will eventually stop being accepted.
     */
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "This field comes in two forms for backward compatibility. The hash form will eventually stop being accepted.\n",
    ///  "anyOf": [
    ///    {
    ///      "title": "Full App Data",
    ///      "description": "**Short**:\nIf you do not care about `appData`, set this field to `\"{}\"` and make sure that the order you signed for this request had its `appData` field set to `0xb48d38f93eaa084033fc5970bf96e559c33c4cdc07d889ab00b4d63f9590739d`.\n**Long**:\nA string encoding a JSON object like `\"{\"hello\":\"world\"}\"`.\nThis field determines the smart contract order's `appData` field, which is assumed to be set to the `keccak256` hash of the UTF-8 encoded bytes of this string. You must ensure that the signature that is part of this request indeed signed a smart contract order with the `appData` field set appropriately. If this isn't the case, signature verification will fail. For easier debugging it is recommended to additionally set the `appDataHash` field.\nThe field must be the encoding of a valid JSON object. The JSON object can contain arbitrary application specific data (JSON key values). The optional key `backend` is special. It **MUST** conform to the schema documented in `ProtocolAppData`.\nThe intended use of the other keys of the object is follow the standardized format defined [here](https://github.com/cowprotocol/app-data). Example:\n```json {\n  \"version\": \"0.7.0\",\n  \"appCode\": \"YOUR_APP_CODE\",\n  \"metadata\": {}\n} ```\nThe total byte size of this field's UTF-8 encoded bytes is limited to 1000.",
    ///      "type": "string",
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/AppData"
    ///        }
    ///      ]
    ///    },
    ///    {
    ///      "$ref": "#/components/schemas/AppDataHash"
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct OrderCreationAppData {
        #[serde(
            flatten,
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub subtype_0: ::std::option::Option<AppData>,
        #[serde(
            flatten,
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub subtype_1: ::std::option::Option<AppDataHash>,
    }
    impl ::std::default::Default for OrderCreationAppData {
        fn default() -> Self {
            Self {
                subtype_0: Default::default(),
                subtype_1: Default::default(),
            }
        }
    }
    /**Optional pre and post interactions associated with the order. Pre-interactions are executed before the order's trade, and post-interactions are executed after.
     */
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Optional pre and post interactions associated with the order. Pre-interactions are executed before the order's trade, and post-interactions are executed after.\n",
    ///  "type": "object",
    ///  "properties": {
    ///    "post": {
    ///      "description": "Interactions to be executed after the order's trade. These can be used for cleanup or follow-up operations.\n",
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/InteractionData"
    ///      }
    ///    },
    ///    "pre": {
    ///      "description": "Interactions to be executed before the order's trade. These can be used for setup operations like token approvals.\n",
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/InteractionData"
    ///      }
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct OrderInteractions {
        /**Interactions to be executed after the order's trade. These can be used for cleanup or follow-up operations.
         */
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub post: ::std::vec::Vec<InteractionData>,
        /**Interactions to be executed before the order's trade. These can be used for setup operations like token approvals.
         */
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub pre: ::std::vec::Vec<InteractionData>,
    }
    impl ::std::default::Default for OrderInteractions {
        fn default() -> Self {
            Self {
                post: Default::default(),
                pre: Default::default(),
            }
        }
    }
    ///Is this order a buy or sell?
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Is this order a buy or sell?",
    ///  "type": "string",
    ///  "enum": [
    ///    "buy",
    ///    "sell"
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
    pub enum OrderKind {
        #[serde(rename = "buy")]
        Buy,
        #[serde(rename = "sell")]
        Sell,
    }
    impl ::std::fmt::Display for OrderKind {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Buy => f.write_str("buy"),
                Self::Sell => f.write_str("sell"),
            }
        }
    }
    impl ::std::str::FromStr for OrderKind {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "buy" => Ok(Self::Buy),
                "sell" => Ok(Self::Sell),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for OrderKind {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String> for OrderKind {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for OrderKind {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    /**Extra order data that is returned to users when querying orders but not provided by users when creating orders.
     */
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Extra order data that is returned to users when querying orders but not provided by users when creating orders.\n",
    ///  "type": "object",
    ///  "required": [
    ///    "class",
    ///    "creationDate",
    ///    "executedBuyAmount",
    ///    "executedFeeAmount",
    ///    "executedSellAmount",
    ///    "executedSellAmountBeforeFees",
    ///    "invalidated",
    ///    "owner",
    ///    "settlementContract",
    ///    "status",
    ///    "uid"
    ///  ],
    ///  "properties": {
    ///    "availableBalance": {
    ///      "description": "Unused field that is currently always set to `null` and will be removed in the future.\n",
    ///      "deprecated": true,
    ///      "oneOf": [
    ///        {
    ///          "type": "null"
    ///        },
    ///        {
    ///          "allOf": [
    ///            {
    ///              "$ref": "#/components/schemas/TokenAmount"
    ///            }
    ///          ]
    ///        }
    ///      ]
    ///    },
    ///    "class": {
    ///      "description": "The class of the order (market, limit, or liquidity). Determines how fees are handled.\n",
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/OrderClass"
    ///        }
    ///      ]
    ///    },
    ///    "creationDate": {
    ///      "description": "Creation time of the order. Encoded as ISO 8601 UTC.",
    ///      "examples": [
    ///        "2020-12-03T18:35:18.814523Z"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "ethflowData": {
    ///      "description": "Additional data specific to ethflow orders. Only present for orders placed through the EthFlow contract, which allows trading native ETH directly without wrapping to WETH first.\n",
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/EthflowData"
    ///        }
    ///      ]
    ///    },
    ///    "executedBuyAmount": {
    ///      "description": "The total amount of `buyToken` that has been executed for this order.\n",
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/BigUint"
    ///        }
    ///      ]
    ///    },
    ///    "executedFee": {
    ///      "description": "Total fee charged for execution of the order. Contains network fee and protocol fees. This takes into account the historic static fee signed by the user and the new dynamic fee computed by solvers.\n",
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/BigUint"
    ///        }
    ///      ]
    ///    },
    ///    "executedFeeAmount": {
    ///      "description": "[DEPRECATED] The total amount of the user signed `fee` that have been executed for this order. This value is only non-negative for very old orders.\n",
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/BigUint"
    ///        }
    ///      ]
    ///    },
    ///    "executedFeeToken": {
    ///      "description": "Token the executed fee was captured in.",
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/Address"
    ///        }
    ///      ]
    ///    },
    ///    "executedSellAmount": {
    ///      "description": "The total amount of `sellToken` that has been transferred from the user for this order so far.\n",
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/BigUint"
    ///        }
    ///      ]
    ///    },
    ///    "executedSellAmountBeforeFees": {
    ///      "description": "The total amount of `sellToken` that has been transferred from the user for this order so far minus tokens that were transferred as part of the signed `fee` of the order. This is only relevant for old orders because now all orders have a signed `fee` of 0 and solvers compute an appropriate fee dynamically at the time of the order execution.\n",
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/BigUint"
    ///        }
    ///      ]
    ///    },
    ///    "fullAppData": {
    ///      "description": "Full `appData`, which the contract-level `appData` is a hash of. See `OrderCreation` for more information.\n",
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "invalidated": {
    ///      "description": "Has this order been invalidated?",
    ///      "type": "boolean"
    ///    },
    ///    "isLiquidityOrder": {
    ///      "description": "Liquidity orders are functionally the same as normal smart contract\norders but are not placed with the intent of actively getting\ntraded. Instead they facilitate the trade of normal orders by\nallowing them to be matched against liquidity orders which uses less\ngas and can have better prices than external liquidity.\n\nAs such liquidity orders will only be used in order to improve\nsettlement of normal orders. They should not be expected to be\ntraded otherwise and should not expect to get surplus.",
    ///      "type": "boolean"
    ///    },
    ///    "onchainOrderData": {
    ///      "description": "There is some data only available for orders that are placed on-chain. This data can be found in this object.\n",
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/OnchainOrderData"
    ///        }
    ///      ]
    ///    },
    ///    "onchainUser": {
    ///      "description": "This represents the actual trader of an on-chain order.\n### ethflow orders\nIn this case, the `owner` would be the `EthFlow` contract and *not* the actual trader.\n",
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/Address"
    ///        }
    ///      ]
    ///    },
    ///    "owner": {
    ///      "description": "The address that signed the order and owns it. For regular orders, this is the trader. For EIP 1271 orders, it's the respective contract (see `onchainUser` for the actual trader).\n",
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/Address"
    ///        }
    ///      ]
    ///    },
    ///    "quote": {
    ///      "description": "If the order was created with a quote, this field contains the original quote data for reference. Includes gas estimation and pricing information captured at the time of quoting, which can be used to analyze order execution and calculate fees.\n",
    ///      "oneOf": [
    ///        {
    ///          "type": "null"
    ///        },
    ///        {
    ///          "allOf": [
    ///            {
    ///              "$ref": "#/components/schemas/StoredOrderQuote"
    ///            }
    ///          ]
    ///        }
    ///      ]
    ///    },
    ///    "settlementContract": {
    ///      "description": "The address of the CoW Protocol settlement contract that this order is valid for. Orders are only valid on the settlement contract they were signed for.\n",
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/Address"
    ///        }
    ///      ]
    ///    },
    ///    "status": {
    ///      "description": "Order status.",
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/OrderStatus"
    ///        }
    ///      ]
    ///    },
    ///    "uid": {
    ///      "description": "Unique identifier of the order. Computed as the EIP-712 hash of the order data combined with the owner address and valid_to timestamp.\n",
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/UID"
    ///        }
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct OrderMetaData {
        /**Unused field that is currently always set to `null` and will be removed in the future.
         */
        #[serde(
            rename = "availableBalance",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub available_balance: ::std::option::Option<TokenAmount>,
        /**The class of the order (market, limit, or liquidity). Determines how fees are handled.
         */
        pub class: OrderClass,
        ///Creation time of the order. Encoded as ISO 8601 UTC.
        #[serde(rename = "creationDate")]
        pub creation_date: ::std::string::String,
        /**Additional data specific to ethflow orders. Only present for orders placed through the EthFlow contract, which allows trading native ETH directly without wrapping to WETH first.
         */
        #[serde(
            rename = "ethflowData",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub ethflow_data: ::std::option::Option<EthflowData>,
        /**The total amount of `buyToken` that has been executed for this order.
         */
        #[serde(rename = "executedBuyAmount")]
        pub executed_buy_amount: BigUint,
        /**Total fee charged for execution of the order. Contains network fee and protocol fees. This takes into account the historic static fee signed by the user and the new dynamic fee computed by solvers.
         */
        #[serde(
            rename = "executedFee",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub executed_fee: ::std::option::Option<BigUint>,
        /**[DEPRECATED] The total amount of the user signed `fee` that have been executed for this order. This value is only non-negative for very old orders.
         */
        #[serde(rename = "executedFeeAmount")]
        pub executed_fee_amount: BigUint,
        ///Token the executed fee was captured in.
        #[serde(
            rename = "executedFeeToken",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub executed_fee_token: ::std::option::Option<Address>,
        /**The total amount of `sellToken` that has been transferred from the user for this order so far.
         */
        #[serde(rename = "executedSellAmount")]
        pub executed_sell_amount: BigUint,
        /**The total amount of `sellToken` that has been transferred from the user for this order so far minus tokens that were transferred as part of the signed `fee` of the order. This is only relevant for old orders because now all orders have a signed `fee` of 0 and solvers compute an appropriate fee dynamically at the time of the order execution.
         */
        #[serde(rename = "executedSellAmountBeforeFees")]
        pub executed_sell_amount_before_fees: BigUint,
        /**Full `appData`, which the contract-level `appData` is a hash of. See `OrderCreation` for more information.
         */
        #[serde(
            rename = "fullAppData",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub full_app_data: ::std::option::Option<::std::string::String>,
        ///Has this order been invalidated?
        pub invalidated: bool,
        /**Liquidity orders are functionally the same as normal smart contract
        orders but are not placed with the intent of actively getting
        traded. Instead they facilitate the trade of normal orders by
        allowing them to be matched against liquidity orders which uses less
        gas and can have better prices than external liquidity.

        As such liquidity orders will only be used in order to improve
        settlement of normal orders. They should not be expected to be
        traded otherwise and should not expect to get surplus.*/
        #[serde(
            rename = "isLiquidityOrder",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub is_liquidity_order: ::std::option::Option<bool>,
        /**There is some data only available for orders that are placed on-chain. This data can be found in this object.
         */
        #[serde(
            rename = "onchainOrderData",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub onchain_order_data: ::std::option::Option<OnchainOrderData>,
        /**This represents the actual trader of an on-chain order.
        ### ethflow orders
        In this case, the `owner` would be the `EthFlow` contract and *not* the actual trader.
        */
        #[serde(
            rename = "onchainUser",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub onchain_user: ::std::option::Option<Address>,
        /**The address that signed the order and owns it. For regular orders, this is the trader. For EIP 1271 orders, it's the respective contract (see `onchainUser` for the actual trader).
         */
        pub owner: Address,
        /**If the order was created with a quote, this field contains the original quote data for reference. Includes gas estimation and pricing information captured at the time of quoting, which can be used to analyze order execution and calculate fees.
         */
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub quote: ::std::option::Option<StoredOrderQuote>,
        /**The address of the CoW Protocol settlement contract that this order is valid for. Orders are only valid on the settlement contract they were signed for.
         */
        #[serde(rename = "settlementContract")]
        pub settlement_contract: Address,
        ///Order status.
        pub status: OrderStatus,
        /**Unique identifier of the order. Computed as the EIP-712 hash of the order data combined with the owner address and valid_to timestamp.
         */
        pub uid: Uid,
    }
    ///Order parameters.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Order parameters.",
    ///  "type": "object",
    ///  "required": [
    ///    "appData",
    ///    "buyAmount",
    ///    "buyToken",
    ///    "feeAmount",
    ///    "gasAmount",
    ///    "gasPrice",
    ///    "kind",
    ///    "partiallyFillable",
    ///    "sellAmount",
    ///    "sellToken",
    ///    "sellTokenPrice",
    ///    "validTo"
    ///  ],
    ///  "properties": {
    ///    "appData": {
    ///      "description": "The app data associated with the order. In quote responses, this can be either the full app data JSON string or the app data hash, depending on what was provided in the quote request.\n",
    ///      "oneOf": [
    ///        {
    ///          "$ref": "#/components/schemas/AppData"
    ///        },
    ///        {
    ///          "$ref": "#/components/schemas/AppDataHash"
    ///        }
    ///      ]
    ///    },
    ///    "appDataHash": {
    ///      "description": "The hash of the app data. Only present when the full app data is also provided in the `appData` field.\n",
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/AppDataHash"
    ///        }
    ///      ]
    ///    },
    ///    "buyAmount": {
    ///      "description": "Amount of `buyToken` to be bought in atoms.",
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/TokenAmount"
    ///        }
    ///      ]
    ///    },
    ///    "buyToken": {
    ///      "description": "ERC-20 token to be bought.",
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/Address"
    ///        }
    ///      ]
    ///    },
    ///    "buyTokenBalance": {
    ///      "description": "Where the buy token should be transferred to. Defaults to `erc20` for standard ERC-20 token transfers.\n",
    ///      "default": "erc20",
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/BuyTokenDestination"
    ///        }
    ///      ]
    ///    },
    ///    "feeAmount": {
    ///      "description": "The fee amount in sell token atoms. For quote responses, this represents\nthe estimated network fee, calculated as:\n`feeAmount = ceil((gasAmount * gasPrice) / sellTokenPrice)`.\n\nWhen creating an order, this should be set to zero as fees are now\ncomputed dynamically by solvers.\n",
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/TokenAmount"
    ///        }
    ///      ]
    ///    },
    ///    "gasAmount": {
    ///      "description": "The estimated gas units required to execute the quoted trade.\n",
    ///      "examples": [
    ///        "150000"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "gasPrice": {
    ///      "description": "The estimated gas price at the time of quoting, measured in Wei per gas unit.\n",
    ///      "examples": [
    ///        "15000000000"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "kind": {
    ///      "description": "The kind is either a buy or sell order.",
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/OrderKind"
    ///        }
    ///      ]
    ///    },
    ///    "partiallyFillable": {
    ///      "description": "Is the order fill-or-kill or partially fillable?",
    ///      "type": "boolean"
    ///    },
    ///    "receiver": {
    ///      "description": "An optional Ethereum address to receive the proceeds of the trade instead of the owner (i.e. the order signer).\n",
    ///      "oneOf": [
    ///        {
    ///          "type": "null"
    ///        },
    ///        {
    ///          "allOf": [
    ///            {
    ///              "$ref": "#/components/schemas/Address"
    ///            }
    ///          ]
    ///        }
    ///      ]
    ///    },
    ///    "sellAmount": {
    ///      "description": "Amount of `sellToken` to be sold in atoms.",
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/TokenAmount"
    ///        }
    ///      ]
    ///    },
    ///    "sellToken": {
    ///      "description": "ERC-20 token to be sold.",
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/Address"
    ///        }
    ///      ]
    ///    },
    ///    "sellTokenBalance": {
    ///      "description": "Where the sell token should be drawn from. Defaults to `erc20` for standard ERC-20 token transfers.\n",
    ///      "default": "erc20",
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/SellTokenSource"
    ///        }
    ///      ]
    ///    },
    ///    "sellTokenPrice": {
    ///      "description": "Represents how much one atomic unit of the sell token is worth\nin the network's native token (in Wei or the equivalent atom).\n",
    ///      "examples": [
    ///        "0.0004"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "signingScheme": {
    ///      "description": "The signing scheme to use for the order. Defaults to `eip712` for standard typed data signing.\n",
    ///      "default": "eip712",
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/SigningScheme"
    ///        }
    ///      ]
    ///    },
    ///    "validTo": {
    ///      "description": "Unix timestamp (`uint32`) until which the order is valid.",
    ///      "type": "integer"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct OrderParameters {
        /**The app data associated with the order. In quote responses, this can be either the full app data JSON string or the app data hash, depending on what was provided in the quote request.
         */
        #[serde(rename = "appData")]
        pub app_data: OrderParametersAppData,
        /**The hash of the app data. Only present when the full app data is also provided in the `appData` field.
         */
        #[serde(
            rename = "appDataHash",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub app_data_hash: ::std::option::Option<AppDataHash>,
        ///Amount of `buyToken` to be bought in atoms.
        #[serde(rename = "buyAmount")]
        pub buy_amount: TokenAmount,
        ///ERC-20 token to be bought.
        #[serde(rename = "buyToken")]
        pub buy_token: Address,
        /**Where the buy token should be transferred to. Defaults to `erc20` for standard ERC-20 token transfers.
         */
        #[serde(
            rename = "buyTokenBalance",
            default = "defaults::order_parameters_buy_token_balance"
        )]
        pub buy_token_balance: BuyTokenDestination,
        /**The fee amount in sell token atoms. For quote responses, this represents
        the estimated network fee, calculated as:
        `feeAmount = ceil((gasAmount * gasPrice) / sellTokenPrice)`.

        When creating an order, this should be set to zero as fees are now
        computed dynamically by solvers.
        */
        #[serde(rename = "feeAmount")]
        pub fee_amount: TokenAmount,
        /**The estimated gas units required to execute the quoted trade.
         */
        #[serde(rename = "gasAmount")]
        pub gas_amount: ::std::string::String,
        /**The estimated gas price at the time of quoting, measured in Wei per gas unit.
         */
        #[serde(rename = "gasPrice")]
        pub gas_price: ::std::string::String,
        ///The kind is either a buy or sell order.
        pub kind: OrderKind,
        ///Is the order fill-or-kill or partially fillable?
        #[serde(rename = "partiallyFillable")]
        pub partially_fillable: bool,
        /**An optional Ethereum address to receive the proceeds of the trade instead of the owner (i.e. the order signer).
         */
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub receiver: ::std::option::Option<Address>,
        ///Amount of `sellToken` to be sold in atoms.
        #[serde(rename = "sellAmount")]
        pub sell_amount: TokenAmount,
        ///ERC-20 token to be sold.
        #[serde(rename = "sellToken")]
        pub sell_token: Address,
        /**Where the sell token should be drawn from. Defaults to `erc20` for standard ERC-20 token transfers.
         */
        #[serde(
            rename = "sellTokenBalance",
            default = "defaults::order_parameters_sell_token_balance"
        )]
        pub sell_token_balance: SellTokenSource,
        /**Represents how much one atomic unit of the sell token is worth
        in the network's native token (in Wei or the equivalent atom).
        */
        #[serde(rename = "sellTokenPrice")]
        pub sell_token_price: ::std::string::String,
        /**The signing scheme to use for the order. Defaults to `eip712` for standard typed data signing.
         */
        #[serde(
            rename = "signingScheme",
            default = "defaults::order_parameters_signing_scheme"
        )]
        pub signing_scheme: SigningScheme,
        ///Unix timestamp (`uint32`) until which the order is valid.
        #[serde(rename = "validTo")]
        pub valid_to: i64,
    }
    /**The app data associated with the order. In quote responses, this can be either the full app data JSON string or the app data hash, depending on what was provided in the quote request.
     */
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "The app data associated with the order. In quote responses, this can be either the full app data JSON string or the app data hash, depending on what was provided in the quote request.\n",
    ///  "oneOf": [
    ///    {
    ///      "$ref": "#/components/schemas/AppData"
    ///    },
    ///    {
    ///      "$ref": "#/components/schemas/AppDataHash"
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    #[serde(untagged)]
    pub enum OrderParametersAppData {
        Variant0(AppData),
        Variant1(AppDataHash),
    }
    impl ::std::fmt::Display for OrderParametersAppData {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                Self::Variant0(x) => x.fmt(f),
                Self::Variant1(x) => x.fmt(f),
            }
        }
    }
    impl ::std::convert::From<AppData> for OrderParametersAppData {
        fn from(value: AppData) -> Self {
            Self::Variant0(value)
        }
    }
    impl ::std::convert::From<AppDataHash> for OrderParametersAppData {
        fn from(value: AppDataHash) -> Self {
            Self::Variant1(value)
        }
    }
    ///`OrderPostError`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "description",
    ///    "errorType"
    ///  ],
    ///  "properties": {
    ///    "description": {
    ///      "type": "string"
    ///    },
    ///    "errorType": {
    ///      "type": "string",
    ///      "enum": [
    ///        "DuplicatedOrder",
    ///        "QuoteNotFound",
    ///        "QuoteNotVerified",
    ///        "InvalidQuote",
    ///        "MissingFrom",
    ///        "WrongOwner",
    ///        "InvalidEip1271Signature",
    ///        "InsufficientBalance",
    ///        "InsufficientAllowance",
    ///        "InvalidSignature",
    ///        "SellAmountOverflow",
    ///        "TransferSimulationFailed",
    ///        "ZeroAmount",
    ///        "IncompatibleSigningScheme",
    ///        "TooManyLimitOrders",
    ///        "TooMuchGas",
    ///        "UnsupportedBuyTokenDestination",
    ///        "UnsupportedSellTokenSource",
    ///        "UnsupportedOrderType",
    ///        "InsufficientValidTo",
    ///        "ExcessiveValidTo",
    ///        "InvalidNativeSellToken",
    ///        "SameBuyAndSellToken",
    ///        "UnsupportedToken",
    ///        "InvalidAppData",
    ///        "AppDataHashMismatch",
    ///        "AppDataMismatch",
    ///        "AppdataFromMismatch",
    ///        "MetadataSerializationFailed",
    ///        "OldOrderActivelyBidOn"
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct OrderPostError {
        pub description: ::std::string::String,
        #[serde(rename = "errorType")]
        pub error_type: OrderPostErrorErrorType,
    }
    ///`OrderPostErrorErrorType`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "DuplicatedOrder",
    ///    "QuoteNotFound",
    ///    "QuoteNotVerified",
    ///    "InvalidQuote",
    ///    "MissingFrom",
    ///    "WrongOwner",
    ///    "InvalidEip1271Signature",
    ///    "InsufficientBalance",
    ///    "InsufficientAllowance",
    ///    "InvalidSignature",
    ///    "SellAmountOverflow",
    ///    "TransferSimulationFailed",
    ///    "ZeroAmount",
    ///    "IncompatibleSigningScheme",
    ///    "TooManyLimitOrders",
    ///    "TooMuchGas",
    ///    "UnsupportedBuyTokenDestination",
    ///    "UnsupportedSellTokenSource",
    ///    "UnsupportedOrderType",
    ///    "InsufficientValidTo",
    ///    "ExcessiveValidTo",
    ///    "InvalidNativeSellToken",
    ///    "SameBuyAndSellToken",
    ///    "UnsupportedToken",
    ///    "InvalidAppData",
    ///    "AppDataHashMismatch",
    ///    "AppDataMismatch",
    ///    "AppdataFromMismatch",
    ///    "MetadataSerializationFailed",
    ///    "OldOrderActivelyBidOn"
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
    pub enum OrderPostErrorErrorType {
        DuplicatedOrder,
        QuoteNotFound,
        QuoteNotVerified,
        InvalidQuote,
        MissingFrom,
        WrongOwner,
        InvalidEip1271Signature,
        InsufficientBalance,
        InsufficientAllowance,
        InvalidSignature,
        SellAmountOverflow,
        TransferSimulationFailed,
        ZeroAmount,
        IncompatibleSigningScheme,
        TooManyLimitOrders,
        TooMuchGas,
        UnsupportedBuyTokenDestination,
        UnsupportedSellTokenSource,
        UnsupportedOrderType,
        InsufficientValidTo,
        ExcessiveValidTo,
        InvalidNativeSellToken,
        SameBuyAndSellToken,
        UnsupportedToken,
        InvalidAppData,
        AppDataHashMismatch,
        AppDataMismatch,
        AppdataFromMismatch,
        MetadataSerializationFailed,
        OldOrderActivelyBidOn,
    }
    impl ::std::fmt::Display for OrderPostErrorErrorType {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::DuplicatedOrder => f.write_str("DuplicatedOrder"),
                Self::QuoteNotFound => f.write_str("QuoteNotFound"),
                Self::QuoteNotVerified => f.write_str("QuoteNotVerified"),
                Self::InvalidQuote => f.write_str("InvalidQuote"),
                Self::MissingFrom => f.write_str("MissingFrom"),
                Self::WrongOwner => f.write_str("WrongOwner"),
                Self::InvalidEip1271Signature => f.write_str("InvalidEip1271Signature"),
                Self::InsufficientBalance => f.write_str("InsufficientBalance"),
                Self::InsufficientAllowance => f.write_str("InsufficientAllowance"),
                Self::InvalidSignature => f.write_str("InvalidSignature"),
                Self::SellAmountOverflow => f.write_str("SellAmountOverflow"),
                Self::TransferSimulationFailed => f.write_str("TransferSimulationFailed"),
                Self::ZeroAmount => f.write_str("ZeroAmount"),
                Self::IncompatibleSigningScheme => f.write_str("IncompatibleSigningScheme"),
                Self::TooManyLimitOrders => f.write_str("TooManyLimitOrders"),
                Self::TooMuchGas => f.write_str("TooMuchGas"),
                Self::UnsupportedBuyTokenDestination => {
                    f.write_str("UnsupportedBuyTokenDestination")
                }
                Self::UnsupportedSellTokenSource => f.write_str("UnsupportedSellTokenSource"),
                Self::UnsupportedOrderType => f.write_str("UnsupportedOrderType"),
                Self::InsufficientValidTo => f.write_str("InsufficientValidTo"),
                Self::ExcessiveValidTo => f.write_str("ExcessiveValidTo"),
                Self::InvalidNativeSellToken => f.write_str("InvalidNativeSellToken"),
                Self::SameBuyAndSellToken => f.write_str("SameBuyAndSellToken"),
                Self::UnsupportedToken => f.write_str("UnsupportedToken"),
                Self::InvalidAppData => f.write_str("InvalidAppData"),
                Self::AppDataHashMismatch => f.write_str("AppDataHashMismatch"),
                Self::AppDataMismatch => f.write_str("AppDataMismatch"),
                Self::AppdataFromMismatch => f.write_str("AppdataFromMismatch"),
                Self::MetadataSerializationFailed => f.write_str("MetadataSerializationFailed"),
                Self::OldOrderActivelyBidOn => f.write_str("OldOrderActivelyBidOn"),
            }
        }
    }
    impl ::std::str::FromStr for OrderPostErrorErrorType {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "DuplicatedOrder" => Ok(Self::DuplicatedOrder),
                "QuoteNotFound" => Ok(Self::QuoteNotFound),
                "QuoteNotVerified" => Ok(Self::QuoteNotVerified),
                "InvalidQuote" => Ok(Self::InvalidQuote),
                "MissingFrom" => Ok(Self::MissingFrom),
                "WrongOwner" => Ok(Self::WrongOwner),
                "InvalidEip1271Signature" => Ok(Self::InvalidEip1271Signature),
                "InsufficientBalance" => Ok(Self::InsufficientBalance),
                "InsufficientAllowance" => Ok(Self::InsufficientAllowance),
                "InvalidSignature" => Ok(Self::InvalidSignature),
                "SellAmountOverflow" => Ok(Self::SellAmountOverflow),
                "TransferSimulationFailed" => Ok(Self::TransferSimulationFailed),
                "ZeroAmount" => Ok(Self::ZeroAmount),
                "IncompatibleSigningScheme" => Ok(Self::IncompatibleSigningScheme),
                "TooManyLimitOrders" => Ok(Self::TooManyLimitOrders),
                "TooMuchGas" => Ok(Self::TooMuchGas),
                "UnsupportedBuyTokenDestination" => Ok(Self::UnsupportedBuyTokenDestination),
                "UnsupportedSellTokenSource" => Ok(Self::UnsupportedSellTokenSource),
                "UnsupportedOrderType" => Ok(Self::UnsupportedOrderType),
                "InsufficientValidTo" => Ok(Self::InsufficientValidTo),
                "ExcessiveValidTo" => Ok(Self::ExcessiveValidTo),
                "InvalidNativeSellToken" => Ok(Self::InvalidNativeSellToken),
                "SameBuyAndSellToken" => Ok(Self::SameBuyAndSellToken),
                "UnsupportedToken" => Ok(Self::UnsupportedToken),
                "InvalidAppData" => Ok(Self::InvalidAppData),
                "AppDataHashMismatch" => Ok(Self::AppDataHashMismatch),
                "AppDataMismatch" => Ok(Self::AppDataMismatch),
                "AppdataFromMismatch" => Ok(Self::AppdataFromMismatch),
                "MetadataSerializationFailed" => Ok(Self::MetadataSerializationFailed),
                "OldOrderActivelyBidOn" => Ok(Self::OldOrderActivelyBidOn),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for OrderPostErrorErrorType {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String> for OrderPostErrorErrorType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for OrderPostErrorErrorType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///Request fee and price quote.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Request fee and price quote.",
    ///  "allOf": [
    ///    {
    ///      "$ref": "#/components/schemas/OrderQuoteSide"
    ///    },
    ///    {
    ///      "$ref": "#/components/schemas/OrderQuoteValidity"
    ///    },
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "buyToken",
    ///        "from",
    ///        "sellToken"
    ///      ],
    ///      "properties": {
    ///        "appData": {
    ///          "description": "AppData which will be assigned to the order.\n\nExpects either a string JSON doc as defined on\n[AppData](https://github.com/cowprotocol/app-data) or a hex\nencoded string for backwards compatibility.\n\nWhen the first format is used, it's possible to provide the\nderived appDataHash field.",
    ///          "oneOf": [
    ///            {
    ///              "$ref": "#/components/schemas/AppData"
    ///            },
    ///            {
    ///              "$ref": "#/components/schemas/AppDataHash"
    ///            }
    ///          ]
    ///        },
    ///        "appDataHash": {
    ///          "description": "The hash of the stringified JSON appData doc.\n\nIf present, `appData` field must be set with the aforementioned\ndata where this hash is derived from.\n\nIn case they differ, the call will fail.",
    ///          "anyOf": [
    ///            {
    ///              "$ref": "#/components/schemas/AppDataHash"
    ///            }
    ///          ]
    ///        },
    ///        "buyToken": {
    ///          "description": "ERC-20 token to be bought",
    ///          "allOf": [
    ///            {
    ///              "$ref": "#/components/schemas/Address"
    ///            }
    ///          ]
    ///        },
    ///        "buyTokenBalance": {
    ///          "default": "erc20",
    ///          "allOf": [
    ///            {
    ///              "$ref": "#/components/schemas/BuyTokenDestination"
    ///            }
    ///          ]
    ///        },
    ///        "from": {
    ///          "$ref": "#/components/schemas/Address"
    ///        },
    ///        "onchainOrder": {
    ///          "description": "Flag to signal whether the order is intended for on-chain order placement. Only valid for non ECDSA-signed orders.\"\n",
    ///          "default": false
    ///        },
    ///        "priceQuality": {
    ///          "default": "verified",
    ///          "allOf": [
    ///            {
    ///              "$ref": "#/components/schemas/PriceQuality"
    ///            }
    ///          ]
    ///        },
    ///        "receiver": {
    ///          "description": "An optional address to receive the proceeds of the trade instead of the\n`owner` (i.e. the order signer).\n",
    ///          "oneOf": [
    ///            {
    ///              "type": "null"
    ///            },
    ///            {
    ///              "allOf": [
    ///                {
    ///                  "$ref": "#/components/schemas/Address"
    ///                }
    ///              ]
    ///            }
    ///          ]
    ///        },
    ///        "sellToken": {
    ///          "description": "ERC-20 token to be sold",
    ///          "allOf": [
    ///            {
    ///              "$ref": "#/components/schemas/Address"
    ///            }
    ///          ]
    ///        },
    ///        "sellTokenBalance": {
    ///          "default": "erc20",
    ///          "allOf": [
    ///            {
    ///              "$ref": "#/components/schemas/SellTokenSource"
    ///            }
    ///          ]
    ///        },
    ///        "signingScheme": {
    ///          "default": "eip712",
    ///          "allOf": [
    ///            {
    ///              "$ref": "#/components/schemas/SigningScheme"
    ///            }
    ///          ]
    ///        },
    ///        "timeout": {
    ///          "description": "User provided timeout in milliseconds. If no value is provided the systems default quote timeout will be used. Values get capped at a generous maximum timeout. Note that reducing the timeout can result in worse quotes because it might be too short for some price estimators.\n",
    ///          "type": "integer"
    ///        }
    ///      }
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    #[serde(untagged)]
    pub enum OrderQuoteRequest {
        Variant0(OrderQuoteRequestVariant0),
        Variant1(OrderQuoteRequestVariant1),
        Variant2(OrderQuoteRequestVariant2),
    }
    impl ::std::convert::From<OrderQuoteRequestVariant0> for OrderQuoteRequest {
        fn from(value: OrderQuoteRequestVariant0) -> Self {
            Self::Variant0(value)
        }
    }
    impl ::std::convert::From<OrderQuoteRequestVariant1> for OrderQuoteRequest {
        fn from(value: OrderQuoteRequestVariant1) -> Self {
            Self::Variant1(value)
        }
    }
    impl ::std::convert::From<OrderQuoteRequestVariant2> for OrderQuoteRequest {
        fn from(value: OrderQuoteRequestVariant2) -> Self {
            Self::Variant2(value)
        }
    }
    ///`OrderQuoteRequestVariant0`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "allOf": [
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "buyToken",
    ///        "from",
    ///        "sellToken"
    ///      ],
    ///      "properties": {
    ///        "appData": {
    ///          "description": "AppData which will be assigned to the order.\n\nExpects either a string JSON doc as defined on\n[AppData](https://github.com/cowprotocol/app-data) or a hex\nencoded string for backwards compatibility.\n\nWhen the first format is used, it's possible to provide the\nderived appDataHash field.",
    ///          "oneOf": [
    ///            {
    ///              "$ref": "#/components/schemas/AppData"
    ///            },
    ///            {
    ///              "$ref": "#/components/schemas/AppDataHash"
    ///            }
    ///          ]
    ///        },
    ///        "appDataHash": {
    ///          "description": "The hash of the stringified JSON appData doc.\n\nIf present, `appData` field must be set with the aforementioned\ndata where this hash is derived from.\n\nIn case they differ, the call will fail.",
    ///          "anyOf": [
    ///            {
    ///              "$ref": "#/components/schemas/AppDataHash"
    ///            }
    ///          ]
    ///        },
    ///        "buyToken": {
    ///          "description": "ERC-20 token to be bought",
    ///          "allOf": [
    ///            {
    ///              "$ref": "#/components/schemas/Address"
    ///            }
    ///          ]
    ///        },
    ///        "buyTokenBalance": {
    ///          "default": "erc20",
    ///          "allOf": [
    ///            {
    ///              "$ref": "#/components/schemas/BuyTokenDestination"
    ///            }
    ///          ]
    ///        },
    ///        "from": {
    ///          "$ref": "#/components/schemas/Address"
    ///        },
    ///        "onchainOrder": {
    ///          "description": "Flag to signal whether the order is intended for on-chain order placement. Only valid for non ECDSA-signed orders.\"\n",
    ///          "default": false
    ///        },
    ///        "priceQuality": {
    ///          "default": "verified",
    ///          "allOf": [
    ///            {
    ///              "$ref": "#/components/schemas/PriceQuality"
    ///            }
    ///          ]
    ///        },
    ///        "receiver": {
    ///          "description": "An optional address to receive the proceeds of the trade instead of the\n`owner` (i.e. the order signer).\n",
    ///          "oneOf": [
    ///            {
    ///              "type": "null"
    ///            },
    ///            {
    ///              "allOf": [
    ///                {
    ///                  "$ref": "#/components/schemas/Address"
    ///                }
    ///              ]
    ///            }
    ///          ]
    ///        },
    ///        "sellToken": {
    ///          "description": "ERC-20 token to be sold",
    ///          "allOf": [
    ///            {
    ///              "$ref": "#/components/schemas/Address"
    ///            }
    ///          ]
    ///        },
    ///        "sellTokenBalance": {
    ///          "default": "erc20",
    ///          "allOf": [
    ///            {
    ///              "$ref": "#/components/schemas/SellTokenSource"
    ///            }
    ///          ]
    ///        },
    ///        "signingScheme": {
    ///          "default": "eip712",
    ///          "allOf": [
    ///            {
    ///              "$ref": "#/components/schemas/SigningScheme"
    ///            }
    ///          ]
    ///        },
    ///        "timeout": {
    ///          "description": "User provided timeout in milliseconds. If no value is provided the systems default quote timeout will be used. Values get capped at a generous maximum timeout. Note that reducing the timeout can result in worse quotes because it might be too short for some price estimators.\n",
    ///          "type": "integer"
    ///        }
    ///      }
    ///    },
    ///    {
    ///      "allOf": [
    ///        {
    ///          "oneOf": [
    ///            {
    ///              "description": "Absolute validity.",
    ///              "type": "object",
    ///              "properties": {
    ///                "validTo": {
    ///                  "description": "Unix timestamp (`uint32`) until which the order is valid.",
    ///                  "type": "integer"
    ///                }
    ///              }
    ///            },
    ///            {
    ///              "description": "Relative validity",
    ///              "type": "object",
    ///              "properties": {
    ///                "validFor": {
    ///                  "description": "Number (`uint32`) of seconds that the order should be valid for.",
    ///                  "type": "integer"
    ///                }
    ///              }
    ///            }
    ///          ]
    ///        },
    ///        {
    ///          "description": "Quote a sell order given the final total `sellAmount` including fees.",
    ///          "type": "object",
    ///          "required": [
    ///            "kind",
    ///            "sellAmountBeforeFee"
    ///          ],
    ///          "properties": {
    ///            "kind": {
    ///              "allOf": [
    ///                {
    ///                  "$ref": "#/components/schemas/OrderQuoteSideKindSell"
    ///                }
    ///              ]
    ///            },
    ///            "sellAmountBeforeFee": {
    ///              "description": "The total amount that is available for the order. From this value, the fee is deducted and the buy amount is calculated.\n",
    ///              "allOf": [
    ///                {
    ///                  "$ref": "#/components/schemas/TokenAmount"
    ///                }
    ///              ]
    ///            }
    ///          }
    ///        },
    ///        {
    ///          "not": {
    ///            "description": "Quote a sell order given the `sellAmount`.",
    ///            "type": "object",
    ///            "required": [
    ///              "kind",
    ///              "sellAmountAfterFee"
    ///            ],
    ///            "properties": {
    ///              "kind": {
    ///                "allOf": [
    ///                  {
    ///                    "$ref": "#/components/schemas/OrderQuoteSideKindSell"
    ///                  }
    ///                ]
    ///              },
    ///              "sellAmountAfterFee": {
    ///                "description": "The `sellAmount` for the order.",
    ///                "allOf": [
    ///                  {
    ///                    "$ref": "#/components/schemas/TokenAmount"
    ///                  }
    ///                ]
    ///              }
    ///            }
    ///          }
    ///        },
    ///        {
    ///          "not": {
    ///            "description": "Quote a buy order given an exact `buyAmount`.",
    ///            "type": "object",
    ///            "required": [
    ///              "buyAmountAfterFee",
    ///              "kind"
    ///            ],
    ///            "properties": {
    ///              "buyAmountAfterFee": {
    ///                "description": "The `buyAmount` for the order.",
    ///                "allOf": [
    ///                  {
    ///                    "$ref": "#/components/schemas/TokenAmount"
    ///                  }
    ///                ]
    ///              },
    ///              "kind": {
    ///                "allOf": [
    ///                  {
    ///                    "$ref": "#/components/schemas/OrderQuoteSideKindBuy"
    ///                  }
    ///                ]
    ///              }
    ///            }
    ///          }
    ///        }
    ///      ]
    ///    },
    ///    {
    ///      "not": {
    ///        "allOf": [
    ///          {
    ///            "oneOf": [
    ///              {
    ///                "description": "Absolute validity.",
    ///                "type": "object",
    ///                "properties": {
    ///                  "validTo": {
    ///                    "description": "Unix timestamp (`uint32`) until which the order is valid.",
    ///                    "type": "integer"
    ///                  }
    ///                }
    ///              },
    ///              {
    ///                "description": "Relative validity",
    ///                "type": "object",
    ///                "properties": {
    ///                  "validFor": {
    ///                    "description": "Number (`uint32`) of seconds that the order should be valid for.",
    ///                    "type": "integer"
    ///                  }
    ///                }
    ///              }
    ///            ]
    ///          },
    ///          {
    ///            "description": "Quote a sell order given the `sellAmount`.",
    ///            "type": "object",
    ///            "required": [
    ///              "kind",
    ///              "sellAmountAfterFee"
    ///            ],
    ///            "properties": {
    ///              "kind": {
    ///                "allOf": [
    ///                  {
    ///                    "$ref": "#/components/schemas/OrderQuoteSideKindSell"
    ///                  }
    ///                ]
    ///              },
    ///              "sellAmountAfterFee": {
    ///                "description": "The `sellAmount` for the order.",
    ///                "allOf": [
    ///                  {
    ///                    "$ref": "#/components/schemas/TokenAmount"
    ///                  }
    ///                ]
    ///              }
    ///            }
    ///          },
    ///          {
    ///            "not": {
    ///              "description": "Quote a sell order given the final total `sellAmount` including fees.",
    ///              "type": "object",
    ///              "required": [
    ///                "kind",
    ///                "sellAmountBeforeFee"
    ///              ],
    ///              "properties": {
    ///                "kind": {
    ///                  "allOf": [
    ///                    {
    ///                      "$ref": "#/components/schemas/OrderQuoteSideKindSell"
    ///                    }
    ///                  ]
    ///                },
    ///                "sellAmountBeforeFee": {
    ///                  "description": "The total amount that is available for the order. From this value, the fee is deducted and the buy amount is calculated.\n",
    ///                  "allOf": [
    ///                    {
    ///                      "$ref": "#/components/schemas/TokenAmount"
    ///                    }
    ///                  ]
    ///                }
    ///              }
    ///            }
    ///          },
    ///          {
    ///            "not": {
    ///              "description": "Quote a buy order given an exact `buyAmount`.",
    ///              "type": "object",
    ///              "required": [
    ///                "buyAmountAfterFee",
    ///                "kind"
    ///              ],
    ///              "properties": {
    ///                "buyAmountAfterFee": {
    ///                  "description": "The `buyAmount` for the order.",
    ///                  "allOf": [
    ///                    {
    ///                      "$ref": "#/components/schemas/TokenAmount"
    ///                    }
    ///                  ]
    ///                },
    ///                "kind": {
    ///                  "allOf": [
    ///                    {
    ///                      "$ref": "#/components/schemas/OrderQuoteSideKindBuy"
    ///                    }
    ///                  ]
    ///                }
    ///              }
    ///            }
    ///          }
    ///        ]
    ///      }
    ///    },
    ///    {
    ///      "not": {
    ///        "allOf": [
    ///          {
    ///            "oneOf": [
    ///              {
    ///                "description": "Absolute validity.",
    ///                "type": "object",
    ///                "properties": {
    ///                  "validTo": {
    ///                    "description": "Unix timestamp (`uint32`) until which the order is valid.",
    ///                    "type": "integer"
    ///                  }
    ///                }
    ///              },
    ///              {
    ///                "description": "Relative validity",
    ///                "type": "object",
    ///                "properties": {
    ///                  "validFor": {
    ///                    "description": "Number (`uint32`) of seconds that the order should be valid for.",
    ///                    "type": "integer"
    ///                  }
    ///                }
    ///              }
    ///            ]
    ///          },
    ///          {
    ///            "description": "Quote a buy order given an exact `buyAmount`.",
    ///            "type": "object",
    ///            "required": [
    ///              "buyAmountAfterFee",
    ///              "kind"
    ///            ],
    ///            "properties": {
    ///              "buyAmountAfterFee": {
    ///                "description": "The `buyAmount` for the order.",
    ///                "allOf": [
    ///                  {
    ///                    "$ref": "#/components/schemas/TokenAmount"
    ///                  }
    ///                ]
    ///              },
    ///              "kind": {
    ///                "allOf": [
    ///                  {
    ///                    "$ref": "#/components/schemas/OrderQuoteSideKindBuy"
    ///                  }
    ///                ]
    ///              }
    ///            }
    ///          },
    ///          {
    ///            "not": {
    ///              "description": "Quote a sell order given the final total `sellAmount` including fees.",
    ///              "type": "object",
    ///              "required": [
    ///                "kind",
    ///                "sellAmountBeforeFee"
    ///              ],
    ///              "properties": {
    ///                "kind": {
    ///                  "allOf": [
    ///                    {
    ///                      "$ref": "#/components/schemas/OrderQuoteSideKindSell"
    ///                    }
    ///                  ]
    ///                },
    ///                "sellAmountBeforeFee": {
    ///                  "description": "The total amount that is available for the order. From this value, the fee is deducted and the buy amount is calculated.\n",
    ///                  "allOf": [
    ///                    {
    ///                      "$ref": "#/components/schemas/TokenAmount"
    ///                    }
    ///                  ]
    ///                }
    ///              }
    ///            }
    ///          },
    ///          {
    ///            "not": {
    ///              "description": "Quote a sell order given the `sellAmount`.",
    ///              "type": "object",
    ///              "required": [
    ///                "kind",
    ///                "sellAmountAfterFee"
    ///              ],
    ///              "properties": {
    ///                "kind": {
    ///                  "allOf": [
    ///                    {
    ///                      "$ref": "#/components/schemas/OrderQuoteSideKindSell"
    ///                    }
    ///                  ]
    ///                },
    ///                "sellAmountAfterFee": {
    ///                  "description": "The `sellAmount` for the order.",
    ///                  "allOf": [
    ///                    {
    ///                      "$ref": "#/components/schemas/TokenAmount"
    ///                    }
    ///                  ]
    ///                }
    ///              }
    ///            }
    ///          }
    ///        ]
    ///      }
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    #[serde(untagged)]
    pub enum OrderQuoteRequestVariant0 {
        Variant0 {
            /**AppData which will be assigned to the order.

            Expects either a string JSON doc as defined on
            [AppData](https://github.com/cowprotocol/app-data) or a hex
            encoded string for backwards compatibility.

            When the first format is used, it's possible to provide the
            derived appDataHash field.*/
            #[serde(
                rename = "appData",
                default,
                skip_serializing_if = "::std::option::Option::is_none"
            )]
            app_data: ::std::option::Option<OrderQuoteRequestVariant0Variant0AppData>,
            #[serde(
                rename = "appDataHash",
                default,
                skip_serializing_if = "::std::option::Option::is_none"
            )]
            app_data_hash: ::std::option::Option<AppDataHash>,
            ///ERC-20 token to be bought
            #[serde(rename = "buyToken")]
            buy_token: Address,
            #[serde(
                rename = "buyTokenBalance",
                default = "defaults::order_quote_request_variant0_variant0_buy_token_balance"
            )]
            buy_token_balance: BuyTokenDestination,
            from: Address,
            kind: OrderQuoteSideKindSell,
            /**Flag to signal whether the order is intended for on-chain order placement. Only valid for non ECDSA-signed orders."
             */
            #[serde(
                rename = "onchainOrder",
                default = "defaults::order_quote_request_variant0_variant0_onchain_order"
            )]
            onchain_order: ::serde_json::Value,
            #[serde(
                rename = "priceQuality",
                default = "defaults::order_quote_request_variant0_variant0_price_quality"
            )]
            price_quality: PriceQuality,
            /**An optional address to receive the proceeds of the trade instead of the
            `owner` (i.e. the order signer).
            */
            #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
            receiver: ::std::option::Option<Address>,
            /**The total amount that is available for the order. From this value, the fee is deducted and the buy amount is calculated.
             */
            #[serde(rename = "sellAmountBeforeFee")]
            sell_amount_before_fee: TokenAmount,
            ///ERC-20 token to be sold
            #[serde(rename = "sellToken")]
            sell_token: Address,
            #[serde(
                rename = "sellTokenBalance",
                default = "defaults::order_quote_request_variant0_variant0_sell_token_balance"
            )]
            sell_token_balance: SellTokenSource,
            #[serde(
                rename = "signingScheme",
                default = "defaults::order_quote_request_variant0_variant0_signing_scheme"
            )]
            signing_scheme: SigningScheme,
            /**User provided timeout in milliseconds. If no value is provided the systems default quote timeout will be used. Values get capped at a generous maximum timeout. Note that reducing the timeout can result in worse quotes because it might be too short for some price estimators.
             */
            #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
            timeout: ::std::option::Option<i64>,
            ///Unix timestamp (`uint32`) until which the order is valid.
            #[serde(
                rename = "validTo",
                default,
                skip_serializing_if = "::std::option::Option::is_none"
            )]
            valid_to: ::std::option::Option<i64>,
        },
        Variant1 {
            /**AppData which will be assigned to the order.

            Expects either a string JSON doc as defined on
            [AppData](https://github.com/cowprotocol/app-data) or a hex
            encoded string for backwards compatibility.

            When the first format is used, it's possible to provide the
            derived appDataHash field.*/
            #[serde(
                rename = "appData",
                default,
                skip_serializing_if = "::std::option::Option::is_none"
            )]
            app_data: ::std::option::Option<OrderQuoteRequestVariant0Variant1AppData>,
            #[serde(
                rename = "appDataHash",
                default,
                skip_serializing_if = "::std::option::Option::is_none"
            )]
            app_data_hash: ::std::option::Option<AppDataHash>,
            ///ERC-20 token to be bought
            #[serde(rename = "buyToken")]
            buy_token: Address,
            #[serde(
                rename = "buyTokenBalance",
                default = "defaults::order_quote_request_variant0_variant1_buy_token_balance"
            )]
            buy_token_balance: BuyTokenDestination,
            from: Address,
            kind: OrderQuoteSideKindSell,
            /**Flag to signal whether the order is intended for on-chain order placement. Only valid for non ECDSA-signed orders."
             */
            #[serde(
                rename = "onchainOrder",
                default = "defaults::order_quote_request_variant0_variant1_onchain_order"
            )]
            onchain_order: ::serde_json::Value,
            #[serde(
                rename = "priceQuality",
                default = "defaults::order_quote_request_variant0_variant1_price_quality"
            )]
            price_quality: PriceQuality,
            /**An optional address to receive the proceeds of the trade instead of the
            `owner` (i.e. the order signer).
            */
            #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
            receiver: ::std::option::Option<Address>,
            /**The total amount that is available for the order. From this value, the fee is deducted and the buy amount is calculated.
             */
            #[serde(rename = "sellAmountBeforeFee")]
            sell_amount_before_fee: TokenAmount,
            ///ERC-20 token to be sold
            #[serde(rename = "sellToken")]
            sell_token: Address,
            #[serde(
                rename = "sellTokenBalance",
                default = "defaults::order_quote_request_variant0_variant1_sell_token_balance"
            )]
            sell_token_balance: SellTokenSource,
            #[serde(
                rename = "signingScheme",
                default = "defaults::order_quote_request_variant0_variant1_signing_scheme"
            )]
            signing_scheme: SigningScheme,
            /**User provided timeout in milliseconds. If no value is provided the systems default quote timeout will be used. Values get capped at a generous maximum timeout. Note that reducing the timeout can result in worse quotes because it might be too short for some price estimators.
             */
            #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
            timeout: ::std::option::Option<i64>,
            ///Number (`uint32`) of seconds that the order should be valid for.
            #[serde(
                rename = "validFor",
                default,
                skip_serializing_if = "::std::option::Option::is_none"
            )]
            valid_for: ::std::option::Option<i64>,
        },
    }
    /**AppData which will be assigned to the order.

    Expects either a string JSON doc as defined on
    [AppData](https://github.com/cowprotocol/app-data) or a hex
    encoded string for backwards compatibility.

    When the first format is used, it's possible to provide the
    derived appDataHash field.*/
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "AppData which will be assigned to the order.\n\nExpects either a string JSON doc as defined on\n[AppData](https://github.com/cowprotocol/app-data) or a hex\nencoded string for backwards compatibility.\n\nWhen the first format is used, it's possible to provide the\nderived appDataHash field.",
    ///  "oneOf": [
    ///    {
    ///      "$ref": "#/components/schemas/AppData"
    ///    },
    ///    {
    ///      "$ref": "#/components/schemas/AppDataHash"
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    #[serde(untagged)]
    pub enum OrderQuoteRequestVariant0Variant0AppData {
        Variant0(AppData),
        Variant1(AppDataHash),
    }
    impl ::std::fmt::Display for OrderQuoteRequestVariant0Variant0AppData {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                Self::Variant0(x) => x.fmt(f),
                Self::Variant1(x) => x.fmt(f),
            }
        }
    }
    impl ::std::convert::From<AppData> for OrderQuoteRequestVariant0Variant0AppData {
        fn from(value: AppData) -> Self {
            Self::Variant0(value)
        }
    }
    impl ::std::convert::From<AppDataHash> for OrderQuoteRequestVariant0Variant0AppData {
        fn from(value: AppDataHash) -> Self {
            Self::Variant1(value)
        }
    }
    /**AppData which will be assigned to the order.

    Expects either a string JSON doc as defined on
    [AppData](https://github.com/cowprotocol/app-data) or a hex
    encoded string for backwards compatibility.

    When the first format is used, it's possible to provide the
    derived appDataHash field.*/
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "AppData which will be assigned to the order.\n\nExpects either a string JSON doc as defined on\n[AppData](https://github.com/cowprotocol/app-data) or a hex\nencoded string for backwards compatibility.\n\nWhen the first format is used, it's possible to provide the\nderived appDataHash field.",
    ///  "oneOf": [
    ///    {
    ///      "$ref": "#/components/schemas/AppData"
    ///    },
    ///    {
    ///      "$ref": "#/components/schemas/AppDataHash"
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    #[serde(untagged)]
    pub enum OrderQuoteRequestVariant0Variant1AppData {
        Variant0(AppData),
        Variant1(AppDataHash),
    }
    impl ::std::fmt::Display for OrderQuoteRequestVariant0Variant1AppData {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                Self::Variant0(x) => x.fmt(f),
                Self::Variant1(x) => x.fmt(f),
            }
        }
    }
    impl ::std::convert::From<AppData> for OrderQuoteRequestVariant0Variant1AppData {
        fn from(value: AppData) -> Self {
            Self::Variant0(value)
        }
    }
    impl ::std::convert::From<AppDataHash> for OrderQuoteRequestVariant0Variant1AppData {
        fn from(value: AppDataHash) -> Self {
            Self::Variant1(value)
        }
    }
    ///`OrderQuoteRequestVariant1`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "allOf": [
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "buyToken",
    ///        "from",
    ///        "sellToken"
    ///      ],
    ///      "properties": {
    ///        "appData": {
    ///          "description": "AppData which will be assigned to the order.\n\nExpects either a string JSON doc as defined on\n[AppData](https://github.com/cowprotocol/app-data) or a hex\nencoded string for backwards compatibility.\n\nWhen the first format is used, it's possible to provide the\nderived appDataHash field.",
    ///          "oneOf": [
    ///            {
    ///              "$ref": "#/components/schemas/AppData"
    ///            },
    ///            {
    ///              "$ref": "#/components/schemas/AppDataHash"
    ///            }
    ///          ]
    ///        },
    ///        "appDataHash": {
    ///          "description": "The hash of the stringified JSON appData doc.\n\nIf present, `appData` field must be set with the aforementioned\ndata where this hash is derived from.\n\nIn case they differ, the call will fail.",
    ///          "anyOf": [
    ///            {
    ///              "$ref": "#/components/schemas/AppDataHash"
    ///            }
    ///          ]
    ///        },
    ///        "buyToken": {
    ///          "description": "ERC-20 token to be bought",
    ///          "allOf": [
    ///            {
    ///              "$ref": "#/components/schemas/Address"
    ///            }
    ///          ]
    ///        },
    ///        "buyTokenBalance": {
    ///          "default": "erc20",
    ///          "allOf": [
    ///            {
    ///              "$ref": "#/components/schemas/BuyTokenDestination"
    ///            }
    ///          ]
    ///        },
    ///        "from": {
    ///          "$ref": "#/components/schemas/Address"
    ///        },
    ///        "onchainOrder": {
    ///          "description": "Flag to signal whether the order is intended for on-chain order placement. Only valid for non ECDSA-signed orders.\"\n",
    ///          "default": false
    ///        },
    ///        "priceQuality": {
    ///          "default": "verified",
    ///          "allOf": [
    ///            {
    ///              "$ref": "#/components/schemas/PriceQuality"
    ///            }
    ///          ]
    ///        },
    ///        "receiver": {
    ///          "description": "An optional address to receive the proceeds of the trade instead of the\n`owner` (i.e. the order signer).\n",
    ///          "oneOf": [
    ///            {
    ///              "type": "null"
    ///            },
    ///            {
    ///              "allOf": [
    ///                {
    ///                  "$ref": "#/components/schemas/Address"
    ///                }
    ///              ]
    ///            }
    ///          ]
    ///        },
    ///        "sellToken": {
    ///          "description": "ERC-20 token to be sold",
    ///          "allOf": [
    ///            {
    ///              "$ref": "#/components/schemas/Address"
    ///            }
    ///          ]
    ///        },
    ///        "sellTokenBalance": {
    ///          "default": "erc20",
    ///          "allOf": [
    ///            {
    ///              "$ref": "#/components/schemas/SellTokenSource"
    ///            }
    ///          ]
    ///        },
    ///        "signingScheme": {
    ///          "default": "eip712",
    ///          "allOf": [
    ///            {
    ///              "$ref": "#/components/schemas/SigningScheme"
    ///            }
    ///          ]
    ///        },
    ///        "timeout": {
    ///          "description": "User provided timeout in milliseconds. If no value is provided the systems default quote timeout will be used. Values get capped at a generous maximum timeout. Note that reducing the timeout can result in worse quotes because it might be too short for some price estimators.\n",
    ///          "type": "integer"
    ///        }
    ///      }
    ///    },
    ///    {
    ///      "allOf": [
    ///        {
    ///          "oneOf": [
    ///            {
    ///              "description": "Absolute validity.",
    ///              "type": "object",
    ///              "properties": {
    ///                "validTo": {
    ///                  "description": "Unix timestamp (`uint32`) until which the order is valid.",
    ///                  "type": "integer"
    ///                }
    ///              }
    ///            },
    ///            {
    ///              "description": "Relative validity",
    ///              "type": "object",
    ///              "properties": {
    ///                "validFor": {
    ///                  "description": "Number (`uint32`) of seconds that the order should be valid for.",
    ///                  "type": "integer"
    ///                }
    ///              }
    ///            }
    ///          ]
    ///        },
    ///        {
    ///          "description": "Quote a sell order given the `sellAmount`.",
    ///          "type": "object",
    ///          "required": [
    ///            "kind",
    ///            "sellAmountAfterFee"
    ///          ],
    ///          "properties": {
    ///            "kind": {
    ///              "allOf": [
    ///                {
    ///                  "$ref": "#/components/schemas/OrderQuoteSideKindSell"
    ///                }
    ///              ]
    ///            },
    ///            "sellAmountAfterFee": {
    ///              "description": "The `sellAmount` for the order.",
    ///              "allOf": [
    ///                {
    ///                  "$ref": "#/components/schemas/TokenAmount"
    ///                }
    ///              ]
    ///            }
    ///          }
    ///        },
    ///        {
    ///          "not": {
    ///            "description": "Quote a sell order given the final total `sellAmount` including fees.",
    ///            "type": "object",
    ///            "required": [
    ///              "kind",
    ///              "sellAmountBeforeFee"
    ///            ],
    ///            "properties": {
    ///              "kind": {
    ///                "allOf": [
    ///                  {
    ///                    "$ref": "#/components/schemas/OrderQuoteSideKindSell"
    ///                  }
    ///                ]
    ///              },
    ///              "sellAmountBeforeFee": {
    ///                "description": "The total amount that is available for the order. From this value, the fee is deducted and the buy amount is calculated.\n",
    ///                "allOf": [
    ///                  {
    ///                    "$ref": "#/components/schemas/TokenAmount"
    ///                  }
    ///                ]
    ///              }
    ///            }
    ///          }
    ///        },
    ///        {
    ///          "not": {
    ///            "description": "Quote a buy order given an exact `buyAmount`.",
    ///            "type": "object",
    ///            "required": [
    ///              "buyAmountAfterFee",
    ///              "kind"
    ///            ],
    ///            "properties": {
    ///              "buyAmountAfterFee": {
    ///                "description": "The `buyAmount` for the order.",
    ///                "allOf": [
    ///                  {
    ///                    "$ref": "#/components/schemas/TokenAmount"
    ///                  }
    ///                ]
    ///              },
    ///              "kind": {
    ///                "allOf": [
    ///                  {
    ///                    "$ref": "#/components/schemas/OrderQuoteSideKindBuy"
    ///                  }
    ///                ]
    ///              }
    ///            }
    ///          }
    ///        }
    ///      ]
    ///    },
    ///    {
    ///      "not": {
    ///        "allOf": [
    ///          {
    ///            "oneOf": [
    ///              {
    ///                "description": "Absolute validity.",
    ///                "type": "object",
    ///                "properties": {
    ///                  "validTo": {
    ///                    "description": "Unix timestamp (`uint32`) until which the order is valid.",
    ///                    "type": "integer"
    ///                  }
    ///                }
    ///              },
    ///              {
    ///                "description": "Relative validity",
    ///                "type": "object",
    ///                "properties": {
    ///                  "validFor": {
    ///                    "description": "Number (`uint32`) of seconds that the order should be valid for.",
    ///                    "type": "integer"
    ///                  }
    ///                }
    ///              }
    ///            ]
    ///          },
    ///          {
    ///            "description": "Quote a sell order given the final total `sellAmount` including fees.",
    ///            "type": "object",
    ///            "required": [
    ///              "kind",
    ///              "sellAmountBeforeFee"
    ///            ],
    ///            "properties": {
    ///              "kind": {
    ///                "allOf": [
    ///                  {
    ///                    "$ref": "#/components/schemas/OrderQuoteSideKindSell"
    ///                  }
    ///                ]
    ///              },
    ///              "sellAmountBeforeFee": {
    ///                "description": "The total amount that is available for the order. From this value, the fee is deducted and the buy amount is calculated.\n",
    ///                "allOf": [
    ///                  {
    ///                    "$ref": "#/components/schemas/TokenAmount"
    ///                  }
    ///                ]
    ///              }
    ///            }
    ///          },
    ///          {
    ///            "not": {
    ///              "description": "Quote a sell order given the `sellAmount`.",
    ///              "type": "object",
    ///              "required": [
    ///                "kind",
    ///                "sellAmountAfterFee"
    ///              ],
    ///              "properties": {
    ///                "kind": {
    ///                  "allOf": [
    ///                    {
    ///                      "$ref": "#/components/schemas/OrderQuoteSideKindSell"
    ///                    }
    ///                  ]
    ///                },
    ///                "sellAmountAfterFee": {
    ///                  "description": "The `sellAmount` for the order.",
    ///                  "allOf": [
    ///                    {
    ///                      "$ref": "#/components/schemas/TokenAmount"
    ///                    }
    ///                  ]
    ///                }
    ///              }
    ///            }
    ///          },
    ///          {
    ///            "not": {
    ///              "description": "Quote a buy order given an exact `buyAmount`.",
    ///              "type": "object",
    ///              "required": [
    ///                "buyAmountAfterFee",
    ///                "kind"
    ///              ],
    ///              "properties": {
    ///                "buyAmountAfterFee": {
    ///                  "description": "The `buyAmount` for the order.",
    ///                  "allOf": [
    ///                    {
    ///                      "$ref": "#/components/schemas/TokenAmount"
    ///                    }
    ///                  ]
    ///                },
    ///                "kind": {
    ///                  "allOf": [
    ///                    {
    ///                      "$ref": "#/components/schemas/OrderQuoteSideKindBuy"
    ///                    }
    ///                  ]
    ///                }
    ///              }
    ///            }
    ///          }
    ///        ]
    ///      }
    ///    },
    ///    {
    ///      "not": {
    ///        "allOf": [
    ///          {
    ///            "oneOf": [
    ///              {
    ///                "description": "Absolute validity.",
    ///                "type": "object",
    ///                "properties": {
    ///                  "validTo": {
    ///                    "description": "Unix timestamp (`uint32`) until which the order is valid.",
    ///                    "type": "integer"
    ///                  }
    ///                }
    ///              },
    ///              {
    ///                "description": "Relative validity",
    ///                "type": "object",
    ///                "properties": {
    ///                  "validFor": {
    ///                    "description": "Number (`uint32`) of seconds that the order should be valid for.",
    ///                    "type": "integer"
    ///                  }
    ///                }
    ///              }
    ///            ]
    ///          },
    ///          {
    ///            "description": "Quote a buy order given an exact `buyAmount`.",
    ///            "type": "object",
    ///            "required": [
    ///              "buyAmountAfterFee",
    ///              "kind"
    ///            ],
    ///            "properties": {
    ///              "buyAmountAfterFee": {
    ///                "description": "The `buyAmount` for the order.",
    ///                "allOf": [
    ///                  {
    ///                    "$ref": "#/components/schemas/TokenAmount"
    ///                  }
    ///                ]
    ///              },
    ///              "kind": {
    ///                "allOf": [
    ///                  {
    ///                    "$ref": "#/components/schemas/OrderQuoteSideKindBuy"
    ///                  }
    ///                ]
    ///              }
    ///            }
    ///          },
    ///          {
    ///            "not": {
    ///              "description": "Quote a sell order given the final total `sellAmount` including fees.",
    ///              "type": "object",
    ///              "required": [
    ///                "kind",
    ///                "sellAmountBeforeFee"
    ///              ],
    ///              "properties": {
    ///                "kind": {
    ///                  "allOf": [
    ///                    {
    ///                      "$ref": "#/components/schemas/OrderQuoteSideKindSell"
    ///                    }
    ///                  ]
    ///                },
    ///                "sellAmountBeforeFee": {
    ///                  "description": "The total amount that is available for the order. From this value, the fee is deducted and the buy amount is calculated.\n",
    ///                  "allOf": [
    ///                    {
    ///                      "$ref": "#/components/schemas/TokenAmount"
    ///                    }
    ///                  ]
    ///                }
    ///              }
    ///            }
    ///          },
    ///          {
    ///            "not": {
    ///              "description": "Quote a sell order given the `sellAmount`.",
    ///              "type": "object",
    ///              "required": [
    ///                "kind",
    ///                "sellAmountAfterFee"
    ///              ],
    ///              "properties": {
    ///                "kind": {
    ///                  "allOf": [
    ///                    {
    ///                      "$ref": "#/components/schemas/OrderQuoteSideKindSell"
    ///                    }
    ///                  ]
    ///                },
    ///                "sellAmountAfterFee": {
    ///                  "description": "The `sellAmount` for the order.",
    ///                  "allOf": [
    ///                    {
    ///                      "$ref": "#/components/schemas/TokenAmount"
    ///                    }
    ///                  ]
    ///                }
    ///              }
    ///            }
    ///          }
    ///        ]
    ///      }
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    #[serde(untagged)]
    pub enum OrderQuoteRequestVariant1 {
        Variant0 {
            /**AppData which will be assigned to the order.

            Expects either a string JSON doc as defined on
            [AppData](https://github.com/cowprotocol/app-data) or a hex
            encoded string for backwards compatibility.

            When the first format is used, it's possible to provide the
            derived appDataHash field.*/
            #[serde(
                rename = "appData",
                default,
                skip_serializing_if = "::std::option::Option::is_none"
            )]
            app_data: ::std::option::Option<OrderQuoteRequestVariant1Variant0AppData>,
            #[serde(
                rename = "appDataHash",
                default,
                skip_serializing_if = "::std::option::Option::is_none"
            )]
            app_data_hash: ::std::option::Option<AppDataHash>,
            ///ERC-20 token to be bought
            #[serde(rename = "buyToken")]
            buy_token: Address,
            #[serde(
                rename = "buyTokenBalance",
                default = "defaults::order_quote_request_variant1_variant0_buy_token_balance"
            )]
            buy_token_balance: BuyTokenDestination,
            from: Address,
            kind: OrderQuoteSideKindSell,
            /**Flag to signal whether the order is intended for on-chain order placement. Only valid for non ECDSA-signed orders."
             */
            #[serde(
                rename = "onchainOrder",
                default = "defaults::order_quote_request_variant1_variant0_onchain_order"
            )]
            onchain_order: ::serde_json::Value,
            #[serde(
                rename = "priceQuality",
                default = "defaults::order_quote_request_variant1_variant0_price_quality"
            )]
            price_quality: PriceQuality,
            /**An optional address to receive the proceeds of the trade instead of the
            `owner` (i.e. the order signer).
            */
            #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
            receiver: ::std::option::Option<Address>,
            ///The `sellAmount` for the order.
            #[serde(rename = "sellAmountAfterFee")]
            sell_amount_after_fee: TokenAmount,
            ///ERC-20 token to be sold
            #[serde(rename = "sellToken")]
            sell_token: Address,
            #[serde(
                rename = "sellTokenBalance",
                default = "defaults::order_quote_request_variant1_variant0_sell_token_balance"
            )]
            sell_token_balance: SellTokenSource,
            #[serde(
                rename = "signingScheme",
                default = "defaults::order_quote_request_variant1_variant0_signing_scheme"
            )]
            signing_scheme: SigningScheme,
            /**User provided timeout in milliseconds. If no value is provided the systems default quote timeout will be used. Values get capped at a generous maximum timeout. Note that reducing the timeout can result in worse quotes because it might be too short for some price estimators.
             */
            #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
            timeout: ::std::option::Option<i64>,
            ///Unix timestamp (`uint32`) until which the order is valid.
            #[serde(
                rename = "validTo",
                default,
                skip_serializing_if = "::std::option::Option::is_none"
            )]
            valid_to: ::std::option::Option<i64>,
        },
        Variant1 {
            /**AppData which will be assigned to the order.

            Expects either a string JSON doc as defined on
            [AppData](https://github.com/cowprotocol/app-data) or a hex
            encoded string for backwards compatibility.

            When the first format is used, it's possible to provide the
            derived appDataHash field.*/
            #[serde(
                rename = "appData",
                default,
                skip_serializing_if = "::std::option::Option::is_none"
            )]
            app_data: ::std::option::Option<OrderQuoteRequestVariant1Variant1AppData>,
            #[serde(
                rename = "appDataHash",
                default,
                skip_serializing_if = "::std::option::Option::is_none"
            )]
            app_data_hash: ::std::option::Option<AppDataHash>,
            ///ERC-20 token to be bought
            #[serde(rename = "buyToken")]
            buy_token: Address,
            #[serde(
                rename = "buyTokenBalance",
                default = "defaults::order_quote_request_variant1_variant1_buy_token_balance"
            )]
            buy_token_balance: BuyTokenDestination,
            from: Address,
            kind: OrderQuoteSideKindSell,
            /**Flag to signal whether the order is intended for on-chain order placement. Only valid for non ECDSA-signed orders."
             */
            #[serde(
                rename = "onchainOrder",
                default = "defaults::order_quote_request_variant1_variant1_onchain_order"
            )]
            onchain_order: ::serde_json::Value,
            #[serde(
                rename = "priceQuality",
                default = "defaults::order_quote_request_variant1_variant1_price_quality"
            )]
            price_quality: PriceQuality,
            /**An optional address to receive the proceeds of the trade instead of the
            `owner` (i.e. the order signer).
            */
            #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
            receiver: ::std::option::Option<Address>,
            ///The `sellAmount` for the order.
            #[serde(rename = "sellAmountAfterFee")]
            sell_amount_after_fee: TokenAmount,
            ///ERC-20 token to be sold
            #[serde(rename = "sellToken")]
            sell_token: Address,
            #[serde(
                rename = "sellTokenBalance",
                default = "defaults::order_quote_request_variant1_variant1_sell_token_balance"
            )]
            sell_token_balance: SellTokenSource,
            #[serde(
                rename = "signingScheme",
                default = "defaults::order_quote_request_variant1_variant1_signing_scheme"
            )]
            signing_scheme: SigningScheme,
            /**User provided timeout in milliseconds. If no value is provided the systems default quote timeout will be used. Values get capped at a generous maximum timeout. Note that reducing the timeout can result in worse quotes because it might be too short for some price estimators.
             */
            #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
            timeout: ::std::option::Option<i64>,
            ///Number (`uint32`) of seconds that the order should be valid for.
            #[serde(
                rename = "validFor",
                default,
                skip_serializing_if = "::std::option::Option::is_none"
            )]
            valid_for: ::std::option::Option<i64>,
        },
    }
    /**AppData which will be assigned to the order.

    Expects either a string JSON doc as defined on
    [AppData](https://github.com/cowprotocol/app-data) or a hex
    encoded string for backwards compatibility.

    When the first format is used, it's possible to provide the
    derived appDataHash field.*/
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "AppData which will be assigned to the order.\n\nExpects either a string JSON doc as defined on\n[AppData](https://github.com/cowprotocol/app-data) or a hex\nencoded string for backwards compatibility.\n\nWhen the first format is used, it's possible to provide the\nderived appDataHash field.",
    ///  "oneOf": [
    ///    {
    ///      "$ref": "#/components/schemas/AppData"
    ///    },
    ///    {
    ///      "$ref": "#/components/schemas/AppDataHash"
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    #[serde(untagged)]
    pub enum OrderQuoteRequestVariant1Variant0AppData {
        Variant0(AppData),
        Variant1(AppDataHash),
    }
    impl ::std::fmt::Display for OrderQuoteRequestVariant1Variant0AppData {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                Self::Variant0(x) => x.fmt(f),
                Self::Variant1(x) => x.fmt(f),
            }
        }
    }
    impl ::std::convert::From<AppData> for OrderQuoteRequestVariant1Variant0AppData {
        fn from(value: AppData) -> Self {
            Self::Variant0(value)
        }
    }
    impl ::std::convert::From<AppDataHash> for OrderQuoteRequestVariant1Variant0AppData {
        fn from(value: AppDataHash) -> Self {
            Self::Variant1(value)
        }
    }
    /**AppData which will be assigned to the order.

    Expects either a string JSON doc as defined on
    [AppData](https://github.com/cowprotocol/app-data) or a hex
    encoded string for backwards compatibility.

    When the first format is used, it's possible to provide the
    derived appDataHash field.*/
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "AppData which will be assigned to the order.\n\nExpects either a string JSON doc as defined on\n[AppData](https://github.com/cowprotocol/app-data) or a hex\nencoded string for backwards compatibility.\n\nWhen the first format is used, it's possible to provide the\nderived appDataHash field.",
    ///  "oneOf": [
    ///    {
    ///      "$ref": "#/components/schemas/AppData"
    ///    },
    ///    {
    ///      "$ref": "#/components/schemas/AppDataHash"
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    #[serde(untagged)]
    pub enum OrderQuoteRequestVariant1Variant1AppData {
        Variant0(AppData),
        Variant1(AppDataHash),
    }
    impl ::std::fmt::Display for OrderQuoteRequestVariant1Variant1AppData {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                Self::Variant0(x) => x.fmt(f),
                Self::Variant1(x) => x.fmt(f),
            }
        }
    }
    impl ::std::convert::From<AppData> for OrderQuoteRequestVariant1Variant1AppData {
        fn from(value: AppData) -> Self {
            Self::Variant0(value)
        }
    }
    impl ::std::convert::From<AppDataHash> for OrderQuoteRequestVariant1Variant1AppData {
        fn from(value: AppDataHash) -> Self {
            Self::Variant1(value)
        }
    }
    ///`OrderQuoteRequestVariant2`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "allOf": [
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "buyToken",
    ///        "from",
    ///        "sellToken"
    ///      ],
    ///      "properties": {
    ///        "appData": {
    ///          "description": "AppData which will be assigned to the order.\n\nExpects either a string JSON doc as defined on\n[AppData](https://github.com/cowprotocol/app-data) or a hex\nencoded string for backwards compatibility.\n\nWhen the first format is used, it's possible to provide the\nderived appDataHash field.",
    ///          "oneOf": [
    ///            {
    ///              "$ref": "#/components/schemas/AppData"
    ///            },
    ///            {
    ///              "$ref": "#/components/schemas/AppDataHash"
    ///            }
    ///          ]
    ///        },
    ///        "appDataHash": {
    ///          "description": "The hash of the stringified JSON appData doc.\n\nIf present, `appData` field must be set with the aforementioned\ndata where this hash is derived from.\n\nIn case they differ, the call will fail.",
    ///          "anyOf": [
    ///            {
    ///              "$ref": "#/components/schemas/AppDataHash"
    ///            }
    ///          ]
    ///        },
    ///        "buyToken": {
    ///          "description": "ERC-20 token to be bought",
    ///          "allOf": [
    ///            {
    ///              "$ref": "#/components/schemas/Address"
    ///            }
    ///          ]
    ///        },
    ///        "buyTokenBalance": {
    ///          "default": "erc20",
    ///          "allOf": [
    ///            {
    ///              "$ref": "#/components/schemas/BuyTokenDestination"
    ///            }
    ///          ]
    ///        },
    ///        "from": {
    ///          "$ref": "#/components/schemas/Address"
    ///        },
    ///        "onchainOrder": {
    ///          "description": "Flag to signal whether the order is intended for on-chain order placement. Only valid for non ECDSA-signed orders.\"\n",
    ///          "default": false
    ///        },
    ///        "priceQuality": {
    ///          "default": "verified",
    ///          "allOf": [
    ///            {
    ///              "$ref": "#/components/schemas/PriceQuality"
    ///            }
    ///          ]
    ///        },
    ///        "receiver": {
    ///          "description": "An optional address to receive the proceeds of the trade instead of the\n`owner` (i.e. the order signer).\n",
    ///          "oneOf": [
    ///            {
    ///              "type": "null"
    ///            },
    ///            {
    ///              "allOf": [
    ///                {
    ///                  "$ref": "#/components/schemas/Address"
    ///                }
    ///              ]
    ///            }
    ///          ]
    ///        },
    ///        "sellToken": {
    ///          "description": "ERC-20 token to be sold",
    ///          "allOf": [
    ///            {
    ///              "$ref": "#/components/schemas/Address"
    ///            }
    ///          ]
    ///        },
    ///        "sellTokenBalance": {
    ///          "default": "erc20",
    ///          "allOf": [
    ///            {
    ///              "$ref": "#/components/schemas/SellTokenSource"
    ///            }
    ///          ]
    ///        },
    ///        "signingScheme": {
    ///          "default": "eip712",
    ///          "allOf": [
    ///            {
    ///              "$ref": "#/components/schemas/SigningScheme"
    ///            }
    ///          ]
    ///        },
    ///        "timeout": {
    ///          "description": "User provided timeout in milliseconds. If no value is provided the systems default quote timeout will be used. Values get capped at a generous maximum timeout. Note that reducing the timeout can result in worse quotes because it might be too short for some price estimators.\n",
    ///          "type": "integer"
    ///        }
    ///      }
    ///    },
    ///    {
    ///      "allOf": [
    ///        {
    ///          "oneOf": [
    ///            {
    ///              "description": "Absolute validity.",
    ///              "type": "object",
    ///              "properties": {
    ///                "validTo": {
    ///                  "description": "Unix timestamp (`uint32`) until which the order is valid.",
    ///                  "type": "integer"
    ///                }
    ///              }
    ///            },
    ///            {
    ///              "description": "Relative validity",
    ///              "type": "object",
    ///              "properties": {
    ///                "validFor": {
    ///                  "description": "Number (`uint32`) of seconds that the order should be valid for.",
    ///                  "type": "integer"
    ///                }
    ///              }
    ///            }
    ///          ]
    ///        },
    ///        {
    ///          "description": "Quote a buy order given an exact `buyAmount`.",
    ///          "type": "object",
    ///          "required": [
    ///            "buyAmountAfterFee",
    ///            "kind"
    ///          ],
    ///          "properties": {
    ///            "buyAmountAfterFee": {
    ///              "description": "The `buyAmount` for the order.",
    ///              "allOf": [
    ///                {
    ///                  "$ref": "#/components/schemas/TokenAmount"
    ///                }
    ///              ]
    ///            },
    ///            "kind": {
    ///              "allOf": [
    ///                {
    ///                  "$ref": "#/components/schemas/OrderQuoteSideKindBuy"
    ///                }
    ///              ]
    ///            }
    ///          }
    ///        },
    ///        {
    ///          "not": {
    ///            "description": "Quote a sell order given the final total `sellAmount` including fees.",
    ///            "type": "object",
    ///            "required": [
    ///              "kind",
    ///              "sellAmountBeforeFee"
    ///            ],
    ///            "properties": {
    ///              "kind": {
    ///                "allOf": [
    ///                  {
    ///                    "$ref": "#/components/schemas/OrderQuoteSideKindSell"
    ///                  }
    ///                ]
    ///              },
    ///              "sellAmountBeforeFee": {
    ///                "description": "The total amount that is available for the order. From this value, the fee is deducted and the buy amount is calculated.\n",
    ///                "allOf": [
    ///                  {
    ///                    "$ref": "#/components/schemas/TokenAmount"
    ///                  }
    ///                ]
    ///              }
    ///            }
    ///          }
    ///        },
    ///        {
    ///          "not": {
    ///            "description": "Quote a sell order given the `sellAmount`.",
    ///            "type": "object",
    ///            "required": [
    ///              "kind",
    ///              "sellAmountAfterFee"
    ///            ],
    ///            "properties": {
    ///              "kind": {
    ///                "allOf": [
    ///                  {
    ///                    "$ref": "#/components/schemas/OrderQuoteSideKindSell"
    ///                  }
    ///                ]
    ///              },
    ///              "sellAmountAfterFee": {
    ///                "description": "The `sellAmount` for the order.",
    ///                "allOf": [
    ///                  {
    ///                    "$ref": "#/components/schemas/TokenAmount"
    ///                  }
    ///                ]
    ///              }
    ///            }
    ///          }
    ///        }
    ///      ]
    ///    },
    ///    {
    ///      "not": {
    ///        "allOf": [
    ///          {
    ///            "oneOf": [
    ///              {
    ///                "description": "Absolute validity.",
    ///                "type": "object",
    ///                "properties": {
    ///                  "validTo": {
    ///                    "description": "Unix timestamp (`uint32`) until which the order is valid.",
    ///                    "type": "integer"
    ///                  }
    ///                }
    ///              },
    ///              {
    ///                "description": "Relative validity",
    ///                "type": "object",
    ///                "properties": {
    ///                  "validFor": {
    ///                    "description": "Number (`uint32`) of seconds that the order should be valid for.",
    ///                    "type": "integer"
    ///                  }
    ///                }
    ///              }
    ///            ]
    ///          },
    ///          {
    ///            "description": "Quote a sell order given the final total `sellAmount` including fees.",
    ///            "type": "object",
    ///            "required": [
    ///              "kind",
    ///              "sellAmountBeforeFee"
    ///            ],
    ///            "properties": {
    ///              "kind": {
    ///                "allOf": [
    ///                  {
    ///                    "$ref": "#/components/schemas/OrderQuoteSideKindSell"
    ///                  }
    ///                ]
    ///              },
    ///              "sellAmountBeforeFee": {
    ///                "description": "The total amount that is available for the order. From this value, the fee is deducted and the buy amount is calculated.\n",
    ///                "allOf": [
    ///                  {
    ///                    "$ref": "#/components/schemas/TokenAmount"
    ///                  }
    ///                ]
    ///              }
    ///            }
    ///          },
    ///          {
    ///            "not": {
    ///              "description": "Quote a sell order given the `sellAmount`.",
    ///              "type": "object",
    ///              "required": [
    ///                "kind",
    ///                "sellAmountAfterFee"
    ///              ],
    ///              "properties": {
    ///                "kind": {
    ///                  "allOf": [
    ///                    {
    ///                      "$ref": "#/components/schemas/OrderQuoteSideKindSell"
    ///                    }
    ///                  ]
    ///                },
    ///                "sellAmountAfterFee": {
    ///                  "description": "The `sellAmount` for the order.",
    ///                  "allOf": [
    ///                    {
    ///                      "$ref": "#/components/schemas/TokenAmount"
    ///                    }
    ///                  ]
    ///                }
    ///              }
    ///            }
    ///          },
    ///          {
    ///            "not": {
    ///              "description": "Quote a buy order given an exact `buyAmount`.",
    ///              "type": "object",
    ///              "required": [
    ///                "buyAmountAfterFee",
    ///                "kind"
    ///              ],
    ///              "properties": {
    ///                "buyAmountAfterFee": {
    ///                  "description": "The `buyAmount` for the order.",
    ///                  "allOf": [
    ///                    {
    ///                      "$ref": "#/components/schemas/TokenAmount"
    ///                    }
    ///                  ]
    ///                },
    ///                "kind": {
    ///                  "allOf": [
    ///                    {
    ///                      "$ref": "#/components/schemas/OrderQuoteSideKindBuy"
    ///                    }
    ///                  ]
    ///                }
    ///              }
    ///            }
    ///          }
    ///        ]
    ///      }
    ///    },
    ///    {
    ///      "not": {
    ///        "allOf": [
    ///          {
    ///            "oneOf": [
    ///              {
    ///                "description": "Absolute validity.",
    ///                "type": "object",
    ///                "properties": {
    ///                  "validTo": {
    ///                    "description": "Unix timestamp (`uint32`) until which the order is valid.",
    ///                    "type": "integer"
    ///                  }
    ///                }
    ///              },
    ///              {
    ///                "description": "Relative validity",
    ///                "type": "object",
    ///                "properties": {
    ///                  "validFor": {
    ///                    "description": "Number (`uint32`) of seconds that the order should be valid for.",
    ///                    "type": "integer"
    ///                  }
    ///                }
    ///              }
    ///            ]
    ///          },
    ///          {
    ///            "description": "Quote a sell order given the `sellAmount`.",
    ///            "type": "object",
    ///            "required": [
    ///              "kind",
    ///              "sellAmountAfterFee"
    ///            ],
    ///            "properties": {
    ///              "kind": {
    ///                "allOf": [
    ///                  {
    ///                    "$ref": "#/components/schemas/OrderQuoteSideKindSell"
    ///                  }
    ///                ]
    ///              },
    ///              "sellAmountAfterFee": {
    ///                "description": "The `sellAmount` for the order.",
    ///                "allOf": [
    ///                  {
    ///                    "$ref": "#/components/schemas/TokenAmount"
    ///                  }
    ///                ]
    ///              }
    ///            }
    ///          },
    ///          {
    ///            "not": {
    ///              "description": "Quote a sell order given the final total `sellAmount` including fees.",
    ///              "type": "object",
    ///              "required": [
    ///                "kind",
    ///                "sellAmountBeforeFee"
    ///              ],
    ///              "properties": {
    ///                "kind": {
    ///                  "allOf": [
    ///                    {
    ///                      "$ref": "#/components/schemas/OrderQuoteSideKindSell"
    ///                    }
    ///                  ]
    ///                },
    ///                "sellAmountBeforeFee": {
    ///                  "description": "The total amount that is available for the order. From this value, the fee is deducted and the buy amount is calculated.\n",
    ///                  "allOf": [
    ///                    {
    ///                      "$ref": "#/components/schemas/TokenAmount"
    ///                    }
    ///                  ]
    ///                }
    ///              }
    ///            }
    ///          },
    ///          {
    ///            "not": {
    ///              "description": "Quote a buy order given an exact `buyAmount`.",
    ///              "type": "object",
    ///              "required": [
    ///                "buyAmountAfterFee",
    ///                "kind"
    ///              ],
    ///              "properties": {
    ///                "buyAmountAfterFee": {
    ///                  "description": "The `buyAmount` for the order.",
    ///                  "allOf": [
    ///                    {
    ///                      "$ref": "#/components/schemas/TokenAmount"
    ///                    }
    ///                  ]
    ///                },
    ///                "kind": {
    ///                  "allOf": [
    ///                    {
    ///                      "$ref": "#/components/schemas/OrderQuoteSideKindBuy"
    ///                    }
    ///                  ]
    ///                }
    ///              }
    ///            }
    ///          }
    ///        ]
    ///      }
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    #[serde(untagged)]
    pub enum OrderQuoteRequestVariant2 {
        Variant0 {
            /**AppData which will be assigned to the order.

            Expects either a string JSON doc as defined on
            [AppData](https://github.com/cowprotocol/app-data) or a hex
            encoded string for backwards compatibility.

            When the first format is used, it's possible to provide the
            derived appDataHash field.*/
            #[serde(
                rename = "appData",
                default,
                skip_serializing_if = "::std::option::Option::is_none"
            )]
            app_data: ::std::option::Option<OrderQuoteRequestVariant2Variant0AppData>,
            #[serde(
                rename = "appDataHash",
                default,
                skip_serializing_if = "::std::option::Option::is_none"
            )]
            app_data_hash: ::std::option::Option<AppDataHash>,
            ///The `buyAmount` for the order.
            #[serde(rename = "buyAmountAfterFee")]
            buy_amount_after_fee: TokenAmount,
            ///ERC-20 token to be bought
            #[serde(rename = "buyToken")]
            buy_token: Address,
            #[serde(
                rename = "buyTokenBalance",
                default = "defaults::order_quote_request_variant2_variant0_buy_token_balance"
            )]
            buy_token_balance: BuyTokenDestination,
            from: Address,
            kind: OrderQuoteSideKindBuy,
            /**Flag to signal whether the order is intended for on-chain order placement. Only valid for non ECDSA-signed orders."
             */
            #[serde(
                rename = "onchainOrder",
                default = "defaults::order_quote_request_variant2_variant0_onchain_order"
            )]
            onchain_order: ::serde_json::Value,
            #[serde(
                rename = "priceQuality",
                default = "defaults::order_quote_request_variant2_variant0_price_quality"
            )]
            price_quality: PriceQuality,
            /**An optional address to receive the proceeds of the trade instead of the
            `owner` (i.e. the order signer).
            */
            #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
            receiver: ::std::option::Option<Address>,
            ///ERC-20 token to be sold
            #[serde(rename = "sellToken")]
            sell_token: Address,
            #[serde(
                rename = "sellTokenBalance",
                default = "defaults::order_quote_request_variant2_variant0_sell_token_balance"
            )]
            sell_token_balance: SellTokenSource,
            #[serde(
                rename = "signingScheme",
                default = "defaults::order_quote_request_variant2_variant0_signing_scheme"
            )]
            signing_scheme: SigningScheme,
            /**User provided timeout in milliseconds. If no value is provided the systems default quote timeout will be used. Values get capped at a generous maximum timeout. Note that reducing the timeout can result in worse quotes because it might be too short for some price estimators.
             */
            #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
            timeout: ::std::option::Option<i64>,
            ///Unix timestamp (`uint32`) until which the order is valid.
            #[serde(
                rename = "validTo",
                default,
                skip_serializing_if = "::std::option::Option::is_none"
            )]
            valid_to: ::std::option::Option<i64>,
        },
        Variant1 {
            /**AppData which will be assigned to the order.

            Expects either a string JSON doc as defined on
            [AppData](https://github.com/cowprotocol/app-data) or a hex
            encoded string for backwards compatibility.

            When the first format is used, it's possible to provide the
            derived appDataHash field.*/
            #[serde(
                rename = "appData",
                default,
                skip_serializing_if = "::std::option::Option::is_none"
            )]
            app_data: ::std::option::Option<OrderQuoteRequestVariant2Variant1AppData>,
            #[serde(
                rename = "appDataHash",
                default,
                skip_serializing_if = "::std::option::Option::is_none"
            )]
            app_data_hash: ::std::option::Option<AppDataHash>,
            ///The `buyAmount` for the order.
            #[serde(rename = "buyAmountAfterFee")]
            buy_amount_after_fee: TokenAmount,
            ///ERC-20 token to be bought
            #[serde(rename = "buyToken")]
            buy_token: Address,
            #[serde(
                rename = "buyTokenBalance",
                default = "defaults::order_quote_request_variant2_variant1_buy_token_balance"
            )]
            buy_token_balance: BuyTokenDestination,
            from: Address,
            kind: OrderQuoteSideKindBuy,
            /**Flag to signal whether the order is intended for on-chain order placement. Only valid for non ECDSA-signed orders."
             */
            #[serde(
                rename = "onchainOrder",
                default = "defaults::order_quote_request_variant2_variant1_onchain_order"
            )]
            onchain_order: ::serde_json::Value,
            #[serde(
                rename = "priceQuality",
                default = "defaults::order_quote_request_variant2_variant1_price_quality"
            )]
            price_quality: PriceQuality,
            /**An optional address to receive the proceeds of the trade instead of the
            `owner` (i.e. the order signer).
            */
            #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
            receiver: ::std::option::Option<Address>,
            ///ERC-20 token to be sold
            #[serde(rename = "sellToken")]
            sell_token: Address,
            #[serde(
                rename = "sellTokenBalance",
                default = "defaults::order_quote_request_variant2_variant1_sell_token_balance"
            )]
            sell_token_balance: SellTokenSource,
            #[serde(
                rename = "signingScheme",
                default = "defaults::order_quote_request_variant2_variant1_signing_scheme"
            )]
            signing_scheme: SigningScheme,
            /**User provided timeout in milliseconds. If no value is provided the systems default quote timeout will be used. Values get capped at a generous maximum timeout. Note that reducing the timeout can result in worse quotes because it might be too short for some price estimators.
             */
            #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
            timeout: ::std::option::Option<i64>,
            ///Number (`uint32`) of seconds that the order should be valid for.
            #[serde(
                rename = "validFor",
                default,
                skip_serializing_if = "::std::option::Option::is_none"
            )]
            valid_for: ::std::option::Option<i64>,
        },
    }
    /**AppData which will be assigned to the order.

    Expects either a string JSON doc as defined on
    [AppData](https://github.com/cowprotocol/app-data) or a hex
    encoded string for backwards compatibility.

    When the first format is used, it's possible to provide the
    derived appDataHash field.*/
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "AppData which will be assigned to the order.\n\nExpects either a string JSON doc as defined on\n[AppData](https://github.com/cowprotocol/app-data) or a hex\nencoded string for backwards compatibility.\n\nWhen the first format is used, it's possible to provide the\nderived appDataHash field.",
    ///  "oneOf": [
    ///    {
    ///      "$ref": "#/components/schemas/AppData"
    ///    },
    ///    {
    ///      "$ref": "#/components/schemas/AppDataHash"
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    #[serde(untagged)]
    pub enum OrderQuoteRequestVariant2Variant0AppData {
        Variant0(AppData),
        Variant1(AppDataHash),
    }
    impl ::std::fmt::Display for OrderQuoteRequestVariant2Variant0AppData {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                Self::Variant0(x) => x.fmt(f),
                Self::Variant1(x) => x.fmt(f),
            }
        }
    }
    impl ::std::convert::From<AppData> for OrderQuoteRequestVariant2Variant0AppData {
        fn from(value: AppData) -> Self {
            Self::Variant0(value)
        }
    }
    impl ::std::convert::From<AppDataHash> for OrderQuoteRequestVariant2Variant0AppData {
        fn from(value: AppDataHash) -> Self {
            Self::Variant1(value)
        }
    }
    /**AppData which will be assigned to the order.

    Expects either a string JSON doc as defined on
    [AppData](https://github.com/cowprotocol/app-data) or a hex
    encoded string for backwards compatibility.

    When the first format is used, it's possible to provide the
    derived appDataHash field.*/
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "AppData which will be assigned to the order.\n\nExpects either a string JSON doc as defined on\n[AppData](https://github.com/cowprotocol/app-data) or a hex\nencoded string for backwards compatibility.\n\nWhen the first format is used, it's possible to provide the\nderived appDataHash field.",
    ///  "oneOf": [
    ///    {
    ///      "$ref": "#/components/schemas/AppData"
    ///    },
    ///    {
    ///      "$ref": "#/components/schemas/AppDataHash"
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    #[serde(untagged)]
    pub enum OrderQuoteRequestVariant2Variant1AppData {
        Variant0(AppData),
        Variant1(AppDataHash),
    }
    impl ::std::fmt::Display for OrderQuoteRequestVariant2Variant1AppData {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                Self::Variant0(x) => x.fmt(f),
                Self::Variant1(x) => x.fmt(f),
            }
        }
    }
    impl ::std::convert::From<AppData> for OrderQuoteRequestVariant2Variant1AppData {
        fn from(value: AppData) -> Self {
            Self::Variant0(value)
        }
    }
    impl ::std::convert::From<AppDataHash> for OrderQuoteRequestVariant2Variant1AppData {
        fn from(value: AppDataHash) -> Self {
            Self::Variant1(value)
        }
    }
    /**An order quoted by the backend that can be directly signed and
    submitted to the order creation backend.
    */
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "An order quoted by the backend that can be directly signed and\nsubmitted to the order creation backend.\n",
    ///  "type": "object",
    ///  "required": [
    ///    "expiration",
    ///    "quote",
    ///    "verified"
    ///  ],
    ///  "properties": {
    ///    "expiration": {
    ///      "description": "Expiration date of the offered fee. Order service might not accept\nthe fee after this expiration date. Encoded as ISO 8601 UTC.\n",
    ///      "examples": [
    ///        "1985-03-10T18:35:18.814523Z"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "from": {
    ///      "description": "The address of the trader for whom the quote was requested.\n",
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/Address"
    ///        }
    ///      ]
    ///    },
    ///    "id": {
    ///      "description": "Quote ID linked to a quote to enable providing more metadata when analysing order slippage.\n",
    ///      "type": "integer"
    ///    },
    ///    "protocolFeeBps": {
    ///      "description": "Protocol fee in basis points (e.g., \"2\" for 0.02%). This represents the volume-based fee policy. Only present when a volume fee is configured.\n",
    ///      "examples": [
    ///        "2"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "quote": {
    ///      "description": "The quoted order parameters. These values can be used directly to create and sign an order.\n",
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/OrderParameters"
    ///        }
    ///      ]
    ///    },
    ///    "verified": {
    ///      "description": "Whether it was possible to verify that the quoted amounts are accurate using a simulation.\n",
    ///      "type": "boolean"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct OrderQuoteResponse {
        /**Expiration date of the offered fee. Order service might not accept
        the fee after this expiration date. Encoded as ISO 8601 UTC.
        */
        pub expiration: ::std::string::String,
        /**The address of the trader for whom the quote was requested.
         */
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub from: ::std::option::Option<Address>,
        /**Quote ID linked to a quote to enable providing more metadata when analysing order slippage.
         */
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub id: ::std::option::Option<i64>,
        /**Protocol fee in basis points (e.g., "2" for 0.02%). This represents the volume-based fee policy. Only present when a volume fee is configured.
         */
        #[serde(
            rename = "protocolFeeBps",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub protocol_fee_bps: ::std::option::Option<::std::string::String>,
        /**The quoted order parameters. These values can be used directly to create and sign an order.
         */
        pub quote: OrderParameters,
        /**Whether it was possible to verify that the quoted amounts are accurate using a simulation.
         */
        pub verified: bool,
    }
    ///The buy or sell side when quoting an order.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "The buy or sell side when quoting an order.",
    ///  "oneOf": [
    ///    {
    ///      "description": "Quote a sell order given the final total `sellAmount` including fees.",
    ///      "type": "object",
    ///      "required": [
    ///        "kind",
    ///        "sellAmountBeforeFee"
    ///      ],
    ///      "properties": {
    ///        "kind": {
    ///          "allOf": [
    ///            {
    ///              "$ref": "#/components/schemas/OrderQuoteSideKindSell"
    ///            }
    ///          ]
    ///        },
    ///        "sellAmountBeforeFee": {
    ///          "description": "The total amount that is available for the order. From this value, the fee is deducted and the buy amount is calculated.\n",
    ///          "allOf": [
    ///            {
    ///              "$ref": "#/components/schemas/TokenAmount"
    ///            }
    ///          ]
    ///        }
    ///      }
    ///    },
    ///    {
    ///      "description": "Quote a sell order given the `sellAmount`.",
    ///      "type": "object",
    ///      "required": [
    ///        "kind",
    ///        "sellAmountAfterFee"
    ///      ],
    ///      "properties": {
    ///        "kind": {
    ///          "allOf": [
    ///            {
    ///              "$ref": "#/components/schemas/OrderQuoteSideKindSell"
    ///            }
    ///          ]
    ///        },
    ///        "sellAmountAfterFee": {
    ///          "description": "The `sellAmount` for the order.",
    ///          "allOf": [
    ///            {
    ///              "$ref": "#/components/schemas/TokenAmount"
    ///            }
    ///          ]
    ///        }
    ///      }
    ///    },
    ///    {
    ///      "description": "Quote a buy order given an exact `buyAmount`.",
    ///      "type": "object",
    ///      "required": [
    ///        "buyAmountAfterFee",
    ///        "kind"
    ///      ],
    ///      "properties": {
    ///        "buyAmountAfterFee": {
    ///          "description": "The `buyAmount` for the order.",
    ///          "allOf": [
    ///            {
    ///              "$ref": "#/components/schemas/TokenAmount"
    ///            }
    ///          ]
    ///        },
    ///        "kind": {
    ///          "allOf": [
    ///            {
    ///              "$ref": "#/components/schemas/OrderQuoteSideKindBuy"
    ///            }
    ///          ]
    ///        }
    ///      }
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    #[serde(untagged)]
    pub enum OrderQuoteSide {
        Variant0 {
            kind: OrderQuoteSideKindSell,
            /**The total amount that is available for the order. From this value, the fee is deducted and the buy amount is calculated.
             */
            #[serde(rename = "sellAmountBeforeFee")]
            sell_amount_before_fee: TokenAmount,
        },
        Variant1 {
            kind: OrderQuoteSideKindSell,
            ///The `sellAmount` for the order.
            #[serde(rename = "sellAmountAfterFee")]
            sell_amount_after_fee: TokenAmount,
        },
        Variant2 {
            ///The `buyAmount` for the order.
            #[serde(rename = "buyAmountAfterFee")]
            buy_amount_after_fee: TokenAmount,
            kind: OrderQuoteSideKindBuy,
        },
    }
    ///`OrderQuoteSideKindBuy`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "buy"
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
    pub enum OrderQuoteSideKindBuy {
        #[serde(rename = "buy")]
        Buy,
    }
    impl ::std::fmt::Display for OrderQuoteSideKindBuy {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Buy => f.write_str("buy"),
            }
        }
    }
    impl ::std::str::FromStr for OrderQuoteSideKindBuy {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "buy" => Ok(Self::Buy),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for OrderQuoteSideKindBuy {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String> for OrderQuoteSideKindBuy {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for OrderQuoteSideKindBuy {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///`OrderQuoteSideKindSell`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "sell"
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
    pub enum OrderQuoteSideKindSell {
        #[serde(rename = "sell")]
        Sell,
    }
    impl ::std::fmt::Display for OrderQuoteSideKindSell {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Sell => f.write_str("sell"),
            }
        }
    }
    impl ::std::str::FromStr for OrderQuoteSideKindSell {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "sell" => Ok(Self::Sell),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for OrderQuoteSideKindSell {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String> for OrderQuoteSideKindSell {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for OrderQuoteSideKindSell {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///The validity for the order.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "The validity for the order.",
    ///  "oneOf": [
    ///    {
    ///      "description": "Absolute validity.",
    ///      "type": "object",
    ///      "properties": {
    ///        "validTo": {
    ///          "description": "Unix timestamp (`uint32`) until which the order is valid.",
    ///          "type": "integer"
    ///        }
    ///      }
    ///    },
    ///    {
    ///      "description": "Relative validity",
    ///      "type": "object",
    ///      "properties": {
    ///        "validFor": {
    ///          "description": "Number (`uint32`) of seconds that the order should be valid for.",
    ///          "type": "integer"
    ///        }
    ///      }
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    #[serde(untagged)]
    pub enum OrderQuoteValidity {
        Variant0 {
            ///Unix timestamp (`uint32`) until which the order is valid.
            #[serde(
                rename = "validTo",
                default,
                skip_serializing_if = "::std::option::Option::is_none"
            )]
            valid_to: ::std::option::Option<i64>,
        },
        Variant1 {
            ///Number (`uint32`) of seconds that the order should be valid for.
            #[serde(
                rename = "validFor",
                default,
                skip_serializing_if = "::std::option::Option::is_none"
            )]
            valid_for: ::std::option::Option<i64>,
        },
    }
    /**The Tenderly simulation request for an order, along with any simulation error.
     */
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "The Tenderly simulation request for an order, along with any simulation error.\n",
    ///  "type": "object",
    ///  "required": [
    ///    "tenderly_request"
    ///  ],
    ///  "properties": {
    ///    "error": {
    ///      "description": "Simulation error message, if the simulation failed.",
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "tenderly_request": {
    ///      "$ref": "#/components/schemas/TenderlyRequest"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct OrderSimulation {
        ///Simulation error message, if the simulation failed.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub error: ::std::option::Option<::std::string::String>,
        pub tenderly_request: TenderlyRequest,
    }
    ///The current order status.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "The current order status.",
    ///  "type": "string",
    ///  "enum": [
    ///    "presignaturePending",
    ///    "open",
    ///    "fulfilled",
    ///    "cancelled",
    ///    "expired"
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
    pub enum OrderStatus {
        #[serde(rename = "presignaturePending")]
        PresignaturePending,
        #[serde(rename = "open")]
        Open,
        #[serde(rename = "fulfilled")]
        Fulfilled,
        #[serde(rename = "cancelled")]
        Cancelled,
        #[serde(rename = "expired")]
        Expired,
    }
    impl ::std::fmt::Display for OrderStatus {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::PresignaturePending => f.write_str("presignaturePending"),
                Self::Open => f.write_str("open"),
                Self::Fulfilled => f.write_str("fulfilled"),
                Self::Cancelled => f.write_str("cancelled"),
                Self::Expired => f.write_str("expired"),
            }
        }
    }
    impl ::std::str::FromStr for OrderStatus {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "presignaturePending" => Ok(Self::PresignaturePending),
                "open" => Ok(Self::Open),
                "fulfilled" => Ok(Self::Fulfilled),
                "cancelled" => Ok(Self::Cancelled),
                "expired" => Ok(Self::Expired),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for OrderStatus {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String> for OrderStatus {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for OrderStatus {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///Empty signature bytes. Used for "presign" signatures.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Empty signature bytes. Used for \"presign\" signatures.",
    ///  "examples": [
    ///    "0x"
    ///  ],
    ///  "type": "string"
    ///}
    /// ```
    /// </details>
    #[derive(
        ::serde::Deserialize, ::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd,
    )]
    #[serde(transparent)]
    pub struct PreSignature(pub ::std::string::String);
    impl ::std::ops::Deref for PreSignature {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }
    impl ::std::convert::From<PreSignature> for ::std::string::String {
        fn from(value: PreSignature) -> Self {
            value.0
        }
    }
    impl ::std::convert::From<::std::string::String> for PreSignature {
        fn from(value: ::std::string::String) -> Self {
            Self(value)
        }
    }
    impl ::std::str::FromStr for PreSignature {
        type Err = ::std::convert::Infallible;
        fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
            Ok(Self(value.to_string()))
        }
    }
    impl ::std::fmt::Display for PreSignature {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            self.0.fmt(f)
        }
    }
    ///`PriceEstimationError`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "description",
    ///    "errorType"
    ///  ],
    ///  "properties": {
    ///    "description": {
    ///      "type": "string"
    ///    },
    ///    "errorType": {
    ///      "type": "string",
    ///      "enum": [
    ///        "QuoteNotVerified",
    ///        "UnsupportedToken",
    ///        "NoLiquidity",
    ///        "UnsupportedOrderType"
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct PriceEstimationError {
        pub description: ::std::string::String,
        #[serde(rename = "errorType")]
        pub error_type: PriceEstimationErrorErrorType,
    }
    ///`PriceEstimationErrorErrorType`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "QuoteNotVerified",
    ///    "UnsupportedToken",
    ///    "NoLiquidity",
    ///    "UnsupportedOrderType"
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
    pub enum PriceEstimationErrorErrorType {
        QuoteNotVerified,
        UnsupportedToken,
        NoLiquidity,
        UnsupportedOrderType,
    }
    impl ::std::fmt::Display for PriceEstimationErrorErrorType {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::QuoteNotVerified => f.write_str("QuoteNotVerified"),
                Self::UnsupportedToken => f.write_str("UnsupportedToken"),
                Self::NoLiquidity => f.write_str("NoLiquidity"),
                Self::UnsupportedOrderType => f.write_str("UnsupportedOrderType"),
            }
        }
    }
    impl ::std::str::FromStr for PriceEstimationErrorErrorType {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "QuoteNotVerified" => Ok(Self::QuoteNotVerified),
                "UnsupportedToken" => Ok(Self::UnsupportedToken),
                "NoLiquidity" => Ok(Self::NoLiquidity),
                "UnsupportedOrderType" => Ok(Self::UnsupportedOrderType),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for PriceEstimationErrorErrorType {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String> for PriceEstimationErrorErrorType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for PriceEstimationErrorErrorType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///The protocol fee is taken as a percent of the order price improvement which is a difference between the executed price and the best quote.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "The protocol fee is taken as a percent of the order price improvement which is a difference between the executed price and the best quote.",
    ///  "type": "object",
    ///  "required": [
    ///    "factor",
    ///    "maxVolumeFactor",
    ///    "quote"
    ///  ],
    ///  "properties": {
    ///    "factor": {
    ///      "type": "number",
    ///      "exclusiveMaximum": 1.0,
    ///      "minimum": 0.0
    ///    },
    ///    "maxVolumeFactor": {
    ///      "type": "number",
    ///      "exclusiveMaximum": 1.0,
    ///      "minimum": 0.0
    ///    },
    ///    "quote": {
    ///      "description": "The best quote received.",
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/Quote"
    ///        }
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct PriceImprovement {
        pub factor: f64,
        #[serde(rename = "maxVolumeFactor")]
        pub max_volume_factor: f64,
        ///The best quote received.
        pub quote: Quote,
    }
    /**How good should the price estimate be?

    Fast: The price estimate is chosen among the fastest N price estimates.
    Optimal: The price estimate is chosen among all price estimates.
    Verified: The price estimate is chosen among all verified/simulated
    price estimates.

    **NOTE**: Orders are supposed to be created from `verified` or `optimal`
    price estimates.*/
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "How good should the price estimate be?\n\nFast: The price estimate is chosen among the fastest N price estimates.\nOptimal: The price estimate is chosen among all price estimates.\nVerified: The price estimate is chosen among all verified/simulated\nprice estimates.\n\n**NOTE**: Orders are supposed to be created from `verified` or `optimal`\nprice estimates.",
    ///  "type": "string",
    ///  "enum": [
    ///    "fast",
    ///    "optimal",
    ///    "verified"
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
    pub enum PriceQuality {
        #[serde(rename = "fast")]
        Fast,
        #[serde(rename = "optimal")]
        Optimal,
        #[serde(rename = "verified")]
        Verified,
    }
    impl ::std::fmt::Display for PriceQuality {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Fast => f.write_str("fast"),
                Self::Optimal => f.write_str("optimal"),
                Self::Verified => f.write_str("verified"),
            }
        }
    }
    impl ::std::str::FromStr for PriceQuality {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "fast" => Ok(Self::Fast),
                "optimal" => Ok(Self::Optimal),
                "verified" => Ok(Self::Verified),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for PriceQuality {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String> for PriceQuality {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for PriceQuality {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    /**A calculated order quote used in solver auctions.
     */
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "A calculated order quote used in solver auctions.\n",
    ///  "type": "object",
    ///  "properties": {
    ///    "buyAmount": {
    ///      "description": "The amount of the buy token.",
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/TokenAmount"
    ///        }
    ///      ]
    ///    },
    ///    "fee": {
    ///      "description": "The amount that needs to be paid, denominated in the sell token.",
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/TokenAmount"
    ///        }
    ///      ]
    ///    },
    ///    "sellAmount": {
    ///      "description": "The amount of the sell token.",
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/TokenAmount"
    ///        }
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct Quote {
        ///The amount of the buy token.
        #[serde(
            rename = "buyAmount",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub buy_amount: ::std::option::Option<TokenAmount>,
        ///The amount that needs to be paid, denominated in the sell token.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub fee: ::std::option::Option<TokenAmount>,
        ///The amount of the sell token.
        #[serde(
            rename = "sellAmount",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub sell_amount: ::std::option::Option<TokenAmount>,
    }
    impl ::std::default::Default for Quote {
        fn default() -> Self {
            Self {
                buy_amount: Default::default(),
                fee: Default::default(),
                sell_amount: Default::default(),
            }
        }
    }
    ///Where should the `sellToken` be drawn from?
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Where should the `sellToken` be drawn from?",
    ///  "type": "string",
    ///  "enum": [
    ///    "erc20",
    ///    "internal",
    ///    "external"
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
    pub enum SellTokenSource {
        #[serde(rename = "erc20")]
        Erc20,
        #[serde(rename = "internal")]
        Internal,
        #[serde(rename = "external")]
        External,
    }
    impl ::std::fmt::Display for SellTokenSource {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Erc20 => f.write_str("erc20"),
                Self::Internal => f.write_str("internal"),
                Self::External => f.write_str("external"),
            }
        }
    }
    impl ::std::str::FromStr for SellTokenSource {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "erc20" => Ok(Self::Erc20),
                "internal" => Ok(Self::Internal),
                "external" => Ok(Self::External),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for SellTokenSource {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String> for SellTokenSource {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for SellTokenSource {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///A signature.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "A signature.",
    ///  "oneOf": [
    ///    {
    ///      "$ref": "#/components/schemas/EcdsaSignature"
    ///    },
    ///    {
    ///      "$ref": "#/components/schemas/PreSignature"
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    #[serde(untagged)]
    pub enum Signature {
        EcdsaSignature(EcdsaSignature),
        PreSignature(PreSignature),
    }
    impl ::std::fmt::Display for Signature {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                Self::EcdsaSignature(x) => x.fmt(f),
                Self::PreSignature(x) => x.fmt(f),
            }
        }
    }
    impl ::std::convert::From<EcdsaSignature> for Signature {
        fn from(value: EcdsaSignature) -> Self {
            Self::EcdsaSignature(value)
        }
    }
    impl ::std::convert::From<PreSignature> for Signature {
        fn from(value: PreSignature) -> Self {
            Self::PreSignature(value)
        }
    }
    ///How was the order signed?
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "How was the order signed?",
    ///  "type": "string",
    ///  "enum": [
    ///    "eip712",
    ///    "ethsign",
    ///    "presign",
    ///    "eip1271"
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
    pub enum SigningScheme {
        #[serde(rename = "eip712")]
        Eip712,
        #[serde(rename = "ethsign")]
        Ethsign,
        #[serde(rename = "presign")]
        Presign,
        #[serde(rename = "eip1271")]
        Eip1271,
    }
    impl ::std::fmt::Display for SigningScheme {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Eip712 => f.write_str("eip712"),
                Self::Ethsign => f.write_str("ethsign"),
                Self::Presign => f.write_str("presign"),
                Self::Eip1271 => f.write_str("eip1271"),
            }
        }
    }
    impl ::std::str::FromStr for SigningScheme {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "eip712" => Ok(Self::Eip712),
                "ethsign" => Ok(Self::Ethsign),
                "presign" => Ok(Self::Presign),
                "eip1271" => Ok(Self::Eip1271),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for SigningScheme {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String> for SigningScheme {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for SigningScheme {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    /**Request body for simulating an arbitrary order without it being stored in the orderbook.
     */
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Request body for simulating an arbitrary order without it being stored in the orderbook.\n",
    ///  "type": "object",
    ///  "required": [
    ///    "buyAmount",
    ///    "buyToken",
    ///    "kind",
    ///    "owner",
    ///    "sellAmount",
    ///    "sellToken"
    ///  ],
    ///  "properties": {
    ///    "appData": {
    ///      "description": "Full app data JSON string. Defaults to `\"{}\"` if omitted.\n",
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "blockNumber": {
    ///      "type": "integer"
    ///    },
    ///    "buyAmount": {
    ///      "description": "Amount of buy token (hex- or decimal-encoded uint256).",
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/TokenAmount"
    ///        }
    ///      ]
    ///    },
    ///    "buyToken": {
    ///      "description": "The token being bought.",
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/Address"
    ///        }
    ///      ]
    ///    },
    ///    "buyTokenBalance": {
    ///      "description": "Where the buy token should be transferred to.",
    ///      "default": "erc20",
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/BuyTokenDestination"
    ///        }
    ///      ]
    ///    },
    ///    "kind": {
    ///      "description": "Whether this is a sell or buy order.",
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/OrderKind"
    ///        }
    ///      ]
    ///    },
    ///    "owner": {
    ///      "description": "The address of the order owner.",
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/Address"
    ///        }
    ///      ]
    ///    },
    ///    "receiver": {
    ///      "description": "The address that will receive the buy tokens. Defaults to the owner if omitted.\n",
    ///      "oneOf": [
    ///        {
    ///          "type": "null"
    ///        },
    ///        {
    ///          "allOf": [
    ///            {
    ///              "$ref": "#/components/schemas/Address"
    ///            }
    ///          ]
    ///        }
    ///      ]
    ///    },
    ///    "sellAmount": {
    ///      "description": "Amount of sell token (hex- or decimal-encoded uint256). Must be greater than zero.\n",
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/TokenAmount"
    ///        }
    ///      ]
    ///    },
    ///    "sellToken": {
    ///      "description": "The token being sold.",
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/Address"
    ///        }
    ///      ]
    ///    },
    ///    "sellTokenBalance": {
    ///      "description": "Where the sell token should be drawn from.",
    ///      "default": "erc20",
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/SellTokenSource"
    ///        }
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct SimulationRequest {
        /**Full app data JSON string. Defaults to `"{}"` if omitted.
         */
        #[serde(
            rename = "appData",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub app_data: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "blockNumber",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub block_number: ::std::option::Option<i64>,
        ///Amount of buy token (hex- or decimal-encoded uint256).
        #[serde(rename = "buyAmount")]
        pub buy_amount: TokenAmount,
        ///The token being bought.
        #[serde(rename = "buyToken")]
        pub buy_token: Address,
        ///Where the buy token should be transferred to.
        #[serde(
            rename = "buyTokenBalance",
            default = "defaults::simulation_request_buy_token_balance"
        )]
        pub buy_token_balance: BuyTokenDestination,
        ///Whether this is a sell or buy order.
        pub kind: OrderKind,
        ///The address of the order owner.
        pub owner: Address,
        /**The address that will receive the buy tokens. Defaults to the owner if omitted.
         */
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub receiver: ::std::option::Option<Address>,
        /**Amount of sell token (hex- or decimal-encoded uint256). Must be greater than zero.
         */
        #[serde(rename = "sellAmount")]
        pub sell_amount: TokenAmount,
        ///The token being sold.
        #[serde(rename = "sellToken")]
        pub sell_token: Address,
        ///Where the sell token should be drawn from.
        #[serde(
            rename = "sellTokenBalance",
            default = "defaults::simulation_request_sell_token_balance"
        )]
        pub sell_token_balance: SellTokenSource,
    }
    ///The kind of Tenderly simulation to run.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "The kind of Tenderly simulation to run.",
    ///  "type": "string",
    ///  "enum": [
    ///    "full",
    ///    "quick"
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
    pub enum SimulationType {
        #[serde(rename = "full")]
        Full,
        #[serde(rename = "quick")]
        Quick,
    }
    impl ::std::fmt::Display for SimulationType {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Full => f.write_str("full"),
                Self::Quick => f.write_str("quick"),
            }
        }
    }
    impl ::std::str::FromStr for SimulationType {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "full" => Ok(Self::Full),
                "quick" => Ok(Self::Quick),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for SimulationType {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String> for SimulationType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for SimulationType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    /**The settlements submitted by every solver for a specific auction.
    The `auctionId` corresponds to the id external solvers are provided
    with.
    */
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "The settlements submitted by every solver for a specific auction.\nThe `auctionId` corresponds to the id external solvers are provided\nwith.\n",
    ///  "type": "object",
    ///  "properties": {
    ///    "auction": {
    ///      "$ref": "#/components/schemas/CompetitionAuction"
    ///    },
    ///    "auctionDeadlineBlock": {
    ///      "description": "Block deadline by which the auction must be settled.",
    ///      "type": "integer"
    ///    },
    ///    "auctionId": {
    ///      "description": "The ID of the auction the competition info is for.",
    ///      "type": "integer"
    ///    },
    ///    "auctionStartBlock": {
    ///      "description": "Block that the auction started on.",
    ///      "type": "integer"
    ///    },
    ///    "referenceScores": {
    ///      "description": "The reference scores for each winning solver according to [CIP-67](https://forum.cow.fi/t/cip-67-moving-from-batch-auction-to-the-fair-combinatorial-auction/2967) (if available).\n",
    ///      "type": "object",
    ///      "additionalProperties": {
    ///        "$ref": "#/components/schemas/BigUint"
    ///      }
    ///    },
    ///    "solutions": {
    ///      "description": "Maps from solver name to object describing that solver's settlement.",
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/SolverSettlement"
    ///      }
    ///    },
    ///    "transactionHashes": {
    ///      "description": "The hashes of the transactions for the winning solutions of this competition.\n",
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/TransactionHash"
    ///      }
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct SolverCompetitionResponse {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub auction: ::std::option::Option<CompetitionAuction>,
        ///Block deadline by which the auction must be settled.
        #[serde(
            rename = "auctionDeadlineBlock",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub auction_deadline_block: ::std::option::Option<i64>,
        ///The ID of the auction the competition info is for.
        #[serde(
            rename = "auctionId",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub auction_id: ::std::option::Option<i64>,
        ///Block that the auction started on.
        #[serde(
            rename = "auctionStartBlock",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub auction_start_block: ::std::option::Option<i64>,
        /**The reference scores for each winning solver according to [CIP-67](https://forum.cow.fi/t/cip-67-moving-from-batch-auction-to-the-fair-combinatorial-auction/2967) (if available).
         */
        #[serde(
            rename = "referenceScores",
            default,
            skip_serializing_if = ":: std :: collections :: HashMap::is_empty"
        )]
        pub reference_scores: ::std::collections::HashMap<::std::string::String, BigUint>,
        ///Maps from solver name to object describing that solver's settlement.
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub solutions: ::std::vec::Vec<SolverSettlement>,
        /**The hashes of the transactions for the winning solutions of this competition.
         */
        #[serde(
            rename = "transactionHashes",
            default,
            skip_serializing_if = "::std::vec::Vec::is_empty"
        )]
        pub transaction_hashes: ::std::vec::Vec<TransactionHash>,
    }
    impl ::std::default::Default for SolverCompetitionResponse {
        fn default() -> Self {
            Self {
                auction: Default::default(),
                auction_deadline_block: Default::default(),
                auction_id: Default::default(),
                auction_start_block: Default::default(),
                reference_scores: Default::default(),
                solutions: Default::default(),
                transaction_hashes: Default::default(),
            }
        }
    }
    ///`SolverSettlement`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "clearingPrices": {
    ///      "description": "The prices of tokens for settled user orders as passed to the settlement contract.\n",
    ///      "type": "object",
    ///      "additionalProperties": {
    ///        "$ref": "#/components/schemas/BigUint"
    ///      }
    ///    },
    ///    "filteredOut": {
    ///      "description": "whether the solution was filtered out according to the rules of [CIP-67](https://forum.cow.fi/t/cip-67-moving-from-batch-auction-to-the-fair-combinatorial-auction/2967).",
    ///      "type": "boolean"
    ///    },
    ///    "isWinner": {
    ///      "description": "whether the solution is a winner (received the right to get executed) or not",
    ///      "type": "boolean"
    ///    },
    ///    "orders": {
    ///      "description": "Touched orders.",
    ///      "type": "array",
    ///      "items": {
    ///        "type": "object",
    ///        "properties": {
    ///          "buyAmount": {
    ///            "$ref": "#/components/schemas/BigUint"
    ///          },
    ///          "id": {
    ///            "$ref": "#/components/schemas/UID"
    ///          },
    ///          "sellAmount": {
    ///            "$ref": "#/components/schemas/BigUint"
    ///          }
    ///        }
    ///      }
    ///    },
    ///    "ranking": {
    ///      "description": "Which position the solution achieved in the total ranking of the competition.",
    ///      "type": "number"
    ///    },
    ///    "referenceScore": {
    ///      "description": "The reference score as defined in [CIP-67](https://forum.cow.fi/t/cip-67-moving-from-batch-auction-to-the-fair-combinatorial-auction/2967) (if available).\n",
    ///      "oneOf": [
    ///        {
    ///          "type": "null"
    ///        },
    ///        {
    ///          "allOf": [
    ///            {
    ///              "$ref": "#/components/schemas/BigUint"
    ///            }
    ///          ]
    ///        }
    ///      ]
    ///    },
    ///    "score": {
    ///      "description": "The score of the current auction as defined in [CIP-20](https://snapshot.org/#/cow.eth/proposal/0x2d3f9bd1ea72dca84b03e97dda3efc1f4a42a772c54bd2037e8b62e7d09a491f).\n",
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/BigUint"
    ///        }
    ///      ]
    ///    },
    ///    "solverAddress": {
    ///      "description": "The address used by the solver to execute the settlement on-chain.\n\nThis field is missing for old settlements, the zero address has been\nused instead.",
    ///      "type": "string"
    ///    },
    ///    "txHash": {
    ///      "description": "Transaction in which the solution was executed onchain (if available).\n",
    ///      "oneOf": [
    ///        {
    ///          "type": "null"
    ///        },
    ///        {
    ///          "allOf": [
    ///            {
    ///              "$ref": "#/components/schemas/TransactionHash"
    ///            }
    ///          ]
    ///        }
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct SolverSettlement {
        /**The prices of tokens for settled user orders as passed to the settlement contract.
         */
        #[serde(
            rename = "clearingPrices",
            default,
            skip_serializing_if = ":: std :: collections :: HashMap::is_empty"
        )]
        pub clearing_prices: ::std::collections::HashMap<::std::string::String, BigUint>,
        ///whether the solution was filtered out according to the rules of [CIP-67](https://forum.cow.fi/t/cip-67-moving-from-batch-auction-to-the-fair-combinatorial-auction/2967).
        #[serde(
            rename = "filteredOut",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub filtered_out: ::std::option::Option<bool>,
        ///whether the solution is a winner (received the right to get executed) or not
        #[serde(
            rename = "isWinner",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub is_winner: ::std::option::Option<bool>,
        ///Touched orders.
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub orders: ::std::vec::Vec<SolverSettlementOrdersItem>,
        ///Which position the solution achieved in the total ranking of the competition.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub ranking: ::std::option::Option<f64>,
        /**The reference score as defined in [CIP-67](https://forum.cow.fi/t/cip-67-moving-from-batch-auction-to-the-fair-combinatorial-auction/2967) (if available).
         */
        #[serde(
            rename = "referenceScore",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub reference_score: ::std::option::Option<BigUint>,
        /**The score of the current auction as defined in [CIP-20](https://snapshot.org/#/cow.eth/proposal/0x2d3f9bd1ea72dca84b03e97dda3efc1f4a42a772c54bd2037e8b62e7d09a491f).
         */
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub score: ::std::option::Option<BigUint>,
        /**The address used by the solver to execute the settlement on-chain.

        This field is missing for old settlements, the zero address has been
        used instead.*/
        #[serde(
            rename = "solverAddress",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub solver_address: ::std::option::Option<::std::string::String>,
        /**Transaction in which the solution was executed onchain (if available).
         */
        #[serde(
            rename = "txHash",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub tx_hash: ::std::option::Option<TransactionHash>,
    }
    impl ::std::default::Default for SolverSettlement {
        fn default() -> Self {
            Self {
                clearing_prices: Default::default(),
                filtered_out: Default::default(),
                is_winner: Default::default(),
                orders: Default::default(),
                ranking: Default::default(),
                reference_score: Default::default(),
                score: Default::default(),
                solver_address: Default::default(),
                tx_hash: Default::default(),
            }
        }
    }
    ///`SolverSettlementOrdersItem`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "buyAmount": {
    ///      "$ref": "#/components/schemas/BigUint"
    ///    },
    ///    "id": {
    ///      "$ref": "#/components/schemas/UID"
    ///    },
    ///    "sellAmount": {
    ///      "$ref": "#/components/schemas/BigUint"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct SolverSettlementOrdersItem {
        #[serde(
            rename = "buyAmount",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub buy_amount: ::std::option::Option<BigUint>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub id: ::std::option::Option<Uid>,
        #[serde(
            rename = "sellAmount",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub sell_amount: ::std::option::Option<BigUint>,
    }
    impl ::std::default::Default for SolverSettlementOrdersItem {
        fn default() -> Self {
            Self {
                buy_amount: Default::default(),
                id: Default::default(),
                sell_amount: Default::default(),
            }
        }
    }
    ///State overrides for a given account before simulation.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "State overrides for a given account before simulation.",
    ///  "type": "object",
    ///  "properties": {
    ///    "balance": {
    ///      "description": "Fake balance to set for the account (decimal-encoded uint256).",
    ///      "examples": [
    ///        "1000000000000000000"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "code": {
    ///      "description": "Fake EVM bytecode to inject into the account (hex with `0x` prefix).",
    ///      "examples": [
    ///        "0x6080604052"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "storage": {
    ///      "description": "Fake key-value mapping to override individual storage slots. Keys and values are 32-byte hex strings with `0x` prefix.\n",
    ///      "type": "object",
    ///      "additionalProperties": {
    ///        "type": "string"
    ///      }
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct StateObject {
        ///Fake balance to set for the account (decimal-encoded uint256).
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub balance: ::std::option::Option<::std::string::String>,
        ///Fake EVM bytecode to inject into the account (hex with `0x` prefix).
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub code: ::std::option::Option<::std::string::String>,
        /**Fake key-value mapping to override individual storage slots. Keys and values are 32-byte hex strings with `0x` prefix.
         */
        #[serde(
            default,
            skip_serializing_if = ":: std :: collections :: HashMap::is_empty"
        )]
        pub storage: ::std::collections::HashMap<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for StateObject {
        fn default() -> Self {
            Self {
                balance: Default::default(),
                code: Default::default(),
                storage: Default::default(),
            }
        }
    }
    /**Quote data stored with an order. This represents the original quote used to
    create the order, containing gas estimation and pricing information captured
    at the time of quoting.

    Note: This is different from `OrderQuoteResponse` which is returned by the
    `POST /api/v1/quote` endpoint and contains order parameters to sign.
    */
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Quote data stored with an order. This represents the original quote used to\ncreate the order, containing gas estimation and pricing information captured\nat the time of quoting.\n\nNote: This is different from `OrderQuoteResponse` which is returned by the\n`POST /api/v1/quote` endpoint and contains order parameters to sign.\n",
    ///  "type": "object",
    ///  "required": [
    ///    "buyAmount",
    ///    "feeAmount",
    ///    "gasAmount",
    ///    "gasPrice",
    ///    "sellAmount",
    ///    "sellTokenPrice",
    ///    "solver",
    ///    "verified"
    ///  ],
    ///  "properties": {
    ///    "buyAmount": {
    ///      "description": "The quoted buy amount in atoms of the buy token.",
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/TokenAmount"
    ///        }
    ///      ]
    ///    },
    ///    "feeAmount": {
    ///      "description": "The fee amount in atoms of the sell token, calculated from the gas parameters\nat the time of quoting.\n\nComputed as: `ceil((gasAmount * gasPrice) / sellTokenPrice)`.\n\nThis represents the network fee that was estimated when the quote was created.\n",
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/TokenAmount"
    ///        }
    ///      ]
    ///    },
    ///    "gasAmount": {
    ///      "description": "The estimated gas units required to execute the quoted trade.\nMeasured in gas units (not Wei). Used together with `gasPrice` and\n`sellTokenPrice` to calculate the network fee in sell token atoms.\n",
    ///      "examples": [
    ///        "150000"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "gasPrice": {
    ///      "description": "The estimated gas price at the time of quoting, measured in Wei per gas unit.\nThe network fee in Wei can be calculated as: `feeInWei = gasAmount * gasPrice`.\n",
    ///      "examples": [
    ///        "15000000000"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "metadata": {
    ///      "description": "Additional metadata about the quote execution plan (e.g., the route taken).\nThis field is only populated for orders that are no longer fillable\n(filled, cancelled, or expired) to prevent solvers from copying\nexecution strategies for active orders.\n",
    ///      "type": "object"
    ///    },
    ///    "sellAmount": {
    ///      "description": "The quoted sell amount in atoms of the sell token.",
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/TokenAmount"
    ///        }
    ///      ]
    ///    },
    ///    "sellTokenPrice": {
    ///      "description": "The price of the sell token expressed in native token atoms per sell token atom.\n\nUnits: `native token atoms / sell token atoms`\n\n**Example calculation (Mainnet, selling USDC):**\n- Sell token: USDC (6 decimals)\n- Native token: ETH (18 decimals)\n- Market price: 1 ETH = 1000 USDC\n\n`sellTokenPrice = 1 × 10^18 wei / (1000 × 10^6 USDC atoms) = 10^9`\n\nThis value is used to convert network fees (in native token) to sell token amounts.\n",
    ///      "examples": [
    ///        "1000000000"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "solver": {
    ///      "description": "The address of the solver that provided this quote.",
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/Address"
    ///        }
    ///      ]
    ///    },
    ///    "verified": {
    ///      "description": "Whether the quote was verified through simulation. A verified quote\nprovides higher confidence that the trade will execute successfully.\n",
    ///      "type": "boolean"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct StoredOrderQuote {
        ///The quoted buy amount in atoms of the buy token.
        #[serde(rename = "buyAmount")]
        pub buy_amount: TokenAmount,
        /**The fee amount in atoms of the sell token, calculated from the gas parameters
        at the time of quoting.

        Computed as: `ceil((gasAmount * gasPrice) / sellTokenPrice)`.

        This represents the network fee that was estimated when the quote was created.
        */
        #[serde(rename = "feeAmount")]
        pub fee_amount: TokenAmount,
        /**The estimated gas units required to execute the quoted trade.
        Measured in gas units (not Wei). Used together with `gasPrice` and
        `sellTokenPrice` to calculate the network fee in sell token atoms.
        */
        #[serde(rename = "gasAmount")]
        pub gas_amount: ::std::string::String,
        /**The estimated gas price at the time of quoting, measured in Wei per gas unit.
        The network fee in Wei can be calculated as: `feeInWei = gasAmount * gasPrice`.
        */
        #[serde(rename = "gasPrice")]
        pub gas_price: ::std::string::String,
        /**Additional metadata about the quote execution plan (e.g., the route taken).
        This field is only populated for orders that are no longer fillable
        (filled, cancelled, or expired) to prevent solvers from copying
        execution strategies for active orders.
        */
        #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
        pub metadata: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        ///The quoted sell amount in atoms of the sell token.
        #[serde(rename = "sellAmount")]
        pub sell_amount: TokenAmount,
        /**The price of the sell token expressed in native token atoms per sell token atom.

        Units: `native token atoms / sell token atoms`

        **Example calculation (Mainnet, selling USDC):**
        - Sell token: USDC (6 decimals)
        - Native token: ETH (18 decimals)
        - Market price: 1 ETH = 1000 USDC

        `sellTokenPrice = 1 × 10^18 wei / (1000 × 10^6 USDC atoms) = 10^9`

        This value is used to convert network fees (in native token) to sell token amounts.
        */
        #[serde(rename = "sellTokenPrice")]
        pub sell_token_price: ::std::string::String,
        ///The address of the solver that provided this quote.
        pub solver: Address,
        /**Whether the quote was verified through simulation. A verified quote
        provides higher confidence that the trade will execute successfully.
        */
        pub verified: bool,
    }
    ///The protocol fee is taken as a percent of the surplus.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "The protocol fee is taken as a percent of the surplus.",
    ///  "type": "object",
    ///  "required": [
    ///    "factor",
    ///    "maxVolumeFactor"
    ///  ],
    ///  "properties": {
    ///    "factor": {
    ///      "type": "number",
    ///      "exclusiveMaximum": 1.0,
    ///      "minimum": 0.0
    ///    },
    ///    "maxVolumeFactor": {
    ///      "type": "number",
    ///      "exclusiveMaximum": 1.0,
    ///      "minimum": 0.0
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct Surplus {
        pub factor: f64,
        #[serde(rename = "maxVolumeFactor")]
        pub max_volume_factor: f64,
    }
    ///A Tenderly transaction simulation request.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "A Tenderly transaction simulation request.",
    ///  "type": "object",
    ///  "required": [
    ///    "from",
    ///    "input",
    ///    "network_id",
    ///    "to"
    ///  ],
    ///  "properties": {
    ///    "access_list": {
    ///      "description": "EIP-2930 access list for the transaction.",
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/AccessListItem"
    ///      }
    ///    },
    ///    "block_number": {
    ///      "description": "Block number to simulate the transaction at.",
    ///      "type": "integer"
    ///    },
    ///    "from": {
    ///      "$ref": "#/components/schemas/Address"
    ///    },
    ///    "gas": {
    ///      "description": "Gas limit for the transaction.",
    ///      "type": "integer"
    ///    },
    ///    "gas_price": {
    ///      "description": "Gas price in Wei.",
    ///      "type": "integer"
    ///    },
    ///    "generate_access_list": {
    ///      "description": "Whether to generate an access list for the transaction.",
    ///      "type": "boolean"
    ///    },
    ///    "input": {
    ///      "description": "Transaction calldata encoded as hex with `0x` prefix.",
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/CallData"
    ///        }
    ///      ]
    ///    },
    ///    "network_id": {
    ///      "description": "The network identifier (e.g. \"1\" for mainnet).",
    ///      "examples": [
    ///        "1"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "save": {
    ///      "description": "Whether to save the simulation on Tenderly.",
    ///      "type": "boolean"
    ///    },
    ///    "save_if_fails": {
    ///      "description": "Whether to save the simulation only if it fails.",
    ///      "type": "boolean"
    ///    },
    ///    "simulation_type": {
    ///      "$ref": "#/components/schemas/SimulationType"
    ///    },
    ///    "state_objects": {
    ///      "description": "State overrides applied before simulation. Keys are account addresses (hex with `0x` prefix).\n",
    ///      "type": "object",
    ///      "additionalProperties": {
    ///        "$ref": "#/components/schemas/StateObject"
    ///      }
    ///    },
    ///    "to": {
    ///      "$ref": "#/components/schemas/Address"
    ///    },
    ///    "transaction_index": {
    ///      "description": "Transaction index within the block.",
    ///      "type": "integer"
    ///    },
    ///    "value": {
    ///      "description": "ETH value to send with the transaction (decimal-encoded uint256).",
    ///      "examples": [
    ///        "0"
    ///      ],
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct TenderlyRequest {
        ///EIP-2930 access list for the transaction.
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub access_list: ::std::vec::Vec<AccessListItem>,
        ///Block number to simulate the transaction at.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub block_number: ::std::option::Option<i64>,
        pub from: Address,
        ///Gas limit for the transaction.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub gas: ::std::option::Option<i64>,
        ///Gas price in Wei.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub gas_price: ::std::option::Option<i64>,
        ///Whether to generate an access list for the transaction.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub generate_access_list: ::std::option::Option<bool>,
        ///Transaction calldata encoded as hex with `0x` prefix.
        pub input: CallData,
        ///The network identifier (e.g. "1" for mainnet).
        pub network_id: ::std::string::String,
        ///Whether to save the simulation on Tenderly.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub save: ::std::option::Option<bool>,
        ///Whether to save the simulation only if it fails.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub save_if_fails: ::std::option::Option<bool>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub simulation_type: ::std::option::Option<SimulationType>,
        /**State overrides applied before simulation. Keys are account addresses (hex with `0x` prefix).
         */
        #[serde(
            default,
            skip_serializing_if = ":: std :: collections :: HashMap::is_empty"
        )]
        pub state_objects: ::std::collections::HashMap<::std::string::String, StateObject>,
        pub to: Address,
        ///Transaction index within the block.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub transaction_index: ::std::option::Option<i64>,
        ///ETH value to send with the transaction (decimal-encoded uint256).
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub value: ::std::option::Option<::std::string::String>,
    }
    ///Amount of a token. `uint256` encoded in decimal.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Amount of a token. `uint256` encoded in decimal.",
    ///  "examples": [
    ///    "1234567890"
    ///  ],
    ///  "type": "string"
    ///}
    /// ```
    /// </details>
    #[derive(
        ::serde::Deserialize, ::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd,
    )]
    #[serde(transparent)]
    pub struct TokenAmount(pub ::std::string::String);
    impl ::std::ops::Deref for TokenAmount {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }
    impl ::std::convert::From<TokenAmount> for ::std::string::String {
        fn from(value: TokenAmount) -> Self {
            value.0
        }
    }
    impl ::std::convert::From<::std::string::String> for TokenAmount {
        fn from(value: ::std::string::String) -> Self {
            Self(value)
        }
    }
    impl ::std::str::FromStr for TokenAmount {
        type Err = ::std::convert::Infallible;
        fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
            Ok(Self(value.to_string()))
        }
    }
    impl ::std::fmt::Display for TokenAmount {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            self.0.fmt(f)
        }
    }
    /**The total surplus.
     */
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "The total surplus.\n",
    ///  "type": "object",
    ///  "properties": {
    ///    "totalSurplus": {
    ///      "description": "The total surplus.",
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct TotalSurplus {
        ///The total surplus.
        #[serde(
            rename = "totalSurplus",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub total_surplus: ::std::option::Option<::std::string::String>,
    }
    impl ::std::default::Default for TotalSurplus {
        fn default() -> Self {
            Self {
                total_surplus: Default::default(),
            }
        }
    }
    /**Trade data such as executed amounts, fees, `orderUid` and `block` number.
     */
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Trade data such as executed amounts, fees, `orderUid` and `block` number.\n",
    ///  "type": "object",
    ///  "required": [
    ///    "blockNumber",
    ///    "buyAmount",
    ///    "buyToken",
    ///    "logIndex",
    ///    "orderUid",
    ///    "owner",
    ///    "sellAmount",
    ///    "sellAmountBeforeFees",
    ///    "sellToken",
    ///    "txHash"
    ///  ],
    ///  "properties": {
    ///    "blockNumber": {
    ///      "description": "Block in which trade occurred.",
    ///      "type": "integer"
    ///    },
    ///    "buyAmount": {
    ///      "description": "Total amount of `buyToken` received in this trade.",
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/TokenAmount"
    ///        }
    ///      ]
    ///    },
    ///    "buyToken": {
    ///      "description": "Address of token bought.",
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/Address"
    ///        }
    ///      ]
    ///    },
    ///    "executedProtocolFees": {
    ///      "description": "Executed protocol fees for this trade, together with the fee policies used. Listed in the order they got applied.\n",
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/ExecutedProtocolFee"
    ///      }
    ///    },
    ///    "logIndex": {
    ///      "description": "Index in which transaction was included in block.",
    ///      "type": "integer"
    ///    },
    ///    "orderUid": {
    ///      "description": "UID of the order matched by this trade.",
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/UID"
    ///        }
    ///      ]
    ///    },
    ///    "owner": {
    ///      "description": "Address of trader.",
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/Address"
    ///        }
    ///      ]
    ///    },
    ///    "sellAmount": {
    ///      "description": "Total amount of `sellToken` that has been executed for this trade (including fees).",
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/TokenAmount"
    ///        }
    ///      ]
    ///    },
    ///    "sellAmountBeforeFees": {
    ///      "description": "The total amount of `sellToken` that has been executed for this order without fees.",
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/BigUint"
    ///        }
    ///      ]
    ///    },
    ///    "sellToken": {
    ///      "description": "Address of token sold.",
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/Address"
    ///        }
    ///      ]
    ///    },
    ///    "txHash": {
    ///      "description": "Transaction hash of the corresponding settlement transaction containing the trade (if available).",
    ///      "oneOf": [
    ///        {
    ///          "type": "null"
    ///        },
    ///        {
    ///          "allOf": [
    ///            {
    ///              "$ref": "#/components/schemas/TransactionHash"
    ///            }
    ///          ]
    ///        }
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct Trade {
        ///Block in which trade occurred.
        #[serde(rename = "blockNumber")]
        pub block_number: i64,
        ///Total amount of `buyToken` received in this trade.
        #[serde(rename = "buyAmount")]
        pub buy_amount: TokenAmount,
        ///Address of token bought.
        #[serde(rename = "buyToken")]
        pub buy_token: Address,
        /**Executed protocol fees for this trade, together with the fee policies used. Listed in the order they got applied.
         */
        #[serde(
            rename = "executedProtocolFees",
            default,
            skip_serializing_if = "::std::vec::Vec::is_empty"
        )]
        pub executed_protocol_fees: ::std::vec::Vec<ExecutedProtocolFee>,
        ///Index in which transaction was included in block.
        #[serde(rename = "logIndex")]
        pub log_index: i64,
        ///UID of the order matched by this trade.
        #[serde(rename = "orderUid")]
        pub order_uid: Uid,
        ///Address of trader.
        pub owner: Address,
        ///Total amount of `sellToken` that has been executed for this trade (including fees).
        #[serde(rename = "sellAmount")]
        pub sell_amount: TokenAmount,
        ///The total amount of `sellToken` that has been executed for this order without fees.
        #[serde(rename = "sellAmountBeforeFees")]
        pub sell_amount_before_fees: BigUint,
        ///Address of token sold.
        #[serde(rename = "sellToken")]
        pub sell_token: Address,
        ///Transaction hash of the corresponding settlement transaction containing the trade (if available).
        #[serde(rename = "txHash")]
        pub tx_hash: ::std::option::Option<TransactionHash>,
    }
    ///32 byte digest encoded as a hex with `0x` prefix.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "32 byte digest encoded as a hex with `0x` prefix.",
    ///  "examples": [
    ///    "0xd51f28edffcaaa76be4a22f6375ad289272c037f3cc072345676e88d92ced8b5"
    ///  ],
    ///  "type": "string"
    ///}
    /// ```
    /// </details>
    #[derive(
        ::serde::Deserialize, ::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd,
    )]
    #[serde(transparent)]
    pub struct TransactionHash(pub ::std::string::String);
    impl ::std::ops::Deref for TransactionHash {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }
    impl ::std::convert::From<TransactionHash> for ::std::string::String {
        fn from(value: TransactionHash) -> Self {
            value.0
        }
    }
    impl ::std::convert::From<::std::string::String> for TransactionHash {
        fn from(value: ::std::string::String) -> Self {
            Self(value)
        }
    }
    impl ::std::str::FromStr for TransactionHash {
        type Err = ::std::convert::Infallible;
        fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
            Ok(Self(value.to_string()))
        }
    }
    impl ::std::fmt::Display for TransactionHash {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            self.0.fmt(f)
        }
    }
    /**Unique identifier for the order: 56 bytes encoded as hex with `0x`
    prefix.

    Bytes 0..32 are the order digest, bytes 30..52 the owner address and
    bytes 52..56 the expiry (`validTo`) as a `uint32` unix epoch timestamp.*/
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Unique identifier for the order: 56 bytes encoded as hex with `0x`\nprefix.\n\nBytes 0..32 are the order digest, bytes 30..52 the owner address and\nbytes 52..56 the expiry (`validTo`) as a `uint32` unix epoch timestamp.",
    ///  "examples": [
    ///    "0xff2e2e54d178997f173266817c1e9ed6fee1a1aae4b43971c53b543cffcc2969845c6f5599fbb25dbdd1b9b013daf85c03f3c63763e4bc4a"
    ///  ],
    ///  "type": "string"
    ///}
    /// ```
    /// </details>
    #[derive(
        ::serde::Deserialize, ::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd,
    )]
    #[serde(transparent)]
    pub struct Uid(pub ::std::string::String);
    impl ::std::ops::Deref for Uid {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }
    impl ::std::convert::From<Uid> for ::std::string::String {
        fn from(value: Uid) -> Self {
            value.0
        }
    }
    impl ::std::convert::From<::std::string::String> for Uid {
        fn from(value: ::std::string::String) -> Self {
            Self(value)
        }
    }
    impl ::std::str::FromStr for Uid {
        type Err = ::std::convert::Infallible;
        fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
            Ok(Self(value.to_string()))
        }
    }
    impl ::std::fmt::Display for Uid {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            self.0.fmt(f)
        }
    }
    ///The protocol fee is taken as a percent of the order volume.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "The protocol fee is taken as a percent of the order volume.",
    ///  "type": "object",
    ///  "required": [
    ///    "factor"
    ///  ],
    ///  "properties": {
    ///    "factor": {
    ///      "type": "number",
    ///      "exclusiveMaximum": 1.0,
    ///      "minimum": 0.0
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct Volume {
        pub factor: f64,
    }
    /// Generation of default values for serde.
    pub mod defaults {
        pub(super) fn order_buy_token_balance() -> super::BuyTokenDestination {
            super::BuyTokenDestination::Erc20
        }
        pub(super) fn order_sell_token_balance() -> super::SellTokenSource {
            super::SellTokenSource::Erc20
        }
        pub(super) fn order_creation_buy_token_balance() -> super::BuyTokenDestination {
            super::BuyTokenDestination::Erc20
        }
        pub(super) fn order_creation_sell_token_balance() -> super::SellTokenSource {
            super::SellTokenSource::Erc20
        }
        pub(super) fn order_parameters_buy_token_balance() -> super::BuyTokenDestination {
            super::BuyTokenDestination::Erc20
        }
        pub(super) fn order_parameters_sell_token_balance() -> super::SellTokenSource {
            super::SellTokenSource::Erc20
        }
        pub(super) fn order_parameters_signing_scheme() -> super::SigningScheme {
            super::SigningScheme::Eip712
        }
        pub(super) fn order_quote_request_variant0_variant0_buy_token_balance()
        -> super::BuyTokenDestination {
            super::BuyTokenDestination::Erc20
        }
        pub(super) fn order_quote_request_variant0_variant0_onchain_order() -> ::serde_json::Value {
            ::serde_json::from_str::<::serde_json::Value>("false").unwrap()
        }
        pub(super) fn order_quote_request_variant0_variant0_price_quality() -> super::PriceQuality {
            super::PriceQuality::Verified
        }
        pub(super) fn order_quote_request_variant0_variant0_sell_token_balance()
        -> super::SellTokenSource {
            super::SellTokenSource::Erc20
        }
        pub(super) fn order_quote_request_variant0_variant0_signing_scheme() -> super::SigningScheme
        {
            super::SigningScheme::Eip712
        }
        pub(super) fn order_quote_request_variant0_variant1_buy_token_balance()
        -> super::BuyTokenDestination {
            super::BuyTokenDestination::Erc20
        }
        pub(super) fn order_quote_request_variant0_variant1_onchain_order() -> ::serde_json::Value {
            ::serde_json::from_str::<::serde_json::Value>("false").unwrap()
        }
        pub(super) fn order_quote_request_variant0_variant1_price_quality() -> super::PriceQuality {
            super::PriceQuality::Verified
        }
        pub(super) fn order_quote_request_variant0_variant1_sell_token_balance()
        -> super::SellTokenSource {
            super::SellTokenSource::Erc20
        }
        pub(super) fn order_quote_request_variant0_variant1_signing_scheme() -> super::SigningScheme
        {
            super::SigningScheme::Eip712
        }
        pub(super) fn order_quote_request_variant1_variant0_buy_token_balance()
        -> super::BuyTokenDestination {
            super::BuyTokenDestination::Erc20
        }
        pub(super) fn order_quote_request_variant1_variant0_onchain_order() -> ::serde_json::Value {
            ::serde_json::from_str::<::serde_json::Value>("false").unwrap()
        }
        pub(super) fn order_quote_request_variant1_variant0_price_quality() -> super::PriceQuality {
            super::PriceQuality::Verified
        }
        pub(super) fn order_quote_request_variant1_variant0_sell_token_balance()
        -> super::SellTokenSource {
            super::SellTokenSource::Erc20
        }
        pub(super) fn order_quote_request_variant1_variant0_signing_scheme() -> super::SigningScheme
        {
            super::SigningScheme::Eip712
        }
        pub(super) fn order_quote_request_variant1_variant1_buy_token_balance()
        -> super::BuyTokenDestination {
            super::BuyTokenDestination::Erc20
        }
        pub(super) fn order_quote_request_variant1_variant1_onchain_order() -> ::serde_json::Value {
            ::serde_json::from_str::<::serde_json::Value>("false").unwrap()
        }
        pub(super) fn order_quote_request_variant1_variant1_price_quality() -> super::PriceQuality {
            super::PriceQuality::Verified
        }
        pub(super) fn order_quote_request_variant1_variant1_sell_token_balance()
        -> super::SellTokenSource {
            super::SellTokenSource::Erc20
        }
        pub(super) fn order_quote_request_variant1_variant1_signing_scheme() -> super::SigningScheme
        {
            super::SigningScheme::Eip712
        }
        pub(super) fn order_quote_request_variant2_variant0_buy_token_balance()
        -> super::BuyTokenDestination {
            super::BuyTokenDestination::Erc20
        }
        pub(super) fn order_quote_request_variant2_variant0_onchain_order() -> ::serde_json::Value {
            ::serde_json::from_str::<::serde_json::Value>("false").unwrap()
        }
        pub(super) fn order_quote_request_variant2_variant0_price_quality() -> super::PriceQuality {
            super::PriceQuality::Verified
        }
        pub(super) fn order_quote_request_variant2_variant0_sell_token_balance()
        -> super::SellTokenSource {
            super::SellTokenSource::Erc20
        }
        pub(super) fn order_quote_request_variant2_variant0_signing_scheme() -> super::SigningScheme
        {
            super::SigningScheme::Eip712
        }
        pub(super) fn order_quote_request_variant2_variant1_buy_token_balance()
        -> super::BuyTokenDestination {
            super::BuyTokenDestination::Erc20
        }
        pub(super) fn order_quote_request_variant2_variant1_onchain_order() -> ::serde_json::Value {
            ::serde_json::from_str::<::serde_json::Value>("false").unwrap()
        }
        pub(super) fn order_quote_request_variant2_variant1_price_quality() -> super::PriceQuality {
            super::PriceQuality::Verified
        }
        pub(super) fn order_quote_request_variant2_variant1_sell_token_balance()
        -> super::SellTokenSource {
            super::SellTokenSource::Erc20
        }
        pub(super) fn order_quote_request_variant2_variant1_signing_scheme() -> super::SigningScheme
        {
            super::SigningScheme::Eip712
        }
        pub(super) fn simulation_request_buy_token_balance() -> super::BuyTokenDestination {
            super::BuyTokenDestination::Erc20
        }
        pub(super) fn simulation_request_sell_token_balance() -> super::SellTokenSource {
            super::SellTokenSource::Erc20
        }
    }
}
#[derive(Clone, Debug)]
/**Client for Order Book API

Version: 0.0.1*/
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
        "0.0.1"
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
    /**Create a new order. In order to replace an existing order with a new one, the appData must contain a [valid replacement order UID](https://github.com/cowprotocol/app-data/blob/main/src/schemas/v1.1.0.json#L62), then the indicated order is cancelled, and a new one placed.
    This allows an old order to be cancelled AND a new order to be created in an atomic operation with a single signature.
    This may be useful for replacing orders when on-chain prices move outside of the original order's limit price

    Sends a `POST` request to `/api/v1/orders`

    Arguments:
    - `body`: The order to create.
    */
    pub async fn create_order<'a>(
        &'a self,
        body: &'a types::OrderCreation,
    ) -> Result<ResponseValue<types::Uid>, Error<()>> {
        let url = format!("{}/api/v1/orders", self.baseurl,);
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
            operation_id: "create_order",
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
    /**Cancel multiple orders by marking them invalid with a timestamp

    This is a *best effort* cancellation, and might not prevent solvers from settling the orders (if the order is part of an in-flight settlement transaction for example). Authentication must be provided by an [EIP-712](https://eips.ethereum.org/EIPS/eip-712) signature of an `OrderCancellations(bytes[] orderUids)` message.


    Sends a `DELETE` request to `/api/v1/orders`

    Arguments:
    - `body`: Signed `OrderCancellations`.
    */
    pub async fn cancel_orders<'a>(
        &'a self,
        body: &'a types::OrderCancellations,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!("{}/api/v1/orders", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .delete(url)
            .json(&body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "cancel_orders",
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
    /**Get existing orders from the list of UIDs

    Returns an array where each element is an object with either
    an "order" key containing the full order, or an "error" key
    containing the UID and a description of what went wrong.


    Sends a `POST` request to `/api/v1/orders/by_uids`

    Arguments:
    - `body`: The list of up to 128 order uids to fetch
    */
    pub async fn get_orders<'a>(
        &'a self,
        body: &'a ::std::vec::Vec<types::Uid>,
    ) -> Result<ResponseValue<types::GetOrdersResponse>, Error<()>> {
        let url = format!("{}/api/v1/orders/by_uids", self.baseurl,);
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
    /**Get existing order from UID

    Sends a `GET` request to `/api/v1/orders/{UID}`

    */
    pub async fn get_order<'a>(
        &'a self,
        uid: &'a types::Uid,
    ) -> Result<ResponseValue<types::Order>, Error<()>> {
        let url = format!(
            "{}/api/v1/orders/{}",
            self.baseurl,
            encode_path(&uid.to_string()),
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
            operation_id: "get_order",
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
    /**Cancel an order by marking it invalid with a timestamp

    The successful deletion might not prevent solvers from settling the
    order.

    Authentication must be provided by providing an
    [EIP-712](https://eips.ethereum.org/EIPS/eip-712) signature of an
    `OrderCancellation(bytes orderUid)` message.

    Sends a `DELETE` request to `/api/v1/orders/{UID}`

    Arguments:
    - `uid`
    - `body`: Signed `OrderCancellation`
    */
    pub async fn cancel_order<'a>(
        &'a self,
        uid: &'a types::Uid,
        body: &'a types::OrderCancellation,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/api/v1/orders/{}",
            self.baseurl,
            encode_path(&uid.to_string()),
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
            .json(&body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "cancel_order",
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
    /**Get the status of an order

    Sends a `GET` request to `/api/v1/orders/{UID}/status`

    */
    pub async fn get_order_status<'a>(
        &'a self,
        uid: &'a types::Uid,
    ) -> Result<ResponseValue<types::CompetitionOrderStatus>, Error<()>> {
        let url = format!(
            "{}/api/v1/orders/{}/status",
            self.baseurl,
            encode_path(&uid.to_string()),
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
            operation_id: "get_order_status",
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
    /**Get orders by settlement transaction hash

    Sends a `GET` request to `/api/v1/transactions/{txHash}/orders`

    */
    pub async fn get_orders_by_tx_hash<'a>(
        &'a self,
        tx_hash: &'a types::TransactionHash,
    ) -> Result<ResponseValue<::std::vec::Vec<types::Order>>, Error<()>> {
        let url = format!(
            "{}/api/v1/transactions/{}/orders",
            self.baseurl,
            encode_path(&tx_hash.to_string()),
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
            operation_id: "get_orders_by_tx_hash",
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
    /**Get existing trades (unpaginated)

    **Deprecated:** This endpoint is deprecated and will be removed in the future. Please use `/api/v2/trades` instead, which provides pagination support.

    Exactly one of `owner` or `orderUid` must be set.

    Results are sorted by block number and log index descending (newest trades first).

    **Note:** This endpoint returns all matching trades without pagination. For paginated results, use `/api/v2/trades`.


    Sends a `GET` request to `/api/v1/trades`

    */
    pub async fn get_trades<'a>(
        &'a self,
        order_uid: Option<&'a types::Uid>,
        owner: Option<&'a types::Address>,
    ) -> Result<ResponseValue<::std::vec::Vec<types::Trade>>, Error<()>> {
        let url = format!("{}/api/v1/trades", self.baseurl,);
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
            .query(&progenitor_client::QueryParam::new("orderUid", &order_uid))
            .query(&progenitor_client::QueryParam::new("owner", &owner))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "get_trades",
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
    /**Get existing trades (paginated)

    Exactly one of `owner` or `orderUid` must be set.

    Results are paginated and sorted by block number and log index descending (newest trades first).

    To enumerate all trades start with `offset` 0 and keep increasing the
    `offset` by the total number of returned results. When a response
    contains less than `limit` the last page has been reached.


    Sends a `GET` request to `/api/v2/trades`

    Arguments:
    - `limit`: The maximum number of trades to return. Defaults to 10. Must be between 1 and 1000.

    - `offset`: The pagination offset. Defaults to 0.

    - `order_uid`
    - `owner`
    */
    pub async fn get_trades_v2<'a>(
        &'a self,
        limit: Option<::std::num::NonZeroU64>,
        offset: Option<i64>,
        order_uid: Option<&'a types::Uid>,
        owner: Option<&'a types::Address>,
    ) -> Result<ResponseValue<::std::vec::Vec<types::Trade>>, Error<()>> {
        let url = format!("{}/api/v2/trades", self.baseurl,);
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
            .query(&progenitor_client::QueryParam::new("offset", &offset))
            .query(&progenitor_client::QueryParam::new("orderUid", &order_uid))
            .query(&progenitor_client::QueryParam::new("owner", &owner))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "get_trades_v2",
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
    /**Get the current batch auction

    The current batch auction that solvers should be solving right now. This
    includes:

    * A list of solvable orders. * The block on which the batch was created.
    * Prices for all tokens being traded (used for objective value
    computation).

    **Note: This endpoint is currently permissioned. Reach out in discord if
    you need access.**

    Sends a `GET` request to `/api/v1/auction`

    */
    pub async fn get_current_batch_auction<'a>(
        &'a self,
    ) -> Result<ResponseValue<types::Auction>, Error<()>> {
        let url = format!("{}/api/v1/auction", self.baseurl,);
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
            operation_id: "get_current_batch_auction",
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
    /**Get orders of one user paginated

    The orders are sorted by their creation date descending (newest orders
    first).

    To enumerate all orders start with `offset` 0 and keep increasing the
    `offset` by the total number of returned results. When a response
    contains less than `limit` the last page has been reached.

    Sends a `GET` request to `/api/v1/account/{owner}/orders`

    Arguments:
    - `owner`
    - `limit`: The pagination limit. Defaults to 10. Maximum 1000. Minimum 1.

    - `offset`: The pagination offset. Defaults to 0.

    */
    pub async fn get_user_orders_paginated<'a>(
        &'a self,
        owner: &'a types::Address,
        limit: Option<i64>,
        offset: Option<i64>,
    ) -> Result<ResponseValue<::std::vec::Vec<types::Order>>, Error<()>> {
        let url = format!(
            "{}/api/v1/account/{}/orders",
            self.baseurl,
            encode_path(&owner.to_string()),
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
            .query(&progenitor_client::QueryParam::new("offset", &offset))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "get_user_orders_paginated",
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
    /**Get native price for the given token

    Price is the exchange rate between the specified token and the network's
    native currency.

    It represents the amount of native token atoms needed to buy 1 atom of
    the specified token.

    Sends a `GET` request to `/api/v1/token/{token}/native_price`

    */
    pub async fn get_token_native_price<'a>(
        &'a self,
        token: &'a types::Address,
    ) -> Result<ResponseValue<types::NativePriceResponse>, Error<()>> {
        let url = format!(
            "{}/api/v1/token/{}/native_price",
            self.baseurl,
            encode_path(&token.to_string()),
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
            operation_id: "get_token_native_price",
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
    /**Quote a price and fee for the specified order parameters

    Given a partial order compute the minimum fee and a price estimate for the order. Return a full order that can be used directly for signing, and with an included signature, passed directly to the order creation endpoint.


    Sends a `POST` request to `/api/v1/quote`

    Arguments:
    - `body`: The order parameters to compute a quote for.
    */
    pub async fn quote<'a>(
        &'a self,
        body: &'a types::OrderQuoteRequest,
    ) -> Result<ResponseValue<types::OrderQuoteResponse>, Error<()>> {
        let url = format!("{}/api/v1/quote", self.baseurl,);
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
            operation_id: "quote",
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
    /**Get information about a solver competition

    Returns the competition information by `auction_id`.


    Sends a `GET` request to `/api/v1/solver_competition/{auction_id}`

    */
    pub async fn get_solver_competition_by_auction_id<'a>(
        &'a self,
        auction_id: i64,
    ) -> Result<ResponseValue<types::SolverCompetitionResponse>, Error<()>> {
        let url = format!(
            "{}/api/v1/solver_competition/{}",
            self.baseurl,
            encode_path(&auction_id.to_string()),
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
            operation_id: "get_solver_competition_by_auction_id",
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
    /**Get information about solver competition

    Returns the competition information by `tx_hash`.


    Sends a `GET` request to `/api/v1/solver_competition/by_tx_hash/{tx_hash}`

    Arguments:
    - `tx_hash`: Transaction hash in which the competition was settled.
    */
    pub async fn get_solver_competition_by_tx_hash<'a>(
        &'a self,
        tx_hash: &'a types::TransactionHash,
    ) -> Result<ResponseValue<types::SolverCompetitionResponse>, Error<()>> {
        let url = format!(
            "{}/api/v1/solver_competition/by_tx_hash/{}",
            self.baseurl,
            encode_path(&tx_hash.to_string()),
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
            operation_id: "get_solver_competition_by_tx_hash",
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
    /**Get information about the most recent solver competition

    Returns the competition information for the last seen auction_id.


    Sends a `GET` request to `/api/v1/solver_competition/latest`

    */
    pub async fn get_solver_competition_latest<'a>(
        &'a self,
    ) -> Result<ResponseValue<types::SolverCompetitionResponse>, Error<()>> {
        let url = format!("{}/api/v1/solver_competition/latest", self.baseurl,);
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
            operation_id: "get_solver_competition_latest",
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
    /**Get information about a solver competition

    Returns the competition information by `auction_id`.


    Sends a `GET` request to `/api/v2/solver_competition/{auction_id}`

    */
    pub async fn get_solver_competition_by_auction_id_v2<'a>(
        &'a self,
        auction_id: i64,
    ) -> Result<ResponseValue<types::SolverCompetitionResponse>, Error<()>> {
        let url = format!(
            "{}/api/v2/solver_competition/{}",
            self.baseurl,
            encode_path(&auction_id.to_string()),
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
            operation_id: "get_solver_competition_by_auction_id_v2",
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
    /**Get information about solver competition

    Returns the competition information by `tx_hash`.


    Sends a `GET` request to `/api/v2/solver_competition/by_tx_hash/{tx_hash}`

    Arguments:
    - `tx_hash`: Transaction hash in which the competition was settled.
    */
    pub async fn get_solver_competition_by_tx_hash_v2<'a>(
        &'a self,
        tx_hash: &'a types::TransactionHash,
    ) -> Result<ResponseValue<types::SolverCompetitionResponse>, Error<()>> {
        let url = format!(
            "{}/api/v2/solver_competition/by_tx_hash/{}",
            self.baseurl,
            encode_path(&tx_hash.to_string()),
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
            operation_id: "get_solver_competition_by_tx_hash_v2",
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
    /**Get information about the most recent solver competition

    Returns the competition information for the last seen auction_id.


    Sends a `GET` request to `/api/v2/solver_competition/latest`

    */
    pub async fn get_solver_competition_latest_v2<'a>(
        &'a self,
    ) -> Result<ResponseValue<types::SolverCompetitionResponse>, Error<()>> {
        let url = format!("{}/api/v2/solver_competition/latest", self.baseurl,);
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
            operation_id: "get_solver_competition_latest_v2",
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
    /**Get the API's current deployed version

    Returns the git commit hash, branch name and release tag (code: https://github.com/cowprotocol/services).


    Sends a `GET` request to `/api/v1/version`

    */
    pub async fn get_api_version<'a>(&'a self) -> Result<ResponseValue<ByteStream>, Error<()>> {
        let url = format!("{}/api/v1/version", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self.client.get(url).headers(header_map).build()?;
        let info = OperationInfo {
            operation_id: "get_api_version",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::stream(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Get the full `appData` from contract `appDataHash`

    Sends a `GET` request to `/api/v1/app_data/{app_data_hash}`

    */
    pub async fn get_app_data_by_hash<'a>(
        &'a self,
        app_data_hash: &'a types::AppDataHash,
    ) -> Result<ResponseValue<types::AppDataObject>, Error<()>> {
        let url = format!(
            "{}/api/v1/app_data/{}",
            self.baseurl,
            encode_path(&app_data_hash.to_string()),
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
            operation_id: "get_app_data_by_hash",
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
    /**Registers a full `appData` so it can be referenced by `appDataHash`

    Uploads a full `appData` to orderbook so that orders created with the corresponding `appDataHash` can be linked to the original full `appData`.


    Sends a `PUT` request to `/api/v1/app_data/{app_data_hash}`

    Arguments:
    - `app_data_hash`
    - `body`: The `appData` document to upload.
    */
    pub async fn register_app_data_by_hash<'a>(
        &'a self,
        app_data_hash: &'a types::AppDataHash,
        body: &'a types::AppDataObject,
    ) -> Result<ResponseValue<types::AppDataHash>, Error<()>> {
        let url = format!(
            "{}/api/v1/app_data/{}",
            self.baseurl,
            encode_path(&app_data_hash.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .put(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "register_app_data_by_hash",
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
    /**Registers a full `appData` and returns `appDataHash`

    Uploads a full `appData` to orderbook and returns the corresponding `appDataHash`.


    Sends a `PUT` request to `/api/v1/app_data`

    Arguments:
    - `body`: The `appData` document to upload.
    */
    pub async fn register_app_data<'a>(
        &'a self,
        body: &'a types::AppDataObject,
    ) -> Result<ResponseValue<types::AppDataHash>, Error<()>> {
        let url = format!("{}/api/v1/app_data", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .put(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "register_app_data",
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
    /**Get the total surplus earned by the user. [UNSTABLE]

    ### Caution

    This endpoint is under active development and should NOT be considered
    stable.

    Sends a `GET` request to `/api/v1/users/{address}/total_surplus`

    */
    pub async fn get_address_total_surplus<'a>(
        &'a self,
        address: &'a types::Address,
    ) -> Result<ResponseValue<types::TotalSurplus>, Error<()>> {
        let url = format!(
            "{}/api/v1/users/{}/total_surplus",
            self.baseurl,
            encode_path(&address.to_string()),
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
            operation_id: "get_address_total_surplus",
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
    /**Simulate an arbitrary order

    Simulates an arbitrary order specified in the request body and returns the Tenderly simulation request, along with any simulation error if applicable.


    Sends a `POST` request to `/restricted/api/v1/debug/simulation`

    */
    pub async fn debug_simulation_post<'a>(
        &'a self,
        body: &'a types::SimulationRequest,
    ) -> Result<ResponseValue<types::OrderSimulation>, Error<()>> {
        let url = format!("{}/restricted/api/v1/debug/simulation", self.baseurl,);
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
            operation_id: "debug_simulation_post",
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
    /**Get Tenderly simulation request for an order

    Returns the Tenderly simulation request that would be used to simulate the given order, along with any simulation error if applicable.


    Sends a `GET` request to `/restricted/api/v1/debug/simulation/{uid}`

    Arguments:
    - `uid`
    - `block_number`: Block number to simulate the order at. If not specified, the simulation uses the latest block.

    */
    pub async fn debug_simulation<'a>(
        &'a self,
        uid: &'a types::Uid,
        block_number: Option<i64>,
    ) -> Result<ResponseValue<types::OrderSimulation>, Error<()>> {
        let url = format!(
            "{}/restricted/api/v1/debug/simulation/{}",
            self.baseurl,
            encode_path(&uid.to_string()),
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
                "block_number",
                &block_number,
            ))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "debug_simulation",
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
    /**Debug an order's lifecycle

    Returns a comprehensive debug report for the given order, including order details, lifecycle events, auction participation, proposed solutions, executions, trades, and settlement attempts.


    Sends a `GET` request to `/restricted/api/v1/debug/order/{uid}`

    */
    pub async fn debug_order<'a>(
        &'a self,
        uid: &'a types::Uid,
    ) -> Result<ResponseValue<types::DebugOrderResponse>, Error<()>> {
        let url = format!(
            "{}/restricted/api/v1/debug/order/{}",
            self.baseurl,
            encode_path(&uid.to_string()),
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
            operation_id: "debug_order",
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
