use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{rest_api::endpoint, web_socket_api::web_socket};

endpoint!(
    "/fapi/v1/depth",
    Method::GET,
    OrderBookEndpoint,
    OrderBookParams,
    OrderBookResponse
);

/// Query symbol orderbook.
///
/// - Weight: Adjust based on the limit.
///     - Limit 5,10,20,50: 2
///     - Limit 100: 5
///     - Limit 500: 10
///     - Limit 1000: 20
pub struct OrderBookEndpoint<'r> {
    client: &'r crate::rest_api::RestApiClient,
}

impl<'r> OrderBookEndpoint<'r> {
    pub fn new(client: &'r crate::rest_api::RestApiClient) -> Self {
        Self { client }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderBookParams {
    symbol: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<i64>,
}

impl OrderBookParams {
    pub fn new(symbol: &str) -> Self {
        Self {
            symbol: symbol.to_string(),
            limit: None,
        }
    }

    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }
}

#[derive(Debug, Deserialize)]
pub struct OrderBookResponse {
    #[serde(rename = "lastUpdateId")]
    pub last_update_id: i64,
    #[serde(rename = "E")]
    pub message_output_time: i64,
    #[serde(rename = "T")]
    pub transaction_time: i64,
    pub bids: Vec<(String, String)>,
    pub asks: Vec<(String, String)>,
}

web_socket!(
    "depth",
    OrderBookWebSocket,
    OrderBookParams,
    OrderBookResponse
);

pub struct OrderBookWebSocket<'w> {
    client: &'w crate::web_socket_api::WebSocketApiClient,
}

impl<'w> OrderBookWebSocket<'w> {
    pub fn new(client: &'w crate::web_socket_api::WebSocketApiClient) -> Self {
        Self { client }
    }
}
