use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

pub fn de_opt_string<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let value = Option::<Value>::deserialize(deserializer)?;
    Ok(match value {
        None | Some(Value::Null) => None,
        Some(Value::String(s)) => Some(s),
        Some(Value::Number(n)) => Some(n.to_string()),
        Some(Value::Bool(b)) => Some(b.to_string()),
        Some(other) => Some(other.to_string()),
    })
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OneInchTransaction {
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "de_opt_string"
    )]
    pub from: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "de_opt_string"
    )]
    pub to: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "de_opt_string"
    )]
    pub data: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "de_opt_string"
    )]
    pub value: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OneInchAllowanceResponse {
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "de_opt_string"
    )]
    pub allowance: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OneInchQuoteResponse {
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "de_opt_string"
    )]
    pub src_amount: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "de_opt_string"
    )]
    pub dst_amount: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub protocols: Option<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OneInchSwapResponse {
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "de_opt_string"
    )]
    pub src_amount: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "de_opt_string"
    )]
    pub dst_amount: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub protocols: Option<Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tx: Option<OneInchTransaction>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OneInchLiquiditySource {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OneInchLiquiditySourcesResponse {
    pub protocols: Vec<OneInchLiquiditySource>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OneInchToken {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "de_opt_string"
    )]
    pub address: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub decimals: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OneInchTokensResponse {
    pub tokens: HashMap<String, OneInchToken>,
}
