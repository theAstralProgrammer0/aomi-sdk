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
    /**Per-agent guardrails returned by Simmer (max position size, daily
spend, etc.). Shape varies — kept loose for forward compat.
*/
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Per-agent guardrails returned by Simmer (max position size, daily\nspend, etc.). Shape varies — kept loose for forward compat.\n",
    ///  "type": "object",
    ///  "additionalProperties": true
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    #[serde(transparent)]
    pub struct AgentLimits(
        pub ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    );
    impl ::std::ops::Deref for AgentLimits {
        type Target = ::serde_json::Map<::std::string::String, ::serde_json::Value>;
        fn deref(
            &self,
        ) -> &::serde_json::Map<::std::string::String, ::serde_json::Value> {
            &self.0
        }
    }
    impl ::std::convert::From<AgentLimits>
    for ::serde_json::Map<::std::string::String, ::serde_json::Value> {
        fn from(value: AgentLimits) -> Self {
            value.0
        }
    }
    impl ::std::convert::From<
        ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    > for AgentLimits {
        fn from(
            value: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        ) -> Self {
            Self(value)
        }
    }
    ///Sandbox + live balances and onboarding state.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Sandbox + live balances and onboarding state.",
    ///  "type": "object",
    ///  "properties": {
    ///    "agent_id": {
    ///      "type": "string"
    ///    },
    ///    "balance": {
    ///      "description": "Fallback alias for `balance_usd` from older API versions.",
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "balance_usd": {
    ///      "description": "Live USD balance (if live trading is enabled).",
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "claim_url": {
    ///      "type": "string"
    ///    },
    ///    "limits": {
    ///      "$ref": "#/components/schemas/AgentLimits"
    ///    },
    ///    "name": {
    ///      "type": "string"
    ///    },
    ///    "real_trading_enabled": {
    ///      "description": "True iff the user has claimed the agent and live Kalshi trading is unlocked.",
    ///      "type": "boolean"
    ///    },
    ///    "sim_balance": {
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "status": {
    ///      "description": "e.g. `pending_claim`, `claimed`, `live`.",
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct AgentStatus {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub agent_id: ::std::option::Option<::std::string::String>,
        ///Fallback alias for `balance_usd` from older API versions.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub balance: ::std::option::Option<f64>,
        ///Live USD balance (if live trading is enabled).
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub balance_usd: ::std::option::Option<f64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub claim_url: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub limits: ::std::option::Option<AgentLimits>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub name: ::std::option::Option<::std::string::String>,
        ///True iff the user has claimed the agent and live Kalshi trading is unlocked.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub real_trading_enabled: ::std::option::Option<bool>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub sim_balance: ::std::option::Option<f64>,
        ///e.g. `pending_claim`, `claimed`, `live`.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub status: ::std::option::Option<::std::string::String>,
    }
    impl ::std::default::Default for AgentStatus {
        fn default() -> Self {
            Self {
                agent_id: Default::default(),
                balance: Default::default(),
                balance_usd: Default::default(),
                claim_url: Default::default(),
                limits: Default::default(),
                name: Default::default(),
                real_trading_enabled: Default::default(),
                sim_balance: Default::default(),
                status: Default::default(),
            }
        }
    }
    ///One-shot dashboard. Inner objects are kept loose intentionally.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "One-shot dashboard. Inner objects are kept loose intentionally.",
    ///  "type": "object",
    ///  "properties": {
    ///    "checked_at": {
    ///      "type": "string"
    ///    },
    ///    "opportunities": {
    ///      "type": "array",
    ///      "items": {
    ///        "type": "object",
    ///        "additionalProperties": true
    ///      }
    ///    },
    ///    "performance": {
    ///      "type": "object",
    ///      "additionalProperties": true
    ///    },
    ///    "portfolio": {
    ///      "type": "object",
    ///      "additionalProperties": true
    ///    },
    ///    "positions": {
    ///      "type": "array",
    ///      "items": {
    ///        "type": "object",
    ///        "additionalProperties": true
    ///      }
    ///    },
    ///    "risk_alerts": {
    ///      "type": "array",
    ///      "items": {
    ///        "type": "object",
    ///        "additionalProperties": true
    ///      }
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct Briefing {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub checked_at: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub opportunities: ::std::vec::Vec<
            ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        >,
        #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
        pub performance: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
        pub portfolio: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub positions: ::std::vec::Vec<
            ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        >,
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub risk_alerts: ::std::vec::Vec<
            ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        >,
    }
    impl ::std::default::Default for Briefing {
        fn default() -> Self {
            Self {
                checked_at: Default::default(),
                opportunities: Default::default(),
                performance: Default::default(),
                portfolio: Default::default(),
                positions: Default::default(),
                risk_alerts: Default::default(),
            }
        }
    }
    ///`ImportKalshiMarketRequest`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "kalshi_url"
    ///  ],
    ///  "properties": {
    ///    "kalshi_url": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ImportKalshiMarketRequest {
        pub kalshi_url: ::std::string::String,
    }
    /**A Kalshi market we can import. The `url` is the Kalshi market URL
(NOT a Simmer UUID); pass it to `importKalshiMarket` to convert.
Inner shape kept additive — Simmer adds new metadata fields over time.
*/
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "A Kalshi market we can import. The `url` is the Kalshi market URL\n(NOT a Simmer UUID); pass it to `importKalshiMarket` to convert.\nInner shape kept additive — Simmer adds new metadata fields over time.\n",
    ///  "type": "object",
    ///  "properties": {
    ///    "external_volume": {
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "no_price": {
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "ticker": {
    ///      "type": "string"
    ///    },
    ///    "title": {
    ///      "type": "string"
    ///    },
    ///    "url": {
    ///      "type": "string"
    ///    },
    ///    "yes_price": {
    ///      "type": "number",
    ///      "format": "double"
    ///    }
    ///  },
    ///  "additionalProperties": true
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ImportableMarket {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub external_volume: ::std::option::Option<f64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub no_price: ::std::option::Option<f64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub ticker: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub title: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub url: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub yes_price: ::std::option::Option<f64>,
    }
    impl ::std::default::Default for ImportableMarket {
        fn default() -> Self {
            Self {
                external_volume: Default::default(),
                no_price: Default::default(),
                ticker: Default::default(),
                title: Default::default(),
                url: Default::default(),
                yes_price: Default::default(),
            }
        }
    }
    ///`ImportableMarketsResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "markets": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/ImportableMarket"
    ///      }
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ImportableMarketsResponse {
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub markets: ::std::vec::Vec<ImportableMarket>,
    }
    impl ::std::default::Default for ImportableMarketsResponse {
        fn default() -> Self {
            Self {
                markets: Default::default(),
            }
        }
    }
    /**Result of registering a Kalshi market. The `market_id` is the Simmer
UUID required by `getMarketContext` and `trade`.
*/
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Result of registering a Kalshi market. The `market_id` is the Simmer\nUUID required by `getMarketContext` and `trade`.\n",
    ///  "type": "object",
    ///  "properties": {
    ///    "kalshi_url": {
    ///      "description": "Fallback alias for `url` from older API versions.",
    ///      "type": "string"
    ///    },
    ///    "market": {
    ///      "type": "object",
    ///      "additionalProperties": true
    ///    },
    ///    "market_id": {
    ///      "type": "string"
    ///    },
    ///    "status": {
    ///      "type": "string"
    ///    },
    ///    "url": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ImportedMarket {
        ///Fallback alias for `url` from older API versions.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub kalshi_url: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
        pub market: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub market_id: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub status: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub url: ::std::option::Option<::std::string::String>,
    }
    impl ::std::default::Default for ImportedMarket {
        fn default() -> Self {
            Self {
                kalshi_url: Default::default(),
                market: Default::default(),
                market_id: Default::default(),
                status: Default::default(),
                url: Default::default(),
            }
        }
    }
    ///Pre-trade context — surface warnings + slippage + fees before placing.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Pre-trade context — surface warnings + slippage + fees before placing.",
    ///  "type": "object",
    ///  "properties": {
    ///    "fee_note": {
    ///      "type": "string"
    ///    },
    ///    "fee_rate_bps": {
    ///      "type": "integer"
    ///    },
    ///    "is_paid": {
    ///      "type": "boolean"
    ///    },
    ///    "market": {
    ///      "type": "object",
    ///      "additionalProperties": true
    ///    },
    ///    "position": {
    ///      "description": "User's existing position on this market, if any.",
    ///      "type": "object",
    ///      "additionalProperties": true
    ///    },
    ///    "resolution_criteria": {
    ///      "type": "string"
    ///    },
    ///    "slippage_estimate": {
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "time_to_resolution": {
    ///      "type": "string"
    ///    },
    ///    "warnings": {
    ///      "type": "array",
    ///      "items": {
    ///        "type": "string"
    ///      }
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct MarketContext {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub fee_note: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub fee_rate_bps: ::std::option::Option<i64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub is_paid: ::std::option::Option<bool>,
        #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
        pub market: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        ///User's existing position on this market, if any.
        #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
        pub position: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub resolution_criteria: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub slippage_estimate: ::std::option::Option<f64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub time_to_resolution: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub warnings: ::std::vec::Vec<::std::string::String>,
    }
    impl ::std::default::Default for MarketContext {
        fn default() -> Self {
            Self {
                fee_note: Default::default(),
                fee_rate_bps: Default::default(),
                is_paid: Default::default(),
                market: Default::default(),
                position: Default::default(),
                resolution_criteria: Default::default(),
                slippage_estimate: Default::default(),
                time_to_resolution: Default::default(),
                warnings: Default::default(),
            }
        }
    }
    ///`Portfolio`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "balance": {
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "currency": {
    ///      "type": "string"
    ///    },
    ///    "positions_value": {
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "realized_pnl": {
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "total_value": {
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "unrealized_pnl": {
    ///      "type": "number",
    ///      "format": "double"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct Portfolio {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub balance: ::std::option::Option<f64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub currency: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub positions_value: ::std::option::Option<f64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub realized_pnl: ::std::option::Option<f64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub total_value: ::std::option::Option<f64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub unrealized_pnl: ::std::option::Option<f64>,
    }
    impl ::std::default::Default for Portfolio {
        fn default() -> Self {
            Self {
                balance: Default::default(),
                currency: Default::default(),
                positions_value: Default::default(),
                realized_pnl: Default::default(),
                total_value: Default::default(),
                unrealized_pnl: Default::default(),
            }
        }
    }
    ///A single open position, with PnL.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "A single open position, with PnL.",
    ///  "type": "object",
    ///  "properties": {
    ///    "average_price": {
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "current_price": {
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "market_id": {
    ///      "type": "string"
    ///    },
    ///    "pnl": {
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "shares": {
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "side": {
    ///      "type": "string"
    ///    }
    ///  },
    ///  "additionalProperties": true
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct Position {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub average_price: ::std::option::Option<f64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub current_price: ::std::option::Option<f64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub market_id: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub pnl: ::std::option::Option<f64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub shares: ::std::option::Option<f64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub side: ::std::option::Option<::std::string::String>,
    }
    impl ::std::default::Default for Position {
        fn default() -> Self {
            Self {
                average_price: Default::default(),
                current_price: Default::default(),
                market_id: Default::default(),
                pnl: Default::default(),
                shares: Default::default(),
                side: Default::default(),
            }
        }
    }
    ///`PositionsResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "positions": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/Position"
    ///      }
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct PositionsResponse {
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub positions: ::std::vec::Vec<Position>,
    }
    impl ::std::default::Default for PositionsResponse {
        fn default() -> Self {
            Self {
                positions: Default::default(),
            }
        }
    }
    ///`RegisterAgentRequest`
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
    ///    "description": {
    ///      "type": "string"
    ///    },
    ///    "name": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct RegisterAgentRequest {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub description: ::std::option::Option<::std::string::String>,
        pub name: ::std::string::String,
    }
    /**Returned once on first registration. `api_key` is `sk_…` and must be
saved (the user runs `/apikey simmer <key>`). `claim_url` is where
the user verifies identity before live Kalshi trading.
*/
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Returned once on first registration. `api_key` is `sk_…` and must be\nsaved (the user runs `/apikey simmer <key>`). `claim_url` is where\nthe user verifies identity before live Kalshi trading.\n",
    ///  "type": "object",
    ///  "required": [
    ///    "api_key"
    ///  ],
    ///  "properties": {
    ///    "agent_id": {
    ///      "type": "string"
    ///    },
    ///    "api_key": {
    ///      "type": "string"
    ///    },
    ///    "claim_code": {
    ///      "type": "string"
    ///    },
    ///    "claim_url": {
    ///      "type": "string"
    ///    },
    ///    "limits": {
    ///      "$ref": "#/components/schemas/AgentLimits"
    ///    },
    ///    "starting_balance": {
    ///      "type": "number",
    ///      "format": "double"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct RegisterAgentResponse {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub agent_id: ::std::option::Option<::std::string::String>,
        pub api_key: ::std::string::String,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub claim_code: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub claim_url: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub limits: ::std::option::Option<AgentLimits>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub starting_balance: ::std::option::Option<f64>,
    }
    ///`TradeRequest`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "action",
    ///    "market_id",
    ///    "side",
    ///    "source",
    ///    "venue"
    ///  ],
    ///  "properties": {
    ///    "action": {
    ///      "description": "`buy` or `sell`.",
    ///      "type": "string"
    ///    },
    ///    "amount": {
    ///      "description": "USD amount (buys only).",
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "dry_run": {
    ///      "type": "boolean"
    ///    },
    ///    "market_id": {
    ///      "type": "string"
    ///    },
    ///    "reasoning": {
    ///      "type": "string"
    ///    },
    ///    "shares": {
    ///      "description": "Shares quantity (required for sells).",
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "side": {
    ///      "description": "`yes` or `no`.",
    ///      "type": "string"
    ///    },
    ///    "source": {
    ///      "type": "string"
    ///    },
    ///    "venue": {
    ///      "description": "`sim` or `kalshi`.",
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct TradeRequest {
        ///`buy` or `sell`.
        pub action: ::std::string::String,
        ///USD amount (buys only).
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub amount: ::std::option::Option<f64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub dry_run: ::std::option::Option<bool>,
        pub market_id: ::std::string::String,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub reasoning: ::std::option::Option<::std::string::String>,
        ///Shares quantity (required for sells).
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub shares: ::std::option::Option<f64>,
        ///`yes` or `no`.
        pub side: ::std::string::String,
        pub source: ::std::string::String,
        ///`sim` or `kalshi`.
        pub venue: ::std::string::String,
    }
    ///Returned by /trade on success. `shares_bought` for buys, `shares_sold` for sells.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Returned by /trade on success. `shares_bought` for buys, `shares_sold` for sells.",
    ///  "type": "object",
    ///  "required": [
    ///    "trade_id"
    ///  ],
    ///  "properties": {
    ///    "average_price": {
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "cost": {
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "market_id": {
    ///      "type": "string"
    ///    },
    ///    "shares_bought": {
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "shares_sold": {
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "side": {
    ///      "type": "string"
    ///    },
    ///    "trade_id": {
    ///      "type": "string"
    ///    },
    ///    "venue": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct TradeResult {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub average_price: ::std::option::Option<f64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub cost: ::std::option::Option<f64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub market_id: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub shares_bought: ::std::option::Option<f64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub shares_sold: ::std::option::Option<f64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub side: ::std::option::Option<::std::string::String>,
        pub trade_id: ::std::string::String,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub venue: ::std::option::Option<::std::string::String>,
    }
}
#[derive(Clone, Debug)]
/**Client for Simmer SDK API (Kalshi proxy)

The Aomi `kalshi` app trades Kalshi prediction markets via the Simmer SDK
(https://api.simmer.markets). Only the endpoints actually called by
`apps/kalshi/src/tool.rs` are described here.

## Auth
All authenticated endpoints take a `Bearer` token in the `Authorization`
header (the Simmer API key, format `sk_…`). The agent-register endpoint is
unauthenticated. No HMAC signing — the bearer token is enough.

## Response trimming policy
Response schemas list ONLY the fields the curated `apps/kalshi/src/tool.rs`
actually surfaces to the LLM. The live Simmer API returns more fields (UI
blobs, ICS calendars, mutable display flags); those are silently dropped on
deserialization since the spec doesn't list them. To re-enable a field, add
it to the appropriate schema below and `aomi-build gen-client kalshi
--shared --force`.


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
    /**Register a new Simmer agent (unauthenticated)

Returns an `api_key`, `claim_url`, and starting sandbox balance. The
user must visit `claim_url` to verify identity before live Kalshi
trading is enabled.


Sends a `POST` request to `/api/sdk/agents/register`

*/
    pub async fn register_agent<'a>(
        &'a self,
        body: &'a types::RegisterAgentRequest,
    ) -> Result<ResponseValue<types::RegisterAgentResponse>, Error<()>> {
        let url = format!("{}/api/sdk/agents/register", self.baseurl,);
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
            operation_id: "register_agent",
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
    /**Status of the agent tied to the bearer token

Sends a `GET` request to `/api/sdk/agents/me`

Arguments:
- `authorization`: `Bearer sk_…` — the Simmer API key.
*/
    pub async fn get_agent_status<'a>(
        &'a self,
        authorization: &'a str,
    ) -> Result<ResponseValue<types::AgentStatus>, Error<()>> {
        let url = format!("{}/api/sdk/agents/me", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(2usize);
        header_map
            .append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(Self::api_version()),
            );
        header_map.append("Authorization", authorization.to_string().try_into()?);
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
            operation_id: "get_agent_status",
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
    /**One-shot dashboard for the agent

Portfolio snapshot, positions, opportunities, risk alerts, performance.

Sends a `GET` request to `/api/sdk/briefing`

Arguments:
- `since`: ISO timestamp to scope to changes since a moment.
- `authorization`: `Bearer sk_…` — the Simmer API key.
*/
    pub async fn get_briefing<'a>(
        &'a self,
        since: Option<&'a str>,
        authorization: &'a str,
    ) -> Result<ResponseValue<types::Briefing>, Error<()>> {
        let url = format!("{}/api/sdk/briefing", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(2usize);
        header_map
            .append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(Self::api_version()),
            );
        header_map.append("Authorization", authorization.to_string().try_into()?);
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new("since", &since))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "get_briefing",
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
    /**Pre-trade context for a Simmer market (warnings, slippage, fees)

Sends a `GET` request to `/api/sdk/context/{market_id}`

Arguments:
- `market_id`
- `authorization`: `Bearer sk_…` — the Simmer API key.
*/
    pub async fn get_market_context<'a>(
        &'a self,
        market_id: &'a str,
        authorization: &'a str,
    ) -> Result<ResponseValue<types::MarketContext>, Error<()>> {
        let url = format!(
            "{}/api/sdk/context/{}", self.baseurl, encode_path(& market_id.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(2usize);
        header_map
            .append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(Self::api_version()),
            );
        header_map.append("Authorization", authorization.to_string().try_into()?);
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
            operation_id: "get_market_context",
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
    /**Place a buy or sell on a Kalshi market via Simmer

Sends a `POST` request to `/api/sdk/trade`

Arguments:
- `authorization`: `Bearer sk_…` — the Simmer API key.
- `body`
*/
    pub async fn trade<'a>(
        &'a self,
        authorization: &'a str,
        body: &'a types::TradeRequest,
    ) -> Result<ResponseValue<types::TradeResult>, Error<()>> {
        let url = format!("{}/api/sdk/trade", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(2usize);
        header_map
            .append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(Self::api_version()),
            );
        header_map.append("Authorization", authorization.to_string().try_into()?);
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
            operation_id: "trade",
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
    /**Open positions on a venue

Sends a `GET` request to `/api/sdk/positions`

Arguments:
- `venue`: `sim` (sandbox) or `kalshi` (live).
- `authorization`: `Bearer sk_…` — the Simmer API key.
*/
    pub async fn get_positions<'a>(
        &'a self,
        venue: Option<&'a str>,
        authorization: &'a str,
    ) -> Result<ResponseValue<types::PositionsResponse>, Error<()>> {
        let url = format!("{}/api/sdk/positions", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(2usize);
        header_map
            .append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(Self::api_version()),
            );
        header_map.append("Authorization", authorization.to_string().try_into()?);
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new("venue", &venue))
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
    /**Cash balance, positions value, total value, realized/unrealized PnL

Sends a `GET` request to `/api/sdk/portfolio`

Arguments:
- `authorization`: `Bearer sk_…` — the Simmer API key.
*/
    pub async fn get_portfolio<'a>(
        &'a self,
        authorization: &'a str,
    ) -> Result<ResponseValue<types::Portfolio>, Error<()>> {
        let url = format!("{}/api/sdk/portfolio", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(2usize);
        header_map
            .append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(Self::api_version()),
            );
        header_map.append("Authorization", authorization.to_string().try_into()?);
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
            operation_id: "get_portfolio",
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
    /**Importable Kalshi markets (Kalshi URLs, not Simmer UUIDs)

Sends a `GET` request to `/api/sdk/markets/importable`

Arguments:
- `limit`
- `min_volume`
- `q`: Free-text search query.
- `venue`: Always `kalshi` for this app.
- `authorization`: `Bearer sk_…` — the Simmer API key.
*/
    pub async fn list_importable_kalshi_markets<'a>(
        &'a self,
        limit: Option<&'a str>,
        min_volume: Option<&'a str>,
        q: Option<&'a str>,
        venue: &'a str,
        authorization: &'a str,
    ) -> Result<ResponseValue<types::ImportableMarketsResponse>, Error<()>> {
        let url = format!("{}/api/sdk/markets/importable", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(2usize);
        header_map
            .append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(Self::api_version()),
            );
        header_map.append("Authorization", authorization.to_string().try_into()?);
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new("limit", &limit))
            .query(&progenitor_client::QueryParam::new("min_volume", &min_volume))
            .query(&progenitor_client::QueryParam::new("q", &q))
            .query(&progenitor_client::QueryParam::new("venue", &venue))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "list_importable_kalshi_markets",
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
    /**Register a Kalshi market URL with Simmer; returns the Simmer market_id UUID

Sends a `POST` request to `/api/sdk/markets/import/kalshi`

Arguments:
- `authorization`: `Bearer sk_…` — the Simmer API key.
- `body`
*/
    pub async fn import_kalshi_market<'a>(
        &'a self,
        authorization: &'a str,
        body: &'a types::ImportKalshiMarketRequest,
    ) -> Result<ResponseValue<types::ImportedMarket>, Error<()>> {
        let url = format!("{}/api/sdk/markets/import/kalshi", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(2usize);
        header_map
            .append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(Self::api_version()),
            );
        header_map.append("Authorization", authorization.to_string().try_into()?);
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
            operation_id: "import_kalshi_market",
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
