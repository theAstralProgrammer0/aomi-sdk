//! Curated tool layer for Delta RFQ Arena. Hand-written from the
//! progenitor-generated client in `crate::client` (see
//! `apps/delta/openapi.yaml`).
//!
//! Designed for the user story: place RFQ quotes on the local Delta RFQ
//! Arena, fill them, and view receipts of past fill attempts.
//!
//! 5 endpoints collapse into 4 user-centric tools:
//!
//!   * `delta_create_quote`       — POST /quotes
//!   * `delta_list_quotes`        — GET  /quotes
//!   * `delta_get_quote`          — GET  /quotes/{id} (+ optional receipts)
//!   * `delta_fill_quote`         — composite POST /fill + GET /receipts in one call

use crate::client::Client as GenClient;
use crate::client::types::{CreateQuoteRequest, FeedEvidence, FillQuoteRequest};
use aomi_sdk::schemars::JsonSchema;
use aomi_sdk::*;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

#[derive(Clone, Default)]
pub(crate) struct DeltaApp;

const DEFAULT_BASE_URL: &str = "http://localhost:3335";
const BASE_URL_ENV: &str = "DELTA_RFQ_API_URL";

// ============================================================================
// Helpers
// ============================================================================

fn ok<T: Serialize>(value: T) -> Result<Value, String> {
    let value = serde_json::to_value(value).map_err(|e| format!("[delta] serialize: {e}"))?;
    Ok(match value {
        Value::Object(mut m) => {
            m.insert("source".into(), Value::String("delta".into()));
            Value::Object(m)
        }
        other => json!({ "source": "delta", "data": other }),
    })
}

fn rt() -> Result<tokio::runtime::Runtime, String> {
    tokio::runtime::Runtime::new().map_err(|e| format!("[delta] runtime: {e}"))
}

fn base_url() -> String {
    std::env::var(BASE_URL_ENV).unwrap_or_else(|_| DEFAULT_BASE_URL.to_string())
}

// ============================================================================
// Slim projection structs — keep what the LLM can act on, drop opaque blobs.
// ============================================================================

#[derive(Serialize)]
struct QuoteSummary {
    id: String,
    text: String,
    status: String,
    asset: String,
    direction: String,
    size: f64,
    price_limit: Option<f64>,
    currency: String,
    expires_at: i64,
    created_at: i64,
    maker_owner_id: String,
    maker_shard: i64,
    constraints_summary: Option<String>,
    message: Option<String>,
}

impl From<crate::client::types::Quote> for QuoteSummary {
    fn from(q: crate::client::types::Quote) -> Self {
        Self {
            id: q.id,
            text: q.text,
            status: q.status,
            asset: q.asset,
            direction: q.direction,
            size: q.size,
            price_limit: q.price_limit,
            currency: q.currency,
            expires_at: q.expires_at,
            created_at: q.created_at,
            maker_owner_id: q.maker_owner_id,
            maker_shard: q.maker_shard,
            constraints_summary: q.constraints_summary,
            message: q.message,
        }
    }
}

#[derive(Serialize)]
struct ReceiptSummary {
    id: String,
    quote_id: String,
    success: bool,
    status: String,
    taker_owner_id: String,
    taker_shard: i64,
    size: f64,
    price: f64,
    attempted_at: i64,
    error_code: Option<String>,
    error_message: Option<String>,
    sdl_hash: Option<String>,
}

impl From<crate::client::types::Receipt> for ReceiptSummary {
    fn from(r: crate::client::types::Receipt) -> Self {
        Self {
            id: r.id,
            quote_id: r.quote_id,
            success: r.success,
            status: r.status,
            taker_owner_id: r.taker_owner_id,
            taker_shard: r.taker_shard,
            size: r.size,
            price: r.price,
            attempted_at: r.attempted_at,
            error_code: r.error_code,
            error_message: r.error_message,
            sdl_hash: r.sdl_hash,
        }
    }
}

// ============================================================================
// Shared FeedEvidence input shape (mirrors the client type but JsonSchema-able)
// ============================================================================

#[derive(Debug, Deserialize, JsonSchema)]
pub(crate) struct FeedEvidenceInput {
    /// Price-feed source name (e.g. "chainlink", "pyth", "redstone").
    pub source: String,
    /// Asset the price is for (e.g. "ETH", "BTC").
    pub asset: String,
    /// Price reported by this feed.
    pub price: f64,
    /// Unix timestamp (seconds) of the price observation.
    pub timestamp: i64,
    /// Cryptographic signature over the feed payload.
    pub signature: String,
}

