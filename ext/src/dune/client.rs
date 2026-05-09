use serde_json::Value;
use std::time::Duration;

pub const DEFAULT_DUNE_API: &str = "https://api.dune.com/api/v1";

pub struct DuneClient {
    pub http: reqwest::blocking::Client,
    pub api_endpoint: String,
    pub api_key: String,
}

impl DuneClient {
    pub fn new(api_key: &str) -> Result<Self, String> {
        let http = reqwest::blocking::Client::builder()
            .timeout(Duration::from_secs(30))
            .build()
            .map_err(|e| format!("[dune] build HTTP client failed: {e}"))?;
        Ok(Self {
            http,
            api_endpoint: std::env::var("DUNE_API_ENDPOINT")
                .unwrap_or_else(|_| DEFAULT_DUNE_API.to_string()),
            api_key: api_key.to_string(),
        })
    }

    fn get_json(&self, url: &str, op: &str) -> Result<Value, String> {
        let response = self
            .http
            .get(url)
            .header("X-DUNE-API-KEY", &self.api_key)
            .send()
            .map_err(|e| format!("[dune] {op} failed: {e}"))?;

        let status = response.status();
        let body = response.text().unwrap_or_default();
        if !status.is_success() {
            return Err(format!("[dune] {op} failed: {status} {body}"));
        }

        serde_json::from_str::<Value>(&body)
            .map_err(|e| format!("[dune] {op} decode failed: {e}; body: {body}"))
    }

    fn post_json(&self, url: &str, payload: &Value, op: &str) -> Result<Value, String> {
        let response = self
            .http
            .post(url)
            .header("X-DUNE-API-KEY", &self.api_key)
            .json(payload)
            .send()
            .map_err(|e| format!("[dune] {op} failed: {e}"))?;

        let status = response.status();
        let body = response.text().unwrap_or_default();
        if !status.is_success() {
            return Err(format!("[dune] {op} failed: {status} {body}"));
        }

        serde_json::from_str::<Value>(&body)
            .map_err(|e| format!("[dune] {op} decode failed: {e}; body: {body}"))
    }

    pub fn execute_query(
        &self,
        query_id: u64,
        query_parameters: Option<&Value>,
    ) -> Result<Value, String> {
        let url = format!("{}/query/{}/execute", self.api_endpoint, query_id);
        let payload = match query_parameters {
            Some(params) => serde_json::json!({ "query_parameters": params }),
            None => serde_json::json!({}),
        };
        self.post_json(&url, &payload, "execute_query")
    }

    pub fn get_execution_status(&self, execution_id: &str) -> Result<Value, String> {
        let url = format!("{}/execution/{}/status", self.api_endpoint, execution_id);
        self.get_json(&url, "get_execution_status")
    }

    pub fn get_execution_results(
        &self,
        execution_id: &str,
        limit: Option<u64>,
        offset: Option<u64>,
    ) -> Result<Value, String> {
        let mut url = format!("{}/execution/{}/results", self.api_endpoint, execution_id);
        let mut params = Vec::new();
        if let Some(l) = limit {
            params.push(format!("limit={l}"));
        }
        if let Some(o) = offset {
            params.push(format!("offset={o}"));
        }
        if !params.is_empty() {
            url.push('?');
            url.push_str(&params.join("&"));
        }
        self.get_json(&url, "get_execution_results")
    }

    pub fn get_query_results(
        &self,
        query_id: u64,
        limit: Option<u64>,
        offset: Option<u64>,
    ) -> Result<Value, String> {
        let mut url = format!("{}/query/{}/results", self.api_endpoint, query_id);
        let mut params = Vec::new();
        if let Some(l) = limit {
            params.push(format!("limit={l}"));
        }
        if let Some(o) = offset {
            params.push(format!("offset={o}"));
        }
        if !params.is_empty() {
            url.push('?');
            url.push_str(&params.join("&"));
        }
        self.get_json(&url, "get_query_results")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn api_key_or_skip() -> Option<String> {
        match std::env::var("DUNE_API_KEY") {
            Ok(key) if !key.is_empty() => Some(key),
            _ => None,
        }
    }

    /// Story: "Run a query to track on-chain data and act on findings"
    /// Execute a query → poll status → fetch results when completed.
    #[test]
    fn execute_and_poll_workflow() {
        let api_key = match api_key_or_skip() {
            Some(k) => k,
            None => return,
        };

        let client = DuneClient::new(&api_key).expect("failed to create DuneClient");
        let query_id: u64 = 1747157;

        let exec_response = client
            .execute_query(query_id, None)
            .expect("execute_query failed");
        let execution_id = exec_response["execution_id"]
            .as_str()
            .expect("response should contain execution_id");
        assert!(!execution_id.is_empty(), "execution_id should not be empty");

        let mut state = String::new();
        for _ in 1..=5 {
            std::thread::sleep(Duration::from_secs(2));
            let status_response = client
                .get_execution_status(execution_id)
                .expect("get_execution_status failed");
            state = status_response["state"]
                .as_str()
                .unwrap_or("UNKNOWN")
                .to_string();
            if state == "QUERY_STATE_COMPLETED" || state == "QUERY_STATE_FAILED" {
                break;
            }
        }
        assert!(
            [
                "QUERY_STATE_PENDING",
                "QUERY_STATE_EXECUTING",
                "QUERY_STATE_COMPLETED"
            ]
            .contains(&state.as_str()),
            "state should be a valid Dune execution state, got {state}"
        );

        if state == "QUERY_STATE_COMPLETED" {
            let results = client
                .get_execution_results(execution_id, Some(10), None)
                .expect("get_execution_results failed");
            let rows = results["result"]["rows"]
                .as_array()
                .expect("results should contain rows array");
            assert!(!rows.is_empty(), "completed query should return rows");
        } else {
            let cached = client
                .get_query_results(query_id, Some(5), None)
                .expect("get_query_results (cached) failed");
            let cached_rows = cached["result"]["rows"]
                .as_array()
                .expect("cached results should contain rows");
            assert!(!cached_rows.is_empty(), "cached results should have rows");
        }
    }

    /// Story: "Fetch cached analytics to inform a trading decision"
    #[test]
    fn cached_query_results_workflow() {
        let api_key = match api_key_or_skip() {
            Some(k) => k,
            None => return,
        };

        let client = DuneClient::new(&api_key).expect("failed to create DuneClient");
        let query_id: u64 = 1747157;

        let response = client
            .get_query_results(query_id, Some(10), None)
            .expect("get_query_results failed");

        let result = &response["result"];
        assert!(
            !result.is_null(),
            "response should contain a 'result' field"
        );

        let rows = result["rows"]
            .as_array()
            .expect("result should contain a rows array");
        assert!(!rows.is_empty(), "cached results should have rows");

        let metadata = &result["metadata"];
        assert!(!metadata.is_null(), "result should contain metadata");
    }
}
