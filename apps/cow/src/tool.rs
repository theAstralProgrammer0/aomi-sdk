use aomi_ext::cow::{
    CancelOrdersRequest, CowClient, CowNativePrice, CowOrder, CowOrderStatus, CowTrade,
    QuoteRequest, amount_to_base_units, get_chain_info, get_token_address, get_token_decimals,
};
use aomi_sdk::*;
use aomi_sdk::schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Clone, Default)]
pub(crate) struct CowApp;

fn ok<T: Serialize>(value: T) -> Result<Value, String> {
    let value = serde_json::to_value(value)
        .map_err(|e| format!("[cow] failed to serialize response: {e}"))?;
    Ok(match value {
        Value::Object(mut map) => {
            map.insert("source".to_string(), Value::String("cow".to_string()));
            Value::Object(map)
        }
        other => serde_json::json!({ "source": "cow", "data": other }),
    })
}

// ============================================================================
// GetCowSwapQuote
// ============================================================================

pub(crate) struct GetCowSwapQuote;

#[derive(Debug, Deserialize, JsonSchema)]
pub(crate) struct GetCowSwapQuoteArgs {
    /// Chain: ethereum, gnosis, arbitrum, base, polygon, avalanche, bsc, sepolia
    pub(crate) chain: String,
    /// Sell token symbol or address
    pub(crate) sell_token: String,
    /// Buy token symbol or address
    pub(crate) buy_token: String,
    /// Amount to swap (human-readable units)
    pub(crate) amount: f64,
    /// Sender/from address
    pub(crate) sender_address: String,
    /// Receiver address (optional, defaults to sender)
    pub(crate) receiver_address: Option<String>,
    /// Order side: "sell" or "buy" (default: "sell")
    pub(crate) order_side: Option<String>,
    /// Quote validity timestamp (optional)
    pub(crate) valid_to: Option<u64>,
    /// Allow partial fills (optional)
    pub(crate) partially_fillable: Option<bool>,
    /// Signing scheme: eip712, ethsign (optional)
    pub(crate) signing_scheme: Option<String>,
    /// Slippage tolerance as decimal (0.005 = 0.5%)
    pub(crate) slippage: Option<f64>,
}

impl DynAomiTool for GetCowSwapQuote {
    type App = CowApp;
    type Args = GetCowSwapQuoteArgs;
    const NAME: &'static str = "get_cow_swap_quote";
    const DESCRIPTION: &'static str = "Get a CoW Protocol swap quote with fee estimation.";

    fn run(_app: &CowApp, args: Self::Args, _ctx: DynToolCallCtx) -> Result<Value, String> {
        let client = CowClient::new()?;
        let (chain_name, _) = get_chain_info(&args.chain)?;
        let sell_addr = get_token_address(chain_name, &args.sell_token)?;
        let buy_addr = get_token_address(chain_name, &args.buy_token)?;
        let decimals = get_token_decimals(chain_name, &args.sell_token);
        let amount_base = amount_to_base_units(args.amount, decimals)?;
        let payload = QuoteRequest {
            sell_token: &sell_addr,
            buy_token: &buy_addr,
            sell_amount_before_fee: &amount_base,
            from: &args.sender_address,
            kind: args.order_side.as_deref().unwrap_or("sell"),
            receiver: args.receiver_address.as_deref(),
            valid_to: args.valid_to,
            partially_fillable: args.partially_fillable,
            signing_scheme: args.signing_scheme.as_deref(),
            slippage_bps: args.slippage.map(|s| (s * 10_000.0) as u32),
        };
        ok(client.get_quote::<QuoteRequest>(&args.chain, &payload)?)
    }
}

// ============================================================================
// PlaceCowOrder
// ============================================================================

pub(crate) struct PlaceCowOrder;

#[derive(Debug, Deserialize, JsonSchema)]
pub(crate) struct PlaceCowOrderArgs {
    /// CoW chain: ethereum, gnosis, arbitrum, base, polygon, avalanche, bsc, sepolia
    pub(crate) chain: String,
    /// Signed order payload to submit to CoW /orders endpoint
    pub(crate) signed_order: Value,
}

impl DynAomiTool for PlaceCowOrder {
    type App = CowApp;
    type Args = PlaceCowOrderArgs;
    const NAME: &'static str = "place_cow_order";
    const DESCRIPTION: &'static str = "Submit a signed CoW Protocol orderbook payload to CoW /orders API on the requested chain. Use the host's wallet/signing tools for any required user approval.";

    fn run(_app: &CowApp, args: Self::Args, _ctx: DynToolCallCtx) -> Result<Value, String> {
        ok(CowClient::new()?.place_order(&args.chain, args.signed_order)?)
    }
}

