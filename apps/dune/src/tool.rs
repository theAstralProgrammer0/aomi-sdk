//! Curated tool layer for Dune Analytics. Hand-written from the generated
//! client in `aomi_ext::dune` — see ext/specs/dune.yaml for the full surface.
//!
//! Designed for the user story: run SQL queries on Dune, fetch results, and
//! explore curated datasets/dashboards. The 44 mechanical tools from
//! `aomi-build gen-tool` were collapsed into 8 user-centric ones:
//!
//!   * `dune_run_query`           — composite execute → poll → fetch
//!   * `dune_get_latest_results`  — fetch results without re-executing
//!   * `dune_get_execution_status`— manual polling for advanced async use
//!   * `dune_list_my_queries`     — queries owned by the API key
//!   * `dune_search_datasets`     — text + filter search
//!   * `dune_search_by_contract`  — datasets containing a contract address
//!   * `dune_get_dataset`         — full schema/sample for one dataset
//!   * `dune_get_dashboard`       — full dashboard contents by id

use aomi_ext::dune::Client as DuneClient;
use aomi_ext::dune::types::{
    ModelsSearchDatasetsByContractAddressRequest, ModelsSearchDatasetsRequest,
};
use aomi_sdk::*;
use aomi_sdk::schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::time::{Duration, Instant};

#[derive(Clone, Default)]
pub(crate) struct DuneApp;

const BASE_URL: &str = "https://api.dune.com/api";

// ============================================================================
// Helpers
// ============================================================================

fn ok<T: Serialize>(value: T) -> Result<Value, String> {
    let value = serde_json::to_value(value).map_err(|e| format!("[dune] serialize: {e}"))?;
    Ok(match value {
        Value::Object(mut m) => {
            m.insert("source".into(), Value::String("dune".into()));
            Value::Object(m)
        }
        other => json!({ "source": "dune", "data": other }),
    })
}

fn rt() -> Result<tokio::runtime::Runtime, String> {
    tokio::runtime::Runtime::new().map_err(|e| format!("[dune] runtime: {e}"))
}

fn resolve_key(arg: Option<&str>) -> Result<String, String> {
    resolve_secret_value(
        arg,
        "DUNE_API_KEY",
        "[dune] missing api_key argument and DUNE_API_KEY env var",
    )
}

// ============================================================================
// Tool 1: run_query — composite execute → poll → fetch
// ============================================================================

pub(crate) struct RunQuery;

#[derive(Debug, Deserialize, JsonSchema)]
pub(crate) struct RunQueryArgs {
    /// API credential (falls back to env var DUNE_API_KEY).
    pub api_key: Option<String>,
    /// Numeric Dune query ID (visible in the query URL on dune.com, e.g. 1234567).
    pub query_id: i64,
    /// Optional URL-encoded query parameters (e.g. "param1=value1&param2=value2").
    #[serde(default)]
    pub query_parameters: Option<String>,
    /// Maximum seconds to wait for the query to finish before erroring out (default 60).
    #[serde(default)]
    pub max_wait_seconds: Option<u64>,
}

impl DynAomiTool for RunQuery {
    type App = DuneApp;
    type Args = RunQueryArgs;
    const NAME: &'static str = "dune_run_query";
    const DESCRIPTION: &'static str = "Execute a saved Dune SQL query, wait for it to finish, and return the result rows. Use this when the user wants to run analytics on Dune. Polls execution status every 2 seconds until QUERY_STATE_COMPLETED, then fetches the results in one call.";

