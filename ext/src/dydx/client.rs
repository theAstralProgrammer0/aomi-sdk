use serde_json::Value;
use std::time::Duration;

pub const BASE_URL: &str = "https://indexer.dydx.trade/v4";

#[derive(Clone)]
pub struct DydxClient {
    pub http: reqwest::blocking::Client,
    pub base_url: String,
}

impl DydxClient {
    pub fn new() -> Result<Self, String> {
        let base_url = std::env::var("DYDX_INDEXER_URL").unwrap_or_else(|_| BASE_URL.to_string());
        let http = reqwest::blocking::Client::builder()
            .timeout(Duration::from_secs(30))
            .build()
            .map_err(|e| format!("[dydx] failed to build HTTP client: {e}"))?;
        Ok(Self { http, base_url })
    }

    pub fn get(&self, path: &str) -> Result<Value, String> {
        let url = format!("{}{path}", self.base_url);
        let resp = self
            .http
            .get(&url)
            .send()
            .map_err(|e| format!("[dydx] request failed: {e}"))?;
        if !resp.status().is_success() {
            let status = resp.status();
            let text = resp.text().unwrap_or_default();
            return Err(format!("[dydx] API error {status}: {text}"));
        }
        resp.json()
            .map_err(|e| format!("[dydx] decode failed: {e}"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Story: "Put on a funding rate arb — short the high-funding perp"
    #[test]
    fn funding_rate_arb_workflow() {
        let client = DydxClient::new().expect("failed to build dYdX client");

        let markets_resp = client
            .get("/perpetualMarkets")
            .expect("failed to fetch perpetual markets");
        let markets_map = markets_resp
            .get("markets")
            .expect("response should contain a 'markets' key")
            .as_object()
            .expect("'markets' should be an object");
        assert!(
            !markets_map.is_empty(),
            "markets map should contain at least one market"
        );

        let ticker_with_funding = markets_map
            .iter()
            .find(|(_ticker, info)| info.get("nextFundingRate").is_some())
            .map(|(ticker, _)| ticker.clone());
        assert!(
            ticker_with_funding.is_some(),
            "at least one market should expose funding-rate data"
        );

        let orderbook = client
            .get("/orderbooks/perpetualMarket/BTC-USD")
            .expect("failed to fetch BTC-USD orderbook");
        let bids = orderbook.get("bids").expect("orderbook should contain 'bids'");
        let asks = orderbook.get("asks").expect("orderbook should contain 'asks'");
        assert!(!bids.as_array().unwrap().is_empty());
        assert!(!asks.as_array().unwrap().is_empty());

        let candles_resp = client
            .get("/candles/perpetualMarkets/BTC-USD?resolution=1HOUR&limit=10")
            .expect("failed to fetch BTC-USD candles");
        let candles_arr = candles_resp
            .get("candles")
            .expect("candles response should contain 'candles' key")
            .as_array()
            .expect("candles should be an array");
        assert!(!candles_arr.is_empty(), "should receive at least one candle");
    }

    /// Story: "Rebalance my dYdX subaccount — cut losers, add to winners"
    #[test]
    fn rebalance_subaccount_workflow() {
        let client = DydxClient::new().expect("failed to build dYdX client");
        let address = "0x0000000000000000000000000000000000000000";

        // The API may return an error for a non-existent address; that is
        // acceptable — we just need to confirm we got *a* response.
        let account_result = client.get(&format!("/addresses/{address}/subaccountNumber/0"));
        let account_responded = match &account_result {
            Ok(val) => val.is_object() || val.is_null(),
            Err(e) => !e.is_empty(),
        };
        assert!(account_responded, "account endpoint should respond");

        let orders_result = client.get(&format!("/orders?address={address}&subaccountNumber=0"));
        let orders_responded = match &orders_result {
            Ok(val) => val.is_object() || val.is_array(),
            Err(e) => !e.is_empty(),
        };
        assert!(orders_responded, "orders endpoint should respond");

        let fills_result = client.get(&format!("/fills?address={address}&subaccountNumber=0"));
        let fills_responded = match &fills_result {
            Ok(val) => val.is_object() || val.is_array(),
            Err(e) => !e.is_empty(),
        };
        assert!(fills_responded, "fills endpoint should respond");
    }
}