// ============================================================================
// GetCowOrder
// ============================================================================

pub(crate) struct GetCowOrder;

#[derive(Debug, Deserialize, JsonSchema)]
pub(crate) struct GetCowOrderArgs {
    /// Chain: ethereum, gnosis, arbitrum, base, polygon, avalanche, bsc, sepolia
    pub(crate) chain: String,
    /// Order UID returned when the order was placed
    pub(crate) order_uid: String,
}

impl DynAomiTool for GetCowOrder {
    type App = CowApp;
    type Args = GetCowOrderArgs;
    const NAME: &'static str = "get_cow_order";
    const DESCRIPTION: &'static str = "Get the full order object for a CoW Protocol order by UID (status, executed amounts, fees, etc.).";

    fn run(_app: &CowApp, args: Self::Args, _ctx: DynToolCallCtx) -> Result<Value, String> {
        ok::<CowOrder>(CowClient::new()?.get_order(&args.chain, &args.order_uid)?)
    }
}

// ============================================================================
// GetCowOrderStatus
// ============================================================================

pub(crate) struct GetCowOrderStatus;

#[derive(Debug, Deserialize, JsonSchema)]
pub(crate) struct GetCowOrderStatusArgs {
    /// Chain: ethereum, gnosis, arbitrum, base, polygon, avalanche, bsc, sepolia
    pub(crate) chain: String,
    /// Order UID returned when the order was placed
    pub(crate) order_uid: String,
}

impl DynAomiTool for GetCowOrderStatus {
    type App = CowApp;
    type Args = GetCowOrderStatusArgs;
    const NAME: &'static str = "get_cow_order_status";
    const DESCRIPTION: &'static str = "Get the competition status of a CoW Protocol order (open/scheduled/active/solved/executing/traded/cancelled).";

    fn run(_app: &CowApp, args: Self::Args, _ctx: DynToolCallCtx) -> Result<Value, String> {
        ok::<CowOrderStatus>(CowClient::new()?.get_order_status(&args.chain, &args.order_uid)?)
    }
}

// ============================================================================
// GetCowUserOrders
// ============================================================================

pub(crate) struct GetCowUserOrders;

#[derive(Debug, Deserialize, JsonSchema)]
pub(crate) struct GetCowUserOrdersArgs {
    /// Chain: ethereum, gnosis, arbitrum, base, polygon, avalanche, bsc, sepolia
    pub(crate) chain: String,
    /// Owner (wallet) address
    pub(crate) owner_address: String,
    /// Pagination offset
    pub(crate) offset: Option<u32>,
    /// Maximum number of results to return
    pub(crate) limit: Option<u32>,
}

impl DynAomiTool for GetCowUserOrders {
    type App = CowApp;
    type Args = GetCowUserOrdersArgs;
    const NAME: &'static str = "get_cow_user_orders";
    const DESCRIPTION: &'static str = "Get a paginated list of CoW Protocol orders for a given owner address, sorted by creation date.";

    fn run(_app: &CowApp, args: Self::Args, _ctx: DynToolCallCtx) -> Result<Value, String> {
        ok::<Vec<CowOrder>>(CowClient::new()?.get_user_orders(
            &args.chain,
            &args.owner_address,
            args.offset,
            args.limit,
        )?)
    }
}

// ============================================================================
// CancelCowOrders
// ============================================================================

pub(crate) struct CancelCowOrders;

#[derive(Debug, Deserialize, JsonSchema)]
pub(crate) struct CancelCowOrdersArgs {
    /// Chain: ethereum, gnosis, arbitrum, base, polygon, avalanche, bsc, sepolia
    pub(crate) chain: String,
    /// List of order UIDs to cancel
    pub(crate) order_uids: Vec<String>,
    /// Cancellation signature from the order owner
    pub(crate) signature: String,
    /// Signing scheme used: "eip712" or "ethsign"
    pub(crate) signing_scheme: String,
}

impl DynAomiTool for CancelCowOrders {
    type App = CowApp;
    type Args = CancelCowOrdersArgs;
    const NAME: &'static str = "cancel_cow_orders";
    const DESCRIPTION: &'static str = "Cancel one or more open CoW Protocol orders. Requires the cancellation signature from the order owner.";

    fn run(_app: &CowApp, args: Self::Args, _ctx: DynToolCallCtx) -> Result<Value, String> {
        let payload = CancelOrdersRequest {
            order_uids: &args.order_uids,
            signature: &args.signature,
            signing_scheme: &args.signing_scheme,
        };
        ok(CowClient::new()?.cancel_orders::<CancelOrdersRequest>(&args.chain, &payload)?)
    }
}

// ============================================================================
// GetCowTrades
// ============================================================================

