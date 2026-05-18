//! GameFi dynamic plugin — Molinar 3D world bot agent.

use crate::types::{
    ChatRequest, CreateObjectRequest, CustomizeRequest, EmptyRequest, ExploreRequest, MoveRequest,
    PingRequest,
};
use aomi_sdk::schemars::JsonSchema;
use aomi_sdk::*;
use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;
use std::collections::HashSet;
use std::sync::Mutex;
use std::sync::OnceLock;
use std::time::Duration;

#[derive(Clone, Default)]
pub(crate) struct MolinarApp;

pub(crate) use crate::tool::*;

// ============================================================================
// Molinar Client (blocking)
// ============================================================================

pub(crate) const DEFAULT_MOLINAR_API: &str = "https://molinar.ai/api/bot";

#[derive(Clone)]
pub struct MolinarClient {
    pub(crate) http: reqwest::blocking::Client,
    pub(crate) api_endpoint: String,
}

impl MolinarClient {
    pub fn new() -> Result<Self, String> {
        let http = reqwest::blocking::Client::builder()
            .timeout(Duration::from_secs(30))
            .build()
            .map_err(|e| format!("failed to build HTTP client: {e}"))?;
        Ok(Self {
            http,
            api_endpoint: std::env::var("MOLINAR_API_ENDPOINT")
                .unwrap_or_else(|_| DEFAULT_MOLINAR_API.to_string()),
        })
    }

    pub fn get_json(&self, url: &str, op: &str) -> Result<Value, String> {
        let response = self
            .http
            .get(url)
            .send()
            .map_err(|e| format!("[molinar] {op} request failed ({url}): {e}"))?;

        let status = response.status();
        let body = response.text().unwrap_or_default();
        if !status.is_success() {
            return Err(format!("[molinar] {op} failed ({url}): {status} {body}"));
        }

        serde_json::from_str::<Value>(&body)
            .map_err(|e| format!("[molinar] {op} decode failed ({url}): {e}"))
    }

    pub fn post_json<B: Serialize>(&self, url: &str, body: &B, op: &str) -> Result<Value, String> {
        let response = self
            .http
            .post(url)
            .json(body)
            .send()
            .map_err(|e| format!("[molinar] {op} request failed ({url}): {e}"))?;

        let status = response.status();
        let text = response.text().unwrap_or_default();
        if !status.is_success() {
            return Err(format!("[molinar] {op} failed ({url}): {status} {text}"));
        }

        serde_json::from_str::<Value>(&text)
            .map_err(|e| format!("[molinar] {op} decode failed ({url}): {e}"))
    }

    // ── API Methods ──────────────────────────────────────────────────────

    /// GET /{botId}/state
    pub fn get_state(&self, bot_id: &str) -> Result<Value, String> {
        self.get_json(
            &format!("{}/{}/state", self.api_endpoint, bot_id),
            "get_state",
        )
    }

    /// GET /{botId}/look
    pub fn look(&self, bot_id: &str) -> Result<Value, String> {
        self.get_json(&format!("{}/{}/look", self.api_endpoint, bot_id), "look")
    }

    /// POST /{botId}/move
    pub fn move_bot(&self, bot_id: &str, payload: &MoveRequest) -> Result<Value, String> {
        self.post_json(
            &format!("{}/{}/move", self.api_endpoint, bot_id),
            payload,
            "move",
        )
    }

    /// POST /{botId}/jump
    pub fn jump(&self, bot_id: &str) -> Result<Value, String> {
        self.post_json(
            &format!("{}/{}/jump", self.api_endpoint, bot_id),
            &EmptyRequest::default(),
            "jump",
        )
    }

    /// POST /{botId}/chat
    pub fn send_chat(&self, bot_id: &str, message: &str) -> Result<Value, String> {
        let body = ChatRequest {
            message: message.to_string(),
        };
        self.post_json(
            &format!("{}/{}/chat", self.api_endpoint, bot_id),
            &body,
            "chat",
        )
    }

    /// GET /{botId}/chat
    pub fn get_chat(&self, bot_id: &str) -> Result<Value, String> {
        self.get_json(
            &format!("{}/{}/chat", self.api_endpoint, bot_id),
            "get_chat",
        )
    }

    /// GET /{botId}/chat/new
    pub fn get_new_messages(&self, bot_id: &str) -> Result<Value, String> {
        self.get_json(
            &format!("{}/{}/chat/new", self.api_endpoint, bot_id),
            "get_new_messages",
        )
    }

    /// GET /{botId}/players
    pub fn get_players(&self, bot_id: &str) -> Result<Value, String> {
        self.get_json(
            &format!("{}/{}/players", self.api_endpoint, bot_id),
            "get_players",
        )
    }

