use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AcrossRoute {
    pub origin_chain_id: u64,
    pub origin_token: String,
    pub destination_chain_id: u64,
    pub destination_token: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub origin_token_symbol: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub destination_token_symbol: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_native: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AcrossFeeComponent {
    pub pct: String,
    pub total: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AcrossLimits {
    pub min_deposit: String,
    pub max_deposit: String,
    pub max_deposit_instant: String,
    pub max_deposit_short_delay: String,
    pub recommended_deposit_instant: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AcrossTokenRef {
    pub address: String,
    pub symbol: String,
    pub decimals: u64,
    pub chain_id: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AcrossSuggestedFees {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub estimated_fill_time_sec: Option<u64>,
    pub total_relay_fee: AcrossFeeComponent,
    pub relayer_capital_fee: AcrossFeeComponent,
    pub relayer_gas_fee: AcrossFeeComponent,
    pub lp_fee: AcrossFeeComponent,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limits: Option<AcrossLimits>,
    pub output_amount: String,
    pub input_token: AcrossTokenRef,
    pub output_token: AcrossTokenRef,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AcrossDepositStatus {
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AcrossTokenPrice {
    pub price: f64,
}
