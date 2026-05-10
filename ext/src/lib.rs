//! Shared client and API-layer types for Aomi apps that wrap external services.
//!
//! Each provider lives behind a Cargo feature so apps only compile in the
//! deps they actually need.

#[cfg(feature = "across")]
pub mod across;
#[cfg(feature = "binance")]
pub mod binance;
#[cfg(feature = "bybit")]
pub mod bybit;
#[cfg(feature = "cow")]
pub mod cow;
#[cfg(feature = "defillama")]
pub mod defillama;
#[cfg(feature = "dune")]
pub mod dune;
#[cfg(feature = "dydx")]
pub mod dydx;
#[cfg(feature = "gmx")]
pub mod gmx;
#[cfg(feature = "hyperliquid")]
pub mod hyperliquid;
#[cfg(feature = "kalshi")]
pub mod kalshi;
#[cfg(feature = "lifi")]
pub mod lifi;
#[cfg(feature = "manifold")]
pub mod manifold;
#[cfg(feature = "morpho")]
pub mod morpho;
#[cfg(feature = "neynar")]
pub mod neynar;
#[cfg(feature = "okx")]
pub mod okx;
#[cfg(feature = "oneinch")]
pub mod oneinch;
#[cfg(feature = "x")]
pub mod x;
#[cfg(feature = "yearn")]
pub mod yearn;
#[cfg(feature = "zerox")]
pub mod zerox;
#[cfg(feature = "khalani")]
pub mod khalani;
