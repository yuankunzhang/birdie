use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{models::Trade, rest::endpoint};

endpoint!(
    "/api/v3/trades",
    Method::GET,
    RecentTradesListEndpoint,
    RecentTradesListParams,
    RecentTradesListResponse
);

/// Get recent trades.
///
/// - Weight: 25
/// - Data Source: Memory
pub struct RecentTradesListEndpoint<'r> {
    client: &'r crate::rest::RestClient,
}

impl<'r> RecentTradesListEndpoint<'r> {
    pub fn new(client: &'r crate::rest::RestClient) -> Self {
        Self { client }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RecentTradesListParams {
    symbol: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<u64>,
}

impl RecentTradesListParams {
    pub fn new(symbol: &str) -> Self {
        Self {
            symbol: symbol.to_string(),
            limit: None,
        }
    }

    pub fn limit(mut self, limit: u64) -> Self {
        self.limit = Some(limit);
        self
    }
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum RecentTradesListResponse {
    Trades(Vec<Trade>),
}
