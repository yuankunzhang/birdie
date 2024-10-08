use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::rest_api::endpoint;

endpoint!(
    "/fapi/v1/aggTrades",
    Method::GET,
    AggregateTradesListEndpoint,
    AggregateTradesListParams,
    AggregateTradesListResponse
);

/// Get compressed, aggregate trades. Trades that fill at the time, from the
/// same taker order, with the same price will have the quantity aggregated.
///
/// - Weight: 20
pub struct AggregateTradesListEndpoint<'r> {
    client: &'r crate::rest_api::RestApiClient,
}

impl<'r> AggregateTradesListEndpoint<'r> {
    pub fn new(client: &'r crate::rest_api::RestApiClient) -> Self {
        Self { client }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AggregateTradesListParams {
    symbol: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    from_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    start_time: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    end_time: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<i64>,
}

impl AggregateTradesListParams {
    pub fn new(symbol: &str) -> Self {
        Self {
            symbol: symbol.to_owned(),
            from_id: None,
            start_time: None,
            end_time: None,
            limit: None,
        }
    }

    pub fn from_id(mut self, from_id: i64) -> Self {
        self.from_id = Some(from_id);
        self
    }

    pub fn start_time(mut self, start_time: i64) -> Self {
        self.start_time = Some(start_time);
        self
    }

    pub fn end_time(mut self, end_time: i64) -> Self {
        self.end_time = Some(end_time);
        self
    }

    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }
}

pub type AggregateTradesListResponse = Vec<AggTrade>;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AggTrade {
    #[serde(rename = "a")]
    pub aggregated: i64,
    #[serde(rename = "p")]
    pub price: String,
    #[serde(rename = "q")]
    pub quantity: String,
    #[serde(rename = "f")]
    pub first_trade_id: i64,
    #[serde(rename = "l")]
    pub last_trade_id: i64,
    #[serde(rename = "T")]
    pub time: i64,
    #[serde(rename = "m")]
    pub is_buyer_maker: bool,
}
