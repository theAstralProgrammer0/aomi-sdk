use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct QuoteQuery<'a> {
    pub from_chain: &'a str,
    pub to_chain: &'a str,
    pub from_token: &'a str,
    pub to_token: &'a str,
    pub from_amount: &'a str,
    pub from_address: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_address: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slippage: Option<f64>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StatusQuery<'a> {
    pub tx_hash: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_chain: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_chain: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bridge: Option<&'a str>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ChainsQuery<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chain_types: Option<&'a str>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TokensQuery<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chains: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chain_types: Option<&'a str>,
}

#[derive(Debug, Serialize)]
pub struct TokenQuery<'a> {
    pub chain: &'a str,
    pub token: &'a str,
}

#[derive(Debug, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct RouteOptions<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slippage: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<&'a str>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RouteRequest<'a> {
    pub from_chain_id: u64,
    pub to_chain_id: u64,
    pub from_token_address: &'a str,
    pub to_token_address: &'a str,
    pub from_amount: &'a str,
    pub from_address: &'a str,
    pub options: RouteOptions<'a>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ConnectionsQuery<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_chain: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_chain: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_token: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_token: Option<&'a str>,
}

#[derive(Debug, Serialize)]
pub struct ToolsQuery<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chains: Option<&'a str>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ReverseQuoteQuery<'a> {
    pub from_chain: &'a str,
    pub to_chain: &'a str,
    pub from_token: &'a str,
    pub to_token: &'a str,
    pub to_amount: &'a str,
    pub from_address: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_address: Option<&'a str>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GasSuggestionQuery<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_chain: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_token: Option<&'a str>,
}

#[derive(Debug, Serialize)]
pub struct PreparedTransaction {
    pub to: Value,
    pub data: Value,
    pub value: Value,
    pub gas_limit: Value,
    pub description: &'static str,
}

