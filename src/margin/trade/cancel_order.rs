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
    "/sapi/v1/margin/order",
    Method::DELETE,
    SecurityType::Trade,
    CancelOrderEndpoint,
    CancelOrderParams,
    CancelOrderResponse
);

/// Cancel an active order for margin account.
///
/// - Weight: 10
pub struct CancelOrderEndpoint<'r> {
    client: &'r crate::rest_api::RestApiClient,
}

impl<'r> CancelOrderEndpoint<'r> {
    pub fn new(client: &'r crate::rest_api::RestApiClient) -> Self {
        Self { client }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelOrderParams {
    symbol: String,
    isolated: Option<String>,
    order_id: Option<i64>,
    list_client_order_id: Option<String>,
    new_client_order_id: Option<String>,
    recv_window: Option<i64>,
    timestamp: i64,
}

impl CancelOrderParams {
    pub fn new(symbol: &str) -> Self {
        Self {
            symbol: symbol.to_owned(),
            isolated: None,
            order_id: None,
            list_client_order_id: None,
            new_client_order_id: None,
            recv_window: None,
            timestamp: Timestamp::now().as_millisecond(),
        }
    }

    pub fn isolated(mut self, isolated: &str) -> Self {
        self.isolated = Some(isolated.to_owned());
        self
    }

    pub fn order_id(mut self, order_id: i64) -> Self {
        self.order_id = Some(order_id);
        self
    }

    pub fn list_client_order_id(mut self, list_client_order_id: &str) -> Self {
        self.list_client_order_id = Some(list_client_order_id.to_owned());
        self
    }

    pub fn new_client_order_id(mut self, new_client_order_id: &str) -> Self {
        self.new_client_order_id = Some(new_client_order_id.to_owned());
        self
    }

    pub fn recv_window(mut self, recv_window: i64) -> Self {
        self.recv_window = Some(recv_window);
        self
    }
}

pub type CancelOrderResponse = CancelMarginOrderDetail;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelMarginOrderDetail {
    pub symbol: String,
    pub is_isolated: bool,
    pub order_id: i64,
    pub orig_client_order_id: String,
    pub client_order_id: String,
    pub price: String,
    pub orig_qty: String,
    pub executed_qty: String,
    pub cummulative_quote_qty: String,
    pub status: OrderStatus,
    pub time_in_force: TimeInForce,
    pub r#type: OrderType,
    pub side: OrderSide,
    pub self_trade_prevention_mode: Option<SelfTradePreventionMode>,
}
