use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::rest_api::endpoint;

endpoint!(
    "/fapi/v2/ticker/price",
    Method::GET,
    SymbolOrderBookTickerV2Endpoint,
    SymbolOrderBookTickerV2Params,
    SymbolOrderBookTickerV2Response
);

/// Best price/qty on the order book for a symbol or symbols.
///
/// - Weight:
pub struct SymbolOrderBookTickerV2Endpoint<'r> {
    client: &'r crate::rest_api::RestApiClient,
}

impl<'r> SymbolOrderBookTickerV2Endpoint<'r> {
    pub fn new(client: &'r crate::rest_api::RestApiClient) -> Self {
        Self { client }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SymbolOrderBookTickerV2Params {
    symbol: Option<String>,
}

impl Default for SymbolOrderBookTickerV2Params {
    fn default() -> Self {
        Self::new()
    }
}

impl SymbolOrderBookTickerV2Params {
    pub fn new() -> Self {
        Self { symbol: None }
    }

    pub fn symbol(mut self, symbol: &str) -> Self {
        self.symbol = Some(symbol.to_owned());
        self
    }
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum SymbolOrderBookTickerV2Response {
    Vec(Box<Vec<SymbolOrderBookTickerV2>>),
    Item(Box<SymbolOrderBookTickerV2>),
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SymbolOrderBookTickerV2 {
    pub symbol: String,
    pub price: String,
    pub time: i64,
}
