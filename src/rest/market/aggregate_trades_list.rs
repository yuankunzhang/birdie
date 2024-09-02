use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{models::AggTrade, rest::endpoint};

endpoint!(
    "/api/v3/aggTrades",
    Method::GET,
    AggregateTradesListEndpoint,
    AggregateTradesListParams,
    AggregateTradesListResponse
);

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AggregateTradesListParams {
    symbol: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    from_id: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    start_time: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    end_time: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<u64>,
}

impl AggregateTradesListParams {
    pub fn new(symbol: &str) -> Self {
        Self {
            symbol: symbol.to_string(),
            from_id: None,
            start_time: None,
            end_time: None,
            limit: None,
        }
    }

    pub fn from_id(mut self, from_id: u64) -> Self {
        self.from_id = Some(from_id);
        self
    }

    pub fn start_time(mut self, start_time: u64) -> Self {
        self.start_time = Some(start_time);
        self
    }

    pub fn end_time(mut self, end_time: u64) -> Self {
        self.end_time = Some(end_time);
        self
    }

    pub fn limit(mut self, limit: u64) -> Self {
        self.limit = Some(limit);
        self
    }
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum AggregateTradesListResponse {
    Trades(Vec<AggTrade>),
}
