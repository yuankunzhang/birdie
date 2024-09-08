use jiff::Timestamp;
use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{enums::SecurityType, rest_api::endpoint};

endpoint!(
    "/sapi/v1/margin/exchange-small-liability-history",
    Method::GET,
    SecurityType::UserData,
    GetSmallLiabilityExchangeHistoryEndpoint,
    GetSmallLiabilityExchangeHistoryParams,
    GetSmallLiabilityExchangeHistoryResponse
);

/// Get Small liability Exchange History.
///
/// - Weight: 100
pub struct GetSmallLiabilityExchangeHistoryEndpoint<'r> {
    client: &'r crate::rest_api::RestApiClient,
}

impl<'r> GetSmallLiabilityExchangeHistoryEndpoint<'r> {
    pub fn new(client: &'r crate::rest_api::RestApiClient) -> Self {
        Self { client }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetSmallLiabilityExchangeHistoryParams {
    current: i64,
    size: i64,
    start_time: Option<i64>,
    end_time: Option<i64>,
    recv_window: Option<i64>,
    timestamp: i64,
}

impl GetSmallLiabilityExchangeHistoryParams {
    pub fn new(current: i64, size: i64) -> Self {
        Self {
            current,
            size,
            start_time: None,
            end_time: None,
            recv_window: None,
            timestamp: Timestamp::now().as_millisecond(),
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

    pub fn recv_window(mut self, recv_window: i64) -> Self {
        self.recv_window = Some(recv_window);
        self
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetSmallLiabilityExchangeHistoryResponse {
    pub total: i64,
    pub rows: Vec<SmallLiabilityExchangeCoinHistory>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SmallLiabilityExchangeCoinHistory {
    pub asset: String,
    pub amount: String,
    pub target_asset: String,
    pub target_amount: String,
    pub biz_type: String,
    pub timestamp: i64,
}
