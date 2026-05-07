use crate::client::{
    ByrealApp, OrderInputs, build_cancel_action, build_exchange_body, build_order_action,
    build_update_leverage_action, byreal_client, parse_signature, prepare_l1_action,
};
use aomi_sdk::schemars::JsonSchema;
use aomi_sdk::*;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

const SIGNATURE_ALIAS: &str = "master_signature";
const DEFAULT_MARKET_SLIPPAGE_PCT: f64 = 5.0;

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn ok<T: Serialize>(value: T) -> Result<Value, String> {
    let v = serde_json::to_value(value).map_err(|e| format!("[byreal] response serialize: {e}"))?;
    Ok(match v {
        Value::Object(mut map) => {
            map.insert("source".to_string(), Value::String("byreal".to_string()));
            Value::Object(map)
        }
        other => json!({ "source": "byreal", "data": other }),
    })
}

fn validate_confirmation(token: Option<&str>) -> Result<(), String> {
    match token {
        Some("confirm") => Ok(()),
        _ => Err(
            "submit_* requires `confirmation: \"confirm\"`. Show the user the build_* preview \
             and obtain explicit go-ahead before submitting."
                .to_string(),
        ),
    }
}

fn build_commit_args(typed_data: Value, description: String) -> Value {
    json!({
        "typed_data": typed_data,
        "description": description,
    })
}

fn build_signed_routes<Submit: RouteTarget>(
    value: Value,
    typed_data: Value,
    description: String,
    submit_template: Value,
) -> Result<ToolReturn, String> {
    ToolReturn::route(value)
        .next(|next| {
            next.add::<host::CommitEip712>(build_commit_args(typed_data, description))
                .bind_as(SIGNATURE_ALIAS)
                .note("Wait for explicit user confirmation, then sign this Hyperliquid action with the master wallet.");
        })
        .after::<Submit>(submit_template)
        .awaits(SIGNATURE_ALIAS)
        .note("Wallet signed — submit the action to Hyperliquid.")
        .try_build()
        .map_err(|e| format!("[byreal] route build failed: {e}"))
}

// ---------------------------------------------------------------------------
// build_order / submit_order
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Deserialize, JsonSchema)]
pub(crate) struct BuildOrderArgs {
    /// Asset ticker (e.g. "BTC", "ETH", "SOL"). Must exist in the Hyperliquid universe.
    pub coin: String,
    /// true for long / buy, false for short / sell.
    pub is_buy: bool,
    /// Order kind: "market" or "limit".
    pub order_kind: String,
    /// Size in coin units (NOT USD). Use `usd_notional / mid_price` if you have a USD target.
    pub sz: f64,
    /// Limit price in USD. Required for "limit" orders. Ignored for "market".
    pub limit_px: Option<f64>,
    /// Current mid price in USD. Required for "market" orders so we can apply slippage.
    pub mid_price: Option<f64>,
    /// Slippage tolerance for market orders, in percent. Default 5.0 (matches the Byreal CLI).
    pub slippage_pct: Option<f64>,
    /// Time-in-force for limit orders: "Gtc" (default), "Ioc", or "Alo".
    pub tif: Option<String>,
    /// Reduce-only flag. Set true when closing a position so you don't accidentally flip side.
    pub reduce_only: Option<bool>,
}

pub(crate) struct BuildOrder;

impl DynAomiTool for BuildOrder {
    type App = ByrealApp;
    type Args = BuildOrderArgs;
    const NAME: &'static str = "byreal_build_order";
    const DESCRIPTION: &'static str = "Build (do not submit) a Hyperliquid perpetual order. Returns a preview and a routed `commit_eip712` step the host wallet signs. The matched `byreal_submit_order` continuation runs after the signature comes back. Always emit a confirmation summary and stop the turn before calling this.";

