use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::rest_api::endpoint;

endpoint!(
    "/fapi/v1/fundingInfo",
    Method::GET,
    GetFundingRateHistoryEndpoint,
    GetFundingRateHistoryParams,
    GetFundingRateHistoryResponse
);

/// Get Funding Rate History.
///
/// - Weight: 0
pub struct GetFundingRateHistoryEndpoint<'r> {
    client: &'r crate::rest_api::RestApiClient,
}

impl<'r> GetFundingRateHistoryEndpoint<'r> {
    pub fn new(client: &'r crate::rest_api::RestApiClient) -> Self {
        Self { client }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetFundingRateHistoryParams {
    pub symbol: Option<String>,
    pub start_time: Option<i64>,
    pub end_time: Option<i64>,
    pub limit: Option<i64>,
}

impl Default for GetFundingRateHistoryParams {
    fn default() -> Self {
        GetFundingRateHistoryParams::new()
    }
}

impl GetFundingRateHistoryParams {
    pub fn new() -> Self {
        GetFundingRateHistoryParams {
            symbol: None,
            start_time: None,
            end_time: None,
            limit: None,
        }
    }

    pub fn symbol(mut self, symbol: &str) -> Self {
        self.symbol = Some(symbol.to_string());
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
}

pub type GetFundingRateHistoryResponse = Vec<FundingRate>;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FundingRate {
    pub symbol: String,
    pub funding_rate: String,
    pub funding_time: i64,
    pub mark_price: String,
}
