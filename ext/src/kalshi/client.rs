use serde_json::Value;
use std::time::Duration;
#[cfg(test)]
use std::time::{SystemTime, UNIX_EPOCH};

use crate::kalshi::types::{ImportKalshiMarketRequest, SimmerRegisterRequest, SimmerTradeRequest};

pub const SIMMER_API_URL: &str = "https://api.simmer.markets";

#[derive(Clone)]
pub struct SimmerClient {
    pub http: reqwest::blocking::Client,
    pub api_key: String,
}

impl SimmerClient {
    pub fn new(api_key: &str) -> Result<Self, String> {
        let http = reqwest::blocking::Client::builder()
            .timeout(Duration::from_secs(30))
            .build()
            .map_err(|e| format!("failed to build HTTP client: {e}"))?;
        Ok(Self {
            http,
            api_key: api_key.to_string(),
        })
    }

    pub fn auth_header(&self) -> String {
        format!("Bearer {}", self.api_key)
    }

    pub fn send_json(
        request: reqwest::blocking::RequestBuilder,
        operation: &str,
    ) -> Result<Value, String> {
        let response = request
            .send()
            .map_err(|e| format!("[simmer] {operation} request failed: {e}"))?;
        let status = response.status();
        let body = response.text().unwrap_or_default();
        if !status.is_success() {
            return Err(format!("[simmer] {operation} failed: {status} {body}"));
        }
        serde_json::from_str::<Value>(&body)
            .map_err(|e| format!("[simmer] {operation} decode failed: {e}; body: {body}"))
    }

    #[cfg(test)]
    pub fn health_check() -> Result<Value, String> {
        let http = reqwest::blocking::Client::builder()
            .timeout(Duration::from_secs(30))
            .build()
            .map_err(|e| format!("failed to build HTTP client: {e}"))?;
        let req = http.get(format!("{}/api/sdk/health", SIMMER_API_URL));
        Self::send_json(req, "health")
    }

    pub fn get_agent_status(&self) -> Result<Value, String> {
        let url = format!("{SIMMER_API_URL}/api/sdk/agents/me");
        let req = self
            .http
            .get(&url)
            .header("Authorization", self.auth_header());
        Self::send_json(req, "get_agent_status")
    }

    pub fn get_briefing(&self, since: Option<&str>) -> Result<Value, String> {
        let url = format!("{SIMMER_API_URL}/api/sdk/briefing");
        let mut req = self
            .http
            .get(&url)
            .header("Authorization", self.auth_header());
        if let Some(since) = since {
            req = req.query(&[("since", since)]);
        }
        Self::send_json(req, "get_briefing")
    }

    pub fn get_market_context(&self, market_id: &str) -> Result<Value, String> {
        let url = format!("{SIMMER_API_URL}/api/sdk/context/{market_id}");
        let req = self
            .http
            .get(&url)
            .header("Authorization", self.auth_header());
        Self::send_json(req, "get_market_context")
    }

    pub fn trade(&self, body: &SimmerTradeRequest) -> Result<Value, String> {
        let url = format!("{SIMMER_API_URL}/api/sdk/trade");
        let req = self
            .http
            .post(&url)
            .header("Authorization", self.auth_header())
            .json(&body);
        Self::send_json(req, "trade")
    }

    pub fn get_positions(&self, venue: Option<&str>) -> Result<Value, String> {
        let url = format!("{SIMMER_API_URL}/api/sdk/positions");
        let mut req = self
            .http
            .get(&url)
            .header("Authorization", self.auth_header());
        if let Some(venue) = venue {
            req = req.query(&[("venue", venue)]);
        }
        Self::send_json(req, "get_positions")
    }

    pub fn get_portfolio(&self) -> Result<Value, String> {
        let url = format!("{SIMMER_API_URL}/api/sdk/portfolio");
        let req = self
            .http
            .get(&url)
            .header("Authorization", self.auth_header());
        Self::send_json(req, "get_portfolio")
    }