    fn run_with_routes(
        _app: &Self::App,
        args: Self::Args,
        _ctx: DynToolCallCtx,
    ) -> Result<ToolReturn, String> {
        if args.sz <= 0.0 {
            return Err("[byreal] sz must be > 0".to_string());
        }
        let client = byreal_client()?;
        let asset_index = client.lookup_asset(&args.coin)?;

        let kind = args.order_kind.to_ascii_lowercase();
        let (limit_px, tif) = match kind.as_str() {
            "limit" => {
                let px = args
                    .limit_px
                    .ok_or_else(|| "[byreal] limit orders require limit_px".to_string())?;
                if px <= 0.0 {
                    return Err("[byreal] limit_px must be > 0".to_string());
                }
                (px, args.tif.clone().unwrap_or_else(|| "Gtc".to_string()))
            }
            "market" => {
                let mid = args.mid_price.ok_or_else(|| {
                    "[byreal] market orders require mid_price (fetch it from `byreal_get_all_mids` first)".to_string()
                })?;
                if mid <= 0.0 {
                    return Err("[byreal] mid_price must be > 0".to_string());
                }
                let slip = args.slippage_pct.unwrap_or(DEFAULT_MARKET_SLIPPAGE_PCT);
                let factor = if args.is_buy {
                    1.0 + slip / 100.0
                } else {
                    1.0 - slip / 100.0
                };
                (mid * factor, "Ioc".to_string())
            }
            other => {
                return Err(format!(
                    "[byreal] unknown order_kind '{other}', expected 'market' or 'limit'"
                ));
            }
        };

        let reduce_only = args.reduce_only.unwrap_or(false);

        let action = build_order_action(
            &OrderInputs {
                coin: &args.coin,
                is_buy: args.is_buy,
                limit_px,
                sz: args.sz,
                reduce_only,
                tif: tif.clone(),
            },
            client.coin_to_asset()?,
        )?;
        let (action_json, nonce, typed_data) = prepare_l1_action(action, None)?;

        let submit_template = serde_json::to_value(&SubmitOrderArgs {
            confirmation: Some("confirm".to_string()),
            action: action_json.clone(),
            nonce,
            master_signature: None,
            vault_address: None,
        })
        .map_err(|e| format!("[byreal] submit template serialize: {e}"))?;

        let preview = json!({
            "action_kind": "order",
            "preview": {
                "coin": args.coin,
                "asset_index": asset_index,
                "is_buy": args.is_buy,
                "side": if args.is_buy { "long" } else { "short" },
                "order_kind": kind,
                "sz": args.sz,
                "limit_px": limit_px,
                "tif": tif,
                "reduce_only": reduce_only,
                "slippage_applied_pct": if kind == "market" {
                    Some(args.slippage_pct.unwrap_or(DEFAULT_MARKET_SLIPPAGE_PCT))
                } else { None },
            },
            "nonce": nonce,
            "requires_user_confirmation": true,
            "confirmation_phrase": "confirm",
            "submit_args_template": submit_template.clone(),
        });

        let description = format!(
            "Hyperliquid {} {} {} {} @ ${:.4}{}",
            if args.is_buy { "BUY" } else { "SELL" },
            args.sz,
            args.coin,
            kind,
            limit_px,
            if reduce_only { " (reduce-only)" } else { "" },
        );

        build_signed_routes::<SubmitOrder>(preview, typed_data, description, submit_template)
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
pub(crate) struct SubmitOrderArgs {
    /// Must be the literal string "confirm". Forwarded from the build_* preview.
    pub confirmation: Option<String>,
    /// The signed Hyperliquid action JSON. Forward verbatim from the build_* output.
    pub action: Value,
    /// The nonce that was hashed into the EIP-712 connection_id.
    pub nonce: u64,
    /// EIP-712 signature (65-byte hex). Filled in by the host wallet via `commit_eip712`.
    pub master_signature: Option<String>,
    /// Optional vault address for sub-account / vault trading.
    pub vault_address: Option<String>,
}

pub(crate) struct SubmitOrder;

impl DynAomiTool for SubmitOrder {
    type App = ByrealApp;
    type Args = SubmitOrderArgs;
    const NAME: &'static str = "byreal_submit_order";
    const DESCRIPTION: &'static str = "Submit a Hyperliquid order that was previously prepared by `byreal_build_order` and signed via `commit_eip712`. The `master_signature` field is filled in automatically by the runtime — never invent one.";

    fn run(_app: &Self::App, args: Self::Args, _ctx: DynToolCallCtx) -> Result<Value, String> {
        validate_confirmation(args.confirmation.as_deref())?;
        let sig_hex = args.master_signature.as_deref().ok_or_else(|| {
            "[byreal] master_signature missing — wait for commit_eip712 callback".to_string()
        })?;
        let sig = parse_signature(sig_hex)?;
        let body =
            build_exchange_body(args.action, args.nonce, &sig, args.vault_address.as_deref());
        ok(byreal_client()?.post_exchange(body)?)
    }
}

// ---------------------------------------------------------------------------
// build_cancel / submit_cancel
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Deserialize, JsonSchema)]
pub(crate) struct BuildCancelArgs {
    /// Asset ticker for the order being canceled.
    pub coin: String,
    /// Order ID returned when the order was placed (look it up via `byreal_get_open_orders` if unknown).
    pub oid: u64,
}

