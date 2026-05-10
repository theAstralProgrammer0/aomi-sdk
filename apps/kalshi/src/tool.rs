//! Curated tool layer for Kalshi (via the Simmer SDK proxy).
//!
//! Built on the progenitor-generated client at `aomi_ext::kalshi::Client` —
//! see `ext/specs/kalshi.yaml` for the underlying surface. No HMAC needed;
//! auth is a Bearer token (`sk_…`) passed in the `Authorization` header.
//!
//! Designed for the user story: trade Kalshi prediction markets through the
//! Simmer SDK — register an agent, browse & import markets, inspect context
//! before trading, place buys/sells, and view positions/portfolio.
//!
//! 9 curated tools:
//!   * simmer_register             — first-time agent setup
//!   * simmer_status               — agent state, balances, claim_url
//!   * simmer_briefing             — one-shot dashboard
//!   * search_simmer_markets       — discover importable Kalshi markets
//!   * import_kalshi_market        — Kalshi URL → Simmer market_id UUID
//!   * fetch_simmer_market_context — pre-trade warnings/slippage/fees
//!   * simmer_place_order          — place a buy/sell
//!   * simmer_get_positions        — open positions per venue
//!   * simmer_get_portfolio        — cash + positions value + PnL
//!
//! Response trimming policy: every endpoint's response schema is curated in
//! `ext/specs/kalshi.yaml` to surface only the fields the LLM acts on.
//! Each tool body forwards the typed response directly — no `.get("foo")`
//! walking, no `json!({...})` re-projection. To re-add a field the live API
//! returns, edit the spec and `aomi-build gen-client kalshi --shared --force`.

use aomi_ext::kalshi::Client as SimmerClient;
use aomi_ext::kalshi::types::{ImportKalshiMarketRequest, RegisterAgentRequest, TradeRequest};
use aomi_sdk::*;
use aomi_sdk::schemars::JsonSchema;
use chrono::Local;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

const BASE_URL: &str = "https://api.simmer.markets";

#[derive(Clone, Default)]
pub(crate) struct KalshiApp;

pub(crate) fn build_preamble() -> String {
    let now = Local::now();
    format!(
        r#"## Role
You specialize in Kalshi prediction markets via the Simmer SDK.

## Current Date
Today is {} ({}). Use this exact date when interpreting relative terms like 'today', 'tomorrow', and 'yesterday'.

## Simmer SDK
Simmer SDK (simmer.markets) provides the trading API used by this app.

Venues: 'sim' = sandbox ($SIM, no real money, no KYC). 'kalshi' = live Kalshi trading after the agent is claimed and the user's Kalshi wallet/account setup is complete. Default to sim unless the user explicitly wants a live Kalshi trade.

Setup: simmer_register -> get api_key + claim_url -> user runs /apikey simmer <key> -> user visits claim_url to complete identity verification and link the agent. When presenting the claim link, always tell the user: 'Visit this link to verify your identity and claim your agent. This is where Simmer handles account linking and unlocks live Kalshi trading.'

Discovery flow: search_simmer_markets returns importable Kalshi markets, not Simmer UUIDs. Before trading, call import_kalshi_market with the Kalshi URL. That returns the Simmer market_id UUID required for fetch_simmer_market_context and simmer_place_order.

Trading flow: search_simmer_markets -> import_kalshi_market -> fetch_simmer_market_context -> simmer_place_order. Always check context warnings before trading. The reasoning field is public on the user's Simmer profile, so write a real thesis.

Live Kalshi trading uses venue='kalshi' and currently requires the user's live wallet/account setup through Simmer. Sandbox trading uses venue='sim'.

Compliance: Aomi is only an interface. We do not hold funds. KYC, custody, and compliance are handled by Simmer and the underlying Kalshi integration. Users are responsible for ensuring prediction market trading is legal in their jurisdiction.

IMPORTANT -- show this disclaimer on registration (before claim link), on first live Kalshi trade (venue=kalshi), and in /apikey simmer response:
'Aomi is an interface to Simmer (simmer.markets) -- we do not hold your funds. KYC and compliance are handled by Simmer and the underlying Kalshi integration. You are responsible for ensuring prediction market trading is legal in your jurisdiction. By claiming your agent you agree to Simmer ToS.'
"#,
        now.format("%Y-%m-%d"),
        now.format("%Z")
    )
}

