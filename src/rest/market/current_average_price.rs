use reqwest::Method;
use serde::Serialize;

use crate::{models::AveragePrice, rest::endpoint};

endpoint!(
    "/api/v3/avgPrice",
    Method::GET,
    CurrentAveragePriceEndpoint,
    CurrentAveragePriceParams,
    CurrentAveragePriceResponse
);

/// Current average price for a symbol.
///
/// - Weight: 2
/// - Data Source: Memory
pub struct CurrentAveragePriceEndpoint<'r> {
    client: &'r crate::rest::RestClient,
}

impl<'r> CurrentAveragePriceEndpoint<'r> {
    pub fn new(client: &'r crate::rest::RestClient) -> Self {
        Self { client }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CurrentAveragePriceParams {
    symbol: String,
}

impl CurrentAveragePriceParams {
    pub fn new(symbol: &str) -> Self {
        Self {
            symbol: symbol.to_owned(),
        }
    }
}

pub type CurrentAveragePriceResponse = AveragePrice;
