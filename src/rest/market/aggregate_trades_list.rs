use reqwest::Method;
use serde::Serialize;

use crate::{models::AggTrade, rest::endpoint};

endpoint!(
    "/api/v3/aggTrades",
    Method::GET,
    AggregateTradesListEndpoint,
    AggregateTradesListParams,
    AggregateTradesListResponse
);

/// Get compressed, aggregate trades. Trades that fill at the time, from the
/// same taker order, with the same price will have the quantity aggregated.
///
/// - Weight: 2
/// - Data Source: Database
pub struct AggregateTradesListEndpoint<'r> {
    client: &'r crate::rest::RestClient,
}

impl<'r> AggregateTradesListEndpoint<'r> {
    pub fn new(client: &'r crate::rest::RestClient) -> Self {
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

    /// ID to get aggregate trades from (inclusive).
    pub fn from_id(mut self, from_id: i64) -> Self {
        self.from_id = Some(from_id);
        self
    }

    /// Timestamp in ms to get aggregate trades from (inclusive).
    pub fn start_time(mut self, start_time: i64) -> Self {
        self.start_time = Some(start_time);
        self
    }

    /// Timestamp in ms to get aggregate trades until (inclusive).
    pub fn end_time(mut self, end_time: i64) -> Self {
        self.end_time = Some(end_time);
        self
    }

    /// Default 500; max 1000.
    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }
}

pub type AggregateTradesListResponse = Vec<AggTrade>;
