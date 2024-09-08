use jiff::Timestamp;
use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{enums::SecurityType, rest_api::endpoint};

endpoint!(
    "/sapi/v1/margin/transfer",
    Method::GET,
    SecurityType::UserData,
    GetCrossMarginTransferHistoryEndpoint,
    GetCrossMarginTransferHistoryParams,
    GetCrossMarginTransferHistoryResponse
);

/// Get Cross Margin Transfer History.
///
/// - Weight: 1
pub struct GetCrossMarginTransferHistoryEndpoint<'r> {
    client: &'r crate::rest_api::RestApiClient,
}

impl<'r> GetCrossMarginTransferHistoryEndpoint<'r> {
    pub fn new(client: &'r crate::rest_api::RestApiClient) -> Self {
        Self { client }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetCrossMarginTransferHistoryParams {
    asset: Option<String>,
    r#type: Option<String>,
    start_time: Option<i64>,
    end_time: Option<i64>,
    current: Option<i64>,
    size: Option<i64>,
    archived: Option<String>,
    isolated_symbol: Option<String>,
    recv_window: Option<i64>,
    timestamp: i64,
}

impl GetCrossMarginTransferHistoryParams {
    pub fn new() -> Self {
        Self {
            asset: None,
            r#type: None,
            start_time: None,
            end_time: None,
            current: None,
            size: None,
            archived: None,
            isolated_symbol: None,
            recv_window: None,
            timestamp: Timestamp::now().as_millisecond(),
        }
    }

    pub fn asset(mut self, asset: String) -> Self {
        self.asset = Some(asset);
        self
    }

    pub fn r#type(mut self, r#type: String) -> Self {
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

    pub fn current(mut self, current: i64) -> Self {
        self.current = Some(current);
        self
    }

    pub fn size(mut self, size: i64) -> Self {
        self.size = Some(size);
        self
    }

    pub fn archived(mut self, archived: String) -> Self {
        self.archived = Some(archived);
        self
    }

    pub fn isolated_symbol(mut self, isolated_symbol: String) -> Self {
        self.isolated_symbol = Some(isolated_symbol);
        self
    }

    pub fn recv_window(mut self, recv_window: i64) -> Self {
        self.recv_window = Some(recv_window);
        self
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetCrossMarginTransferHistoryResponse {
    pub rows: Vec<CrossMarginTransferHistory>,
    pub total: i64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CrossMarginTransferHistory {
    pub amount: String,
    pub asset: String,
    pub status: String,
    pub timestamp: i64,
    pub tx_id: String,
    pub r#type: String,
    pub trans_from: String,
    pub trans_to: String,
}
