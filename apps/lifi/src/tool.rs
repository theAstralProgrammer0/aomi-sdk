use aomi_ext::lifi::{
    LifiClient, amount_to_base_units, get_chain_info, get_token_address, get_token_decimals,
};
use aomi_sdk::*;
use aomi_sdk::schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

#[derive(Clone, Default)]
pub(crate) struct LifiApp;

fn ok<T: Serialize>(value: T) -> Result<Value, String> {
    let value = serde_json::to_value(value)
        .map_err(|e| format!("[lifi] failed to serialize response: {e}"))?;
    Ok(match value {
        Value::Object(mut map) => {
            map.insert("source".to_string(), Value::String("lifi".to_string()));
            Value::Object(map)
        }
        other => serde_json::json!({ "source": "lifi", "data": other }),
    })
}

// ============================================================================
// GetLifiSwapQuote
// ============================================================================

pub(crate) struct GetLifiSwapQuote;

#[derive(Debug, Deserialize, JsonSchema)]
pub(crate) struct GetLifiSwapQuoteArgs {
    /// Source chain
    pub(crate) chain: String,
    /// Destination chain (defaults to source chain for same-chain swaps)
    pub(crate) destination_chain: Option<String>,
    /// Sell token symbol or address
    pub(crate) sell_token: String,
    /// Buy token symbol or address
    pub(crate) buy_token: String,
    /// Amount to swap (human-readable units)
    pub(crate) amount: f64,
    /// Sender wallet address
    pub(crate) sender_address: String,
    /// Receiver wallet address (defaults to sender)
    pub(crate) receiver_address: Option<String>,
}

impl DynAomiTool for GetLifiSwapQuote {
    type App = LifiApp;
    type Args = GetLifiSwapQuoteArgs;
    const NAME: &'static str = "get_lifi_swap_quote";
    const DESCRIPTION: &'static str = "Get a LI.FI swap quote for same-chain or cross-chain swaps.";

    fn run(_app: &LifiApp, args: Self::Args, _ctx: DynToolCallCtx) -> Result<Value, String> {
        let client = LifiClient::new()?;
        let (chain_name, _) = get_chain_info(&args.chain)?;
        let from_decimals = get_token_decimals(chain_name, &args.sell_token);
        let amount_base_units = amount_to_base_units(args.amount, from_decimals)?;
        let from_addr = get_token_address(chain_name, &args.sell_token)?;

        let destination_chain = args
            .destination_chain
            .as_deref()
            .unwrap_or(args.chain.as_str());
        let (to_chain_name, _) = get_chain_info(destination_chain)?;
        let to_addr = get_token_address(to_chain_name, &args.buy_token)?;

        ok(client.get_quote(
            &args.chain,
            destination_chain,
            &from_addr,
            &to_addr,
            &amount_base_units,
            &args.sender_address,
            args.receiver_address.as_deref(),
        )?)
    }
}

// ============================================================================
// PlaceLifiOrder
// ============================================================================

pub(crate) struct PlaceLifiOrder;

#[derive(Debug, Deserialize, JsonSchema)]
pub(crate) struct PlaceLifiOrderArgs {
    /// Source chain
    pub(crate) chain: String,
    /// Destination chain (defaults to source chain)
    pub(crate) destination_chain: Option<String>,
    /// Sell token symbol or address
    pub(crate) sell_token: String,
    /// Buy token symbol or address
    pub(crate) buy_token: String,
    /// Sell amount (human-readable units)
    pub(crate) amount: f64,
    /// Sender/taker wallet address
    pub(crate) sender_address: String,
    /// Receiver wallet address
    pub(crate) receiver_address: Option<String>,
    /// Slippage tolerance as decimal (0.005 = 0.5%)
    pub(crate) slippage: Option<f64>,
}

impl DynAomiTool for PlaceLifiOrder {
    type App = LifiApp;
    type Args = PlaceLifiOrderArgs;
    const NAME: &'static str = "place_lifi_order";
    const DESCRIPTION: &'static str = "Get executable tx data via LI.FI. Returns approval_tx (if needed) and main_tx. Stage them with `stage_tx` using the raw-calldata path, verify them with `simulate_batch`, then finalize with `commit_tx`.";