// ============================================================================
// Helpers
// ============================================================================

fn ok<T: Serialize>(value: T) -> Result<Value, String> {
    let value = serde_json::to_value(value)
        .map_err(|e| format!("[kalshi] failed to serialize response: {e}"))?;
    Ok(match value {
        Value::Object(mut map) => {
            map.insert("source".to_string(), Value::String("kalshi".to_string()));
            Value::Object(map)
        }
        other => json!({ "source": "kalshi", "data": other }),
    })
}

fn rt() -> Result<tokio::runtime::Runtime, String> {
    tokio::runtime::Runtime::new().map_err(|e| format!("[kalshi] runtime: {e}"))
}

fn resolve_simmer_api_key(api_key: Option<&str>) -> Result<String, String> {
    resolve_secret_value(
        api_key,
        "SIMMER_API_KEY",
        "[simmer] missing api_key argument and SIMMER_API_KEY environment variable",
    )
}

fn bearer(api_key: &str) -> String {
    format!("Bearer {api_key}")
}

pub(crate) fn parse_venue(venue: &str) -> Result<String, String> {
    match venue.to_lowercase().as_str() {
        "sim" | "sandbox" | "simmer" => Ok("sim".to_string()),
        "kalshi" => Ok("kalshi".to_string()),
        other => Err(format!("Unknown venue: {other}. Use sim or kalshi.")),
    }
}

// ============================================================================
// Tool 1: SimmerRegister
// ============================================================================

pub(crate) struct SimmerRegister;

#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub(crate) struct SimmerRegisterArgs {
    /// Agent name (e.g., "aomi-kalshi")
    name: String,
    /// Brief description of what the agent does
    description: Option<String>,
}

impl DynAomiTool for SimmerRegister {
    type App = KalshiApp;
    type Args = SimmerRegisterArgs;
    const NAME: &'static str = "simmer_register";
    const DESCRIPTION: &'static str =
        "Use when the user has no Simmer API key yet and wants to start trading Kalshi markets. Returns an api_key (must be saved with /apikey simmer <key>), a claim_url for identity verification, and a sandbox starting balance. One-time setup.";

    fn run(_app: &KalshiApp, args: Self::Args, _ctx: DynToolCallCtx) -> Result<Value, String> {
        let body = RegisterAgentRequest {
            name: args.name,
            description: args.description,
        };
        let runtime = rt()?;
        let result = runtime
            .block_on(async move {
                let client = SimmerClient::new(BASE_URL);
                client.register_agent(&body).await
            })
            .map_err(|e| format!("[simmer] register_agent: {e}"))?
            .into_inner();
        ok(result)
    }
}

// ============================================================================
// Tool 2: SimmerStatus
// ============================================================================

pub(crate) struct SimmerStatus;

#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub(crate) struct SimmerStatusArgs {
    /// Simmer API key (sk_...)
    api_key: Option<String>,
}

impl DynAomiTool for SimmerStatus {
    type App = KalshiApp;
    type Args = SimmerStatusArgs;
    const NAME: &'static str = "simmer_status";
    const DESCRIPTION: &'static str = "Use when the user asks 'is my agent set up?' or 'can I trade live Kalshi yet?'. Returns agent state, $SIM and USD balances, claim_url, and live_trading_enabled flag.";

    fn run(_app: &KalshiApp, args: Self::Args, _ctx: DynToolCallCtx) -> Result<Value, String> {
        let api_key = resolve_simmer_api_key(args.api_key.as_deref())?;
        let auth = bearer(&api_key);
        let runtime = rt()?;
        let status = runtime
            .block_on(async move {
                let client = SimmerClient::new(BASE_URL);
                client.get_agent_status(auth.as_str()).await
            })
            .map_err(|e| format!("[simmer] get_agent_status: {e}"))?
            .into_inner();
        ok(status)
    }
}

