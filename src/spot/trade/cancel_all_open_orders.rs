use jiff::Timestamp;
use reqwest::Method;
use serde::Serialize;

use crate::{enums::SecurityType, rest_api::endpoint};

use super::CancelOrderResult;

endpoint!(
    "/api/v3/openOrders",
    Method::DELETE,
    SecurityType::Trade,
    CancelAllOpenOrdersEndpoint,
    CancelAllOpenOrdersParams,
    CancelAllOpenOrdersResponse
);

/// Cancel an active order.
///
/// - Weight: 1
/// - Data Source: Matching Engine
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
    #[serde(skip_serializing_if = "Option::is_none")]
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

    /// The value cannot be greater than 60000.
    pub fn recv_window(mut self, recv_window: i64) -> Self {
        self.recv_window = Some(recv_window);
        self
    }
}

pub type CancelAllOpenOrdersResponse = Vec<CancelOrderResult>;
