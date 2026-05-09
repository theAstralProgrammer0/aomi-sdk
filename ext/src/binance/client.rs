use hmac::{Hmac, Mac};
use serde::de::DeserializeOwned;
use sha2::Sha256;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

// ============================================================================
// HMAC-SHA256 Signing
// ============================================================================

type HmacSha256 = Hmac<Sha256>;

/// Compute HMAC-SHA256 signature over query string, returned as a hex string.
pub fn sign(secret_key: &str, query_string: &str) -> Result<String, String> {
    let mut mac = HmacSha256::new_from_slice(secret_key.as_bytes())
        .map_err(|e| format!("[binance] failed to create HMAC key: {e}"))?;
    mac.update(query_string.as_bytes());
    let result = mac.finalize();
    Ok(hex_encode(&result.into_bytes()))
}

fn hex_encode(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}

pub fn current_timestamp_ms() -> Result<u64, String> {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_millis() as u64)
        .map_err(|e| format!("[binance] failed to get timestamp: {e}"))
}

// ============================================================================
// HTTP Client
// ============================================================================

pub const SPOT_BASE_URL: &str = "https://api.binance.com/api/v3";
pub const FUTURES_BASE_URL: &str = "https://fapi.binance.com/fapi/v1";

pub struct BinanceClient {
    pub http: reqwest::blocking::Client,
}

impl BinanceClient {
    pub fn new() -> Result<Self, String> {
        let http = reqwest::blocking::Client::builder()
            .timeout(Duration::from_secs(30))
            .build()
            .map_err(|e| format!("[binance] failed to build HTTP client: {e}"))?;
        Ok(Self { http })
    }

    /// Public GET request (no auth required).
    pub fn public_get<T: DeserializeOwned>(
        &self,
        base_url: &str,
        path: &str,
        query: &str,
    ) -> Result<T, String> {
        let url = if query.is_empty() {
            format!("{base_url}{path}")
        } else {
            format!("{base_url}{path}?{query}")
        };
        let resp = self
            .http
            .get(&url)
            .send()
            .map_err(|e| format!("[binance] request failed: {e}"))?;
        Self::parse_response(resp, "public_get")
    }

    /// Signed GET request (HMAC-SHA256 auth).
    pub fn signed_get<T: DeserializeOwned>(
        &self,
        base_url: &str,
        path: &str,
        api_key: &str,
        secret_key: &str,
        query: &str,
    ) -> Result<T, String> {
        let full_query = Self::build_signed_query(secret_key, query)?;
        let url = format!("{base_url}{path}?{full_query}");
        let resp = self
            .http
            .get(&url)
            .header("X-MBX-APIKEY", api_key)
            .send()
            .map_err(|e| format!("[binance] signed_get failed: {e}"))?;
        Self::parse_response(resp, "signed_get")
    }

    /// Signed POST request (HMAC-SHA256 auth).
    pub fn signed_post<T: DeserializeOwned>(
        &self,
        base_url: &str,
        path: &str,
        api_key: &str,
        secret_key: &str,
        query: &str,
    ) -> Result<T, String> {
        let full_query = Self::build_signed_query(secret_key, query)?;
        let url = format!("{base_url}{path}?{full_query}");
        let resp = self
            .http
            .post(&url)
            .header("X-MBX-APIKEY", api_key)
            .send()
            .map_err(|e| format!("[binance] signed_post failed: {e}"))?;
        Self::parse_response(resp, "signed_post")
    }

    /// Signed DELETE request (HMAC-SHA256 auth).
    pub fn signed_delete<T: DeserializeOwned>(
        &self,
        base_url: &str,
        path: &str,
        api_key: &str,
        secret_key: &str,
        query: &str,
    ) -> Result<T, String> {
        let full_query = Self::build_signed_query(secret_key, query)?;
        let url = format!("{base_url}{path}?{full_query}");
        let resp = self
            .http
            .delete(&url)
            .header("X-MBX-APIKEY", api_key)
            .send()
            .map_err(|e| format!("[binance] signed_delete failed: {e}"))?;
        Self::parse_response(resp, "signed_delete")
    }

