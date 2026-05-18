use aomi_sdk::*;
use std::sync::LazyLock;

mod tool;

static PREAMBLE: LazyLock<String> = LazyLock::new(tool::build_preamble);

const SECRET_API_KEY: Secret = Secret::new(
    "SIMMER_API_KEY",
    "Simmer SDK API key for Kalshi prediction market trading.",
    true,
);

dyn_aomi_app!(
    app = tool::KalshiApp,
    name = "kalshi",
    version = "0.1.0",
    preamble = PREAMBLE.as_str(),
    tools = [
        tool::SimmerRegister,
        tool::SimmerStatus,
        tool::SimmerBriefing,
        tool::ImportKalshiMarket,
        tool::FetchSimmerMarketContext,
        tool::SimmerPlaceOrder,
        tool::SimmerGetPositions,
        tool::SimmerGetPortfolio,
        tool::SearchSimmerMarkets,
    ],
    secrets = [SECRET_API_KEY],namespaces = ["evm-core"]
);
