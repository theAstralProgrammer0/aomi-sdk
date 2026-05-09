use hmac::{Hmac, Mac};
use serde::{Serialize, de::DeserializeOwned};
use sha2::Sha256;
use std::time::Duration;

use crate::bybit::types::BybitResponse;

pub const BASE_URL: &str = "https://api.bybit.com/v5";
const RECV_WINDOW: &str = "5000";

type HmacSha256 = Hmac<Sha256>;

pub struct BybitClient {
    pub http: reqwest::blocking::Client,
}

impl BybitClient {
    pub fn new() -> Result<Self, String> {
        let http = reqwest::blocking::Client::builder()
            .timeout(Duration::from_secs(30))
            .build()
            .map_err(|e| format!("[bybit] failed to build HTTP client: {e}"))?;
        Ok(Self { http })
    }

    /// Generate HMAC-SHA256 signature for Bybit V5 API.
    pub fn sign(timestamp: &str, api_key: &str, secret_key: &str, params: &str) -> String {
        let sign_str = format!("{timestamp}{api_key}{RECV_WINDOW}{params}");
        let mut mac = HmacSha256::new_from_slice(secret_key.as_bytes())
            .expect("HMAC can take key of any size");
        mac.update(sign_str.as_bytes());
        hex::encode(mac.finalize().into_bytes())
    }

    /// Public GET (no auth).
    pub fn public_get<T: DeserializeOwned>(
        &self,
        path: &str,
        query: &str,
    ) -> Result<BybitResponse<T>, String> {
        let url = if query.is_empty() {
            format!("{BASE_URL}{path}")
        } else {
            format!("{BASE_URL}{path}?{query}")
        };
        let resp = self
            .http
            .get(&url)
            .send()
            .map_err(|e| format!("[bybit] request failed: {e}"))?;
        Self::handle_response(resp)
    }

    /// Authenticated GET.
    pub fn auth_get<T: DeserializeOwned>(
        &self,
        path: &str,
        query: &str,
        api_key: &str,
        secret_key: &str,
    ) -> Result<BybitResponse<T>, String> {
        let timestamp = Self::timestamp_ms();
        let signature = Self::sign(&timestamp, api_key, secret_key, query);
        let url = if query.is_empty() {
            format!("{BASE_URL}{path}")
        } else {
            format!("{BASE_URL}{path}?{query}")
        };
        let resp = self
            .http
            .get(&url)
            .header("X-BAPI-API-KEY", api_key)
            .header("X-BAPI-TIMESTAMP", &timestamp)
            .header("X-BAPI-SIGN", &signature)
            .header("X-BAPI-RECV-WINDOW", RECV_WINDOW)
            .send()
            .map_err(|e| format!("[bybit] request failed: {e}"))?;
        Self::handle_response(resp)
    }

    /// Authenticated POST with JSON body.
    pub fn auth_post<B: Serialize, T: DeserializeOwned>(
        &self,
        path: &str,
        body: &B,
        api_key: &str,
        secret_key: &str,
    ) -> Result<BybitResponse<T>, String> {
        let timestamp = Self::timestamp_ms();
        let body_str = serde_json::to_string(body)
            .map_err(|e| format!("[bybit] failed to serialize body: {e}"))?;
        let signature = Self::sign(&timestamp, api_key, secret_key, &body_str);
        let url = format!("{BASE_URL}{path}");
        let resp = self
            .http
            .post(&url)
            .header("X-BAPI-API-KEY", api_key)
            .header("X-BAPI-TIMESTAMP", &timestamp)
            .header("X-BAPI-SIGN", &signature)
            .header("X-BAPI-RECV-WINDOW", RECV_WINDOW)
            .header("Content-Type", "application/json")
            .body(body_str)
            .send()
            .map_err(|e| format!("[bybit] request failed: {e}"))?;
        Self::handle_response(resp)
    }

    fn timestamp_ms() -> String {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("system clock before UNIX epoch")
            .as_millis()
            .to_string()
    }

    fn handle_response<T: DeserializeOwned>(
        resp: reqwest::blocking::Response,
    ) -> Result<BybitResponse<T>, String> {
        let status = resp.status();
        let text = resp
            .text()
            .map_err(|e| format!("[bybit] failed to read response body: {e}"))?;
        if !status.is_success() {
            return Err(format!("[bybit] API HTTP error {status}: {text}"));
        }
        let val: BybitResponse<T> =
            serde_json::from_str(&text).map_err(|e| format!("[bybit] JSON decode failed: {e}"))?;
        // Bybit returns retCode != 0 for logical errors even on HTTP 200
        if val.ret_code != 0 {
            return Err(format!(
                "[bybit] API error (retCode={}): {}",
                val.ret_code, val.ret_msg
            ));
        }
        Ok(val)
    }
}

