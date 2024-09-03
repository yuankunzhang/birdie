use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{enums::TickerType, rest::endpoint};

endpoint!(
    "/api/v3/ticker/24hr",
    Method::GET,
    Ticker24hrEndpoint,
    Ticker24hrParams,
    Ticker24hrResponse
);

/// 24 hour rolling window price change statistics. Careful when accessing this
/// with no symbol.
///
/// - Weight:
/// - Data Source: Memory
pub struct Ticker24hrEndpoint<'r> {
    client: &'r crate::rest::RestClient,
}

impl<'r> Ticker24hrEndpoint<'r> {
    pub fn new(client: &'r crate::rest::RestClient) -> Self {
        Self { client }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Ticker24hrParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    symbol: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    symbols: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    r#type: Option<TickerType>,
}

impl Default for Ticker24hrParams {
    fn default() -> Self {
        Self::new()
    }
}

impl Ticker24hrParams {
    pub fn new() -> Self {
        Self {
            symbol: None,
            symbols: None,
            r#type: None,
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

    /// If none provided, the default is FULL.
    pub fn r#type(mut self, r#type: TickerType) -> Self {
        self.r#type = Some(r#type);
        self
    }
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum Ticker24hrResponse {
    Full(Box<Ticker24hrFull>),
    FullVec(Box<Vec<Ticker24hrFull>>),
    Mini(Box<Ticker24hrMini>),
    MiniVec(Box<Vec<Ticker24hrMini>>),
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Ticker24hrFull {
    pub symbol: String,
    pub price_change: String,
    pub price_change_percent: String,
    pub weighted_avg_price: String,
    pub prev_close_price: String,
    pub last_price: String,
    pub last_qty: String,
    pub bid_price: String,
    pub bid_qty: String,
    pub ask_price: String,
    pub ask_qty: String,
    pub open_price: String,
    pub high_price: String,
    pub low_price: String,
    pub volume: String,
    pub quote_volume: String,
    pub open_time: i64,
    pub close_time: i64,
    pub first_id: i64,
    pub last_id: i64,
    pub count: i64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Ticker24hrMini {
    pub symbol: String,
    pub open_price: String,
    pub high_price: String,
    pub low_price: String,
    pub last_price: String,
    pub volume: String,
    pub quote_volume: String,
    pub open_time: i64,
    pub close_time: i64,
    pub first_id: i64,
    pub last_id: i64,
    pub count: i64,
}
