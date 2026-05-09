use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BinanceSymbolPrice {
    pub symbol: String,
    pub price: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BinancePriceResponse {
    Single(BinanceSymbolPrice),
    Many(Vec<BinanceSymbolPrice>),
}

impl BinancePriceResponse {
    pub fn first(&self) -> Option<&BinanceSymbolPrice> {
        match self {
            Self::Single(item) => Some(item),
            Self::Many(items) => items.first(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BinanceDepthResponse {
    #[serde(rename = "lastUpdateId")]
    pub last_update_id: u64,
    pub bids: Vec<BinanceOrderBookLevel>,
    pub asks: Vec<BinanceOrderBookLevel>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct BinanceOrderBookLevel(pub Vec<String>);

impl BinanceOrderBookLevel {
    pub fn price(&self) -> Option<&str> {
        self.0.first().map(String::as_str)
    }

    pub fn qty(&self) -> Option<&str> {
        self.0.get(1).map(String::as_str)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct BinanceKline(pub Vec<Value>);

impl BinanceKline {
    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn open(&self) -> Option<&str> {
        self.0.get(1).and_then(Value::as_str)
    }

    pub fn high(&self) -> Option<&str> {
        self.0.get(2).and_then(Value::as_str)
    }

    pub fn low(&self) -> Option<&str> {
        self.0.get(3).and_then(Value::as_str)
    }

    pub fn close(&self) -> Option<&str> {
        self.0.get(4).and_then(Value::as_str)
    }

    pub fn volume(&self) -> Option<&str> {
        self.0.get(5).and_then(Value::as_str)
    }
}

pub type BinanceKlineResponse = Vec<BinanceKline>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Binance24hrStats {
    pub symbol: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub price_change_percent: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_price: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub high_price: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub low_price: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub volume: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Binance24hrStatsResponse {
    Single(Binance24hrStats),
    Many(Vec<Binance24hrStats>),
}

impl Binance24hrStatsResponse {
    pub fn first(&self) -> Option<&Binance24hrStats> {
        match self {
            Self::Single(item) => Some(item),
            Self::Many(items) => items.first(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct BinanceOrderResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub order_id: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BinanceAccountResponse {
    #[serde(default)]
    pub balances: Vec<BinanceBalance>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BinanceBalance {
    pub asset: String,
    pub free: String,
    pub locked: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BinanceTrade {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub price: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub qty: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub quote_qty: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub time: Option<u64>,
}

pub type BinanceTradeList = Vec<BinanceTrade>;
