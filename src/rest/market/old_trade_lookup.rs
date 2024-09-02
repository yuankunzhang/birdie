use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{models::Trade, rest::endpoint};

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

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum OldTradeLookupResponse {
    Trades(Vec<Trade>),
}