pub(crate) struct BuildCancel;

impl DynAomiTool for BuildCancel {
    type App = ByrealApp;
    type Args = BuildCancelArgs;
    const NAME: &'static str = "byreal_build_cancel";
    const DESCRIPTION: &'static str = "Build (do not submit) a cancel for a single resting order. Returns a preview and a routed `commit_eip712` step the host wallet signs.";

    fn run_with_routes(
        _app: &Self::App,
        args: Self::Args,
        _ctx: DynToolCallCtx,
    ) -> Result<ToolReturn, String> {
        let client = byreal_client()?;
        let asset_index = client.lookup_asset(&args.coin)?;
        let action = build_cancel_action(asset_index, args.oid);
        let (action_json, nonce, typed_data) = prepare_l1_action(action, None)?;

        let submit_template = serde_json::to_value(&SubmitCancelArgs {
            confirmation: Some("confirm".to_string()),
            action: action_json.clone(),
            nonce,
            master_signature: None,
            vault_address: None,
        })
        .map_err(|e| format!("[byreal] submit template serialize: {e}"))?;

        let preview = json!({
            "action_kind": "cancel",
            "preview": {
                "coin": args.coin,
                "asset_index": asset_index,
                "oid": args.oid,
            },
            "nonce": nonce,
            "requires_user_confirmation": true,
            "confirmation_phrase": "confirm",
            "submit_args_template": submit_template.clone(),
        });

        let description = format!("Hyperliquid CANCEL {} order #{}", args.coin, args.oid);

        build_signed_routes::<SubmitCancel>(preview, typed_data, description, submit_template)
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
pub(crate) struct SubmitCancelArgs {
    pub confirmation: Option<String>,
    pub action: Value,
    pub nonce: u64,
    pub master_signature: Option<String>,
    pub vault_address: Option<String>,
}

pub(crate) struct SubmitCancel;

impl DynAomiTool for SubmitCancel {
    type App = ByrealApp;
    type Args = SubmitCancelArgs;
    const NAME: &'static str = "byreal_submit_cancel";
    const DESCRIPTION: &'static str = "Submit a Hyperliquid cancel that was prepared by `byreal_build_cancel` and signed via `commit_eip712`.";

    fn run(_app: &Self::App, args: Self::Args, _ctx: DynToolCallCtx) -> Result<Value, String> {
        validate_confirmation(args.confirmation.as_deref())?;
        let sig_hex = args
            .master_signature
            .as_deref()
            .ok_or_else(|| "[byreal] master_signature missing".to_string())?;
        let sig = parse_signature(sig_hex)?;
        let body =
            build_exchange_body(args.action, args.nonce, &sig, args.vault_address.as_deref());
        ok(byreal_client()?.post_exchange(body)?)
    }
}

// ---------------------------------------------------------------------------
// build_update_leverage / submit_update_leverage
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Deserialize, JsonSchema)]
pub(crate) struct BuildUpdateLeverageArgs {
    /// Asset ticker.
    pub coin: String,
    /// Target leverage (1..=max for the asset). Caps vary per asset; check `byreal_get_meta`.
    pub leverage: u32,
    /// true = cross margin, false = isolated margin.
    pub is_cross: bool,
}

pub(crate) struct BuildUpdateLeverage;

