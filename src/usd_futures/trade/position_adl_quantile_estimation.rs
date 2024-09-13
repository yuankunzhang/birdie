use jiff::Timestamp;
use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{enums::SecurityType, rest_api::endpoint};

endpoint!(
    "/fapi/v1/adlQuantile",
    Method::GET,
    SecurityType::UserData,
    PositionAdlQuantileEstimationEndpoint,
    PositionAdlQuantileEstimationParams,
    PositionAdlQuantileEstimationResponse
);

/// Position ADL Quantile Estimation.
///
/// - Weight: 5
pub struct PositionAdlQuantileEstimationEndpoint<'r> {
    client: &'r crate::rest_api::RestApiClient,
}

impl<'r> PositionAdlQuantileEstimationEndpoint<'r> {
    pub fn new(client: &'r crate::rest_api::RestApiClient) -> Self {
        Self { client }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PositionAdlQuantileEstimationParams {
    symbol: String,
    recv_window: Option<i64>,
    timestamp: i64,
}

impl PositionAdlQuantileEstimationParams {
    pub fn new(symbol: &str) -> Self {
        Self {
            symbol: symbol.to_owned(),
            recv_window: None,
            timestamp: Timestamp::now().as_millisecond(),
        }
    }

    pub fn recv_window(mut self, recv_window: i64) -> Self {
        self.recv_window = Some(recv_window);
        self
    }
}

pub type PositionAdlQuantileEstimationResponse = Vec<PositionAdlQuantileEstimation>;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PositionAdlQuantileEstimation {
    pub symbol: String,
    pub adl_quantile: AdlQuantile,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct AdlQuantile {
    pub long: i64,
    pub short: i64,
    pub both: i64,
}