    fn build_signed_query(secret_key: &str, query: &str) -> Result<String, String> {
        let timestamp = current_timestamp_ms()?;
        let query_with_ts = if query.is_empty() {
            format!("timestamp={timestamp}")
        } else {
            format!("{query}&timestamp={timestamp}")
        };
        let signature = sign(secret_key, &query_with_ts)?;
        Ok(format!("{query_with_ts}&signature={signature}"))
    }

    fn parse_response<T: DeserializeOwned>(
        resp: reqwest::blocking::Response,
        op: &str,
    ) -> Result<T, String> {
        let status = resp.status();
        let text = resp.text().unwrap_or_default();
        if !status.is_success() {
            return Err(format!("[binance] {op} failed: HTTP {status}: {text}"));
        }
        serde_json::from_str(&text)
            .map_err(|e| format!("[binance] {op} failed: could not parse response: {e}"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::binance::types::{
        Binance24hrStatsResponse, BinanceDepthResponse, BinanceKlineResponse, BinancePriceResponse,
    };

    /// Story: "Buy 2 ETH on Binance with a limit order below market"
    #[test]
    fn buy_eth_workflow() {
        let client = BinanceClient::new().expect("failed to create BinanceClient");

        let price_resp: BinancePriceResponse = client
            .public_get(SPOT_BASE_URL, "/ticker/price", "symbol=ETHUSDT")
            .expect("GET /ticker/price for ETHUSDT failed");
        let price_str = price_resp
            .first()
            .map(|t| t.price.as_str())
            .expect("price field missing");
        let price: f64 = price_str.parse().expect("could not parse price as f64");
        assert!(price > 0.0);

        let depth: BinanceDepthResponse = client
            .public_get(SPOT_BASE_URL, "/depth", "symbol=ETHUSDT&limit=5")
            .expect("GET /depth failed");
        assert!(!depth.bids.is_empty() && !depth.asks.is_empty());

        let best_bid: f64 = depth.bids[0].price().unwrap().parse().unwrap();
        let best_ask: f64 = depth.asks[0].price().unwrap().parse().unwrap();
        let mid = (best_bid + best_ask) / 2.0;
        let limit_price = mid * 0.99;
        assert!(limit_price > 0.0);

        let query = format!(
            "symbol=ETHUSDT&side=BUY&type=LIMIT&timeInForce=GTC&quantity=2&price={limit_price:.2}"
        );
        let sig = sign("test_secret_key", &query).expect("sign failed");
        assert_eq!(sig.len(), 64);
    }

    /// Story: "Set up a 10x BTC short with stop-loss on futures"
    #[test]
    fn btc_short_with_stoploss_workflow() {
        let client = BinanceClient::new().expect("failed to create BinanceClient");

        let price_resp: BinancePriceResponse = client
            .public_get(SPOT_BASE_URL, "/ticker/price", "symbol=BTCUSDT")
            .expect("GET /ticker/price failed");
        let btc_price: f64 = price_resp
            .first()
            .unwrap()
            .price
            .parse()
            .expect("could not parse BTC price");
        assert!(btc_price > 0.0);

        let klines: BinanceKlineResponse = client
            .public_get(
                SPOT_BASE_URL,
                "/klines",
                "symbol=BTCUSDT&interval=1h&limit=24",
            )
            .expect("GET /klines failed");
        assert!(!klines.is_empty());

        let stats: Binance24hrStatsResponse = client
            .public_get(SPOT_BASE_URL, "/ticker/24hr", "symbol=BTCUSDT")
            .expect("GET /ticker/24hr failed");
        let stats = stats.first().expect("ticker stats missing");
        let volume: f64 = stats.volume.as_deref().unwrap().parse().unwrap();
        assert!(volume > 0.0);

        let query = format!(
            "symbol=BTCUSDT&side=SELL&type=LIMIT&timeInForce=GTC&quantity=0.1&price={btc_price:.2}"
        );
        let sig = sign("test_secret_key", &query).expect("sign failed");
        assert_eq!(sig.len(), 64);
    }
}
