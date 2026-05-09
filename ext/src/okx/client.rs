use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64};
use hmac::{Hmac, Mac};
use serde::Serialize;
use serde_json::Value;
use sha2::Sha256;
use std::time::Duration;

pub const BASE_URL: &str = "https://www.okx.com/api/v5";

type HmacSha256 = Hmac<Sha256>;

pub struct OkxClient {
    pub http: reqwest::blocking::Client,
}

impl OkxClient {
    pub fn new() -> Result<Self, String> {
        let http = reqwest::blocking::Client::builder()
            .timeout(Duration::from_secs(30))
            .build()
            .map_err(|e| format!("[okx] failed to build HTTP client: {e}"))?;
        Ok(Self { http })
    }

    /// Generate OKX API signature: HMAC-SHA256(timestamp + method + requestPath + body), base64-encoded.
    pub fn sign(
        secret_key: &str,
        timestamp: &str,
        method: &str,
        request_path: &str,
        body: &str,
    ) -> Result<String, String> {
        let prehash = format!("{timestamp}{method}{request_path}{body}");
        let mut mac = HmacSha256::new_from_slice(secret_key.as_bytes())
            .map_err(|e| format!("[okx] HMAC key error: {e}"))?;
        mac.update(prehash.as_bytes());
        let result = mac.finalize();
        Ok(BASE64.encode(result.into_bytes()))
    }

    pub fn iso_timestamp() -> String {
        // OKX expects ISO 8601 timestamp e.g. 2024-01-01T00:00:00.000Z
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default();
        let secs = now.as_secs();
        let millis = now.subsec_millis();
        let days_since_epoch = secs / 86400;
        let time_of_day = secs % 86400;
        let hours = time_of_day / 3600;
        let minutes = (time_of_day % 3600) / 60;
        let seconds = time_of_day % 60;

        let (year, month, day) = days_to_ymd(days_since_epoch);
        format!("{year:04}-{month:02}-{day:02}T{hours:02}:{minutes:02}:{seconds:02}.{millis:03}Z")
    }

    /// Public GET request (no auth).
    pub fn public_get(&self, path: &str) -> Result<Value, String> {
        let url = format!("{BASE_URL}{path}");
        let resp = self
            .http
            .get(&url)
            .send()
            .map_err(|e| format!("[okx] request failed: {e}"))?;
        Self::parse_response(resp)
    }

    /// Authenticated GET request.
    pub fn auth_get(
        &self,
        path: &str,
        api_key: &str,
        secret_key: &str,
        passphrase: &str,
    ) -> Result<Value, String> {
        let timestamp = Self::iso_timestamp();
        let sign = Self::sign(secret_key, &timestamp, "GET", path, "")?;
        let url = format!("{BASE_URL}{path}");
        let resp = self
            .http
            .get(&url)
            .header("OK-ACCESS-KEY", api_key)
            .header("OK-ACCESS-SIGN", sign)
            .header("OK-ACCESS-TIMESTAMP", &timestamp)
            .header("OK-ACCESS-PASSPHRASE", passphrase)
            .send()
            .map_err(|e| format!("[okx] request failed: {e}"))?;
        Self::parse_response(resp)
    }

    /// Authenticated POST request.
    pub fn auth_post<B: Serialize>(
        &self,
        path: &str,
        body: &B,
        api_key: &str,
        secret_key: &str,
        passphrase: &str,
    ) -> Result<Value, String> {
        let timestamp = Self::iso_timestamp();
        let body_str = serde_json::to_string(body)
            .map_err(|e| format!("[okx] failed to serialize body: {e}"))?;
        let sign = Self::sign(secret_key, &timestamp, "POST", path, &body_str)?;
        let url = format!("{BASE_URL}{path}");
        let resp = self
            .http
            .post(&url)
            .header("OK-ACCESS-KEY", api_key)
            .header("OK-ACCESS-SIGN", sign)
            .header("OK-ACCESS-TIMESTAMP", &timestamp)
            .header("OK-ACCESS-PASSPHRASE", passphrase)
            .header("Content-Type", "application/json")
            .body(body_str)
            .send()
            .map_err(|e| format!("[okx] request failed: {e}"))?;
        Self::parse_response(resp)
    }

    fn parse_response(resp: reqwest::blocking::Response) -> Result<Value, String> {
        let status = resp.status();
        let text = resp.text().unwrap_or_default();
        if !status.is_success() {
            return Err(format!("[okx] HTTP {status}: {text}"));
        }
        let parsed: Value = serde_json::from_str(&text)
            .map_err(|e| format!("[okx] failed to parse response: {e}"))?;
        // OKX returns { "code": "0", "msg": "", "data": [...] } on success
        let code = parsed.get("code").and_then(|c| c.as_str()).unwrap_or("");
        if code != "0" {
            let msg = parsed
                .get("msg")
                .and_then(|m| m.as_str())
                .unwrap_or("unknown error");
            return Err(format!("[okx] API error (code {code}): {msg}"));
        }
        Ok(parsed)
    }
}

