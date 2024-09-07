use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{rest_api::endpoint, web_socket_api::web_socket};

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

    /// Default 100; max 5000. If limit > 5000, then the response will truncate
    /// to 5000.
    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderBookResponse {
    pub last_update_id: i64,
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
