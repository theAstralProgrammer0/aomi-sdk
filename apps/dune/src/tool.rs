use aomi_ext::dune::DuneClient;
use aomi_sdk::*;
use aomi_sdk::schemars::JsonSchema;
use serde::Deserialize;
use serde_json::Value;

#[derive(Clone, Default)]
pub(crate) struct DuneApp;

fn resolve_dune_api_key(api_key: Option<&str>) -> Result<String, String> {
    resolve_secret_value(
        api_key,
        "DUNE_API_KEY",
        "[dune] missing api_key argument and DUNE_API_KEY environment variable",
    )
}

// ============================================================================
// Tool structs & arg types
// ============================================================================

pub(crate) struct ExecuteQuery;

#[derive(Debug, Deserialize, JsonSchema)]
pub(crate) struct ExecuteQueryArgs {
    /// Dune API key for authentication.
    pub api_key: Option<String>,
    /// Numeric Dune query ID (from the dashboard URL).
    pub query_id: u64,
    /// Optional JSON object of query parameters that map to {{param}} placeholders in the SQL.
    #[serde(default)]
    pub query_parameters: Option<Value>,
}

pub(crate) struct GetExecutionStatus;

#[derive(Debug, Deserialize, JsonSchema)]
pub(crate) struct GetExecutionStatusArgs {
    /// Dune API key for authentication.
    pub api_key: Option<String>,
    /// Execution ID returned by execute_query.
    pub execution_id: String,
}

pub(crate) struct GetExecutionResults;

#[derive(Debug, Deserialize, JsonSchema)]
pub(crate) struct GetExecutionResultsArgs {
    /// Dune API key for authentication.
    pub api_key: Option<String>,
    /// Execution ID returned by execute_query.
    pub execution_id: String,
    /// Maximum number of rows to return.
    #[serde(default)]
    pub limit: Option<u64>,
    /// Number of rows to skip (for pagination).
    #[serde(default)]
    pub offset: Option<u64>,
}

pub(crate) struct GetQueryResults;

#[derive(Debug, Deserialize, JsonSchema)]
pub(crate) struct GetQueryResultsArgs {
    /// Dune API key for authentication.
    pub api_key: Option<String>,
    /// Numeric Dune query ID.
    pub query_id: u64,
    /// Maximum number of rows to return.
    #[serde(default)]
    pub limit: Option<u64>,
    /// Number of rows to skip (for pagination).
    #[serde(default)]
    pub offset: Option<u64>,
}

// ============================================================================
// Tool impls
// ============================================================================

impl DynAomiTool for ExecuteQuery {
    type App = DuneApp;
    type Args = ExecuteQueryArgs;
    const NAME: &'static str = "execute_query";
    const DESCRIPTION: &'static str = "Execute a Dune SQL query by its numeric ID. Returns an execution_id to poll for status and results.";

    fn run(_app: &DuneApp, args: Self::Args, _ctx: DynToolCallCtx) -> Result<Value, String> {
        let api_key = resolve_dune_api_key(args.api_key.as_deref())?;
        let client = DuneClient::new(&api_key)?;
        client.execute_query(args.query_id, args.query_parameters.as_ref())
    }
}

impl DynAomiTool for GetExecutionStatus {
    type App = DuneApp;
    type Args = GetExecutionStatusArgs;
    const NAME: &'static str = "get_execution_status";
    const DESCRIPTION: &'static str = "Poll the status of a running Dune query execution. Returns state (e.g. QUERY_STATE_COMPLETED).";

    fn run(_app: &DuneApp, args: Self::Args, _ctx: DynToolCallCtx) -> Result<Value, String> {
        let api_key = resolve_dune_api_key(args.api_key.as_deref())?;
        let client = DuneClient::new(&api_key)?;
        client.get_execution_status(&args.execution_id)
    }
}

impl DynAomiTool for GetExecutionResults {
    type App = DuneApp;
    type Args = GetExecutionResultsArgs;
    const NAME: &'static str = "get_execution_results";
    const DESCRIPTION: &'static str = "Fetch result rows from a completed Dune query execution. Supports limit/offset pagination.";

    fn run(_app: &DuneApp, args: Self::Args, _ctx: DynToolCallCtx) -> Result<Value, String> {
        let api_key = resolve_dune_api_key(args.api_key.as_deref())?;
        let client = DuneClient::new(&api_key)?;
        client.get_execution_results(&args.execution_id, args.limit, args.offset)
    }
}

impl DynAomiTool for GetQueryResults {
    type App = DuneApp;
    type Args = GetQueryResultsArgs;
    const NAME: &'static str = "get_query_results";
    const DESCRIPTION: &'static str = "Get the latest cached results for a Dune query by its numeric ID. Useful for community queries without re-executing.";

    fn run(_app: &DuneApp, args: Self::Args, _ctx: DynToolCallCtx) -> Result<Value, String> {
        let api_key = resolve_dune_api_key(args.api_key.as_deref())?;
        let client = DuneClient::new(&api_key)?;
        client.get_query_results(args.query_id, args.limit, args.offset)
    }
}