impl DynAomiTool for BuildUpdateLeverage {
    type App = ByrealApp;
    type Args = BuildUpdateLeverageArgs;
    const NAME: &'static str = "byreal_build_update_leverage";
    const DESCRIPTION: &'static str = "Build (do not submit) a leverage update for one asset. Apply this BEFORE opening a position so the order opens at the intended leverage.";

    fn run_with_routes(
        _app: &Self::App,
        args: Self::Args,
        _ctx: DynToolCallCtx,
    ) -> Result<ToolReturn, String> {
        if args.leverage == 0 {
            return Err("[byreal] leverage must be >= 1".to_string());
        }
        let client = byreal_client()?;
        let asset_index = client.lookup_asset(&args.coin)?;
        let action = build_update_leverage_action(asset_index, args.is_cross, args.leverage);
        let (action_json, nonce, typed_data) = prepare_l1_action(action, None)?;

        let submit_template = serde_json::to_value(&SubmitUpdateLeverageArgs {
            confirmation: Some("confirm".to_string()),
            action: action_json.clone(),
            nonce,
            master_signature: None,
            vault_address: None,
        })
        .map_err(|e| format!("[byreal] submit template serialize: {e}"))?;

        let preview = json!({
            "action_kind": "update_leverage",
            "preview": {
                "coin": args.coin,
                "asset_index": asset_index,
                "leverage": args.leverage,
                "margin_mode": if args.is_cross { "cross" } else { "isolated" },
            },
            "nonce": nonce,
            "requires_user_confirmation": true,
            "confirmation_phrase": "confirm",
            "submit_args_template": submit_template.clone(),
        });

        let description = format!(
            "Hyperliquid SET LEVERAGE {} = {}x ({})",
            args.coin,
            args.leverage,
            if args.is_cross { "cross" } else { "isolated" },
        );

        build_signed_routes::<SubmitUpdateLeverage>(
            preview,
            typed_data,
            description,
            submit_template,
        )
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
pub(crate) struct SubmitUpdateLeverageArgs {
    pub confirmation: Option<String>,
    pub action: Value,
    pub nonce: u64,
    pub master_signature: Option<String>,
    pub vault_address: Option<String>,
}

pub(crate) struct SubmitUpdateLeverage;

impl DynAomiTool for SubmitUpdateLeverage {
    type App = ByrealApp;
    type Args = SubmitUpdateLeverageArgs;
    const NAME: &'static str = "byreal_submit_update_leverage";
    const DESCRIPTION: &'static str = "Submit a Hyperliquid leverage update prepared by `byreal_build_update_leverage` and signed via `commit_eip712`.";

    fn run(_app: &Self::App, args: Self::Args, _ctx: DynToolCallCtx) -> Result<Value, String> {
        validate_confirmation(args.confirmation.as_deref())?;
        let sig_hex = args
            .master_signature
            .as_deref()
            .ok_or_else(|| "[byreal] master_signature missing".to_string())?;
        let sig = parse_signature(sig_hex)?;
        let body =
            build_exchange_body(args.action, args.nonce, &sig, args.vault_address.as_deref());
        ok(byreal_client()?.post_exchange(body)?)
    }
}

// ===========================================================================
// READ TOOLS — all hit the public /info endpoint, no signing required.
// ===========================================================================

fn ctx_user_address(ctx: &DynToolCallCtx) -> Option<String> {
    ctx.attribute_string(&["domain", "evm", "address"])
}

fn resolve_user(arg: Option<String>, ctx: &DynToolCallCtx) -> Result<String, String> {
    arg.or_else(|| ctx_user_address(ctx)).ok_or_else(|| {
        "[byreal] no user address provided and none in context — pass `user` explicitly".to_string()
    })
}

// -- byreal_get_meta -------------------------------------------------------

#[derive(Debug, Clone, Deserialize, JsonSchema)]
pub(crate) struct GetMetaArgs {}

pub(crate) struct GetMeta;

impl DynAomiTool for GetMeta {
    type App = ByrealApp;
    type Args = GetMetaArgs;
    const NAME: &'static str = "byreal_get_meta";
    const DESCRIPTION: &'static str = "List every tradeable Hyperliquid perpetual asset along with its `szDecimals` (size precision) and `maxLeverage`. Call this once per session to discover the asset universe and to look up size precision before placing an order.";

    fn run(_app: &Self::App, _args: Self::Args, _ctx: DynToolCallCtx) -> Result<Value, String> {
        ok(byreal_client()?.get_meta()?)
    }
}

// -- byreal_get_all_mids ---------------------------------------------------

#[derive(Debug, Clone, Deserialize, JsonSchema)]
pub(crate) struct GetAllMidsArgs {}

pub(crate) struct GetAllMids;

impl DynAomiTool for GetAllMids {
    type App = ByrealApp;
    type Args = GetAllMidsArgs;
    const NAME: &'static str = "byreal_get_all_mids";
    const DESCRIPTION: &'static str = "Get the current mid-price for every listed asset, returned as a `{coin: price_string}` map. Use this to convert a USD notional into a coin size before calling `byreal_build_order`, or to apply slippage to a market order.";

    fn run(_app: &Self::App, _args: Self::Args, _ctx: DynToolCallCtx) -> Result<Value, String> {
        ok(byreal_client()?.get_all_mids()?)
    }
}

// -- byreal_get_l2_book ----------------------------------------------------

#[derive(Debug, Clone, Deserialize, JsonSchema)]
pub(crate) struct GetL2BookArgs {
    /// Asset ticker (e.g. "BTC", "ETH", "SOL").
    pub coin: String,
}

pub(crate) struct GetL2Book;

impl DynAomiTool for GetL2Book {
    type App = ByrealApp;
    type Args = GetL2BookArgs;
    const NAME: &'static str = "byreal_get_l2_book";
    const DESCRIPTION: &'static str = "Snapshot the L2 order book for one asset (top bids and asks with px/sz/n). Use to inspect liquidity depth or pick a limit price near top-of-book.";

    fn run(_app: &Self::App, args: Self::Args, _ctx: DynToolCallCtx) -> Result<Value, String> {
        ok(byreal_client()?.get_l2_book(&args.coin)?)
    }
}

// -- byreal_get_account_state ---------------------------------------------

#[derive(Debug, Clone, Deserialize, JsonSchema)]
pub(crate) struct GetAccountStateArgs {
    /// Ethereum-style address (0x...). Optional — falls back to the connected wallet in context.
    pub user: Option<String>,
}

pub(crate) struct GetAccountState;

impl DynAomiTool for GetAccountState {
    type App = ByrealApp;
    type Args = GetAccountStateArgs;
    const NAME: &'static str = "byreal_get_account_state";
    const DESCRIPTION: &'static str = "Get an address's perp account state: margin summary (account value, total margin used, withdrawable), every open position with size/entry/leverage/liquidation/PnL, and cross-margin parameters. Call before opening a position to verify free margin, and after to confirm the new position.";

    fn run(_app: &Self::App, args: Self::Args, ctx: DynToolCallCtx) -> Result<Value, String> {
        let user = resolve_user(args.user, &ctx)?;
        ok(byreal_client()?.get_account_state(&user)?)
    }
}

// -- byreal_get_open_orders -----------------------------------------------

#[derive(Debug, Clone, Deserialize, JsonSchema)]
pub(crate) struct GetOpenOrdersArgs {
    /// Ethereum-style address (0x...). Optional — falls back to the connected wallet in context.
    pub user: Option<String>,
}

pub(crate) struct GetOpenOrders;

impl DynAomiTool for GetOpenOrders {
    type App = ByrealApp;
    type Args = GetOpenOrdersArgs;
    const NAME: &'static str = "byreal_get_open_orders";
    const DESCRIPTION: &'static str = "List every resting (unfilled) order for an address: coin, side, size, limit price, order ID, timestamp. Use to find the `oid` for `byreal_build_cancel`, or to confirm a freshly-placed limit order is on the book.";

    fn run(_app: &Self::App, args: Self::Args, ctx: DynToolCallCtx) -> Result<Value, String> {
        let user = resolve_user(args.user, &ctx)?;
        ok(byreal_client()?.get_open_orders(&user)?)
    }
}

// -- byreal_get_user_fills ------------------------------------------------

#[derive(Debug, Clone, Deserialize, JsonSchema)]
pub(crate) struct GetUserFillsArgs {
    /// Ethereum-style address (0x...). Optional — falls back to the connected wallet in context.
    pub user: Option<String>,
}

pub(crate) struct GetUserFills;

impl DynAomiTool for GetUserFills {
    type App = ByrealApp;
    type Args = GetUserFillsArgs;
    const NAME: &'static str = "byreal_get_user_fills";
    const DESCRIPTION: &'static str = "Recent trade fill history for an address: each fill's coin, side, px, sz, fee, closedPnl, oid, txHash, timestamp. Use to review what just executed or to compute realised PnL.";

    fn run(_app: &Self::App, args: Self::Args, ctx: DynToolCallCtx) -> Result<Value, String> {
        let user = resolve_user(args.user, &ctx)?;
        ok(byreal_client()?.get_user_fills(&user)?)
    }
}

// -- byreal_get_funding_history -------------------------------------------

#[derive(Debug, Clone, Deserialize, JsonSchema)]
pub(crate) struct GetFundingHistoryArgs {
    /// Asset ticker (e.g. "BTC").
    pub coin: String,
    /// Start timestamp in milliseconds (Unix epoch).
    pub start_time: u64,
    /// End timestamp in milliseconds. Optional — defaults to now.
    pub end_time: Option<u64>,
}

pub(crate) struct GetFundingHistory;

impl DynAomiTool for GetFundingHistory {
    type App = ByrealApp;
    type Args = GetFundingHistoryArgs;
    const NAME: &'static str = "byreal_get_funding_history";
    const DESCRIPTION: &'static str = "Historical funding-rate snapshots for an asset over a time window. The `fundingRate` field is the 8-hour rate; settlement happens hourly at 1/8 of the displayed rate. Annualised ≈ rate × 3 × 365.";

    fn run(_app: &Self::App, args: Self::Args, _ctx: DynToolCallCtx) -> Result<Value, String> {
        ok(byreal_client()?.get_funding_history(&args.coin, args.start_time, args.end_time)?)
    }
}

// -- byreal_get_candles ---------------------------------------------------

#[derive(Debug, Clone, Deserialize, JsonSchema)]
pub(crate) struct GetCandlesArgs {
    /// Asset ticker (e.g. "BTC").
    pub coin: String,
    /// Candle interval: "1m", "5m", "15m", "1h", "4h", "1d".
    pub interval: String,
    /// Start timestamp in milliseconds (Unix epoch).
    pub start_time: u64,
    /// End timestamp in milliseconds.
    pub end_time: u64,
}

pub(crate) struct GetCandles;

impl DynAomiTool for GetCandles {
    type App = ByrealApp;
    type Args = GetCandlesArgs;
    const NAME: &'static str = "byreal_get_candles";
    const DESCRIPTION: &'static str = "OHLCV candle data for one asset over a time window at the given interval. Use for charting context or short-horizon technical reads. Supported intervals: 1m, 5m, 15m, 1h, 4h, 1d.";

    fn run(_app: &Self::App, args: Self::Args, _ctx: DynToolCallCtx) -> Result<Value, String> {
        ok(byreal_client()?.get_candles(
            &args.coin,
            &args.interval,
            args.start_time,
            args.end_time,
        )?)
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validate_confirmation_requires_confirm_token() {
        assert!(validate_confirmation(Some("confirm")).is_ok());
        assert!(validate_confirmation(Some("yes")).is_err());
        assert!(validate_confirmation(None).is_err());
    }

    #[test]
    fn submit_order_args_round_trip() {
        let args = SubmitOrderArgs {
            confirmation: Some("confirm".to_string()),
            action: json!({"type": "order"}),
            nonce: 12345,
            master_signature: Some("0xdead".to_string()),
            vault_address: None,
        };
        let v = serde_json::to_value(&args).unwrap();
        assert_eq!(v["confirmation"], "confirm");
        assert_eq!(v["nonce"], 12345);
        assert_eq!(v["master_signature"], "0xdead");

        let back: SubmitOrderArgs = serde_json::from_value(v).unwrap();
        assert_eq!(back.nonce, 12345);
    }
}
