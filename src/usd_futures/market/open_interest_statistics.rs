use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::rest_api::endpoint;

endpoint!(
    "/futures/data/openInterestHist",
    Method::GET,
    OpenInterestStatisticsEndpoint,
    OpenInterestStatisticsParams,
    OpenInterestStatisticsResponse
);

/// Open Interest Statistics.
///
/// - Weight: 0
pub struct OpenInterestStatisticsEndpoint<'r> {
    client: &'r crate::rest_api::RestApiClient,
}

impl<'r> OpenInterestStatisticsEndpoint<'r> {
    pub fn new(client: &'r crate::rest_api::RestApiClient) -> Self {
        Self { client }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OpenInterestStatisticsParams {
    pub symbol: String,
    pub period: String,
    pub limit: Option<i64>,
    pub start_time: Option<i64>,
    pub end_time: Option<i64>,
}

impl OpenInterestStatisticsParams {
    pub fn new(symbol: &str, period: &str) -> Self {
        OpenInterestStatisticsParams {
            symbol: symbol.to_owned(),
            period: period.to_owned(),
            limit: None,
            start_time: None,
            end_time: None,
        }
    }

    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
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
}

pub type OpenInterestStatisticsResponse = Vec<OpenInterestStatistics>;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OpenInterestStatistics {
    pub symbol: String,
    pub sub_open_interest: String,
    pub sum_open_interest_value: String,
    pub timestamp: i64,
}
