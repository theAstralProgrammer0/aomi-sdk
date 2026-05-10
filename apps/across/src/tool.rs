//! Curated tool layer for Across Protocol. Hand-written from the
//! progenitor-generated client at `aomi_ext::across::Client` — see
//! ext/specs/across.yaml for the full surface.
//!
//! Across is unauthenticated; the client just takes a base URL.

use aomi_ext::across::Client as AcrossClient;
use aomi_sdk::*;
use aomi_sdk::schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Clone, Default)]
pub(crate) struct AcrossApp;

const BASE_URL: &str = "https://app.across.to/api";

// ============================================================================
// Helpers
// ============================================================================

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

fn rt() -> Result<tokio::runtime::Runtime, String> {
    tokio::runtime::Runtime::new().map_err(|e| format!("[across] runtime: {e}"))
}

fn client() -> AcrossClient {
    let base = std::env::var("ACROSS_API_ENDPOINT").unwrap_or_else(|_| BASE_URL.to_string());
    AcrossClient::new(&base)
}

// ============================================================================
// AcrossListRoutes — discovery
// ============================================================================

pub(crate) struct AcrossListRoutes;

#[derive(Debug, Deserialize, JsonSchema)]
pub(crate) struct AcrossListRoutesArgs {
    /// Filter by origin chain ID (numeric, e.g. 1 for Ethereum, 42161 for Arbitrum). Optional.
    #[serde(default)]
    pub origin_chain_id: Option<i64>,
    /// Filter by destination chain ID. Optional.
    #[serde(default)]
    pub destination_chain_id: Option<i64>,
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
        let runtime = rt()?;
        runtime.block_on(async move {
            let routes = client()
                .get_available_routes(
                    args.destination_chain_id,
                    args.destination_token.as_deref(),
                    args.origin_chain_id,
                    args.origin_token.as_deref(),
                )
                .await
                .map_err(|e| format!("[across] available routes: {e}"))?
                .into_inner();
            ok(routes)
        })
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
    pub origin_chain_id: i64,
    /// Destination chain ID (numeric).
    pub destination_chain_id: i64,
}

impl DynAomiTool for AcrossGetLimits {
    type App = AcrossApp;
    type Args = AcrossGetLimitsArgs;
    const NAME: &'static str = "across_get_limits";
    const DESCRIPTION: &'static str = "Use before quoting a bridge to check that the user's amount is within bounds. Returns minDeposit, maxDeposit, and recommended instant-fill caps for the given origin token / destination token / chain pair, in the input token's smallest unit.";

    fn run(_app: &AcrossApp, args: Self::Args, _ctx: DynToolCallCtx) -> Result<Value, String> {
        let runtime = rt()?;
        runtime.block_on(async move {
            let limits = client()
                .get_limits(
                    args.destination_chain_id,
                    args.input_token.as_str(),
                    args.origin_chain_id,
                    args.output_token.as_str(),
                )
                .await
                .map_err(|e| format!("[across] limits: {e}"))?
                .into_inner();
            ok(limits)
        })
    }
}

// ============================================================================
// AcrossGetBridgeQuote — fee + output amount + relayer params
// ============================================================================

pub(crate) struct AcrossGetBridgeQuote;

#[derive(Debug, Deserialize, JsonSchema)]
pub(crate) struct AcrossGetBridgeQuoteArgs {
    /// Origin-chain ERC-20 address (input token).
    pub input_token: String,
    /// Destination-chain ERC-20 address (output token).
    pub output_token: String,
    /// Origin chain ID (e.g. 1, 10, 137, 8453, 42161).
    pub origin_chain_id: i64,
    /// Destination chain ID.
    pub destination_chain_id: i64,
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
        let runtime = rt()?;
        runtime.block_on(async move {
            let fees = client()
                .get_suggested_fees(
                    args.amount.as_str(),
                    args.destination_chain_id,
                    args.input_token.as_str(),
                    args.message.as_deref(),
                    args.origin_chain_id,
                    args.output_token.as_str(),
                    args.recipient.as_deref(),
                )
                .await
                .map_err(|e| format!("[across] suggested fees: {e}"))?
                .into_inner();
            ok(fees)
        })
    }
}

// ============================================================================
// AcrossGetDepositStatus
// ============================================================================

pub(crate) struct AcrossGetDepositStatus;

#[derive(Debug, Deserialize, JsonSchema)]
pub(crate) struct AcrossGetDepositStatusArgs {
    /// Origin chain ID where the deposit was made.
    pub origin_chain_id: i64,
    /// Numeric deposit ID emitted by the origin SpokePool when the deposit tx confirmed.
    pub deposit_id: i64,
}

impl DynAomiTool for AcrossGetDepositStatus {
    type App = AcrossApp;
    type Args = AcrossGetDepositStatusArgs;
    const NAME: &'static str = "across_get_deposit_status";
    const DESCRIPTION: &'static str = "Use to track an Across bridge deposit. Returns the fill status (PENDING / FILLED / etc.) and, when filled, the destination-chain fill tx hash. Poll while the user is waiting (typical fill is under 30 seconds for instant-eligible amounts).";

    fn run(_app: &AcrossApp, args: Self::Args, _ctx: DynToolCallCtx) -> Result<Value, String> {
        let runtime = rt()?;
        runtime.block_on(async move {
            let status = client()
                .get_deposit_status(args.deposit_id, args.origin_chain_id)
                .await
                .map_err(|e| format!("[across] deposit status: {e}"))?
                .into_inner();
            ok(status)
        })
    }
}
