use jiff::Timestamp;
use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{enums::SecurityType, rest_api::endpoint};

endpoint!(
    "/sapi/v1/margin/isolatedMarginTier",
    Method::GET,
    SecurityType::MarketData,
    QueryIsolatedMarginTierDataEndpoint,
    QueryIsolatedMarginTierDataParams,
    QueryIsolatedMarginTierDataResponse
);

/// Get isolated margin tier data collection with any tier as
/// <https://www.binance.com/en/margin-data>.
///
/// - Weight: 1
pub struct QueryIsolatedMarginTierDataEndpoint<'r> {
    client: &'r crate::rest_api::RestApiClient,
}

impl<'r> QueryIsolatedMarginTierDataEndpoint<'r> {
    pub fn new(client: &'r crate::rest_api::RestApiClient) -> Self {
        Self { client }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct QueryIsolatedMarginTierDataParams {
    symbol: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    tier: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
    timestamp: i64,
}

impl QueryIsolatedMarginTierDataParams {
    pub fn new(symbol: &str) -> Self {
        Self {
            symbol: symbol.to_owned(),
            tier: None,
            recv_window: None,
            timestamp: Timestamp::now().as_millisecond(),
        }
    }

    pub fn tier(mut self, tier: i64) -> Self {
        self.tier = Some(tier);
        self
    }

    pub fn recv_window(mut self, recv_window: i64) -> Self {
        self.recv_window = Some(recv_window);
        self
    }
}

pub type QueryIsolatedMarginTierDataResponse = Vec<IsolatedMarginTierData>;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IsolatedMarginTierData {
    pub symbol: String,
    pub tier: i64,
    pub effective_multiple: String,
    pub initial_risk_ratio: String,
    pub liquidation_risk_ratio: String,
    pub base_asset_max_borrowable: String,
    pub quote_asset_max_borrowable: String,
}
