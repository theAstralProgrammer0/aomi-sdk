use aomi_ext::yearn::YearnClient;
use aomi_sdk::*;
use aomi_sdk::schemars::JsonSchema;
use serde::Deserialize;
use serde_json::Value;

#[derive(Clone, Default)]
pub(crate) struct YearnApp;

fn default_chain_id() -> u64 {
    1
}

// ============================================================================
// GetAllVaults
// ============================================================================

pub(crate) struct GetAllVaults;

#[derive(Debug, Deserialize, JsonSchema)]
pub(crate) struct GetAllVaultsArgs {
    /// Chain ID to query. Supported: 1 (Ethereum), 10 (Optimism), 137 (Polygon), 250 (Fantom), 8453 (Base), 42161 (Arbitrum). Default: 1.
    #[serde(default = "default_chain_id")]
    pub(crate) chain_id: u64,
}

impl DynAomiTool for GetAllVaults {
    type App = YearnApp;
    type Args = GetAllVaultsArgs;
    const NAME: &'static str = "get_all_vaults";
    const DESCRIPTION: &'static str =
        "List all Yearn vaults on a given chain with TVL, APY, strategies, and fees.";

    fn run(_app: &YearnApp, args: Self::Args, _ctx: DynToolCallCtx) -> Result<Value, String> {
        YearnClient::new()?.get_all_vaults(args.chain_id)
    }
}

// ============================================================================
// GetVaultDetail
// ============================================================================

pub(crate) struct GetVaultDetail;

#[derive(Debug, Deserialize, JsonSchema)]
pub(crate) struct GetVaultDetailArgs {
    /// Chain ID to query. Supported: 1 (Ethereum), 10 (Optimism), 137 (Polygon), 250 (Fantom), 8453 (Base), 42161 (Arbitrum). Default: 1.
    #[serde(default = "default_chain_id")]
    pub(crate) chain_id: u64,
    /// The vault contract address (e.g. "0x...")
    pub(crate) address: String,
}

impl DynAomiTool for GetVaultDetail {
    type App = YearnApp;
    type Args = GetVaultDetailArgs;
    const NAME: &'static str = "get_vault_detail";
    const DESCRIPTION: &'static str = "Get detailed info for a specific Yearn vault by address: APY breakdown (net, gross, weekly, monthly, inception), strategies, fees, and TVL.";

    fn run(_app: &YearnApp, args: Self::Args, _ctx: DynToolCallCtx) -> Result<Value, String> {
        YearnClient::new()?.get_vault_detail(args.chain_id, &args.address)
    }
}

// ============================================================================
// GetBlacklistedVaults
// ============================================================================

pub(crate) struct GetBlacklistedVaults;

#[derive(Debug, Deserialize, JsonSchema)]
pub(crate) struct GetBlacklistedVaultsArgs {}

impl DynAomiTool for GetBlacklistedVaults {
    type App = YearnApp;
    type Args = GetBlacklistedVaultsArgs;
    const NAME: &'static str = "get_blacklisted_vaults";
    const DESCRIPTION: &'static str = "Get the list of blacklisted Yearn vaults across all chains.";

    fn run(_app: &YearnApp, _args: Self::Args, _ctx: DynToolCallCtx) -> Result<Value, String> {
        YearnClient::new()?.get_blacklisted_vaults()
    }
}
