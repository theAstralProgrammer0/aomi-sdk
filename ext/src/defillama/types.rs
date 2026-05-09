use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct ExcludedChartsQuery {
    #[serde(rename = "excludeTotalDataChart")]
    pub exclude_total_data_chart: bool,
    #[serde(rename = "excludeTotalDataChartBreakdown")]
    pub exclude_total_data_chart_breakdown: bool,
}

#[derive(Debug, Clone, Serialize)]
pub struct FeesOverviewQuery {
    #[serde(rename = "excludeTotalDataChart")]
    pub exclude_total_data_chart: bool,
    #[serde(rename = "excludeTotalDataChartBreakdown")]
    pub exclude_total_data_chart_breakdown: bool,
    #[serde(rename = "dataType", skip_serializing_if = "Option::is_none")]
    pub data_type: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct ProtocolFeesQuery {
    #[serde(rename = "dataType", skip_serializing_if = "Option::is_none")]
    pub data_type: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct IncludePricesQuery {
    #[serde(rename = "includePrices")]
    pub include_prices: bool,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct HistoricalTokenPriceQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub span: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub period: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct TokenPriceChangeQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<u64>,
    #[serde(rename = "lookForward", skip_serializing_if = "Option::is_none")]
    pub look_forward: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub period: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct StablecoinHistoryQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stablecoin: Option<u64>,
}