    fn run(_app: &DuneApp, args: Self::Args, _ctx: DynToolCallCtx) -> Result<Value, String> {
        let api_key = resolve_key(args.api_key.as_deref())?;
        let max_wait = Duration::from_secs(args.max_wait_seconds.unwrap_or(60));
        let query_id = args.query_id;
        let query_parameters = args.query_parameters.clone();
        let runtime = rt()?;
        runtime.block_on(async move {
            let client = DuneClient::new(BASE_URL);

            let exec = client
                .postv1_query_queryid_execute(
                    query_id,
                    None,
                    None,
                    query_parameters.as_deref(),
                    api_key.as_str(),
                )
                .await
                .map_err(|e| format!("[dune] execute query {query_id}: {e}"))?
                .into_inner();
            let execution_id = exec
                .execution_id
                .clone()
                .ok_or_else(|| "[dune] execute response missing execution_id".to_string())?;

            let deadline = Instant::now() + max_wait;
            let mut last_state;
            loop {
                let s = client
                    .getv1_execution_executionid_status(
                        execution_id.as_str(),
                        None,
                        api_key.as_str(),
                    )
                    .await
                    .map_err(|e| format!("[dune] status {execution_id}: {e}"))?
                    .into_inner();
                last_state = s.state.clone().unwrap_or_default();
                match last_state.as_str() {
                    "QUERY_STATE_COMPLETED" => break,
                    "QUERY_STATE_FAILED" | "QUERY_STATE_CANCELLED" => {
                        return Err(format!(
                            "[dune] query {query_id} ended in state {last_state}"
                        ));
                    }
                    _ => {}
                }
                if Instant::now() >= deadline {
                    return Err(format!(
                        "[dune] timed out after {}s waiting for execution {execution_id} (last state: {last_state})",
                        max_wait.as_secs()
                    ));
                }
                tokio::time::sleep(Duration::from_secs(2)).await;
            }

            let results = client
                .getv1_execution_executionid_results(
                    execution_id.as_str(),
                    None,
                    None,
                    None,
                    None,
                    None,
                    None,
                    None,
                    None,
                    None,
                    api_key.as_str(),
                )
                .await
                .map_err(|e| format!("[dune] fetch results {execution_id}: {e}"))?
                .into_inner();
            ok(results)
        })
    }
}

// ============================================================================
// Tool 2: get_latest_results — fetch existing results without re-executing
// ============================================================================

pub(crate) struct GetLatestResults;

#[derive(Debug, Deserialize, JsonSchema)]
pub(crate) struct GetLatestResultsArgs {
    pub api_key: Option<String>,
    /// Dune query ID. Returns the most recent execution's results without
    /// triggering a new run — useful for queries that auto-refresh.
    pub query_id: String,
    /// Limit rows returned (default Dune-side: 1000).
    #[serde(default)]
    pub limit: Option<i64>,
    /// Offset for paging.
    #[serde(default)]
    pub offset: Option<i64>,
}

impl DynAomiTool for GetLatestResults {
    type App = DuneApp;
    type Args = GetLatestResultsArgs;
    const NAME: &'static str = "dune_get_latest_results";
    const DESCRIPTION: &'static str = "Get the most recent results of a Dune query without re-executing it. Use when the query refreshes on a schedule (free-tier and many paid queries) and the user just wants the cached output. Cheaper than running the query.";

    fn run(_app: &DuneApp, args: Self::Args, _ctx: DynToolCallCtx) -> Result<Value, String> {
        let api_key = resolve_key(args.api_key.as_deref())?;
        let runtime = rt()?;
        runtime.block_on(async move {
            let client = DuneClient::new(BASE_URL);
            let r = client
                .getv1_query_queryid_results(
                    args.query_id.as_str(),
                    None,
                    None,
                    None,
                    None,
                    None,
                    args.limit,
                    args.offset,
                    None,
                    None,
                    api_key.as_str(),
                )
                .await
                .map_err(|e| format!("[dune] latest results {}: {e}", args.query_id))?
                .into_inner();
            ok(r)
        })
    }
}

// ============================================================================
// Tool 3: get_execution_status — manual polling for advanced use
// ============================================================================

pub(crate) struct GetExecutionStatus;

#[derive(Debug, Deserialize, JsonSchema)]
pub(crate) struct GetExecutionStatusArgs {
    pub api_key: Option<String>,
    /// Execution ID returned by an earlier query execution.
    pub execution_id: String,
}

impl DynAomiTool for GetExecutionStatus {
    type App = DuneApp;
    type Args = GetExecutionStatusArgs;
    const NAME: &'static str = "dune_get_execution_status";
    const DESCRIPTION: &'static str = "Check the state of a Dune query execution by its execution_id. Returns one of QUERY_STATE_PENDING, QUERY_STATE_EXECUTING, QUERY_STATE_COMPLETED, QUERY_STATE_FAILED, QUERY_STATE_CANCELLED, plus timing/cost. Use only when you need to inspect an in-flight execution started elsewhere — `dune_run_query` waits for completion itself.";

