use jiff::Timestamp;
use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{enums::SecurityType, rest_api::endpoint};

endpoint!(
    "/api/v3/myPreventedMatches",
    Method::GET,
    SecurityType::UserData,
    QueryPreventedMatchesEndpoint,
    QueryPreventedMatchesParams,
    QueryPreventedMatchesResponse
);

/// Displays the list of orders that were expired due to STP.
///
/// - Weight:
///     - If `symbol` is invalid: 2
///     - Querying by `preventedMatchId`: 2
///     - Query by `orderId`: 20
/// - Data Source: Database
pub struct QueryPreventedMatchesEndpoint<'r> {
    client: &'r crate::rest_api::RestApiClient,
}

impl<'r> QueryPreventedMatchesEndpoint<'r> {
    pub fn new(client: &'r crate::rest_api::RestApiClient) -> Self {
        Self { client }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct QueryPreventedMatchesParams {
    symbol: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    prevented_match_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    order_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    from_prevented_match_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
    timestamp: i64,
}

impl QueryPreventedMatchesParams {
    pub fn new(symbol: &str) -> Self {
        Self {
            symbol: symbol.to_owned(),
            prevented_match_id: None,
            order_id: None,
            from_prevented_match_id: None,
            limit: None,
            recv_window: None,
            timestamp: Timestamp::now().as_millisecond(),
        }
    }

    pub fn prevented_match_id(mut self, prevented_match_id: i64) -> Self {
        self.prevented_match_id = Some(prevented_match_id);
        self
    }

    pub fn order_id(mut self, order_id: i64) -> Self {
        self.order_id = Some(order_id);
        self
    }

    pub fn from_prevented_match_id(mut self, from_prevented_match_id: i64) -> Self {
        self.from_prevented_match_id = Some(from_prevented_match_id);
        self
    }

    /// Default: 500; Max: 1000.
    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }

    /// The value cannot be greater than 60000.
    pub fn recv_window(mut self, recv_window: i64) -> Self {
        self.recv_window = Some(recv_window);
        self
    }
}

pub type QueryPreventedMatchesResponse = Vec<PreventedMatch>;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PreventedMatch {
    pub symbol: String,
    pub prevented_match_id: i64,
    pub taker_order_id: i64,
    pub maker_symbol: String,
    pub maker_order_id: i64,
    pub trade_group_id: i64,
    pub self_trade_prevention_mode: String,
    pub price: String,
    pub maker_prevented_quantity: String,
    pub transact_time: i64,
}
