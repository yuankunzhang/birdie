use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::rest_api::endpoint;

endpoint!(
    "/fapi/v1/fundingInfo",
    Method::GET,
    GetFundingInfoEndpoint,
    GetFundingInfoParams,
    GetFundingInfoResponse
);

/// Test connectivity to the Rest API and get the current server time.
///
/// - Weight: 1
pub struct GetFundingInfoEndpoint<'r> {
    client: &'r crate::rest_api::RestApiClient,
}

impl<'r> GetFundingInfoEndpoint<'r> {
    pub fn new(client: &'r crate::rest_api::RestApiClient) -> Self {
        Self { client }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetFundingInfoParams {}

impl Default for GetFundingInfoParams {
    fn default() -> Self {
        GetFundingInfoParams::new()
    }
}

impl GetFundingInfoParams {
    pub fn new() -> Self {
        GetFundingInfoParams {}
    }
}

pub type GetFundingInfoResponse = Vec<FundingInfo>;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FundingInfo {
    pub symbol: String,
    pub adjusted_funding_rate_cap: String,
    pub adjusted_funding_rate_floor: String,
    pub funding_interval_hours: i64,
    pub disclaimer: bool,
}
