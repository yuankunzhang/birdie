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

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OldTradeLookupParams {
    symbol: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    from_id: Option<u64>,
}

impl OldTradeLookupParams {
    pub fn new(symbol: &str) -> Self {
        Self {
            symbol: symbol.to_string(),
            limit: None,
            from_id: None,
        }
    }

    pub fn limit(mut self, limit: u64) -> Self {
        self.limit = Some(limit);
        self
    }

    pub fn from_id(mut self, from_id: u64) -> Self {
        self.from_id = Some(from_id);
        self
    }
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum OldTradeLookupResponse {
    Trades(Vec<Trade>),
}
