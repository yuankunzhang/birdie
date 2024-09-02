use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::rest::endpoint;

endpoint!(
    "/api/v3/depth",
    Method::GET,
    OrderBookEndpoint,
    OrderBookParams,
    OrderBookResponse
);

/// Get order book.
///
/// - Weight: Adjust based on the limit.
///     - Limit 1-100: 5
///     - Limit 101-500: 25
///     - Limit 501-1000: 50
///     - Limit 1001-5000: 250
/// - Data Source: Memory
pub struct OrderBookEndpoint<'r> {
    client: &'r crate::rest::RestClient,
}

impl<'r> OrderBookEndpoint<'r> {
    pub fn new(client: &'r crate::rest::RestClient) -> Self {
        Self { client }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderBookParams {
    symbol: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<u64>,
}

impl OrderBookParams {
    pub fn new(symbol: &str) -> Self {
        Self {
            symbol: symbol.to_string(),
            limit: None,
        }
    }

    pub fn limit(mut self, limit: u64) -> Self {
        self.limit = Some(limit);
        self
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderBookResponse {
    pub last_update_id: u64,
    pub bids: Vec<(String, String)>,
    pub asks: Vec<(String, String)>,
}