    fn run(_app: &DuneApp, args: Self::Args, _ctx: DynToolCallCtx) -> Result<Value, String> {
        let api_key = resolve_key(args.api_key.as_deref())?;
        let runtime = rt()?;
        runtime.block_on(async move {
            let client = DuneClient::new(BASE_URL);
            let s = client
                .getv1_execution_executionid_status(
                    args.execution_id.as_str(),
                    None,
                    api_key.as_str(),
                )
                .await
                .map_err(|e| format!("[dune] status {}: {e}", args.execution_id))?
                .into_inner();
            ok(s)
        })
    }
}

// ============================================================================
// Tool 4: list_my_queries — queries owned by the API key
// ============================================================================

pub(crate) struct ListMyQueries;

#[derive(Debug, Deserialize, JsonSchema)]
pub(crate) struct ListMyQueriesArgs {
    pub api_key: Option<String>,
    /// Max queries returned (default 100).
    #[serde(default)]
    pub limit: Option<i64>,
    /// Offset for paging.
    #[serde(default)]
    pub offset: Option<i64>,
}

impl DynAomiTool for ListMyQueries {
    type App = DuneApp;
    type Args = ListMyQueriesArgs;
    const NAME: &'static str = "dune_list_my_queries";
    const DESCRIPTION: &'static str = "List Dune SQL queries owned by the account tied to the API key. Returns IDs, names, and metadata you can pass to `dune_run_query` or `dune_get_latest_results`. Use when the user asks 'what queries do I have' or wants to discover their saved queries.";

    fn run(_app: &DuneApp, args: Self::Args, _ctx: DynToolCallCtx) -> Result<Value, String> {
        let api_key = resolve_key(args.api_key.as_deref())?;
        let runtime = rt()?;
        runtime.block_on(async move {
            let client = DuneClient::new(BASE_URL);
            let r = client
                .getv1_queries(None, args.limit, args.offset, api_key.as_str())
                .await
                .map_err(|e| format!("[dune] list queries: {e}"))?
                .into_inner();
            ok(r)
        })
    }
}

// ============================================================================
// Tool 5: search_datasets — text + filter search across the catalog
// ============================================================================

pub(crate) struct SearchDatasets;

#[derive(Debug, Deserialize, JsonSchema)]
pub(crate) struct SearchDatasetsArgs {
    pub api_key: Option<String>,
    /// Filter by blockchain slugs (e.g. ["ethereum", "arbitrum"]).
    #[serde(default)]
    pub blockchains: Vec<String>,
    /// Filter by category slugs (e.g. ["dex", "lending"]).
    #[serde(default)]
    pub categories: Vec<String>,
    /// Max datasets returned (default 50).
    #[serde(default)]
    pub limit: Option<i64>,
}

impl DynAomiTool for SearchDatasets {
    type App = DuneApp;
    type Args = SearchDatasetsArgs;
    const NAME: &'static str = "dune_search_datasets";
    const DESCRIPTION: &'static str = "Search Dune's curated dataset catalog by blockchain and/or category filters. Returns a list of dataset summaries (slug, name, blockchain, category). Use when the user wants to discover what data is available before running a query.";

    fn run(_app: &DuneApp, args: Self::Args, _ctx: DynToolCallCtx) -> Result<Value, String> {
        let api_key = resolve_key(args.api_key.as_deref())?;
        let runtime = rt()?;
        runtime.block_on(async move {
            let client = DuneClient::new(BASE_URL);
            let body = ModelsSearchDatasetsRequest {
                blockchains: args.blockchains,
                categories: args.categories,
                limit: args.limit,
                ..Default::default()
            };
            let r = client
                .postv1_datasets_search(None, api_key.as_str(), &body)
                .await
                .map_err(|e| format!("[dune] search datasets: {e}"))?
                .into_inner();
            ok(r)
        })
    }
}

// ============================================================================
// Tool 6: search_by_contract — datasets that decode a specific contract
// ============================================================================

pub(crate) struct SearchByContract;

