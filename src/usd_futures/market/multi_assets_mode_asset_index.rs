use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::rest_api::endpoint;

endpoint!(
    "/fapi/v1/assetIndex",
    Method::GET,
    MultiAssetsModeAssetIndexEndpoint,
    MultiAssetsModeAssetIndexParams,
    MultiAssetsModeAssetIndexResponse
);

/// Asset index for Multi-Assets mode.
///
/// - Weight:
pub struct MultiAssetsModeAssetIndexEndpoint<'r> {
    client: &'r crate::rest_api::RestApiClient,
}

impl<'r> MultiAssetsModeAssetIndexEndpoint<'r> {
    pub fn new(client: &'r crate::rest_api::RestApiClient) -> Self {
        Self { client }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MultiAssetsModeAssetIndexParams {
    pub symbol: Option<String>,
}

impl MultiAssetsModeAssetIndexParams {
    pub fn new() -> Self {
        MultiAssetsModeAssetIndexParams { symbol: None }
    }

    pub fn symbol(mut self, symbol: &str) -> Self {
        self.symbol = Some(symbol.to_string());
        self
    }
}

#[derive(Debug, Deserialize)]
pub enum MultiAssetsModeAssetIndexResponse {
    Item(Box<MultiAssetsModeAssetIndex>),
    Vec(Box<Vec<MultiAssetsModeAssetIndex>>),
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MultiAssetsModeAssetIndex {
    pub symbol: String,
    pub time: i64,
    pub index: String,
    pub big_buffer: String,
    pub ask_buffer: String,
    pub bid_rate: String,
    pub ask_rate: String,
    pub auto_exchange_bid_buffer: String,
    pub auto_exchange_ask_buffer: String,
    pub auto_exchange_bid_rate: String,
    pub auto_exchange_ask_rate: String,
}
