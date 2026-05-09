use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SwapQuoteQuery<'a> {
    pub chain_id: u64,
    pub sell_token: &'a str,
    pub buy_token: &'a str,
    pub sell_amount: &'a str,
    pub slippage_percentage: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub taker: Option<&'a str>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SourcesQuery {
    pub chain_id: u64,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GaslessStatusQuery {
    pub chain_id: u64,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GaslessSubmitRequest<'a> {
    pub chain_id: u64,
    pub trade: &'a Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approval: Option<&'a Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ZeroxTransactionPayload {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub to: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data: Option<Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gas: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gas_price: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_fee_per_gas: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_priority_fee_per_gas: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ZeroxSwapQuotePayload {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub chain_id: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub buy_amount: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sell_amount: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub min_buy_amount: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gross_buy_amount: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub estimated_price_impact: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub price: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allowance_target: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub liquidity_available: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub transaction: Option<ZeroxTransactionPayload>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fees: Option<Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub issues: Option<Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub route: Option<Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub token_metadata: Option<Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub approval: Option<Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub trade: Option<Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub trade_hash: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ZeroxChainPayload {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub chain_id: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub chain_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ZeroxLiquiditySourcePayload {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub proportion: Option<String>,
}
