use serde_json::{Value, json};
use std::time::Duration;

// ============================================================================
// Morpho GraphQL Client (blocking)
// ============================================================================

pub const DEFAULT_MORPHO_API: &str = "https://blue-api.morpho.org/graphql";

#[derive(Clone)]
pub struct MorphoClient {
    pub http: reqwest::blocking::Client,
    pub api_endpoint: String,
}

impl MorphoClient {
    pub fn new() -> Result<Self, String> {
        let http = reqwest::blocking::Client::builder()
            .timeout(Duration::from_secs(30))
            .build()
            .map_err(|e| format!("[morpho] failed to build HTTP client: {e}"))?;
        Ok(Self {
            http,
            api_endpoint: std::env::var("MORPHO_API_ENDPOINT")
                .unwrap_or_else(|_| DEFAULT_MORPHO_API.to_string()),
        })
    }

    pub fn post_graphql(
        &self,
        query: &str,
        variables: Option<Value>,
        op: &str,
    ) -> Result<Value, String> {
        let body = match variables {
            Some(vars) => json!({ "query": query, "variables": vars }),
            None => json!({ "query": query }),
        };

        let response = self
            .http
            .post(&self.api_endpoint)
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .map_err(|e| format!("[morpho] {op} failed: {e}"))?;

        let status = response.status();
        let text = response.text().unwrap_or_default();
        if !status.is_success() {
            return Err(format!("[morpho] {op} failed: {status} {text}"));
        }

        let parsed: Value = serde_json::from_str(&text)
            .map_err(|e| format!("[morpho] {op} failed: decode error: {e}; body: {text}"))?;

        if let Some(errors) = parsed.get("errors") {
            return Err(format!("[morpho] {op} failed: GraphQL errors: {errors}"));
        }

        Ok(parsed.get("data").cloned().unwrap_or(parsed))
    }

    pub fn get_markets(&self) -> Result<Value, String> {
        let query = r#"
            query {
                markets(first: 100) {
                    items {
                        uniqueKey
                        lltv
                        collateralAsset {
                            symbol
                            address
                            decimals
                        }
                        loanAsset {
                            symbol
                            address
                            decimals
                        }
                        state {
                            supplyApy
                            borrowApy
                            supplyAssetsUsd
                            borrowAssetsUsd
                            liquidityAssetsUsd
                            utilization
                        }
                    }
                }
            }
        "#;
        let data = self.post_graphql(query, None, "get_markets")?;
        Ok(json!({
            "source": "morpho",
            "markets": data.get("markets").cloned().unwrap_or(Value::Null),
        }))
    }

    pub fn get_vaults(&self) -> Result<Value, String> {
        let query = r#"
            query {
                vaults(first: 100) {
                    items {
                        address
                        name
                        symbol
                        asset {
                            symbol
                            address
                        }
                        state {
                            apy
                            netApy
                            totalAssetsUsd
                            allocation {
                                market {
                                    uniqueKey
                                    collateralAsset {
                                        symbol
                                    }
                                    loanAsset {
                                        symbol
                                    }
                                }
                            }
                        }
                    }
                }
            }
        "#;
        let data = self.post_graphql(query, None, "get_vaults")?;
        Ok(json!({
            "source": "morpho",
            "vaults": data.get("vaults").cloned().unwrap_or(Value::Null),
        }))
    }

