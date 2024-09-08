use jiff::Timestamp;
use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{
    enums::{
        OrderSide, OrderStatus, OrderType, SecurityType, SelfTradePreventionMode, TimeInForce,
    },
    rest_api::endpoint,
};

endpoint!(
    "/sapi/v1/margin/allOrders",
    Method::GET,
    SecurityType::UserData,
    QueryAllOrdersEndpoint,
    QueryAllOrdersParams,
    QueryAllOrdersResponse
);

/// Query Margin Account's All Orders.
///
/// - Weight: 200
pub struct QueryAllOrdersEndpoint<'r> {
    client: &'r crate::rest_api::RestApiClient,
}

impl<'r> QueryAllOrdersEndpoint<'r> {
    pub fn new(client: &'r crate::rest_api::RestApiClient) -> Self {
        Self { client }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct QueryAllOrdersParams {
    symbol: Option<String>,
    is_isolated: Option<String>,
    start_time: Option<i64>,
    end_time: Option<i64>,
    limit: Option<i64>,
    recv_window: Option<i64>,
    timestamp: i64,
}

impl QueryAllOrdersParams {
    pub fn new() -> Self {
        Self {
            symbol: None,
            is_isolated: None,
            start_time: None,
            end_time: None,
            limit: None,
            recv_window: None,
            timestamp: Timestamp::now().as_millisecond(),
        }
    }

    pub fn symbol(mut self, symbol: &str) -> Self {
        self.symbol = Some(symbol.to_owned());
        self
    }

    pub fn is_isolated(mut self, is_isolated: &str) -> Self {
        self.is_isolated = Some(is_isolated.to_owned());
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

    pub fn recv_window(mut self, recv_window: i64) -> Self {
        self.recv_window = Some(recv_window);
        self
    }
}

pub type QueryAllOrdersResponse = Vec<OrderResult>;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderResult {
    pub client_order_id: String,
    pub cummulative_quote_qty: String,
    pub executed_qty: String,
    pub iceberg_qty: String,
    pub is_working: bool,
    pub order_id: i64,
    pub orig_qty: String,
    pub price: String,
    pub side: OrderSide,
    pub status: OrderStatus,
    pub stop_price: String,
    pub symbol: String,
    pub is_isolated: bool,
    pub time: i64,
    pub time_in_force: TimeInForce,
    pub r#type: OrderType,
    pub self_trade_prevention_mode: SelfTradePreventionMode,
    pub update_time: i64,
}
