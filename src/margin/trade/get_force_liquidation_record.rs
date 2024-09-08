use jiff::Timestamp;
use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{
    enums::{OrderSide, SecurityType, TimeInForce},
    rest_api::endpoint,
};

endpoint!(
    "/sapi/v1/margin/forceLiquidationRec",
    Method::GET,
    SecurityType::UserData,
    GetForceLiquidationRecordEndpoint,
    GetForceLiquidationRecordParams,
    GetForceLiquidationRecordResponse
);

/// Margin account borrow/repay(MARGIN).
///
/// - Weight: 1
pub struct GetForceLiquidationRecordEndpoint<'r> {
    client: &'r crate::rest_api::RestApiClient,
}

impl<'r> GetForceLiquidationRecordEndpoint<'r> {
    pub fn new(client: &'r crate::rest_api::RestApiClient) -> Self {
        Self { client }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetForceLiquidationRecordParams {
    start_time: Option<i64>,
    end_time: Option<i64>,
    isolated_symbol: Option<String>,
    current: Option<i64>,
    size: Option<i64>,
    recv_window: Option<i64>,
    timestamp: i64,
}

impl GetForceLiquidationRecordParams {
    pub fn new() -> Self {
        Self {
            start_time: None,
            end_time: None,
            isolated_symbol: None,
            current: None,
            size: None,
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

    pub fn isolated_symbol(mut self, isolated_symbol: &str) -> Self {
        self.isolated_symbol = Some(isolated_symbol.to_owned());
        self
    }

    pub fn current(mut self, current: i64) -> Self {
        self.current = Some(current);
        self
    }

    pub fn size(mut self, size: i64) -> Self {
        self.size = Some(size);
        self
    }

    pub fn recv_window(mut self, recv_window: i64) -> Self {
        self.recv_window = Some(recv_window);
        self
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetForceLiquidationRecordResponse {
    pub rows: Vec<ForceLiquidationRecord>,
    pub total: i64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ForceLiquidationRecord {
    pub avg_price: String,
    pub executed_qty: String,
    pub order_id: i64,
    pub price: String,
    pub qty: String,
    pub side: OrderSide,
    pub symbol: String,
    pub time_in_force: TimeInForce,
    pub is_isolated: bool,
    pub updated_time: i64,
}
