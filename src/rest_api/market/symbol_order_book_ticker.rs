use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::rest_api::endpoint;

endpoint!(
    "/api/v3/ticker/bookTicker",
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
    client: &'r crate::rest_api::RestClient,
}

impl<'r> SymbolOrderBookTickerEndpoint<'r> {
    pub fn new(client: &'r crate::rest_api::RestClient) -> Self {
        Self { client }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SymbolOrderBookTickerParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    symbol: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    symbols: Option<Vec<String>>,
}

impl Default for SymbolOrderBookTickerParams {
    fn default() -> Self {
        Self::new()
    }
}

impl SymbolOrderBookTickerParams {
    pub fn new() -> Self {
        Self {
            symbol: None,
            symbols: None,
        }
    }

    pub fn symbol(mut self, symbol: &str) -> Self {
        self.symbol = Some(symbol.to_owned());
        self
    }

    pub fn symbols(mut self, symbols: &[&str]) -> Self {
        self.symbols = Some(symbols.iter().map(|s| s.to_string()).collect());
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
}
