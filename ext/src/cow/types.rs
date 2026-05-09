use serde::{Deserialize, Deserializer, Serialize};
use serde_json::Value;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct QuoteRequest<'a> {
    pub sell_token: &'a str,
    pub buy_token: &'a str,
    pub sell_amount_before_fee: &'a str,
    pub from: &'a str,
    pub kind: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub receiver: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valid_to: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partially_fillable: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signing_scheme: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slippage_bps: Option<u32>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelOrdersRequest<'a> {
    pub order_uids: &'a [String],
    pub signature: &'a str,
    pub signing_scheme: &'a str,
}

fn deserialize_optional_f64ish<'de, D>(deserializer: D) -> Result<Option<f64>, D::Error>
where
    D: Deserializer<'de>,
{
    let value = Option::<Value>::deserialize(deserializer)?;
    Ok(value.and_then(|value| {
        value
            .as_f64()
            .or_else(|| value.as_str().and_then(|raw| raw.parse::<f64>().ok()))
    }))
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CowQuote {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub quote: Option<Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub from: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sell_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub buy_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sell_amount: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub buy_amount: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fee_amount: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub valid_to: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub partially_fillable: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CowOrder {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uid: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sell_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub buy_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sell_amount: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub buy_amount: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub executed_sell_amount: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub executed_buy_amount: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CowOrderStatus {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uid: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CowTrade {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uid: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub order_uid: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sell_amount: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub buy_amount: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fee_amount: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CowNativePrice {
    #[serde(default, deserialize_with = "deserialize_optional_f64ish")]
    pub price: Option<f64>,
}
