use jiff::Timestamp;
use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{enums::SecurityType, rest_api::endpoint};

endpoint!(
    "/sapi/v1/margin/next-hourly-interest-rate",
    Method::GET,
    SecurityType::UserData,
    GetFutureHourlyInterestRateEndpoint,
    GetFutureHourlyInterestRateParams,
    GetFutureHourlyInterestRateResponse
);

/// Get future hourly interest rate.
///
/// - Weight: 100
pub struct GetFutureHourlyInterestRateEndpoint<'r> {
    client: &'r crate::rest_api::RestApiClient,
}

impl<'r> GetFutureHourlyInterestRateEndpoint<'r> {
    pub fn new(client: &'r crate::rest_api::RestApiClient) -> Self {
        Self { client }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetFutureHourlyInterestRateParams {
    assets: String,
    is_isolated: String,
    timestamp: i64,
}

impl GetFutureHourlyInterestRateParams {
    pub fn new(assets: &str, is_isolated: &str) -> Self {
        Self {
            assets: assets.to_owned(),
            is_isolated: is_isolated.to_owned(),
            timestamp: Timestamp::now().as_millisecond(),
        }
    }
}

pub type GetFutureHourlyInterestRateResponse = Vec<FutureHourlyInterestRate>;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FutureHourlyInterestRate {
    pub asset: String,
    pub next_hourly_interest_rate: String,
}
