//! Curated tool layer for Para — embedded MPC wallets via REST.
//!
//! Wraps the generated client in `crate::client` (see `apps/para/openapi.yaml`).
//! Designed for the user story: "manage embedded wallets via Para — create a
//! wallet for a user identifier, look it up, sign raw payloads."
//!
//! The 3 spec endpoints map 1:1 to 3 user-centric tools:
//!
//!   * `para_create_wallet`  — POST /v1/wallets
//!   * `para_get_wallet`     — GET  /v1/wallets/{walletId}
//!   * `para_sign_payload`   — POST /v1/wallets/{walletId}/sign-raw
//!
//! Auth: Para requires `X-API-Key` on every request. Progenitor's spec only
//! declares the header name, so we wire it in via `make_client(api_key)` —
//! a `reqwest::Client` with the header preset, fed to `Client::new_with_client`.

use crate::client::Client as GenClient;
use crate::client::types::{
    CreateWalletRequest, CreateWalletRequestScheme, CreateWalletRequestType,
    CreateWalletRequestUserIdentifierType, SignRawRequest, SignRawRequestData,
};
use aomi_sdk::schemars::JsonSchema;
use aomi_sdk::*;
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::str::FromStr;
use std::time::Duration;

#[derive(Clone, Default)]
pub(crate) struct ParaApp;

const BASE_URL: &str = "https://api.beta.getpara.com";

// ============================================================================
// Helpers
// ============================================================================

fn ok<T: Serialize>(value: T) -> Result<Value, String> {
    let value = serde_json::to_value(value).map_err(|e| format!("[para] serialize: {e}"))?;
    Ok(match value {
        Value::Object(mut m) => {
            m.insert("source".into(), Value::String("para".into()));
            Value::Object(m)
        }
        other => json!({ "source": "para", "data": other }),
    })
}

fn rt() -> Result<tokio::runtime::Runtime, String> {
    tokio::runtime::Runtime::new().map_err(|e| format!("[para] runtime: {e}"))
}

fn resolve_key(ctx: &DynToolCallCtx,
    arg: Option<&str>) -> Result<String, String> {
    resolve_secret_value(ctx, arg,
        "PARA_API_KEY",
        "[para] missing api_key argument and PARA_API_KEY env var",
    )
}

/// Build a generated client with `X-API-Key` set as a default header.
/// Para's spec only declares the header name; we wire the value in here.
fn make_client(api_key: &str) -> Result<GenClient, String> {
    let mut headers = HeaderMap::new();
    let mut key =
        HeaderValue::from_str(api_key).map_err(|e| format!("[para] invalid api_key: {e}"))?;
    key.set_sensitive(true);
    let name = HeaderName::from_static("x-api-key");
    headers.insert(name, key);

    let http = reqwest::ClientBuilder::new()
        .timeout(Duration::from_secs(30))
        .default_headers(headers)
        .build()
        .map_err(|e| format!("[para] failed to build HTTP client: {e}"))?;

    Ok(GenClient::new_with_client(BASE_URL, http))
}

fn parse_chain(s: &str) -> Result<CreateWalletRequestType, String> {
    CreateWalletRequestType::from_str(s)
        .map_err(|_| format!("[para] invalid chain {s:?}; expected one of EVM, SOLANA, COSMOS"))
}

fn parse_identifier_type(s: &str) -> Result<CreateWalletRequestUserIdentifierType, String> {
    CreateWalletRequestUserIdentifierType::from_str(s).map_err(|_| {
        format!(
            "[para] invalid identifier_type {s:?}; expected one of EMAIL, PHONE, CUSTOM_ID, GUEST_ID, TELEGRAM, DISCORD, TWITTER"
        )
    })
}

fn parse_scheme(s: &str) -> Result<CreateWalletRequestScheme, String> {
    CreateWalletRequestScheme::from_str(s)
        .map_err(|_| format!("[para] invalid scheme {s:?}; expected one of DKLS, CGGMP, ED25519"))
}

// ============================================================================
// Tool 1: para_create_wallet — POST /v1/wallets
// ============================================================================

pub(crate) struct CreateWallet;

#[derive(Debug, Deserialize, JsonSchema)]
pub(crate) struct CreateWalletArgs {
    /// Optional Para API key. Falls back to PARA_API_KEY env var.
    #[serde(default)]
    pub api_key: Option<String>,
    /// Chain family for the wallet. One of: EVM, SOLANA, COSMOS.
    pub chain: String,
    /// User handle the wallet is anchored to (email address, phone number, custom string, etc.).
    pub user_identifier: String,
    /// How Para should validate / normalize `user_identifier`. One of: EMAIL,
    /// PHONE, CUSTOM_ID, GUEST_ID, TELEGRAM, DISCORD, TWITTER.
    pub identifier_type: String,
    /// Optional MPC signing scheme override. One of: DKLS, CGGMP, ED25519.
    /// Para picks a chain-appropriate default if omitted.
    #[serde(default)]
    pub scheme: Option<String>,
    /// Optional bech32 prefix for Cosmos wallets only (e.g. "cosmos", "osmo").
    #[serde(default)]
    pub cosmos_prefix: Option<String>,
}

impl DynAomiTool for CreateWallet {
    type App = ParaApp;
    type Args = CreateWalletArgs;
    const NAME: &'static str = "para_create_wallet";
    const DESCRIPTION: &'static str = "Use when the user wants to provision a new Para MPC wallet for an end-user identifier (email/phone/custom ID). Creation is asynchronous: the response usually has `status = \"creating\"` and you must poll `para_get_wallet` until `status = \"ready\"` before signing. If a wallet for that (chain, scheme, identifier) already exists, Para returns the existing record.";

