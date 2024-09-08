use jiff::Timestamp;
use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{
    enums::{RateLimit, SecurityType},
    rest_api::endpoint,
};

endpoint!(
    "/sapi/v1/margin/rateLimit/order",
    Method::GET,
    SecurityType::Trade,
    QueryCurrentMarginOrderCountUsageEndpoint,
    QueryCurrentMarginOrderCountUsageParams,
    QueryCurrentMarginOrderCountUsageResponse
);

/// Displays the user's current margin order count usage for all intervals.
///
/// - Weight: 20
pub struct QueryCurrentMarginOrderCountUsageEndpoint<'r> {
    client: &'r crate::rest_api::RestApiClient,
}

impl<'r> QueryCurrentMarginOrderCountUsageEndpoint<'r> {
    pub fn new(client: &'r crate::rest_api::RestApiClient) -> Self {
        Self { client }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct QueryCurrentMarginOrderCountUsageParams {
    symbol: Option<String>,
    is_isolated: Option<String>,
    recv_window: Option<i64>,
    timestamp: i64,
}

impl QueryCurrentMarginOrderCountUsageParams {
    pub fn new() -> Self {
        Self {
            symbol: None,
            is_isolated: None,
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

    pub fn recv_window(mut self, recv_window: i64) -> Self {
        self.recv_window = Some(recv_window);
        self
    }
}

pub type QueryCurrentMarginOrderCountUsageResponse = Vec<CurrentMarginOrderCountUsage>;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CurrentMarginOrderCountUsage {
    #[serde(flatten)]
    pub rate_limit: RateLimit,
    pub count: i64,
}
