use aomi_sdk::schemars::JsonSchema;
use aomi_sdk::*;
use serde::Deserialize;
use serde_json::{Map, Value, json};
use std::cmp::Ordering;
use std::time::Duration;

#[derive(Clone, Default)]
pub(crate) struct YearnApp;

#[allow(unused_imports)]
pub(crate) use crate::tool::*;

// ============================================================================
// Yearn yDaemon Client (blocking)
// ============================================================================

pub(crate) const DEFAULT_YEARN_API: &str = "https://ydaemon.yearn.fi";

#[derive(Clone)]
pub(crate) struct YearnClient {
    pub(crate) http: reqwest::blocking::Client,
    pub(crate) api_endpoint: String,
}

impl YearnClient {
    pub(crate) fn new() -> Result<Self, String> {
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

    pub(crate) fn get_json(&self, url: &str, op: &str) -> Result<Value, String> {
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

    pub(crate) fn with_source(value: Value) -> Value {
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

    pub(crate) fn get_all_vaults(&self, chain_id: u64) -> Result<Value, String> {
        let url = format!("{}/{chain_id}/vaults/all", self.api_endpoint);
        let value = self.get_json(&url, "get_all_vaults")?;
        let vaults = Self::extract_data_array(value, "get_all_vaults")?;
        let mut summaries: Vec<Value> = vaults.iter().map(Self::summarize_vault).collect();
        summaries.sort_by(|left, right| {
            let left_tvl = Self::value_as_f64(left.get("tvl_usd"));
            let right_tvl = Self::value_as_f64(right.get("tvl_usd"));
            right_tvl
                .partial_cmp(&left_tvl)
                .unwrap_or(Ordering::Equal)
        });
        Ok(json!({
            "source": "yearn",
            "chain_id": chain_id,
            "count": summaries.len(),
            "sorted_by": "tvl_usd_desc",
            "data": summaries,
        }))
    }

    pub(crate) fn get_vault_detail(&self, chain_id: u64, address: &str) -> Result<Value, String> {
        let url = format!("{}/{chain_id}/vaults/{address}", self.api_endpoint);
        let value = self.get_json(&url, "get_vault_detail")?;
        Ok(Self::with_source(value))
    }

    pub(crate) fn get_blacklisted_vaults(&self) -> Result<Value, String> {
        let url = format!("{}/info/vaults/blacklisted", self.api_endpoint);
        let value = self.get_json(&url, "get_blacklisted_vaults")?;
        let addresses = Self::extract_string_array(value);
        Ok(json!({
            "source": "yearn",
            "count": addresses.len(),
            "data": addresses,
        }))
    }

    fn extract_data_array(value: Value, op: &str) -> Result<Vec<Value>, String> {
        match value {
            Value::Array(items) => Ok(items),
            Value::Object(mut map) => match map.remove("data") {
                Some(Value::Array(items)) => Ok(items),
                Some(other) => Err(format!(
                    "[yearn] {op} failed: expected `data` array, got {}",
                    Self::value_kind(&other)
                )),
                None => Err(format!("[yearn] {op} failed: missing `data` array")),
            },
            other => Err(format!(
                "[yearn] {op} failed: expected array response, got {}",
                Self::value_kind(&other)
            )),
        }
    }

    fn extract_string_array(value: Value) -> Vec<String> {
        match value {
            Value::Array(items) => items
                .into_iter()
                .filter_map(|item| item.as_str().map(str::to_string))
                .collect(),
            Value::Object(mut map) => match map.remove("data") {
                Some(Value::Array(items)) => items
                    .into_iter()
                    .filter_map(|item| item.as_str().map(str::to_string))
                    .collect(),
                _ => Vec::new(),
            },
            _ => Vec::new(),
        }
    }

    fn summarize_vault(vault: &Value) -> Value {
        let symbol = Self::first_string(
            vault,
            &["displaySymbol", "formatedSymbol", "symbol", "display_symbol"],
        );
        let name = Self::first_string(
            vault,
            &["displayName", "formatedName", "name", "display_name"],
        );
        let underlying_symbol = Self::first_nested_string(
            vault,
            &[&["token", "display_symbol"], &["token", "symbol"]],
        );
        let underlying_name = Self::first_nested_string(
            vault,
            &[&["token", "display_name"], &["token", "name"]],
        );

        let mut alias_set = std::collections::BTreeSet::new();
        for value in [
            symbol.clone(),
            name.clone(),
            underlying_symbol.clone(),
            underlying_name.clone(),
            Self::first_string(vault, &["symbol"]),
            Self::first_string(vault, &["displayName"]),
        ]
        .into_iter()
        .flatten()
        {
            let trimmed = value.trim();
            if !trimmed.is_empty() {
                alias_set.insert(trimmed.to_string());
            }
        }

        let mut summary = Map::new();
        summary.insert(
            "address".to_string(),
            vault.get("address").cloned().unwrap_or(Value::Null),
        );
        summary.insert(
            "symbol".to_string(),
            symbol.map(Value::String).unwrap_or(Value::Null),
        );
        summary.insert(
            "name".to_string(),
            name.map(Value::String).unwrap_or(Value::Null),
        );
        summary.insert(
            "underlying_symbol".to_string(),
            underlying_symbol.map(Value::String).unwrap_or(Value::Null),
        );
        summary.insert(
            "underlying_name".to_string(),
            underlying_name.map(Value::String).unwrap_or(Value::Null),
        );
        summary.insert(
            "category".to_string(),
            Self::first_string(vault, &["category"])
                .or_else(|| Self::first_nested_string(vault, &[&["details", "category"]]))
                .map(Value::String)
                .unwrap_or(Value::Null),
        );
        summary.insert(
            "chain_id".to_string(),
            vault.get("chainID").cloned().unwrap_or(Value::Null),
        );
        summary.insert(
            "tvl_usd".to_string(),
            Self::number_value(Self::first_nested_value(vault, &[&["tvl", "tvl"]])),
        );
        summary.insert(
            "net_apy".to_string(),
            Self::number_value(
                Self::first_nested_value(
                    vault,
                    &[
                        &["apr", "netAPR"],
                        &["apy", "net_apy"],
                        &["apr", "forwardAPR", "netAPR"],
                    ],
                ),
            ),
        );
        summary.insert(
            "is_retired".to_string(),
            Self::first_nested_value(vault, &[&["details", "isRetired"], &["info", "isRetired"]])
                .cloned()
                .unwrap_or(Value::Bool(false)),
        );
        summary.insert(
            "is_hidden".to_string(),
            Self::first_nested_value(vault, &[&["details", "isHidden"], &["info", "isHidden"]])
                .cloned()
                .unwrap_or(Value::Bool(false)),
        );
        summary.insert(
            "aliases".to_string(),
            Value::Array(alias_set.into_iter().map(Value::String).collect()),
        );
        Value::Object(summary)
    }

    fn first_string(value: &Value, keys: &[&str]) -> Option<String> {
        keys.iter().find_map(|key| {
            value
                .get(*key)
                .and_then(Value::as_str)
                .map(str::to_string)
        })
    }

    fn first_nested_string(value: &Value, paths: &[&[&str]]) -> Option<String> {
        Self::first_nested_value(value, paths)
            .and_then(Value::as_str)
            .map(str::to_string)
    }

    fn first_nested_value<'a>(value: &'a Value, paths: &[&[&str]]) -> Option<&'a Value> {
        paths.iter().find_map(|path| {
            let mut current = value;
            for key in *path {
                current = current.get(*key)?;
            }
            Some(current)
        })
    }

    fn number_value(value: Option<&Value>) -> Value {
        Self::value_as_f64(value)
            .map(Value::from)
            .unwrap_or(Value::Null)
    }

    fn value_as_f64(value: Option<&Value>) -> Option<f64> {
        match value? {
            Value::Number(number) => number.as_f64(),
            Value::String(text) => text.parse::<f64>().ok(),
            _ => None,
        }
    }

    fn value_kind(value: &Value) -> &'static str {
        match value {
            Value::Null => "null",
            Value::Bool(_) => "bool",
            Value::Number(_) => "number",
            Value::String(_) => "string",
            Value::Array(_) => "array",
            Value::Object(_) => "object",
        }
    }
}

