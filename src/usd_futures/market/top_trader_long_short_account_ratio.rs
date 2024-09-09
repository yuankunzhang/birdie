use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::rest_api::endpoint;

endpoint!(
    "/futures/data/topLongShortPositionRatio",
    Method::GET,
    TopTraderLongShortAccountRatioEndpoint,
    TopTraderLongShortAccountRatioParams,
    TopTraderLongShortAccountRatioResponse
);

/// The proportion of net long and net short accounts to total accounts of the
/// top 20% users with the highest margin balance. Each account is counted once
/// only. Long Account % = Accounts of top traders with net long positions
/// / Total accounts of top traders with open positions Short Account % =
/// Accounts of top traders with net short positions / Total accounts of top
/// traders with open positions Long/Short Ratio (Accounts) = Long Account % /
/// Short Account %
///
/// - Weight: 0
pub struct TopTraderLongShortAccountRatioEndpoint<'r> {
    client: &'r crate::rest_api::RestApiClient,
}

impl<'r> TopTraderLongShortAccountRatioEndpoint<'r> {
    pub fn new(client: &'r crate::rest_api::RestApiClient) -> Self {
        Self { client }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TopTraderLongShortAccountRatioParams {
    symbol: String,
    period: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    start_time: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    end_time: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<i64>,
}

impl TopTraderLongShortAccountRatioParams {
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

pub type TopTraderLongShortAccountRatioResponse = Vec<TopTraderLongShortAccountRatio>;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TopTraderLongShortAccountRatio {
    pub symbol: String,
    pub long_short_ratio: String,
    pub long_account: String,
    pub short_account: String,
    pub timestamp: String,
}
