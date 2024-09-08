use jiff::Timestamp;
use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{enums::SecurityType, rest_api::endpoint};

endpoint!(
    "/sapi/v1/margin/myTrades",
    Method::GET,
    SecurityType::UserData,
    QueryTradeListEndpoint,
    QueryTradeListParams,
    QueryTradeListResponse
);

/// Query Margin Account's Trade List.
///
/// - Weight: 10
pub struct QueryTradeListEndpoint<'r> {
    client: &'r crate::rest_api::RestApiClient,
}

impl<'r> QueryTradeListEndpoint<'r> {
    pub fn new(client: &'r crate::rest_api::RestApiClient) -> Self {
        Self { client }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct QueryTradeListParams {
    symbol: String,
    is_isolated: Option<String>,
    order_id: Option<i64>,
    start_time: Option<i64>,
    end_time: Option<i64>,
    from_id: Option<i64>,
    limit: Option<i64>,
    recv_window: Option<i64>,
    timestamp: i64,
}

impl QueryTradeListParams {
    pub fn new(symbol: &str) -> Self {
        Self {
            symbol: symbol.to_owned(),
            is_isolated: None,
            order_id: None,
            start_time: None,
            end_time: None,
            from_id: None,
            limit: None,
            recv_window: None,
            timestamp: Timestamp::now().as_millisecond(),
        }
    }

    pub fn is_isolated(mut self, is_isolated: &str) -> Self {
        self.is_isolated = Some(is_isolated.to_owned());
        self
    }

    pub fn order_id(mut self, order_id: i64) -> Self {
        self.order_id = Some(order_id);
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

    pub fn from_id(mut self, from_id: i64) -> Self {
        self.from_id = Some(from_id);
        self
    }

    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }

    pub fn recv_window(mut self, recv_window: i64) -> Self {
        self.recv_window = Some(recv_window);
        self
    }
}

pub type QueryTradeListResponse = Vec<Trade>;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Trade {
    pub commission: String,
    pub commission_asset: String,
    pub id: i64,
    pub is_best_match: bool,
    pub is_buyer: bool,
    pub is_maker: bool,
    pub order_id: i64,
    pub price: String,
    pub qty: String,
    pub symbol: String,
    pub is_isolated: bool,
    pub time: i64,
}
