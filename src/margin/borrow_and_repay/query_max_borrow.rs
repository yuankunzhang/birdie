use jiff::Timestamp;
use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{enums::SecurityType, rest_api::endpoint};

endpoint!(
    "/sapi/v1/margin/maxBorrowable",
    Method::GET,
    SecurityType::UserData,
    QueryMaxBorrowEndpoint,
    QueryMaxBorrowParams,
    QueryMaxBorrowResponse
);

/// Query Max Borrow.
///
/// - Weight: 50
pub struct QueryMaxBorrowEndpoint<'r> {
    client: &'r crate::rest_api::RestApiClient,
}

impl<'r> QueryMaxBorrowEndpoint<'r> {
    pub fn new(client: &'r crate::rest_api::RestApiClient) -> Self {
        Self { client }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct QueryMaxBorrowParams {
    asset: String,
    isolated_symbol: Option<String>,
    recv_window: Option<i64>,
    timestamp: i64,
}

impl QueryMaxBorrowParams {
    pub fn new(asset: &str) -> Self {
        Self {
            asset: asset.to_owned(),
            isolated_symbol: None,
            recv_window: None,
            timestamp: Timestamp::now().as_millisecond(),
        }
    }

    pub fn isolated_symbol(mut self, isolated_symbol: &str) -> Self {
        self.isolated_symbol = Some(isolated_symbol.to_owned());
        self
    }

    pub fn recv_window(mut self, recv_window: i64) -> Self {
        self.recv_window = Some(recv_window);
        self
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QueryMaxBorrowResponse {
    pub amount: String,
    pub borrow_limit: String,
}