#[derive(Debug, Deserialize, JsonSchema)]
pub(crate) struct SearchByContractArgs {
    pub api_key: Option<String>,
    /// Contract address (any case; chains may differ on checksumming).
    pub contract_address: String,
    /// Optional blockchain filter (e.g. ["ethereum"]).
    #[serde(default)]
    pub blockchains: Vec<String>,
    /// Max datasets returned.
    #[serde(default)]
    pub limit: Option<i64>,
}

impl DynAomiTool for SearchByContract {
    type App = DuneApp;
    type Args = SearchByContractArgs;
    const NAME: &'static str = "dune_search_by_contract";
    const DESCRIPTION: &'static str = "Find Dune datasets (decoded events, function calls, etc.) for a specific contract address. Use when the user has a contract and wants to know what tables Dune already decodes for it.";

    fn run(_app: &DuneApp, args: Self::Args, _ctx: DynToolCallCtx) -> Result<Value, String> {
        let api_key = resolve_key(args.api_key.as_deref())?;
        let runtime = rt()?;
        runtime.block_on(async move {
            let client = DuneClient::new(BASE_URL);
            let body = ModelsSearchDatasetsByContractAddressRequest {
                blockchains: args.blockchains,
                contract_address: args.contract_address,
                include_schema: None,
                limit: args.limit,
                offset: None,
            };
            let r = client
                .postv1_datasets_search_by_contract(None, api_key.as_str(), &body)
                .await
                .map_err(|e| format!("[dune] search by contract: {e}"))?
                .into_inner();
            ok(r)
        })
    }
}

// ============================================================================
// Tool 7: get_dataset — full schema and sample for one dataset slug
// ============================================================================

pub(crate) struct GetDataset;

#[derive(Debug, Deserialize, JsonSchema)]
pub(crate) struct GetDatasetArgs {
    pub api_key: Option<String>,
    /// Dataset slug (e.g. "ethereum.transactions"). Get one from `dune_search_datasets`.
    pub slug: String,
}

impl DynAomiTool for GetDataset {
    type App = DuneApp;
    type Args = GetDatasetArgs;
    const NAME: &'static str = "dune_get_dataset";
    const DESCRIPTION: &'static str = "Get full metadata for a Dune dataset by slug — column names/types, descriptions, and a sample row. Use after `dune_search_datasets` when the user picks a dataset and wants to know its schema before writing a query against it.";

    fn run(_app: &DuneApp, args: Self::Args, _ctx: DynToolCallCtx) -> Result<Value, String> {
        let api_key = resolve_key(args.api_key.as_deref())?;
        let runtime = rt()?;
        runtime.block_on(async move {
            let client = DuneClient::new(BASE_URL);
            let r = client
                .getv1_datasets_slug(args.slug.as_str(), None, api_key.as_str())
                .await
                .map_err(|e| format!("[dune] get dataset {}: {e}", args.slug))?
                .into_inner();
            ok(r)
        })
    }
}

// ============================================================================
// Tool 8: get_dashboard — full dashboard contents by id
// ============================================================================

pub(crate) struct GetDashboard;

#[derive(Debug, Deserialize, JsonSchema)]
pub(crate) struct GetDashboardArgs {
    pub api_key: Option<String>,
    /// Dashboard ID (numeric, from the dashboard URL).
    pub dashboard_id: i64,
}

impl DynAomiTool for GetDashboard {
    type App = DuneApp;
    type Args = GetDashboardArgs;
    const NAME: &'static str = "dune_get_dashboard";
    const DESCRIPTION: &'static str = "Fetch a Dune dashboard by ID — returns its title, description, and the list of constituent queries/visualizations. Use when the user references a specific dashboard and wants to inspect its contents.";

    fn run(_app: &DuneApp, args: Self::Args, _ctx: DynToolCallCtx) -> Result<Value, String> {
        let api_key = resolve_key(args.api_key.as_deref())?;
        let runtime = rt()?;
        runtime.block_on(async move {
            let client = DuneClient::new(BASE_URL);
            let r = client
                .getv1_dashboards_dashboardid(args.dashboard_id, api_key.as_str())
                .await
                .map_err(|e| format!("[dune] get dashboard {}: {e}", args.dashboard_id))?
                .into_inner();
            ok(r)
        })
    }
}
