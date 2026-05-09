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
// GetAcrossBridgeQuote
// ============================================================================

pub(crate) struct GetAcrossBridgeQuote;

#[derive(Debug, Deserialize, JsonSchema)]
pub(crate) struct GetAcrossBridgeQuoteArgs {
    #[schemars(description = "ERC-20 token address on the origin chain (input token)")]
    pub input_token: String,

    #[schemars(description = "ERC-20 token address on the destination chain (output token)")]
    pub output_token: String,

    #[schemars(
        description = "Origin chain ID (e.g. 1 for Ethereum, 42161 for Arbitrum, 10 for Optimism, 137 for Polygon, 8453 for Base)"
    )]
    pub origin_chain_id: u64,

    #[schemars(
        description = "Destination chain ID (e.g. 1 for Ethereum, 42161 for Arbitrum, 10 for Optimism, 137 for Polygon, 8453 for Base)"
    )]
    pub destination_chain_id: u64,

    #[schemars(description = "Amount in the token's smallest unit (e.g. wei for ETH)")]
    pub amount: String,

    #[schemars(description = "Recipient address on the destination chain. Optional.")]
    #[serde(default)]
    pub recipient: Option<String>,

    #[schemars(description = "Optional message for cross-chain actions")]
    #[serde(default)]
    pub message: Option<String>,
}

impl DynAomiTool for GetAcrossBridgeQuote {
    type App = AcrossApp;
    type Args = GetAcrossBridgeQuoteArgs;
    const NAME: &'static str = "get_across_bridge_quote";
    const DESCRIPTION: &'static str = "Get a bridge fee quote from Across Protocol. Returns suggested fees, estimated fill time, and fee breakdown for a cross-chain transfer.";

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
// GetAcrossBridgeLimits
// ============================================================================

pub(crate) struct GetAcrossBridgeLimits;

#[derive(Debug, Deserialize, JsonSchema)]
pub(crate) struct GetAcrossBridgeLimitsArgs {
    #[schemars(description = "ERC-20 token address on the origin chain (input token)")]
    pub input_token: String,

    #[schemars(description = "ERC-20 token address on the destination chain (output token)")]
    pub output_token: String,

    #[schemars(
        description = "Origin chain ID (e.g. 1 for Ethereum, 42161 for Arbitrum, 10 for Optimism)"
    )]
    pub origin_chain_id: u64,

    #[schemars(
        description = "Destination chain ID (e.g. 1 for Ethereum, 42161 for Arbitrum, 10 for Optimism)"
    )]
    pub destination_chain_id: u64,
}

impl DynAomiTool for GetAcrossBridgeLimits {
    type App = AcrossApp;
    type Args = GetAcrossBridgeLimitsArgs;
    const NAME: &'static str = "get_across_bridge_limits";
    const DESCRIPTION: &'static str =
        "Get minimum and maximum transfer limits for a specific token route on Across Protocol.";

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
// GetAcrossDepositStatus
// ============================================================================

pub(crate) struct GetAcrossDepositStatus;

#[derive(Debug, Deserialize, JsonSchema)]
pub(crate) struct GetAcrossDepositStatusArgs {
    #[schemars(description = "Origin chain ID where the deposit was made")]
    pub origin_chain_id: u64,

    #[schemars(description = "Deposit ID to track")]
    pub deposit_id: u64,
}

impl DynAomiTool for GetAcrossDepositStatus {
    type App = AcrossApp;
    type Args = GetAcrossDepositStatusArgs;
    const NAME: &'static str = "get_across_deposit_status";
    const DESCRIPTION: &'static str = "Track the status of a bridge deposit on Across Protocol. Returns fill status and corresponding fill transaction hash if filled.";

    fn run(_app: &AcrossApp, args: Self::Args, _ctx: DynToolCallCtx) -> Result<Value, String> {
        ok(AcrossClient::new()?.get_deposit_status(args.origin_chain_id, args.deposit_id)?)
    }
}

// ============================================================================
// GetAcrossAvailableRoutes
// ============================================================================

pub(crate) struct GetAcrossAvailableRoutes;

#[derive(Debug, Deserialize, JsonSchema)]
pub(crate) struct GetAcrossAvailableRoutesArgs {
    #[schemars(description = "Filter by origin chain ID. Optional.")]
    #[serde(default)]
    pub origin_chain_id: Option<u64>,

    #[schemars(description = "Filter by destination chain ID. Optional.")]
    #[serde(default)]
    pub destination_chain_id: Option<u64>,

    #[schemars(description = "Filter by origin token address. Optional.")]
    #[serde(default)]
    pub origin_token: Option<String>,

    #[schemars(description = "Filter by destination token address. Optional.")]
    #[serde(default)]
    pub destination_token: Option<String>,
}

impl DynAomiTool for GetAcrossAvailableRoutes {
    type App = AcrossApp;
    type Args = GetAcrossAvailableRoutesArgs;
    const NAME: &'static str = "get_across_available_routes";
    const DESCRIPTION: &'static str = "List available bridge routes on Across Protocol. Optionally filter by origin/destination chain ID or token address.";

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
// GetAcrossTokenPrice
// ============================================================================

pub(crate) struct GetAcrossTokenPrice;

#[derive(Debug, Deserialize, JsonSchema)]
pub(crate) struct GetAcrossTokenPriceArgs {
    #[schemars(
        description = "L1 (Ethereum mainnet) token address. Optional if l2_token is provided."
    )]
    #[serde(default)]
    pub l1_token: Option<String>,

    #[schemars(description = "L2 token address. Optional if l1_token is provided.")]
    #[serde(default)]
    pub l2_token: Option<String>,
}

impl DynAomiTool for GetAcrossTokenPrice {
    type App = AcrossApp;
    type Args = GetAcrossTokenPriceArgs;
    const NAME: &'static str = "get_across_token_price";
    const DESCRIPTION: &'static str = "Get token price from Across Protocol's coingecko endpoint. Provide either an L1 or L2 token address.";

    fn run(_app: &AcrossApp, args: Self::Args, _ctx: DynToolCallCtx) -> Result<Value, String> {
        if args.l1_token.is_none() && args.l2_token.is_none() {
            return Err("[across] token price failed: at least one of l1_token or l2_token must be provided".to_string());
        }
        ok(AcrossClient::new()?
            .get_coingecko_price(args.l1_token.as_deref(), args.l2_token.as_deref())?)
    }
}