// ============================================================================
// Tool 3: SimmerBriefing
// ============================================================================

pub(crate) struct SimmerBriefing;

#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub(crate) struct SimmerBriefingArgs {
    api_key: Option<String>,
    /// ISO timestamp to get changes since (optional, defaults to backend default)
    since: Option<String>,
}

impl DynAomiTool for SimmerBriefing {
    type App = KalshiApp;
    type Args = SimmerBriefingArgs;
    const NAME: &'static str = "simmer_briefing";
    const DESCRIPTION: &'static str = "Use when the user opens a session or asks 'what's new?' / 'how am I doing?'. Returns a one-shot dashboard: portfolio snapshot, current positions, fresh opportunities, risk alerts, and performance. Optional `since` ISO timestamp to scope to changes since a moment.";

    fn run(_app: &KalshiApp, args: Self::Args, _ctx: DynToolCallCtx) -> Result<Value, String> {
        let api_key = resolve_simmer_api_key(args.api_key.as_deref())?;
        let auth = bearer(&api_key);
        let since = args.since;
        let runtime = rt()?;
        let briefing = runtime
            .block_on(async move {
                let client = SimmerClient::new(BASE_URL);
                client.get_briefing(since.as_deref(), auth.as_str()).await
            })
            .map_err(|e| format!("[simmer] get_briefing: {e}"))?
            .into_inner();
        ok(briefing)
    }
}

// ============================================================================
// Tool 4: SearchSimmerMarkets
// ============================================================================

pub(crate) struct SearchSimmerMarkets;

#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub(crate) struct SearchSimmerMarketsArgs {
    api_key: Option<String>,
    /// Optional free-text search query
    query: Option<String>,
    /// Optional minimum external volume filter (default in API docs: 10000)
    min_volume: Option<f64>,
    /// Maximum number of markets to return (default: 20)
    limit: Option<u32>,
}

impl DynAomiTool for SearchSimmerMarkets {
    type App = KalshiApp;
    type Args = SearchSimmerMarketsArgs;
    const NAME: &'static str = "search_simmer_markets";
    const DESCRIPTION: &'static str = "Use when the user wants to discover Kalshi markets to trade (e.g. 'find election markets'). Returns importable markets with Kalshi URLs — NOT Simmer market_ids. Next step: call import_kalshi_market with one of the returned URLs.";

    fn run(_app: &KalshiApp, args: Self::Args, _ctx: DynToolCallCtx) -> Result<Value, String> {
        let api_key = resolve_simmer_api_key(args.api_key.as_deref())?;
        let auth = bearer(&api_key);
        let limit_str = args.limit.map(|v| v.to_string());
        let min_vol_str = args.min_volume.map(|v| v.to_string());
        let q = args.query;
        let runtime = rt()?;
        let result = runtime
            .block_on(async move {
                let client = SimmerClient::new(BASE_URL);
                client
                    .list_importable_kalshi_markets(
                        limit_str.as_deref(),
                        min_vol_str.as_deref(),
                        q.as_deref(),
                        "kalshi",
                        auth.as_str(),
                    )
                    .await
            })
            .map_err(|e| format!("[simmer] list_importable_kalshi_markets: {e}"))?
            .into_inner();
        ok(result)
    }
}

// ============================================================================
// Tool 5: ImportKalshiMarket
// ============================================================================

pub(crate) struct ImportKalshiMarket;

#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub(crate) struct ImportKalshiMarketArgs {
    api_key: Option<String>,
    /// Kalshi market URL returned by search_simmer_markets
    kalshi_url: String,
}

impl DynAomiTool for ImportKalshiMarket {
    type App = KalshiApp;
    type Args = ImportKalshiMarketArgs;
    const NAME: &'static str = "import_kalshi_market";
    const DESCRIPTION: &'static str = "Use after search_simmer_markets (or when the user pastes a Kalshi URL) to register the market with Simmer. Returns the Simmer market_id UUID — required for fetch_simmer_market_context and simmer_place_order.";

