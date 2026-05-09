use crate::oneinch::types::{
    OneInchAllowanceResponse, OneInchLiquiditySourcesResponse, OneInchQuoteResponse,
    OneInchSwapResponse, OneInchTokensResponse, OneInchTransaction,
};
use serde::de::DeserializeOwned;
use std::time::Duration;

// ============================================================================
// 1inch HTTP Client (blocking)
// ============================================================================

pub const BASE_URL: &str = "https://api.1inch.dev/swap/v6.0";

/// Supported chain IDs for 1inch Swap API v6.0.
pub const SUPPORTED_CHAINS: &[u64] = &[1, 10, 56, 100, 137, 8453, 42161, 43114];

#[derive(Clone)]
pub struct OneInchClient {
    pub http: reqwest::blocking::Client,
    pub base_url: String,
    pub api_key: String,
}

impl OneInchClient {
    pub fn new(api_key: String) -> Result<Self, String> {
        let http = reqwest::blocking::Client::builder()
            .timeout(Duration::from_secs(30))
            .build()
            .map_err(|e| format!("[1inch] failed to build HTTP client: {e}"))?;
        Ok(Self {
            http,
            base_url: std::env::var("ONEINCH_API_ENDPOINT")
                .unwrap_or_else(|_| BASE_URL.to_string()),
            api_key,
        })
    }

    fn chain_url(&self, chain_id: u64) -> String {
        format!("{}/{chain_id}", self.base_url)
    }

    fn validate_chain(chain_id: u64) -> Result<(), String> {
        if SUPPORTED_CHAINS.contains(&chain_id) {
            Ok(())
        } else {
            Err(format!(
                "[1inch] unsupported chain_id {chain_id}. Supported: {SUPPORTED_CHAINS:?}"
            ))
        }
    }

    fn send_json<T: DeserializeOwned>(
        request: reqwest::blocking::RequestBuilder,
        operation: &str,
    ) -> Result<T, String> {
        let response = request
            .send()
            .map_err(|e| format!("[1inch] {operation} failed: {e}"))?;

        let status = response.status();
        let body = response.text().unwrap_or_default();
        if !status.is_success() {
            return Err(format!("[1inch] {operation} failed: {status} {body}"));
        }

        serde_json::from_str::<T>(&body)
            .map_err(|e| format!("[1inch] {operation} decode failed: {e}; body: {body}"))
    }

    // ========================================================================
    // Endpoints
    // ========================================================================

    pub fn get_quote(
        &self,
        chain_id: u64,
        src: &str,
        dst: &str,
        amount: &str,
        protocols: Option<&str>,
    ) -> Result<OneInchQuoteResponse, String> {
        Self::validate_chain(chain_id)?;
        let mut request = self
            .http
            .get(format!("{}/quote", self.chain_url(chain_id)))
            .bearer_auth(&self.api_key)
            .query(&[("src", src), ("dst", dst), ("amount", amount)]);
        if let Some(protocols) = protocols {
            request = request.query(&[("protocols", protocols)]);
        }
        Self::send_json(request, "quote")
    }

    #[allow(clippy::too_many_arguments)]
    pub fn get_swap(
        &self,
        chain_id: u64,
        src: &str,
        dst: &str,
        amount: &str,
        from: &str,
        slippage: f64,
        protocols: Option<&str>,
    ) -> Result<OneInchSwapResponse, String> {
        Self::validate_chain(chain_id)?;
        let mut request = self
            .http
            .get(format!("{}/swap", self.chain_url(chain_id)))
            .bearer_auth(&self.api_key)
            .query(&[
                ("src", src),
                ("dst", dst),
                ("amount", amount),
                ("from", from),
                ("slippage", &slippage.to_string()),
            ]);
        if let Some(protocols) = protocols {
            request = request.query(&[("protocols", protocols)]);
        }
        Self::send_json(request, "swap")
    }

    pub fn get_approve_transaction(
        &self,
        chain_id: u64,
        token_address: &str,
        amount: Option<&str>,
    ) -> Result<OneInchTransaction, String> {
        Self::validate_chain(chain_id)?;
        let mut request = self
            .http
            .get(format!("{}/approve/transaction", self.chain_url(chain_id)))
            .bearer_auth(&self.api_key)
            .query(&[("tokenAddress", token_address)]);
        if let Some(amount) = amount {
            request = request.query(&[("amount", amount)]);
        }
        Self::send_json(request, "approve/transaction")
    }

