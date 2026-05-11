use serde_json::{Value, json};
use std::time::Duration;

// ============================================================================
// Yearn yDaemon Client (blocking)
// ============================================================================

pub const DEFAULT_YEARN_API: &str = "https://ydaemon.yearn.fi";

#[derive(Clone)]
pub struct YearnClient {
    pub http: reqwest::blocking::Client,
    pub api_endpoint: String,
}

impl YearnClient {
    pub fn new() -> Result<Self, String> {
        let http = reqwest::blocking::Client::builder()
            .timeout(Duration::from_secs(30))
            .build()
            .map_err(|e| format!("[yearn] failed to build HTTP client: {e}"))?;
        Ok(Self {
            http,
            api_endpoint: std::env::var("YEARN_API_ENDPOINT")
                .unwrap_or_else(|_| DEFAULT_YEARN_API.to_string()),
        })
    }

    pub fn get_json(&self, url: &str, op: &str) -> Result<Value, String> {
        let response = self
            .http
            .get(url)
            .send()
            .map_err(|e| format!("[yearn] {op} failed: {e}"))?;

        let status = response.status();
        let body = response.text().unwrap_or_default();
        if !status.is_success() {
            return Err(format!("[yearn] {op} failed: {status} {body}"));
        }

        serde_json::from_str::<Value>(&body)
            .map_err(|e| format!("[yearn] {op} failed: decode error: {e}"))
    }

    pub fn with_source(value: Value) -> Value {
        match value {
            Value::Object(mut map) => {
                map.insert("source".to_string(), Value::String("yearn".to_string()));
                Value::Object(map)
            }
            other => json!({
                "source": "yearn",
                "data": other,
            }),
        }
    }

    pub fn get_all_vaults(&self, chain_id: u64) -> Result<Value, String> {
        let url = format!("{}/{chain_id}/vaults/all", self.api_endpoint);
        let value = self.get_json(&url, "get_all_vaults")?;
        Ok(Self::with_source(value))
    }

    pub fn get_vault_detail(&self, chain_id: u64, address: &str) -> Result<Value, String> {
        let url = format!("{}/{chain_id}/vaults/{address}", self.api_endpoint);
        let value = self.get_json(&url, "get_vault_detail")?;
        Ok(Self::with_source(value))
    }

    pub fn get_blacklisted_vaults(&self) -> Result<Value, String> {
        let url = format!("{}/info/vaults/blacklisted", self.api_endpoint);
        let value = self.get_json(&url, "get_blacklisted_vaults")?;
        Ok(Self::with_source(value))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deposit_stablecoin_vault_workflow() {
        let client = YearnClient::new().expect("failed to create YearnClient");

        let all_vaults = client.get_all_vaults(1).expect("get_all_vaults failed");
        let vaults_arr = all_vaults["data"]
            .as_array()
            .or_else(|| all_vaults.as_array())
            .expect("expected vaults to be an array");
        assert!(!vaults_arr.is_empty());

        let sample = &vaults_arr[0];
        assert!(sample.get("apy").is_some() || sample.get("apr").is_some());
        assert!(sample.get("tvl").is_some());

        let blacklisted = client
            .get_blacklisted_vaults()
            .expect("get_blacklisted_vaults failed");
        assert!(blacklisted.is_object() || blacklisted.is_array());

        let empty: Vec<Value> = vec![];
        let blacklisted_addrs: Vec<String> = blacklisted["data"]
            .as_array()
            .or_else(|| blacklisted.as_array())
            .unwrap_or(&empty)
            .iter()
            .filter_map(|v| v.as_str().map(|s| s.to_lowercase()))
            .collect();

        let filtered: Vec<&Value> = vaults_arr
            .iter()
            .filter(|v| {
                let addr = v["address"].as_str().unwrap_or_default().to_lowercase();
                !blacklisted_addrs.contains(&addr)
            })
            .collect();
        assert!(!filtered.is_empty());

        let vault_addr = filtered[0]["address"]
            .as_str()
            .expect("vault should have an address field");
        let detail = client
            .get_vault_detail(1, vault_addr)
            .expect("get_vault_detail failed");
        assert!(detail.is_object());

        let detail_data = if detail.get("data").is_some() {
            &detail["data"]
        } else {
            &detail
        };
        let has_strategies =
            detail_data.get("strategies").is_some() || detail_data.get("strategy").is_some();
        let has_apr = detail_data.get("apy").is_some() || detail_data.get("apr").is_some();
        assert!(has_strategies);
        assert!(has_apr);
        assert!(detail_data.get("address").is_some() || detail_data.get("token").is_some());
    }

    #[test]
    fn cross_chain_vault_comparison_workflow() {
        let client = YearnClient::new().expect("failed to create YearnClient");

        let eth_vaults_resp = client.get_all_vaults(1).expect("get_all_vaults(1) failed");
        let eth_vaults = eth_vaults_resp["data"]
            .as_array()
            .or_else(|| eth_vaults_resp.as_array())
            .expect("expected Ethereum vaults to be an array");
        assert!(!eth_vaults.is_empty());

        let arb_vaults_resp = client
            .get_all_vaults(10)
            .expect("get_all_vaults(10) failed");
        let arb_vaults = arb_vaults_resp["data"]
            .as_array()
            .or_else(|| arb_vaults_resp.as_array())
            .expect("expected Optimism vaults to be an array");
        assert!(!arb_vaults.is_empty());

        let eth_symbols: std::collections::HashSet<String> = eth_vaults
            .iter()
            .filter_map(|v| {
                v["token"]["symbol"]
                    .as_str()
                    .or_else(|| v["symbol"].as_str())
                    .map(|s| s.to_uppercase())
            })
            .collect();

        let arb_symbols: std::collections::HashSet<String> = arb_vaults
            .iter()
            .filter_map(|v| {
                v["token"]["symbol"]
                    .as_str()
                    .or_else(|| v["symbol"].as_str())
                    .map(|s| s.to_uppercase())
            })
            .collect();

        let common_symbols: Vec<&String> = eth_symbols.intersection(&arb_symbols).collect();
        assert!(!common_symbols.is_empty());

        let target = common_symbols[0];

        let best_apy = |vaults: &[Value], symbol: &str| -> Option<f64> {
            vaults
                .iter()
                .filter(|v| {
                    let sym = v["token"]["symbol"]
                        .as_str()
                        .or_else(|| v["symbol"].as_str())
                        .unwrap_or_default()
                        .to_uppercase();
                    sym == symbol
                })
                .filter_map(|v| {
                    v["apy"]["net_apy"]
                        .as_f64()
                        .or_else(|| v["apr"]["netAPR"].as_f64())
                        .or_else(|| v["apr"]["net_apy"].as_f64())
                        .or_else(|| v["apy"]["points"]["week_ago"].as_f64())
                })
                .max_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
        };

        let eth_best = best_apy(eth_vaults, target);
        let arb_best = best_apy(arb_vaults, target);

        assert!(eth_best.is_some() || arb_best.is_some());
    }
}
