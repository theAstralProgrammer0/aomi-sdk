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
    /// Chain: ethereum, gnosis, arbitrum, base, polygon, avalanche, bsc, sepolia.
    pub(crate) chain: String,
    /// Sell token symbol (eth, usdc, weth, ...) or 0x address.
    pub(crate) sell_token: String,
    /// Buy token symbol or 0x address.
    pub(crate) buy_token: String,
    /// Amount in human units. Treated as sell amount when order_side="sell" (default), buy amount when "buy".
    pub(crate) amount: f64,
    /// Wallet address signing the order (and default receiver).
    pub(crate) sender_address: String,
    /// Optional alternate receiver. Defaults to sender_address.
    pub(crate) receiver_address: Option<String>,
    /// "sell" (exact-in, default) or "buy" (exact-out).
    pub(crate) order_side: Option<String>,
    /// Slippage tolerance as decimal (0.005 = 0.5%). Defaults to CoW's recommendation when omitted.
    pub(crate) slippage: Option<f64>,
}

impl DynAomiTool for GetCowSwapQuote {
    type App = CowApp;
    type Args = GetCowSwapQuoteArgs;
    const NAME: &'static str = "get_cow_swap_quote";
    const DESCRIPTION: &'static str = "Use when the user wants to price a swap on CoW Protocol (\"swap 100 USDC for WETH on base\"). Returns the quote with sellAmount, buyAmount, fees, and the order parameters that must be signed before placing. Always run this BEFORE place_cow_order.";

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
            // Knobs we don't expose to the LLM — CoW applies sensible defaults.
            valid_to: None,
            partially_fillable: None,
            signing_scheme: None,
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
    const DESCRIPTION: &'static str = "Use AFTER get_cow_swap_quote and the user has signed the order. Submits the complete signed order JSON to CoW's orderbook. The `signed_order` payload must contain the quote fields plus `signature` and `signingScheme` from the host wallet. Returns the orderUid you can poll with get_cow_order_status.";

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
    const DESCRIPTION: &'static str = "Use when the user wants full detail on a CoW order they previously placed (executed amounts, fees, signature, status). Provide the orderUid returned from place_cow_order. For just the lifecycle state, prefer get_cow_order_status.";

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
    const DESCRIPTION: &'static str = "Use to poll a CoW order's lifecycle state (open / scheduled / active / solved / executing / traded / cancelled). Lighter than get_cow_order. Don't poll faster than every ~3s; CoW solver auctions clear in ~30s.";

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
    const DESCRIPTION: &'static str = "Use when the user asks about their CoW order history on a chain (\"my recent swaps on base\"). Paginated, newest first. Default limit if omitted is CoW's default (~10).";

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
    const DESCRIPTION: &'static str = "Use when the user wants to cancel open CoW orders (only orders not yet executed can be cancelled). Requires a cancellation signature from the order owner — the host wallet must sign the cancellation message before calling this. Pass `signing_scheme=\"eip712\"` when in doubt.";

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
    const DESCRIPTION: &'static str = "Use when the user wants the on-chain settlement record (executed amounts, tx hashes) for either a wallet (`owner`) or one specific order (`order_uid`). Pass exactly one. Use get_cow_user_orders when the user wants the order book view rather than fills.";

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
    const DESCRIPTION: &'static str = "Use to read CoW's internal estimate of a token's price in the chain's native asset (ETH on mainnet/arbitrum/base, xDAI on gnosis, etc.). Useful for sanity-checking a quote before signing. `token_address` must be a 0x address — symbol shorthand not supported here.";

    fn run(_app: &CowApp, args: Self::Args, _ctx: DynToolCallCtx) -> Result<Value, String> {
        ok::<CowNativePrice>(CowClient::new()?.get_native_price(&args.chain, &args.token_address)?)
    }
}

