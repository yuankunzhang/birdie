use jiff::Timestamp;
use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{enums::SecurityType, errors::BinanceError, rest_api::endpoint};

endpoint!(
    "/fapi/v1/multiAssetsMargin",
    Method::POST,
    SecurityType::Trade,
    ChangeMultiAssetsModeEndpoint,
    ChangeMultiAssetsModeParams,
    ChangeMultiAssetsModeResponse
);

/// Change user's Multi-Assets mode (Multi-Assets Mode or Single-Asset Mode) on Every symbol.
///
/// - Weight: 1
pub struct ChangeMultiAssetsModeEndpoint<'r> {
    client: &'r crate::rest_api::RestApiClient,
}

impl<'r> ChangeMultiAssetsModeEndpoint<'r> {
    pub fn new(client: &'r crate::rest_api::RestApiClient) -> Self {
        Self { client }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ChangeMultiAssetsModeParams {
    multi_assets_margin: String,
    recv_window: Option<i64>,
    timestamp: i64,
}

impl ChangeMultiAssetsModeParams {
    pub fn new(multi_assets_margin: &str) -> Self {
        Self {
            multi_assets_margin: multi_assets_margin.to_owned(),
            recv_window: None,
            timestamp: Timestamp::now().as_millisecond(),
        }
    }

    pub fn recv_window(mut self, recv_window: i64) -> Self {
        self.recv_window = Some(recv_window);
        self
    }
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum ChangeMultiAssetsModeResponse {
    Success,
    Failure(BinanceError),
}
