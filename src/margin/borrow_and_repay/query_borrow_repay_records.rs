use jiff::Timestamp;
use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{enums::SecurityType, rest_api::endpoint};

endpoint!(
    "/sapi/v1/margin/borrow-repay",
    Method::GET,
    SecurityType::UserData,
    QueryBorrowRepayRecordsEndpoint,
    QueryBorrowRepayRecordsParams,
    QueryBorrowRepayRecordsResponse
);

/// Get Interest History.
///
/// - Weight: 10
pub struct QueryBorrowRepayRecordsEndpoint<'r> {
    client: &'r crate::rest_api::RestApiClient,
}

impl<'r> QueryBorrowRepayRecordsEndpoint<'r> {
    pub fn new(client: &'r crate::rest_api::RestApiClient) -> Self {
        Self { client }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct QueryBorrowRepayRecordsParams {
    asset: Option<String>,
    isolated_symbol: Option<String>,
    tx_id: Option<i64>,
    start_time: Option<i64>,
    end_time: Option<i64>,
    current: Option<i64>,
    size: Option<i64>,
    r#type: String,
    recv_window: Option<i64>,
    timestamp: i64,
}

impl QueryBorrowRepayRecordsParams {
    pub fn new(r#type: &str) -> Self {
        Self {
            asset: None,
            isolated_symbol: None,
            tx_id: None,
            start_time: None,
            end_time: None,
            current: None,
            size: None,
            r#type: r#type.to_owned(),
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

    pub fn tx_id(mut self, tx_id: i64) -> Self {
        self.tx_id = Some(tx_id);
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

    pub fn recv_window(mut self, recv_window: i64) -> Self {
        self.recv_window = Some(recv_window);
        self
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QueryBorrowRepayRecordsResponse {
    pub rows: Vec<BorrowRepayRecord>,
    pub total: i64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BorrowRepayRecord {
    pub r#type: String,
    pub isolated_symbol: String,
    pub amount: String,
    pub asset: String,
    pub interest: String,
    pub principal: String,
    pub status: String,
    pub timestamp: i64,
    pub tx_id: i64,
}
