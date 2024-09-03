use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::rest::endpoint;

endpoint!(
    "/api/v3/historicalTrades",
    Method::GET,
    OldTradeLookupEndpoint,
    OldTradeLookupParams,
    OldTradeLookupResponse
);

/// Get older trades.
///
/// - Weight: 25
/// - Data Source: Database
pub struct OldTradeLookupEndpoint<'r> {
    client: &'r crate::rest::RestClient,
}

impl<'r> OldTradeLookupEndpoint<'r> {
    pub fn new(client: &'r crate::rest::RestClient) -> Self {
        Self { client }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OldTradeLookupParams {
    symbol: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    from_id: Option<i64>,
}

impl OldTradeLookupParams {
    pub fn new(symbol: &str) -> Self {
        Self {
            symbol: symbol.to_string(),
            limit: None,
            from_id: None,
        }
    }

    /// Default 500; max 1000.
    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }

    /// TradeId to fetch from. Default gets most recent trades.
    pub fn from_id(mut self, from_id: i64) -> Self {
        self.from_id = Some(from_id);
        self
    }
}

pub type OldTradeLookupResponse = Vec<OldTrade>;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OldTrade {
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
