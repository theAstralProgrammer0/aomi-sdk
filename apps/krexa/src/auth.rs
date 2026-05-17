//! Hand-written companion to the generated `client.rs`.
//!
//! Krexa runs two API tiers:
//!   - Anonymous: 30 req/min, no key needed. The public reads (score,
//!     credit eligibility, vault stats, KYA lookup, …) work in this tier.
//!   - Authenticated: 60+ req/min and access to the Pay.sh authenticated
//!     surface (`call`, `discover`, `history`, `budget`, `confirm`,
//!     `tier`). Carry `X-API-Key: kx_...` on every authed request.
//!
//! Unlike the HMAC-signed venues (Binance, Bybit, OKX, Limitless) Krexa
//! does **not** compute a per-request signature — the key is a static
//! bearer. This module therefore only resolves the key from the
//! environment; there is no `sign(...)` or `current_timestamp_ms()`.
//!
//! Provisioning the key itself requires an Ed25519 signature over a
//! literal challenge message — that would belong in this file under a
//! `sign_provision_message(...)` helper, but Krex does not provision keys
//! at runtime so it is deferred. See `apps/krexa/openapi.meta.json`.

/// Env var that holds the `kx_`-prefixed key. Provisioned via
/// `POST /access/provision-key` (offline, by the operator) or returned
/// from `POST /solana/paysh/onboard`.
pub const API_KEY_ENV: &str = "KREXA_API_KEY";

/// Read the API key from the env, returning a tool-shaped `Result` so the
/// curated tool layer can propagate the error with `?`.
pub fn api_key() -> Result<String, String> {
    std::env::var(API_KEY_ENV).map_err(|_| {
        format!(
            "[krexa] {API_KEY_ENV} not set; Pay.sh authenticated endpoints \
             require an X-API-Key header (`kx_...`)."
        )
    })
}