    fn run(_app: &LifiApp, args: Self::Args, _ctx: DynToolCallCtx) -> Result<Value, String> {
        let client = LifiClient::new()?;
        let (chain_name, _) = get_chain_info(&args.chain)?;
        let from_decimals = get_token_decimals(chain_name, &args.sell_token);
        let amount_base_units = amount_to_base_units(args.amount, from_decimals)?;
        let from_addr = get_token_address(chain_name, &args.sell_token)?;

        let to_chain = args
            .destination_chain
            .clone()
            .unwrap_or_else(|| args.chain.clone());
        let (to_chain_name, _) = get_chain_info(&to_chain)?;
        let to_addr = get_token_address(to_chain_name, &args.buy_token)?;

        let payload = client.place_order(
            &args.chain,
            &to_chain,
            &from_addr,
            &to_addr,
            &amount_base_units,
            &args.sender_address,
            args.receiver_address.as_deref(),
            args.slippage,
        )?;

        let approval_tx = serde_json::to_value(&payload.approval_tx)
            .map_err(|e| format!("[lifi] failed to serialize approval_tx: {e}"))?;
        let main_tx = serde_json::to_value(&payload.main_tx)
            .map_err(|e| format!("[lifi] failed to serialize main_tx: {e}"))?;

        ok(json!({
            "payload": payload,
            "approval_tx": approval_tx,
            "main_tx": main_tx,
            "note": "If approval_tx is present, stage approval_tx with stage_tx first using data.raw, stage main_tx the same way, simulate the staged pending_tx_id list with simulate_batch, then call commit_tx once per staged tx. Do not re-encode LI.FI calldata.",
        }))
    }
}

// ============================================================================
// GetLifiBridgeQuote
// ============================================================================

pub(crate) struct GetLifiBridgeQuote;

#[derive(Debug, Deserialize, JsonSchema)]
pub(crate) struct GetLifiBridgeQuoteArgs {
    /// Source chain
    pub(crate) from_chain: String,
    /// Destination chain
    pub(crate) to_chain: String,
    /// Source token symbol/address
    pub(crate) from_token: String,
    /// Destination token symbol/address
    pub(crate) to_token: String,
    /// Amount to bridge
    pub(crate) amount: f64,
    /// Sender wallet address; needed for executable quote
    pub(crate) from_address: Option<String>,
    /// Receiver wallet address; needed for executable quote
    pub(crate) to_address: Option<String>,
    /// Slippage tolerance in basis points (default 50)
    pub(crate) slippage_bps: Option<u32>,
}

impl DynAomiTool for GetLifiBridgeQuote {
    type App = LifiApp;
    type Args = GetLifiBridgeQuoteArgs;
    const NAME: &'static str = "get_lifi_bridge_quote";
    const DESCRIPTION: &'static str = "Get cross-chain bridge route with executable tx data via LI.FI. Returns executable bridge payload when available; otherwise planning-only estimate.";

    fn run(_app: &LifiApp, args: Self::Args, _ctx: DynToolCallCtx) -> Result<Value, String> {
        ok(LifiClient::new()?.get_bridge_quote(
            &args.from_chain,
            &args.to_chain,
            &args.from_token,
            &args.to_token,
            args.amount,
            args.from_address.as_deref(),
            args.to_address.as_deref(),
            args.slippage_bps,
        )?)
    }
}

// ============================================================================
// GetLifiTransferStatus
// ============================================================================

pub(crate) struct GetLifiTransferStatus;

#[derive(Debug, Deserialize, JsonSchema)]
pub(crate) struct GetLifiTransferStatusArgs {
    /// Transaction hash to track
    pub(crate) tx_hash: String,
    /// Source chain name or ID (speeds up lookup)
    pub(crate) from_chain: Option<String>,
    /// Destination chain name or ID (speeds up lookup)
    pub(crate) to_chain: Option<String>,
    /// Bridge name (speeds up lookup)
    pub(crate) bridge: Option<String>,
}

impl DynAomiTool for GetLifiTransferStatus {
    type App = LifiApp;
    type Args = GetLifiTransferStatusArgs;
    const NAME: &'static str = "get_lifi_transfer_status";
    const DESCRIPTION: &'static str = "Track the status of a cross-chain transfer by transaction hash. Returns status (NOT_FOUND, INVALID, PENDING, DONE, FAILED), substatus, and transaction details.";

    fn run(_app: &LifiApp, args: Self::Args, _ctx: DynToolCallCtx) -> Result<Value, String> {
        ok(LifiClient::new()?.get_transfer_status(
            &args.tx_hash,
            args.from_chain.as_deref(),
            args.to_chain.as_deref(),
            args.bridge.as_deref(),
        )?)
    }
}

// ============================================================================
// GetLifiChains
// ============================================================================

pub(crate) struct GetLifiChains;

#[derive(Debug, Deserialize, JsonSchema)]
pub(crate) struct GetLifiChainsArgs {
    /// Filter by chain type, e.g. "EVM", "SVM"
    pub(crate) chain_types: Option<String>,
}

impl DynAomiTool for GetLifiChains {
    type App = LifiApp;
    type Args = GetLifiChainsArgs;
    const NAME: &'static str = "get_lifi_chains";
    const DESCRIPTION: &'static str =
        "List all chains supported by LI.FI. Optionally filter by chain type (e.g. EVM, SVM).";

