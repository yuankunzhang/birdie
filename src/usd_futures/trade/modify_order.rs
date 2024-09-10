use jiff::Timestamp;
use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{
    enums::{
        futures::{OrderSide, OrderStatus, PositionSide, TimeInForce, WorkingType},
        OrderType, SecurityType, SelfTradePreventionMode,
    },
    rest_api::endpoint,
};

endpoint!(
    "/fapi/v1/order",
    Method::PUT,
    SecurityType::Trade,
    ModifyOrderEndpoint,
    ModifyOrderParams,
    ModifyOrderResponse
);

/// Order modify function, currently only LIMIT order modification is supported,
/// modified orders will be reordered in the match queue.
///
/// - Weight:
pub struct ModifyOrderEndpoint<'r> {
    client: &'r crate::rest_api::RestApiClient,
}

impl<'r> ModifyOrderEndpoint<'r> {
    pub fn new(client: &'r crate::rest_api::RestApiClient) -> Self {
        Self { client }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ModifyOrderInput {
    order_id: Option<i64>,
    orig_client_order_id: Option<String>,
    symbol: String,
    side: OrderSide,
    quantity: f64,
    price: f64,
    price_match: Option<String>,
}

impl ModifyOrderInput {
    pub fn new(symbol: &str, side: OrderSide, quantity: f64, price: f64) -> Self {
        Self {
            order_id: None,
            orig_client_order_id: None,
            symbol: symbol.to_owned(),
            side,
            quantity,
            price,
            price_match: None,
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

    pub fn price_match(mut self, price_match: &str) -> Self {
        self.price_match = Some(price_match.to_owned());
        self
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ModifyOrderParams {
    #[serde(flatten)]
    input: ModifyOrderInput,
    recv_window: Option<i64>,
    timestamp: i64,
}

impl ModifyOrderParams {
    pub fn new(input: ModifyOrderInput) -> Self {
        Self {
            input,
            recv_window: None,
            timestamp: Timestamp::now().as_millisecond(),
        }
    }

    pub fn recv_window(mut self, recv_window: i64) -> Self {
        self.recv_window = Some(recv_window);
        self
    }
}

pub type ModifyOrderResponse = ModifyOrderDetail;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ModifyOrderDetail {
    pub order_id: i64,
    pub symbol: String,
    pub pair: String,
    pub status: OrderStatus,
    pub client_order_id: String,
    pub price: String,
    pub avg_price: String,
    pub orig_qty: String,
    pub executed_qty: String,
    pub cum_qty: String,
    pub cum_base: String,
    pub time_in_force: TimeInForce,
    pub r#type: OrderType,
    pub reduce_only: bool,
    pub close_position: bool,
    pub side: OrderSide,
    pub position_side: PositionSide,
    pub stop_price: String,
    pub working_type: WorkingType,
    pub price_protect: bool,
    pub orig_type: OrderType,
    pub price_match: String,
    pub self_trade_prevention_mode: SelfTradePreventionMode,
    pub good_till_date: i64,
    pub update_time: i64,
}
