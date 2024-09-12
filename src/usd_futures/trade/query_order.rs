use jiff::Timestamp;
use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{
    enums::{
        futures::{OrderSide, OrderStatus, OrderType, PositionSide, WorkingType},
        SecurityType, SelfTradePreventionMode,
    },
    rest_api::endpoint,
};

endpoint!(
    "/fapi/v1/order",
    Method::GET,
    SecurityType::UserData,
    QueryOrderEndpoint,
    QueryOrderParams,
    QueryOrderResponse
);

/// Check an order's status.
///
/// - Weight: 1
pub struct QueryOrderEndpoint<'r> {
    client: &'r crate::rest_api::RestApiClient,
}

impl<'r> QueryOrderEndpoint<'r> {
    pub fn new(client: &'r crate::rest_api::RestApiClient) -> Self {
        Self { client }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct QueryOrderParams {
    symbol: String,
    order_id: Option<i64>,
    orig_client_order_id: Option<String>,
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

    pub fn recv_window(mut self, recv_window: i64) -> Self {
        self.recv_window = Some(recv_window);
        self
    }
}

pub type QueryOrderResponse = OrderDetail;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderDetail {
    pub avg_price: String,
    pub client_order_id: String,
    pub cum_quote: String,
    pub executed_qty: String,
    pub order_id: i64,
    pub orig_qty: String,
    pub orig_type: String,
    pub price: String,
    pub reduce_only: bool,
    pub side: OrderSide,
    pub position_side: PositionSide,
    pub status: OrderStatus,
    pub stop_price: String,
    pub close_position: bool,
    pub symbol: String,
    pub time: i64,
    pub time_in_force: String,
    pub r#type: OrderType,
    pub activate_price: String,
    pub price_rate: String,
    pub update_time: i64,
    pub working_type: WorkingType,
    pub price_protect: bool,
    pub price_match: String,
    pub self_trade_prevention_mode: SelfTradePreventionMode,
    pub good_till_date: i64,
}