    fn run(_app: &LifiApp, args: Self::Args, _ctx: DynToolCallCtx) -> Result<Value, String> {
        ok(LifiClient::new()?.get_chains(args.chain_types.as_deref())?)
    }
}

// ============================================================================
// GetLifiTokens
// ============================================================================

pub(crate) struct GetLifiTokens;

#[derive(Debug, Deserialize, JsonSchema)]
pub(crate) struct GetLifiTokensArgs {
    /// Comma-separated chain IDs to filter tokens
    pub(crate) chains: Option<String>,
    /// Filter by chain type, e.g. "EVM", "SVM"
    pub(crate) chain_types: Option<String>,
}

impl DynAomiTool for GetLifiTokens {
    type App = LifiApp;
    type Args = GetLifiTokensArgs;
    const NAME: &'static str = "get_lifi_tokens";
    const DESCRIPTION: &'static str = "List supported tokens on LI.FI. Optionally filter by chain IDs (comma-separated) or chain type.";

    fn run(_app: &LifiApp, args: Self::Args, _ctx: DynToolCallCtx) -> Result<Value, String> {
        ok(LifiClient::new()?.get_tokens(args.chains.as_deref(), args.chain_types.as_deref())?)
    }
}

// ============================================================================
// GetLifiToken
// ============================================================================

pub(crate) struct GetLifiToken;

#[derive(Debug, Deserialize, JsonSchema)]
pub(crate) struct GetLifiTokenArgs {
    /// Chain name or ID
    pub(crate) chain: String,
    /// Token address or symbol
    pub(crate) token: String,
}

impl DynAomiTool for GetLifiToken {
    type App = LifiApp;
    type Args = GetLifiTokenArgs;
    const NAME: &'static str = "get_lifi_token";
    const DESCRIPTION: &'static str =
        "Get detailed information for a single token including decimals and price.";

    fn run(_app: &LifiApp, args: Self::Args, _ctx: DynToolCallCtx) -> Result<Value, String> {
        ok(LifiClient::new()?.get_token(&args.chain, &args.token)?)
    }
}

// ============================================================================
// GetLifiRoutes
// ============================================================================

pub(crate) struct GetLifiRoutes;

#[derive(Debug, Deserialize, JsonSchema)]
pub(crate) struct GetLifiRoutesArgs {
    /// Source chain name or ID
    pub(crate) from_chain: String,
    /// Destination chain name or ID
    pub(crate) to_chain: String,
    /// Source token symbol or address
    pub(crate) from_token: String,
    /// Destination token symbol or address
    pub(crate) to_token: String,
    /// Amount to swap/bridge (human-readable units)
    pub(crate) amount: f64,
    /// Sender wallet address
    pub(crate) from_address: String,
    /// Slippage tolerance as decimal (e.g. 0.005 = 0.5%)
    pub(crate) slippage: Option<f64>,
    /// Route ordering preference: "CHEAPEST", "FASTEST", "SAFEST", or "RECOMMENDED"
    pub(crate) order_preference: Option<String>,
}

impl DynAomiTool for GetLifiRoutes {
    type App = LifiApp;
    type Args = GetLifiRoutesArgs;
    const NAME: &'static str = "get_lifi_routes";
    const DESCRIPTION: &'static str = "Get multiple route alternatives for a swap or bridge via LI.FI advanced routing. Compare routes by cost, speed, or safety. Use order_preference to sort: CHEAPEST, FASTEST, SAFEST, or RECOMMENDED.";

    fn run(_app: &LifiApp, args: Self::Args, _ctx: DynToolCallCtx) -> Result<Value, String> {
        ok(LifiClient::new()?.get_routes(
            &args.from_chain,
            &args.to_chain,
            &args.from_token,
            &args.to_token,
            args.amount,
            &args.from_address,
            args.slippage,
            args.order_preference.as_deref(),
        )?)
    }
}

// ============================================================================
// GetLifiStepTransaction
// ============================================================================

pub(crate) struct GetLifiStepTransaction;

#[derive(Debug, Deserialize, JsonSchema)]
pub(crate) struct GetLifiStepTransactionArgs {
    /// A Step object as returned by /advanced/routes
    pub(crate) step: Value,
}

impl DynAomiTool for GetLifiStepTransaction {
    type App = LifiApp;
    type Args = GetLifiStepTransactionArgs;
    const NAME: &'static str = "get_lifi_step_transaction";
    const DESCRIPTION: &'static str = "Get executable transaction data for a single route step returned by get_lifi_routes. Pass the step object directly.";

    fn run(_app: &LifiApp, args: Self::Args, _ctx: DynToolCallCtx) -> Result<Value, String> {
        ok(LifiClient::new()?.get_step_transaction(&args.step)?)
    }
}

