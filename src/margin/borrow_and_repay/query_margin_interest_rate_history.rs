use jiff::Timestamp;
use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{enums::SecurityType, rest_api::endpoint};

endpoint!(
    "/sapi/v1/margin/interestRateHistory",
    Method::GET,
    SecurityType::UserData,
    QueryMarginInterestRateHistoryEndpoint,
    QueryMarginInterestRateHistoryParams,
    QueryMarginInterestRateHistoryResponse
);

/// Query Margin Interest Rate History.
///
/// - Weight: 1
pub struct QueryMarginInterestRateHistoryEndpoint<'r> {
    client: &'r crate::rest_api::RestApiClient,
}

impl<'r> QueryMarginInterestRateHistoryEndpoint<'r> {
    pub fn new(client: &'r crate::rest_api::RestApiClient) -> Self {
        Self { client }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct QueryMarginInterestRateHistoryParams {
    asset: String,
    vip_level: Option<i64>,
    start_time: Option<i64>,
    end_time: Option<i64>,
    recv_window: Option<i64>,
    timestamp: i64,
}

impl QueryMarginInterestRateHistoryParams {
    pub fn new(asset: &str) -> Self {
        Self {
            asset: asset.to_owned(),
            vip_level: None,
            start_time: None,
            end_time: None,
            recv_window: None,
            timestamp: Timestamp::now().as_millisecond(),
        }
    }

    pub fn vip_level(mut self, vip_level: i64) -> Self {
        self.vip_level = Some(vip_level);
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

    pub fn recv_window(mut self, recv_window: i64) -> Self {
        self.recv_window = Some(recv_window);
        self
    }
}

pub type QueryMarginInterestRateHistoryResponse = Vec<InterestRateHistory>;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InterestRateHistory {
    pub asset: String,
    pub daily_interest_rate: String,
    pub timestamp: i64,
    pub vip_level: i64,
}
