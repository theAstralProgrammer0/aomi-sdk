use serde::Serialize;
use serde_json::Value;
use std::time::Duration;

pub const API_BASE: &str = "https://api.neynar.com/v2";

#[derive(Clone)]
pub struct NeynarClient {
    pub http: reqwest::blocking::Client,
    pub api_key: String,
}

impl NeynarClient {
    pub fn new(api_key: String) -> Result<Self, String> {
        let http = reqwest::blocking::Client::builder()
            .timeout(Duration::from_secs(30))
            .build()
            .map_err(|e| format!("[neynar] failed to build HTTP client: {e}"))?;
        Ok(Self { http, api_key })
    }

    pub fn get<Q: Serialize>(&self, path: &str, query: &Q, op: &str) -> Result<Value, String> {
        let url = format!("{API_BASE}{path}");
        let resp = self
            .http
            .get(&url)
            .header("x-api-key", &self.api_key)
            .query(query)
            .send()
            .map_err(|e| format!("[neynar] {op} failed: {e}"))?;

        let status = resp.status();
        let text = resp.text().unwrap_or_default();
        if !status.is_success() {
            return Err(format!("[neynar] {op} failed: {status} {text}"));
        }

        serde_json::from_str::<Value>(&text)
            .map_err(|e| format!("[neynar] {op} decode failed: {e}"))
    }

    pub fn post_json<B: Serialize>(&self, path: &str, body: &B, op: &str) -> Result<Value, String> {
        let url = format!("{API_BASE}{path}");
        let resp = self
            .http
            .post(&url)
            .header("x-api-key", &self.api_key)
            .json(body)
            .send()
            .map_err(|e| format!("[neynar] {op} failed: {e}"))?;

        let status = resp.status();
        let text = resp.text().unwrap_or_default();
        if !status.is_success() {
            return Err(format!("[neynar] {op} failed: {status} {text}"));
        }

        serde_json::from_str::<Value>(&text)
            .map_err(|e| format!("[neynar] {op} decode failed: {e}"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::neynar::types::UserByUsernameQuery;

    #[derive(Serialize)]
    struct BulkUsersQuery<'a> {
        fids: &'a str,
    }

    fn client_or_skip() -> Option<NeynarClient> {
        std::env::var("NEYNAR_API_KEY")
            .ok()
            .map(|key| NeynarClient::new(key).expect("failed to build NeynarClient"))
    }

    #[test]
    fn post_thread_workflow() {
        let Some(client) = client_or_skip() else {
            return;
        };

        let user_resp = client
            .get(
                "/farcaster/user/by_username",
                &UserByUsernameQuery {
                    username: "vitalik.eth",
                },
                "get_user_by_username",
            )
            .expect("get_user_by_username should succeed");
        let user = &user_resp["user"];
        assert!(user.get("fid").is_some(), "user should have an fid");

        let bulk_resp = client
            .get(
                "/farcaster/user/bulk",
                &BulkUsersQuery { fids: "3,5650,2" },
                "bulk_users",
            )
            .expect("bulk_users should succeed");
        let users = bulk_resp["users"]
            .as_array()
            .expect("bulk response should contain users array");
        assert!(
            users.len() >= 2,
            "should get at least 2 users in bulk response"
        );

        let _dwr_resp = client
            .get(
                "/farcaster/user/by_username",
                &UserByUsernameQuery { username: "dwr" },
                "get_user_by_username",
            )
            .expect("get_user_by_username for dwr should succeed");
    }

    #[test]
    fn research_user_and_channel_workflow() {
        let Some(client) = client_or_skip() else {
            return;
        };

        let user_resp = client
            .get(
                "/farcaster/user/by_username",
                &UserByUsernameQuery { username: "dwr" },
                "get_user_by_username",
            )
            .expect("get_user_by_username should succeed");
        let user = &user_resp["user"];
        let fid = user["fid"].as_u64().expect("fid should be a number");
        assert!(fid > 0, "fid should be a positive integer");

        let user2_resp = client
            .get(
                "/farcaster/user/by_username",
                &UserByUsernameQuery {
                    username: "vitalik.eth",
                },
                "get_user_by_username",
            )
            .expect("get_user_by_username for vitalik.eth should succeed");
        let user2_fid = user2_resp["user"]["fid"]
            .as_u64()
            .expect("fid should be a number");

        let fids_param = format!("{fid},{user2_fid},99");
        let bulk_resp = client
            .get(
                "/farcaster/user/bulk",
                &BulkUsersQuery {
                    fids: fids_param.as_str(),
                },
                "bulk_users",
            )
            .expect("bulk_users should succeed");
        let bulk_users = bulk_resp["users"]
            .as_array()
            .expect("bulk should return users array");
        assert!(bulk_users.len() >= 2, "should get at least 2 users");
    }
}