    fn run(_app: &ParaApp, args: Self::Args, ctx: DynToolCallCtx) -> Result<Value, String> {
        let api_key = resolve_key(&ctx, args.api_key.as_deref())?;
        let chain = parse_chain(&args.chain)?;
        let id_type = parse_identifier_type(&args.identifier_type)?;
        let scheme = args.scheme.as_deref().map(parse_scheme).transpose()?;

        let body = CreateWalletRequest {
            type_: chain,
            user_identifier: args.user_identifier,
            user_identifier_type: id_type,
            scheme,
            cosmos_prefix: args.cosmos_prefix,
        };

        let client = make_client(&api_key)?;
        let runtime = rt()?;
        let wallet = runtime
            .block_on(async move { client.create_wallet(None, &body).await })
            .map_err(|e| format!("[para] create_wallet: {e}"))?
            .into_inner();

        ok(json!({
            "id": wallet.id,
            "status": wallet.status,
            "type": wallet.type_,
            "scheme": wallet.scheme,
            "address": wallet.address,
            "public_key": wallet.public_key,
            "user_identifier": wallet.user_identifier,
            "user_identifier_type": wallet.user_identifier_type,
            "cosmos_prefix": wallet.cosmos_prefix,
        }))
    }
}

// ============================================================================
// Tool 2: para_get_wallet — GET /v1/wallets/{walletId}
// ============================================================================

pub(crate) struct GetWallet;

#[derive(Debug, Deserialize, JsonSchema)]
pub(crate) struct GetWalletArgs {
    /// Optional Para API key. Falls back to PARA_API_KEY env var.
    #[serde(default)]
    pub api_key: Option<String>,
    /// Para wallet identifier (UUID returned by `para_create_wallet`).
    pub wallet_id: String,
}

impl DynAomiTool for GetWallet {
    type App = ParaApp;
    type Args = GetWalletArgs;
    const NAME: &'static str = "para_get_wallet";
    const DESCRIPTION: &'static str = "Use to look up a Para wallet by ID — returns its lifecycle `status` (creating/ready/error), on-chain `address`, `public_key`, and signing scheme. Call this after `para_create_wallet` to poll until `status = \"ready\"` before attempting `para_sign_payload`.";

    fn run(_app: &ParaApp, args: Self::Args, ctx: DynToolCallCtx) -> Result<Value, String> {
        let api_key = resolve_key(&ctx, args.api_key.as_deref())?;
        let client = make_client(&api_key)?;
        let runtime = rt()?;
        let wallet_id = args.wallet_id.clone();
        let wallet = runtime
            .block_on(async move { client.get_wallet(wallet_id.as_str(), None).await })
            .map_err(|e| format!("[para] get_wallet {}: {e}", args.wallet_id))?
            .into_inner();

        ok(json!({
            "id": wallet.id,
            "status": wallet.status,
            "type": wallet.type_,
            "scheme": wallet.scheme,
            "address": wallet.address,
            "public_key": wallet.public_key,
            "user_identifier": wallet.user_identifier,
            "user_identifier_type": wallet.user_identifier_type,
            "cosmos_prefix": wallet.cosmos_prefix,
        }))
    }
}

// ============================================================================
// Tool 3: para_sign_payload — POST /v1/wallets/{walletId}/sign-raw
// ============================================================================

pub(crate) struct SignPayload;

#[derive(Debug, Deserialize, JsonSchema)]
pub(crate) struct SignPayloadArgs {
    /// Optional Para API key. Falls back to PARA_API_KEY env var.
    #[serde(default)]
    pub api_key: Option<String>,
    /// Para wallet ID. Must be in `status = "ready"` (see `para_get_wallet`).
    pub wallet_id: String,
    /// 0x-prefixed hex bytes to sign. For EVM, this is typically the keccak256
    /// digest of an RLP-encoded transaction or an EIP-712 message hash.
    pub data: String,
}

impl DynAomiTool for SignPayload {
    type App = ParaApp;
    type Args = SignPayloadArgs;
    const NAME: &'static str = "para_sign_payload";
    const DESCRIPTION: &'static str = "Use to MPC-sign raw 0x-prefixed hex bytes with a Para wallet. The wallet must be in `status = \"ready\"` first (verify with `para_get_wallet`). Returns a hex-encoded signature in `signature` (some schemes use `sig`). The caller is responsible for hashing/encoding the payload — Para signs whatever bytes you supply.";

    fn run(_app: &ParaApp, args: Self::Args, ctx: DynToolCallCtx) -> Result<Value, String> {
        let api_key = resolve_key(&ctx, args.api_key.as_deref())?;
        let data = SignRawRequestData::from_str(&args.data)
            .map_err(|e| format!("[para] data must be 0x-prefixed hex: {e}"))?;
        let body = SignRawRequest { data };
        let client = make_client(&api_key)?;
        let runtime = rt()?;
        let wallet_id = args.wallet_id.clone();
        let resp = runtime
            .block_on(async move { client.sign_raw(wallet_id.as_str(), None, &body).await })
            .map_err(|e| format!("[para] sign_raw {}: {e}", args.wallet_id))?
            .into_inner();

        ok(json!({
            "wallet_id": args.wallet_id,
            "signature": resp.signature.clone().or(resp.sig.clone()),
            "signature_field": if resp.signature.is_some() { "signature" } else { "sig" },
        }))
    }
}
