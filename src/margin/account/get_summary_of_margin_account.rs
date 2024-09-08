use jiff::Timestamp;
use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{enums::SecurityType, rest_api::endpoint};

endpoint!(
    "/sapi/v1/margin/tradeCoeff",
    Method::GET,
    SecurityType::UserData,
    GetSummaryOfMarginAccountEndpoint,
    GetSummaryOfMarginAccountParams,
    GetSummaryOfMarginAccountResponse
);

/// Get personal margin level information.
///
/// - Weight: 10
pub struct GetSummaryOfMarginAccountEndpoint<'r> {
    client: &'r crate::rest_api::RestApiClient,
}

impl<'r> GetSummaryOfMarginAccountEndpoint<'r> {
    pub fn new(client: &'r crate::rest_api::RestApiClient) -> Self {
        Self { client }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetSummaryOfMarginAccountParams {
    recv_window: Option<i64>,
    timestamp: i64,
}

impl GetSummaryOfMarginAccountParams {
    pub fn new() -> Self {
        Self {
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
pub struct GetSummaryOfMarginAccountResponse {
    pub normal_bar: String,
    pub margin_call_bar: String,
    pub force_liquidation_bar: String,
}
