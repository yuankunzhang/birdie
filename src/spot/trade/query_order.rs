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
    "/api/v3/order",
    Method::GET,
    SecurityType::UserData,
    QueryOrderEndpoint,
    QueryOrderParams,
    QueryOrderResponse
);

/// Check an order's status.
///
/// - Weight: 4
/// - Data Source: Memory => Database
pub struct QueryOrderEndpoint<'r> {
    client: &'r crate::rest_api::RestApiClient,
}

impl<'r> QueryOrderEndpoint<'r> {
    pub fn new(client: &'r crate::rest_api::RestApiClient) -> Self {
        Self { client }
    }
}

/// Notes:
///
/// - Either `order_id` or `orig_client_order_id` must be sent.
/// - For some historical orders `cummulative_quote_qty` will be < 0, meaning
///   the data is not available at this time.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct QueryOrderParams {
    symbol: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    order_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    orig_client_order_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
    timestamp: i64,
}

impl QueryOrderParams {
    pub fn new(symbol: &str) -> Self {
        Self {
            symbol: symbol.to_owned(),
            order_id: None,
            orig_client_order_id: None,
            recv_window: None,
            timestamp: Timestamp::now().as_millisecond(),
        }
    }

    pub fn order_id(mut self, order_id: i64) -> Self {
        self.order_id = Some(order_id);
        self
    }

    pub fn orig_client_order_id(mut self, orig_client_order_id: &str) -> Self {
        self.orig_client_order_id = Some(orig_client_order_id.to_owned());
        self
    }

    /// The value cannot be greater than 60000.
    pub fn recv_window(mut self, recv_window: i64) -> Self {
        self.recv_window = Some(recv_window);
        self
    }
}

pub type QueryOrderResponse = OrderDetail;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderDetail {
    pub symbol: String,
    pub order_id: i64,
    pub order_list_id: i64,
    pub client_order_id: String,
    pub price: String,
    pub orig_qty: String,
    pub executed_qty: String,
    pub cummulative_quote_qty: String,
    pub status: OrderStatus,
    pub time_in_force: TimeInForce,
    pub r#type: OrderType,
    pub side: OrderSide,
    pub stop_price: String,
    pub iceberg_qty: String,
    pub time: i64,
    pub update_time: i64,
    pub is_working: bool,
    pub working_time: i64,
    pub orig_quote_order_qty: String,
    pub self_trade_prevention_mode: SelfTradePreventionMode,
}