    pub fn get_allowance(
        &self,
        chain_id: u64,
        token_address: &str,
        wallet_address: &str,
    ) -> Result<OneInchAllowanceResponse, String> {
        Self::validate_chain(chain_id)?;
        let request = self
            .http
            .get(format!("{}/approve/allowance", self.chain_url(chain_id)))
            .bearer_auth(&self.api_key)
            .query(&[
                ("tokenAddress", token_address),
                ("walletAddress", wallet_address),
            ]);
        Self::send_json(request, "approve/allowance")
    }

    pub fn get_liquidity_sources(
        &self,
        chain_id: u64,
    ) -> Result<OneInchLiquiditySourcesResponse, String> {
        Self::validate_chain(chain_id)?;
        let request = self
            .http
            .get(format!("{}/liquidity-sources", self.chain_url(chain_id)))
            .bearer_auth(&self.api_key);
        Self::send_json(request, "liquidity-sources")
    }

    pub fn get_tokens(&self, chain_id: u64) -> Result<OneInchTokensResponse, String> {
        Self::validate_chain(chain_id)?;
        let request = self
            .http
            .get(format!("{}/tokens", self.chain_url(chain_id)))
            .bearer_auth(&self.api_key);
        Self::send_json(request, "tokens")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const USDC: &str = "0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48";
    const WETH: &str = "0xc02aaa39b223fe8d0a0e5d3983027f52daa7aa3b";
    const DAI: &str = "0x6b175474e89094c44da98b954eedeac495271d0f";
    const CHAIN_ID: u64 = 1;
    const WALLET: &str = "0xd8dA6BF26964aF9D7eEd9e03E53415D37aA96045";

    fn client_or_skip() -> Option<OneInchClient> {
        std::env::var("ONEINCH_API_KEY")
            .ok()
            .map(|k| OneInchClient::new(k).expect("client should build"))
    }

    /// Swap 10k USDC for ETH at the best rate: quote -> allowance -> approve -> ready to swap.
    #[test]
    fn swap_usdc_to_eth_workflow() {
        let Some(c) = client_or_skip() else { return };

        let quote = c
            .get_quote(CHAIN_ID, USDC, WETH, "10000000000", None)
            .expect("quote should succeed");
        assert!(
            quote.dst_amount.as_deref().is_some_and(|s| !s.is_empty()),
            "dstAmount should not be empty"
        );

        let allowance = c
            .get_allowance(CHAIN_ID, USDC, WALLET)
            .expect("allowance check should succeed");
        assert!(
            allowance
                .allowance
                .as_deref()
                .is_some_and(|s| !s.is_empty()),
            "allowance must not be empty"
        );

        let approve_tx = c
            .get_approve_transaction(CHAIN_ID, USDC, None)
            .expect("approve transaction should succeed");
        assert!(
            approve_tx.data.is_some() || approve_tx.to.is_some(),
            "approve tx must contain transaction data"
        );
    }

    /// Discover tokens and liquidity sources, then quote a swap across multiple DEXs.
    #[test]
    fn check_liquidity_and_swap_workflow() {
        let Some(c) = client_or_skip() else { return };

        let tokens = c
            .get_tokens(CHAIN_ID)
            .expect("tokens endpoint should succeed");
        assert!(
            !tokens.tokens.is_empty(),
            "response must contain a tokens map"
        );

        let sources = c
            .get_liquidity_sources(CHAIN_ID)
            .expect("liquidity-sources should succeed");
        assert!(
            !sources.protocols.is_empty(),
            "response must contain protocols list"
        );

        let quote = c
            .get_quote(CHAIN_ID, DAI, WETH, "1000000000000000000000", None)
            .expect("quote should succeed");
        assert!(
            quote.dst_amount.as_deref().is_some_and(|s| !s.is_empty()),
            "dstAmount should not be empty"
        );
        assert!(
            quote.protocols.is_some(),
            "quote should contain routing protocols info"
        );
    }
}
