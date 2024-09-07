use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{rest_api::endpoint, web_socket_api::web_socket};

endpoint!(
    "/api/v3/trades",
    Method::GET,
    RecentTradesListEndpoint,
    RecentTradesListParams,
    RecentTradesListResponse
);

/// Get recent trades.
///
/// - Weight: 25
/// - Data Source: Memory
pub struct RecentTradesListEndpoint<'r> {
    client: &'r crate::rest_api::RestApiClient,
}

impl<'r> RecentTradesListEndpoint<'r> {
    pub fn new(client: &'r crate::rest_api::RestApiClient) -> Self {
        Self { client }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RecentTradesListParams {
    symbol: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<i64>,
}

impl RecentTradesListParams {
    pub fn new(symbol: &str) -> Self {
        Self {
            symbol: symbol.to_string(),
            limit: None,
        }
    }

    /// Default 500; max 1000.
    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }
}

pub type RecentTradesListResponse = Vec<RecentTrade>;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RecentTrade {
    pub id: i64,
    pub price: String,
    pub qty: String,
    pub quote_qty: String,
    pub time: i64,
    pub is_buyer_maker: bool,
    pub is_best_match: bool,
}

web_socket!(
    "trades.recent",
    RecentTradesListWebSocket,
    RecentTradesListParams,
    RecentTradesListResponse
);

pub struct RecentTradesListWebSocket<'w> {
    client: &'w crate::web_socket_api::WebSocketApiClient,
}

impl<'w> RecentTradesListWebSocket<'w> {
    pub fn new(client: &'w crate::web_socket_api::WebSocketApiClient) -> Self {
        Self { client }
    }
}
