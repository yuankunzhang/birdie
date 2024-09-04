use jiff::Timestamp;
use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::rest_api::{endpoint, SecurityType};

endpoint!(
    "/api/v3/allOrderList",
    Method::GET,
    SecurityType::UserData,
    QueryAllOrderListsEndpoint,
    QueryAllOrderListsParams,
    QueryAllOrderListsResponse
);

/// Retrieves all order lists based on provided optional parameters.
///
/// Note that the time between `start_time` and `end_time` can't be longer than
/// 24 hours.
///
/// - Weight: 20
/// - Data Source: Database
pub struct QueryAllOrderListsEndpoint<'r> {
    client: &'r crate::rest_api::RestApiClient,
}

impl<'r> QueryAllOrderListsEndpoint<'r> {
    pub fn new(client: &'r crate::rest_api::RestApiClient) -> Self {
        Self { client }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct QueryAllOrderListsParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    from_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    start_time: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    end_time: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
    timestamp: i64,
}

impl QueryAllOrderListsParams {
    pub fn new() -> Self {
        Self {
            from_id: None,
            start_time: None,
            end_time: None,
            limit: None,
            recv_window: None,
            timestamp: Timestamp::now().as_millisecond(),
        }
    }

    pub fn from_id(mut self, from_id: i64) -> Self {
        self.from_id = Some(from_id);
        self
    }

    pub fn start_time(mut self, start_time: i64) -> Self {
        self.start_time = Some(start_time);
        self
    }

    pub fn end_time(mut self, end_time: i64) -> Self {
        self.end_time = Some(end_time);
        self
    }

    /// Default Value: 500; Max Value: 1000.
    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }

    /// The value cannot be greater than 60000.
    pub fn recv_window(mut self, recv_window: i64) -> Self {
        self.recv_window = Some(recv_window);
        self
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QueryAllOrderListsResponse;
