use jiff::Timestamp;
use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{enums::SecurityType, rest_api::endpoint};

endpoint!(
    "/sapi/v1/margin/isolatedMarginData",
    Method::GET,
    SecurityType::UserData,
    QueryIsolatedMarginFeeDataEndpoint,
    QueryIsolatedMarginFeeDataParams,
    QueryIsolatedMarginFeeDataResponse
);

/// Get isolated margin fee data collection with any vip level or user's current
/// specific data as <https://www.binance.com/en/margin-fee>.
///
/// - Weight:
///   - when symbol is specified: 1
///   - when symbol is not specified: 10
pub struct QueryIsolatedMarginFeeDataEndpoint<'r> {
    client: &'r crate::rest_api::RestApiClient,
}

impl<'r> QueryIsolatedMarginFeeDataEndpoint<'r> {
    pub fn new(client: &'r crate::rest_api::RestApiClient) -> Self {
        Self { client }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct QueryIsolatedMarginFeeDataParams {
    vip_level: Option<i64>,
    symbol: Option<String>,
    recv_window: Option<i64>,
    timestamp: i64,
}

impl QueryIsolatedMarginFeeDataParams {
    pub fn new() -> Self {
        Self {
            vip_level: None,
            symbol: None,
            recv_window: None,
            timestamp: Timestamp::now().as_millisecond(),
        }
    }

    pub fn vip_level(mut self, vip_level: i64) -> Self {
        self.vip_level = Some(vip_level);
        self
    }

    pub fn symbol(mut self, symbol: String) -> Self {
        self.symbol = Some(symbol);
        self
    }

    pub fn recv_window(mut self, recv_window: i64) -> Self {
        self.recv_window = Some(recv_window);
        self
    }
}

pub type QueryIsolatedMarginFeeDataResponse = Vec<IsolatedMarginFee>;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IsolatedMarginFee {
    pub vip_level: i64,
    pub symbol: String,
    pub leverage: String,
    pub data: Vec<IsolatedMarginFeeData>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IsolatedMarginFeeData {
    pub coin: String,
    pub daily_interest: String,
    pub borrow_limit: String,
}