    fn run(_app: &KalshiApp, args: Self::Args, _ctx: DynToolCallCtx) -> Result<Value, String> {
        let api_key = resolve_simmer_api_key(args.api_key.as_deref())?;
        let auth = bearer(&api_key);
        let body = ImportKalshiMarketRequest {
            kalshi_url: args.kalshi_url,
        };
        let runtime = rt()?;
        let result = runtime
            .block_on(async move {
                let client = SimmerClient::new(BASE_URL);
                client.import_kalshi_market(auth.as_str(), &body).await
            })
            .map_err(|e| format!("[simmer] import_kalshi_market: {e}"))?
            .into_inner();
        ok(result)
    }
}

// ============================================================================
// Tool 6: FetchSimmerMarketContext
// ============================================================================

pub(crate) struct FetchSimmerMarketContext;

#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub(crate) struct FetchSimmerMarketContextArgs {
    api_key: Option<String>,
    /// Simmer market ID to analyze before trading
    market_id: String,
}

impl DynAomiTool for FetchSimmerMarketContext {
    type App = KalshiApp;
    type Args = FetchSimmerMarketContextArgs;
    const NAME: &'static str = "fetch_simmer_market_context";
    const DESCRIPTION: &'static str = "Use immediately before simmer_place_order to inspect a market. Returns warnings, slippage estimate, fees, time-to-resolution, and resolution criteria. Always surface warnings to the user before placing a trade.";

    fn run(_app: &KalshiApp, args: Self::Args, _ctx: DynToolCallCtx) -> Result<Value, String> {
        let api_key = resolve_simmer_api_key(args.api_key.as_deref())?;
        let auth = bearer(&api_key);
        let market_id = args.market_id;
        let runtime = rt()?;
        let context = runtime
            .block_on(async move {
                let client = SimmerClient::new(BASE_URL);
                client
                    .get_market_context(market_id.as_str(), auth.as_str())
                    .await
            })
            .map_err(|e| format!("[simmer] get_market_context: {e}"))?
            .into_inner();
        ok(context)
    }
}

// ============================================================================
// Tool 7: SimmerPlaceOrder
// ============================================================================

pub(crate) struct SimmerPlaceOrder;

#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub(crate) struct SimmerPlaceOrderArgs {
    api_key: Option<String>,
    /// Simmer market ID to trade on
    market_id: String,
    /// Outcome to bet on: "yes" or "no"
    side: String,
    /// Amount in USD (for buys). Provide either amount or shares.
    amount: Option<f64>,
    /// Shares quantity (required for sells; optional for buys). Provide either amount or shares.
    shares: Option<f64>,
    /// Trading venue: sim (sandbox) or kalshi (live)
    venue: Option<String>,
    /// Action: "buy" or "sell" (default: buy)
    action: Option<String>,
    /// Validate without executing. Supported for real-money venues like kalshi.
    dry_run: Option<bool>,
    /// Your thesis for this trade -- displayed publicly on Simmer
    reasoning: Option<String>,
}

impl DynAomiTool for SimmerPlaceOrder {
    type App = KalshiApp;
    type Args = SimmerPlaceOrderArgs;
    const NAME: &'static str = "simmer_place_order";
    const DESCRIPTION: &'static str = "Use to place a buy or sell on a Kalshi market via Simmer. side is 'yes'/'no'; provide either amount (USD, buys only) or shares (required for sells). venue defaults to 'sim' (sandbox); pass 'kalshi' only for real-money trades. Set dry_run=true to validate first on live venues. The reasoning field is public on the user's Simmer profile — write a real thesis.";

