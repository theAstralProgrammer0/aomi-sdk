use serde::Serialize;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PlaceOrderRequest<'a> {
    pub inst_id: &'a str,
    pub td_mode: &'a str,
    pub side: &'a str,
    pub ord_type: &'a str,
    pub sz: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub px: Option<&'a str>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelOrderRequest<'a> {
    pub inst_id: &'a str,
    pub ord_id: &'a str,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SetLeverageRequest<'a> {
    pub inst_id: &'a str,
    pub lever: &'a str,
    pub mgn_mode: &'a str,
}
