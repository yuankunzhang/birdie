use jiff::Timestamp;
use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{enums::SecurityType, rest_api::endpoint};

endpoint!(
    "/sapi/v1/margin/crossMarginData",
    Method::GET,
    SecurityType::UserData,
    QueryCrossMarginFeeDataEndpoint,
    QueryCrossMarginFeeDataParams,
    QueryCrossMarginFeeDataResponse
);

/// Get cross margin fee data collection with any vip level or user's current
/// specific data as https://www.binance.com/en/margin-fee.
///
/// - Weight:
///   - when coin is specified: 1
///   - when the coin parameter is omitted: 5
pub struct QueryCrossMarginFeeDataEndpoint<'r> {
    client: &'r crate::rest_api::RestApiClient,
}

impl<'r> QueryCrossMarginFeeDataEndpoint<'r> {
    pub fn new(client: &'r crate::rest_api::RestApiClient) -> Self {
        Self { client }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct QueryCrossMarginFeeDataParams {
    vip_level: Option<i64>,
    coin: Option<String>,
    recv_window: Option<i64>,
    timestamp: i64,
}

impl QueryCrossMarginFeeDataParams {
    pub fn new() -> Self {
        Self {
            vip_level: None,
            coin: None,
            recv_window: None,
            timestamp: Timestamp::now().as_millisecond(),
        }
    }

    pub fn vip_level(mut self, vip_level: i64) -> Self {
        self.vip_level = Some(vip_level);
        self
    }

    pub fn coin(mut self, coin: String) -> Self {
        self.coin = Some(coin);
        self
    }

    pub fn recv_window(mut self, recv_window: i64) -> Self {
        self.recv_window = Some(recv_window);
        self
    }
}

pub type QueryCrossMarginFeeDataResponse = Vec<CrossMarginFeeData>;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CrossMarginFeeData {
    pub vip_level: i64,
    pub coin: String,
    pub transfer_in: bool,
    pub borrowable: bool,
    pub daily_interest: String,
    pub yearly_interest: String,
    pub borrow_limit: String,
    pub marginable_pairs: Vec<String>,
}
