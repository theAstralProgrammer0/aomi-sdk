use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct AccountQuery<'a> {
    pub account: &'a str,
}