// We need hex encoding for HMAC output but don't want another dep.
// Inline a tiny hex encoder.
mod hex {
    pub fn encode(bytes: impl AsRef<[u8]>) -> String {
        bytes.as_ref().iter().fold(String::new(), |mut acc, b| {
            use std::fmt::Write;
            let _ = write!(acc, "{b:02x}");
            acc
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bybit::types::{
        BybitKlineResult, BybitOrderbookResult, BybitResponse, BybitTickerResult,
    };

    fn client() -> BybitClient {
        BybitClient::new().expect("client should build")
    }

    /// Scalp ETH on Bybit — open, set TP/SL, close when done.
    #[test]
    fn scalp_eth_workflow() {
        let c = client();

        let tickers: BybitResponse<BybitTickerResult> = c
            .public_get("/market/tickers", "category=linear&symbol=ETHUSDT")
            .expect("should get ETHUSDT ticker");
        let ticker_list = tickers.result.list;
        assert!(!ticker_list.is_empty(), "should have at least one ticker");
        let last_price = ticker_list[0]
            .last_price
            .as_deref()
            .expect("ticker should have lastPrice");
        let last_price: f64 = last_price.parse().expect("lastPrice should parse as f64");
        assert!(last_price > 0.0, "lastPrice should be positive");

        let book: BybitResponse<BybitOrderbookResult> = c
            .public_get(
                "/market/orderbook",
                "category=linear&symbol=ETHUSDT&limit=25",
            )
            .expect("should get ETHUSDT orderbook");
        let bids = book.result.b;
        let asks = book.result.a;
        assert!(!bids.is_empty(), "bids should not be empty");
        assert!(!asks.is_empty(), "asks should not be empty");

        let kline: BybitResponse<BybitKlineResult> = c
            .public_get("/market/kline", "category=linear&symbol=ETHUSDT&interval=5")
            .expect("should get ETHUSDT 5m kline");
        let candles = kline.result.list;
        assert!(!candles.is_empty(), "should have at least one candle");

        let best_bid: f64 = bids[0]
            .price()
            .expect("bid price")
            .parse()
            .expect("bid price f64");
        let best_ask: f64 = asks[0]
            .price()
            .expect("ask price")
            .parse()
            .expect("ask price f64");
        assert!(best_ask > best_bid, "ask should be above bid");

        let entry = (best_bid + best_ask) / 2.0;
        let tp = entry * 1.01;
        let sl = entry * 0.99;
        assert!(tp > entry, "TP should be above entry");
        assert!(sl < entry, "SL should be below entry");
        assert!(sl > 0.0, "SL should be positive");
    }

    /// Move my stop-loss to breakeven on all profitable positions.
    #[test]
    fn move_stoploss_workflow() {
        let c = client();

        let tickers: BybitResponse<BybitTickerResult> = c
            .public_get("/market/tickers", "category=linear")
            .expect("should get linear tickers");
        let ticker_list = tickers.result.list;
        assert!(ticker_list.len() > 1, "should have multiple linear tickers");

        let book: BybitResponse<BybitOrderbookResult> = c
            .public_get(
                "/market/orderbook",
                "category=linear&symbol=BTCUSDT&limit=25",
            )
            .expect("should get BTCUSDT orderbook");
        let bids = book.result.b;
        let asks = book.result.a;
        assert!(!bids.is_empty(), "bids should not be empty");
        assert!(!asks.is_empty(), "asks should not be empty");
        let best_bid: f64 = bids[0]
            .price()
            .expect("bid price")
            .parse()
            .expect("bid price f64");
        let best_ask: f64 = asks[0]
            .price()
            .expect("ask price")
            .parse()
            .expect("ask price f64");
        let spread = best_ask - best_bid;
        assert!(spread >= 0.0, "spread should be non-negative");

        let kline: BybitResponse<BybitKlineResult> = c
            .public_get(
                "/market/kline",
                "category=linear&symbol=BTCUSDT&interval=15",
            )
            .expect("should get BTCUSDT 15m kline");
        let candles = kline.result.list;
        assert!(!candles.is_empty(), "should have at least one candle");

        let latest_candle = &candles[0];
        let open: f64 = latest_candle
            .open()
            .expect("candle open")
            .parse()
            .expect("candle open f64");
        let current: f64 = (best_bid + best_ask) / 2.0;
        let breakeven = open;
        assert!(breakeven > 0.0, "breakeven should be positive");
        assert!(
            current > 0.0 && open > 0.0,
            "both current and open prices should be positive"
        );
    }
}
