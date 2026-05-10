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
    ///`AmendOrderRequest`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "category",
    ///    "orderId",
    ///    "symbol"
    ///  ],
    ///  "properties": {
    ///    "category": {
    ///      "type": "string"
    ///    },
    ///    "orderId": {
    ///      "type": "string"
    ///    },
    ///    "price": {
    ///      "type": "string"
    ///    },
    ///    "qty": {
    ///      "type": "string"
    ///    },
    ///    "symbol": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct AmendOrderRequest {
        pub category: ::std::string::String,
        #[serde(rename = "orderId")]
        pub order_id: ::std::string::String,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub price: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub qty: ::std::option::Option<::std::string::String>,
        pub symbol: ::std::string::String,
    }
    ///`CancelOrderRequest`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "category",
    ///    "orderId",
    ///    "symbol"
    ///  ],
    ///  "properties": {
    ///    "category": {
    ///      "type": "string"
    ///    },
    ///    "orderId": {
    ///      "type": "string"
    ///    },
    ///    "orderLinkId": {
    ///      "type": "string"
    ///    },
    ///    "symbol": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct CancelOrderRequest {
        pub category: ::std::string::String,
        #[serde(rename = "orderId")]
        pub order_id: ::std::string::String,
        #[serde(
            rename = "orderLinkId",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub order_link_id: ::std::option::Option<::std::string::String>,
        pub symbol: ::std::string::String,
    }
    ///`CreateOrderRequest`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "category",
    ///    "orderType",
    ///    "qty",
    ///    "side",
    ///    "symbol"
    ///  ],
    ///  "properties": {
    ///    "category": {
    ///      "description": "`spot`, `linear`, `inverse`, or `option`.",
    ///      "type": "string"
    ///    },
    ///    "orderLinkId": {
    ///      "type": "string"
    ///    },
    ///    "orderType": {
    ///      "description": "`Limit` or `Market` (capitalised).",
    ///      "type": "string"
    ///    },
    ///    "price": {
    ///      "description": "Required for Limit orders.",
    ///      "type": "string"
    ///    },
    ///    "qty": {
    ///      "type": "string"
    ///    },
    ///    "reduceOnly": {
    ///      "type": "boolean"
    ///    },
    ///    "side": {
    ///      "description": "`Buy` or `Sell` (capitalised).",
    ///      "type": "string"
    ///    },
    ///    "symbol": {
    ///      "type": "string"
    ///    },
    ///    "timeInForce": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct CreateOrderRequest {
        ///`spot`, `linear`, `inverse`, or `option`.
        pub category: ::std::string::String,
        #[serde(
            rename = "orderLinkId",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub order_link_id: ::std::option::Option<::std::string::String>,
        ///`Limit` or `Market` (capitalised).
        #[serde(rename = "orderType")]
        pub order_type: ::std::string::String,
        ///Required for Limit orders.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub price: ::std::option::Option<::std::string::String>,
        pub qty: ::std::string::String,
        #[serde(
            rename = "reduceOnly",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub reduce_only: ::std::option::Option<bool>,
        ///`Buy` or `Sell` (capitalised).
        pub side: ::std::string::String,
        pub symbol: ::std::string::String,
        #[serde(
            rename = "timeInForce",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub time_in_force: ::std::option::Option<::std::string::String>,
    }
    ///`EmptyResultEnvelope`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "allOf": [
    ///    {
    ///      "$ref": "#/components/schemas/EnvelopeBase"
    ///    },
    ///    {
    ///      "type": "object",
    ///      "properties": {
    ///        "result": {
    ///          "type": "object",
    ///          "additionalProperties": true
    ///        }
    ///      }
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct EmptyResultEnvelope {
        #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
        pub result: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        #[serde(
            rename = "retCode",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub ret_code: ::std::option::Option<i64>,
        #[serde(
            rename = "retExtInfo",
            default,
            skip_serializing_if = "::serde_json::Map::is_empty"
        )]
        pub ret_ext_info: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        #[serde(
            rename = "retMsg",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub ret_msg: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub time: ::std::option::Option<i64>,
    }
    impl ::std::default::Default for EmptyResultEnvelope {
        fn default() -> Self {
            Self {
                result: Default::default(),
                ret_code: Default::default(),
                ret_ext_info: Default::default(),
                ret_msg: Default::default(),
                time: Default::default(),
            }
        }
    }
    ///`EnvelopeBase`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "retCode": {
    ///      "type": "integer",
    ///      "format": "int64"
    ///    },
    ///    "retExtInfo": {
    ///      "type": "object",
    ///      "additionalProperties": true
    ///    },
    ///    "retMsg": {
    ///      "type": "string"
    ///    },
    ///    "time": {
    ///      "type": "integer",
    ///      "format": "int64"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct EnvelopeBase {
        #[serde(
            rename = "retCode",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub ret_code: ::std::option::Option<i64>,
        #[serde(
            rename = "retExtInfo",
            default,
            skip_serializing_if = "::serde_json::Map::is_empty"
        )]
        pub ret_ext_info: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        #[serde(
            rename = "retMsg",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub ret_msg: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub time: ::std::option::Option<i64>,
    }
    impl ::std::default::Default for EnvelopeBase {
        fn default() -> Self {
            Self {
                ret_code: Default::default(),
                ret_ext_info: Default::default(),
                ret_msg: Default::default(),
                time: Default::default(),
            }
        }
    }
    ///`KlineEnvelope`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "allOf": [
    ///    {
    ///      "$ref": "#/components/schemas/EnvelopeBase"
    ///    },
    ///    {
    ///      "type": "object",
    ///      "properties": {
    ///        "result": {
    ///          "$ref": "#/components/schemas/KlineResult"
    ///        }
    ///      }
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct KlineEnvelope {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub result: ::std::option::Option<KlineResult>,
        #[serde(
            rename = "retCode",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub ret_code: ::std::option::Option<i64>,
        #[serde(
            rename = "retExtInfo",
            default,
            skip_serializing_if = "::serde_json::Map::is_empty"
        )]
        pub ret_ext_info: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        #[serde(
            rename = "retMsg",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub ret_msg: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub time: ::std::option::Option<i64>,
    }
    impl ::std::default::Default for KlineEnvelope {
        fn default() -> Self {
            Self {
                result: Default::default(),
                ret_code: Default::default(),
                ret_ext_info: Default::default(),
                ret_msg: Default::default(),
                time: Default::default(),
            }
        }
    }
    ///`KlineResult`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "category": {
    ///      "type": "string"
    ///    },
    ///    "list": {
    ///      "description": "Each candle is `[startTime, open, high, low, close, volume, turnover]`.\n",
    ///      "type": "array",
    ///      "items": {
    ///        "type": "array",
    ///        "items": {
    ///          "type": "string"
    ///        }
    ///      }
    ///    },
    ///    "symbol": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct KlineResult {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub category: ::std::option::Option<::std::string::String>,
        /**Each candle is `[startTime, open, high, low, close, volume, turnover]`.
*/
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub list: ::std::vec::Vec<::std::vec::Vec<::std::string::String>>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub symbol: ::std::option::Option<::std::string::String>,
    }
    impl ::std::default::Default for KlineResult {
        fn default() -> Self {
            Self {
                category: Default::default(),
                list: Default::default(),
                symbol: Default::default(),
            }
        }
    }
    ///`Order`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "avgPrice": {
    ///      "type": "string"
    ///    },
    ///    "createdTime": {
    ///      "type": "string"
    ///    },
    ///    "cumExecQty": {
    ///      "type": "string"
    ///    },
    ///    "orderId": {
    ///      "type": "string"
    ///    },
    ///    "orderLinkId": {
    ///      "type": "string"
    ///    },
    ///    "orderStatus": {
    ///      "type": "string"
    ///    },
    ///    "orderType": {
    ///      "type": "string"
    ///    },
    ///    "price": {
    ///      "type": "string"
    ///    },
    ///    "qty": {
    ///      "type": "string"
    ///    },
    ///    "side": {
    ///      "type": "string"
    ///    },
    ///    "symbol": {
    ///      "type": "string"
    ///    },
    ///    "timeInForce": {
    ///      "type": "string"
    ///    },
    ///    "updatedTime": {
    ///      "type": "string"
    ///    }
    ///  },
    ///  "additionalProperties": true
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct Order {
        #[serde(
            rename = "avgPrice",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub avg_price: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "createdTime",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub created_time: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "cumExecQty",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub cum_exec_qty: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "orderId",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub order_id: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "orderLinkId",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub order_link_id: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "orderStatus",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub order_status: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "orderType",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub order_type: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub price: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub qty: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub side: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub symbol: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "timeInForce",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub time_in_force: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "updatedTime",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub updated_time: ::std::option::Option<::std::string::String>,
    }
    impl ::std::default::Default for Order {
        fn default() -> Self {
            Self {
                avg_price: Default::default(),
                created_time: Default::default(),
                cum_exec_qty: Default::default(),
                order_id: Default::default(),
                order_link_id: Default::default(),
                order_status: Default::default(),
                order_type: Default::default(),
                price: Default::default(),
                qty: Default::default(),
                side: Default::default(),
                symbol: Default::default(),
                time_in_force: Default::default(),
                updated_time: Default::default(),
            }
        }
    }
    ///`OrderActionEnvelope`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "allOf": [
    ///    {
    ///      "$ref": "#/components/schemas/EnvelopeBase"
    ///    },
    ///    {
    ///      "type": "object",
    ///      "properties": {
    ///        "result": {
    ///          "$ref": "#/components/schemas/OrderActionResult"
    ///        }
    ///      }
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct OrderActionEnvelope {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub result: ::std::option::Option<OrderActionResult>,
        #[serde(
            rename = "retCode",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub ret_code: ::std::option::Option<i64>,
        #[serde(
            rename = "retExtInfo",
            default,
            skip_serializing_if = "::serde_json::Map::is_empty"
        )]
        pub ret_ext_info: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        #[serde(
            rename = "retMsg",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub ret_msg: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub time: ::std::option::Option<i64>,
    }
    impl ::std::default::Default for OrderActionEnvelope {
        fn default() -> Self {
            Self {
                result: Default::default(),
                ret_code: Default::default(),
                ret_ext_info: Default::default(),
                ret_msg: Default::default(),
                time: Default::default(),
            }
        }
    }
    ///`OrderActionResult`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "orderId": {
    ///      "type": "string"
    ///    },
    ///    "orderLinkId": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct OrderActionResult {
        #[serde(
            rename = "orderId",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub order_id: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "orderLinkId",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub order_link_id: ::std::option::Option<::std::string::String>,
    }
    impl ::std::default::Default for OrderActionResult {
        fn default() -> Self {
            Self {
                order_id: Default::default(),
                order_link_id: Default::default(),
            }
        }
    }
    ///`OrderListEnvelope`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "allOf": [
    ///    {
    ///      "$ref": "#/components/schemas/EnvelopeBase"
    ///    },
    ///    {
    ///      "type": "object",
    ///      "properties": {
    ///        "result": {
    ///          "$ref": "#/components/schemas/OrderListResult"
    ///        }
    ///      }
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct OrderListEnvelope {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub result: ::std::option::Option<OrderListResult>,
        #[serde(
            rename = "retCode",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub ret_code: ::std::option::Option<i64>,
        #[serde(
            rename = "retExtInfo",
            default,
            skip_serializing_if = "::serde_json::Map::is_empty"
        )]
        pub ret_ext_info: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        #[serde(
            rename = "retMsg",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub ret_msg: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub time: ::std::option::Option<i64>,
    }
    impl ::std::default::Default for OrderListEnvelope {
        fn default() -> Self {
            Self {
                result: Default::default(),
                ret_code: Default::default(),
                ret_ext_info: Default::default(),
                ret_msg: Default::default(),
                time: Default::default(),
            }
        }
    }
    ///`OrderListResult`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "category": {
    ///      "type": "string"
    ///    },
    ///    "list": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/Order"
    ///      }
    ///    },
    ///    "nextPageCursor": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct OrderListResult {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub category: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub list: ::std::vec::Vec<Order>,
        #[serde(
            rename = "nextPageCursor",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub next_page_cursor: ::std::option::Option<::std::string::String>,
    }
    impl ::std::default::Default for OrderListResult {
        fn default() -> Self {
            Self {
                category: Default::default(),
                list: Default::default(),
                next_page_cursor: Default::default(),
            }
        }
    }
    ///`OrderbookEnvelope`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "allOf": [
    ///    {
    ///      "$ref": "#/components/schemas/EnvelopeBase"
    ///    },
    ///    {
    ///      "type": "object",
    ///      "properties": {
    ///        "result": {
    ///          "$ref": "#/components/schemas/OrderbookResult"
    ///        }
    ///      }
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct OrderbookEnvelope {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub result: ::std::option::Option<OrderbookResult>,
        #[serde(
            rename = "retCode",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub ret_code: ::std::option::Option<i64>,
        #[serde(
            rename = "retExtInfo",
            default,
            skip_serializing_if = "::serde_json::Map::is_empty"
        )]
        pub ret_ext_info: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        #[serde(
            rename = "retMsg",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub ret_msg: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub time: ::std::option::Option<i64>,
    }
    impl ::std::default::Default for OrderbookEnvelope {
        fn default() -> Self {
            Self {
                result: Default::default(),
                ret_code: Default::default(),
                ret_ext_info: Default::default(),
                ret_msg: Default::default(),
                time: Default::default(),
            }
        }
    }
    ///`OrderbookResult`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "a": {
    ///      "description": "Asks — each entry is `[price, qty]`.",
    ///      "type": "array",
    ///      "items": {
    ///        "type": "array",
    ///        "items": {
    ///          "type": "string"
    ///        }
    ///      }
    ///    },
    ///    "b": {
    ///      "description": "Bids — each entry is `[price, qty]`.",
    ///      "type": "array",
    ///      "items": {
    ///        "type": "array",
    ///        "items": {
    ///          "type": "string"
    ///        }
    ///      }
    ///    },
    ///    "s": {
    ///      "type": "string"
    ///    },
    ///    "ts": {
    ///      "type": "integer",
    ///      "format": "int64"
    ///    },
    ///    "u": {
    ///      "type": "integer",
    ///      "format": "int64"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct OrderbookResult {
        ///Asks — each entry is `[price, qty]`.
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub a: ::std::vec::Vec<::std::vec::Vec<::std::string::String>>,
        ///Bids — each entry is `[price, qty]`.
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub b: ::std::vec::Vec<::std::vec::Vec<::std::string::String>>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub s: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub ts: ::std::option::Option<i64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub u: ::std::option::Option<i64>,
    }
    impl ::std::default::Default for OrderbookResult {
        fn default() -> Self {
            Self {
                a: Default::default(),
                b: Default::default(),
                s: Default::default(),
                ts: Default::default(),
                u: Default::default(),
            }
        }
    }
    ///`Position`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "avgPrice": {
    ///      "type": "string"
    ///    },
    ///    "cumRealisedPnl": {
    ///      "type": "string"
    ///    },
    ///    "leverage": {
    ///      "type": "string"
    ///    },
    ///    "liqPrice": {
    ///      "type": "string"
    ///    },
    ///    "markPrice": {
    ///      "type": "string"
    ///    },
    ///    "positionValue": {
    ///      "type": "string"
    ///    },
    ///    "side": {
    ///      "type": "string"
    ///    },
    ///    "size": {
    ///      "type": "string"
    ///    },
    ///    "symbol": {
    ///      "type": "string"
    ///    },
    ///    "unrealisedPnl": {
    ///      "type": "string"
    ///    }
    ///  },
    ///  "additionalProperties": true
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct Position {
        #[serde(
            rename = "avgPrice",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub avg_price: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "cumRealisedPnl",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub cum_realised_pnl: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub leverage: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "liqPrice",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub liq_price: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "markPrice",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub mark_price: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "positionValue",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub position_value: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub side: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub size: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub symbol: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "unrealisedPnl",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub unrealised_pnl: ::std::option::Option<::std::string::String>,
    }
    impl ::std::default::Default for Position {
        fn default() -> Self {
            Self {
                avg_price: Default::default(),
                cum_realised_pnl: Default::default(),
                leverage: Default::default(),
                liq_price: Default::default(),
                mark_price: Default::default(),
                position_value: Default::default(),
                side: Default::default(),
                size: Default::default(),
                symbol: Default::default(),
                unrealised_pnl: Default::default(),
            }
        }
    }
    ///`PositionListEnvelope`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "allOf": [
    ///    {
    ///      "$ref": "#/components/schemas/EnvelopeBase"
    ///    },
    ///    {
    ///      "type": "object",
    ///      "properties": {
    ///        "result": {
    ///          "$ref": "#/components/schemas/PositionListResult"
    ///        }
    ///      }
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct PositionListEnvelope {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub result: ::std::option::Option<PositionListResult>,
        #[serde(
            rename = "retCode",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub ret_code: ::std::option::Option<i64>,
        #[serde(
            rename = "retExtInfo",
            default,
            skip_serializing_if = "::serde_json::Map::is_empty"
        )]
        pub ret_ext_info: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        #[serde(
            rename = "retMsg",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub ret_msg: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub time: ::std::option::Option<i64>,
    }
    impl ::std::default::Default for PositionListEnvelope {
        fn default() -> Self {
            Self {
                result: Default::default(),
                ret_code: Default::default(),
                ret_ext_info: Default::default(),
                ret_msg: Default::default(),
                time: Default::default(),
            }
        }
    }
    ///`PositionListResult`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "category": {
    ///      "type": "string"
    ///    },
    ///    "list": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/Position"
    ///      }
    ///    },
    ///    "nextPageCursor": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct PositionListResult {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub category: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub list: ::std::vec::Vec<Position>,
        #[serde(
            rename = "nextPageCursor",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub next_page_cursor: ::std::option::Option<::std::string::String>,
    }
    impl ::std::default::Default for PositionListResult {
        fn default() -> Self {
            Self {
                category: Default::default(),
                list: Default::default(),
                next_page_cursor: Default::default(),
            }
        }
    }
    ///`SetLeverageRequest`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "buyLeverage",
    ///    "category",
    ///    "sellLeverage",
    ///    "symbol"
    ///  ],
    ///  "properties": {
    ///    "buyLeverage": {
    ///      "type": "string"
    ///    },
    ///    "category": {
    ///      "type": "string"
    ///    },
    ///    "sellLeverage": {
    ///      "type": "string"
    ///    },
    ///    "symbol": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct SetLeverageRequest {
        #[serde(rename = "buyLeverage")]
        pub buy_leverage: ::std::string::String,
        pub category: ::std::string::String,
        #[serde(rename = "sellLeverage")]
        pub sell_leverage: ::std::string::String,
        pub symbol: ::std::string::String,
    }
    ///`Ticker`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "ask1Price": {
    ///      "type": "string"
    ///    },
    ///    "ask1Size": {
    ///      "type": "string"
    ///    },
    ///    "bid1Price": {
    ///      "type": "string"
    ///    },
    ///    "bid1Size": {
    ///      "type": "string"
    ///    },
    ///    "fundingRate": {
    ///      "type": "string"
    ///    },
    ///    "highPrice24h": {
    ///      "type": "string"
    ///    },
    ///    "indexPrice": {
    ///      "type": "string"
    ///    },
    ///    "lastPrice": {
    ///      "type": "string"
    ///    },
    ///    "lowPrice24h": {
    ///      "type": "string"
    ///    },
    ///    "markPrice": {
    ///      "type": "string"
    ///    },
    ///    "openInterest": {
    ///      "type": "string"
    ///    },
    ///    "prevPrice24h": {
    ///      "type": "string"
    ///    },
    ///    "price24hPcnt": {
    ///      "type": "string"
    ///    },
    ///    "symbol": {
    ///      "type": "string"
    ///    },
    ///    "turnover24h": {
    ///      "type": "string"
    ///    },
    ///    "volume24h": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct Ticker {
        #[serde(
            rename = "ask1Price",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub ask1_price: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "ask1Size",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub ask1_size: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "bid1Price",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub bid1_price: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "bid1Size",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub bid1_size: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "fundingRate",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub funding_rate: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "highPrice24h",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub high_price24h: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "indexPrice",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub index_price: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "lastPrice",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub last_price: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "lowPrice24h",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub low_price24h: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "markPrice",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub mark_price: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "openInterest",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub open_interest: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "prevPrice24h",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub prev_price24h: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "price24hPcnt",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub price24h_pcnt: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub symbol: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub turnover24h: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub volume24h: ::std::option::Option<::std::string::String>,
    }
    impl ::std::default::Default for Ticker {
        fn default() -> Self {
            Self {
                ask1_price: Default::default(),
                ask1_size: Default::default(),
                bid1_price: Default::default(),
                bid1_size: Default::default(),
                funding_rate: Default::default(),
                high_price24h: Default::default(),
                index_price: Default::default(),
                last_price: Default::default(),
                low_price24h: Default::default(),
                mark_price: Default::default(),
                open_interest: Default::default(),
                prev_price24h: Default::default(),
                price24h_pcnt: Default::default(),
                symbol: Default::default(),
                turnover24h: Default::default(),
                volume24h: Default::default(),
            }
        }
    }
    ///`TickerListResult`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "category": {
    ///      "type": "string"
    ///    },
    ///    "list": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/Ticker"
    ///      }
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct TickerListResult {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub category: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub list: ::std::vec::Vec<Ticker>,
    }
    impl ::std::default::Default for TickerListResult {
        fn default() -> Self {
            Self {
                category: Default::default(),
                list: Default::default(),
            }
        }
    }
    ///`TickersEnvelope`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "allOf": [
    ///    {
    ///      "$ref": "#/components/schemas/EnvelopeBase"
    ///    },
    ///    {
    ///      "type": "object",
    ///      "properties": {
    ///        "result": {
    ///          "$ref": "#/components/schemas/TickerListResult"
    ///        }
    ///      }
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct TickersEnvelope {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub result: ::std::option::Option<TickerListResult>,
        #[serde(
            rename = "retCode",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub ret_code: ::std::option::Option<i64>,
        #[serde(
            rename = "retExtInfo",
            default,
            skip_serializing_if = "::serde_json::Map::is_empty"
        )]
        pub ret_ext_info: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        #[serde(
            rename = "retMsg",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub ret_msg: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub time: ::std::option::Option<i64>,
    }
    impl ::std::default::Default for TickersEnvelope {
        fn default() -> Self {
            Self {
                result: Default::default(),
                ret_code: Default::default(),
                ret_ext_info: Default::default(),
                ret_msg: Default::default(),
                time: Default::default(),
            }
        }
    }
    ///`WalletAccount`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "accountType": {
    ///      "type": "string"
    ///    },
    ///    "coin": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/WalletCoin"
    ///      }
    ///    },
    ///    "totalAvailableBalance": {
    ///      "type": "string"
    ///    },
    ///    "totalEquity": {
    ///      "type": "string"
    ///    },
    ///    "totalWalletBalance": {
    ///      "type": "string"
    ///    }
    ///  },
    ///  "additionalProperties": true
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct WalletAccount {
        #[serde(
            rename = "accountType",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub account_type: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub coin: ::std::vec::Vec<WalletCoin>,
        #[serde(
            rename = "totalAvailableBalance",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub total_available_balance: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "totalEquity",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub total_equity: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "totalWalletBalance",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub total_wallet_balance: ::std::option::Option<::std::string::String>,
    }
    impl ::std::default::Default for WalletAccount {
        fn default() -> Self {
            Self {
                account_type: Default::default(),
                coin: Default::default(),
                total_available_balance: Default::default(),
                total_equity: Default::default(),
                total_wallet_balance: Default::default(),
            }
        }
    }
    ///`WalletBalanceEnvelope`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "allOf": [
    ///    {
    ///      "$ref": "#/components/schemas/EnvelopeBase"
    ///    },
    ///    {
    ///      "type": "object",
    ///      "properties": {
    ///        "result": {
    ///          "$ref": "#/components/schemas/WalletBalanceResult"
    ///        }
    ///      }
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct WalletBalanceEnvelope {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub result: ::std::option::Option<WalletBalanceResult>,
        #[serde(
            rename = "retCode",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub ret_code: ::std::option::Option<i64>,
        #[serde(
            rename = "retExtInfo",
            default,
            skip_serializing_if = "::serde_json::Map::is_empty"
        )]
        pub ret_ext_info: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        #[serde(
            rename = "retMsg",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub ret_msg: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub time: ::std::option::Option<i64>,
    }
    impl ::std::default::Default for WalletBalanceEnvelope {
        fn default() -> Self {
            Self {
                result: Default::default(),
                ret_code: Default::default(),
                ret_ext_info: Default::default(),
                ret_msg: Default::default(),
                time: Default::default(),
            }
        }
    }
    ///`WalletBalanceResult`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "list": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/WalletAccount"
    ///      }
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct WalletBalanceResult {
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub list: ::std::vec::Vec<WalletAccount>,
    }
    impl ::std::default::Default for WalletBalanceResult {
        fn default() -> Self {
            Self { list: Default::default() }
        }
    }
    ///`WalletCoin`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "availableToWithdraw": {
    ///      "type": "string"
    ///    },
    ///    "coin": {
    ///      "type": "string"
    ///    },
    ///    "equity": {
    ///      "type": "string"
    ///    },
    ///    "usdValue": {
    ///      "type": "string"
    ///    },
    ///    "walletBalance": {
    ///      "type": "string"
    ///    }
    ///  },
    ///  "additionalProperties": true
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct WalletCoin {
        #[serde(
            rename = "availableToWithdraw",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub available_to_withdraw: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub coin: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub equity: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "usdValue",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub usd_value: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "walletBalance",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub wallet_balance: ::std::option::Option<::std::string::String>,
    }
    impl ::std::default::Default for WalletCoin {
        fn default() -> Self {
            Self {
                available_to_withdraw: Default::default(),
                coin: Default::default(),
                equity: Default::default(),
                usd_value: Default::default(),
                wallet_balance: Default::default(),
            }
        }
    }
}
#[derive(Clone, Debug)]
/**Client for Bybit V5 Unified API

Bybit V5 unified API. Spot, USDT/USDC perpetuals (linear), coin-margined perpetuals (inverse),
and options all live behind one set of endpoints, distinguished by a `category` parameter.

## Auth
Authenticated endpoints require these request headers, sent on every signed call:
  - `X-BAPI-API-KEY`     — your API key
  - `X-BAPI-TIMESTAMP`   — current millis since epoch
  - `X-BAPI-RECV-WINDOW` — request validity window (commonly `5000`)
  - `X-BAPI-SIGN`        — HMAC-SHA256 hex signature

The signature payload is `timestamp + api_key + recv_window + payload`, where `payload` is
the URL-encoded query string for GET requests, or the raw JSON body for POST requests. The
HMAC key is the API secret. The signing logic is hand-written in `ext/src/bybit/auth.rs`
— this spec only describes the resulting headers for codegen purposes.

All responses share an envelope: `{ retCode, retMsg, result, retExtInfo, time }`. A non-zero
`retCode` indicates a logical error even on HTTP 200.


Version: 5.0*/
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
        "5.0"
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
    /**Latest price snapshot for symbols in a category

Returns the latest price snapshot (last price, 24h volume, best bid/ask, etc.) for one
or all symbols in a product category. Public endpoint — no signing required.


Sends a `GET` request to `/v5/market/tickers`

Arguments:
- `base_coin`
- `category`: Product category — `spot`, `linear`, `inverse`, or `option`.
- `exp_date`
- `symbol`: Symbol like `BTCUSDT`. Omit to return all symbols in the category.
*/
    pub async fn get_tickers<'a>(
        &'a self,
        base_coin: Option<&'a str>,
        category: &'a str,
        exp_date: Option<&'a str>,
        symbol: Option<&'a str>,
    ) -> Result<ResponseValue<types::TickersEnvelope>, Error<()>> {
        let url = format!("{}/v5/market/tickers", self.baseurl,);
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
            .query(&progenitor_client::QueryParam::new("baseCoin", &base_coin))
            .query(&progenitor_client::QueryParam::new("category", &category))
            .query(&progenitor_client::QueryParam::new("expDate", &exp_date))
            .query(&progenitor_client::QueryParam::new("symbol", &symbol))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "get_tickers",
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
    /**Order book snapshot

Bids and asks for a symbol at the requested depth. Public endpoint.

Sends a `GET` request to `/v5/market/orderbook`

Arguments:
- `category`: Product category — `spot`, `linear`, `inverse`, or `option`.
- `limit`: Depth — typical values 1, 25, 50, 200.
- `symbol`
*/
    pub async fn get_orderbook<'a>(
        &'a self,
        category: &'a str,
        limit: Option<i32>,
        symbol: &'a str,
    ) -> Result<ResponseValue<types::OrderbookEnvelope>, Error<()>> {
        let url = format!("{}/v5/market/orderbook", self.baseurl,);
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
            .query(&progenitor_client::QueryParam::new("limit", &limit))
            .query(&progenitor_client::QueryParam::new("symbol", &symbol))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "get_orderbook",
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
    /**Historical candles (OHLCV)

OHLCV candles for a symbol at the chosen interval. Public endpoint. Intervals:
`1`, `3`, `5`, `15`, `30`, `60`, `120`, `240`, `360`, `720`, `D`, `W`, `M`.


Sends a `GET` request to `/v5/market/kline`

Arguments:
- `category`: Product category — `spot`, `linear`, `inverse`, or `option`.
- `end`: End timestamp in ms.
- `interval`
- `limit`
- `start`: Start timestamp in ms.
- `symbol`
*/
    pub async fn get_kline<'a>(
        &'a self,
        category: &'a str,
        end: Option<i64>,
        interval: &'a str,
        limit: Option<i32>,
        start: Option<i64>,
        symbol: &'a str,
    ) -> Result<ResponseValue<types::KlineEnvelope>, Error<()>> {
        let url = format!("{}/v5/market/kline", self.baseurl,);
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
            .query(&progenitor_client::QueryParam::new("end", &end))
            .query(&progenitor_client::QueryParam::new("interval", &interval))
            .query(&progenitor_client::QueryParam::new("limit", &limit))
            .query(&progenitor_client::QueryParam::new("start", &start))
            .query(&progenitor_client::QueryParam::new("symbol", &symbol))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "get_kline",
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
    /**Account wallet balance

Wallet balance per coin for the unified or contract account. Signed.

Sends a `GET` request to `/v5/account/wallet-balance`

Arguments:
- `account_type`: `UNIFIED` (default) or `CONTRACT`.
- `coin`
- `x_bapi_api_key`
- `x_bapi_recv_window`
- `x_bapi_sign`
- `x_bapi_timestamp`
*/
    pub async fn get_wallet_balance<'a>(
        &'a self,
        account_type: &'a str,
        coin: Option<&'a str>,
        x_bapi_api_key: &'a str,
        x_bapi_recv_window: &'a str,
        x_bapi_sign: &'a str,
        x_bapi_timestamp: &'a str,
    ) -> Result<ResponseValue<types::WalletBalanceEnvelope>, Error<()>> {
        let url = format!("{}/v5/account/wallet-balance", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(5usize);
        header_map
            .append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(Self::api_version()),
            );
        header_map.append("X-BAPI-API-KEY", x_bapi_api_key.to_string().try_into()?);
        header_map
            .append("X-BAPI-RECV-WINDOW", x_bapi_recv_window.to_string().try_into()?);
        header_map.append("X-BAPI-SIGN", x_bapi_sign.to_string().try_into()?);
        header_map.append("X-BAPI-TIMESTAMP", x_bapi_timestamp.to_string().try_into()?);
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new("accountType", &account_type))
            .query(&progenitor_client::QueryParam::new("coin", &coin))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "get_wallet_balance",
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
    /**Open derivative positions

Open perpetual or inverse positions. Signed. `category` must be `linear` or `inverse`.

Sends a `GET` request to `/v5/position/list`

Arguments:
- `category`: Product category — `spot`, `linear`, `inverse`, or `option`.
- `settle_coin`
- `symbol`
- `x_bapi_api_key`
- `x_bapi_recv_window`
- `x_bapi_sign`
- `x_bapi_timestamp`
*/
    pub async fn get_positions<'a>(
        &'a self,
        category: &'a str,
        settle_coin: Option<&'a str>,
        symbol: Option<&'a str>,
        x_bapi_api_key: &'a str,
        x_bapi_recv_window: &'a str,
        x_bapi_sign: &'a str,
        x_bapi_timestamp: &'a str,
    ) -> Result<ResponseValue<types::PositionListEnvelope>, Error<()>> {
        let url = format!("{}/v5/position/list", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(5usize);
        header_map
            .append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(Self::api_version()),
            );
        header_map.append("X-BAPI-API-KEY", x_bapi_api_key.to_string().try_into()?);
        header_map
            .append("X-BAPI-RECV-WINDOW", x_bapi_recv_window.to_string().try_into()?);
        header_map.append("X-BAPI-SIGN", x_bapi_sign.to_string().try_into()?);
        header_map.append("X-BAPI-TIMESTAMP", x_bapi_timestamp.to_string().try_into()?);
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new("category", &category))
            .query(&progenitor_client::QueryParam::new("settleCoin", &settle_coin))
            .query(&progenitor_client::QueryParam::new("symbol", &symbol))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "get_positions",
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
    /**Open / unfilled orders

Real-time open orders. Signed.

Sends a `GET` request to `/v5/order/realtime`

Arguments:
- `category`: Product category — `spot`, `linear`, `inverse`, or `option`.
- `limit`
- `order_id`
- `symbol`
- `x_bapi_api_key`
- `x_bapi_recv_window`
- `x_bapi_sign`
- `x_bapi_timestamp`
*/
    pub async fn get_open_orders<'a>(
        &'a self,
        category: &'a str,
        limit: Option<i32>,
        order_id: Option<&'a str>,
        symbol: Option<&'a str>,
        x_bapi_api_key: &'a str,
        x_bapi_recv_window: &'a str,
        x_bapi_sign: &'a str,
        x_bapi_timestamp: &'a str,
    ) -> Result<ResponseValue<types::OrderListEnvelope>, Error<()>> {
        let url = format!("{}/v5/order/realtime", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(5usize);
        header_map
            .append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(Self::api_version()),
            );
        header_map.append("X-BAPI-API-KEY", x_bapi_api_key.to_string().try_into()?);
        header_map
            .append("X-BAPI-RECV-WINDOW", x_bapi_recv_window.to_string().try_into()?);
        header_map.append("X-BAPI-SIGN", x_bapi_sign.to_string().try_into()?);
        header_map.append("X-BAPI-TIMESTAMP", x_bapi_timestamp.to_string().try_into()?);
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new("category", &category))
            .query(&progenitor_client::QueryParam::new("limit", &limit))
            .query(&progenitor_client::QueryParam::new("orderId", &order_id))
            .query(&progenitor_client::QueryParam::new("symbol", &symbol))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "get_open_orders",
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
    /**Filled / cancelled order history

Historical orders (filled, cancelled, rejected). Signed.

Sends a `GET` request to `/v5/order/history`

Arguments:
- `category`: Product category — `spot`, `linear`, `inverse`, or `option`.
- `limit`
- `order_id`
- `symbol`
- `x_bapi_api_key`
- `x_bapi_recv_window`
- `x_bapi_sign`
- `x_bapi_timestamp`
*/
    pub async fn get_order_history<'a>(
        &'a self,
        category: &'a str,
        limit: Option<i32>,
        order_id: Option<&'a str>,
        symbol: Option<&'a str>,
        x_bapi_api_key: &'a str,
        x_bapi_recv_window: &'a str,
        x_bapi_sign: &'a str,
        x_bapi_timestamp: &'a str,
    ) -> Result<ResponseValue<types::OrderListEnvelope>, Error<()>> {
        let url = format!("{}/v5/order/history", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(5usize);
        header_map
            .append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(Self::api_version()),
            );
        header_map.append("X-BAPI-API-KEY", x_bapi_api_key.to_string().try_into()?);
        header_map
            .append("X-BAPI-RECV-WINDOW", x_bapi_recv_window.to_string().try_into()?);
        header_map.append("X-BAPI-SIGN", x_bapi_sign.to_string().try_into()?);
        header_map.append("X-BAPI-TIMESTAMP", x_bapi_timestamp.to_string().try_into()?);
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new("category", &category))
            .query(&progenitor_client::QueryParam::new("limit", &limit))
            .query(&progenitor_client::QueryParam::new("orderId", &order_id))
            .query(&progenitor_client::QueryParam::new("symbol", &symbol))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "get_order_history",
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
    /**Place a new order

Place a new spot or derivatives order. Signed.

Sends a `POST` request to `/v5/order/create`

*/
    pub async fn create_order<'a>(
        &'a self,
        x_bapi_api_key: &'a str,
        x_bapi_recv_window: &'a str,
        x_bapi_sign: &'a str,
        x_bapi_timestamp: &'a str,
        body: &'a types::CreateOrderRequest,
    ) -> Result<ResponseValue<types::OrderActionEnvelope>, Error<()>> {
        let url = format!("{}/v5/order/create", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(5usize);
        header_map
            .append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(Self::api_version()),
            );
        header_map.append("X-BAPI-API-KEY", x_bapi_api_key.to_string().try_into()?);
        header_map
            .append("X-BAPI-RECV-WINDOW", x_bapi_recv_window.to_string().try_into()?);
        header_map.append("X-BAPI-SIGN", x_bapi_sign.to_string().try_into()?);
        header_map.append("X-BAPI-TIMESTAMP", x_bapi_timestamp.to_string().try_into()?);
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
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Cancel an order

Cancel a single order by orderId. Signed.

Sends a `POST` request to `/v5/order/cancel`

*/
    pub async fn cancel_order<'a>(
        &'a self,
        x_bapi_api_key: &'a str,
        x_bapi_recv_window: &'a str,
        x_bapi_sign: &'a str,
        x_bapi_timestamp: &'a str,
        body: &'a types::CancelOrderRequest,
    ) -> Result<ResponseValue<types::OrderActionEnvelope>, Error<()>> {
        let url = format!("{}/v5/order/cancel", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(5usize);
        header_map
            .append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(Self::api_version()),
            );
        header_map.append("X-BAPI-API-KEY", x_bapi_api_key.to_string().try_into()?);
        header_map
            .append("X-BAPI-RECV-WINDOW", x_bapi_recv_window.to_string().try_into()?);
        header_map.append("X-BAPI-SIGN", x_bapi_sign.to_string().try_into()?);
        header_map.append("X-BAPI-TIMESTAMP", x_bapi_timestamp.to_string().try_into()?);
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
            operation_id: "cancel_order",
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
    /**Amend an open order

Modify quantity and/or price of an open order. Signed.

Sends a `POST` request to `/v5/order/amend`

*/
    pub async fn amend_order<'a>(
        &'a self,
        x_bapi_api_key: &'a str,
        x_bapi_recv_window: &'a str,
        x_bapi_sign: &'a str,
        x_bapi_timestamp: &'a str,
        body: &'a types::AmendOrderRequest,
    ) -> Result<ResponseValue<types::OrderActionEnvelope>, Error<()>> {
        let url = format!("{}/v5/order/amend", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(5usize);
        header_map
            .append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(Self::api_version()),
            );
        header_map.append("X-BAPI-API-KEY", x_bapi_api_key.to_string().try_into()?);
        header_map
            .append("X-BAPI-RECV-WINDOW", x_bapi_recv_window.to_string().try_into()?);
        header_map.append("X-BAPI-SIGN", x_bapi_sign.to_string().try_into()?);
        header_map.append("X-BAPI-TIMESTAMP", x_bapi_timestamp.to_string().try_into()?);
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
            operation_id: "amend_order",
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
    /**Set leverage for a derivatives symbol

Set per-symbol leverage (separate buy and sell legs). Signed.

Sends a `POST` request to `/v5/position/set-leverage`

*/
    pub async fn set_leverage<'a>(
        &'a self,
        x_bapi_api_key: &'a str,
        x_bapi_recv_window: &'a str,
        x_bapi_sign: &'a str,
        x_bapi_timestamp: &'a str,
        body: &'a types::SetLeverageRequest,
    ) -> Result<ResponseValue<types::EmptyResultEnvelope>, Error<()>> {
        let url = format!("{}/v5/position/set-leverage", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(5usize);
        header_map
            .append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(Self::api_version()),
            );
        header_map.append("X-BAPI-API-KEY", x_bapi_api_key.to_string().try_into()?);
        header_map
            .append("X-BAPI-RECV-WINDOW", x_bapi_recv_window.to_string().try_into()?);
        header_map.append("X-BAPI-SIGN", x_bapi_sign.to_string().try_into()?);
        header_map.append("X-BAPI-TIMESTAMP", x_bapi_timestamp.to_string().try_into()?);
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
            operation_id: "set_leverage",
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
