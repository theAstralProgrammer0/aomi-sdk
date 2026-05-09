use serde::{Deserialize, Serialize};
use std::time::Duration;

pub const API_BASE: &str = "https://api.twitterapi.io";

#[derive(Clone)]
pub struct XClient {
    pub http: reqwest::blocking::Client,
    pub api_key: String,
}

impl XClient {
    pub fn new(api_key: String) -> Result<Self, String> {
        let http = reqwest::blocking::Client::builder()
            .timeout(Duration::from_secs(30))
            .build()
            .map_err(|e| format!("failed to build HTTP client: {e}"))?;
        Ok(Self { http, api_key })
    }

    pub fn get<T, Q>(&self, path: &str, query: &Q) -> Result<T, String>
    where
        T: serde::de::DeserializeOwned,
        Q: Serialize,
    {
        let url = format!("{API_BASE}{path}");
        let resp = self
            .http
            .get(&url)
            .header("X-API-Key", &self.api_key)
            .query(query)
            .send()
            .map_err(|e| format!("X API request failed: {e}"))?;

        if !resp.status().is_success() {
            let status = resp.status();
            let text = resp.text().unwrap_or_default();
            return Err(format!("X API error {status}: {text}"));
        }

        let text = resp
            .text()
            .map_err(|e| format!("X API body read failed: {e}"))?;

        if let Ok(api_resp) = serde_json::from_str::<ApiResponse<T>>(&text) {
            if api_resp.is_success() {
                if let Some(data) = api_resp.data {
                    return Ok(data);
                }
            } else if api_resp.data.is_some()
                || api_resp.status.is_some()
                || api_resp.msg.is_some()
                || api_resp.message.is_some()
                || api_resp.code.is_some()
                || api_resp.success.is_some()
            {
                return Err(format!("X API logical error: {}", api_resp.error_message()));
            }
        }

        serde_json::from_str::<T>(&text).map_err(|e| format!("X API decode failed: {e}"))
    }
}

#[derive(Debug, Deserialize)]
pub struct ApiResponse<T> {
    #[serde(default)]
    pub status: Option<String>,
    #[serde(default)]
    pub msg: Option<String>,
    #[serde(default)]
    pub message: Option<String>,
    #[serde(default)]
    pub code: Option<i64>,
    #[serde(default)]
    pub success: Option<bool>,
    pub data: Option<T>,
}

impl<T> ApiResponse<T> {
    pub fn is_success(&self) -> bool {
        if let Some(success) = self.success {
            return success;
        }
        if let Some(ref status) = self.status
            && (status.eq_ignore_ascii_case("success") || status.eq_ignore_ascii_case("ok"))
        {
            return true;
        }
        if let Some(code) = self.code {
            return code == 0 || code == 200;
        }
        false
    }

    pub fn error_message(&self) -> String {
        self.msg
            .clone()
            .or_else(|| self.message.clone())
            .unwrap_or_else(|| "Unknown API error".to_string())
    }
}
