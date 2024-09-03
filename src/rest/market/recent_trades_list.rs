use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::rest::endpoint;

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
    limit: Option<i64>,
}

impl RecentTradesListParams {
    pub fn new(symbol: &str) -> Self {
        Self {
            symbol: symbol.to_string(),
            limit: None,
        }
    }

    /// Default 500; max 1000.
    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }
}

pub type RecentTradesListResponse = Vec<RecentTrade>;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RecentTrade {
    /// Example: `345196462`
    pub id: i64,
    /// Example: `"9638.99000000"`
    pub price: String,
    /// Example: `"0.02077200"`
    pub qty: String,
    /// Example: `"0.02077200"`
    pub quote_qty: String,
    /// Example: `1592887772684`
    pub time: i64,
    pub is_buyer_maker: bool,
    pub is_best_match: bool,
}
