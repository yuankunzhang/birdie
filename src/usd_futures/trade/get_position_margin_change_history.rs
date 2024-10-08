use jiff::Timestamp;
use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{
    enums::{futures::PositionSide, SecurityType},
    rest_api::endpoint,
};

endpoint!(
    "/fapi/v1/positionMargin/history",
    Method::GET,
    SecurityType::Trade,
    GetPositionMarginChangeHistoryEndpoint,
    GetPositionMarginChangeHistoryParams,
    GetPositionMarginChangeHistoryResponse
);

/// Get position margin change history.
///
/// - Weight: 1
pub struct GetPositionMarginChangeHistoryEndpoint<'r> {
    client: &'r crate::rest_api::RestApiClient,
}

impl<'r> GetPositionMarginChangeHistoryEndpoint<'r> {
    pub fn new(client: &'r crate::rest_api::RestApiClient) -> Self {
        Self { client }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetPositionMarginChangeHistoryParams {
    symbol: String,
    r#type: Option<i64>,
    start_time: Option<i64>,
    end_time: Option<i64>,
    limit: Option<i64>,
    recv_window: Option<i64>,
    timestamp: i64,
}

impl GetPositionMarginChangeHistoryParams {
    pub fn new(symbol: &str) -> Self {
        Self {
            symbol: symbol.to_owned(),
            r#type: None,
            start_time: None,
            end_time: None,
            limit: None,
            recv_window: None,
            timestamp: Timestamp::now().as_millisecond(),
        }
    }

    pub fn r#type(mut self, r#type: i64) -> Self {
        self.r#type = Some(r#type);
        self
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

    pub fn recv_window(mut self, recv_window: i64) -> Self {
        self.recv_window = Some(recv_window);
        self
    }
}

pub type GetPositionMarginChangeHistoryResponse = Vec<PositionMarginChangeHistory>;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PositionMarginChangeHistory {
    pub symbol: String,
    pub r#type: i64,
    pub delta_type: String,
    pub amount: String,
    pub asset: String,
    pub time: i64,
    pub position_side: PositionSide,
}
