use jiff::Timestamp;
use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{enums::SecurityType, rest_api::endpoint};

endpoint!(
    "/sapi/v1/margin/maxTransferable",
    Method::GET,
    SecurityType::UserData,
    QueryMaxTransferOutAmountEndpoint,
    QueryMaxTransferOutAmountParams,
    QueryMaxTransferOutAmountResponse
);

/// Query Max Transfer-Out Amount.
///
/// - Weight: 50
pub struct QueryMaxTransferOutAmountEndpoint<'r> {
    client: &'r crate::rest_api::RestApiClient,
}

impl<'r> QueryMaxTransferOutAmountEndpoint<'r> {
    pub fn new(client: &'r crate::rest_api::RestApiClient) -> Self {
        Self { client }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct QueryMaxTransferOutAmountParams {
    asset: String,
    isolated_symbol: Option<String>,
    recv_window: Option<i64>,
    timestamp: i64,
}

impl QueryMaxTransferOutAmountParams {
    pub fn new(asset: &str) -> Self {
        Self {
            asset: asset.to_owned(),
            isolated_symbol: None,
            recv_window: None,
            timestamp: Timestamp::now().as_millisecond(),
        }
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
pub struct QueryMaxTransferOutAmountResponse {
    pub amount: String,
}
