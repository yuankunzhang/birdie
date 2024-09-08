use jiff::Timestamp;
use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{enums::SecurityType, rest_api::endpoint};

endpoint!(
    "/sapi/v1/margin/account",
    Method::GET,
    SecurityType::UserData,
    QueryCrossMarginAccountDetailsEndpoint,
    QueryCrossMarginAccountDetailsParams,
    QueryCrossMarginAccountDetailsResponse
);

/// Query Cross Margin Account Details.
///
/// - Weight: 10
pub struct QueryCrossMarginAccountDetailsEndpoint<'r> {
    client: &'r crate::rest_api::RestApiClient,
}

impl<'r> QueryCrossMarginAccountDetailsEndpoint<'r> {
    pub fn new(client: &'r crate::rest_api::RestApiClient) -> Self {
        Self { client }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct QueryCrossMarginAccountDetailsParams {
    recv_window: Option<i64>,
    timestamp: i64,
}

impl QueryCrossMarginAccountDetailsParams {
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

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QueryCrossMarginAccountDetailsResponse {
    pub created: bool,
    pub borrow_enabled: bool,
    pub margin_level: String,
    pub collateral_margin_level: String,
    pub total_asset_of_btc: String,
    pub total_liability_of_btc: String,
    pub total_net_asset_of_btc: String,
    pub total_collateral_value_in_usdt: String,
    pub trade_enabled: bool,
    pub transfer_enabled: bool,
    pub account_type: String,
    pub user_assets: Vec<UserAsset>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserAsset {
    pub asset: String,
    pub borrowed: String,
    pub free: String,
    pub interest: String,
    pub locked: String,
    pub net_asset: String,
}
