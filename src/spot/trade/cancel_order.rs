use jiff::Timestamp;
use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{
    enums::{
        CancelRestriction, OrderSide, OrderType, SecurityType, SelfTradePreventionMode, TimeInForce,
    },
    rest_api::endpoint,
};

endpoint!(
    "/api/v3/order",
    Method::DELETE,
    SecurityType::Trade,
    CancelOrderEndpoint,
    CancelOrderParams,
    CancelOrderResponse
);

/// Cancel an active order.
///
/// - Weight: 1
/// - Data Source: Matching Engine
pub struct CancelOrderEndpoint<'r> {
    client: &'r crate::rest_api::RestApiClient,
}

impl<'r> CancelOrderEndpoint<'r> {
    pub fn new(client: &'r crate::rest_api::RestApiClient) -> Self {
        Self { client }
    }
}

/// Either `order_id` or `orig_client_order_id` must be sent. If both parameters
/// are sent, `order_id` takes precedence.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelOrderParams {
    symbol: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    order_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    orig_client_order_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    new_client_order_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cancel_restrictions: Option<CancelRestriction>,
    #[serde(skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
    timestamp: i64,
}

impl CancelOrderParams {
    pub fn new(symbol: &str) -> Self {
        Self {
            symbol: symbol.to_owned(),
            order_id: None,
            orig_client_order_id: None,
            new_client_order_id: None,
            cancel_restrictions: None,
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

    /// Used to uniquely identify this cancel. Automatically generated by
    /// default.
    pub fn new_client_order_id(mut self, new_client_order_id: &str) -> Self {
        self.new_client_order_id = Some(new_client_order_id.to_owned());
        self
    }

    pub fn cancel_restrictions(mut self, cancel_restrictions: CancelRestriction) -> Self {
        self.cancel_restrictions = Some(cancel_restrictions);
        self
    }

    /// The value cannot be greater than 60000.
    pub fn recv_window(mut self, recv_window: i64) -> Self {
        self.recv_window = Some(recv_window);
        self
    }
}

pub type CancelOrderResponse = CancelOrderResult;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelOrderResult {
    pub symbol: String,
    pub orig_client_order_id: String,
    pub order_id: i64,
    pub order_list_id: i64,
    pub client_order_id: String,
    pub transact_time: i64,
    pub price: String,
    pub orig_qty: String,
    pub executed_qty: String,
    pub cummulative_quote_qty: String,
    pub status: String,
    pub time_in_force: TimeInForce,
    pub r#type: OrderType,
    pub side: OrderSide,
    pub self_trade_prevention_mode: SelfTradePreventionMode,
}