    fn run(_app: &KalshiApp, args: Self::Args, _ctx: DynToolCallCtx) -> Result<Value, String> {
        let venue = args
            .venue
            .as_deref()
            .map(parse_venue)
            .transpose()?
            .unwrap_or_else(|| "sim".to_string());
        let action = args
            .action
            .as_deref()
            .unwrap_or("buy")
            .trim()
            .to_ascii_lowercase();

        if action != "buy" && action != "sell" {
            return Err("action must be buy or sell".to_string());
        }

        match (args.amount, args.shares) {
            (Some(_), Some(_)) => {
                return Err("Provide either amount or shares, not both.".to_string());
            }
            (None, None) => {
                return Err("Provide one of amount or shares.".to_string());
            }
            _ => {}
        }

        if action == "sell" && args.shares.is_none() {
            return Err("Sell orders must use shares, not amount.".to_string());
        }

        let api_key = resolve_simmer_api_key(args.api_key.as_deref())?;
        let auth = bearer(&api_key);
        let body = TradeRequest {
            action,
            amount: args.amount,
            dry_run: args.dry_run,
            market_id: args.market_id,
            reasoning: args.reasoning,
            shares: args.shares,
            side: args.side.to_lowercase(),
            source: "sdk:aomi".to_string(),
            venue,
        };
        let runtime = rt()?;
        let result = runtime
            .block_on(async move {
                let client = SimmerClient::new(BASE_URL);
                client.trade(auth.as_str(), &body).await
            })
            .map_err(|e| format!("[simmer] trade: {e}"))?
            .into_inner();
        ok(result)
    }
}

// ============================================================================
// Tool 8: SimmerGetPositions
// ============================================================================

pub(crate) struct SimmerGetPositions;

#[derive(Debug, Deserialize, JsonSchema)]
pub(crate) struct SimmerGetPositionsArgs {
    api_key: Option<String>,
    /// Optional venue filter: sim or kalshi
    venue: Option<String>,
}

impl DynAomiTool for SimmerGetPositions {
    type App = KalshiApp;
    type Args = SimmerGetPositionsArgs;
    const NAME: &'static str = "simmer_get_positions";
    const DESCRIPTION: &'static str = "Use when the user asks 'what am I holding?'. Returns open positions on a venue (defaults to 'sim'; pass 'kalshi' for live). Each position has shares, average_price, current_price, and pnl.";

    fn run(_app: &KalshiApp, args: Self::Args, _ctx: DynToolCallCtx) -> Result<Value, String> {
        let venue = args
            .venue
            .as_deref()
            .map(parse_venue)
            .transpose()?
            .unwrap_or_else(|| "sim".to_string());
        let api_key = resolve_simmer_api_key(args.api_key.as_deref())?;
        let auth = bearer(&api_key);
        let runtime = rt()?;
        let result = runtime
            .block_on(async move {
                let client = SimmerClient::new(BASE_URL);
                client.get_positions(Some(venue.as_str()), auth.as_str()).await
            })
            .map_err(|e| format!("[simmer] get_positions: {e}"))?
            .into_inner();
        ok(result)
    }
}

// ============================================================================
// Tool 9: SimmerGetPortfolio
// ============================================================================

pub(crate) struct SimmerGetPortfolio;

#[derive(Debug, Deserialize, JsonSchema)]
pub(crate) struct SimmerGetPortfolioArgs {
    api_key: Option<String>,
}

impl DynAomiTool for SimmerGetPortfolio {
    type App = KalshiApp;
    type Args = SimmerGetPortfolioArgs;
    const NAME: &'static str = "simmer_get_portfolio";
    const DESCRIPTION: &'static str =
        "Use when the user asks 'what's my account worth?'. Returns cash balance, currency, positions value, total value, and realized/unrealized PnL across the linked Kalshi account.";

    fn run(_app: &KalshiApp, args: Self::Args, _ctx: DynToolCallCtx) -> Result<Value, String> {
        let api_key = resolve_simmer_api_key(args.api_key.as_deref())?;
        let auth = bearer(&api_key);
        let runtime = rt()?;
        let portfolio = runtime
            .block_on(async move {
                let client = SimmerClient::new(BASE_URL);
                client.get_portfolio(auth.as_str()).await
            })
            .map_err(|e| format!("[simmer] get_portfolio: {e}"))?
            .into_inner();
        ok(portfolio)
    }
}
