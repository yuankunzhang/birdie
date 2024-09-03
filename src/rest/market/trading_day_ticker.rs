use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{enums::TickerType, rest::endpoint};

endpoint!(
    "/api/v3/ticker/tradingDay",
    Method::GET,
    TradingDayTickerEndpoint,
    TradingDayTickerParams,
    TradingDayTickerResponse
);

/// Price change statistics for a trading day.
///
/// - Weight: 4 for each requested symbol. The weight for this request will cap
///   at 200 once the number of symbols in the request is more than 50.
/// - Data Source: Memory
pub struct TradingDayTickerEndpoint<'r> {
    client: &'r crate::rest::RestClient,
}

impl<'r> TradingDayTickerEndpoint<'r> {
    pub fn new(client: &'r crate::rest::RestClient) -> Self {
        Self { client }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TradingDayTickerParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    symbol: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    symbols: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    time_zone: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    r#type: Option<TickerType>,
}

impl Default for TradingDayTickerParams {
    fn default() -> Self {
        Self::new()
    }
}

impl TradingDayTickerParams {
    pub fn new() -> Self {
        Self {
            symbol: None,
            symbols: None,
            time_zone: None,
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

    /// Default: 0 (UTC).
    pub fn time_zone(mut self, time_zone: &str) -> Self {
        self.time_zone = Some(time_zone.to_owned());
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
pub enum TradingDayTickerResponse {
    Full(Box<TickerFull>),
    FullVec(Box<Vec<TickerFull>>),
    Mini(Box<TickerMini>),
    MiniVec(Box<Vec<TickerMini>>),
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TickerFull {
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
pub struct TickerMini {
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
