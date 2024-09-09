use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::rest_api::endpoint;

endpoint!(
    "/futures/data/globalLongShortAccountRatio",
    Method::GET,
    LongShortRatioEndpoint,
    LongShortRatioParams,
    LongShortRatioResponse
);

/// Query symbol Long/Short Ratio.
///
/// - Weight: 0
pub struct LongShortRatioEndpoint<'r> {
    client: &'r crate::rest_api::RestApiClient,
}

impl<'r> LongShortRatioEndpoint<'r> {
    pub fn new(client: &'r crate::rest_api::RestApiClient) -> Self {
        Self { client }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LongShortRatioParams {
    pub symbol: String,
    pub period: String,
    pub limit: Option<i64>,
    pub start_time: Option<i64>,
    pub end_time: Option<i64>,
}

impl LongShortRatioParams {
    pub fn new(symbol: &str, period: &str) -> Self {
        LongShortRatioParams {
            symbol: symbol.to_owned(),
            period: period.to_owned(),
            limit: None,
            start_time: None,
            end_time: None,
        }
    }
}

#[derive(Debug, Deserialize)]
pub enum LongShortRatioResponse {
    Item(Box<LongShortRatio>),
    Vec(Box<Vec<LongShortRatio>>),
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LongShortRatio {
    pub symbol: String,
    pub long_short_ratio: String,
    pub long_account: String,
    pub timestamp: i64,
}
