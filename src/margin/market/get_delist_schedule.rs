use jiff::Timestamp;
use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{enums::SecurityType, rest_api::endpoint};

endpoint!(
    "/sapi/v1/margin/delist-schedule",
    Method::GET,
    SecurityType::MarketData,
    GetDelistScheduleEndpoint,
    GetDelistScheduleParams,
    GetDelistScheduleResponse
);

/// Get tokens or symbols delist schedule for cross margin and isolated margin.
///
/// - Weight: 100
pub struct GetDelistScheduleEndpoint<'r> {
    client: &'r crate::rest_api::RestApiClient,
}

impl<'r> GetDelistScheduleEndpoint<'r> {
    pub fn new(client: &'r crate::rest_api::RestApiClient) -> Self {
        Self { client }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetDelistScheduleParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
    timestamp: i64,
}

impl GetDelistScheduleParams {
    pub fn new() -> Self {
        Self {
            recv_window: None,
            timestamp: Timestamp::now().as_millisecond(),
        }
    }

    pub fn recv_window(mut self, recv_window: i64) -> Self {
        self.recv_window = Some(recv_window);
        self
    }
}

pub type GetDelistScheduleResponse = Vec<DelistSchedule>;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DelistSchedule {
    pub delist_time: i64,
    pub cross_margin_assets: Vec<String>,
    pub isolated_margin_symbols: Vec<String>,
}