    pub fn list_importable_kalshi_markets(
        &self,
        query: Option<&str>,
        limit: Option<u32>,
        min_volume: Option<f64>,
    ) -> Result<Value, String> {
        let url = format!("{SIMMER_API_URL}/api/sdk/markets/importable");
        let mut req = self
            .http
            .get(&url)
            .header("Authorization", self.auth_header())
            .query(&[("venue", "kalshi")]);
        if let Some(query) = query {
            req = req.query(&[("q", query)]);
        }
        if let Some(limit) = limit {
            req = req.query(&[("limit", limit.to_string())]);
        }
        if let Some(min_volume) = min_volume {
            req = req.query(&[("min_volume", min_volume.to_string())]);
        }
        Self::send_json(req, "list_importable_kalshi_markets")
    }

    pub fn import_kalshi_market(&self, kalshi_url: &str) -> Result<Value, String> {
        let url = format!("{SIMMER_API_URL}/api/sdk/markets/import/kalshi");
        let body = ImportKalshiMarketRequest {
            kalshi_url: kalshi_url.to_string(),
        };
        let req = self
            .http
            .post(&url)
            .header("Authorization", self.auth_header())
            .json(&body);
        Self::send_json(req, "import_kalshi_market")
    }
}

pub fn simmer_register_agent(name: &str, description: Option<&str>) -> Result<Value, String> {
    let http = reqwest::blocking::Client::builder()
        .timeout(Duration::from_secs(30))
        .build()
        .map_err(|e| format!("failed to build HTTP client: {e}"))?;

    let body = SimmerRegisterRequest {
        name: name.to_string(),
        description: description.map(str::to_string),
    };

    let req = http
        .post(format!("{SIMMER_API_URL}/api/sdk/agents/register"))
        .json(&body);
    SimmerClient::send_json(req, "register_agent")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn unique_name(prefix: &str) -> String {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("clock should be after epoch")
            .as_millis();
        format!("{prefix}-{now}")
    }

    fn register_temp_agent() -> Value {
        simmer_register_agent(
            &unique_name("codex-kalshi-smoke"),
            Some("temporary smoke test"),
        )
        .expect("agent registration should succeed")
    }

    fn temp_api_key() -> String {
        register_temp_agent()["api_key"]
            .as_str()
            .expect("register response should include api_key")
            .to_string()
    }

    #[test]
    fn simmer_health_smoke() {
        let payload = SimmerClient::health_check().expect("health endpoint should respond");
        assert_eq!(payload.get("status").and_then(Value::as_str), Some("ok"));
    }

    #[test]
    fn register_and_status_smoke() {
        let registered = register_temp_agent();
        let api_key = registered["api_key"]
            .as_str()
            .expect("register response should include api_key");
        let client = SimmerClient::new(api_key).expect("client should build");
        let status = client
            .get_agent_status()
            .expect("agent status request should succeed");

        assert_eq!(
            status.get("status").and_then(Value::as_str),
            Some("unclaimed")
        );
    }

    #[test]
    fn list_importable_kalshi_markets_smoke() {
        let client = SimmerClient::new(&temp_api_key()).expect("client should build");
        let payload = client
            .list_importable_kalshi_markets(None, Some(1), None)
            .expect("importable markets request should succeed");
        let markets = payload["markets"]
            .as_array()
            .expect("markets should be an array");

        assert!(
            !markets.is_empty(),
            "expected at least one importable Kalshi market"
        );
        assert_eq!(
            markets[0].get("venue").and_then(Value::as_str),
            Some("kalshi")
        );
        assert!(
            markets[0].get("url").and_then(Value::as_str).is_some(),
            "expected importable market to include a Kalshi URL"
        );
    }

    #[test]
    fn import_kalshi_market_requires_claimed_agent_smoke() {
        let client = SimmerClient::new(&temp_api_key()).expect("client should build");
        let importables = client
            .list_importable_kalshi_markets(None, Some(1), None)
            .expect("importable markets request should succeed");
        let kalshi_url = importables["markets"][0]["url"]
            .as_str()
            .expect("importable market should include url");

        let err = client
            .import_kalshi_market(kalshi_url)
            .expect_err("fresh agents should not be able to import before claiming");

        assert!(
            err.contains("claimed agent") || err.contains("Claim your agent first"),
            "unexpected import error: {err}"
        );
    }
}
