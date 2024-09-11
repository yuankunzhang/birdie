use jiff::Timestamp;
use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{enums::SecurityType, rest_api::endpoint};

endpoint!(
    "/fapi/v1/countdownCancelAll",
    Method::POST,
    SecurityType::Trade,
    AutoCancelAllOpenOrdersEndpoint,
    AutoCancelAllOpenOrdersParams,
    AutoCancelAllOpenOrdersResponse
);

/// Cancel all open orders of the specified symbol at the end of the specified
/// countdown. The endpoint should be called repeatedly as heartbeats so that the
/// existing countdown time can be canceled and replaced by a new one.
///
/// - Weight: 1
pub struct AutoCancelAllOpenOrdersEndpoint<'r> {
    client: &'r crate::rest_api::RestApiClient,
}

impl<'r> AutoCancelAllOpenOrdersEndpoint<'r> {
    pub fn new(client: &'r crate::rest_api::RestApiClient) -> Self {
        Self { client }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AutoCancelAllOpenOrdersParams {
    symbol: String,
    countdown_time: i64,
    recv_window: Option<i64>,
    timestamp: i64,
}

impl AutoCancelAllOpenOrdersParams {
    pub fn new(symbol: &str, countdown_time: i64) -> Self {
        Self {
            symbol: symbol.to_owned(),
            countdown_time,
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
#[serde(rename_all = "camelCase")]
pub struct AutoCancelAllOpenOrdersResponse {
    pub symbol: String,
    pub countdown_time: i64,
}
