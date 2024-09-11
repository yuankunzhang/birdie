use jiff::Timestamp;
use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{enums::SecurityType, errors::BinanceError, rest_api::endpoint};

endpoint!(
    "/fapi/v1/allOpenOrders",
    Method::DELETE,
    SecurityType::Trade,
    CancelAllOpenOrdersEndpoint,
    CancelAllOpenOrdersParams,
    CancelAllOpenOrdersResponse
);

/// Cancel All Open Orders.
///
/// - Weight: 1
pub struct CancelAllOpenOrdersEndpoint<'r> {
    client: &'r crate::rest_api::RestApiClient,
}

impl<'r> CancelAllOpenOrdersEndpoint<'r> {
    pub fn new(client: &'r crate::rest_api::RestApiClient) -> Self {
        Self { client }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelAllOpenOrdersParams {
    symbol: String,
    recv_window: Option<i64>,
    timestamp: i64,
}

impl CancelAllOpenOrdersParams {
    pub fn new(symbol: &str) -> Self {
        Self {
            symbol: symbol.to_owned(),
            recv_window: None,
            timestamp: Timestamp::now().as_millisecond(),
        }
    }

    pub fn recv_window(mut self, recv_window: i64) -> Self {
        self.recv_window = Some(recv_window);
        self
    }
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum CancelAllOpenOrdersResponse {
    Success,
    Failure(BinanceError),
}