impl From<FeedEvidenceInput> for FeedEvidence {
    fn from(f: FeedEvidenceInput) -> Self {
        Self {
            source: f.source,
            asset: f.asset,
            price: f.price,
            timestamp: f.timestamp,
            signature: f.signature,
        }
    }
}

// ============================================================================
// Tool 1: delta_create_quote — POST /quotes
// ============================================================================

pub(crate) struct CreateQuote;

#[derive(Debug, Deserialize, JsonSchema)]
pub(crate) struct CreateQuoteArgs {
    /// Natural-language quote, e.g. "Buy 10 dETH at most 2000 USDD, expires in 5 minutes".
    /// The backend compiles this into a Local Law that fills must satisfy.
    pub text: String,
    /// Maker's owner identifier.
    pub maker_owner_id: String,
    /// Maker's shard number (non-negative).
    pub maker_shard: i64,
}

impl DynAomiTool for CreateQuote {
    type App = DeltaApp;
    type Args = CreateQuoteArgs;
    const NAME: &'static str = "delta_create_quote";
    const DESCRIPTION: &'static str = "Use when the user (acting as Maker) wants to post a new RFQ quote. Provide the quote in plain English (asset, direction, size, price bound, expiration); the backend compiles it into a Local Law. Returns the new Quote with its id, status, parsed fields, and a constraints_summary describing the compiled Local Law.";

    fn run(_app: &DeltaApp, args: Self::Args, _ctx: DynToolCallCtx) -> Result<Value, String> {
        let body = CreateQuoteRequest {
            maker_owner_id: args.maker_owner_id,
            maker_shard: args.maker_shard,
            text: args.text,
        };
        let runtime = rt()?;
        let quote = runtime
            .block_on(async move {
                let client = GenClient::new(&base_url());
                client.create_quote(&body).await
            })
            .map_err(|e| format!("[delta] create_quote: {e}"))?
            .into_inner();
        ok(QuoteSummary::from(quote))
    }
}

// ============================================================================
// Tool 2: delta_list_quotes — GET /quotes
// ============================================================================

pub(crate) struct ListQuotes;

#[derive(Debug, Deserialize, JsonSchema)]
pub(crate) struct ListQuotesArgs {
    /// Limit returned quotes (default 50). Applied client-side after the fetch.
    #[serde(default)]
    pub limit: Option<usize>,
    /// Optional status filter ("active", "filled", "expired", "cancelled").
    #[serde(default)]
    pub status: Option<String>,
}

impl DynAomiTool for ListQuotes {
    type App = DeltaApp;
    type Args = ListQuotesArgs;
    const NAME: &'static str = "delta_list_quotes";
    const DESCRIPTION: &'static str = "Use when the user wants to browse the arena for trading opportunities or audit their own posted quotes. Returns a slim list of quotes (id, text, status, asset, direction, size, price_limit, expires_at, maker). Optionally filter by status and cap the count.";

    fn run(_app: &DeltaApp, args: Self::Args, _ctx: DynToolCallCtx) -> Result<Value, String> {
        let runtime = rt()?;
        let quotes = runtime
            .block_on(async move {
                let client = GenClient::new(&base_url());
                client.list_quotes().await
            })
            .map_err(|e| format!("[delta] list_quotes: {e}"))?
            .into_inner();

        let limit = args.limit.unwrap_or(50);
        let want_status = args.status.as_deref();
        let trimmed: Vec<QuoteSummary> = quotes
            .into_iter()
            .filter(|q| match want_status {
                Some(s) => q.status.eq_ignore_ascii_case(s),
                None => true,
            })
            .take(limit)
            .map(QuoteSummary::from)
            .collect();
        ok(json!({ "quotes": trimmed }))
    }
}

// ============================================================================
// Tool 3: delta_get_quote — GET /quotes/{id} (+ optional receipts)
// ============================================================================

pub(crate) struct GetQuote;

#[derive(Debug, Deserialize, JsonSchema)]
pub(crate) struct GetQuoteArgs {
    /// Quote identifier (returned by `delta_create_quote` or `delta_list_quotes`).
    pub quote_id: String,
    /// Also fetch the quote's fill receipts. Defaults to false.
    #[serde(default)]
    pub include_receipts: Option<bool>,
}