pub(crate) struct GetCowTrades;

#[derive(Debug, Deserialize, JsonSchema)]
pub(crate) struct GetCowTradesArgs {
    /// Chain: ethereum, gnosis, arbitrum, base, polygon, avalanche, bsc, sepolia
    pub(crate) chain: String,
    /// Owner address (provide exactly one of owner or order_uid)
    pub(crate) owner: Option<String>,
    /// Order UID (provide exactly one of owner or order_uid)
    pub(crate) order_uid: Option<String>,
    /// Pagination offset
    pub(crate) offset: Option<u32>,
    /// Maximum number of results to return
    pub(crate) limit: Option<u32>,
}

impl DynAomiTool for GetCowTrades {
    type App = CowApp;
    type Args = GetCowTradesArgs;
    const NAME: &'static str = "get_cow_trades";
    const DESCRIPTION: &'static str =
        "Get trade execution history from CoW Protocol. Provide exactly one of owner or order_uid.";

    fn run(_app: &CowApp, args: Self::Args, _ctx: DynToolCallCtx) -> Result<Value, String> {
        match (&args.owner, &args.order_uid) {
            (Some(_), Some(_)) => {
                return Err(
                    "[cow] provide exactly one of `owner` or `order_uid`, not both".to_string(),
                );
            }
            (None, None) => {
                return Err("[cow] provide exactly one of `owner` or `order_uid`".to_string());
            }
            _ => {}
        }
        ok::<Vec<CowTrade>>(CowClient::new()?.get_trades(
            &args.chain,
            args.owner.as_deref(),
            args.order_uid.as_deref(),
            args.offset,
            args.limit,
        )?)
    }
}

// ============================================================================
// GetCowNativePrice
// ============================================================================

pub(crate) struct GetCowNativePrice;

#[derive(Debug, Deserialize, JsonSchema)]
pub(crate) struct GetCowNativePriceArgs {
    /// Chain: ethereum, gnosis, arbitrum, base, polygon, avalanche, bsc, sepolia
    pub(crate) chain: String,
    /// Token contract address (0x...)
    pub(crate) token_address: String,
}

impl DynAomiTool for GetCowNativePrice {
    type App = CowApp;
    type Args = GetCowNativePriceArgs;
    const NAME: &'static str = "get_cow_native_price";
    const DESCRIPTION: &'static str =
        "Get the price of a token relative to the chain's native currency via CoW Protocol.";

    fn run(_app: &CowApp, args: Self::Args, _ctx: DynToolCallCtx) -> Result<Value, String> {
        ok::<CowNativePrice>(CowClient::new()?.get_native_price(&args.chain, &args.token_address)?)
    }
}

// ============================================================================
// GetCowOrdersByTx
// ============================================================================

pub(crate) struct GetCowOrdersByTx;

#[derive(Debug, Deserialize, JsonSchema)]
pub(crate) struct GetCowOrdersByTxArgs {
    /// Chain: ethereum, gnosis, arbitrum, base, polygon, avalanche, bsc, sepolia
    pub(crate) chain: String,
    /// Transaction hash (0x...)
    pub(crate) tx_hash: String,
}

impl DynAomiTool for GetCowOrdersByTx {
    type App = CowApp;
    type Args = GetCowOrdersByTxArgs;
    const NAME: &'static str = "get_cow_orders_by_tx";
    const DESCRIPTION: &'static str =
        "Get all CoW Protocol orders that were settled in a specific transaction.";

    fn run(_app: &CowApp, args: Self::Args, _ctx: DynToolCallCtx) -> Result<Value, String> {
        ok::<Vec<CowOrder>>(CowClient::new()?.get_orders_by_tx(&args.chain, &args.tx_hash)?)
    }
}

// ============================================================================
// DebugCowOrder
// ============================================================================

pub(crate) struct DebugCowOrder;

#[derive(Debug, Deserialize, JsonSchema)]
pub(crate) struct DebugCowOrderArgs {
    /// Chain: ethereum, gnosis, arbitrum, base, polygon, avalanche, bsc, sepolia
    pub(crate) chain: String,
    /// Order UID to debug
    pub(crate) order_uid: String,
}

impl DynAomiTool for DebugCowOrder {
    type App = CowApp;
    type Args = DebugCowOrderArgs;
    const NAME: &'static str = "debug_cow_order";
    const DESCRIPTION: &'static str = "Get the full lifecycle debug info for a CoW Protocol order, including events and auction participation.";

    fn run(_app: &CowApp, args: Self::Args, _ctx: DynToolCallCtx) -> Result<Value, String> {
        ok::<CowOrder>(CowClient::new()?.debug_order(&args.chain, &args.order_uid)?)
    }
}