    pub fn get_user_positions(&self, address: &str) -> Result<Value, String> {
        let query = r#"
            query GetUserPositions($address: String!) {
                userByAddress(address: $address) {
                    address
                    marketPositions {
                        market {
                            uniqueKey
                            collateralAsset {
                                symbol
                            }
                            loanAsset {
                                symbol
                            }
                        }
                        supplyAssetsUsd
                        borrowAssetsUsd
                        collateralUsd
                    }
                    vaultPositions {
                        vault {
                            address
                            name
                            symbol
                        }
                        assetsUsd
                    }
                }
            }
        "#;
        let variables = json!({ "address": address });
        let data = self.post_graphql(query, Some(variables), "get_user_positions")?;
        Ok(json!({
            "source": "morpho",
            "user": data.get("userByAddress").cloned().unwrap_or(Value::Null),
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deposit_best_vault_workflow() {
        let client = MorphoClient::new().expect("failed to create MorphoClient");

        let vaults_response = client.get_vaults().expect("get_vaults failed");
        let vault_items = vaults_response["vaults"]["items"]
            .as_array()
            .expect("vaults response should contain items array");
        assert!(!vault_items.is_empty());

        let first_vault = &vault_items[0];
        assert!(
            first_vault["state"]["apy"].is_number() || first_vault["state"]["netApy"].is_number()
        );
        assert!(
            first_vault["state"]["totalAssetsUsd"].is_number()
                || first_vault["state"]["totalAssetsUsd"].is_string()
        );

        let markets_response = client.get_markets().expect("get_markets failed");
        let market_items = markets_response["markets"]["items"]
            .as_array()
            .expect("markets response should contain items array");
        assert!(!market_items.is_empty());

        let first_market = &market_items[0];
        assert!(first_market["state"]["supplyApy"].is_number());
        assert!(first_market["state"]["borrowApy"].is_number());

        let best_vault_apy = vault_items
            .iter()
            .filter_map(|v| {
                v["state"]["netApy"]
                    .as_f64()
                    .or_else(|| v["state"]["apy"].as_f64())
            })
            .fold(f64::NEG_INFINITY, f64::max);

        let best_market_supply_apy = market_items
            .iter()
            .filter_map(|m| m["state"]["supplyApy"].as_f64())
            .fold(f64::NEG_INFINITY, f64::max);

        assert!(best_vault_apy.is_finite());
        assert!(best_market_supply_apy.is_finite());
    }

    #[test]
    fn borrow_against_collateral_workflow() {
        let client = MorphoClient::new().expect("failed to create MorphoClient");

        let markets_response = client.get_markets().expect("get_markets failed");
        let market_items = markets_response["markets"]["items"]
            .as_array()
            .expect("markets response should contain items array");
        assert!(!market_items.is_empty());

        let markets_with_ltv: Vec<&Value> = market_items
            .iter()
            .filter(|m| m["lltv"].is_number() || m["lltv"].is_string())
            .collect();
        assert!(!markets_with_ltv.is_empty());

        let first = &markets_with_ltv[0];
        assert!(first["state"]["borrowApy"].is_number());

        let zero_addr = "0x0000000000000000000000000000000000000000";
        let positions_response = client
            .get_user_positions(zero_addr)
            .expect("get_user_positions failed");
        assert_eq!(
            positions_response["source"]
                .as_str()
                .expect("should have source"),
            "morpho"
        );
        let user = &positions_response["user"];
        if !user.is_null() {
            assert!(user.get("marketPositions").is_some() || user.get("vaultPositions").is_some());
        }

        let cheapest_borrow = market_items
            .iter()
            .filter_map(|m| {
                let apy = m["state"]["borrowApy"].as_f64()?;
                let key = m["uniqueKey"].as_str()?;
                Some((key, apy))
            })
            .filter(|(_, apy)| *apy > 0.0)
            .min_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

        let (cheapest_key, cheapest_apy) =
            cheapest_borrow.expect("should find at least one market with a positive borrow APY");
        assert!(cheapest_apy > 0.0);

        let cheapest_market = market_items
            .iter()
            .find(|m| m["uniqueKey"].as_str() == Some(cheapest_key))
            .expect("should find the cheapest market in items");

        assert!(cheapest_market["lltv"].is_number() || cheapest_market["lltv"].is_string());
        assert!(cheapest_market["state"]["borrowApy"].is_number());
        assert!(cheapest_market.get("collateralAsset").is_some());
    }
}
