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
    ///`MatviewsMatviewListElement`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "id": {
    ///      "type": "string"
    ///    },
    ///    "is_private": {
    ///      "type": "boolean"
    ///    },
    ///    "query_id": {
    ///      "type": "integer"
    ///    },
    ///    "sql_id": {
    ///      "type": "string"
    ///    },
    ///    "table_size_bytes": {
    ///      "type": "integer"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct MatviewsMatviewListElement {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub id: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub is_private: ::std::option::Option<bool>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub query_id: ::std::option::Option<i64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub sql_id: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub table_size_bytes: ::std::option::Option<i64>,
    }
    impl ::std::default::Default for MatviewsMatviewListElement {
        fn default() -> Self {
            Self {
                id: Default::default(),
                is_private: Default::default(),
                query_id: Default::default(),
                sql_id: Default::default(),
                table_size_bytes: Default::default(),
            }
        }
    }
    ///`MatviewsMatviewsDeleteResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "message": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct MatviewsMatviewsDeleteResponse {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub message: ::std::option::Option<::std::string::String>,
    }
    impl ::std::default::Default for MatviewsMatviewsDeleteResponse {
        fn default() -> Self {
            Self {
                message: Default::default(),
            }
        }
    }
    ///`MatviewsMatviewsGetResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "id": {
    ///      "type": "string"
    ///    },
    ///    "is_private": {
    ///      "type": "boolean"
    ///    },
    ///    "last_execution_ids": {
    ///      "type": "array",
    ///      "items": {
    ///        "type": "string"
    ///      }
    ///    },
    ///    "owner_team_id": {
    ///      "type": "integer"
    ///    },
    ///    "owner_user_id": {
    ///      "type": "integer"
    ///    },
    ///    "query_id": {
    ///      "type": "integer"
    ///    },
    ///    "sql_id": {
    ///      "type": "string"
    ///    },
    ///    "table_size_bytes": {
    ///      "type": "integer"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct MatviewsMatviewsGetResponse {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub id: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub is_private: ::std::option::Option<bool>,
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub last_execution_ids: ::std::vec::Vec<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub owner_team_id: ::std::option::Option<i64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub owner_user_id: ::std::option::Option<i64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub query_id: ::std::option::Option<i64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub sql_id: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub table_size_bytes: ::std::option::Option<i64>,
    }
    impl ::std::default::Default for MatviewsMatviewsGetResponse {
        fn default() -> Self {
            Self {
                id: Default::default(),
                is_private: Default::default(),
                last_execution_ids: Default::default(),
                owner_team_id: Default::default(),
                owner_user_id: Default::default(),
                query_id: Default::default(),
                sql_id: Default::default(),
                table_size_bytes: Default::default(),
            }
        }
    }
    ///`MatviewsMatviewsListResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "materialized_views": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/matviews.MatviewListElement"
    ///      }
    ///    },
    ///    "next_offset": {
    ///      "type": "integer"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct MatviewsMatviewsListResponse {
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub materialized_views: ::std::vec::Vec<MatviewsMatviewListElement>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub next_offset: ::std::option::Option<i64>,
    }
    impl ::std::default::Default for MatviewsMatviewsListResponse {
        fn default() -> Self {
            Self {
                materialized_views: Default::default(),
                next_offset: Default::default(),
            }
        }
    }
    ///`MatviewsMatviewsRefreshRequest`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "performance": {
    ///      "description": "Performance tier for the refresh execution. Accepts `small`, `medium`, `large`.\nOmit to use the default tier for the source query's engine.",
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct MatviewsMatviewsRefreshRequest {
        /**Performance tier for the refresh execution. Accepts `small`, `medium`, `large`.
Omit to use the default tier for the source query's engine.*/
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub performance: ::std::option::Option<::std::string::String>,
    }
    impl ::std::default::Default for MatviewsMatviewsRefreshRequest {
        fn default() -> Self {
            Self {
                performance: Default::default(),
            }
        }
    }
    ///`MatviewsMatviewsRefreshResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "execution_id": {
    ///      "description": "Unique identifier for the execution triggered to refresh the materialized view",
    ///      "examples": [
    ///        "01HZ065JVE23C23FM2HKWQP2RT"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "sql_id": {
    ///      "description": "Unique identifier for the materialized view",
    ///      "examples": [
    ///        "dune.dune.result_erc_20_token_summary"
    ///      ],
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct MatviewsMatviewsRefreshResponse {
        ///Unique identifier for the execution triggered to refresh the materialized view
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub execution_id: ::std::option::Option<::std::string::String>,
        ///Unique identifier for the materialized view
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub sql_id: ::std::option::Option<::std::string::String>,
    }
    impl ::std::default::Default for MatviewsMatviewsRefreshResponse {
        fn default() -> Self {
            Self {
                execution_id: Default::default(),
                sql_id: Default::default(),
            }
        }
    }
    ///`MatviewsMatviewsUpsertRequest`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "cron_expression": {
    ///      "type": "string"
    ///    },
    ///    "expires_at": {
    ///      "type": "string"
    ///    },
    ///    "is_private": {
    ///      "type": "boolean"
    ///    },
    ///    "name": {
    ///      "type": "string"
    ///    },
    ///    "performance": {
    ///      "description": "Performance tier for the refresh execution. Accepts `small`, `medium`, `large`.\nOmit to use the default tier for the source query's engine.",
    ///      "type": "string"
    ///    },
    ///    "query_id": {
    ///      "type": "integer"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct MatviewsMatviewsUpsertRequest {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub cron_expression: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub expires_at: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub is_private: ::std::option::Option<bool>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub name: ::std::option::Option<::std::string::String>,
        /**Performance tier for the refresh execution. Accepts `small`, `medium`, `large`.
Omit to use the default tier for the source query's engine.*/
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub performance: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub query_id: ::std::option::Option<i64>,
    }
    impl ::std::default::Default for MatviewsMatviewsUpsertRequest {
        fn default() -> Self {
            Self {
                cron_expression: Default::default(),
                expires_at: Default::default(),
                is_private: Default::default(),
                name: Default::default(),
                performance: Default::default(),
                query_id: Default::default(),
            }
        }
    }
    ///`MatviewsMatviewsUpsertResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "execution_id": {
    ///      "description": "Unique identifier for the execution triggered to refresh the materialized view",
    ///      "examples": [
    ///        "01HZ065JVE23C23FM2HKWQP2RT"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "name": {
    ///      "description": "Unique identifier for the materialized view",
    ///      "examples": [
    ///        "dune.dune.result_erc_20_token_summary"
    ///      ],
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct MatviewsMatviewsUpsertResponse {
        ///Unique identifier for the execution triggered to refresh the materialized view
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub execution_id: ::std::option::Option<::std::string::String>,
        ///Unique identifier for the materialized view
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub name: ::std::option::Option<::std::string::String>,
    }
    impl ::std::default::Default for MatviewsMatviewsUpsertResponse {
        fn default() -> Self {
            Self {
                execution_id: Default::default(),
                name: Default::default(),
            }
        }
    }
    ///`ModelsArchiveDashboardCrudResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "dashboard_id": {
    ///      "type": "integer"
    ///    },
    ///    "ok": {
    ///      "type": "boolean"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ModelsArchiveDashboardCrudResponse {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub dashboard_id: ::std::option::Option<i64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub ok: ::std::option::Option<bool>,
    }
    impl ::std::default::Default for ModelsArchiveDashboardCrudResponse {
        fn default() -> Self {
            Self {
                dashboard_id: Default::default(),
                ok: Default::default(),
            }
        }
    }
    ///`ModelsBillingPeriod`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "credits_included": {
    ///      "description": "Float value",
    ///      "type": "number"
    ///    },
    ///    "credits_used": {
    ///      "description": "Float value",
    ///      "type": "number"
    ///    },
    ///    "end_date": {
    ///      "description": "YYYY-MM-DD format",
    ///      "type": "string"
    ///    },
    ///    "start_date": {
    ///      "description": "YYYY-MM-DD format",
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ModelsBillingPeriod {
        ///Float value
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub credits_included: ::std::option::Option<f64>,
        ///Float value
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub credits_used: ::std::option::Option<f64>,
        ///YYYY-MM-DD format
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub end_date: ::std::option::Option<::std::string::String>,
        ///YYYY-MM-DD format
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub start_date: ::std::option::Option<::std::string::String>,
    }
    impl ::std::default::Default for ModelsBillingPeriod {
        fn default() -> Self {
            Self {
                credits_included: Default::default(),
                credits_used: Default::default(),
                end_date: Default::default(),
                start_date: Default::default(),
            }
        }
    }
    ///`ModelsCancelQueryExecutionResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "success": {
    ///      "type": "boolean"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ModelsCancelQueryExecutionResponse {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub success: ::std::option::Option<bool>,
    }
    impl ::std::default::Default for ModelsCancelQueryExecutionResponse {
        fn default() -> Self {
            Self {
                success: Default::default(),
            }
        }
    }
    ///`ModelsCreateDashboardCrudRequest`
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
    ///    "columns_per_row": {
    ///      "type": "integer"
    ///    },
    ///    "is_private": {
    ///      "type": "boolean"
    ///    },
    ///    "name": {
    ///      "type": "string"
    ///    },
    ///    "text_widgets": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/models.TextWidgetInput"
    ///      }
    ///    },
    ///    "visualization_ids": {
    ///      "type": "array",
    ///      "items": {
    ///        "type": "integer"
    ///      }
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ModelsCreateDashboardCrudRequest {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub columns_per_row: ::std::option::Option<i64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub is_private: ::std::option::Option<bool>,
        pub name: ::std::string::String,
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub text_widgets: ::std::vec::Vec<ModelsTextWidgetInput>,
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub visualization_ids: ::std::vec::Vec<i64>,
    }
    ///`ModelsCreateQueryRequest`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "name",
    ///    "query_sql"
    ///  ],
    ///  "properties": {
    ///    "description": {
    ///      "description": "The description of the created query.",
    ///      "type": "string"
    ///    },
    ///    "is_private": {
    ///      "description": "Indicates if the query is private, meaning that only the\nteam or, in case of personal queries, the user that created it can see it.",
    ///      "examples": [
    ///        true
    ///      ],
    ///      "type": "boolean"
    ///    },
    ///    "is_temp": {
    ///      "description": "Indicates if the query is temporary (unsaved).\nTemporary queries can be executed but won't appear in the library.",
    ///      "examples": [
    ///        false
    ///      ],
    ///      "type": "boolean"
    ///    },
    ///    "name": {
    ///      "description": "The name of the created query.",
    ///      "type": "string"
    ///    },
    ///    "parameters": {
    ///      "description": "The parameters that the SQL query accepts.",
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/models.Parameter"
    ///      }
    ///    },
    ///    "query_sql": {
    ///      "description": "The SQL of the query.",
    ///      "type": "string"
    ///    },
    ///    "tags": {
    ///      "description": "The tags of the query.",
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
    pub struct ModelsCreateQueryRequest {
        ///The description of the created query.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub description: ::std::option::Option<::std::string::String>,
        /**Indicates if the query is private, meaning that only the
team or, in case of personal queries, the user that created it can see it.*/
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub is_private: ::std::option::Option<bool>,
        /**Indicates if the query is temporary (unsaved).
Temporary queries can be executed but won't appear in the library.*/
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub is_temp: ::std::option::Option<bool>,
        ///The name of the created query.
        pub name: ::std::string::String,
        ///The parameters that the SQL query accepts.
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub parameters: ::std::vec::Vec<ModelsParameter>,
        ///The SQL of the query.
        pub query_sql: ::std::string::String,
        ///The tags of the query.
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub tags: ::std::vec::Vec<::std::string::String>,
    }
    ///`ModelsCreateQueryResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "query_id": {
    ///      "description": "The Unique ID of the created query",
    ///      "examples": [
    ///        1
    ///      ],
    ///      "type": "integer"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ModelsCreateQueryResponse {
        ///The Unique ID of the created query
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub query_id: ::std::option::Option<i64>,
    }
    impl ::std::default::Default for ModelsCreateQueryResponse {
        fn default() -> Self {
            Self {
                query_id: Default::default(),
            }
        }
    }
    ///`ModelsCreateVisualizationRequest`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "name",
    ///    "type"
    ///  ],
    ///  "properties": {
    ///    "description": {
    ///      "type": "string"
    ///    },
    ///    "name": {
    ///      "type": "string"
    ///    },
    ///    "options": {
    ///      "type": "object",
    ///      "additionalProperties": {}
    ///    },
    ///    "type": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ModelsCreateVisualizationRequest {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub description: ::std::option::Option<::std::string::String>,
        pub name: ::std::string::String,
        #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
        pub options: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        #[serde(rename = "type")]
        pub type_: ::std::string::String,
    }
    ///`ModelsCreateVisualizationResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "id": {
    ///      "type": "integer"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ModelsCreateVisualizationResponse {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub id: ::std::option::Option<i64>,
    }
    impl ::std::default::Default for ModelsCreateVisualizationResponse {
        fn default() -> Self {
            Self { id: Default::default() }
        }
    }
    ///`ModelsCsvUploadRequest`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "data": {
    ///      "description": "The data to be uploaded in CSV format.",
    ///      "type": "string"
    ///    },
    ///    "description": {
    ///      "description": "Description of the upload.",
    ///      "type": "string"
    ///    },
    ///    "is_private": {
    ///      "description": "Indicates if the upload is private.",
    ///      "type": "boolean"
    ///    },
    ///    "table_name": {
    ///      "description": "The name of the table to store the data.",
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ModelsCsvUploadRequest {
        ///The data to be uploaded in CSV format.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub data: ::std::option::Option<::std::string::String>,
        ///Description of the upload.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub description: ::std::option::Option<::std::string::String>,
        ///Indicates if the upload is private.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub is_private: ::std::option::Option<bool>,
        ///The name of the table to store the data.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub table_name: ::std::option::Option<::std::string::String>,
    }
    impl ::std::default::Default for ModelsCsvUploadRequest {
        fn default() -> Self {
            Self {
                data: Default::default(),
                description: Default::default(),
                is_private: Default::default(),
                table_name: Default::default(),
            }
        }
    }
    ///`ModelsCsvUploadResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "full_name": {
    ///      "description": "the full name of the table that was created",
    ///      "examples": [
    ///        "dune.my_team.dataset_ten_year_us_interest_rates"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "success": {
    ///      "description": "Indicator if the request was successful",
    ///      "type": "boolean"
    ///    },
    ///    "table_name": {
    ///      "description": "The name of the table that was created",
    ///      "examples": [
    ///        "ten_year_us_interest_rates"
    ///      ],
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ModelsCsvUploadResponse {
        ///the full name of the table that was created
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub full_name: ::std::option::Option<::std::string::String>,
        ///Indicator if the request was successful
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub success: ::std::option::Option<bool>,
        ///The name of the table that was created
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub table_name: ::std::option::Option<::std::string::String>,
    }
    impl ::std::default::Default for ModelsCsvUploadResponse {
        fn default() -> Self {
            Self {
                full_name: Default::default(),
                success: Default::default(),
                table_name: Default::default(),
            }
        }
    }
    ///`ModelsDashboardCrudResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "dashboard_id": {
    ///      "type": "integer"
    ///    },
    ///    "dashboard_url": {
    ///      "type": "string"
    ///    },
    ///    "is_private": {
    ///      "type": "boolean"
    ///    },
    ///    "name": {
    ///      "type": "string"
    ///    },
    ///    "param_widgets": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/models.ParamWidgetOutput"
    ///      }
    ///    },
    ///    "slug": {
    ///      "type": "string"
    ///    },
    ///    "tags": {
    ///      "type": "array",
    ///      "items": {
    ///        "type": "string"
    ///      }
    ///    },
    ///    "text_widgets": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/models.TextWidgetOutput"
    ///      }
    ///    },
    ///    "visualization_widgets": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/models.VisualizationWidgetOutput"
    ///      }
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ModelsDashboardCrudResponse {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub dashboard_id: ::std::option::Option<i64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub dashboard_url: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub is_private: ::std::option::Option<bool>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub name: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub param_widgets: ::std::vec::Vec<ModelsParamWidgetOutput>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub slug: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub tags: ::std::vec::Vec<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub text_widgets: ::std::vec::Vec<ModelsTextWidgetOutput>,
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub visualization_widgets: ::std::vec::Vec<ModelsVisualizationWidgetOutput>,
    }
    impl ::std::default::Default for ModelsDashboardCrudResponse {
        fn default() -> Self {
            Self {
                dashboard_id: Default::default(),
                dashboard_url: Default::default(),
                is_private: Default::default(),
                name: Default::default(),
                param_widgets: Default::default(),
                slug: Default::default(),
                tags: Default::default(),
                text_widgets: Default::default(),
                visualization_widgets: Default::default(),
            }
        }
    }
    ///`ModelsDatasetColumn`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "metadata": {
    ///      "$ref": "#/components/schemas/models.DatasetColumnMetadata"
    ///    },
    ///    "name": {
    ///      "type": "string"
    ///    },
    ///    "nullable": {
    ///      "type": "boolean"
    ///    },
    ///    "type": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ModelsDatasetColumn {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub metadata: ::std::option::Option<ModelsDatasetColumnMetadata>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub name: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub nullable: ::std::option::Option<bool>,
        #[serde(
            rename = "type",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub type_: ::std::option::Option<::std::string::String>,
    }
    impl ::std::default::Default for ModelsDatasetColumn {
        fn default() -> Self {
            Self {
                metadata: Default::default(),
                name: Default::default(),
                nullable: Default::default(),
                type_: Default::default(),
            }
        }
    }
    ///`ModelsDatasetColumnMetadata`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "description": {
    ///      "type": "string"
    ///    },
    ///    "filtering_column": {
    ///      "type": "boolean"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ModelsDatasetColumnMetadata {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub description: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub filtering_column: ::std::option::Option<bool>,
    }
    impl ::std::default::Default for ModelsDatasetColumnMetadata {
        fn default() -> Self {
            Self {
                description: Default::default(),
                filtering_column: Default::default(),
            }
        }
    }
    ///`ModelsDatasetOwner`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "handle": {
    ///      "type": "string"
    ///    },
    ///    "type": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ModelsDatasetOwner {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub handle: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "type",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub type_: ::std::option::Option<::std::string::String>,
    }
    impl ::std::default::Default for ModelsDatasetOwner {
        fn default() -> Self {
            Self {
                handle: Default::default(),
                type_: Default::default(),
            }
        }
    }
    ///`ModelsDatasetResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "columns": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/models.DatasetColumn"
    ///      }
    ///    },
    ///    "created_at": {
    ///      "type": "string"
    ///    },
    ///    "full_name": {
    ///      "type": "string"
    ///    },
    ///    "is_private": {
    ///      "type": "boolean"
    ///    },
    ///    "metadata": {},
    ///    "owner": {
    ///      "$ref": "#/components/schemas/models.DatasetOwner"
    ///    },
    ///    "type": {
    ///      "type": "string"
    ///    },
    ///    "updated_at": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ModelsDatasetResponse {
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub columns: ::std::vec::Vec<ModelsDatasetColumn>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub created_at: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub full_name: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub is_private: ::std::option::Option<bool>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub metadata: ::std::option::Option<::serde_json::Value>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub owner: ::std::option::Option<ModelsDatasetOwner>,
        #[serde(
            rename = "type",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub type_: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub updated_at: ::std::option::Option<::std::string::String>,
    }
    impl ::std::default::Default for ModelsDatasetResponse {
        fn default() -> Self {
            Self {
                columns: Default::default(),
                created_at: Default::default(),
                full_name: Default::default(),
                is_private: Default::default(),
                metadata: Default::default(),
                owner: Default::default(),
                type_: Default::default(),
                updated_at: Default::default(),
            }
        }
    }
    ///`ModelsDeleteVisualizationResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "ok": {
    ///      "type": "boolean"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ModelsDeleteVisualizationResponse {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub ok: ::std::option::Option<bool>,
    }
    impl ::std::default::Default for ModelsDeleteVisualizationResponse {
        fn default() -> Self {
            Self { ok: Default::default() }
        }
    }
    ///`ModelsEnumFromResults`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "columnName": {
    ///      "type": "string"
    ///    },
    ///    "queryId": {
    ///      "type": "integer"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ModelsEnumFromResults {
        #[serde(
            rename = "columnName",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub column_name: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "queryId",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub query_id: ::std::option::Option<i64>,
    }
    impl ::std::default::Default for ModelsEnumFromResults {
        fn default() -> Self {
            Self {
                column_name: Default::default(),
                query_id: Default::default(),
            }
        }
    }
    ///`ModelsError400`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "error": {
    ///      "examples": [
    ///        "Bad Request"
    ///      ],
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ModelsError400 {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub error: ::std::option::Option<::std::string::String>,
    }
    impl ::std::default::Default for ModelsError400 {
        fn default() -> Self {
            Self { error: Default::default() }
        }
    }
    ///`ModelsError401`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "error": {
    ///      "examples": [
    ///        "Invalid API Key"
    ///      ],
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ModelsError401 {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub error: ::std::option::Option<::std::string::String>,
    }
    impl ::std::default::Default for ModelsError401 {
        fn default() -> Self {
            Self { error: Default::default() }
        }
    }
    ///`ModelsError402`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "error": {
    ///      "examples": [
    ///        "This API request would exceed your configured limits per billing cycle."
    ///      ],
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ModelsError402 {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub error: ::std::option::Option<::std::string::String>,
    }
    impl ::std::default::Default for ModelsError402 {
        fn default() -> Self {
            Self { error: Default::default() }
        }
    }
    ///`ModelsError403`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "error": {
    ///      "examples": [
    ///        "Not allowed to execute query. Query is archived, unsaved or not enough permissions"
    ///      ],
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ModelsError403 {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub error: ::std::option::Option<::std::string::String>,
    }
    impl ::std::default::Default for ModelsError403 {
        fn default() -> Self {
            Self { error: Default::default() }
        }
    }
    ///`ModelsError404`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "error": {
    ///      "examples": [
    ///        "Object not found"
    ///      ],
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ModelsError404 {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub error: ::std::option::Option<::std::string::String>,
    }
    impl ::std::default::Default for ModelsError404 {
        fn default() -> Self {
            Self { error: Default::default() }
        }
    }
    ///`ModelsError500`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "error": {
    ///      "examples": [
    ///        "Internal error"
    ///      ],
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ModelsError500 {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub error: ::std::option::Option<::std::string::String>,
    }
    impl ::std::default::Default for ModelsError500 {
        fn default() -> Self {
            Self { error: Default::default() }
        }
    }
    ///`ModelsExecutePipelineRequest`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "pipeline": {
    ///      "description": "The pipeline definition containing nodes to execute",
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/models.Pipeline"
    ///        }
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ModelsExecutePipelineRequest {
        ///The pipeline definition containing nodes to execute
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub pipeline: ::std::option::Option<ModelsPipeline>,
    }
    impl ::std::default::Default for ModelsExecutePipelineRequest {
        fn default() -> Self {
            Self {
                pipeline: Default::default(),
            }
        }
    }
    ///`ModelsExecutePipelineResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "pipeline_execution_id": {
    ///      "description": "Unique identifier for the pipeline execution. Use this ID to check the status\nand retrieve results of the pipeline execution.",
    ///      "examples": [
    ///        "01HKZJ2683PHF9Q9PHHQ8FW4Q1"
    ///      ],
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ModelsExecutePipelineResponse {
        /**Unique identifier for the pipeline execution. Use this ID to check the status
and retrieve results of the pipeline execution.*/
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub pipeline_execution_id: ::std::option::Option<::std::string::String>,
    }
    impl ::std::default::Default for ModelsExecutePipelineResponse {
        fn default() -> Self {
            Self {
                pipeline_execution_id: Default::default(),
            }
        }
    }
    ///`ModelsExecuteQueryPipelineRequest`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "performance"
    ///  ],
    ///  "properties": {
    ///    "performance": {
    ///      "type": "string",
    ///      "enum": [
    ///        "small",
    ///        "medium",
    ///        "large"
    ///      ]
    ///    },
    ///    "query_parameters": {
    ///      "description": "SQL Query parameters in json key-value pairs. Each parameter is to be provided in key-value pairs. This enables you to execute a parameterized query with the provided values for your parameter keys.",
    ///      "type": "object"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ModelsExecuteQueryPipelineRequest {
        pub performance: ModelsExecuteQueryPipelineRequestPerformance,
        ///SQL Query parameters in json key-value pairs. Each parameter is to be provided in key-value pairs. This enables you to execute a parameterized query with the provided values for your parameter keys.
        #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
        pub query_parameters: ::serde_json::Map<
            ::std::string::String,
            ::serde_json::Value,
        >,
    }
    ///`ModelsExecuteQueryPipelineRequestPerformance`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "small",
    ///    "medium",
    ///    "large"
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
    pub enum ModelsExecuteQueryPipelineRequestPerformance {
        #[serde(rename = "small")]
        Small,
        #[serde(rename = "medium")]
        Medium,
        #[serde(rename = "large")]
        Large,
    }
    impl ::std::fmt::Display for ModelsExecuteQueryPipelineRequestPerformance {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Small => f.write_str("small"),
                Self::Medium => f.write_str("medium"),
                Self::Large => f.write_str("large"),
            }
        }
    }
    impl ::std::str::FromStr for ModelsExecuteQueryPipelineRequestPerformance {
        type Err = self::error::ConversionError;
        fn from_str(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "small" => Ok(Self::Small),
                "medium" => Ok(Self::Medium),
                "large" => Ok(Self::Large),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for ModelsExecuteQueryPipelineRequestPerformance {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String>
    for ModelsExecuteQueryPipelineRequestPerformance {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String>
    for ModelsExecuteQueryPipelineRequestPerformance {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///`ModelsExecuteQueryPipelineResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "pipeline_execution_id": {
    ///      "description": "Unique identifier for the pipeline execution. Use this ID to check the status\nand retrieve results of the pipeline execution.",
    ///      "examples": [
    ///        "01HKZJ2683PHF9Q9PHHQ8FW4Q1"
    ///      ],
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ModelsExecuteQueryPipelineResponse {
        /**Unique identifier for the pipeline execution. Use this ID to check the status
and retrieve results of the pipeline execution.*/
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub pipeline_execution_id: ::std::option::Option<::std::string::String>,
    }
    impl ::std::default::Default for ModelsExecuteQueryPipelineResponse {
        fn default() -> Self {
            Self {
                pipeline_execution_id: Default::default(),
            }
        }
    }
    ///`ModelsExecuteQueryResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "execution_id": {
    ///      "examples": [
    ///        "01HKZJ2683PHF9Q9PHHQ8FW4Q1"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "state": {
    ///      "examples": [
    ///        "QUERY_STATE_PENDING"
    ///      ],
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ModelsExecuteQueryResponse {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub execution_id: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub state: ::std::option::Option<::std::string::String>,
    }
    impl ::std::default::Default for ModelsExecuteQueryResponse {
        fn default() -> Self {
            Self {
                execution_id: Default::default(),
                state: Default::default(),
            }
        }
    }
    ///`ModelsExecuteSqlRequest`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "sql"
    ///  ],
    ///  "properties": {
    ///    "performance": {
    ///      "description": "The performance engine tier the execution will be run on",
    ///      "type": "string",
    ///      "enum": [
    ///        "small",
    ///        "medium",
    ///        "large"
    ///      ]
    ///    },
    ///    "sql": {
    ///      "description": "The SQL query to execute",
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ModelsExecuteSqlRequest {
        ///The performance engine tier the execution will be run on
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub performance: ::std::option::Option<ModelsExecuteSqlRequestPerformance>,
        ///The SQL query to execute
        pub sql: ::std::string::String,
    }
    ///The performance engine tier the execution will be run on
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "The performance engine tier the execution will be run on",
    ///  "type": "string",
    ///  "enum": [
    ///    "small",
    ///    "medium",
    ///    "large"
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
    pub enum ModelsExecuteSqlRequestPerformance {
        #[serde(rename = "small")]
        Small,
        #[serde(rename = "medium")]
        Medium,
        #[serde(rename = "large")]
        Large,
    }
    impl ::std::fmt::Display for ModelsExecuteSqlRequestPerformance {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Small => f.write_str("small"),
                Self::Medium => f.write_str("medium"),
                Self::Large => f.write_str("large"),
            }
        }
    }
    impl ::std::str::FromStr for ModelsExecuteSqlRequestPerformance {
        type Err = self::error::ConversionError;
        fn from_str(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "small" => Ok(Self::Small),
                "medium" => Ok(Self::Medium),
                "large" => Ok(Self::Large),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for ModelsExecuteSqlRequestPerformance {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String>
    for ModelsExecuteSqlRequestPerformance {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String>
    for ModelsExecuteSqlRequestPerformance {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///`ModelsExecutionResultMetadata`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "column_names": {
    ///      "description": "Names of the columns in the result set.",
    ///      "examples": [
    ///        [
    ///          "Rank",
    ///          "Project",
    ///          "Volume"
    ///        ]
    ///      ],
    ///      "type": "array",
    ///      "items": {
    ///        "type": "string"
    ///      }
    ///    },
    ///    "column_types": {
    ///      "description": "Types of the columns in the result set.",
    ///      "examples": [
    ///        [
    ///          "double",
    ///          "varchar",
    ///          "bigint"
    ///        ]
    ///      ],
    ///      "type": "array",
    ///      "items": {
    ///        "type": "string"
    ///      }
    ///    },
    ///    "datapoint_count": {
    ///      "description": "Results cell count is used for billing/pricing plans\nhere we expose the these values to the user, so that they can track their costs",
    ///      "examples": [
    ///        1000
    ///      ],
    ///      "type": "integer"
    ///    },
    ///    "execution_time_millis": {
    ///      "description": "Time in milliseconds that the query took to execute.",
    ///      "examples": [
    ///        1000
    ///      ],
    ///      "type": "integer"
    ///    },
    ///    "pending_time_millis": {
    ///      "description": "Time in milliseconds that the query was pending before execution.",
    ///      "examples": [
    ///        1000
    ///      ],
    ///      "type": "integer"
    ///    },
    ///    "result_set_bytes": {
    ///      "description": "ResultSetBytes represents the raw data bytes returned by the SQL execution engine, it includes:\n + total nr of bytes used on 1 line with all the column names (the header of the result set)\n + total nr of bytes for all the row values (the result set of rows)\n\nit doesn't include overheads such as the presence of column names for every row in the JSON result type.\nit also doesn't include opmitizations such as compression",
    ///      "examples": [
    ///        1000
    ///      ],
    ///      "type": "integer"
    ///    },
    ///    "row_count": {
    ///      "description": "Number of rows in the result set for the current page of results.",
    ///      "examples": [
    ///        10
    ///      ],
    ///      "type": "integer"
    ///    },
    ///    "total_result_set_bytes": {
    ///      "description": "Total number of bytes in the result set. This doesn't include the json representation overhead.",
    ///      "examples": [
    ///        10000
    ///      ],
    ///      "type": "integer"
    ///    },
    ///    "total_row_count": {
    ///      "description": "Number of rows in the result set for the entire result set.",
    ///      "examples": [
    ///        1000
    ///      ],
    ///      "type": "integer"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ModelsExecutionResultMetadata {
        ///Names of the columns in the result set.
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub column_names: ::std::vec::Vec<::std::string::String>,
        ///Types of the columns in the result set.
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub column_types: ::std::vec::Vec<::std::string::String>,
        /**Results cell count is used for billing/pricing plans
here we expose the these values to the user, so that they can track their costs*/
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub datapoint_count: ::std::option::Option<i64>,
        ///Time in milliseconds that the query took to execute.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub execution_time_millis: ::std::option::Option<i64>,
        ///Time in milliseconds that the query was pending before execution.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub pending_time_millis: ::std::option::Option<i64>,
        /**ResultSetBytes represents the raw data bytes returned by the SQL execution engine, it includes:
 + total nr of bytes used on 1 line with all the column names (the header of the result set)
 + total nr of bytes for all the row values (the result set of rows)

it doesn't include overheads such as the presence of column names for every row in the JSON result type.
it also doesn't include opmitizations such as compression*/
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub result_set_bytes: ::std::option::Option<i64>,
        ///Number of rows in the result set for the current page of results.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub row_count: ::std::option::Option<i64>,
        ///Total number of bytes in the result set. This doesn't include the json representation overhead.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub total_result_set_bytes: ::std::option::Option<i64>,
        ///Number of rows in the result set for the entire result set.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub total_row_count: ::std::option::Option<i64>,
    }
    impl ::std::default::Default for ModelsExecutionResultMetadata {
        fn default() -> Self {
            Self {
                column_names: Default::default(),
                column_types: Default::default(),
                datapoint_count: Default::default(),
                execution_time_millis: Default::default(),
                pending_time_millis: Default::default(),
                result_set_bytes: Default::default(),
                row_count: Default::default(),
                total_result_set_bytes: Default::default(),
                total_row_count: Default::default(),
            }
        }
    }
    ///`ModelsGetExecutionStatusResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "cancelled_at": {
    ///      "description": "Timestamp of when the query execution was cancelled, if applicable.",
    ///      "examples": [
    ///        "2024-12-20T11:04:18.724658237Z"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "error": {
    ///      "description": "In case the execution had an error, this object will contain the error details",
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/models.QueryResultError"
    ///        }
    ///      ]
    ///    },
    ///    "execution_cost_credits": {
    ///      "description": "Cost of the execution",
    ///      "type": "number"
    ///    },
    ///    "execution_ended_at": {
    ///      "description": "Timestamp of when the query execution ended.",
    ///      "examples": [
    ///        "2024-12-20T11:04:18.724658237Z"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "execution_id": {
    ///      "description": "Unique identifier for the execution of the query and corresponding result.",
    ///      "examples": [
    ///        "01HKZJ2683PHF9Q9PHHQ8FW4Q1"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "execution_started_at": {
    ///      "description": "Timestamp of when the query execution started.",
    ///      "examples": [
    ///        "2024-12-20T11:04:18.724658237Z"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "expires_at": {
    ///      "description": "Timestamp of when the query result expires.",
    ///      "examples": [
    ///        "2024-12-20T11:04:18.724658237Z"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "is_execution_finished": {
    ///      "description": "Whether the state of the query execution is terminal. This can be used for polling purposes.",
    ///      "examples": [
    ///        true
    ///      ],
    ///      "type": "boolean"
    ///    },
    ///    "max_inflight_interactive_executions": {
    ///      "description": "Number of interactive executions this customer can have running in parallel",
    ///      "examples": [
    ///        3
    ///      ],
    ///      "type": "integer"
    ///    },
    ///    "max_inflight_interactive_reached": {
    ///      "description": "Total number of interactive executions this user has submitted which are still in progress\nonly set to > 0 if the user has reached the limit of concurrent interactive executions",
    ///      "examples": [
    ///        5
    ///      ],
    ///      "type": "integer"
    ///    },
    ///    "query_id": {
    ///      "description": "Unique identifier of the query.",
    ///      "examples": [
    ///        1234
    ///      ],
    ///      "type": "integer"
    ///    },
    ///    "queue_position": {
    ///      "examples": [
    ///        1
    ///      ],
    ///      "type": "integer"
    ///    },
    ///    "result_metadata": {
    ///      "description": "Metadata about the execution of the query, including details like column names, row counts, and execution times.",
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/models.ExecutionResultMetadata"
    ///        }
    ///      ]
    ///    },
    ///    "state": {
    ///      "description": "The state of the query execution.",
    ///      "examples": [
    ///        "QUERY_STATE_COMPLETED"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "submitted_at": {
    ///      "description": "Timestamp of when the query was submitted.",
    ///      "examples": [
    ///        "2024-12-20T11:04:18.724658237Z"
    ///      ],
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ModelsGetExecutionStatusResponse {
        ///Timestamp of when the query execution was cancelled, if applicable.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub cancelled_at: ::std::option::Option<::std::string::String>,
        ///In case the execution had an error, this object will contain the error details
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub error: ::std::option::Option<ModelsQueryResultError>,
        ///Cost of the execution
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub execution_cost_credits: ::std::option::Option<f64>,
        ///Timestamp of when the query execution ended.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub execution_ended_at: ::std::option::Option<::std::string::String>,
        ///Unique identifier for the execution of the query and corresponding result.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub execution_id: ::std::option::Option<::std::string::String>,
        ///Timestamp of when the query execution started.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub execution_started_at: ::std::option::Option<::std::string::String>,
        ///Timestamp of when the query result expires.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub expires_at: ::std::option::Option<::std::string::String>,
        ///Whether the state of the query execution is terminal. This can be used for polling purposes.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub is_execution_finished: ::std::option::Option<bool>,
        ///Number of interactive executions this customer can have running in parallel
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub max_inflight_interactive_executions: ::std::option::Option<i64>,
        /**Total number of interactive executions this user has submitted which are still in progress
only set to > 0 if the user has reached the limit of concurrent interactive executions*/
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub max_inflight_interactive_reached: ::std::option::Option<i64>,
        ///Unique identifier of the query.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub query_id: ::std::option::Option<i64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub queue_position: ::std::option::Option<i64>,
        ///Metadata about the execution of the query, including details like column names, row counts, and execution times.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub result_metadata: ::std::option::Option<ModelsExecutionResultMetadata>,
        ///The state of the query execution.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub state: ::std::option::Option<::std::string::String>,
        ///Timestamp of when the query was submitted.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub submitted_at: ::std::option::Option<::std::string::String>,
    }
    impl ::std::default::Default for ModelsGetExecutionStatusResponse {
        fn default() -> Self {
            Self {
                cancelled_at: Default::default(),
                error: Default::default(),
                execution_cost_credits: Default::default(),
                execution_ended_at: Default::default(),
                execution_id: Default::default(),
                execution_started_at: Default::default(),
                expires_at: Default::default(),
                is_execution_finished: Default::default(),
                max_inflight_interactive_executions: Default::default(),
                max_inflight_interactive_reached: Default::default(),
                query_id: Default::default(),
                queue_position: Default::default(),
                result_metadata: Default::default(),
                state: Default::default(),
                submitted_at: Default::default(),
            }
        }
    }
    ///`ModelsGetPipelineExecutionStatusResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "error": {
    ///      "description": "Error message if the pipeline execution failed",
    ///      "type": "string"
    ///    },
    ///    "node_executions": {
    ///      "description": "List of node executions in the pipeline.",
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/models.PipelineNodeExecution"
    ///      }
    ///    },
    ///    "status": {
    ///      "description": "Overall status of the pipeline execution",
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ModelsGetPipelineExecutionStatusResponse {
        ///Error message if the pipeline execution failed
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub error: ::std::option::Option<::std::string::String>,
        ///List of node executions in the pipeline.
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub node_executions: ::std::vec::Vec<ModelsPipelineNodeExecution>,
        ///Overall status of the pipeline execution
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub status: ::std::option::Option<::std::string::String>,
    }
    impl ::std::default::Default for ModelsGetPipelineExecutionStatusResponse {
        fn default() -> Self {
            Self {
                error: Default::default(),
                node_executions: Default::default(),
                status: Default::default(),
            }
        }
    }
    ///`ModelsGetQueryPipelineResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "pipeline": {
    ///      "description": "The pipeline definition containing nodes to execute",
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/models.Pipeline"
    ///        }
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ModelsGetQueryPipelineResponse {
        ///The pipeline definition containing nodes to execute
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub pipeline: ::std::option::Option<ModelsPipeline>,
    }
    impl ::std::default::Default for ModelsGetQueryPipelineResponse {
        fn default() -> Self {
            Self {
                pipeline: Default::default(),
            }
        }
    }
    ///`ModelsGetQueryResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "contributors": {
    ///      "description": "Contributors to the query (users who have created or saved it).\nOnly included when include_contributors=true query parameter is set.",
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/models.QueryContributor"
    ///      }
    ///    },
    ///    "description": {
    ///      "description": "The description of the query",
    ///      "examples": [
    ///        "Calculate the average dex volume"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "is_archived": {
    ///      "description": "Indicates if the query is archived.\nArchived queries cannot be executed.",
    ///      "examples": [
    ///        false
    ///      ],
    ///      "type": "boolean"
    ///    },
    ///    "is_private": {
    ///      "description": "Indicates if the query is private.",
    ///      "examples": [
    ///        true
    ///      ],
    ///      "type": "boolean"
    ///    },
    ///    "is_temp": {
    ///      "description": "Indicates if the query is temporary (unsaved).",
    ///      "examples": [
    ///        false
    ///      ],
    ///      "type": "boolean"
    ///    },
    ///    "is_unsaved": {
    ///      "description": "Indicates if the query is unsaved (legacy name).",
    ///      "examples": [
    ///        false
    ///      ],
    ///      "type": "boolean"
    ///    },
    ///    "name": {
    ///      "description": "The name of the query",
    ///      "examples": [
    ///        "My Query"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "owner": {
    ///      "description": "username or team handle",
    ///      "examples": [
    ///        "dune"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "parameters": {
    ///      "description": "The parameters that can modify the execution of the sql.",
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/models.Parameter"
    ///      }
    ///    },
    ///    "query_engine": {
    ///      "description": "The query engine used to execute the query.",
    ///      "examples": [
    ///        "medium"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "query_id": {
    ///      "description": "The unique ID of the query",
    ///      "examples": [
    ///        123
    ///      ],
    ///      "type": "integer"
    ///    },
    ///    "query_sql": {
    ///      "description": "The SQL query text.",
    ///      "examples": [
    ///        "SELECT * FROM dex.trades"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "tags": {
    ///      "description": "Tags associated with the query.",
    ///      "type": "array",
    ///      "items": {
    ///        "type": "string"
    ///      }
    ///    },
    ///    "version": {
    ///      "description": "Version of the query, gets incremented every time the query is updated.",
    ///      "examples": [
    ///        1
    ///      ],
    ///      "type": "integer"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ModelsGetQueryResponse {
        /**Contributors to the query (users who have created or saved it).
Only included when include_contributors=true query parameter is set.*/
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub contributors: ::std::vec::Vec<ModelsQueryContributor>,
        ///The description of the query
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub description: ::std::option::Option<::std::string::String>,
        /**Indicates if the query is archived.
Archived queries cannot be executed.*/
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub is_archived: ::std::option::Option<bool>,
        ///Indicates if the query is private.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub is_private: ::std::option::Option<bool>,
        ///Indicates if the query is temporary (unsaved).
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub is_temp: ::std::option::Option<bool>,
        ///Indicates if the query is unsaved (legacy name).
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub is_unsaved: ::std::option::Option<bool>,
        ///The name of the query
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub name: ::std::option::Option<::std::string::String>,
        ///username or team handle
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub owner: ::std::option::Option<::std::string::String>,
        ///The parameters that can modify the execution of the sql.
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub parameters: ::std::vec::Vec<ModelsParameter>,
        ///The query engine used to execute the query.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub query_engine: ::std::option::Option<::std::string::String>,
        ///The unique ID of the query
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub query_id: ::std::option::Option<i64>,
        ///The SQL query text.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub query_sql: ::std::option::Option<::std::string::String>,
        ///Tags associated with the query.
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub tags: ::std::vec::Vec<::std::string::String>,
        ///Version of the query, gets incremented every time the query is updated.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub version: ::std::option::Option<i64>,
    }
    impl ::std::default::Default for ModelsGetQueryResponse {
        fn default() -> Self {
            Self {
                contributors: Default::default(),
                description: Default::default(),
                is_archived: Default::default(),
                is_private: Default::default(),
                is_temp: Default::default(),
                is_unsaved: Default::default(),
                name: Default::default(),
                owner: Default::default(),
                parameters: Default::default(),
                query_engine: Default::default(),
                query_id: Default::default(),
                query_sql: Default::default(),
                tags: Default::default(),
                version: Default::default(),
            }
        }
    }
    ///`ModelsGetUsageRequest`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "end_date": {
    ///      "description": "Optional field in YYYY-MM-DD format",
    ///      "type": "string"
    ///    },
    ///    "start_date": {
    ///      "description": "Optional field in YYYY-MM-DD format",
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ModelsGetUsageRequest {
        ///Optional field in YYYY-MM-DD format
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub end_date: ::std::option::Option<::std::string::String>,
        ///Optional field in YYYY-MM-DD format
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub start_date: ::std::option::Option<::std::string::String>,
    }
    impl ::std::default::Default for ModelsGetUsageRequest {
        fn default() -> Self {
            Self {
                end_date: Default::default(),
                start_date: Default::default(),
            }
        }
    }
    ///`ModelsGetUsageResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "billing_periods": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/models.BillingPeriod"
    ///      }
    ///    },
    ///    "bytes_allowed": {
    ///      "type": "integer"
    ///    },
    ///    "bytes_used": {
    ///      "type": "integer"
    ///    },
    ///    "private_dashboards": {
    ///      "type": "integer"
    ///    },
    ///    "private_queries": {
    ///      "type": "integer"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ModelsGetUsageResponse {
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub billing_periods: ::std::vec::Vec<ModelsBillingPeriod>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub bytes_allowed: ::std::option::Option<i64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub bytes_used: ::std::option::Option<i64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub private_dashboards: ::std::option::Option<i64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub private_queries: ::std::option::Option<i64>,
    }
    impl ::std::default::Default for ModelsGetUsageResponse {
        fn default() -> Self {
            Self {
                billing_periods: Default::default(),
                bytes_allowed: Default::default(),
                bytes_used: Default::default(),
                private_dashboards: Default::default(),
                private_queries: Default::default(),
            }
        }
    }
    ///`ModelsGetVisualizationResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "created_at": {
    ///      "type": "string"
    ///    },
    ///    "description": {
    ///      "type": "string"
    ///    },
    ///    "id": {
    ///      "type": "integer"
    ///    },
    ///    "name": {
    ///      "type": "string"
    ///    },
    ///    "options": {
    ///      "type": "object",
    ///      "additionalProperties": {}
    ///    },
    ///    "query_id": {
    ///      "type": "integer"
    ///    },
    ///    "type": {
    ///      "type": "string"
    ///    },
    ///    "updated_at": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ModelsGetVisualizationResponse {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub created_at: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub description: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub id: ::std::option::Option<i64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub name: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
        pub options: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub query_id: ::std::option::Option<i64>,
        #[serde(
            rename = "type",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub type_: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub updated_at: ::std::option::Option<::std::string::String>,
    }
    impl ::std::default::Default for ModelsGetVisualizationResponse {
        fn default() -> Self {
            Self {
                created_at: Default::default(),
                description: Default::default(),
                id: Default::default(),
                name: Default::default(),
                options: Default::default(),
                query_id: Default::default(),
                type_: Default::default(),
                updated_at: Default::default(),
            }
        }
    }
    ///`ModelsListDatasetsResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "datasets": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/models.DatasetResponse"
    ///      }
    ///    },
    ///    "total": {
    ///      "type": "integer"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ModelsListDatasetsResponse {
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub datasets: ::std::vec::Vec<ModelsDatasetResponse>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub total: ::std::option::Option<i64>,
    }
    impl ::std::default::Default for ModelsListDatasetsResponse {
        fn default() -> Self {
            Self {
                datasets: Default::default(),
                total: Default::default(),
            }
        }
    }
    ///`ModelsListQueriesResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "queries": {
    ///      "description": "List of queries",
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/models.ResolvedQueryOverview"
    ///      }
    ///    },
    ///    "total": {
    ///      "description": "Total number of queries available",
    ///      "examples": [
    ///        100
    ///      ],
    ///      "type": "integer"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ModelsListQueriesResponse {
        ///List of queries
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub queries: ::std::vec::Vec<ModelsResolvedQueryOverview>,
        ///Total number of queries available
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub total: ::std::option::Option<i64>,
    }
    impl ::std::default::Default for ModelsListQueriesResponse {
        fn default() -> Self {
            Self {
                queries: Default::default(),
                total: Default::default(),
            }
        }
    }
    ///`ModelsListVisualizationsResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "results": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/models.VisualizationSummary"
    ///      }
    ///    },
    ///    "total_count": {
    ///      "type": "integer"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ModelsListVisualizationsResponse {
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub results: ::std::vec::Vec<ModelsVisualizationSummary>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub total_count: ::std::option::Option<i64>,
    }
    impl ::std::default::Default for ModelsListVisualizationsResponse {
        fn default() -> Self {
            Self {
                results: Default::default(),
                total_count: Default::default(),
            }
        }
    }
    ///`ModelsParamWidgetInput`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "key": {
    ///      "type": "string"
    ///    },
    ///    "position": {
    ///      "$ref": "#/components/schemas/models.WidgetPosition"
    ///    },
    ///    "query_id": {
    ///      "type": "integer"
    ///    },
    ///    "visualization_widget_id": {
    ///      "type": "integer"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ModelsParamWidgetInput {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub key: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub position: ::std::option::Option<ModelsWidgetPosition>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub query_id: ::std::option::Option<i64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub visualization_widget_id: ::std::option::Option<i64>,
    }
    impl ::std::default::Default for ModelsParamWidgetInput {
        fn default() -> Self {
            Self {
                key: Default::default(),
                position: Default::default(),
                query_id: Default::default(),
                visualization_widget_id: Default::default(),
            }
        }
    }
    ///`ModelsParamWidgetOutput`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "key": {
    ///      "type": "string"
    ///    },
    ///    "position": {
    ///      "$ref": "#/components/schemas/models.WidgetPosition"
    ///    },
    ///    "query_id": {
    ///      "type": "integer"
    ///    },
    ///    "visualization_widget_id": {
    ///      "type": "integer"
    ///    },
    ///    "widget_id": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ModelsParamWidgetOutput {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub key: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub position: ::std::option::Option<ModelsWidgetPosition>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub query_id: ::std::option::Option<i64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub visualization_widget_id: ::std::option::Option<i64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub widget_id: ::std::option::Option<::std::string::String>,
    }
    impl ::std::default::Default for ModelsParamWidgetOutput {
        fn default() -> Self {
            Self {
                key: Default::default(),
                position: Default::default(),
                query_id: Default::default(),
                visualization_widget_id: Default::default(),
                widget_id: Default::default(),
            }
        }
    }
    ///`ModelsParameter`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "description": {
    ///      "type": "string"
    ///    },
    ///    "enumFromResults": {
    ///      "$ref": "#/components/schemas/models.EnumFromResults"
    ///    },
    ///    "enumOptions": {
    ///      "type": "array",
    ///      "items": {
    ///        "type": "string"
    ///      }
    ///    },
    ///    "isFreeformAllowed": {
    ///      "type": "boolean"
    ///    },
    ///    "isMultiselect": {
    ///      "type": "boolean"
    ///    },
    ///    "key": {
    ///      "type": "string"
    ///    },
    ///    "type": {
    ///      "type": "string"
    ///    },
    ///    "value": {
    ///      "type": "string"
    ///    },
    ///    "values": {
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
    pub struct ModelsParameter {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub description: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "enumFromResults",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub enum_from_results: ::std::option::Option<ModelsEnumFromResults>,
        #[serde(
            rename = "enumOptions",
            default,
            skip_serializing_if = "::std::vec::Vec::is_empty"
        )]
        pub enum_options: ::std::vec::Vec<::std::string::String>,
        #[serde(
            rename = "isFreeformAllowed",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub is_freeform_allowed: ::std::option::Option<bool>,
        #[serde(
            rename = "isMultiselect",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub is_multiselect: ::std::option::Option<bool>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub key: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "type",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub type_: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub value: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub values: ::std::vec::Vec<::std::string::String>,
    }
    impl ::std::default::Default for ModelsParameter {
        fn default() -> Self {
            Self {
                description: Default::default(),
                enum_from_results: Default::default(),
                enum_options: Default::default(),
                is_freeform_allowed: Default::default(),
                is_multiselect: Default::default(),
                key: Default::default(),
                type_: Default::default(),
                value: Default::default(),
                values: Default::default(),
            }
        }
    }
    ///`ModelsPipeline`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "nodes": {
    ///      "description": "List of nodes in the pipeline. Nodes are executed in dependency order.",
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/models.PipelineNode"
    ///      }
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ModelsPipeline {
        ///List of nodes in the pipeline. Nodes are executed in dependency order.
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub nodes: ::std::vec::Vec<ModelsPipelineNode>,
    }
    impl ::std::default::Default for ModelsPipeline {
        fn default() -> Self {
            Self { nodes: Default::default() }
        }
    }
    ///`ModelsPipelineMaterializedViewRefreshNode`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "name": {
    ///      "description": "Name of the materialized view to refresh",
    ///      "examples": [
    ///        "mv_1"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "performance": {
    ///      "description": "The performance engine tier the refresh will be run on. Can be `small`, `medium`, or `large`.",
    ///      "examples": [
    ///        "medium"
    ///      ],
    ///      "type": "string",
    ///      "enum": [
    ///        "small",
    ///        "medium",
    ///        "large"
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ModelsPipelineMaterializedViewRefreshNode {
        ///Name of the materialized view to refresh
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub name: ::std::option::Option<::std::string::String>,
        ///The performance engine tier the refresh will be run on. Can be `small`, `medium`, or `large`.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub performance: ::std::option::Option<
            ModelsPipelineMaterializedViewRefreshNodePerformance,
        >,
    }
    impl ::std::default::Default for ModelsPipelineMaterializedViewRefreshNode {
        fn default() -> Self {
            Self {
                name: Default::default(),
                performance: Default::default(),
            }
        }
    }
    ///The performance engine tier the refresh will be run on. Can be `small`, `medium`, or `large`.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "The performance engine tier the refresh will be run on. Can be `small`, `medium`, or `large`.",
    ///  "examples": [
    ///    "medium"
    ///  ],
    ///  "type": "string",
    ///  "enum": [
    ///    "small",
    ///    "medium",
    ///    "large"
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
    pub enum ModelsPipelineMaterializedViewRefreshNodePerformance {
        #[serde(rename = "small")]
        Small,
        #[serde(rename = "medium")]
        Medium,
        #[serde(rename = "large")]
        Large,
    }
    impl ::std::fmt::Display for ModelsPipelineMaterializedViewRefreshNodePerformance {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Small => f.write_str("small"),
                Self::Medium => f.write_str("medium"),
                Self::Large => f.write_str("large"),
            }
        }
    }
    impl ::std::str::FromStr for ModelsPipelineMaterializedViewRefreshNodePerformance {
        type Err = self::error::ConversionError;
        fn from_str(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "small" => Ok(Self::Small),
                "medium" => Ok(Self::Medium),
                "large" => Ok(Self::Large),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str>
    for ModelsPipelineMaterializedViewRefreshNodePerformance {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String>
    for ModelsPipelineMaterializedViewRefreshNodePerformance {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String>
    for ModelsPipelineMaterializedViewRefreshNodePerformance {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///`ModelsPipelineMaterializedViewRefreshStatus`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "error": {
    ///      "description": "Error message if the refresh failed",
    ///      "type": "string"
    ///    },
    ///    "execution_id": {
    ///      "description": "Unique identifier of the execution, if available",
    ///      "type": "string"
    ///    },
    ///    "name": {
    ///      "description": "Name of the materialized view",
    ///      "type": "string"
    ///    },
    ///    "status": {
    ///      "description": "Status of the materialized view refresh",
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ModelsPipelineMaterializedViewRefreshStatus {
        ///Error message if the refresh failed
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub error: ::std::option::Option<::std::string::String>,
        ///Unique identifier of the execution, if available
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub execution_id: ::std::option::Option<::std::string::String>,
        ///Name of the materialized view
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub name: ::std::option::Option<::std::string::String>,
        ///Status of the materialized view refresh
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub status: ::std::option::Option<::std::string::String>,
    }
    impl ::std::default::Default for ModelsPipelineMaterializedViewRefreshStatus {
        fn default() -> Self {
            Self {
                error: Default::default(),
                execution_id: Default::default(),
                name: Default::default(),
                status: Default::default(),
            }
        }
    }
    ///`ModelsPipelineNode`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "dependencies": {
    ///      "description": "List of node IDs that this node depends on. Nodes are executed in dependency order.",
    ///      "type": "array",
    ///      "items": {
    ///        "type": "integer"
    ///      }
    ///    },
    ///    "id": {
    ///      "description": "Unique identifier of the node in the pipeline",
    ///      "examples": [
    ///        1
    ///      ],
    ///      "type": "integer"
    ///    },
    ///    "materialized_view_refresh": {
    ///      "description": "Materialized view refresh node configuration. Required if this node refreshes a materialized view.",
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/models.PipelineMaterializedViewRefreshNode"
    ///        }
    ///      ]
    ///    },
    ///    "query_execution": {
    ///      "description": "Query execution node configuration. Required if this node executes a query.",
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/models.PipelineQueryExecutionNode"
    ///        }
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ModelsPipelineNode {
        ///List of node IDs that this node depends on. Nodes are executed in dependency order.
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub dependencies: ::std::vec::Vec<i64>,
        ///Unique identifier of the node in the pipeline
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub id: ::std::option::Option<i64>,
        ///Materialized view refresh node configuration. Required if this node refreshes a materialized view.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub materialized_view_refresh: ::std::option::Option<
            ModelsPipelineMaterializedViewRefreshNode,
        >,
        ///Query execution node configuration. Required if this node executes a query.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub query_execution: ::std::option::Option<ModelsPipelineQueryExecutionNode>,
    }
    impl ::std::default::Default for ModelsPipelineNode {
        fn default() -> Self {
            Self {
                dependencies: Default::default(),
                id: Default::default(),
                materialized_view_refresh: Default::default(),
                query_execution: Default::default(),
            }
        }
    }
    ///`ModelsPipelineNodeExecution`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "id": {
    ///      "description": "Unique identifier of the node in the pipeline",
    ///      "type": "integer"
    ///    },
    ///    "materialized_view_refresh_status": {
    ///      "description": "Status of the materialized view refresh, if this node is a materialized view refresh node",
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/models.PipelineMaterializedViewRefreshStatus"
    ///        }
    ///      ]
    ///    },
    ///    "query_execution_status": {
    ///      "description": "Status of the query execution, if this node is a query execution node",
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/models.PipelineQueryExecutionStatus"
    ///        }
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ModelsPipelineNodeExecution {
        ///Unique identifier of the node in the pipeline
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub id: ::std::option::Option<i64>,
        ///Status of the materialized view refresh, if this node is a materialized view refresh node
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub materialized_view_refresh_status: ::std::option::Option<
            ModelsPipelineMaterializedViewRefreshStatus,
        >,
        ///Status of the query execution, if this node is a query execution node
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub query_execution_status: ::std::option::Option<
            ModelsPipelineQueryExecutionStatus,
        >,
    }
    impl ::std::default::Default for ModelsPipelineNodeExecution {
        fn default() -> Self {
            Self {
                id: Default::default(),
                materialized_view_refresh_status: Default::default(),
                query_execution_status: Default::default(),
            }
        }
    }
    ///`ModelsPipelineQueryExecutionNode`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "performance": {
    ///      "description": "The performance engine tier the execution will be run on. Can be `small`, `medium`, or `large`.",
    ///      "examples": [
    ///        "medium"
    ///      ],
    ///      "type": "string",
    ///      "enum": [
    ///        "small",
    ///        "medium",
    ///        "large"
    ///      ]
    ///    },
    ///    "query_id": {
    ///      "description": "Unique identifier of the query to execute",
    ///      "examples": [
    ///        1234
    ///      ],
    ///      "type": "integer"
    ///    },
    ///    "query_parameters": {
    ///      "description": "SQL Query parameters in json key-value pairs. Each parameter is to be provided in key-value pairs. This enables you to execute a parameterized query with the provided values for your parameter keys.",
    ///      "type": "object"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ModelsPipelineQueryExecutionNode {
        ///The performance engine tier the execution will be run on. Can be `small`, `medium`, or `large`.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub performance: ::std::option::Option<
            ModelsPipelineQueryExecutionNodePerformance,
        >,
        ///Unique identifier of the query to execute
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub query_id: ::std::option::Option<i64>,
        ///SQL Query parameters in json key-value pairs. Each parameter is to be provided in key-value pairs. This enables you to execute a parameterized query with the provided values for your parameter keys.
        #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
        pub query_parameters: ::serde_json::Map<
            ::std::string::String,
            ::serde_json::Value,
        >,
    }
    impl ::std::default::Default for ModelsPipelineQueryExecutionNode {
        fn default() -> Self {
            Self {
                performance: Default::default(),
                query_id: Default::default(),
                query_parameters: Default::default(),
            }
        }
    }
    ///The performance engine tier the execution will be run on. Can be `small`, `medium`, or `large`.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "The performance engine tier the execution will be run on. Can be `small`, `medium`, or `large`.",
    ///  "examples": [
    ///    "medium"
    ///  ],
    ///  "type": "string",
    ///  "enum": [
    ///    "small",
    ///    "medium",
    ///    "large"
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
    pub enum ModelsPipelineQueryExecutionNodePerformance {
        #[serde(rename = "small")]
        Small,
        #[serde(rename = "medium")]
        Medium,
        #[serde(rename = "large")]
        Large,
    }
    impl ::std::fmt::Display for ModelsPipelineQueryExecutionNodePerformance {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Small => f.write_str("small"),
                Self::Medium => f.write_str("medium"),
                Self::Large => f.write_str("large"),
            }
        }
    }
    impl ::std::str::FromStr for ModelsPipelineQueryExecutionNodePerformance {
        type Err = self::error::ConversionError;
        fn from_str(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "small" => Ok(Self::Small),
                "medium" => Ok(Self::Medium),
                "large" => Ok(Self::Large),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for ModelsPipelineQueryExecutionNodePerformance {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String>
    for ModelsPipelineQueryExecutionNodePerformance {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String>
    for ModelsPipelineQueryExecutionNodePerformance {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///`ModelsPipelineQueryExecutionStatus`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "error": {
    ///      "description": "Error message, if the execution failed",
    ///      "type": "string"
    ///    },
    ///    "execution_id": {
    ///      "description": "Unique identifier of the execution, if available",
    ///      "type": "string"
    ///    },
    ///    "query_id": {
    ///      "description": "Unique identifier of the query",
    ///      "type": "integer"
    ///    },
    ///    "status": {
    ///      "description": "Status of the query execution",
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ModelsPipelineQueryExecutionStatus {
        ///Error message, if the execution failed
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub error: ::std::option::Option<::std::string::String>,
        ///Unique identifier of the execution, if available
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub execution_id: ::std::option::Option<::std::string::String>,
        ///Unique identifier of the query
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub query_id: ::std::option::Option<i64>,
        ///Status of the query execution
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub status: ::std::option::Option<::std::string::String>,
    }
    impl ::std::default::Default for ModelsPipelineQueryExecutionStatus {
        fn default() -> Self {
            Self {
                error: Default::default(),
                execution_id: Default::default(),
                query_id: Default::default(),
                status: Default::default(),
            }
        }
    }
    ///`ModelsQueryContributor`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "contribution_count": {
    ///      "description": "The number of contributions this user made to the query",
    ///      "examples": [
    ///        5
    ///      ],
    ///      "type": "integer"
    ///    },
    ///    "handle": {
    ///      "description": "The handle (username) of the contributor",
    ///      "examples": [
    ///        "wizardofdefi"
    ///      ],
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ModelsQueryContributor {
        ///The number of contributions this user made to the query
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub contribution_count: ::std::option::Option<i64>,
        ///The handle (username) of the contributor
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub handle: ::std::option::Option<::std::string::String>,
    }
    impl ::std::default::Default for ModelsQueryContributor {
        fn default() -> Self {
            Self {
                contribution_count: Default::default(),
                handle: Default::default(),
            }
        }
    }
    ///`ModelsQueryResultData`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "metadata": {
    ///      "description": "Metadata about the execution of the query, including details like column names,\nrow counts, and execution times.",
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/models.ExecutionResultMetadata"
    ///        }
    ///      ]
    ///    },
    ///    "rows": {
    ///      "description": "A list of rows. A row is dictionary of key-value pairs returned by the query,\neach pair corresponding to a column",
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/models.Row"
    ///      }
    ///    },
    ///    "update_type": {
    ///      "description": "The type of update operation from Trino (e.g., \"SET SESSION\")",
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ModelsQueryResultData {
        /**Metadata about the execution of the query, including details like column names,
row counts, and execution times.*/
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub metadata: ::std::option::Option<ModelsExecutionResultMetadata>,
        /**A list of rows. A row is dictionary of key-value pairs returned by the query,
each pair corresponding to a column*/
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub rows: ::std::vec::Vec<ModelsRow>,
        ///The type of update operation from Trino (e.g., "SET SESSION")
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub update_type: ::std::option::Option<::std::string::String>,
    }
    impl ::std::default::Default for ModelsQueryResultData {
        fn default() -> Self {
            Self {
                metadata: Default::default(),
                rows: Default::default(),
                update_type: Default::default(),
            }
        }
    }
    ///`ModelsQueryResultError`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "message": {
    ///      "description": "A descriptive message about the error.",
    ///      "examples": [
    ///        "Error: Line 1:1: mismatched input 'selecdt'"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "metadata": {
    ///      "description": "Metadata about the syntax error that occurred, if applicable.",
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/models.SyntaxErrorMetadata"
    ///        }
    ///      ]
    ///    },
    ///    "type": {
    ///      "description": "The type of error that occurred.",
    ///      "examples": [
    ///        "syntax_error"
    ///      ],
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ModelsQueryResultError {
        ///A descriptive message about the error.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub message: ::std::option::Option<::std::string::String>,
        ///Metadata about the syntax error that occurred, if applicable.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub metadata: ::std::option::Option<ModelsSyntaxErrorMetadata>,
        ///The type of error that occurred.
        #[serde(
            rename = "type",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub type_: ::std::option::Option<::std::string::String>,
    }
    impl ::std::default::Default for ModelsQueryResultError {
        fn default() -> Self {
            Self {
                message: Default::default(),
                metadata: Default::default(),
                type_: Default::default(),
            }
        }
    }
    ///`ModelsReadExecutionResultResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "cancelled_at": {
    ///      "description": "Timestamp of when the query execution was cancelled, if applicable.",
    ///      "examples": [
    ///        "2024-12-20T11:04:18.724658237Z"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "error": {
    ///      "description": "In case the execution had an error, this object will contain the error details",
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/models.QueryResultError"
    ///        }
    ///      ]
    ///    },
    ///    "execution_ended_at": {
    ///      "description": "Timestamp of when the query execution ended.",
    ///      "examples": [
    ///        "2024-12-20T11:04:18.724658237Z"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "execution_id": {
    ///      "description": "Unique identifier for the execution of the query.",
    ///      "examples": [
    ///        "01HKZJ2683PHF9Q9PHHQ8FW4Q1"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "execution_started_at": {
    ///      "description": "Timestamp of when the query execution started.",
    ///      "examples": [
    ///        "2024-12-20T11:04:18.724658237Z"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "expires_at": {
    ///      "description": "Timestamp of when the query result expires.",
    ///      "examples": [
    ///        "2024-12-20T11:04:18.724658237Z"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "is_execution_finished": {
    ///      "description": "Whether the state of the query execution is terminal. This can be used for polling purposes.",
    ///      "examples": [
    ///        true
    ///      ],
    ///      "type": "boolean"
    ///    },
    ///    "next_offset": {
    ///      "description": "Offset that can be used to retrieve the next page of results.",
    ///      "examples": [
    ///        100
    ///      ],
    ///      "type": "integer"
    ///    },
    ///    "next_uri": {
    ///      "description": "URI that can be used to fetch the next page of results.",
    ///      "examples": [
    ///        "https://api.dune.com/api/v1/execution/01HKZJ2683PHF9Q9PHHQ8FW4Q1/results?offset=100&limit=100"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "query_id": {
    ///      "description": "Unique identifier of the query.",
    ///      "examples": [
    ///        1234
    ///      ],
    ///      "type": "integer"
    ///    },
    ///    "result": {
    ///      "description": "The object containing the results and metadata of the query execution",
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/models.QueryResultData"
    ///        }
    ///      ]
    ///    },
    ///    "state": {
    ///      "description": "The state of the query execution.",
    ///      "examples": [
    ///        "QUERY_STATE_COMPLETED"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "submitted_at": {
    ///      "description": "Timestamp of when the query was submitted.",
    ///      "examples": [
    ///        "2024-12-20T11:04:18.724658237Z"
    ///      ],
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ModelsReadExecutionResultResponse {
        ///Timestamp of when the query execution was cancelled, if applicable.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub cancelled_at: ::std::option::Option<::std::string::String>,
        ///In case the execution had an error, this object will contain the error details
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub error: ::std::option::Option<ModelsQueryResultError>,
        ///Timestamp of when the query execution ended.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub execution_ended_at: ::std::option::Option<::std::string::String>,
        ///Unique identifier for the execution of the query.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub execution_id: ::std::option::Option<::std::string::String>,
        ///Timestamp of when the query execution started.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub execution_started_at: ::std::option::Option<::std::string::String>,
        ///Timestamp of when the query result expires.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub expires_at: ::std::option::Option<::std::string::String>,
        ///Whether the state of the query execution is terminal. This can be used for polling purposes.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub is_execution_finished: ::std::option::Option<bool>,
        ///Offset that can be used to retrieve the next page of results.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub next_offset: ::std::option::Option<i64>,
        ///URI that can be used to fetch the next page of results.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub next_uri: ::std::option::Option<::std::string::String>,
        ///Unique identifier of the query.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub query_id: ::std::option::Option<i64>,
        ///The object containing the results and metadata of the query execution
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub result: ::std::option::Option<ModelsQueryResultData>,
        ///The state of the query execution.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub state: ::std::option::Option<::std::string::String>,
        ///Timestamp of when the query was submitted.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub submitted_at: ::std::option::Option<::std::string::String>,
    }
    impl ::std::default::Default for ModelsReadExecutionResultResponse {
        fn default() -> Self {
            Self {
                cancelled_at: Default::default(),
                error: Default::default(),
                execution_ended_at: Default::default(),
                execution_id: Default::default(),
                execution_started_at: Default::default(),
                expires_at: Default::default(),
                is_execution_finished: Default::default(),
                next_offset: Default::default(),
                next_uri: Default::default(),
                query_id: Default::default(),
                result: Default::default(),
                state: Default::default(),
                submitted_at: Default::default(),
            }
        }
    }
    ///`ModelsResolvedQueryOverview`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "created_at": {
    ///      "type": "string"
    ///    },
    ///    "description": {
    ///      "description": "Description of the query",
    ///      "type": "string"
    ///    },
    ///    "id": {
    ///      "description": "Unique identifier of the query",
    ///      "type": "integer"
    ///    },
    ///    "name": {
    ///      "description": "Name of the query",
    ///      "examples": [
    ///        "My Query"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "owner": {
    ///      "description": "Owner handle (username or team handle)",
    ///      "type": "string"
    ///    },
    ///    "tags": {
    ///      "description": "Tags associated with the query",
    ///      "type": "array",
    ///      "items": {
    ///        "type": "string"
    ///      }
    ///    },
    ///    "updated_at": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ModelsResolvedQueryOverview {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub created_at: ::std::option::Option<::std::string::String>,
        ///Description of the query
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub description: ::std::option::Option<::std::string::String>,
        ///Unique identifier of the query
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub id: ::std::option::Option<i64>,
        ///Name of the query
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub name: ::std::option::Option<::std::string::String>,
        ///Owner handle (username or team handle)
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub owner: ::std::option::Option<::std::string::String>,
        ///Tags associated with the query
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub tags: ::std::vec::Vec<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub updated_at: ::std::option::Option<::std::string::String>,
    }
    impl ::std::default::Default for ModelsResolvedQueryOverview {
        fn default() -> Self {
            Self {
                created_at: Default::default(),
                description: Default::default(),
                id: Default::default(),
                name: Default::default(),
                owner: Default::default(),
                tags: Default::default(),
                updated_at: Default::default(),
            }
        }
    }
    ///`ModelsRow`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "additionalProperties": {}
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    #[serde(transparent)]
    pub struct ModelsRow(
        pub ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    );
    impl ::std::ops::Deref for ModelsRow {
        type Target = ::serde_json::Map<::std::string::String, ::serde_json::Value>;
        fn deref(
            &self,
        ) -> &::serde_json::Map<::std::string::String, ::serde_json::Value> {
            &self.0
        }
    }
    impl ::std::convert::From<ModelsRow>
    for ::serde_json::Map<::std::string::String, ::serde_json::Value> {
        fn from(value: ModelsRow) -> Self {
            value.0
        }
    }
    impl ::std::convert::From<
        ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    > for ModelsRow {
        fn from(
            value: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        ) -> Self {
            Self(value)
        }
    }
    ///`ModelsSearchDatasetMetadata`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "abi_type": {
    ///      "type": "string"
    ///    },
    ///    "contract_name": {
    ///      "type": "string"
    ///    },
    ///    "description": {
    ///      "type": "string"
    ///    },
    ///    "page_rank_score": {
    ///      "type": "number"
    ///    },
    ///    "project_name": {
    ///      "type": "string"
    ///    },
    ///    "spell_metadata": {
    ///      "type": "array",
    ///      "items": {
    ///        "type": "integer"
    ///      }
    ///    },
    ///    "spell_type": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ModelsSearchDatasetMetadata {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub abi_type: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub contract_name: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub description: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub page_rank_score: ::std::option::Option<f64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub project_name: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub spell_metadata: ::std::vec::Vec<i64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub spell_type: ::std::option::Option<::std::string::String>,
    }
    impl ::std::default::Default for ModelsSearchDatasetMetadata {
        fn default() -> Self {
            Self {
                abi_type: Default::default(),
                contract_name: Default::default(),
                description: Default::default(),
                page_rank_score: Default::default(),
                project_name: Default::default(),
                spell_metadata: Default::default(),
                spell_type: Default::default(),
            }
        }
    }
    ///`ModelsSearchDatasetResult`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "blockchains": {
    ///      "type": "array",
    ///      "items": {
    ///        "type": "string"
    ///      }
    ///    },
    ///    "category": {
    ///      "type": "string"
    ///    },
    ///    "dataset_type": {
    ///      "type": "string"
    ///    },
    ///    "description": {
    ///      "type": "string"
    ///    },
    ///    "full_name": {
    ///      "type": "string"
    ///    },
    ///    "metadata": {
    ///      "$ref": "#/components/schemas/models.SearchDatasetMetadata"
    ///    },
    ///    "owner_scope": {
    ///      "type": "string"
    ///    },
    ///    "schema": {
    ///      "type": "array",
    ///      "items": {
    ///        "type": "integer"
    ///      }
    ///    },
    ///    "visibility": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ModelsSearchDatasetResult {
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub blockchains: ::std::vec::Vec<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub category: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub dataset_type: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub description: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub full_name: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub metadata: ::std::option::Option<ModelsSearchDatasetMetadata>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub owner_scope: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub schema: ::std::vec::Vec<i64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub visibility: ::std::option::Option<::std::string::String>,
    }
    impl ::std::default::Default for ModelsSearchDatasetResult {
        fn default() -> Self {
            Self {
                blockchains: Default::default(),
                category: Default::default(),
                dataset_type: Default::default(),
                description: Default::default(),
                full_name: Default::default(),
                metadata: Default::default(),
                owner_scope: Default::default(),
                schema: Default::default(),
                visibility: Default::default(),
            }
        }
    }
    ///`ModelsSearchDatasetsByContractAddressRequest`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "contract_address"
    ///  ],
    ///  "properties": {
    ///    "blockchains": {
    ///      "type": "array",
    ///      "items": {
    ///        "type": "string"
    ///      }
    ///    },
    ///    "contract_address": {
    ///      "type": "string"
    ///    },
    ///    "include_schema": {
    ///      "type": "boolean"
    ///    },
    ///    "limit": {
    ///      "type": "integer"
    ///    },
    ///    "offset": {
    ///      "type": "integer"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ModelsSearchDatasetsByContractAddressRequest {
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub blockchains: ::std::vec::Vec<::std::string::String>,
        pub contract_address: ::std::string::String,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub include_schema: ::std::option::Option<bool>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub limit: ::std::option::Option<i64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub offset: ::std::option::Option<i64>,
    }
    ///`ModelsSearchDatasetsPagination`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "has_more": {
    ///      "type": "boolean"
    ///    },
    ///    "limit": {
    ///      "type": "integer"
    ///    },
    ///    "next_offset": {
    ///      "type": "integer"
    ///    },
    ///    "offset": {
    ///      "type": "integer"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ModelsSearchDatasetsPagination {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub has_more: ::std::option::Option<bool>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub limit: ::std::option::Option<i64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub next_offset: ::std::option::Option<i64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub offset: ::std::option::Option<i64>,
    }
    impl ::std::default::Default for ModelsSearchDatasetsPagination {
        fn default() -> Self {
            Self {
                has_more: Default::default(),
                limit: Default::default(),
                next_offset: Default::default(),
                offset: Default::default(),
            }
        }
    }
    ///`ModelsSearchDatasetsRequest`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "blockchains": {
    ///      "type": "array",
    ///      "items": {
    ///        "type": "string"
    ///      }
    ///    },
    ///    "categories": {
    ///      "type": "array",
    ///      "items": {
    ///        "type": "string"
    ///      }
    ///    },
    ///    "dataset_types": {
    ///      "type": "array",
    ///      "items": {
    ///        "type": "string"
    ///      }
    ///    },
    ///    "include_metadata": {
    ///      "type": "boolean"
    ///    },
    ///    "include_private": {
    ///      "type": "boolean"
    ///    },
    ///    "include_schema": {
    ///      "type": "boolean"
    ///    },
    ///    "limit": {
    ///      "type": "integer"
    ///    },
    ///    "offset": {
    ///      "type": "integer"
    ///    },
    ///    "owner_scope": {
    ///      "type": "string"
    ///    },
    ///    "query": {
    ///      "type": "string"
    ///    },
    ///    "schemas": {
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
    pub struct ModelsSearchDatasetsRequest {
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub blockchains: ::std::vec::Vec<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub categories: ::std::vec::Vec<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub dataset_types: ::std::vec::Vec<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub include_metadata: ::std::option::Option<bool>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub include_private: ::std::option::Option<bool>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub include_schema: ::std::option::Option<bool>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub limit: ::std::option::Option<i64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub offset: ::std::option::Option<i64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub owner_scope: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub query: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub schemas: ::std::vec::Vec<::std::string::String>,
    }
    impl ::std::default::Default for ModelsSearchDatasetsRequest {
        fn default() -> Self {
            Self {
                blockchains: Default::default(),
                categories: Default::default(),
                dataset_types: Default::default(),
                include_metadata: Default::default(),
                include_private: Default::default(),
                include_schema: Default::default(),
                limit: Default::default(),
                offset: Default::default(),
                owner_scope: Default::default(),
                query: Default::default(),
                schemas: Default::default(),
            }
        }
    }
    ///`ModelsSearchDatasetsResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "pagination": {
    ///      "$ref": "#/components/schemas/models.SearchDatasetsPagination"
    ///    },
    ///    "results": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/models.SearchDatasetResult"
    ///      }
    ///    },
    ///    "total": {
    ///      "type": "integer"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ModelsSearchDatasetsResponse {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub pagination: ::std::option::Option<ModelsSearchDatasetsPagination>,
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub results: ::std::vec::Vec<ModelsSearchDatasetResult>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub total: ::std::option::Option<i64>,
    }
    impl ::std::default::Default for ModelsSearchDatasetsResponse {
        fn default() -> Self {
            Self {
                pagination: Default::default(),
                results: Default::default(),
                total: Default::default(),
            }
        }
    }
    ///`ModelsSyntaxErrorMetadata`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "column": {
    ///      "description": "The column number at which the syntax error occurred.",
    ///      "examples": [
    ///        73
    ///      ],
    ///      "type": "integer"
    ///    },
    ///    "line": {
    ///      "description": "The line number at which the syntax error occurred in the query.",
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
    pub struct ModelsSyntaxErrorMetadata {
        ///The column number at which the syntax error occurred.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub column: ::std::option::Option<i64>,
        ///The line number at which the syntax error occurred in the query.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub line: ::std::option::Option<i64>,
    }
    impl ::std::default::Default for ModelsSyntaxErrorMetadata {
        fn default() -> Self {
            Self {
                column: Default::default(),
                line: Default::default(),
            }
        }
    }
    ///`ModelsTableClearResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "message": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ModelsTableClearResponse {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub message: ::std::option::Option<::std::string::String>,
    }
    impl ::std::default::Default for ModelsTableClearResponse {
        fn default() -> Self {
            Self {
                message: Default::default(),
            }
        }
    }
    ///`ModelsTableColumn`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "name": {
    ///      "type": "string"
    ///    },
    ///    "nullable": {
    ///      "type": "boolean"
    ///    },
    ///    "type": {
    ///      "description": "ColumnType is a json.RawMessage so that we can support objects like Array and Struct.",
    ///      "type": "array",
    ///      "items": {
    ///        "type": "integer"
    ///      }
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ModelsTableColumn {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub name: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub nullable: ::std::option::Option<bool>,
        ///ColumnType is a json.RawMessage so that we can support objects like Array and Struct.
        #[serde(
            rename = "type",
            default,
            skip_serializing_if = "::std::vec::Vec::is_empty"
        )]
        pub type_: ::std::vec::Vec<i64>,
    }
    impl ::std::default::Default for ModelsTableColumn {
        fn default() -> Self {
            Self {
                name: Default::default(),
                nullable: Default::default(),
                type_: Default::default(),
            }
        }
    }
    ///`ModelsTableColumnInfo`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "metadata": {
    ///      "description": "Additional column metadata",
    ///      "type": "object",
    ///      "additionalProperties": {}
    ///    },
    ///    "name": {
    ///      "description": "Column name",
    ///      "type": "string"
    ///    },
    ///    "nullable": {
    ///      "description": "Whether the column can contain null values",
    ///      "type": "boolean"
    ///    },
    ///    "type": {
    ///      "description": "Column data type",
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ModelsTableColumnInfo {
        ///Additional column metadata
        #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
        pub metadata: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        ///Column name
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub name: ::std::option::Option<::std::string::String>,
        ///Whether the column can contain null values
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub nullable: ::std::option::Option<bool>,
        ///Column data type
        #[serde(
            rename = "type",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub type_: ::std::option::Option<::std::string::String>,
    }
    impl ::std::default::Default for ModelsTableColumnInfo {
        fn default() -> Self {
            Self {
                metadata: Default::default(),
                name: Default::default(),
                nullable: Default::default(),
                type_: Default::default(),
            }
        }
    }
    ///`ModelsTableCreateRequest`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "description": {
    ///      "type": "string"
    ///    },
    ///    "is_private": {
    ///      "type": "boolean"
    ///    },
    ///    "namespace": {
    ///      "type": "string"
    ///    },
    ///    "schema": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/models.TableColumn"
    ///      }
    ///    },
    ///    "table_name": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ModelsTableCreateRequest {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub description: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub is_private: ::std::option::Option<bool>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub namespace: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub schema: ::std::vec::Vec<ModelsTableColumn>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub table_name: ::std::option::Option<::std::string::String>,
    }
    impl ::std::default::Default for ModelsTableCreateRequest {
        fn default() -> Self {
            Self {
                description: Default::default(),
                is_private: Default::default(),
                namespace: Default::default(),
                schema: Default::default(),
                table_name: Default::default(),
            }
        }
    }
    ///`ModelsTableCreateResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "already_existed": {
    ///      "type": "boolean"
    ///    },
    ///    "example_query": {
    ///      "type": "string"
    ///    },
    ///    "full_name": {
    ///      "type": "string"
    ///    },
    ///    "message": {
    ///      "type": "string"
    ///    },
    ///    "namespace": {
    ///      "type": "string"
    ///    },
    ///    "table_name": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ModelsTableCreateResponse {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub already_existed: ::std::option::Option<bool>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub example_query: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub full_name: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub message: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub namespace: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub table_name: ::std::option::Option<::std::string::String>,
    }
    impl ::std::default::Default for ModelsTableCreateResponse {
        fn default() -> Self {
            Self {
                already_existed: Default::default(),
                example_query: Default::default(),
                full_name: Default::default(),
                message: Default::default(),
                namespace: Default::default(),
                table_name: Default::default(),
            }
        }
    }
    ///`ModelsTableDeleteResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "message": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ModelsTableDeleteResponse {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub message: ::std::option::Option<::std::string::String>,
    }
    impl ::std::default::Default for ModelsTableDeleteResponse {
        fn default() -> Self {
            Self {
                message: Default::default(),
            }
        }
    }
    ///`ModelsTableInsertResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "bytes_written": {
    ///      "type": "integer"
    ///    },
    ///    "name": {
    ///      "type": "string"
    ///    },
    ///    "rows_written": {
    ///      "type": "integer"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ModelsTableInsertResponse {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub bytes_written: ::std::option::Option<i64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub name: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub rows_written: ::std::option::Option<i64>,
    }
    impl ::std::default::Default for ModelsTableInsertResponse {
        fn default() -> Self {
            Self {
                bytes_written: Default::default(),
                name: Default::default(),
                rows_written: Default::default(),
            }
        }
    }
    ///`ModelsTableListElement`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "columns": {
    ///      "description": "List of table columns",
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/models.TableColumnInfo"
    ///      }
    ///    },
    ///    "created_at": {
    ///      "description": "ISO 8601 timestamp of table creation",
    ///      "type": "string"
    ///    },
    ///    "full_name": {
    ///      "description": "Fully qualified table name (catalog.schema.table)",
    ///      "type": "string"
    ///    },
    ///    "is_private": {
    ///      "description": "Whether the table is private",
    ///      "type": "boolean"
    ///    },
    ///    "owner": {
    ///      "description": "Owner information",
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/models.TableOwner"
    ///        }
    ///      ]
    ///    },
    ///    "purged_at": {
    ///      "description": "ISO 8601 timestamp of when table was purged",
    ///      "type": "string"
    ///    },
    ///    "table_size_bytes": {
    ///      "description": "Size of the table in bytes",
    ///      "type": "string"
    ///    },
    ///    "updated_at": {
    ///      "description": "ISO 8601 timestamp of last update",
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ModelsTableListElement {
        ///List of table columns
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub columns: ::std::vec::Vec<ModelsTableColumnInfo>,
        ///ISO 8601 timestamp of table creation
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub created_at: ::std::option::Option<::std::string::String>,
        ///Fully qualified table name (catalog.schema.table)
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub full_name: ::std::option::Option<::std::string::String>,
        ///Whether the table is private
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub is_private: ::std::option::Option<bool>,
        ///Owner information
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub owner: ::std::option::Option<ModelsTableOwner>,
        ///ISO 8601 timestamp of when table was purged
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub purged_at: ::std::option::Option<::std::string::String>,
        ///Size of the table in bytes
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub table_size_bytes: ::std::option::Option<::std::string::String>,
        ///ISO 8601 timestamp of last update
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub updated_at: ::std::option::Option<::std::string::String>,
    }
    impl ::std::default::Default for ModelsTableListElement {
        fn default() -> Self {
            Self {
                columns: Default::default(),
                created_at: Default::default(),
                full_name: Default::default(),
                is_private: Default::default(),
                owner: Default::default(),
                purged_at: Default::default(),
                table_size_bytes: Default::default(),
                updated_at: Default::default(),
            }
        }
    }
    ///`ModelsTableListResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "next_offset": {
    ///      "description": "Offset for next page of results",
    ///      "type": "integer"
    ///    },
    ///    "tables": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/models.TableListElement"
    ///      }
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ModelsTableListResponse {
        ///Offset for next page of results
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub next_offset: ::std::option::Option<i64>,
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub tables: ::std::vec::Vec<ModelsTableListElement>,
    }
    impl ::std::default::Default for ModelsTableListResponse {
        fn default() -> Self {
            Self {
                next_offset: Default::default(),
                tables: Default::default(),
            }
        }
    }
    ///`ModelsTableOwner`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "handle": {
    ///      "description": "User or team identifier",
    ///      "type": "string"
    ///    },
    ///    "type": {
    ///      "description": "Type of entity that created the table",
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ModelsTableOwner {
        ///User or team identifier
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub handle: ::std::option::Option<::std::string::String>,
        ///Type of entity that created the table
        #[serde(
            rename = "type",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub type_: ::std::option::Option<::std::string::String>,
    }
    impl ::std::default::Default for ModelsTableOwner {
        fn default() -> Self {
            Self {
                handle: Default::default(),
                type_: Default::default(),
            }
        }
    }
    ///`ModelsTextWidgetInput`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "position": {
    ///      "$ref": "#/components/schemas/models.WidgetPosition"
    ///    },
    ///    "text": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ModelsTextWidgetInput {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub position: ::std::option::Option<ModelsWidgetPosition>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub text: ::std::option::Option<::std::string::String>,
    }
    impl ::std::default::Default for ModelsTextWidgetInput {
        fn default() -> Self {
            Self {
                position: Default::default(),
                text: Default::default(),
            }
        }
    }
    ///`ModelsTextWidgetOutput`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "position": {
    ///      "$ref": "#/components/schemas/models.WidgetPosition"
    ///    },
    ///    "text": {
    ///      "type": "string"
    ///    },
    ///    "widget_id": {
    ///      "type": "integer"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ModelsTextWidgetOutput {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub position: ::std::option::Option<ModelsWidgetPosition>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub text: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub widget_id: ::std::option::Option<i64>,
    }
    impl ::std::default::Default for ModelsTextWidgetOutput {
        fn default() -> Self {
            Self {
                position: Default::default(),
                text: Default::default(),
                widget_id: Default::default(),
            }
        }
    }
    ///`ModelsUpdateDashboardCrudRequest`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "columns_per_row": {
    ///      "type": "integer"
    ///    },
    ///    "is_private": {
    ///      "type": "boolean"
    ///    },
    ///    "name": {
    ///      "type": "string"
    ///    },
    ///    "param_widgets": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/models.ParamWidgetInput"
    ///      }
    ///    },
    ///    "slug": {
    ///      "type": "string"
    ///    },
    ///    "tags": {
    ///      "type": "array",
    ///      "items": {
    ///        "type": "string"
    ///      }
    ///    },
    ///    "text_widgets": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/models.TextWidgetInput"
    ///      }
    ///    },
    ///    "visualization_widgets": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/models.VisualizationWidgetInput"
    ///      }
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ModelsUpdateDashboardCrudRequest {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub columns_per_row: ::std::option::Option<i64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub is_private: ::std::option::Option<bool>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub name: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub param_widgets: ::std::vec::Vec<ModelsParamWidgetInput>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub slug: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub tags: ::std::vec::Vec<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub text_widgets: ::std::vec::Vec<ModelsTextWidgetInput>,
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub visualization_widgets: ::std::vec::Vec<ModelsVisualizationWidgetInput>,
    }
    impl ::std::default::Default for ModelsUpdateDashboardCrudRequest {
        fn default() -> Self {
            Self {
                columns_per_row: Default::default(),
                is_private: Default::default(),
                name: Default::default(),
                param_widgets: Default::default(),
                slug: Default::default(),
                tags: Default::default(),
                text_widgets: Default::default(),
                visualization_widgets: Default::default(),
            }
        }
    }
    ///`ModelsUpdateQueryResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "query_id": {
    ///      "description": "The unique ID of the query that was updated",
    ///      "type": "integer"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ModelsUpdateQueryResponse {
        ///The unique ID of the query that was updated
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub query_id: ::std::option::Option<i64>,
    }
    impl ::std::default::Default for ModelsUpdateQueryResponse {
        fn default() -> Self {
            Self {
                query_id: Default::default(),
            }
        }
    }
    ///`ModelsUpdateVisualizationRequest`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "name",
    ///    "type"
    ///  ],
    ///  "properties": {
    ///    "description": {
    ///      "type": "string"
    ///    },
    ///    "name": {
    ///      "type": "string"
    ///    },
    ///    "options": {
    ///      "type": "object",
    ///      "additionalProperties": {}
    ///    },
    ///    "type": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ModelsUpdateVisualizationRequest {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub description: ::std::option::Option<::std::string::String>,
        pub name: ::std::string::String,
        #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
        pub options: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        #[serde(rename = "type")]
        pub type_: ::std::string::String,
    }
    ///`ModelsUpdateVisualizationResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "id": {
    ///      "type": "integer"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ModelsUpdateVisualizationResponse {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub id: ::std::option::Option<i64>,
    }
    impl ::std::default::Default for ModelsUpdateVisualizationResponse {
        fn default() -> Self {
            Self { id: Default::default() }
        }
    }
    ///`ModelsVisualizationSummary`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "created_at": {
    ///      "type": "string"
    ///    },
    ///    "id": {
    ///      "type": "integer"
    ///    },
    ///    "name": {
    ///      "type": "string"
    ///    },
    ///    "type": {
    ///      "type": "string"
    ///    },
    ///    "updated_at": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ModelsVisualizationSummary {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub created_at: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub id: ::std::option::Option<i64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub name: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "type",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub type_: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub updated_at: ::std::option::Option<::std::string::String>,
    }
    impl ::std::default::Default for ModelsVisualizationSummary {
        fn default() -> Self {
            Self {
                created_at: Default::default(),
                id: Default::default(),
                name: Default::default(),
                type_: Default::default(),
                updated_at: Default::default(),
            }
        }
    }
    ///`ModelsVisualizationWidgetInput`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "position": {
    ///      "$ref": "#/components/schemas/models.WidgetPosition"
    ///    },
    ///    "visualization_id": {
    ///      "type": "integer"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ModelsVisualizationWidgetInput {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub position: ::std::option::Option<ModelsWidgetPosition>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub visualization_id: ::std::option::Option<i64>,
    }
    impl ::std::default::Default for ModelsVisualizationWidgetInput {
        fn default() -> Self {
            Self {
                position: Default::default(),
                visualization_id: Default::default(),
            }
        }
    }
    ///`ModelsVisualizationWidgetOutput`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "position": {
    ///      "$ref": "#/components/schemas/models.WidgetPosition"
    ///    },
    ///    "visualization_id": {
    ///      "type": "integer"
    ///    },
    ///    "widget_id": {
    ///      "type": "integer"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ModelsVisualizationWidgetOutput {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub position: ::std::option::Option<ModelsWidgetPosition>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub visualization_id: ::std::option::Option<i64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub widget_id: ::std::option::Option<i64>,
    }
    impl ::std::default::Default for ModelsVisualizationWidgetOutput {
        fn default() -> Self {
            Self {
                position: Default::default(),
                visualization_id: Default::default(),
                widget_id: Default::default(),
            }
        }
    }
    ///`ModelsWidgetPosition`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "col": {
    ///      "type": "integer"
    ///    },
    ///    "row": {
    ///      "type": "integer"
    ///    },
    ///    "size_x": {
    ///      "type": "integer"
    ///    },
    ///    "size_y": {
    ///      "type": "integer"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ModelsWidgetPosition {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub col: ::std::option::Option<i64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub row: ::std::option::Option<i64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub size_x: ::std::option::Option<i64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub size_y: ::std::option::Option<i64>,
    }
    impl ::std::default::Default for ModelsWidgetPosition {
        fn default() -> Self {
            Self {
                col: Default::default(),
                row: Default::default(),
                size_x: Default::default(),
                size_y: Default::default(),
            }
        }
    }
    ///`Postv1QueryQueryidExecutePerformance`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "small",
    ///    "medium",
    ///    "large"
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
    pub enum Postv1QueryQueryidExecutePerformance {
        #[serde(rename = "small")]
        Small,
        #[serde(rename = "medium")]
        Medium,
        #[serde(rename = "large")]
        Large,
    }
    impl ::std::fmt::Display for Postv1QueryQueryidExecutePerformance {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Small => f.write_str("small"),
                Self::Medium => f.write_str("medium"),
                Self::Large => f.write_str("large"),
            }
        }
    }
    impl ::std::str::FromStr for Postv1QueryQueryidExecutePerformance {
        type Err = self::error::ConversionError;
        fn from_str(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "small" => Ok(Self::Small),
                "medium" => Ok(Self::Medium),
                "large" => Ok(Self::Large),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for Postv1QueryQueryidExecutePerformance {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String>
    for Postv1QueryQueryidExecutePerformance {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String>
    for Postv1QueryQueryidExecutePerformance {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
}
#[derive(Clone, Debug)]
/**Client for DuneAPI

Dune API

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
    /**Create a dashboard

Creates a new dashboard with optional visualization and text widgets.

Sends a `POST` request to `/v1/dashboards`

Arguments:
- `x_dune_api_key`: API Key for the service
- `body`: CreateDashboardRequest
*/
    pub async fn postv1_dashboards<'a>(
        &'a self,
        x_dune_api_key: &'a str,
        body: &'a types::ModelsCreateDashboardCrudRequest,
    ) -> Result<ResponseValue<types::ModelsDashboardCrudResponse>, Error<()>> {
        let url = format!("{}/v1/dashboards", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(2usize);
        header_map
            .append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(Self::api_version()),
            );
        header_map.append("X-Dune-Api-Key", x_dune_api_key.to_string().try_into()?);
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
            operation_id: "postv1_dashboards",
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
    /**Get a dashboard by owner handle and slug

Retrieves the full state of a dashboard using the owner handle and URL slug.

Sends a `GET` request to `/v1/dashboards/by-slug/{owner_handle}/{slug}`

Arguments:
- `owner_handle`: Owner username or team handle
- `slug`: Dashboard URL slug
- `x_dune_api_key`: API Key for the service
*/
    pub async fn getv1_dashboards_by_slug_ownerhandle_slug<'a>(
        &'a self,
        owner_handle: &'a str,
        slug: &'a str,
        x_dune_api_key: &'a str,
    ) -> Result<ResponseValue<types::ModelsDashboardCrudResponse>, Error<()>> {
        let url = format!(
            "{}/v1/dashboards/by-slug/{}/{}", self.baseurl, encode_path(& owner_handle
            .to_string()), encode_path(& slug.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(2usize);
        header_map
            .append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(Self::api_version()),
            );
        header_map.append("X-Dune-Api-Key", x_dune_api_key.to_string().try_into()?);
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
            operation_id: "getv1_dashboards_by_slug_ownerhandle_slug",
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
    /**Get a dashboard by ID

Retrieves the full state of a dashboard including widgets.

Sends a `GET` request to `/v1/dashboards/{dashboard_id}`

Arguments:
- `dashboard_id`: Dashboard ID
- `x_dune_api_key`: API Key for the service
*/
    pub async fn getv1_dashboards_dashboardid<'a>(
        &'a self,
        dashboard_id: i64,
        x_dune_api_key: &'a str,
    ) -> Result<ResponseValue<types::ModelsDashboardCrudResponse>, Error<()>> {
        let url = format!(
            "{}/v1/dashboards/{}", self.baseurl, encode_path(& dashboard_id.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(2usize);
        header_map
            .append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(Self::api_version()),
            );
        header_map.append("X-Dune-Api-Key", x_dune_api_key.to_string().try_into()?);
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
            operation_id: "getv1_dashboards_dashboardid",
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
    /**Update a dashboard

Updates dashboard metadata and/or replaces widgets.

Sends a `PATCH` request to `/v1/dashboards/{dashboard_id}`

Arguments:
- `dashboard_id`: Dashboard ID
- `x_dune_api_key`: API Key for the service
- `body`: UpdateDashboardRequest
*/
    pub async fn patchv1_dashboards_dashboardid<'a>(
        &'a self,
        dashboard_id: i64,
        x_dune_api_key: &'a str,
        body: &'a types::ModelsUpdateDashboardCrudRequest,
    ) -> Result<ResponseValue<types::ModelsDashboardCrudResponse>, Error<()>> {
        let url = format!(
            "{}/v1/dashboards/{}", self.baseurl, encode_path(& dashboard_id.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(2usize);
        header_map
            .append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(Self::api_version()),
            );
        header_map.append("X-Dune-Api-Key", x_dune_api_key.to_string().try_into()?);
        #[allow(unused_mut)]
        let mut request = self
            .client
            .patch(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "patchv1_dashboards_dashboardid",
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
    /**Archive a dashboard

Archives a dashboard by ID. The dashboard can be restored later.

Sends a `POST` request to `/v1/dashboards/{dashboard_id}/archive`

Arguments:
- `dashboard_id`: Dashboard ID
- `x_dune_api_key`: API Key for the service
*/
    pub async fn postv1_dashboards_dashboardid_archive<'a>(
        &'a self,
        dashboard_id: i64,
        x_dune_api_key: &'a str,
    ) -> Result<ResponseValue<types::ModelsArchiveDashboardCrudResponse>, Error<()>> {
        let url = format!(
            "{}/v1/dashboards/{}/archive", self.baseurl, encode_path(& dashboard_id
            .to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(2usize);
        header_map
            .append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(Self::api_version()),
            );
        header_map.append("X-Dune-Api-Key", x_dune_api_key.to_string().try_into()?);
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
            operation_id: "postv1_dashboards_dashboardid_archive",
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
    /**List datasets

Retrieve a paginated list of datasets with optional filtering by owner and type

Sends a `GET` request to `/v1/datasets`

Arguments:
- `api_key`: API Key, alternative to using the HTTP header X-Dune-Api-Key
- `limit`: Number of results to return (default 50, max 250)
- `offset`: Offset for pagination
- `owner_handle`: Filter by owner handle
- `type_`: Filter by dataset types (comma-separated: transformation_view, transformation_table, uploaded_table, decoded_table, spell, dune_table)
- `x_dune_api_key`: API Key for the service
*/
    pub async fn getv1_datasets<'a>(
        &'a self,
        api_key: Option<&'a str>,
        limit: Option<i64>,
        offset: Option<i64>,
        owner_handle: Option<&'a str>,
        type_: Option<&'a str>,
        x_dune_api_key: &'a str,
    ) -> Result<ResponseValue<types::ModelsListDatasetsResponse>, Error<()>> {
        let url = format!("{}/v1/datasets", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(2usize);
        header_map
            .append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(Self::api_version()),
            );
        header_map.append("X-Dune-Api-Key", x_dune_api_key.to_string().try_into()?);
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new("api_key", &api_key))
            .query(&progenitor_client::QueryParam::new("limit", &limit))
            .query(&progenitor_client::QueryParam::new("offset", &offset))
            .query(&progenitor_client::QueryParam::new("owner_handle", &owner_handle))
            .query(&progenitor_client::QueryParam::new("type", &type_))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "getv1_datasets",
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
    /**Search datasets

Search for datasets across the Dune catalog, with filtering by categories, blockchains, schemas, and more

Sends a `POST` request to `/v1/datasets/search`

Arguments:
- `api_key`: API Key, alternative to using the HTTP header X-Dune-Api-Key
- `x_dune_api_key`: API Key for the service
- `body`: Search datasets request
*/
    pub async fn postv1_datasets_search<'a>(
        &'a self,
        api_key: Option<&'a str>,
        x_dune_api_key: &'a str,
        body: &'a types::ModelsSearchDatasetsRequest,
    ) -> Result<ResponseValue<types::ModelsSearchDatasetsResponse>, Error<()>> {
        let url = format!("{}/v1/datasets/search", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(2usize);
        header_map
            .append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(Self::api_version()),
            );
        header_map.append("X-Dune-Api-Key", x_dune_api_key.to_string().try_into()?);
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .query(&progenitor_client::QueryParam::new("api_key", &api_key))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "postv1_datasets_search",
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
    /**Search datasets by contract address

Find decoded datasets associated with a smart contract address

Sends a `POST` request to `/v1/datasets/search-by-contract`

Arguments:
- `api_key`: API Key, alternative to using the HTTP header X-Dune-Api-Key
- `x_dune_api_key`: API Key for the service
- `body`: Search by contract address request
*/
    pub async fn postv1_datasets_search_by_contract<'a>(
        &'a self,
        api_key: Option<&'a str>,
        x_dune_api_key: &'a str,
        body: &'a types::ModelsSearchDatasetsByContractAddressRequest,
    ) -> Result<ResponseValue<types::ModelsSearchDatasetsResponse>, Error<()>> {
        let url = format!("{}/v1/datasets/search-by-contract", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(2usize);
        header_map
            .append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(Self::api_version()),
            );
        header_map.append("X-Dune-Api-Key", x_dune_api_key.to_string().try_into()?);
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .query(&progenitor_client::QueryParam::new("api_key", &api_key))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "postv1_datasets_search_by_contract",
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
    /**Get a dataset by slug

Retrieve dataset information including columns, metadata, and ownership details

Sends a `GET` request to `/v1/datasets/{slug}`

Arguments:
- `slug`: Dataset slug (e.g., 'dex.trades')
- `api_key`: API Key, alternative to using the HTTP header X-Dune-Api-Key
- `x_dune_api_key`: API Key for the service
*/
    pub async fn getv1_datasets_slug<'a>(
        &'a self,
        slug: &'a str,
        api_key: Option<&'a str>,
        x_dune_api_key: &'a str,
    ) -> Result<ResponseValue<types::ModelsDatasetResponse>, Error<()>> {
        let url = format!(
            "{}/v1/datasets/{}", self.baseurl, encode_path(& slug.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(2usize);
        header_map
            .append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(Self::api_version()),
            );
        header_map.append("X-Dune-Api-Key", x_dune_api_key.to_string().try_into()?);
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new("api_key", &api_key))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "getv1_datasets_slug",
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
    /**Custom Endpoints

Create custom API endpoints from Dune queries

Sends a `GET` request to `/v1/endpoints/{handle}/{endpoint_slug}/results`

Arguments:
- `handle`: Your user or team handle
- `endpoint_slug`: The name of the endpoint as slug
- `allow_partial_results`: This enables returning a query result that was too large and only a partial result is
available. By default, allow_partial_results is set to false and a failed state is returned.
- `api_key`: API Key, alternative to using the HTTP header X-Dune-Api-Key
- `columns`: Specifies a comma-separated list of column names to return. If omitted, all columns are included.
Tip: use this to limit the result to specific columns, reducing datapoints cost of the call.
- `filters`: Expression to filter out rows from the results to return. This expression is similar to
a SQL WHERE clause. More details about it in the Filtering section of the doc.
This parameter is incompatible with sample_count.
- `ignore_max_credits_per_request`: To bypass the default max credits per request limit, set ignore_max_credits_per_request=true
- `limit`: Limit number of rows to return. This together with 'offset' allows easy pagination through
results in an incremental and efficient way. This parameter is incompatible
with sampling (sample_count).
- `offset`: Offset row number to start (inclusive, first row means offset=0) returning results
from. This together with 'limit' allows easy pagination through results in an
incremental and efficient way. This parameter is incompatible with sampling (sample_count).
- `query_id`
- `sample_count`: Number of rows to return from the result by sampling the data. This is useful when you
want to get a uniform sample instead of the entire result. If the result has less
than the sample count, the entire result is returned. Note that this will return a
randomized sample, so not every call will return the same result. This parameter is
incompatible with `offset`, `limit`, and `filters` parameters.
- `sort_by`: Expression to define the order in which the results should be returned. This expression
is similar to a SQL ORDER BY clause. More details about it in the Sorting section of the doc.
- `x_dune_api_key`: API Key for the service
*/
    pub async fn getv1_endpoints_handle_endpointslug_results<'a>(
        &'a self,
        handle: &'a str,
        endpoint_slug: &'a str,
        allow_partial_results: Option<bool>,
        api_key: Option<&'a str>,
        columns: Option<&'a str>,
        filters: Option<&'a str>,
        ignore_max_credits_per_request: Option<bool>,
        limit: Option<i64>,
        offset: Option<i64>,
        query_id: i64,
        sample_count: Option<i64>,
        sort_by: Option<&'a str>,
        x_dune_api_key: &'a str,
    ) -> Result<ResponseValue<types::ModelsReadExecutionResultResponse>, Error<()>> {
        let url = format!(
            "{}/v1/endpoints/{}/{}/results", self.baseurl, encode_path(& handle
            .to_string()), encode_path(& endpoint_slug.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(2usize);
        header_map
            .append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(Self::api_version()),
            );
        header_map.append("X-Dune-Api-Key", x_dune_api_key.to_string().try_into()?);
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
                    "allow_partial_results",
                    &allow_partial_results,
                ),
            )
            .query(&progenitor_client::QueryParam::new("api_key", &api_key))
            .query(&progenitor_client::QueryParam::new("columns", &columns))
            .query(&progenitor_client::QueryParam::new("filters", &filters))
            .query(
                &progenitor_client::QueryParam::new(
                    "ignore_max_credits_per_request",
                    &ignore_max_credits_per_request,
                ),
            )
            .query(&progenitor_client::QueryParam::new("limit", &limit))
            .query(&progenitor_client::QueryParam::new("offset", &offset))
            .query(&progenitor_client::QueryParam::new("queryID", &query_id))
            .query(&progenitor_client::QueryParam::new("sample_count", &sample_count))
            .query(&progenitor_client::QueryParam::new("sort_by", &sort_by))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "getv1_endpoints_handle_endpointslug_results",
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
    /**Cancel a triggered execution request given the execution ID

Cancel a triggered execution request given the execution ID

Sends a `POST` request to `/v1/execution/{execution_id}/cancel`

Arguments:
- `execution_id`: The unique identifier of the execution
- `api_key`: API Key, alternative to using the HTTP header X-Dune-Api-Key
- `x_dune_api_key`: API Key for the service
*/
    pub async fn postv1_execution_executionid_cancel<'a>(
        &'a self,
        execution_id: &'a str,
        api_key: Option<&'a str>,
        x_dune_api_key: &'a str,
    ) -> Result<ResponseValue<types::ModelsCancelQueryExecutionResponse>, Error<()>> {
        let url = format!(
            "{}/v1/execution/{}/cancel", self.baseurl, encode_path(& execution_id
            .to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(2usize);
        header_map
            .append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(Self::api_version()),
            );
        header_map.append("X-Dune-Api-Key", x_dune_api_key.to_string().try_into()?);
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new("api_key", &api_key))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "postv1_execution_executionid_cancel",
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
    /**Given an execution ID, returns result of a an execution request

Given an execution ID, returns result of a an execution request

Sends a `GET` request to `/v1/execution/{execution_id}/results`

Arguments:
- `execution_id`: Execution ID
- `allow_partial_results`: This enables returning a query result that was too large and only a partial result is
available. By default, allow_partial_results is set to false and a failed state is returned.
- `api_key`: Alternative to using the X-Dune-Api-Key header
- `columns`: Specifies a comma-separated list of column names to return. If omitted, all columns are included.
Tip: use this to limit the result to specific columns, reducing datapoints cost of the call.
- `filters`: Expression to filter out rows from the results to return. This expression is similar to
a SQL WHERE clause. More details about it in the Filtering section of the doc.
This parameter is incompatible with sample_count.
- `ignore_max_credits_per_request`: To bypass the default max credits per request limit, set ignore_max_credits_per_request=true
- `limit`: Limit number of rows to return. This together with 'offset' allows easy pagination through
results in an incremental and efficient way. This parameter is incompatible
with sampling (sample_count).
- `offset`: Offset row number to start (inclusive, first row means offset=0) returning results
from. This together with 'limit' allows easy pagination through results in an
incremental and efficient way. This parameter is incompatible with sampling (sample_count).
- `sample_count`: Number of rows to return from the result by sampling the data. This is useful when you
want to get a uniform sample instead of the entire result. If the result has less
than the sample count, the entire result is returned. Note that this will return a
randomized sample, so not every call will return the same result. This parameter is
incompatible with `offset`, `limit`, and `filters` parameters.
- `sort_by`: Expression to define the order in which the results should be returned. This expression
is similar to a SQL ORDER BY clause. More details about it in the Sorting section of the doc.
- `x_dune_api_key`: API Key for the service
*/
    pub async fn getv1_execution_executionid_results<'a>(
        &'a self,
        execution_id: &'a str,
        allow_partial_results: Option<bool>,
        api_key: Option<&'a str>,
        columns: Option<&'a str>,
        filters: Option<&'a str>,
        ignore_max_credits_per_request: Option<bool>,
        limit: Option<i64>,
        offset: Option<i64>,
        sample_count: Option<i64>,
        sort_by: Option<&'a str>,
        x_dune_api_key: &'a str,
    ) -> Result<ResponseValue<types::ModelsReadExecutionResultResponse>, Error<()>> {
        let url = format!(
            "{}/v1/execution/{}/results", self.baseurl, encode_path(& execution_id
            .to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(2usize);
        header_map
            .append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(Self::api_version()),
            );
        header_map.append("X-Dune-Api-Key", x_dune_api_key.to_string().try_into()?);
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
                    "allow_partial_results",
                    &allow_partial_results,
                ),
            )
            .query(&progenitor_client::QueryParam::new("api_key", &api_key))
            .query(&progenitor_client::QueryParam::new("columns", &columns))
            .query(&progenitor_client::QueryParam::new("filters", &filters))
            .query(
                &progenitor_client::QueryParam::new(
                    "ignore_max_credits_per_request",
                    &ignore_max_credits_per_request,
                ),
            )
            .query(&progenitor_client::QueryParam::new("limit", &limit))
            .query(&progenitor_client::QueryParam::new("offset", &offset))
            .query(&progenitor_client::QueryParam::new("sample_count", &sample_count))
            .query(&progenitor_client::QueryParam::new("sort_by", &sort_by))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "getv1_execution_executionid_results",
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
    /**Get results of a query execution in CSV format

Given a query ID, returns the latest executed result of a query on Dune in CSV format

Sends a `GET` request to `/v1/execution/{execution_id}/results/csv`

Arguments:
- `execution_id`: Execution ID
- `allow_partial_results`: This enables returning a query result that was too large and only a partial result is
available. By default, allow_partial_results is set to false and a failed state is returned.
- `api_key`: Alternative to using the X-Dune-Api-Key header
- `columns`: Specifies a comma-separated list of column names to return. If omitted, all columns are included.
Tip: use this to limit the result to specific columns, reducing datapoints cost of the call.
- `filters`: Expression to filter out rows from the results to return. This expression is similar to
a SQL WHERE clause. More details about it in the Filtering section of the doc.
This parameter is incompatible with sample_count.
- `ignore_max_credits_per_request`: To bypass the default max credits per request limit, set ignore_max_credits_per_request=true
- `limit`: Limit number of rows to return. This together with 'offset' allows easy pagination through
results in an incremental and efficient way. This parameter is incompatible
with sampling (sample_count).
- `offset`: Offset row number to start (inclusive, first row means offset=0) returning results
from. This together with 'limit' allows easy pagination through results in an
incremental and efficient way. This parameter is incompatible with sampling (sample_count).
- `sample_count`: Number of rows to return from the result by sampling the data. This is useful when you
want to get a uniform sample instead of the entire result. If the result has less
than the sample count, the entire result is returned. Note that this will return a
randomized sample, so not every call will return the same result. This parameter is
incompatible with `offset`, `limit`, and `filters` parameters.
- `sort_by`: Expression to define the order in which the results should be returned. This expression
is similar to a SQL ORDER BY clause. More details about it in the Sorting section of the doc.
- `x_dune_api_key`: API Key for the service
*/
    pub async fn getv1_execution_executionid_results_csv<'a>(
        &'a self,
        execution_id: &'a str,
        allow_partial_results: Option<bool>,
        api_key: Option<&'a str>,
        columns: Option<&'a str>,
        filters: Option<&'a str>,
        ignore_max_credits_per_request: Option<bool>,
        limit: Option<i64>,
        offset: Option<i64>,
        sample_count: Option<i64>,
        sort_by: Option<&'a str>,
        x_dune_api_key: &'a str,
    ) -> Result<ResponseValue<ByteStream>, Error<()>> {
        let url = format!(
            "{}/v1/execution/{}/results/csv", self.baseurl, encode_path(& execution_id
            .to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(2usize);
        header_map
            .append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(Self::api_version()),
            );
        header_map.append("X-Dune-Api-Key", x_dune_api_key.to_string().try_into()?);
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .query(
                &progenitor_client::QueryParam::new(
                    "allow_partial_results",
                    &allow_partial_results,
                ),
            )
            .query(&progenitor_client::QueryParam::new("api_key", &api_key))
            .query(&progenitor_client::QueryParam::new("columns", &columns))
            .query(&progenitor_client::QueryParam::new("filters", &filters))
            .query(
                &progenitor_client::QueryParam::new(
                    "ignore_max_credits_per_request",
                    &ignore_max_credits_per_request,
                ),
            )
            .query(&progenitor_client::QueryParam::new("limit", &limit))
            .query(&progenitor_client::QueryParam::new("offset", &offset))
            .query(&progenitor_client::QueryParam::new("sample_count", &sample_count))
            .query(&progenitor_client::QueryParam::new("sort_by", &sort_by))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "getv1_execution_executionid_results_csv",
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
    /**Check the status of an execution request

Check the status of an execution request

Sends a `GET` request to `/v1/execution/{execution_id}/status`

Arguments:
- `execution_id`: Execution ID
- `api_key`: Alternative to using the X-Dune-Api-Key header
- `x_dune_api_key`: API Key for the service
*/
    pub async fn getv1_execution_executionid_status<'a>(
        &'a self,
        execution_id: &'a str,
        api_key: Option<&'a str>,
        x_dune_api_key: &'a str,
    ) -> Result<ResponseValue<types::ModelsGetExecutionStatusResponse>, Error<()>> {
        let url = format!(
            "{}/v1/execution/{}/status", self.baseurl, encode_path(& execution_id
            .to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(2usize);
        header_map
            .append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(Self::api_version()),
            );
        header_map.append("X-Dune-Api-Key", x_dune_api_key.to_string().try_into()?);
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new("api_key", &api_key))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "getv1_execution_executionid_status",
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
    /**This lists all materialized view owned by the account tied to the API key

This lists all materialized view owned by the account tied to the API key

Sends a `GET` request to `/v1/materialized-views`

Arguments:
- `api_key`: API Key, alternative to using the HTTP header X-Dune-Api-Key
- `limit`: Number of materialized views to return on a page. Default and max 10000
- `offset`: Offset used for pagination. Use the value provided on a previous response under next_offset
- `x_dune_api_key`: API Key for the service
*/
    pub async fn getv1_materialized_views<'a>(
        &'a self,
        api_key: Option<&'a str>,
        limit: Option<&'a str>,
        offset: Option<&'a str>,
        x_dune_api_key: &'a str,
    ) -> Result<ResponseValue<types::MatviewsMatviewsListResponse>, Error<()>> {
        let url = format!("{}/v1/materialized-views", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(2usize);
        header_map
            .append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(Self::api_version()),
            );
        header_map.append("X-Dune-Api-Key", x_dune_api_key.to_string().try_into()?);
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new("api_key", &api_key))
            .query(&progenitor_client::QueryParam::new("limit", &limit))
            .query(&progenitor_client::QueryParam::new("offset", &offset))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "getv1_materialized_views",
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
    /**This upserts (create or replace) a materialized view from an existing query

This upserts a materialized view from an existing query. If the materialized view with the given name

Sends a `POST` request to `/v1/materialized-views`

Arguments:
- `api_key`: API Key, alternative to using the HTTP header X-Dune-Api-Key
- `x_dune_api_key`: API Key for the service
- `body`: MatviewsUpsertRequest
*/
    pub async fn postv1_materialized_views<'a>(
        &'a self,
        api_key: Option<&'a str>,
        x_dune_api_key: &'a str,
        body: &'a types::MatviewsMatviewsUpsertRequest,
    ) -> Result<ResponseValue<types::MatviewsMatviewsUpsertResponse>, Error<()>> {
        let url = format!("{}/v1/materialized-views", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(2usize);
        header_map
            .append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(Self::api_version()),
            );
        header_map.append("X-Dune-Api-Key", x_dune_api_key.to_string().try_into()?);
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .query(&progenitor_client::QueryParam::new("api_key", &api_key))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "postv1_materialized_views",
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
    /**This fetches a materialized view given a name

This fetches a materialized view given a name

Sends a `GET` request to `/v1/materialized-views/{name}`

Arguments:
- `name`: Matview Name
- `api_key`: API Key, alternative to using the HTTP header X-Dune-Api-Key
- `x_dune_api_key`: API Key for the service
*/
    pub async fn getv1_materialized_views_name<'a>(
        &'a self,
        name: &'a str,
        api_key: Option<&'a str>,
        x_dune_api_key: &'a str,
    ) -> Result<ResponseValue<types::MatviewsMatviewsGetResponse>, Error<()>> {
        let url = format!(
            "{}/v1/materialized-views/{}", self.baseurl, encode_path(& name.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(2usize);
        header_map
            .append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(Self::api_version()),
            );
        header_map.append("X-Dune-Api-Key", x_dune_api_key.to_string().try_into()?);
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new("api_key", &api_key))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "getv1_materialized_views_name",
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
    /**This deletes a materialized view given a full name

This deletes a materialized view given a full name

Sends a `DELETE` request to `/v1/materialized-views/{name}`

Arguments:
- `name`: Matview Name
- `api_key`: API Key, alternative to using the HTTP header X-Dune-Api-Key
- `x_dune_api_key`: API Key for the service
*/
    pub async fn deletev1_materialized_views_name<'a>(
        &'a self,
        name: &'a str,
        api_key: Option<&'a str>,
        x_dune_api_key: &'a str,
    ) -> Result<ResponseValue<types::MatviewsMatviewsDeleteResponse>, Error<()>> {
        let url = format!(
            "{}/v1/materialized-views/{}", self.baseurl, encode_path(& name.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(2usize);
        header_map
            .append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(Self::api_version()),
            );
        header_map.append("X-Dune-Api-Key", x_dune_api_key.to_string().try_into()?);
        #[allow(unused_mut)]
        let mut request = self
            .client
            .delete(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new("api_key", &api_key))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "deletev1_materialized_views_name",
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
    /**This refreshes a materialized view

This refreshes a materialized view

Sends a `POST` request to `/v1/materialized-views/{name}/refresh`

Arguments:
- `name`: Matview Name
- `api_key`: API Key, alternative to using the HTTP header X-Dune-Api-Key
- `x_dune_api_key`: API Key for the service
- `body`: MatviewsRefreshRequest
*/
    pub async fn postv1_materialized_views_name_refresh<'a>(
        &'a self,
        name: &'a str,
        api_key: Option<&'a str>,
        x_dune_api_key: &'a str,
        body: &'a types::MatviewsMatviewsRefreshRequest,
    ) -> Result<ResponseValue<types::MatviewsMatviewsRefreshResponse>, Error<()>> {
        let url = format!(
            "{}/v1/materialized-views/{}/refresh", self.baseurl, encode_path(& name
            .to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(2usize);
        header_map
            .append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(Self::api_version()),
            );
        header_map.append("X-Dune-Api-Key", x_dune_api_key.to_string().try_into()?);
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .query(&progenitor_client::QueryParam::new("api_key", &api_key))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "postv1_materialized_views_name_refresh",
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
    /**Execute a pipeline

Executes a specified pipeline. A pipeline allows you to chain multiple queries and materialized view refreshes together and execute them as a single unit.

Sends a `POST` request to `/v1/pipelines/execute`

Arguments:
- `api_key`: Alternative to using the X-Dune-Api-Key header
- `x_dune_api_key`: API Key for the service
- `body`: Pipeline execution request
*/
    pub async fn postv1_pipelines_execute<'a>(
        &'a self,
        api_key: Option<&'a str>,
        x_dune_api_key: &'a str,
        body: &'a types::ModelsExecutePipelineRequest,
    ) -> Result<ResponseValue<types::ModelsExecutePipelineResponse>, Error<()>> {
        let url = format!("{}/v1/pipelines/execute", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(2usize);
        header_map
            .append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(Self::api_version()),
            );
        header_map.append("X-Dune-Api-Key", x_dune_api_key.to_string().try_into()?);
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .query(&progenitor_client::QueryParam::new("api_key", &api_key))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "postv1_pipelines_execute",
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
    /**Get pipeline execution status

Retrieves the status of a pipeline execution, including the status of each node in the pipeline.

Sends a `GET` request to `/v1/pipelines/executions/{pipeline_execution_id}/status`

Arguments:
- `pipeline_execution_id`: Unique identifier of the pipeline execution
- `api_key`: Alternative to using the X-Dune-Api-Key header
- `x_dune_api_key`: API Key for the service
*/
    pub async fn getv1_pipelines_executions_pipelineexecutionid_status<'a>(
        &'a self,
        pipeline_execution_id: &'a str,
        api_key: Option<&'a str>,
        x_dune_api_key: &'a str,
    ) -> Result<
        ResponseValue<types::ModelsGetPipelineExecutionStatusResponse>,
        Error<()>,
    > {
        let url = format!(
            "{}/v1/pipelines/executions/{}/status", self.baseurl, encode_path(&
            pipeline_execution_id.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(2usize);
        header_map
            .append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(Self::api_version()),
            );
        header_map.append("X-Dune-Api-Key", x_dune_api_key.to_string().try_into()?);
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new("api_key", &api_key))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "getv1_pipelines_executions_pipelineexecutionid_status",
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
    /**List queries

Retrieve a paginated list of queries owned by the account tied to the API key

Sends a `GET` request to `/v1/queries`

Arguments:
- `api_key`: API Key, alternative to using the HTTP header X-Dune-Api-Key
- `limit`: Number of queries to return on a page. Default: 20
- `offset`: Offset used for pagination. Default: 0
- `x_dune_api_key`: API Key for the service
*/
    pub async fn getv1_queries<'a>(
        &'a self,
        api_key: Option<&'a str>,
        limit: Option<i64>,
        offset: Option<i64>,
        x_dune_api_key: &'a str,
    ) -> Result<ResponseValue<types::ModelsListQueriesResponse>, Error<()>> {
        let url = format!("{}/v1/queries", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(2usize);
        header_map
            .append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(Self::api_version()),
            );
        header_map.append("X-Dune-Api-Key", x_dune_api_key.to_string().try_into()?);
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new("api_key", &api_key))
            .query(&progenitor_client::QueryParam::new("limit", &limit))
            .query(&progenitor_client::QueryParam::new("offset", &offset))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "getv1_queries",
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
    /**List visualizations for a query

Lists all visualizations attached to the specified query with pagination.

Sends a `GET` request to `/v1/queries/{query_id}/visualizations`

Arguments:
- `query_id`: Query ID
- `limit`: Max results to return (default 25, max 100)
- `offset`: Pagination offset (default 0)
- `x_dune_api_key`: API Key for the service
*/
    pub async fn getv1_queries_queryid_visualizations<'a>(
        &'a self,
        query_id: i64,
        limit: Option<i64>,
        offset: Option<i64>,
        x_dune_api_key: &'a str,
    ) -> Result<ResponseValue<types::ModelsListVisualizationsResponse>, Error<()>> {
        let url = format!(
            "{}/v1/queries/{}/visualizations", self.baseurl, encode_path(& query_id
            .to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(2usize);
        header_map
            .append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(Self::api_version()),
            );
        header_map.append("X-Dune-Api-Key", x_dune_api_key.to_string().try_into()?);
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
            operation_id: "getv1_queries_queryid_visualizations",
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
    /**Create a visualization on an existing query

Creates a new visualization attached to the specified query.

Sends a `POST` request to `/v1/queries/{query_id}/visualizations`

Arguments:
- `query_id`: Query ID
- `x_dune_api_key`: API Key for the service
- `body`: CreateVisualizationRequest
*/
    pub async fn postv1_queries_queryid_visualizations<'a>(
        &'a self,
        query_id: i64,
        x_dune_api_key: &'a str,
        body: &'a types::ModelsCreateVisualizationRequest,
    ) -> Result<ResponseValue<types::ModelsCreateVisualizationResponse>, Error<()>> {
        let url = format!(
            "{}/v1/queries/{}/visualizations", self.baseurl, encode_path(& query_id
            .to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(2usize);
        header_map
            .append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(Self::api_version()),
            );
        header_map.append("X-Dune-Api-Key", x_dune_api_key.to_string().try_into()?);
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
            operation_id: "postv1_queries_queryid_visualizations",
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
    /**Create and save a query on Dune

This API allows for anyone to create a query.
The owner of the query will be under the context of the API key.

Sends a `POST` request to `/v1/query`

Arguments:
- `api_key`: API Key, alternative to using the HTTP header X-Dune-Api-Key
- `x_dune_api_key`: API Key for the service
- `body`: CreateQueryRequest
*/
    pub async fn postv1_query<'a>(
        &'a self,
        api_key: Option<&'a str>,
        x_dune_api_key: &'a str,
        body: &'a types::ModelsCreateQueryRequest,
    ) -> Result<ResponseValue<types::ModelsCreateQueryResponse>, Error<()>> {
        let url = format!("{}/v1/query", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(2usize);
        header_map
            .append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(Self::api_version()),
            );
        header_map.append("X-Dune-Api-Key", x_dune_api_key.to_string().try_into()?);
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .query(&progenitor_client::QueryParam::new("api_key", &api_key))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "postv1_query",
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
    /**Returns the details of a query on Dune

This API allows for anyone to read the sql text,
parameters, name, tags, and state of a query.
For private queries, only the API key generated
under the context of the owner of that query will work.

Sends a `GET` request to `/v1/query/{queryId}`

Arguments:
- `query_id`: Query ID
- `api_key`: API Key, alternative to using the HTTP header X-Dune-Api-Key
- `x_dune_api_key`: API Key for the service
*/
    pub async fn getv1_query_queryid<'a>(
        &'a self,
        query_id: i64,
        api_key: Option<&'a str>,
        x_dune_api_key: &'a str,
    ) -> Result<ResponseValue<types::ModelsGetQueryResponse>, Error<()>> {
        let url = format!(
            "{}/v1/query/{}", self.baseurl, encode_path(& query_id.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(2usize);
        header_map
            .append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(Self::api_version()),
            );
        header_map.append("X-Dune-Api-Key", x_dune_api_key.to_string().try_into()?);
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new("api_key", &api_key))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "getv1_query_queryid",
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
    /**Update the details of a query on Dune

This API allows for anyone to update the sql text,
parameters, name, tags, and state of a query. Only the API key
generated under the context of the owner of that query will work.

Sends a `PATCH` request to `/v1/query/{queryId}`

Arguments:
- `query_id`: Query ID
- `api_key`: API Key, alternative to using the HTTP header X-Dune-Api-Key
- `x_dune_api_key`: API Key for the service
*/
    pub async fn patchv1_query_queryid<'a>(
        &'a self,
        query_id: i64,
        api_key: Option<&'a str>,
        x_dune_api_key: &'a str,
    ) -> Result<ResponseValue<types::ModelsUpdateQueryResponse>, Error<()>> {
        let url = format!(
            "{}/v1/query/{}", self.baseurl, encode_path(& query_id.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(2usize);
        header_map
            .append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(Self::api_version()),
            );
        header_map.append("X-Dune-Api-Key", x_dune_api_key.to_string().try_into()?);
        #[allow(unused_mut)]
        let mut request = self
            .client
            .patch(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new("api_key", &api_key))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "patchv1_query_queryid",
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
    /**Archive Query on Dune

This API allows for anyone to archive a query. Only the API key
generated under the context of the owner of that query will work.
This does not delete the query, but will make
it uneditable/unexecutable

Sends a `POST` request to `/v1/query/{queryId}/archive`

Arguments:
- `query_id`: Query ID
- `api_key`: API Key, alternative to using the HTTP header X-Dune-Api-Key
- `x_dune_api_key`: API Key for the service
*/
    pub async fn postv1_query_queryid_archive<'a>(
        &'a self,
        query_id: i64,
        api_key: Option<&'a str>,
        x_dune_api_key: &'a str,
    ) -> Result<ResponseValue<types::ModelsUpdateQueryResponse>, Error<()>> {
        let url = format!(
            "{}/v1/query/{}/archive", self.baseurl, encode_path(& query_id.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(2usize);
        header_map
            .append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(Self::api_version()),
            );
        header_map.append("X-Dune-Api-Key", x_dune_api_key.to_string().try_into()?);
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new("api_key", &api_key))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "postv1_query_queryid_archive",
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
    /**Private Query on Dune

This API allows for anyone to private a query. Only the API key
generated under the context of the owner of that query will work.

Sends a `POST` request to `/v1/query/{queryId}/private`

Arguments:
- `query_id`: Query ID
- `api_key`: API Key, alternative to using the HTTP header X-Dune-Api-Key
- `x_dune_api_key`: API Key for the service
*/
    pub async fn postv1_query_queryid_private<'a>(
        &'a self,
        query_id: i64,
        api_key: Option<&'a str>,
        x_dune_api_key: &'a str,
    ) -> Result<ResponseValue<types::ModelsUpdateQueryResponse>, Error<()>> {
        let url = format!(
            "{}/v1/query/{}/private", self.baseurl, encode_path(& query_id.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(2usize);
        header_map
            .append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(Self::api_version()),
            );
        header_map.append("X-Dune-Api-Key", x_dune_api_key.to_string().try_into()?);
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new("api_key", &api_key))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "postv1_query_queryid_private",
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
    /**Unarchive Query on Dune

This API allows for anyone to unarchive a query. Only the API key
generated under the context of the owner of that query will work.

Sends a `POST` request to `/v1/query/{queryId}/unarchive`

Arguments:
- `query_id`: Query ID
- `api_key`: API Key, alternative to using the HTTP header X-Dune-Api-Key
- `x_dune_api_key`: API Key for the service
*/
    pub async fn postv1_query_queryid_unarchive<'a>(
        &'a self,
        query_id: i64,
        api_key: Option<&'a str>,
        x_dune_api_key: &'a str,
    ) -> Result<ResponseValue<types::ModelsUpdateQueryResponse>, Error<()>> {
        let url = format!(
            "{}/v1/query/{}/unarchive", self.baseurl, encode_path(& query_id
            .to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(2usize);
        header_map
            .append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(Self::api_version()),
            );
        header_map.append("X-Dune-Api-Key", x_dune_api_key.to_string().try_into()?);
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new("api_key", &api_key))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "postv1_query_queryid_unarchive",
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
    /**Unprivate a query on Dune

This API allows for anyone to unprivate a query. Only the API key
generated under the context of the owner of that query will work.

Sends a `POST` request to `/v1/query/{queryId}/unprivate`

Arguments:
- `query_id`: Query ID
- `api_key`: API Key, alternative to using the HTTP header X-Dune-Api-Key
- `x_dune_api_key`: API Key for the service
*/
    pub async fn postv1_query_queryid_unprivate<'a>(
        &'a self,
        query_id: i64,
        api_key: Option<&'a str>,
        x_dune_api_key: &'a str,
    ) -> Result<ResponseValue<types::ModelsUpdateQueryResponse>, Error<()>> {
        let url = format!(
            "{}/v1/query/{}/unprivate", self.baseurl, encode_path(& query_id
            .to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(2usize);
        header_map
            .append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(Self::api_version()),
            );
        header_map.append("X-Dune-Api-Key", x_dune_api_key.to_string().try_into()?);
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new("api_key", &api_key))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "postv1_query_queryid_unprivate",
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
    /**Execute, or run a query for the specified query ID

Execute, or run a query for the specified query ID

Sends a `POST` request to `/v1/query/{query_id}/execute`

Arguments:
- `query_id`: Unique identifier of the query
- `api_key`: Alternative to using the X-Dune-Api-Key header
- `performance`: The performance engine tier the execution will be run on. Can be `small`, `medium`, or `large`. Omit to use the default tier for the query engine. Credits are consumed based on actual compute resources used.
- `query_parameters`: SQL Query parameters in json key-value pairs. Each parameter is to be provided in key-value pairs. This enables you to execute a parameterized query with the provided values for your parameter keys. Partial submission of parameters is allowed. For example, if the query expects three parameters and you only pass in two, the third one will automatically use its default value as defined in the Query Parameter Editor page.
- `x_dune_api_key`: API Key for the service
*/
    pub async fn postv1_query_queryid_execute<'a>(
        &'a self,
        query_id: i64,
        api_key: Option<&'a str>,
        performance: Option<types::Postv1QueryQueryidExecutePerformance>,
        query_parameters: Option<&'a str>,
        x_dune_api_key: &'a str,
    ) -> Result<ResponseValue<types::ModelsExecuteQueryResponse>, Error<()>> {
        let url = format!(
            "{}/v1/query/{}/execute", self.baseurl, encode_path(& query_id.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(2usize);
        header_map
            .append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(Self::api_version()),
            );
        header_map.append("X-Dune-Api-Key", x_dune_api_key.to_string().try_into()?);
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new("api_key", &api_key))
            .query(&progenitor_client::QueryParam::new("performance", &performance))
            .query(
                &progenitor_client::QueryParam::new(
                    "query_parameters",
                    &query_parameters,
                ),
            )
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "postv1_query_queryid_execute",
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
    /**Get query pipeline

Builds a query pipeline from the specified query and returns the pipeline definition containing all the nested materialized views that the query depends on. A pipeline allows you to chain multiple queries together and execute them as a single unit.

Sends a `GET` request to `/v1/query/{query_id}/pipeline`

Arguments:
- `query_id`: Unique identifier of the query
- `api_key`: Alternative to using the X-Dune-Api-Key header
- `performance`: The performance engine tier. Can be `small`, `medium`, or `large`.
- `query_parameters`: SQL Query parameters in json key-value pairs
- `x_dune_api_key`: API Key for the service
*/
    pub async fn getv1_query_queryid_pipeline<'a>(
        &'a self,
        query_id: i64,
        api_key: Option<&'a str>,
        performance: Option<&'a str>,
        query_parameters: Option<&'a str>,
        x_dune_api_key: &'a str,
    ) -> Result<ResponseValue<types::ModelsGetQueryPipelineResponse>, Error<()>> {
        let url = format!(
            "{}/v1/query/{}/pipeline", self.baseurl, encode_path(& query_id.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(2usize);
        header_map
            .append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(Self::api_version()),
            );
        header_map.append("X-Dune-Api-Key", x_dune_api_key.to_string().try_into()?);
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new("api_key", &api_key))
            .query(&progenitor_client::QueryParam::new("performance", &performance))
            .query(
                &progenitor_client::QueryParam::new(
                    "query_parameters",
                    &query_parameters,
                ),
            )
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "getv1_query_queryid_pipeline",
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
    /**Execute a query pipeline

Builds a query pipeline from the specified query and executes it. A pipeline allows you to chain multiple queries together and execute them as a single unit.

Sends a `POST` request to `/v1/query/{query_id}/pipeline/execute`

Arguments:
- `query_id`: Unique identifier of the query
- `api_key`: Alternative to using the X-Dune-Api-Key header
- `x_dune_api_key`: API Key for the service
- `body`: Query pipeline execution request
*/
    pub async fn postv1_query_queryid_pipeline_execute<'a>(
        &'a self,
        query_id: i64,
        api_key: Option<&'a str>,
        x_dune_api_key: &'a str,
        body: &'a types::ModelsExecuteQueryPipelineRequest,
    ) -> Result<ResponseValue<types::ModelsExecuteQueryPipelineResponse>, Error<()>> {
        let url = format!(
            "{}/v1/query/{}/pipeline/execute", self.baseurl, encode_path(& query_id
            .to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(2usize);
        header_map
            .append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(Self::api_version()),
            );
        header_map.append("X-Dune-Api-Key", x_dune_api_key.to_string().try_into()?);
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .query(&progenitor_client::QueryParam::new("api_key", &api_key))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "postv1_query_queryid_pipeline_execute",
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
    /**Get Latest Query Result

Given a query ID, returns the latest executed result of a query on Dune in JSON format

Sends a `GET` request to `/v1/query/{query_id}/results`

Arguments:
- `query_id`: Query ID
- `allow_partial_results`: This enables returning a query result that was too large and only a partial result is
available. By default, allow_partial_results is set to false and a failed state is returned.
- `api_key`: Alternative to using the X-Dune-Api-Key header
- `columns`: Specifies a comma-separated list of column names to return. If omitted, all columns are included.
Tip: use this to limit the result to specific columns, reducing datapoints cost of the call.
- `filters`: Expression to filter out rows from the results to return. This expression is similar to
a SQL WHERE clause. More details about it in the Filtering section of the doc.
This parameter is incompatible with sample_count.
- `ignore_max_credits_per_request`: To bypass the default max credits per request limit, set ignore_max_credits_per_request=true
- `limit`: Limit number of rows to return. This together with 'offset' allows easy pagination through
results in an incremental and efficient way. This parameter is incompatible
with sampling (sample_count).
- `offset`: Offset row number to start (inclusive, first row means offset=0) returning results
from. This together with 'limit' allows easy pagination through results in an
incremental and efficient way. This parameter is incompatible with sampling (sample_count).
- `sample_count`: Number of rows to return from the result by sampling the data. This is useful when you
want to get a uniform sample instead of the entire result. If the result has less
than the sample count, the entire result is returned. Note that this will return a
randomized sample, so not every call will return the same result. This parameter is
incompatible with `offset`, `limit`, and `filters` parameters.
- `sort_by`: Expression to define the order in which the results should be returned. This expression
is similar to a SQL ORDER BY clause. More details about it in the Sorting section of the doc.
- `x_dune_api_key`: API Key for the service
*/
    pub async fn getv1_query_queryid_results<'a>(
        &'a self,
        query_id: &'a str,
        allow_partial_results: Option<bool>,
        api_key: Option<&'a str>,
        columns: Option<&'a str>,
        filters: Option<&'a str>,
        ignore_max_credits_per_request: Option<bool>,
        limit: Option<i64>,
        offset: Option<i64>,
        sample_count: Option<i64>,
        sort_by: Option<&'a str>,
        x_dune_api_key: &'a str,
    ) -> Result<ResponseValue<types::ModelsReadExecutionResultResponse>, Error<()>> {
        let url = format!(
            "{}/v1/query/{}/results", self.baseurl, encode_path(& query_id.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(2usize);
        header_map
            .append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(Self::api_version()),
            );
        header_map.append("X-Dune-Api-Key", x_dune_api_key.to_string().try_into()?);
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
                    "allow_partial_results",
                    &allow_partial_results,
                ),
            )
            .query(&progenitor_client::QueryParam::new("api_key", &api_key))
            .query(&progenitor_client::QueryParam::new("columns", &columns))
            .query(&progenitor_client::QueryParam::new("filters", &filters))
            .query(
                &progenitor_client::QueryParam::new(
                    "ignore_max_credits_per_request",
                    &ignore_max_credits_per_request,
                ),
            )
            .query(&progenitor_client::QueryParam::new("limit", &limit))
            .query(&progenitor_client::QueryParam::new("offset", &offset))
            .query(&progenitor_client::QueryParam::new("sample_count", &sample_count))
            .query(&progenitor_client::QueryParam::new("sort_by", &sort_by))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "getv1_query_queryid_results",
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
    /**Given a query ID, returns the latest executed result of a query on Dune as CSV

Given a query ID, returns the latest executed result of a query on Dune as CSV

Sends a `GET` request to `/v1/query/{query_id}/results/csv`

Arguments:
- `query_id`: Query ID
- `allow_partial_results`: This enables returning a query result that was too large and only a partial result is
available. By default, allow_partial_results is set to false and a failed state is returned.
- `api_key`: Alternative to using the X-Dune-Api-Key header
- `columns`: Specifies a comma-separated list of column names to return. If omitted, all columns are included.
Tip: use this to limit the result to specific columns, reducing datapoints cost of the call.
- `filters`: Expression to filter out rows from the results to return. This expression is similar to
a SQL WHERE clause. More details about it in the Filtering section of the doc.
This parameter is incompatible with sample_count.
- `ignore_max_credits_per_request`: To bypass the default max credits per request limit, set ignore_max_credits_per_request=true
- `limit`: Limit number of rows to return. This together with 'offset' allows easy pagination through
results in an incremental and efficient way. This parameter is incompatible
with sampling (sample_count).
- `offset`: Offset row number to start (inclusive, first row means offset=0) returning results
from. This together with 'limit' allows easy pagination through results in an
incremental and efficient way. This parameter is incompatible with sampling (sample_count).
- `sample_count`: Number of rows to return from the result by sampling the data. This is useful when you
want to get a uniform sample instead of the entire result. If the result has less
than the sample count, the entire result is returned. Note that this will return a
randomized sample, so not every call will return the same result. This parameter is
incompatible with `offset`, `limit`, and `filters` parameters.
- `sort_by`: Expression to define the order in which the results should be returned. This expression
is similar to a SQL ORDER BY clause. More details about it in the Sorting section of the doc.
- `x_dune_api_key`: API Key for the service
*/
    pub async fn getv1_query_queryid_results_csv<'a>(
        &'a self,
        query_id: &'a str,
        allow_partial_results: Option<bool>,
        api_key: Option<&'a str>,
        columns: Option<&'a str>,
        filters: Option<&'a str>,
        ignore_max_credits_per_request: Option<bool>,
        limit: Option<i64>,
        offset: Option<i64>,
        sample_count: Option<i64>,
        sort_by: Option<&'a str>,
        x_dune_api_key: &'a str,
    ) -> Result<ResponseValue<ByteStream>, Error<()>> {
        let url = format!(
            "{}/v1/query/{}/results/csv", self.baseurl, encode_path(& query_id
            .to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(2usize);
        header_map
            .append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(Self::api_version()),
            );
        header_map.append("X-Dune-Api-Key", x_dune_api_key.to_string().try_into()?);
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .query(
                &progenitor_client::QueryParam::new(
                    "allow_partial_results",
                    &allow_partial_results,
                ),
            )
            .query(&progenitor_client::QueryParam::new("api_key", &api_key))
            .query(&progenitor_client::QueryParam::new("columns", &columns))
            .query(&progenitor_client::QueryParam::new("filters", &filters))
            .query(
                &progenitor_client::QueryParam::new(
                    "ignore_max_credits_per_request",
                    &ignore_max_credits_per_request,
                ),
            )
            .query(&progenitor_client::QueryParam::new("limit", &limit))
            .query(&progenitor_client::QueryParam::new("offset", &offset))
            .query(&progenitor_client::QueryParam::new("sample_count", &sample_count))
            .query(&progenitor_client::QueryParam::new("sort_by", &sort_by))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "getv1_query_queryid_results_csv",
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
    /**Execute raw SQL query

Execute raw SQL query without requiring a stored query ID

Sends a `POST` request to `/v1/sql/execute`

Arguments:
- `api_key`: Alternative to using the X-Dune-Api-Key header
- `x_dune_api_key`: API Key for the service
- `body`: ExecuteSQLRequest
*/
    pub async fn postv1_sql_execute<'a>(
        &'a self,
        api_key: Option<&'a str>,
        x_dune_api_key: &'a str,
        body: &'a types::ModelsExecuteSqlRequest,
    ) -> Result<ResponseValue<types::ModelsExecuteQueryResponse>, Error<()>> {
        let url = format!("{}/v1/sql/execute", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(2usize);
        header_map
            .append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(Self::api_version()),
            );
        header_map.append("X-Dune-Api-Key", x_dune_api_key.to_string().try_into()?);
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .query(&progenitor_client::QueryParam::new("api_key", &api_key))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "postv1_sql_execute",
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
    /**List all tables owned by the account

Returns a paginated list of all tables owned by the authenticated account.

Sends a `GET` request to `/v1/uploads`

Arguments:
- `api_key`: API Key, alternative to using the HTTP header X-Dune-Api-Key
- `limit`: Number of tables to return on a page. Default: 50, max: 10000
- `offset`: Offset used for pagination. Negative values are treated as 0
- `x_dune_api_key`: API Key for the service
*/
    pub async fn getv1_uploads<'a>(
        &'a self,
        api_key: Option<&'a str>,
        limit: Option<i32>,
        offset: Option<i32>,
        x_dune_api_key: &'a str,
    ) -> Result<ResponseValue<types::ModelsTableListResponse>, Error<()>> {
        let url = format!("{}/v1/uploads", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(2usize);
        header_map
            .append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(Self::api_version()),
            );
        header_map.append("X-Dune-Api-Key", x_dune_api_key.to_string().try_into()?);
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new("api_key", &api_key))
            .query(&progenitor_client::QueryParam::new("limit", &limit))
            .query(&progenitor_client::QueryParam::new("offset", &offset))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "getv1_uploads",
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
    /**Create an empty uploaded table with a defined schema

Creates an empty uploaded table that can be inserted into with the /insert endpoint.
Each successful table creation consumes 10 credits.

Sends a `POST` request to `/v1/uploads`

Arguments:
- `api_key`: Alternative to using the X-Dune-Api-Key header
- `x_dune_api_key`: API Key for the service
- `body`: payload
*/
    pub async fn postv1_uploads<'a>(
        &'a self,
        api_key: Option<&'a str>,
        x_dune_api_key: &'a str,
        body: &'a types::ModelsTableCreateRequest,
    ) -> Result<ResponseValue<types::ModelsTableCreateResponse>, Error<()>> {
        let url = format!("{}/v1/uploads", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(2usize);
        header_map
            .append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(Self::api_version()),
            );
        header_map.append("X-Dune-Api-Key", x_dune_api_key.to_string().try_into()?);
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .query(&progenitor_client::QueryParam::new("api_key", &api_key))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "postv1_uploads",
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
    /**Upload CSV file as a table

Upload a CSV file to create a table with automatic schema inference.
The size limit per upload is 500MB. Storage limits: 100MB (free), 1GB (analyst), 15GB (plus).

Sends a `POST` request to `/v1/uploads/csv`

Arguments:
- `api_key`: Alternative to using the X-Dune-Api-Key header
- `x_dune_api_key`: API Key for the service
- `body`: payload
*/
    pub async fn postv1_uploads_csv<'a>(
        &'a self,
        api_key: Option<&'a str>,
        x_dune_api_key: &'a str,
        body: &'a types::ModelsCsvUploadRequest,
    ) -> Result<ResponseValue<types::ModelsCsvUploadResponse>, Error<()>> {
        let url = format!("{}/v1/uploads/csv", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(2usize);
        header_map
            .append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(Self::api_version()),
            );
        header_map.append("X-Dune-Api-Key", x_dune_api_key.to_string().try_into()?);
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .query(&progenitor_client::QueryParam::new("api_key", &api_key))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "postv1_uploads_csv",
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
    /**Delete an uploaded table

Permanently deletes the specified uploaded table and all its data.

Sends a `DELETE` request to `/v1/uploads/{namespace}/{table_name}`

Arguments:
- `namespace`: The namespace of the table to delete
- `table_name`: The table name of the table to delete
- `api_key`: Alternative to using the X-Dune-Api-Key header
- `x_dune_api_key`: API Key for the service
*/
    pub async fn deletev1_uploads_namespace_tablename<'a>(
        &'a self,
        namespace: &'a str,
        table_name: &'a str,
        api_key: Option<&'a str>,
        x_dune_api_key: &'a str,
    ) -> Result<ResponseValue<types::ModelsTableDeleteResponse>, Error<()>> {
        let url = format!(
            "{}/v1/uploads/{}/{}", self.baseurl, encode_path(& namespace.to_string()),
            encode_path(& table_name.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(2usize);
        header_map
            .append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(Self::api_version()),
            );
        header_map.append("X-Dune-Api-Key", x_dune_api_key.to_string().try_into()?);
        #[allow(unused_mut)]
        let mut request = self
            .client
            .delete(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new("api_key", &api_key))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "deletev1_uploads_namespace_tablename",
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
    /**Clear all data from an uploaded table

Removes all data from the specified uploaded table while preserving the table structure and schema.

Sends a `POST` request to `/v1/uploads/{namespace}/{table_name}/clear`

Arguments:
- `namespace`: The namespace of the table to clear
- `table_name`: The table name of the table to clear
- `api_key`: Alternative to using the X-Dune-Api-Key header
- `x_dune_api_key`: API Key for the service
*/
    pub async fn postv1_uploads_namespace_tablename_clear<'a>(
        &'a self,
        namespace: &'a str,
        table_name: &'a str,
        api_key: Option<&'a str>,
        x_dune_api_key: &'a str,
    ) -> Result<ResponseValue<types::ModelsTableClearResponse>, Error<()>> {
        let url = format!(
            "{}/v1/uploads/{}/{}/clear", self.baseurl, encode_path(& namespace
            .to_string()), encode_path(& table_name.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(2usize);
        header_map
            .append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(Self::api_version()),
            );
        header_map.append("X-Dune-Api-Key", x_dune_api_key.to_string().try_into()?);
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new("api_key", &api_key))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "postv1_uploads_namespace_tablename_clear",
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
    /**Insert data into an uploaded table

Inserts data into an existing table. Accepts CSV (text/csv) or
NDJSON (application/x-ndjson) content types.

Sends a `POST` request to `/v1/uploads/{namespace}/{table_name}/insert`

Arguments:
- `namespace`: The namespace of the table
- `table_name`: The table name
- `api_key`: Alternative to using the X-Dune-Api-Key header
- `content_type`: Content type: text/csv or application/x-ndjson
- `x_dune_api_key`: API Key for the service
- `body`: The data to insert (CSV or NDJSON format)
*/
    pub async fn postv1_uploads_namespace_tablename_insert<'a>(
        &'a self,
        namespace: &'a str,
        table_name: &'a str,
        api_key: Option<&'a str>,
        content_type: &'a str,
        x_dune_api_key: &'a str,
        body: &'a str,
    ) -> Result<ResponseValue<types::ModelsTableInsertResponse>, Error<()>> {
        let url = format!(
            "{}/v1/uploads/{}/{}/insert", self.baseurl, encode_path(& namespace
            .to_string()), encode_path(& table_name.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(3usize);
        header_map
            .append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(Self::api_version()),
            );
        header_map.append("Content-Type", content_type.to_string().try_into()?);
        header_map.append("X-Dune-Api-Key", x_dune_api_key.to_string().try_into()?);
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .query(&progenitor_client::QueryParam::new("api_key", &api_key))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "postv1_uploads_namespace_tablename_insert",
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
    /**Get Usage Data

Get usage data for the authenticated customer including private queries, dashboards,
bytes used/allowed, and billing periods

Sends a `POST` request to `/v1/usage`

Arguments:
- `api_key`: API Key, alternative to using the HTTP header X-Dune-Api-Key
- `x_dune_api_key`: API Key for the service
- `body`: Request payload with optional start_date and end_date
*/
    pub async fn postv1_usage<'a>(
        &'a self,
        api_key: Option<&'a str>,
        x_dune_api_key: &'a str,
        body: &'a types::ModelsGetUsageRequest,
    ) -> Result<ResponseValue<types::ModelsGetUsageResponse>, Error<()>> {
        let url = format!("{}/v1/usage", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(2usize);
        header_map
            .append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(Self::api_version()),
            );
        header_map.append("X-Dune-Api-Key", x_dune_api_key.to_string().try_into()?);
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .query(&progenitor_client::QueryParam::new("api_key", &api_key))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "postv1_usage",
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
    /**Get a visualization by ID

Retrieves a visualization by its ID.

Sends a `GET` request to `/v1/visualizations/{visualization_id}`

Arguments:
- `visualization_id`: Visualization ID
- `x_dune_api_key`: API Key for the service
*/
    pub async fn getv1_visualizations_visualizationid<'a>(
        &'a self,
        visualization_id: i64,
        x_dune_api_key: &'a str,
    ) -> Result<ResponseValue<types::ModelsGetVisualizationResponse>, Error<()>> {
        let url = format!(
            "{}/v1/visualizations/{}", self.baseurl, encode_path(& visualization_id
            .to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(2usize);
        header_map
            .append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(Self::api_version()),
            );
        header_map.append("X-Dune-Api-Key", x_dune_api_key.to_string().try_into()?);
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
            operation_id: "getv1_visualizations_visualizationid",
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
    /**Update a visualization

Updates an existing visualization by its ID.

Sends a `PUT` request to `/v1/visualizations/{visualization_id}`

Arguments:
- `visualization_id`: Visualization ID
- `x_dune_api_key`: API Key for the service
- `body`: UpdateVisualizationRequest
*/
    pub async fn putv1_visualizations_visualizationid<'a>(
        &'a self,
        visualization_id: i64,
        x_dune_api_key: &'a str,
        body: &'a types::ModelsUpdateVisualizationRequest,
    ) -> Result<ResponseValue<types::ModelsUpdateVisualizationResponse>, Error<()>> {
        let url = format!(
            "{}/v1/visualizations/{}", self.baseurl, encode_path(& visualization_id
            .to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(2usize);
        header_map
            .append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(Self::api_version()),
            );
        header_map.append("X-Dune-Api-Key", x_dune_api_key.to_string().try_into()?);
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
            operation_id: "putv1_visualizations_visualizationid",
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
    /**Delete a visualization

Deletes a visualization by its ID.

Sends a `DELETE` request to `/v1/visualizations/{visualization_id}`

Arguments:
- `visualization_id`: Visualization ID
- `x_dune_api_key`: API Key for the service
*/
    pub async fn deletev1_visualizations_visualizationid<'a>(
        &'a self,
        visualization_id: i64,
        x_dune_api_key: &'a str,
    ) -> Result<ResponseValue<types::ModelsDeleteVisualizationResponse>, Error<()>> {
        let url = format!(
            "{}/v1/visualizations/{}", self.baseurl, encode_path(& visualization_id
            .to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(2usize);
        header_map
            .append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(Self::api_version()),
            );
        header_map.append("X-Dune-Api-Key", x_dune_api_key.to_string().try_into()?);
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
            operation_id: "deletev1_visualizations_visualizationid",
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
