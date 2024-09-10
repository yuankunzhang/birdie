use jiff::Timestamp;
use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{
    enums::{
        futures::{OrderSide, OrderStatus, PositionSide, ResponseType, TimeInForce, WorkingType},
        OrderType, SecurityType, SelfTradePreventionMode,
    },
    rest_api::endpoint,
};

endpoint!(
    "/fapi/v1/order",
    Method::POST,
    SecurityType::Trade,
    NewOrderEndpoint,
    NewOrderParams,
    NewOrderResponse
);

/// Send in a new order.
///
/// - Weight:
pub struct NewOrderEndpoint<'r> {
    client: &'r crate::rest_api::RestApiClient,
}

impl<'r> NewOrderEndpoint<'r> {
    pub fn new(client: &'r crate::rest_api::RestApiClient) -> Self {
        Self { client }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NewOrderInput {
    symbol: String,
    side: OrderSide,
    position_side: Option<String>,
    r#type: OrderType,
    time_in_force: Option<TimeInForce>,
    quantity: Option<f64>,
    reduce_only: Option<String>,
    price: Option<f64>,
    new_client_order_id: Option<String>,
    stop_price: Option<f64>,
    close_position: Option<String>,
    activation_price: Option<f64>,
    callback_rate: Option<f64>,
    working_type: Option<WorkingType>,
    price_protect: Option<String>,
    new_order_resp_type: Option<ResponseType>,
    self_trade_prevention_mode: Option<SelfTradePreventionMode>,
    good_till_date: Option<i64>,
}

impl NewOrderInput {
    pub fn new(symbol: &str, side: OrderSide, position_side: String, r#type: OrderType) -> Self {
        Self {
            symbol: symbol.to_owned(),
            side,
            position_side: Some(position_side),
            r#type,
            time_in_force: None,
            quantity: None,
            reduce_only: None,
            price: None,
            new_client_order_id: None,
            stop_price: None,
            close_position: None,
            activation_price: None,
            callback_rate: None,
            working_type: None,
            price_protect: None,
            new_order_resp_type: None,
            self_trade_prevention_mode: None,
            good_till_date: None,
        }
    }

    pub fn time_in_force(mut self, time_in_force: TimeInForce) -> Self {
        self.time_in_force = Some(time_in_force);
        self
    }

    pub fn quantity(mut self, quantity: f64) -> Self {
        self.quantity = Some(quantity);
        self
    }

    pub fn reduce_only(mut self, reduce_only: bool) -> Self {
        self.reduce_only = Some(reduce_only.to_string());
        self
    }

    pub fn price(mut self, price: f64) -> Self {
        self.price = Some(price);
        self
    }

    pub fn new_client_order_id(mut self, new_client_order_id: &str) -> Self {
        self.new_client_order_id = Some(new_client_order_id.to_owned());
        self
    }

    pub fn stop_price(mut self, stop_price: f64) -> Self {
        self.stop_price = Some(stop_price);
        self
    }

    pub fn close_position(mut self, close_position: bool) -> Self {
        self.close_position = Some(close_position.to_string());
        self
    }

    pub fn activation_price(mut self, activation_price: f64) -> Self {
        self.activation_price = Some(activation_price);
        self
    }

    pub fn callback_rate(mut self, callback_rate: f64) -> Self {
        self.callback_rate = Some(callback_rate);
        self
    }

    pub fn working_type(mut self, working_type: WorkingType) -> Self {
        self.working_type = Some(working_type);
        self
    }

    pub fn price_protect(mut self, price_protect: bool) -> Self {
        self.price_protect = Some(price_protect.to_string());
        self
    }

    pub fn new_order_resp_type(mut self, new_order_resp_type: ResponseType) -> Self {
        self.new_order_resp_type = Some(new_order_resp_type);
        self
    }

    pub fn self_trade_prevention_mode(
        mut self,
        self_trade_prevention_mode: SelfTradePreventionMode,
    ) -> Self {
        self.self_trade_prevention_mode = Some(self_trade_prevention_mode);
        self
    }

    pub fn good_till_date(mut self, good_till_date: i64) -> Self {
        self.good_till_date = Some(good_till_date);
        self
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NewOrderParams {
    #[serde(flatten)]
    input: NewOrderInput,
    recv_window: Option<i64>,
    timestamp: i64,
}

impl NewOrderParams {
    pub fn new(input: NewOrderInput) -> Self {
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

pub type NewOrderResponse = NewOrderDetail;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NewOrderDetail {
    pub client_order_id: String,
    pub cum_qty: String,
    pub cum_quote: String,
    pub executed_qty: String,
    pub order_id: i64,
    pub avg_price: String,
    pub orig_qty: String,
    pub price: String,
    pub reduce_only: bool,
    pub side: OrderSide,
    pub position_side: PositionSide,
    pub status: OrderStatus,
    pub stop_price: String,
    pub close_position: bool,
    pub symbol: String,
    pub time_in_force: TimeInForce,
    pub r#type: OrderType,
    pub orig_type: OrderType,
    pub activate_price: String,
    pub price_rate: String,
    pub update_time: i64,
    pub working_type: WorkingType,
    pub price_protect: bool,
    pub price_match: String,
    pub self_trade_prevention_mode: SelfTradePreventionMode,
    pub good_till_date: i64,
}