impl DynAomiTool for GetQuote {
    type App = DeltaApp;
    type Args = GetQuoteArgs;
    const NAME: &'static str = "delta_get_quote";
    const DESCRIPTION: &'static str = "Use when the user names a specific quote (by id) and wants its current state — parsed fields, lifecycle status, and the human-readable Local-Law constraint summary. Set include_receipts=true to also pull every fill attempt against this quote in one call.";

    fn run(_app: &DeltaApp, args: Self::Args, _ctx: DynToolCallCtx) -> Result<Value, String> {
        let include_receipts = args.include_receipts.unwrap_or(false);
        let quote_id = args.quote_id.clone();
        let runtime = rt()?;
        let result = runtime.block_on(async move {
            let client = GenClient::new(&base_url());
            let quote = client
                .get_quote(quote_id.as_str())
                .await
                .map_err(|e| format!("[delta] get_quote {quote_id}: {e}"))?
                .into_inner();
            let receipts = if include_receipts {
                let r = client
                    .get_receipts(quote_id.as_str())
                    .await
                    .map_err(|e| format!("[delta] get_receipts {quote_id}: {e}"))?
                    .into_inner();
                Some(r.into_iter().map(ReceiptSummary::from).collect::<Vec<_>>())
            } else {
                None
            };
            Ok::<_, String>(json!({
                "quote": QuoteSummary::from(quote),
                "receipts": receipts,
            }))
        })?;
        ok(result)
    }
}

// ============================================================================
// Tool 4: delta_fill_quote — composite POST /fill + GET /receipts
// ============================================================================

pub(crate) struct FillQuote;

#[derive(Debug, Deserialize, JsonSchema)]
pub(crate) struct FillQuoteArgs {
    /// Quote to fill (id from `delta_list_quotes` or `delta_get_quote`).
    pub quote_id: String,
    /// Taker's owner identifier.
    pub taker_owner_id: String,
    /// Taker's shard number (non-negative).
    pub taker_shard: i64,
    /// Size to fill.
    pub size: f64,
    /// Price at which to fill — must satisfy the quote's Local Law.
    pub price: f64,
    /// Signed price-feed evidence backing the fill (multiple sources recommended).
    pub feed_evidence: Vec<FeedEvidenceInput>,
}

impl DynAomiTool for FillQuote {
    type App = DeltaApp;
    type Args = FillQuoteArgs;
    const NAME: &'static str = "delta_fill_quote";
    const DESCRIPTION: &'static str = "Use when the user (acting as Taker) wants to fill an existing quote. Composite tool: posts the fill with signed feed_evidence, then fetches the quote's full receipt history so the LLM can confirm settlement state. Returns { fill: success/error/proof/receipt, receipts: [...] }. The fill only settles if every Local-Law constraint is satisfied; otherwise the response carries an error describing the violated constraint.";

    fn run(_app: &DeltaApp, args: Self::Args, _ctx: DynToolCallCtx) -> Result<Value, String> {
        if args.feed_evidence.is_empty() {
            return Err(
                "[delta] fill_quote: feed_evidence must contain at least one signed feed".into(),
            );
        }
        let body = FillQuoteRequest {
            feed_evidence: args.feed_evidence.into_iter().map(Into::into).collect(),
            price: args.price,
            size: args.size,
            taker_owner_id: args.taker_owner_id,
            taker_shard: args.taker_shard,
        };
        let quote_id = args.quote_id.clone();
        let runtime = rt()?;
        let result = runtime.block_on(async move {
            let client = GenClient::new(&base_url());
            let fill = client
                .fill_quote(quote_id.as_str(), &body)
                .await
                .map_err(|e| format!("[delta] fill_quote {quote_id}: {e}"))?
                .into_inner();
            let receipts = client
                .get_receipts(quote_id.as_str())
                .await
                .map_err(|e| format!("[delta] post-fill receipts {quote_id}: {e}"))?
                .into_inner();
            let receipts: Vec<ReceiptSummary> =
                receipts.into_iter().map(ReceiptSummary::from).collect();
            Ok::<_, String>(json!({
                "fill": fill,
                "receipts": receipts,
            }))
        })?;
        ok(result)
    }
}
