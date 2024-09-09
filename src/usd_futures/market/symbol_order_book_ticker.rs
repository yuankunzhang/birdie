use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{rest_api::endpoint, web_socket_api::web_socket};

endpoint!(
    "/fapi/v1/ticker/bookTicker",
    Method::GET,
    SymbolOrderBookTickerEndpoint,
    SymbolOrderBookTickerParams,
    SymbolOrderBookTickerResponse
);

/// Best price/qty on the order book for a symbol or symbols.
///
/// - Weight:
/// - Data Source: Memory
pub struct SymbolOrderBookTickerEndpoint<'r> {
    client: &'r crate::rest_api::RestApiClient,
}

impl<'r> SymbolOrderBookTickerEndpoint<'r> {
    pub fn new(client: &'r crate::rest_api::RestApiClient) -> Self {
        Self { client }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SymbolOrderBookTickerParams {
    symbol: Option<String>,
}

impl Default for SymbolOrderBookTickerParams {
    fn default() -> Self {
        Self::new()
    }
}

impl SymbolOrderBookTickerParams {
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
pub enum SymbolOrderBookTickerResponse {
    Vec(Box<Vec<SymbolOrderBookTicker>>),
    Item(Box<SymbolOrderBookTicker>),
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SymbolOrderBookTicker {
    pub symbol: String,
    pub bid_price: String,
    pub bid_qty: String,
    pub ask_price: String,
    pub ask_qty: String,
    pub time: i64,
}

web_socket!(
    "ticker.book",
    SymbolOrderBookTickerWebSocket,
    SymbolOrderBookTickerParams,
    SymbolOrderBookTickerResponse
);

pub struct SymbolOrderBookTickerWebSocket<'w> {
    client: &'w crate::web_socket_api::WebSocketApiClient,
}

impl<'w> SymbolOrderBookTickerWebSocket<'w> {
    pub fn new(client: &'w crate::web_socket_api::WebSocketApiClient) -> Self {
        Self { client }
    }
}
