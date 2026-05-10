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
// LifiGetSwapQuote -- price discovery for a same-chain or cross-chain swap
// ============================================================================

pub(crate) struct LifiGetSwapQuote;

#[derive(Debug, Deserialize, JsonSchema)]
pub(crate) struct LifiGetSwapQuoteArgs {
    /// Source chain (name like "ethereum", "polygon", "base", "arbitrum", "optimism", "bsc", "avalanche").
    pub(crate) chain: String,
    /// Destination chain. Omit for same-chain swap.
    #[serde(default)]
    pub(crate) destination_chain: Option<String>,
    /// Sell token symbol (USDC, WETH, ETH, ...) or 0x... address.
    pub(crate) sell_token: String,
    /// Buy token symbol or 0x... address.
    pub(crate) buy_token: String,
    /// Sell amount in human-readable units (e.g. 100.0 = 100 USDC).
    pub(crate) amount: f64,
    /// Sender wallet address (0x...). Required.
    pub(crate) sender_address: String,
    /// Receiver wallet address. Defaults to sender.
    #[serde(default)]
    pub(crate) receiver_address: Option<String>,
}

impl DynAomiTool for LifiGetSwapQuote {
    type App = LifiApp;
    type Args = LifiGetSwapQuoteArgs;
    const NAME: &'static str = "lifi_get_swap_quote";
    const DESCRIPTION: &'static str = "Use to preview a same-chain or cross-chain swap via LI.FI (no signing). Returns expected `toAmount`, route summary, and gas/fee estimates. For execution, follow up with `lifi_build_swap_tx`.";

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
// LifiBuildSwapTx -- composite: quote + approval_tx (if needed) + main_tx
// ============================================================================

pub(crate) struct LifiBuildSwapTx;

#[derive(Debug, Deserialize, JsonSchema)]
pub(crate) struct LifiBuildSwapTxArgs {
    /// Source chain name.
    pub(crate) chain: String,
    /// Destination chain. Omit for same-chain swap.
    #[serde(default)]
    pub(crate) destination_chain: Option<String>,
    /// Sell token symbol or 0x... address.
    pub(crate) sell_token: String,
    /// Buy token symbol or 0x... address.
    pub(crate) buy_token: String,
    /// Sell amount in human-readable units.
    pub(crate) amount: f64,
    /// Sender wallet address (0x...).
    pub(crate) sender_address: String,
    /// Receiver wallet address. Defaults to sender.
    #[serde(default)]
    pub(crate) receiver_address: Option<String>,
    /// Slippage tolerance as a decimal (0.005 = 0.5%).
    #[serde(default)]
    pub(crate) slippage: Option<f64>,
}

impl DynAomiTool for LifiBuildSwapTx {
    type App = LifiApp;
    type Args = LifiBuildSwapTxArgs;
    const NAME: &'static str = "lifi_build_swap_tx";
    const DESCRIPTION: &'static str = "Use when the user is ready to execute a same-chain or cross-chain swap via LI.FI. Returns `{ approval_tx?, main_tx, payload }`. If `approval_tx` is present (ERC-20 sell needing allowance), stage it first via `stage_tx` with `data: { raw }`, then stage `main_tx` the same way; `simulate_batch` on the staged ids; then `commit_tx` once per staged tx. Never re-encode LI.FI calldata.";

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
            "note": "If approval_tx is non-null, stage it first with stage_tx { raw }, then stage main_tx the same way, then simulate_batch the staged pending_tx_id list, then commit_tx once per staged tx.",
        }))
    }
}

// ============================================================================
// LifiBuildBridgeTx -- cross-chain bridge (returns executable tx when possible)
// ============================================================================

pub(crate) struct LifiBuildBridgeTx;

#[derive(Debug, Deserialize, JsonSchema)]
pub(crate) struct LifiBuildBridgeTxArgs {
    /// Source chain name or numeric ID.
    pub(crate) from_chain: String,
    /// Destination chain name or numeric ID.
    pub(crate) to_chain: String,
    /// Source token symbol or 0x... address.
    pub(crate) from_token: String,
    /// Destination token symbol or 0x... address.
    pub(crate) to_token: String,
    /// Bridge amount in human-readable units.
    pub(crate) amount: f64,
    /// Sender wallet address. Required for an executable bridge route.
    #[serde(default)]
    pub(crate) from_address: Option<String>,
    /// Receiver wallet address. Required for an executable bridge route.
    #[serde(default)]
    pub(crate) to_address: Option<String>,
    /// Slippage tolerance in basis points (default 50 = 0.5%).
    #[serde(default)]
    pub(crate) slippage_bps: Option<u32>,
}

