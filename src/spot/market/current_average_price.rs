use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{rest_api::endpoint, web_socket_api::web_socket};

endpoint!(
    "/api/v3/avgPrice",
    Method::GET,
    CurrentAveragePriceEndpoint,
    CurrentAveragePriceParams,
    CurrentAveragePriceResponse
);

/// Current average price for a symbol.
///
/// - Weight: 2
/// - Data Source: Memory
pub struct CurrentAveragePriceEndpoint<'r> {
    client: &'r crate::rest_api::RestApiClient,
}

impl<'r> CurrentAveragePriceEndpoint<'r> {
    pub fn new(client: &'r crate::rest_api::RestApiClient) -> Self {
        Self { client }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CurrentAveragePriceParams {
    symbol: String,
}

impl CurrentAveragePriceParams {
    pub fn new(symbol: &str) -> Self {
        Self {
            symbol: symbol.to_owned(),
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CurrentAveragePriceResponse {
    pub mins: i64,
    pub price: String,
    pub close_time: i64,
}

web_socket!(
    "avgPrice",
    CurrentAveragePriceWebSocket,
    CurrentAveragePriceParams,
    CurrentAveragePriceResponse
);

pub struct CurrentAveragePriceWebSocket<'w> {
    client: &'w crate::web_socket_api::WebSocketApiClient,
}

impl<'w> CurrentAveragePriceWebSocket<'w> {
    pub fn new(client: &'w crate::web_socket_api::WebSocketApiClient) -> Self {
        Self { client }
    }
}