    /// POST /{botId}/collect
    pub fn collect_coins(&self, bot_id: &str) -> Result<Value, String> {
        self.post_json(
            &format!("{}/{}/collect", self.api_endpoint, bot_id),
            &EmptyRequest::default(),
            "collect",
        )
    }

    /// POST /{botId}/explore
    pub fn explore(&self, bot_id: &str, payload: &ExploreRequest) -> Result<Value, String> {
        self.post_json(
            &format!("{}/{}/explore", self.api_endpoint, bot_id),
            payload,
            "explore",
        )
    }

    /// POST /{botId}/create
    pub fn create_object(&self, bot_id: &str, prompt: &str) -> Result<Value, String> {
        let body = CreateObjectRequest {
            prompt: prompt.to_string(),
        };
        self.post_json(
            &format!("{}/{}/create", self.api_endpoint, bot_id),
            &body,
            "create_object",
        )
    }

    /// POST /{botId}/customize
    pub fn customize(&self, bot_id: &str, payload: &CustomizeRequest) -> Result<Value, String> {
        self.post_json(
            &format!("{}/{}/customize", self.api_endpoint, bot_id),
            payload,
            "customize",
        )
    }

    /// POST /{botId}/ping
    pub fn ping(&self, bot_id: &str, payload: &PingRequest) -> Result<Value, String> {
        self.post_json(
            &format!("{}/{}/ping", self.api_endpoint, bot_id),
            payload,
            "ping",
        )
    }
}

// ============================================================================
// Helper: extract bot_id from context
// ============================================================================

/// Tracks session_ids we've already auto-connected, so we only POST /connect
/// once per process per session.
static AUTO_CONNECTED: OnceLock<Mutex<HashSet<String>>> = OnceLock::new();

fn already_connected(key: &str) -> bool {
    let set = AUTO_CONNECTED.get_or_init(|| Mutex::new(HashSet::new()));
    let guard = set.lock().expect("AUTO_CONNECTED poisoned");
    guard.contains(key)
}

fn mark_connected(key: &str) {
    let set = AUTO_CONNECTED.get_or_init(|| Mutex::new(HashSet::new()));
    let mut guard = set.lock().expect("AUTO_CONNECTED poisoned");
    guard.insert(key.to_string());
}

/// Sanitize a session_id into something safe for the Molinar API path. The
/// host's session_ids are arbitrary strings (often UUIDs) — strip anything
/// that isn't alphanumeric/dash/underscore so the URL stays well-formed.
fn sanitize(raw: &str) -> String {
    let s: String = raw
        .chars()
        .filter(|c| c.is_ascii_alphanumeric() || *c == '-' || *c == '_')
        .take(48)
        .collect();
    if s.is_empty() {
        "aomi-bot".to_string()
    } else {
        s
    }
}

/// Resolve a Molinar bot_id, with three fallbacks for ergonomic E2E testing:
///
/// 1. Explicit `state_attributes.bot_id` (production path — the integration
///    layer wires a real bot_id from the user's account).
/// 2. `MOLINAR_BOT_ID` env var.
/// 3. Derive a stable bot_id from `ctx.session_id` and lazily POST `/connect`
///    once per process so the rest of the URL routes (`{id}/state`, etc.)
///    succeed. This makes the app self-bootstrapping for the local-app-e2e
///    harness, which has no Molinar-specific wiring.
pub(crate) fn get_bot_id(ctx: &DynToolCallCtx) -> Result<String, String> {
    if let Some(id) = ctx
        .state_attributes
        .get("bot_id")
        .and_then(|v| v.as_str())
        .filter(|s| !s.is_empty())
    {
        return Ok(id.to_string());
    }

    if let Ok(id) = std::env::var("MOLINAR_BOT_ID")
        && !id.is_empty()
    {
        return Ok(id);
    }

    let session_id = sanitize(&ctx.session_id);
    if session_id.is_empty() {
        return Err(
            "bot_id not found in context state_attributes and ctx.session_id is empty".to_string(),
        );
    }

    if !already_connected(&session_id) {
        let client = MolinarClient::new()?;
        let url = format!("{}/connect", client.api_endpoint);
        let body = serde_json::json!({ "session_id": session_id });
        let _ = client
            .http
            .post(&url)
            .json(&body)
            .send()
            .map_err(|e| format!("[molinar] auto-connect request failed: {e}"))?;
        mark_connected(&session_id);
    }

    Ok(session_id)
}

// ============================================================================
// Tool 1: Get World State
// ============================================================================

pub(crate) struct MolinarGetState;

#[derive(Debug, Deserialize, JsonSchema)]
pub(crate) struct MolinarGetStateArgs {}
