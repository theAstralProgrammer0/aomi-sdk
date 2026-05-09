use serde_json::{Value, json};
use std::time::Duration;

pub const DEFAULT_API_URL: &str = "https://api.hyperliquid.xyz";

#[derive(Clone)]
pub struct HyperliquidClient {
    pub http: reqwest::blocking::Client,
    pub api_url: String,
}

impl HyperliquidClient {
    pub fn new() -> Result<Self, String> {
        let http = reqwest::blocking::Client::builder()
            .timeout(Duration::from_secs(30))
            .build()
            .map_err(|e| format!("[hyperliquid] failed to build HTTP client: {e}"))?;
        Ok(Self {
            http,
            api_url: std::env::var("HYPERLIQUID_API_URL")
                .unwrap_or_else(|_| DEFAULT_API_URL.to_string()),
        })
    }

    pub fn post_info(&self, body: Value) -> Result<Value, String> {
        let url = format!("{}/info", self.api_url);
        let response = self
            .http
            .post(&url)
            .json(&body)
            .send()
            .map_err(|e| format!("[hyperliquid] request failed: {e}"))?;

        let status = response.status();
        let text = response.text().unwrap_or_default();
        if !status.is_success() {
            return Err(format!("[hyperliquid] request failed ({status}): {text}"));
        }

        serde_json::from_str::<Value>(&text)
            .map_err(|e| format!("[hyperliquid] decode failed: {e}; body: {text}"))
    }

    pub fn with_source(value: Value) -> Value {
        match value {
            Value::Object(mut map) => {
                map.insert(
                    "source".to_string(),
                    Value::String("hyperliquid".to_string()),
                );
                Value::Object(map)
            }
            other => json!({
                "source": "hyperliquid",
                "data": other,
            }),
        }
    }

    pub fn get_meta(&self) -> Result<Value, String> {
        let body = json!({"type": "meta"});
        let value = self.post_info(body)?;
        Ok(Self::with_source(value))
    }

    pub fn get_all_mids(&self) -> Result<Value, String> {
        let body = json!({"type": "allMids"});
        let value = self.post_info(body)?;
        Ok(Self::with_source(value))
    }

    pub fn get_l2_book(&self, coin: &str) -> Result<Value, String> {
        let body = json!({"type": "l2Book", "coin": coin});
        let value = self.post_info(body)?;
        Ok(Self::with_source(value))
    }

    pub fn get_clearinghouse_state(&self, user: &str) -> Result<Value, String> {
        let body = json!({"type": "clearinghouseState", "user": user});
        let value = self.post_info(body)?;
        Ok(Self::with_source(value))
    }

    pub fn get_open_orders(&self, user: &str) -> Result<Value, String> {
        let body = json!({"type": "openOrders", "user": user});
        let value = self.post_info(body)?;
        Ok(Self::with_source(value))
    }

    pub fn get_user_fills(&self, user: &str) -> Result<Value, String> {
        let body = json!({"type": "userFills", "user": user});
        let value = self.post_info(body)?;
        Ok(Self::with_source(value))
    }

    pub fn get_funding_history(
        &self,
        coin: &str,
        start_time: u64,
        end_time: Option<u64>,
    ) -> Result<Value, String> {
        let mut body = json!({
            "type": "fundingHistory",
            "coin": coin,
            "startTime": start_time,
        });
        if let Some(et) = end_time {
            body.as_object_mut()
                .unwrap()
                .insert("endTime".to_string(), json!(et));
        }
        let value = self.post_info(body)?;
        Ok(Self::with_source(value))
    }

    pub fn get_candle_snapshot(
        &self,
        coin: &str,
        interval: &str,
        start_time: u64,
        end_time: u64,
    ) -> Result<Value, String> {
        let body = json!({
            "type": "candleSnapshot",
            "req": {
                "coin": coin,
                "interval": interval,
                "startTime": start_time,
                "endTime": end_time,
            }
        });
        let value = self.post_info(body)?;
        Ok(Self::with_source(value))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn client() -> HyperliquidClient {
        HyperliquidClient::new().expect("client should build")
    }

    /// Story: "Open a 5x long on ETH-PERP" — read-only steps before placement.
    #[test]
    fn open_leveraged_long_workflow() {
        let c = client();
        let meta = c.get_meta().expect("get_meta should succeed");
        assert_eq!(meta.get("source").and_then(Value::as_str), Some("hyperliquid"));
        let universe = meta
            .get("universe")
            .and_then(Value::as_array)
            .expect("meta should contain a 'universe' array");
        assert!(!universe.is_empty());

        let _eth_asset = universe
            .iter()
            .find(|a| a.get("name").and_then(Value::as_str) == Some("ETH"))
            .expect("ETH should be listed in the universe");

        let mids = c.get_all_mids().expect("get_all_mids should succeed");
        let eth_mid: f64 = mids
            .get("ETH")
            .and_then(Value::as_str)
            .expect("ETH mid")
            .parse()
            .expect("parse mid");
        assert!(eth_mid > 0.0);

        let book = c.get_l2_book("ETH").expect("get_l2_book(ETH) should succeed");
        let levels = book.get("levels").and_then(Value::as_array).expect("levels");
        assert!(levels.len() >= 2);
        assert!(!levels[0].as_array().unwrap().is_empty());
        assert!(!levels[1].as_array().unwrap().is_empty());
    }

    /// Story: "Close my losing positions" — query clearinghouse + open orders + fills.
    #[test]
    fn close_positions_workflow() {
        let c = client();
        let zero_addr = "0x0000000000000000000000000000000000000000";

        let ch = c
            .get_clearinghouse_state(zero_addr)
            .expect("get_clearinghouse_state should succeed");
        assert!(ch.get("marginSummary").is_some());
        assert!(ch.get("assetPositions").is_some());

        let orders = c
            .get_open_orders(zero_addr)
            .expect("get_open_orders should succeed");
        let orders_data = orders.get("data").and_then(Value::as_array).expect("data");
        assert!(orders_data.is_empty());

        let fills = c
            .get_user_fills(zero_addr)
            .expect("get_user_fills should succeed");
        assert!(fills.get("data").and_then(Value::as_array).is_some());
    }
}