/// Convert days since Unix epoch to (year, month, day).
fn days_to_ymd(days: u64) -> (u64, u64, u64) {
    // Algorithm from http://howardhinnant.github.io/date_algorithms.html
    let z = days + 719468;
    let era = z / 146097;
    let doe = z - era * 146097;
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;
    let y = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    let y = if m <= 2 { y + 1 } else { y };
    (y, m, d)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Story: "Sell spot ETH and go short on the SWAP"
    #[test]
    fn spot_to_swap_workflow() {
        let client = OkxClient::new().expect("failed to create OkxClient");

        let spot_resp = client
            .public_get("/market/tickers?instType=SPOT")
            .expect("failed to fetch SPOT tickers");
        let spot_data = spot_resp["data"]
            .as_array()
            .expect("SPOT tickers data should be an array");
        assert!(!spot_data.is_empty(), "SPOT tickers should not be empty");
        let btc_spot = spot_data
            .iter()
            .find(|t| t["instId"].as_str() == Some("BTC-USDT"))
            .expect("BTC-USDT should exist in SPOT tickers");
        assert!(
            btc_spot["last"].as_str().is_some(),
            "BTC-USDT spot ticker should have a last price"
        );

        let swap_resp = client
            .public_get("/market/tickers?instType=SWAP")
            .expect("failed to fetch SWAP tickers");
        let swap_data = swap_resp["data"]
            .as_array()
            .expect("SWAP tickers data should be an array");
        assert!(!swap_data.is_empty(), "SWAP tickers should not be empty");
        let btc_swap = swap_data
            .iter()
            .find(|t| t["instId"].as_str() == Some("BTC-USDT-SWAP"))
            .expect("BTC-USDT-SWAP should exist in SWAP tickers");
        assert!(
            btc_swap["last"].as_str().is_some(),
            "BTC-USDT-SWAP ticker should have a last price"
        );

        let book_resp = client
            .public_get("/market/books?instId=BTC-USDT")
            .expect("failed to fetch BTC-USDT order book");
        let book_data = book_resp["data"]
            .as_array()
            .expect("order book data should be an array");
        assert!(!book_data.is_empty(), "order book data should not be empty");
        let book = &book_data[0];
        let bids = book["bids"]
            .as_array()
            .expect("order book should have bids");
        let asks = book["asks"]
            .as_array()
            .expect("order book should have asks");
        assert!(!bids.is_empty(), "bids should not be empty");
        assert!(!asks.is_empty(), "asks should not be empty");

        let spot_price = btc_spot["last"].as_str().expect("spot last price missing");
        let swap_price = btc_swap["last"].as_str().expect("swap last price missing");
        let spot_f: f64 = spot_price.parse().expect("spot price should parse as f64");
        let swap_f: f64 = swap_price.parse().expect("swap price should parse as f64");
        assert!(spot_f > 0.0, "spot price should be positive");
        assert!(swap_f > 0.0, "swap price should be positive");
    }

    /// Story: "Tighten leverage on all my open positions to reduce liquidation risk"
    #[test]
    fn tighten_leverage_workflow() {
        let client = OkxClient::new().expect("failed to create OkxClient");

        let swap_resp = client
            .public_get("/market/tickers?instType=SWAP")
            .expect("failed to fetch SWAP tickers");
        let swap_data = swap_resp["data"]
            .as_array()
            .expect("SWAP tickers data should be an array");
        assert!(
            swap_data.len() > 1,
            "should have multiple SWAP instruments, got {}",
            swap_data.len()
        );

        let candle_resp = client
            .public_get("/market/candles?instId=BTC-USDT-SWAP")
            .expect("failed to fetch BTC-USDT-SWAP candles");
        let candle_data = candle_resp["data"]
            .as_array()
            .expect("candle data should be an array");
        assert!(
            !candle_data.is_empty(),
            "should have at least one candle for BTC-USDT-SWAP"
        );
        let first_candle = candle_data[0]
            .as_array()
            .expect("each candle should be an array");
        assert!(
            first_candle.len() >= 5,
            "candle should have at least 5 elements (ts, o, h, l, c)"
        );

        let book_resp = client
            .public_get("/market/books?instId=ETH-USDT-SWAP")
            .expect("failed to fetch ETH-USDT-SWAP order book");
        let book_data = book_resp["data"]
            .as_array()
            .expect("order book data should be an array");
        assert!(
            !book_data.is_empty(),
            "ETH-USDT-SWAP order book should not be empty"
        );
        let book = &book_data[0];
        let bids = book["bids"]
            .as_array()
            .expect("order book should have bids");
        let asks = book["asks"]
            .as_array()
            .expect("order book should have asks");
        assert!(!bids.is_empty(), "ETH-USDT-SWAP bids should not be empty");
        assert!(!asks.is_empty(), "ETH-USDT-SWAP asks should not be empty");
    }
}
