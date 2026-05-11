use aomi_ext::morpho::MorphoClient;
use aomi_sdk::schemars::JsonSchema;
use aomi_sdk::*;
use serde::Deserialize;
use serde_json::{Value, json};

#[derive(Clone, Default)]
pub(crate) struct MorphoApp;

// ============================================================================
// morpho_list_markets — all Morpho lending markets
// ============================================================================

pub(crate) struct ListMarkets;

#[derive(Debug, Deserialize, JsonSchema)]
pub(crate) struct ListMarketsArgs {}

impl DynAomiTool for ListMarkets {
    type App = MorphoApp;
    type Args = ListMarketsArgs;
    const NAME: &'static str = "morpho_list_markets";
    const DESCRIPTION: &'static str = "List all Morpho Blue lending markets with collateral/loan asset, LLTV, available liquidity, and live supply/borrow APY. Use when the user asks about Morpho rates, wants to compare markets, or asks 'where can I borrow X against Y'.";

    fn run(_app: &MorphoApp, _args: Self::Args, _ctx: DynToolCallCtx) -> Result<Value, String> {
        MorphoClient::new()?.get_markets()
    }
}

// ============================================================================
// morpho_list_vaults — Morpho vaults with APY/TVL/allocations
// ============================================================================

pub(crate) struct ListVaults;

#[derive(Debug, Deserialize, JsonSchema)]
pub(crate) struct ListVaultsArgs {}

impl DynAomiTool for ListVaults {
    type App = MorphoApp;
    type Args = ListVaultsArgs;
    const NAME: &'static str = "morpho_list_vaults";
    const DESCRIPTION: &'static str = "List Morpho MetaMorpho vaults — curated baskets that allocate one asset across multiple Morpho markets. Returns name, symbol, underlying asset, gross/net APY, total assets in USD, and the allocation breakdown. Use when the user asks 'best Morpho vault for USDC' or wants to compare vault strategies.";

    fn run(_app: &MorphoApp, _args: Self::Args, _ctx: DynToolCallCtx) -> Result<Value, String> {
        MorphoClient::new()?.get_vaults()
    }
}

// ============================================================================
// morpho_get_user_positions — raw position data for an address
// ============================================================================

pub(crate) struct GetUserPositions;

#[derive(Debug, Deserialize, JsonSchema)]
pub(crate) struct GetUserPositionsArgs {
    /// Ethereum wallet address (0x-prefixed, any case).
    pub address: String,
}

impl DynAomiTool for GetUserPositions {
    type App = MorphoApp;
    type Args = GetUserPositionsArgs;
    const NAME: &'static str = "morpho_get_user_positions";
    const DESCRIPTION: &'static str = "Return a wallet's raw Morpho positions — every market position (supply, borrow, collateral in USD) and vault position. Use when the user wants the full per-market breakdown. For a one-shot 'what's my net Morpho exposure', prefer `morpho_position_summary`.";

    fn run(_app: &MorphoApp, args: Self::Args, _ctx: DynToolCallCtx) -> Result<Value, String> {
        MorphoClient::new()?.get_user_positions(&args.address)
    }
}

// ============================================================================
// morpho_position_summary — composite: positions + aggregates
// ============================================================================

pub(crate) struct PositionSummary;

#[derive(Debug, Deserialize, JsonSchema)]
pub(crate) struct PositionSummaryArgs {
    /// Ethereum wallet address (0x-prefixed, any case).
    pub address: String,
}

fn sum_field(items: &[Value], field: &str) -> f64 {
    items
        .iter()
        .filter_map(|p| p.get(field).and_then(Value::as_f64))
        .sum()
}

impl DynAomiTool for PositionSummary {
    type App = MorphoApp;
    type Args = PositionSummaryArgs;
    const NAME: &'static str = "morpho_position_summary";
    const DESCRIPTION: &'static str = "One-shot snapshot of a wallet's Morpho exposure: total supplied, borrowed, collateral, and net USD across all markets, plus vault holdings. Use when the user asks 'how much do I have on Morpho' or 'what's my net Morpho position'. Wraps `get_user_positions` and aggregates the figures.";

    fn run(_app: &MorphoApp, args: Self::Args, _ctx: DynToolCallCtx) -> Result<Value, String> {
        let raw = MorphoClient::new()?.get_user_positions(&args.address)?;

        let user = raw.get("user").cloned().unwrap_or(Value::Null);
        let market_positions: Vec<Value> = user
            .get("marketPositions")
            .and_then(Value::as_array)
            .cloned()
            .unwrap_or_default();
        let vault_positions: Vec<Value> = user
            .get("vaultPositions")
            .and_then(Value::as_array)
            .cloned()
            .unwrap_or_default();

        let total_supplied = sum_field(&market_positions, "supplyAssetsUsd");
        let total_borrowed = sum_field(&market_positions, "borrowAssetsUsd");
        let total_collateral = sum_field(&market_positions, "collateralUsd");
        let total_vault = sum_field(&vault_positions, "assetsUsd");

        let net_market_usd = total_supplied + total_collateral - total_borrowed;
        let net_total_usd = net_market_usd + total_vault;

        Ok(json!({
            "source": "morpho",
            "address": args.address,
            "totals_usd": {
                "supplied": total_supplied,
                "borrowed": total_borrowed,
                "collateral": total_collateral,
                "vault_holdings": total_vault,
                "net_market": net_market_usd,
                "net_total": net_total_usd,
            },
            "market_position_count": market_positions.len(),
            "vault_position_count": vault_positions.len(),
            "market_positions": market_positions,
            "vault_positions": vault_positions,
        }))
    }
}
