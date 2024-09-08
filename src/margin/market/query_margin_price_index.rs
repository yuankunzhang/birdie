use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{enums::SecurityType, rest_api::endpoint};

endpoint!(
    "/sapi/v1/margin/priceIndex",
    Method::GET,
    SecurityType::MarketData,
    QueryMarginPriceIndexEndpoint,
    QueryMarginPriceIndexParams,
    QueryMarginPriceIndexResponse
);

/// Query Margin PriceIndex.
///
/// - Weight: 10
pub struct QueryMarginPriceIndexEndpoint<'r> {
    client: &'r crate::rest_api::RestApiClient,
}

impl<'r> QueryMarginPriceIndexEndpoint<'r> {
    pub fn new(client: &'r crate::rest_api::RestApiClient) -> Self {
        Self { client }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct QueryMarginPriceIndexParams {
    symbol: String,
}

impl QueryMarginPriceIndexParams {
    pub fn new(symbol: &str) -> Self {
        Self {
            symbol: symbol.to_owned(),
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QueryMarginPriceIndexResponse {
    pub calc_time: i64,
    pub price: String,
    pub symbol: String,
}
