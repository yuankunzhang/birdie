use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::rest_api::endpoint;

endpoint!(
    "/fapi/v1/ticker/24hr",
    Method::GET,
    Ticker24hrEndpoint,
    Ticker24hrParams,
    Ticker24hrResponse
);

/// 24 hour rolling window price change statistics. Careful when accessing this
/// with no symbol.
///
/// - Weight:
/// - Data Source: Memory
pub struct Ticker24hrEndpoint<'r> {
    client: &'r crate::rest_api::RestApiClient,
}

impl<'r> Ticker24hrEndpoint<'r> {
    pub fn new(client: &'r crate::rest_api::RestApiClient) -> Self {
        Self { client }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Ticker24hrParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    symbol: Option<String>,
}

impl Default for Ticker24hrParams {
    fn default() -> Self {
        Self::new()
    }
}

impl Ticker24hrParams {
    pub fn new() -> Self {
        Self { symbol: None }
    }

    pub fn symbol(mut self, symbol: &str) -> Self {
        self.symbol = Some(symbol.to_owned());
        self
    }
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum Ticker24hrResponse {
    Vec(Box<Ticker>),
    Item(Box<Ticker>),
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Ticker {
    pub symbol: String,
    pub price_change: String,
    pub price_change_percent: String,
    pub weighted_avg_price: String,
    pub last_price: String,
    pub last_qty: String,
    pub open_price: String,
    pub high_price: String,
    pub low_price: String,
    pub volume: String,
    pub quote_volume: String,
    pub open_time: i64,
    pub close_time: i64,
    pub first_id: i64,
    pub last_id: i64,
    pub count: i64,
}
