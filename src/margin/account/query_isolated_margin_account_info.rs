use jiff::Timestamp;
use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{enums::SecurityType, models::Asset, rest_api::endpoint};

endpoint!(
    "/sapi/v1/margin/isolated/account",
    Method::GET,
    SecurityType::UserData,
    QueryIsolatedMarginAccountInfoEndpoint,
    QueryIsolatedMarginAccountInfoParams,
    QueryIsolatedMarginAccountInfoResponse
);

/// Query Isolated Margin Account Info.
///
/// - Weight: 10
pub struct QueryIsolatedMarginAccountInfoEndpoint<'r> {
    client: &'r crate::rest_api::RestApiClient,
}

impl<'r> QueryIsolatedMarginAccountInfoEndpoint<'r> {
    pub fn new(client: &'r crate::rest_api::RestApiClient) -> Self {
        Self { client }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct QueryIsolatedMarginAccountInfoParams {
    symbol: Option<String>,
    recv_window: Option<i64>,
    timestamp: i64,
}

impl QueryIsolatedMarginAccountInfoParams {
    pub fn new() -> Self {
        Self {
            symbol: None,
            recv_window: None,
            timestamp: Timestamp::now().as_millisecond(),
        }
    }

    pub fn symbol(mut self, symbol: String) -> Self {
        self.symbol = Some(symbol);
        self
    }

    pub fn recv_window(mut self, recv_window: i64) -> Self {
        self.recv_window = Some(recv_window);
        self
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QueryIsolatedMarginAccountInfoResponse {
    pub assets: Vec<Asset>,
    pub total_asset_of_btc: Option<String>,
    pub total_liability_of_btc: Option<String>,
    pub total_net_asset_of_btc: Option<String>,
}
