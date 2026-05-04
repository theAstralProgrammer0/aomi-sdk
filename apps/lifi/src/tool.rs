use crate::client::*;
use crate::types::PreparedTransaction;
use aomi_sdk::*;
use serde::Serialize;
use serde_json::{Value, json};

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

fn json_string(value: &Value) -> Option<String> {
    match value {
        Value::String(text) => Some(text.clone()),
        Value::Number(number) => Some(number.to_string()),
        _ => None,
    }
}

fn stage_tx_args(tx: &PreparedTransaction, kind: &str) -> Option<Value> {
    Some(json!({
        "to": json_string(&tx.to)?,
        "description": tx.description,
        "data": {
            "raw": json_string(&tx.data)?,
        },
        "value": json_string(&tx.value),
        "gas_limit": json_string(&tx.gas_limit),
        "kind": kind,
    }))
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

impl DynAomiTool for PlaceLifiOrder {
    type App = LifiApp;
    type Args = PlaceLifiOrderArgs;
    const NAME: &'static str = "place_lifi_order";
    const DESCRIPTION: &'static str = "Get executable tx data via LI.FI for swaps or bridges. Returns approval_tx (if needed), main_tx, and exact `stage_tx` argument templates. Stage those txs with `stage_tx`, verify them with `simulate_batch`, then finalize with `commit_txs`.";

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

        let approval_stage = payload
            .approval_tx
            .as_ref()
            .and_then(|tx| stage_tx_args(tx, "erc20_approve"));
        let main_stage = stage_tx_args(&payload.main_tx, "bridge");

        ok(json!({
            "payload": payload,
            "approval_tx": approval_tx,
            "main_tx": main_tx,
            "stage_plan": {
                "steps": [
                    {
                        "name": "approval",
                        "required": approval_stage.is_some(),
                        "stage_tx_args": approval_stage,
                    },
                    {
                        "name": "main",
                        "required": true,
                        "stage_tx_args": main_stage,
                    }
                ],
                "next_step": "Stage each non-null step with stage_tx, simulate the staged pending_tx_id list with simulate_batch, then call commit_txs using the staged ids. Do not re-encode LI.FI calldata."
            },
            "note": "For executable bridge or swap flows, copy the provided stage_plan.stage_tx_args directly into stage_tx. Use simulate_batch before commit_txs. Do not re-encode LI.FI calldata.",
        }))
    }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stage_tx_args_uses_raw_calldata_shape() {
        let tx = PreparedTransaction {
            to: Value::String("0x1111111111111111111111111111111111111111".to_string()),
            data: Value::String("0xdeadbeef".to_string()),
            value: Value::String("0".to_string()),
            gas_limit: Value::String("210000".to_string()),
            description: "LI.FI main transaction",
        };

        let args = stage_tx_args(&tx, "bridge").expect("stage args");

        assert_eq!(
            args,
            json!({
                "to": "0x1111111111111111111111111111111111111111",
                "description": "LI.FI main transaction",
                "data": {
                    "raw": "0xdeadbeef"
                },
                "value": "0",
                "gas_limit": "210000",
                "kind": "bridge"
            })
        );
    }
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

impl DynAomiTool for GetLifiTokens {
    type App = LifiApp;
    type Args = GetLifiTokensArgs;
    const NAME: &'static str = "get_lifi_tokens";
    const DESCRIPTION: &'static str = "List supported tokens on LI.FI. Optionally filter by chain IDs (comma-separated) or chain type.";

    fn run(_app: &LifiApp, args: Self::Args, _ctx: DynToolCallCtx) -> Result<Value, String> {
        ok(LifiClient::new()?.get_tokens(args.chains.as_deref(), args.chain_types.as_deref())?)
    }
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

impl DynAomiTool for GetLifiStepTransaction {
    type App = LifiApp;
    type Args = GetLifiStepTransactionArgs;
    const NAME: &'static str = "get_lifi_step_transaction";
    const DESCRIPTION: &'static str = "Get executable transaction data for a single route step returned by get_lifi_routes. Pass the step object directly.";

    fn run(_app: &LifiApp, args: Self::Args, _ctx: DynToolCallCtx) -> Result<Value, String> {
        ok(LifiClient::new()?.get_step_transaction(&args.step)?)
    }
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
