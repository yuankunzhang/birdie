use jiff::Timestamp;
use reqwest::Method;
use serde::Serialize;

use crate::{enums::SecurityType, rest_api::endpoint};

use super::OrderDetail;

endpoint!(
    "/fapi/v1/allOrders",
    Method::GET,
    SecurityType::Trade,
    QueryAllOrdersEndpoint,
    QueryAllOrdersParams,
    QueryAllOrdersResponse
);

/// Get all account orders; active, canceled, or filled.
///
/// - Weight: 5
pub struct QueryAllOrdersEndpoint<'r> {
    client: &'r crate::rest_api::RestApiClient,
}

impl<'r> QueryAllOrdersEndpoint<'r> {
    pub fn new(client: &'r crate::rest_api::RestApiClient) -> Self {
        Self { client }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct QueryAllOrdersParams {
    symbol: String,
    order_id: Option<i64>,
    start_time: Option<i64>,
    end_time: Option<i64>,
    limit: Option<i64>,
    recv_window: Option<i64>,
    timestamp: i64,
}

impl QueryAllOrdersParams {
    pub fn new(symbol: &str) -> Self {
        Self {
            symbol: symbol.to_owned(),
            order_id: None,
            start_time: None,
            end_time: None,
            limit: None,
            recv_window: None,
            timestamp: Timestamp::now().as_millisecond(),
        }
    }

    pub fn order_id(mut self, order_id: i64) -> Self {
        self.order_id = Some(order_id);
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

    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }

    pub fn recv_window(mut self, recv_window: i64) -> Self {
        self.recv_window = Some(recv_window);
        self
    }
}

pub type QueryAllOrdersResponse = Vec<OrderDetail>;
