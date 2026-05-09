use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct SimmerRegisterRequest {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct ImportKalshiMarketRequest {
    pub kalshi_url: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct SimmerTradeRequest {
    pub market_id: String,
    pub side: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shares: Option<f64>,
    pub venue: String,
    pub action: String,
    pub source: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dry_run: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reasoning: Option<String>,
}
