use jiff::Timestamp;
use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{enums::SecurityType, rest_api::endpoint};

endpoint!(
    "/sapi/v1/margin/interestHistory",
    Method::GET,
    SecurityType::UserData,
    GetInterestHistoryEndpoint,
    GetInterestHistoryParams,
    GetInterestHistoryResponse
);

/// Get Interest History.
///
/// - Weight: 1
pub struct GetInterestHistoryEndpoint<'r> {
    client: &'r crate::rest_api::RestApiClient,
}

impl<'r> GetInterestHistoryEndpoint<'r> {
    pub fn new(client: &'r crate::rest_api::RestApiClient) -> Self {
        Self { client }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetInterestHistoryParams {
    asset: Option<String>,
    isolated_symbol: Option<String>,
    start_time: Option<i64>,
    end_time: Option<i64>,
    current: Option<i64>,
    size: Option<i64>,
    archived: Option<bool>,
    recv_window: Option<i64>,
    timestamp: i64,
}

impl GetInterestHistoryParams {
    pub fn new() -> Self {
        Self {
            asset: None,
            isolated_symbol: None,
            start_time: None,
            end_time: None,
            current: None,
            size: None,
            archived: None,
            recv_window: None,
            timestamp: Timestamp::now().as_millisecond(),
        }
    }

    pub fn asset(mut self, asset: &str) -> Self {
        self.asset = Some(asset.to_owned());
        self
    }

    pub fn isolated_symbol(mut self, isolated_symbol: &str) -> Self {
        self.isolated_symbol = Some(isolated_symbol.to_owned());
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

    pub fn current(mut self, current: i64) -> Self {
        self.current = Some(current);
        self
    }

    pub fn size(mut self, size: i64) -> Self {
        self.size = Some(size);
        self
    }

    pub fn archived(mut self, archived: bool) -> Self {
        self.archived = Some(archived);
        self
    }

    pub fn recv_window(mut self, recv_window: i64) -> Self {
        self.recv_window = Some(recv_window);
        self
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetInterestHistoryResponse {
    pub rows: Vec<InterestHistory>,
    pub total: i64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InterestHistory {
    pub tx_id: i64,
    pub interest_accured_time: i64,
    pub asset: String,
    pub raw_asset: String,
    pub principal: String,
    pub interest: String,
    pub interest_rate: String,
    pub r#type: String,
    pub isolated_symbol: String,
}
