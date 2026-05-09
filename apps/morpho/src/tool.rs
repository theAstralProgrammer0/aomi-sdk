use aomi_ext::morpho::MorphoClient;
use aomi_sdk::*;
use aomi_sdk::schemars::JsonSchema;
use serde::Deserialize;
use serde_json::Value;

#[derive(Clone, Default)]
pub(crate) struct MorphoApp;

// ============================================================================
// GetMorphoMarkets
// ============================================================================

pub(crate) struct GetMorphoMarkets;

#[derive(Debug, Deserialize, JsonSchema)]
pub(crate) struct GetMorphoMarketsArgs {}

impl DynAomiTool for GetMorphoMarkets {
    type App = MorphoApp;
    type Args = GetMorphoMarketsArgs;
    const NAME: &'static str = "get_markets";
    const DESCRIPTION: &'static str =
        "List all Morpho lending markets with LTV, supply/borrow APY, and available liquidity.";

    fn run(_app: &MorphoApp, _args: Self::Args, _ctx: DynToolCallCtx) -> Result<Value, String> {
        MorphoClient::new()?.get_markets()
    }
}

// ============================================================================
// GetMorphoVaults
// ============================================================================

pub(crate) struct GetMorphoVaults;

#[derive(Debug, Deserialize, JsonSchema)]
pub(crate) struct GetMorphoVaultsArgs {}

impl DynAomiTool for GetMorphoVaults {
    type App = MorphoApp;
    type Args = GetMorphoVaultsArgs;
    const NAME: &'static str = "get_vaults";
    const DESCRIPTION: &'static str =
        "List Morpho vaults with APY, TVL, and allocation strategy details.";

    fn run(_app: &MorphoApp, _args: Self::Args, _ctx: DynToolCallCtx) -> Result<Value, String> {
        MorphoClient::new()?.get_vaults()
    }
}

// ============================================================================
// GetMorphoUserPositions
// ============================================================================

pub(crate) struct GetMorphoUserPositions;

#[derive(Debug, Deserialize, JsonSchema)]
pub(crate) struct GetMorphoUserPositionsArgs {
    /// Ethereum wallet address (e.g. "0xabc...def")
    pub(crate) address: String,
}

impl DynAomiTool for GetMorphoUserPositions {
    type App = MorphoApp;
    type Args = GetMorphoUserPositionsArgs;
    const NAME: &'static str = "get_user_positions";
    const DESCRIPTION: &'static str = "Get a user's Morpho positions including deposits, borrows, and vault holdings for a given wallet address.";

    fn run(_app: &MorphoApp, args: Self::Args, _ctx: DynToolCallCtx) -> Result<Value, String> {
        MorphoClient::new()?.get_user_positions(&args.address)
    }
}
