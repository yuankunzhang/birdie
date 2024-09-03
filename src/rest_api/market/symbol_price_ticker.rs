use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::rest_api::endpoint;

endpoint!(
    "/api/v3/ticker/price",
    Method::GET,
    SymbolPriceTickerEndpoint,
    SymbolPriceTickerParams,
    SymbolPriceTickerResponse
);

/// Latest price for a symbol or symbols.
///
/// - Weight:
/// - Data Source: Memory
pub struct SymbolPriceTickerEndpoint<'r> {
    client: &'r crate::rest_api::RestClient,
}

impl<'r> SymbolPriceTickerEndpoint<'r> {
    pub fn new(client: &'r crate::rest_api::RestClient) -> Self {
        Self { client }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SymbolPriceTickerParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    symbol: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    symbols: Option<Vec<String>>,
}

impl Default for SymbolPriceTickerParams {
    fn default() -> Self {
        Self::new()
    }
}

impl SymbolPriceTickerParams {
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
pub enum SymbolPriceTickerResponse {
    Vec(Box<Vec<SymbolPriceTicker>>),
    Item(Box<SymbolPriceTicker>),
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SymbolPriceTicker {
    pub symbol: String,
    pub price: String,
}