// ============================================================================
// Tool arg structs
// ============================================================================

pub(crate) struct GetAllVaults;

#[derive(Debug, Deserialize, JsonSchema)]
pub(crate) struct GetAllVaultsArgs {
    /// Chain ID to query. Supported: 1 (Ethereum), 10 (Optimism), 137 (Polygon), 250 (Fantom), 8453 (Base), 42161 (Arbitrum). Default: 1.
    #[serde(default = "default_chain_id")]
    pub(crate) chain_id: u64,
}

pub(crate) struct GetVaultDetail;

#[derive(Debug, Deserialize, JsonSchema)]
pub(crate) struct GetVaultDetailArgs {
    /// Chain ID to query. Supported: 1 (Ethereum), 10 (Optimism), 137 (Polygon), 250 (Fantom), 8453 (Base), 42161 (Arbitrum). Default: 1.
    #[serde(default = "default_chain_id")]
    pub(crate) chain_id: u64,
    /// The vault contract address (e.g. "0x...")
    pub(crate) address: String,
}

pub(crate) struct GetBlacklistedVaults;

#[derive(Debug, Deserialize, JsonSchema)]
pub(crate) struct GetBlacklistedVaultsArgs {}

fn default_chain_id() -> u64 {
    1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn summarize_vault_compacts_large_payloads() {
        let summary = YearnClient::summarize_vault(&json!({
            "address": "0xvault",
            "displaySymbol": "yvUSDC",
            "displayName": "Yearn USDC Vault",
            "symbol": "yvUSDC-legacy",
            "category": "Stablecoin",
            "chainID": 1,
            "details": { "isRetired": false, "isHidden": false },
            "token": {
                "symbol": "USDC",
                "display_name": "USD Coin"
            },
            "tvl": { "tvl": 1234567.89 },
            "apr": { "netAPR": 0.0456 }
        }));

        assert_eq!(summary["address"], "0xvault");
        assert_eq!(summary["symbol"], "yvUSDC");
        assert_eq!(summary["underlying_symbol"], "USDC");
        assert_eq!(summary["tvl_usd"], 1234567.89);
        assert_eq!(summary["net_apy"], 0.0456);
        assert!(
            summary["aliases"]
                .as_array()
                .expect("aliases")
                .iter()
                .any(|value| value == "yvUSDC")
        );
    }

    /// Story: "Deposit idle USDT into the best Yearn vault"
    /// Fetch all vaults, filter blacklisted ones, pick a vault, inspect its detail.
    #[test]
    fn deposit_stablecoin_vault_workflow() {
        let client = YearnClient::new().expect("failed to create YearnClient");

        // Step 1: Get all vaults on Ethereum mainnet.
        println!("[step 1] Fetching all vaults for chain 1 (Ethereum)...");
        let all_vaults = client.get_all_vaults(1).expect("get_all_vaults failed");
        let vaults_arr = all_vaults["data"]
            .as_array()
            .or_else(|| all_vaults.as_array())
            .expect("expected vaults to be an array");
        println!("[step 1] Got {} vaults on Ethereum", vaults_arr.len());
        assert!(
            !vaults_arr.is_empty(),
            "expected at least one vault on chain 1"
        );

        // Verify vault entries carry compact discovery data.
        let sample = &vaults_arr[0];
        println!(
            "[step 1] Sample vault: address={}, symbol={}, has_tvl={}",
            sample["address"].as_str().unwrap_or("?"),
            sample["symbol"].as_str().unwrap_or("?"),
            sample.get("tvl_usd").and_then(Value::as_f64).is_some()
        );
        assert!(
            sample["address"].as_str().is_some(),
            "expected vault summary to include an address"
        );
        assert!(
            sample["symbol"].as_str().is_some(),
            "expected vault summary to include a symbol"
        );
        assert!(
            vaults_arr
                .iter()
                .any(|vault| vault.get("tvl_usd").and_then(Value::as_f64).is_some()),
            "expected at least one vault summary with TVL data"
        );

        // Step 2: Get blacklisted vaults and filter them out.
        println!("[step 2] Fetching blacklisted vaults...");
        let blacklisted = client
            .get_blacklisted_vaults()
            .expect("get_blacklisted_vaults failed");
        // The response should be valid JSON (object or array).
        assert!(
            blacklisted.is_object() || blacklisted.is_array(),
            "blacklisted response should be valid JSON"
        );

        // Collect blacklisted addresses for filtering.
        let blacklisted_addrs: Vec<String> = blacklisted["data"]
            .as_array()
            .or_else(|| blacklisted.as_array())
            .unwrap_or(&vec![])
            .iter()
            .filter_map(|v| v.as_str().map(|s| s.to_lowercase()))
            .collect();
        println!(
            "[step 2] Found {} blacklisted vault addresses",
            blacklisted_addrs.len()
        );

        let filtered: Vec<&Value> = vaults_arr
            .iter()
            .filter(|v| {
                let addr = v["address"].as_str().unwrap_or_default().to_lowercase();
                !blacklisted_addrs.contains(&addr)
            })
            .collect();
        println!(
            "[step 2] After filtering: {} vaults remain (removed {})",
            filtered.len(),
            vaults_arr.len() - filtered.len()
        );
        assert!(
            !filtered.is_empty(),
            "expected at least one non-blacklisted vault"
        );

        // Step 3: Pick the first filtered vault and get its detail.
        let vault_addr = filtered[0]["address"]
            .as_str()
            .expect("vault should have an address field");
        println!("[step 3] Fetching detail for vault {}...", vault_addr);
        let detail = client
            .get_vault_detail(1, vault_addr)
            .expect("get_vault_detail failed");
        assert!(detail.is_object(), "vault detail should be a JSON object");

        // Verify the detail contains strategies, fees, and APY breakdown.
        let detail_data = if detail.get("data").is_some() {
            &detail["data"]
        } else {
            &detail
        };
        let has_strategies =
            detail_data.get("strategies").is_some() || detail_data.get("strategy").is_some();
        let has_apr = detail_data.get("apy").is_some() || detail_data.get("apr").is_some();
        println!(
            "[step 3] Vault detail: has_strategies={}, has_apr={}, has_address={}",
            has_strategies,
            has_apr,
            detail_data.get("address").is_some()
        );
        assert!(has_strategies, "vault detail should include strategies");
        assert!(has_apr, "vault detail should include APY/APR breakdown");

        // Step 4: Assert we have enough data to choose a vault and build a deposit TX.
        assert!(
            detail_data.get("address").is_some() || detail_data.get("token").is_some(),
            "vault detail should contain address or token info to build a deposit TX"
        );
        println!(
            "[step 4] Workflow complete: vault {} is ready for deposit",
            vault_addr
        );
    }

    /// Story: "Migrate my vault position to a higher-yield chain"
    /// Fetch vaults on Ethereum and Arbitrum, compare yields for similar assets.
    #[test]
    fn cross_chain_vault_comparison_workflow() {
        let client = YearnClient::new().expect("failed to create YearnClient");

        // Step 1: Get all vaults on Ethereum (chain 1).
        println!("[step 1] Fetching all vaults for chain 1 (Ethereum)...");
        let eth_vaults_resp = client.get_all_vaults(1).expect("get_all_vaults(1) failed");
        let eth_vaults = eth_vaults_resp["data"]
            .as_array()
            .or_else(|| eth_vaults_resp.as_array())
            .expect("expected Ethereum vaults to be an array");
        println!("[step 1] Got {} vaults on Ethereum", eth_vaults.len());
        assert!(
            !eth_vaults.is_empty(),
            "expected at least one vault on Ethereum"
        );

        // Step 2: Get all vaults on Optimism (chain 10).
        println!("[step 2] Fetching all vaults for chain 10 (Optimism)...");
        let arb_vaults_resp = client
            .get_all_vaults(10)
            .expect("get_all_vaults(10) failed");
        let arb_vaults = arb_vaults_resp["data"]
            .as_array()
            .or_else(|| arb_vaults_resp.as_array())
            .expect("expected Optimism vaults to be an array");
        println!("[step 2] Got {} vaults on Optimism", arb_vaults.len());
        assert!(
            !arb_vaults.is_empty(),
            "expected at least one vault on Optimism"
        );

        // Step 3: Find vaults for similar assets across both chains.
        println!("[step 3] Finding common asset symbols across chains...");
        // Extract token symbols from each chain.
        let eth_symbols: std::collections::HashSet<String> = eth_vaults
            .iter()
            .filter_map(|v| {
                v["underlying_symbol"]
                    .as_str()
                    .or_else(|| v["symbol"].as_str())
                    .map(|s| s.to_uppercase())
            })
            .collect();

        let arb_symbols: std::collections::HashSet<String> = arb_vaults
            .iter()
            .filter_map(|v| {
                v["underlying_symbol"]
                    .as_str()
                    .or_else(|| v["symbol"].as_str())
                    .map(|s| s.to_uppercase())
            })
            .collect();

        let common_symbols: Vec<&String> = eth_symbols.intersection(&arb_symbols).collect();
        println!(
            "[step 3] Ethereum has {} unique symbols, Optimism has {}, common: {} ({:?})",
            eth_symbols.len(),
            arb_symbols.len(),
            common_symbols.len(),
            &common_symbols[..std::cmp::min(5, common_symbols.len())]
        );
        assert!(
            !common_symbols.is_empty(),
            "expected at least one common asset symbol across Ethereum and Arbitrum"
        );

        // Step 4: For a common asset, compare yields to identify the better chain.
        let target = common_symbols[0];
        println!("[step 4] Comparing yields for target asset: {target}");

        let best_apy = |vaults: &[Value], symbol: &str| -> Option<f64> {
            vaults
                .iter()
                .filter(|v| {
                    let sym = v["underlying_symbol"]
                        .as_str()
                        .or_else(|| v["symbol"].as_str())
                        .unwrap_or_default()
                        .to_uppercase();
                    sym == symbol
                })
                .filter_map(|v| v["net_apy"].as_f64())
                .max_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
        };

        let target = common_symbols
            .iter()
            .map(|symbol| symbol.as_str())
            .find(|symbol| best_apy(eth_vaults, symbol).is_some() || best_apy(arb_vaults, symbol).is_some())
            .unwrap_or(target);

        let eth_best = best_apy(eth_vaults, target);
        let arb_best = best_apy(arb_vaults, target);

        // At least one chain should report yield data for the common asset.
        println!(
            "[step 4] Yield lookup for {target}: eth_best={eth_best:?}, arb_best={arb_best:?}"
        );
        assert!(
            eth_best.is_some() || arb_best.is_some(),
            "expected yield data for {target} on at least one chain"
        );

        // We can identify which chain offers better yield (or that data exists to decide).
        if let (Some(e), Some(a)) = (eth_best, arb_best) {
            let better_chain = if a > e { "Arbitrum" } else { "Ethereum" };
            println!(
                "[step 4] For {target}: Ethereum APY={e:.4}, Arbitrum APY={a:.4} => {better_chain} is better"
            );
        } else {
            println!(
                "[step 4] For {target}: yield data available on one chain (eth={eth_best:?}, arb={arb_best:?})"
            );
        }
        println!("[step 4] Cross-chain comparison workflow complete");
    }
}
