use aomi_ext::across::AcrossClient;
use aomi_sdk::*;
use aomi_sdk::schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Clone, Default)]
pub(crate) struct AcrossApp;

fn ok<T: Serialize>(value: T) -> Result<Value, String> {
    let value = serde_json::to_value(value)
        .map_err(|e| format!("[across] failed to serialize response: {e}"))?;
    Ok(match value {
        Value::Object(mut map) => {
            map.insert("source".to_string(), Value::String("across".to_string()));
            Value::Object(map)
        }
        other => serde_json::json!({ "source": "across", "data": other }),
    })
}

// ============================================================================
// AcrossListRoutes -- discovery
// ============================================================================

pub(crate) struct AcrossListRoutes;

#[derive(Debug, Deserialize, JsonSchema)]
pub(crate) struct AcrossListRoutesArgs {
    /// Filter by origin chain ID (numeric, e.g. 1 for Ethereum, 42161 for Arbitrum). Optional.
    #[serde(default)]
    pub origin_chain_id: Option<u64>,
    /// Filter by destination chain ID. Optional.
    #[serde(default)]
    pub destination_chain_id: Option<u64>,
    /// Filter by origin ERC-20 address (0x..., checksummed). Optional.
    #[serde(default)]
    pub origin_token: Option<String>,
    /// Filter by destination ERC-20 address. Optional.
    #[serde(default)]
    pub destination_token: Option<String>,
}

impl DynAomiTool for AcrossListRoutes {
    type App = AcrossApp;
    type Args = AcrossListRoutesArgs;
    const NAME: &'static str = "across_list_routes";
    const DESCRIPTION: &'static str = "Use to discover supported Across bridge routes (which token on which origin chain can be bridged to which destination). Returns route entries with origin/destination chain IDs and token addresses. Filter by any combination of chain or token to narrow results.";

    fn run(_app: &AcrossApp, args: Self::Args, _ctx: DynToolCallCtx) -> Result<Value, String> {
        ok(AcrossClient::new()?.get_available_routes(
            args.origin_chain_id,
            args.destination_chain_id,
            args.origin_token.as_deref(),
            args.destination_token.as_deref(),
        )?)
    }
}

// ============================================================================
// AcrossGetLimits
// ============================================================================

pub(crate) struct AcrossGetLimits;

#[derive(Debug, Deserialize, JsonSchema)]
pub(crate) struct AcrossGetLimitsArgs {
    /// Origin-chain ERC-20 address (input token).
    pub input_token: String,
    /// Destination-chain ERC-20 address (output token).
    pub output_token: String,
    /// Origin chain ID (numeric).
    pub origin_chain_id: u64,
    /// Destination chain ID (numeric).
    pub destination_chain_id: u64,
}

impl DynAomiTool for AcrossGetLimits {
    type App = AcrossApp;
    type Args = AcrossGetLimitsArgs;
    const NAME: &'static str = "across_get_limits";
    const DESCRIPTION: &'static str = "Use before quoting a bridge to check that the user's amount is within bounds. Returns minDeposit, maxDeposit, and recommended instant-fill caps for the given origin token / destination token / chain pair, in the input token's smallest unit.";

    fn run(_app: &AcrossApp, args: Self::Args, _ctx: DynToolCallCtx) -> Result<Value, String> {
        ok(AcrossClient::new()?.get_limits(
            &args.input_token,
            &args.output_token,
            args.origin_chain_id,
            args.destination_chain_id,
        )?)
    }
}

// ============================================================================
// AcrossGetBridgeQuote -- fee + output amount + relayer params
// ============================================================================

pub(crate) struct AcrossGetBridgeQuote;

#[derive(Debug, Deserialize, JsonSchema)]
pub(crate) struct AcrossGetBridgeQuoteArgs {
    /// Origin-chain ERC-20 address (input token).
    pub input_token: String,
    /// Destination-chain ERC-20 address (output token).
    pub output_token: String,
    /// Origin chain ID (e.g. 1, 10, 137, 8453, 42161).
    pub origin_chain_id: u64,
    /// Destination chain ID.
    pub destination_chain_id: u64,
    /// Input amount in the input token's smallest unit (1 USDC = "1000000").
    pub amount: String,
    /// Recipient address on the destination chain. Defaults to depositor when omitted.
    #[serde(default)]
    pub recipient: Option<String>,
    /// Optional cross-chain message hex (for atomic actions on destination).
    #[serde(default)]
    pub message: Option<String>,
}

impl DynAomiTool for AcrossGetBridgeQuote {
    type App = AcrossApp;
    type Args = AcrossGetBridgeQuoteArgs;
    const NAME: &'static str = "across_get_bridge_quote";
    const DESCRIPTION: &'static str = "Use when the user wants to bridge a token via Across. Returns `outputAmount`, the fee breakdown (totalRelayFee, lpFee, gasFee, capitalFee), `estimatedFillTimeSec`, and the relayer parameters needed to call SpokePool. To execute, the host must call `depositV3` (or `deposit`) on the origin-chain SpokePool with these parameters; this tool does NOT return raw calldata.";

    fn run(_app: &AcrossApp, args: Self::Args, _ctx: DynToolCallCtx) -> Result<Value, String> {
        ok(AcrossClient::new()?.get_suggested_fees(
            &args.input_token,
            &args.output_token,
            args.origin_chain_id,
            args.destination_chain_id,
            &args.amount,
            args.recipient.as_deref(),
            args.message.as_deref(),
        )?)
    }
}

// ============================================================================
// AcrossGetDepositStatus
// ============================================================================

pub(crate) struct AcrossGetDepositStatus;

#[derive(Debug, Deserialize, JsonSchema)]
pub(crate) struct AcrossGetDepositStatusArgs {
    /// Origin chain ID where the deposit was made.
    pub origin_chain_id: u64,
    /// Numeric deposit ID emitted by the origin SpokePool when the deposit tx confirmed.
    pub deposit_id: u64,
}

impl DynAomiTool for AcrossGetDepositStatus {
    type App = AcrossApp;
    type Args = AcrossGetDepositStatusArgs;
    const NAME: &'static str = "across_get_deposit_status";
    const DESCRIPTION: &'static str = "Use to track an Across bridge deposit. Returns the fill status (PENDING / FILLED / etc.) and, when filled, the destination-chain fill tx hash. Poll while the user is waiting (typical fill is under 30 seconds for instant-eligible amounts).";

    fn run(_app: &AcrossApp, args: Self::Args, _ctx: DynToolCallCtx) -> Result<Value, String> {
        ok(AcrossClient::new()?.get_deposit_status(args.origin_chain_id, args.deposit_id)?)
    }
}
