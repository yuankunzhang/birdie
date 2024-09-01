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
