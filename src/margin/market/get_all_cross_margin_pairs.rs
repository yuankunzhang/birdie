use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{enums::SecurityType, rest_api::endpoint};

endpoint!(
    "/sapi/v1/margin/allPairs",
    Method::GET,
    SecurityType::MarketData,
    GetAllCrossMarginPairsEndpoint,
    GetAllCrossMarginPairsParams,
    GetAllCrossMarginPairsResponse
);

/// Get All Cross Margin Pairs
///
/// - Weight: 1
pub struct GetAllCrossMarginPairsEndpoint<'r> {
    client: &'r crate::rest_api::RestApiClient,
}

impl<'r> GetAllCrossMarginPairsEndpoint<'r> {
    pub fn new(client: &'r crate::rest_api::RestApiClient) -> Self {
        Self { client }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetAllCrossMarginPairsParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
}

impl GetAllCrossMarginPairsParams {
    pub fn new() -> Self {
        Self { symbol: None }
    }

    pub fn symbol(mut self, symbol: String) -> Self {
        self.symbol = Some(symbol);
        self
    }
}

pub type GetAllCrossMarginPairsResponse = Vec<CrossMarginPairs>;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CrossMarginPairs {
    pub base: String,
    pub id: i64,
    pub is_buy_allowed: bool,
    pub is_margin_trade: bool,
    pub is_sell_allowed: bool,
    pub quote: String,
    pub symbol: String,
}
