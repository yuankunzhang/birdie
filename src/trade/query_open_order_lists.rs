use jiff::Timestamp;
use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::rest_api::{endpoint, SecurityType};

endpoint!(
    "/api/v3/openOrderList",
    Method::GET,
    SecurityType::UserData,
    QueryOpenOrderListsEndpoint,
    QueryOpenOrderListsParams,
    QueryOpenOrderListsResponse
);

/// Query open order lists.
///
/// - Weight: 6
/// - Data Source: Database
pub struct QueryOpenOrderListsEndpoint<'r> {
    client: &'r crate::rest_api::RestApiClient,
}

impl<'r> QueryOpenOrderListsEndpoint<'r> {
    pub fn new(client: &'r crate::rest_api::RestApiClient) -> Self {
        Self { client }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct QueryOpenOrderListsParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
    timestamp: i64,
}

impl QueryOpenOrderListsParams {
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

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QueryOpenOrderListsResponse;
