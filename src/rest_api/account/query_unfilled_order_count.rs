use jiff::Timestamp;
use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{
    enums::RateLimit,
    rest_api::{endpoint, SecurityType},
};

endpoint!(
    "/api/v3/rateLimit/order",
    Method::GET,
    SecurityType::UserData,
    QueryUnfilledOrderCountEndpoint,
    QueryUnfilledOrderCountParams,
    QueryUnfilledOrderCountResponse
);

/// Displays the user's unfilled order count for all intervals.
///
/// - Weight: 40
/// - Data Source: Memory
pub struct QueryUnfilledOrderCountEndpoint<'r> {
    client: &'r crate::rest_api::RestClient,
}

impl<'r> QueryUnfilledOrderCountEndpoint<'r> {
    pub fn new(client: &'r crate::rest_api::RestClient) -> Self {
        Self { client }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct QueryUnfilledOrderCountParams {
    /// The value cannot be greater than `60000`.
    #[serde(skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
    timestamp: i64,
}

impl Default for QueryUnfilledOrderCountParams {
    fn default() -> Self {
        Self::new()
    }
}

impl QueryUnfilledOrderCountParams {
    pub fn new() -> Self {
        Self {
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

pub type QueryUnfilledOrderCountResponse = Vec<UnfilledOrderCount>;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UnfilledOrderCount {
    #[serde(flatten)]
    pub rate_limit: RateLimit,
    pub count: i64,
}
