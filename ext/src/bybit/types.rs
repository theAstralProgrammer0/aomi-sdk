use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateOrderRequest<'a> {
    pub category: &'a str,
    pub symbol: &'a str,
    pub side: &'a str,
    pub order_type: &'a str,
    pub qty: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<&'a str>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelOrderRequest<'a> {
    pub category: &'a str,
    pub symbol: &'a str,
    pub order_id: &'a str,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AmendOrderRequest<'a> {
    pub category: &'a str,
    pub symbol: &'a str,
    pub order_id: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qty: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<&'a str>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SetLeverageRequest<'a> {
    pub category: &'a str,
    pub symbol: &'a str,
    pub buy_leverage: &'a str,
    pub sell_leverage: &'a str,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BybitResponse<T> {
    pub ret_code: i64,
    pub ret_msg: String,
    pub result: T,
    #[serde(default)]
    pub ret_ext_info: Value,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub time: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BybitTickerResult {
    pub category: String,
    pub list: Vec<BybitTicker>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BybitTicker {
    pub symbol: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_price: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub volume24h: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bid1_price: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ask1_price: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BybitOrderbookResult {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub s: Option<String>,
    #[serde(default)]
    pub b: Vec<BybitOrderbookLevel>,
    #[serde(default)]
    pub a: Vec<BybitOrderbookLevel>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct BybitOrderbookLevel(pub Vec<String>);

impl BybitOrderbookLevel {
    pub fn price(&self) -> Option<&str> {
        self.0.first().map(String::as_str)
    }

    pub fn qty(&self) -> Option<&str> {
        self.0.get(1).map(String::as_str)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BybitKlineResult {
    pub category: String,
    pub symbol: String,
    #[serde(default)]
    pub list: Vec<BybitKlineCandle>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct BybitKlineCandle(pub Vec<String>);

impl BybitKlineCandle {
    pub fn open_time(&self) -> Option<&str> {
        self.0.first().map(String::as_str)
    }

    pub fn open(&self) -> Option<&str> {
        self.0.get(1).map(String::as_str)
    }

    pub fn high(&self) -> Option<&str> {
        self.0.get(2).map(String::as_str)
    }

    pub fn low(&self) -> Option<&str> {
        self.0.get(3).map(String::as_str)
    }

    pub fn close(&self) -> Option<&str> {
        self.0.get(4).map(String::as_str)
    }

    pub fn volume(&self) -> Option<&str> {
        self.0.get(5).map(String::as_str)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct BybitActionResult {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub order_link_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub list: Option<Vec<Value>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BybitPositionListResult {
    pub category: String,
    #[serde(default)]
    pub list: Vec<BybitPosition>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BybitPosition {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub side: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub avg_price: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BybitWalletBalanceResult {
    #[serde(default)]
    pub list: Vec<BybitWalletBalanceAccount>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BybitWalletBalanceAccount {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub account_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total_equity: Option<String>,
    #[serde(default)]
    pub coin: Vec<BybitWalletCoinBalance>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BybitWalletCoinBalance {
    pub coin: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub wallet_balance: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub usd_value: Option<String>,
}
