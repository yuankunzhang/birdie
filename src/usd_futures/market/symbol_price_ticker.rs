use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{rest_api::endpoint, web_socket_api::web_socket};

endpoint!(
    "/fapi/v1/ticker/price",
    Method::GET,
    SymbolPriceTickerEndpoint,
    SymbolPriceTickerParams,
    SymbolPriceTickerResponse
);

/// Latest price for a symbol or symbols.
///
/// - Weight:
pub struct SymbolPriceTickerEndpoint<'r> {
    client: &'r crate::rest_api::RestApiClient,
}

impl<'r> SymbolPriceTickerEndpoint<'r> {
    pub fn new(client: &'r crate::rest_api::RestApiClient) -> Self {
        Self { client }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SymbolPriceTickerParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    symbol: Option<String>,
}

impl Default for SymbolPriceTickerParams {
    fn default() -> Self {
        Self::new()
    }
}

impl SymbolPriceTickerParams {
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
pub enum SymbolPriceTickerResponse {
    Vec(Box<Vec<SymbolPriceTicker>>),
    Item(Box<SymbolPriceTicker>),
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SymbolPriceTicker {
    pub symbol: String,
    pub price: String,
    pub time: i64,
}

web_socket!(
    "ticker.price",
    SymbolPriceTickerWebSocket,
    SymbolPriceTickerParams,
    SymbolPriceTickerResponse
);

pub struct SymbolPriceTickerWebSocket<'w> {
    client: &'w crate::web_socket_api::WebSocketApiClient,
}

impl<'w> SymbolPriceTickerWebSocket<'w> {
    pub fn new(client: &'w crate::web_socket_api::WebSocketApiClient) -> Self {
        Self { client }
    }
}
