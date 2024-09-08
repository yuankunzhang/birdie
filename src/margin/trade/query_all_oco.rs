use jiff::Timestamp;
use reqwest::Method;
use serde::Serialize;

use crate::{enums::SecurityType, rest_api::endpoint};

use super::MarginOcoOrder;

endpoint!(
    "/sapi/v1/margin/allOrderList",
    Method::GET,
    SecurityType::UserData,
    QueryAllOcoEndpoint,
    QueryAllOcoParams,
    QueryAllOcoResponse
);

/// Retrieves all OCO for a specific margin account based on provided optional
/// parameters.
///
/// - Weight: 200
pub struct QueryAllOcoEndpoint<'r> {
    client: &'r crate::rest_api::RestApiClient,
}

impl<'r> QueryAllOcoEndpoint<'r> {
    pub fn new(client: &'r crate::rest_api::RestApiClient) -> Self {
        Self { client }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct QueryAllOcoParams {
    symbol: Option<String>,
    is_isolated: Option<String>,
    from_id: Option<i64>,
    start_time: Option<i64>,
    end_time: Option<i64>,
    limit: Option<i64>,
    recv_window: Option<i64>,
    timestamp: i64,
}

impl QueryAllOcoParams {
    pub fn new() -> Self {
        Self {
            symbol: None,
            is_isolated: None,
            from_id: None,
            start_time: None,
            end_time: None,
            limit: None,
            recv_window: None,
            timestamp: Timestamp::now().as_millisecond(),
        }
    }

    pub fn symbol(mut self, symbol: &str) -> Self {
        self.symbol = Some(symbol.to_owned());
        self
    }

    pub fn is_isolated(mut self, is_isolated: &str) -> Self {
        self.is_isolated = Some(is_isolated.to_owned());
        self
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

    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }

    pub fn recv_window(mut self, recv_window: i64) -> Self {
        self.recv_window = Some(recv_window);
        self
    }
}

pub type QueryAllOcoResponse = Vec<MarginOcoOrder>;
