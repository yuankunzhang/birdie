use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::rest_api::endpoint;

endpoint!(
    "/futures/data/openInterestHist",
    Method::GET,
    QueryDeliveryPriceEndpoint,
    QueryDeliveryPriceParams,
    QueryDeliveryPriceResponse
);

/// Latest price for a symbol or symbols.
///
/// - Weight: 0
pub struct QueryDeliveryPriceEndpoint<'r> {
    client: &'r crate::rest_api::RestApiClient,
}

impl<'r> QueryDeliveryPriceEndpoint<'r> {
    pub fn new(client: &'r crate::rest_api::RestApiClient) -> Self {
        Self { client }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct QueryDeliveryPriceParams {
    pub symbol: String,
}

impl QueryDeliveryPriceParams {
    pub fn new(symbol: &str) -> Self {
        QueryDeliveryPriceParams {
            symbol: symbol.to_owned(),
        }
    }
}

pub type QueryDeliveryPriceResponse = Vec<DeliveryPrice>;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeliveryPrice {
    pub delivery_time: i64,
    pub delivery_price: f64,
}
