use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{enums::KlineInterval, rest_api::endpoint};

endpoint!(
    "/fapi/v1/klines",
    Method::GET,
    KlinesEndpoint,
    KlinesParams,
    KlinesResponse
);

/// Kline/candlestick bars for a symbol. Klines are uniquely identified by their
/// open time.
///
/// - Weight:
///   - limit [1,100): 1
///   - limit [100,500): 2
///   - limit [500,1000): 5
///   - limit [1000,): 10
pub struct KlinesEndpoint<'r> {
    client: &'r crate::rest_api::RestApiClient,
}

impl<'r> KlinesEndpoint<'r> {
    pub fn new(client: &'r crate::rest_api::RestApiClient) -> Self {
        Self { client }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct KlinesParams {
    symbol: String,
    interval: KlineInterval,
    #[serde(skip_serializing_if = "Option::is_none")]
    start_time: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    end_time: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    time_zone: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<i64>,
}

impl KlinesParams {
    pub fn new(symbol: &str, interval: KlineInterval) -> Self {
        Self {
            symbol: symbol.to_owned(),
            interval,
            start_time: None,
            end_time: None,
            time_zone: None,
            limit: None,
        }
    }

    pub fn start_time(mut self, start_time: i64) -> Self {
        self.start_time = Some(start_time);
        self
    }

    pub fn end_time(mut self, end_time: i64) -> Self {
        self.end_time = Some(end_time);
        self
    }

    /// Default: 0 (UTC).
    pub fn time_zone(mut self, time_zone: &str) -> Self {
        self.time_zone = Some(time_zone.to_owned());
        self
    }

    /// Default 500; max 1000.
    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }
}

pub type KlinesResponse = Vec<Kline>;

#[derive(Debug, Deserialize)]
pub struct Kline(
    pub i64,    // Open time
    pub String, // Open price
    pub String, // High price
    pub String, // Low price
    pub String, // Close price
    pub String, // Volume
    pub i64,    // Close time
    pub String, // Quote asset volume
    pub i64,    // Number of trades
    pub String, // Taker buy base asset volume
    pub String, // Taker buy quote asset volume
    pub String, // Unused field, ignore.
);
