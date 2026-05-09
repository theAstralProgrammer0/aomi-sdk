use serde::Serialize;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PlaceBetRequest<'a> {
    pub contract_id: &'a str,
    pub amount: f64,
    pub outcome: &'a str,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateMarketRequest<'a> {
    pub outcome_type: &'a str,
    pub question: &'a str,
    pub initial_prob: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub close_time: Option<u64>,
}
