use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::rest_api::endpoint;

endpoint!(
    "/fapi/v1/premiumIndex",
    Method::GET,
    MarkPriceEndpoint,
    MarkPriceParams,
    MarkPriceResponse
);

/// Mark Price and Funding Rate.
///
/// - Weight: 1
pub struct MarkPriceEndpoint<'r> {
    client: &'r crate::rest_api::RestApiClient,
}

impl<'r> MarkPriceEndpoint<'r> {
    pub fn new(client: &'r crate::rest_api::RestApiClient) -> Self {
        Self { client }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MarkPriceParams {
    pub symbol: Option<String>,
}

impl MarkPriceParams {
    pub fn new() -> Self {
        MarkPriceParams { symbol: None }
    }

    pub fn symbol(mut self, symbol: String) -> Self {
        self.symbol = Some(symbol);
        self
    }
}

#[derive(Debug, Deserialize)]
pub enum MarkPriceResponse {
    Vec(Box<MarkPriceResult>),
    Item(Box<MarkPriceResult>),
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MarkPriceResult {
    pub symbol: String,
    pub mark_price: String,
    pub index_price: String,
    pub estimated_settle_price: String,
    pub last_funding_rate: String,
    pub next_funding_time: i64,
    pub interest_rate: String,
    pub time: i64,
}
