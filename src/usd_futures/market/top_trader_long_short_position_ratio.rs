use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::rest_api::endpoint;

endpoint!(
    "/futures/data/topLongShortPositionRatio",
    Method::GET,
    TopTraderLongShortPositionRatioEndpoint,
    TopTraderLongShortPositionRatioParams,
    TopTraderLongShortPositionRatioResponse
);

/// The proportion of net long and net short positions to total open positions of
/// the top 20% users with the highest margin balance. Long Position % = Long
/// positions of top traders / Total open positions of top traders Short Position
/// % = Short positions of top traders / Total open positions of top traders
/// Long/Short Ratio (Positions) = Long Position % / Short Position %
///
/// - Weight: 0
pub struct TopTraderLongShortPositionRatioEndpoint<'r> {
    client: &'r crate::rest_api::RestApiClient,
}

impl<'r> TopTraderLongShortPositionRatioEndpoint<'r> {
    pub fn new(client: &'r crate::rest_api::RestApiClient) -> Self {
        Self { client }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TopTraderLongShortPositionRatioParams {
    symbol: String,
    period: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    start_time: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    end_time: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<i64>,
}

impl TopTraderLongShortPositionRatioParams {
    pub fn new(symbol: &str, period: &str) -> Self {
        Self {
            symbol: symbol.to_owned(),
            period: period.to_owned(),
            start_time: None,
            end_time: None,
            limit: None,
        }
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

pub type TopTraderLongShortPositionRatioResponse = Vec<TopTraderLongShortPositionRatio>;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TopTraderLongShortPositionRatio {
    pub symbol: String,
    pub long_short_ratio: String,
    pub long_account: String,
    pub short_account: String,
    pub timestamp: String,
}