#[derive(Debug, Serialize)]
pub struct PreparedOrder {
    pub raw_quote: Value,
    pub main_tx: PreparedTransaction,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approval_tx: Option<PreparedTransaction>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LifiToolDetails {
    pub key: Option<String>,
    pub name: Option<String>,
    pub logo_uri: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LifiTokenRef {
    pub address: Option<String>,
    pub symbol: Option<String>,
    pub decimals: Option<u8>,
    pub chain_id: Option<u64>,
    pub name: Option<String>,
    pub coin_key: Option<String>,
    #[serde(rename = "priceUSD")]
    pub price_usd: Option<String>,
    pub logo_uri: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LifiAction {
    pub from_chain_id: Option<u64>,
    pub to_chain_id: Option<u64>,
    pub from_token: Option<LifiTokenRef>,
    pub to_token: Option<LifiTokenRef>,
    pub from_amount: Option<String>,
    pub slippage: Option<f64>,
    pub from_address: Option<String>,
    pub to_address: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LifiCost {
    #[serde(rename = "amountUSD")]
    pub amount_usd: Option<String>,
}

impl LifiCost {
    pub fn amount_usd_f64(&self) -> Option<f64> {
        self.amount_usd.as_deref()?.parse::<f64>().ok()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LifiEstimate {
    pub from_amount: Option<String>,
    pub to_amount: Option<String>,
    pub to_amount_min: Option<String>,
    #[serde(rename = "fromAmountUSD")]
    pub from_amount_usd: Option<String>,
    #[serde(rename = "toAmountUSD")]
    pub to_amount_usd: Option<String>,
    pub approval_address: Option<String>,
    pub execution_duration: Option<u64>,
    #[serde(default)]
    pub fee_costs: Vec<LifiCost>,
    #[serde(default)]
    pub gas_costs: Vec<LifiCost>,
    pub tool: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LifiStep {
    pub id: Option<String>,
    #[serde(rename = "type")]
    pub step_type: Option<String>,
    pub tool: Option<String>,
    pub tool_details: Option<LifiToolDetails>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LifiTransactionRequest {
    pub to: Option<Value>,
    pub data: Option<Value>,
    pub value: Option<Value>,
    pub gas_limit: Option<Value>,
    pub gas: Option<Value>,
    pub from: Option<Value>,
    pub gas_price: Option<Value>,
    pub chain_id: Option<Value>,
}

impl LifiTransactionRequest {
    pub fn to_prepared_transaction(&self, description: &'static str) -> PreparedTransaction {
        PreparedTransaction {
            to: self.to.clone().unwrap_or(Value::Null),
            data: self
                .data
                .clone()
                .unwrap_or_else(|| Value::String("0x".to_string())),
            value: self
                .value
                .clone()
                .unwrap_or_else(|| Value::String("0".to_string())),
            gas_limit: self
                .gas_limit
                .clone()
                .or_else(|| self.gas.clone())
                .unwrap_or(Value::Null),
            description,
        }
    }

    pub fn to_executable_transaction(&self) -> ExecutableTransaction {
        ExecutableTransaction {
            to: self.to.clone().unwrap_or(Value::Null),
            data: self.data.clone().unwrap_or(Value::Null),
            value: self.value.clone().unwrap_or(Value::Null),
            gas_limit: self.gas_limit.clone().unwrap_or(Value::Null),
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LifiQuoteResponse {
    pub id: Option<String>,
    #[serde(rename = "type")]
    pub quote_type: Option<String>,
    pub tool: Option<String>,
    pub tool_details: Option<LifiToolDetails>,
    pub action: Option<LifiAction>,
    pub estimate: LifiEstimate,
    #[serde(default)]
    pub included_steps: Vec<LifiStep>,
    pub integrator: Option<String>,
    pub transaction_id: Option<String>,
    pub transaction_request: Option<LifiTransactionRequest>,
}

impl LifiQuoteResponse {
    pub fn bridge_name(&self) -> String {
        self.tool_details
            .as_ref()
            .and_then(|details| details.name.clone())
            .or_else(|| self.tool.clone())
            .or_else(|| self.estimate.tool.clone())
            .unwrap_or_else(|| "unknown".to_string())
    }

    pub fn route_summary(&self) -> Vec<String> {
        let mut steps: Vec<String> = self
            .included_steps
            .iter()
            .filter_map(|step| step.tool.clone())
            .collect();
        if steps.is_empty() {
            steps.push(self.bridge_name());
        }
        steps
    }

    pub fn total_fee_usd(&self) -> Option<String> {
        let total = self
            .estimate
            .fee_costs
            .iter()
            .chain(self.estimate.gas_costs.iter())
            .filter_map(LifiCost::amount_usd_f64)
            .sum::<f64>();
        (total > 0.0).then(|| format!("{total:.2}"))
    }
}

#[derive(Debug, Serialize)]
pub struct ExecutableTransaction {
    pub to: Value,
    pub data: Value,
    pub value: Value,
    pub gas_limit: Value,
}

#[derive(Debug, Serialize)]
pub struct BridgeQuoteResponse {
    pub from: String,
    pub to: String,
    pub to_amount_estimate: Option<String>,
    pub min_received: Option<String>,
    pub bridge: String,
    pub estimated_duration_seconds: Option<u64>,
    pub estimated_fee_usd: Option<String>,
    pub route_summary: Vec<String>,
    pub executable_tx: Option<ExecutableTransaction>,
    pub execution_supported: bool,
    pub warning: Option<String>,
}

impl BridgeQuoteResponse {
    pub fn planning_only(
        from: String,
        to: String,
        route_summary: Vec<String>,
        warning: Option<String>,
    ) -> Self {
        Self {
            from,
            to,
            to_amount_estimate: None,
            min_received: None,
            bridge: "planning-only".to_string(),
            estimated_duration_seconds: None,
            estimated_fee_usd: None,
            route_summary,
            executable_tx: None,
            execution_supported: false,
            warning,
        }
    }

    pub fn from_lifi_quote(
        quote: &LifiQuoteResponse,
        from: String,
        to: String,
        to_decimals: u8,
    ) -> Self {
        let executable_tx = quote
            .transaction_request
            .as_ref()
            .map(LifiTransactionRequest::to_executable_transaction);

        Self {
            from,
            to,
            to_amount_estimate: quote
                .estimate
                .to_amount
                .as_deref()
                .and_then(|value| format_base_units(value, to_decimals)),
            min_received: quote
                .estimate
                .to_amount_min
                .as_deref()
                .and_then(|value| format_base_units(value, to_decimals)),
            bridge: quote.bridge_name(),
            estimated_duration_seconds: quote.estimate.execution_duration,
            estimated_fee_usd: quote.total_fee_usd(),
            route_summary: quote.route_summary(),
            execution_supported: executable_tx.is_some(),
            executable_tx,
            warning: None,
        }
    }
}

fn format_base_units(value: &str, decimals: u8) -> Option<String> {
    value
        .parse::<f64>()
        .ok()
        .map(|raw| raw / 10f64.powi(decimals as i32))
        .map(|amount| format!("{amount:.6}"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn bridge_quote_response_uses_typed_lifi_quote_shape() {
        let quote: LifiQuoteResponse = serde_json::from_value(json!({
            "type": "lifi",
            "tool": "across",
            "toolDetails": {"name": "Across"},
            "action": {
                "fromToken": {"address": "0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48"}
            },
            "estimate": {
                "toAmount": "9900000",
                "toAmountMin": "9800000",
                "executionDuration": 420,
                "approvalAddress": "0x1111111111111111111111111111111111111111",
                "feeCosts": [{"amountUSD": "0.12"}],
                "gasCosts": [{"amountUSD": "1.38"}]
            },
            "includedSteps": [
                {"tool": "across"},
                {"tool": "uniswap"}
            ],
            "transactionRequest": {
                "to": "0x2222222222222222222222222222222222222222",
                "data": "0xdeadbeef",
                "value": "0x0",
                "gasLimit": "0x12345"
            }
        }))
        .expect("quote should deserialize");

        let response = BridgeQuoteResponse::from_lifi_quote(
            &quote,
            "1 USDC on ethereum".to_string(),
            "USDC on polygon".to_string(),
            6,
        );

        assert_eq!(response.bridge, "Across");
        assert_eq!(response.to_amount_estimate.as_deref(), Some("9.900000"));
        assert_eq!(response.min_received.as_deref(), Some("9.800000"));
        assert_eq!(response.estimated_fee_usd.as_deref(), Some("1.50"));
        assert_eq!(response.route_summary, vec!["across", "uniswap"]);
        assert!(response.execution_supported);
        assert_eq!(
            response
                .executable_tx
                .as_ref()
                .and_then(|tx| tx.gas_limit.as_str()),
            Some("0x12345")
        );
    }
}