// ============================================================================
// GetLifiConnections
// ============================================================================

pub(crate) struct GetLifiConnections;

#[derive(Debug, Deserialize, JsonSchema)]
pub(crate) struct GetLifiConnectionsArgs {
    /// Source chain name or ID
    pub(crate) from_chain: Option<String>,
    /// Destination chain name or ID
    pub(crate) to_chain: Option<String>,
    /// Source token address
    pub(crate) from_token: Option<String>,
    /// Destination token address
    pub(crate) to_token: Option<String>,
}

impl DynAomiTool for GetLifiConnections {
    type App = LifiApp;
    type Args = GetLifiConnectionsArgs;
    const NAME: &'static str = "get_lifi_connections";
    const DESCRIPTION: &'static str =
        "Check available transfer pathways between chains and tokens on LI.FI.";

    fn run(_app: &LifiApp, args: Self::Args, _ctx: DynToolCallCtx) -> Result<Value, String> {
        ok(LifiClient::new()?.get_connections(
            args.from_chain.as_deref(),
            args.to_chain.as_deref(),
            args.from_token.as_deref(),
            args.to_token.as_deref(),
        )?)
    }
}

// ============================================================================
// GetLifiTools
// ============================================================================

pub(crate) struct GetLifiTools;

#[derive(Debug, Deserialize, JsonSchema)]
pub(crate) struct GetLifiToolsArgs {
    /// Comma-separated chain IDs to filter
    pub(crate) chains: Option<String>,
}

impl DynAomiTool for GetLifiTools {
    type App = LifiApp;
    type Args = GetLifiToolsArgs;
    const NAME: &'static str = "get_lifi_tools";
    const DESCRIPTION: &'static str =
        "List available bridges and DEX exchanges on LI.FI. Optionally filter by chain IDs.";

    fn run(_app: &LifiApp, args: Self::Args, _ctx: DynToolCallCtx) -> Result<Value, String> {
        ok(LifiClient::new()?.get_tools(args.chains.as_deref())?)
    }
}

// ============================================================================
// GetLifiReverseQuote
// ============================================================================

pub(crate) struct GetLifiReverseQuote;

#[derive(Debug, Deserialize, JsonSchema)]
pub(crate) struct GetLifiReverseQuoteArgs {
    /// Source chain name or ID
    pub(crate) from_chain: String,
    /// Destination chain name or ID (defaults to source chain)
    pub(crate) to_chain: Option<String>,
    /// Source token symbol or address
    pub(crate) from_token: String,
    /// Destination token symbol or address
    pub(crate) to_token: String,
    /// Desired output amount in base units (e.g. wei)
    pub(crate) to_amount: String,
    /// Sender wallet address
    pub(crate) from_address: String,
    /// Receiver wallet address (defaults to sender)
    pub(crate) to_address: Option<String>,
}

impl DynAomiTool for GetLifiReverseQuote {
    type App = LifiApp;
    type Args = GetLifiReverseQuoteArgs;
    const NAME: &'static str = "get_lifi_reverse_quote";
    const DESCRIPTION: &'static str = "Get a quote by specifying the desired output amount (reverse quote). LI.FI calculates the required input amount.";

    fn run(_app: &LifiApp, args: Self::Args, _ctx: DynToolCallCtx) -> Result<Value, String> {
        ok(LifiClient::new()?.get_reverse_quote(
            &args.from_chain,
            args.to_chain.as_deref(),
            &args.from_token,
            &args.to_token,
            &args.to_amount,
            &args.from_address,
            args.to_address.as_deref(),
        )?)
    }
}

// ============================================================================
// GetLifiGasSuggestion
// ============================================================================

pub(crate) struct GetLifiGasSuggestion;

#[derive(Debug, Deserialize, JsonSchema)]
pub(crate) struct GetLifiGasSuggestionArgs {
    /// Destination chain name or ID
    pub(crate) chain: String,
    /// Source chain name or ID
    pub(crate) from_chain: Option<String>,
    /// Source token address
    pub(crate) from_token: Option<String>,
}

impl DynAomiTool for GetLifiGasSuggestion {
    type App = LifiApp;
    type Args = GetLifiGasSuggestionArgs;
    const NAME: &'static str = "get_lifi_gas_suggestion";
    const DESCRIPTION: &'static str = "Get suggested gas amount for a destination chain. Useful for estimating gas needs for cross-chain transfers.";

    fn run(_app: &LifiApp, args: Self::Args, _ctx: DynToolCallCtx) -> Result<Value, String> {
        ok(LifiClient::new()?.get_gas_suggestion(
            &args.chain,
            args.from_chain.as_deref(),
            args.from_token.as_deref(),
        )?)
    }
}