impl DynAomiTool for LifiBuildBridgeTx {
    type App = LifiApp;
    type Args = LifiBuildBridgeTxArgs;
    const NAME: &'static str = "lifi_build_bridge_tx";
    const DESCRIPTION: &'static str = "Use when the user wants to bridge a token from one chain to another via LI.FI. Returns an executable bridge payload (with `executable_tx`) when both `from_address` and `to_address` are provided; otherwise returns a planning-only estimate. Stage and execute the same way as `lifi_build_swap_tx`. After executing, track on-chain finality with `lifi_get_transfer_status`.";

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
// LifiGetTransferStatus -- track a cross-chain transfer
// ============================================================================

pub(crate) struct LifiGetTransferStatus;

#[derive(Debug, Deserialize, JsonSchema)]
pub(crate) struct LifiGetTransferStatusArgs {
    /// Source-chain transaction hash (the deposit tx the user signed).
    pub(crate) tx_hash: String,
    /// Source chain name or numeric ID. Optional but speeds lookup.
    #[serde(default)]
    pub(crate) from_chain: Option<String>,
    /// Destination chain name or numeric ID. Optional but speeds lookup.
    #[serde(default)]
    pub(crate) to_chain: Option<String>,
    /// Bridge name (e.g. "across", "stargate"). Optional but speeds lookup.
    #[serde(default)]
    pub(crate) bridge: Option<String>,
}

impl DynAomiTool for LifiGetTransferStatus {
    type App = LifiApp;
    type Args = LifiGetTransferStatusArgs;
    const NAME: &'static str = "lifi_get_transfer_status";
    const DESCRIPTION: &'static str = "Use to track a LI.FI cross-chain transfer by source-chain tx hash. Returns `status` (NOT_FOUND, INVALID, PENDING, DONE, FAILED), substatus, and the destination-chain receipt when complete. Poll periodically while status is PENDING.";

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
// LifiListChains -- discovery
// ============================================================================

pub(crate) struct LifiListChains;

#[derive(Debug, Deserialize, JsonSchema)]
pub(crate) struct LifiListChainsArgs {
    /// Filter by chain type, e.g. "EVM", "SVM". Omit for all.
    #[serde(default)]
    pub(crate) chain_types: Option<String>,
}

impl DynAomiTool for LifiListChains {
    type App = LifiApp;
    type Args = LifiListChainsArgs;
    const NAME: &'static str = "lifi_list_chains";
    const DESCRIPTION: &'static str = "Use when the user asks 'what chains does LI.FI support?' Returns the supported chain list with names, ids, and native currency. Optionally filter by `chain_types` (EVM, SVM).";

    fn run(_app: &LifiApp, args: Self::Args, _ctx: DynToolCallCtx) -> Result<Value, String> {
        ok(LifiClient::new()?.get_chains(args.chain_types.as_deref())?)
    }
}

// ============================================================================
// LifiListTokens -- discovery
// ============================================================================

pub(crate) struct LifiListTokens;

#[derive(Debug, Deserialize, JsonSchema)]
pub(crate) struct LifiListTokensArgs {
    /// Comma-separated chain IDs to filter by (e.g. "1,137,8453").
    #[serde(default)]
    pub(crate) chains: Option<String>,
}

impl DynAomiTool for LifiListTokens {
    type App = LifiApp;
    type Args = LifiListTokensArgs;
    const NAME: &'static str = "lifi_list_tokens";
    const DESCRIPTION: &'static str = "Use when the user asks 'what tokens are bridgeable on chain X?' or needs a token's address/decimals. Returns the supported-token map keyed by chain ID. Pass `chains` (comma-separated chain IDs) to scope the response.";

    fn run(_app: &LifiApp, args: Self::Args, _ctx: DynToolCallCtx) -> Result<Value, String> {
        ok(LifiClient::new()?.get_tokens(args.chains.as_deref(), None)?)
    }
}
