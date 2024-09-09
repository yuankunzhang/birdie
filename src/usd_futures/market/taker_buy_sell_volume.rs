use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::rest_api::endpoint;

endpoint!(
    "/futures/data/takerlongshortRatio",
    Method::GET,
    TakerBuySellVolumeEndpoint,
    TakerBuySellVolumeParams,
    TakerBuySellVolumeResponse
);

/// Taker Buy/Sell Volume.
///
/// - Weight: 0
pub struct TakerBuySellVolumeEndpoint<'r> {
    client: &'r crate::rest_api::RestApiClient,
}

impl<'r> TakerBuySellVolumeEndpoint<'r> {
    pub fn new(client: &'r crate::rest_api::RestApiClient) -> Self {
        Self { client }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TakerBuySellVolumeParams {
    symbol: String,
    period: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    start_time: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    end_time: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<i64>,
}

impl TakerBuySellVolumeParams {
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

pub type TakerBuySellVolumeResponse = Vec<TakerBuySellVolume>;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TakerBuySellVolume {
    pub buy_sell_ratio: String,
    pub buy_vol: String,
    pub sell_vol: String,
    pub timestamp: String,
}
