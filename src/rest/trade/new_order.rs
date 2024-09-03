use jiff::Timestamp;
use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{
    enums::{OrderResponseType, OrderSide, OrderType, PreventionMode, TimeInForce},
    models::{NewOrderAck, NewOrderFull, NewOrderResult},
    rest::{endpoint, SecurityType},
};

endpoint!(
    "/api/v3/order",
    Method::POST,
    SecurityType::Trade,
    NewOrderEndpoint,
    NewOrderParams,
    NewOrderResponse
);

/// Send in a new order.
///
/// - Weight: 1
/// - Data Source: Matching Engine
pub struct NewOrderEndpoint<'r> {
    client: &'r crate::rest::RestClient,
}

impl<'r> NewOrderEndpoint<'r> {
    pub fn new(client: &'r crate::rest::RestClient) -> Self {
        Self { client }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NewOrderParams {
    symbol: String,
    side: OrderSide,
    r#type: OrderType,
    #[serde(skip_serializing_if = "Option::is_none")]
    time_in_force: Option<TimeInForce>,
    #[serde(skip_serializing_if = "Option::is_none")]
    quantity: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    quote_order_qty: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    price: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    new_client_order_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    strategy_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    strategy_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    stop_price: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    trailing_delta: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    iceberg_qty: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    new_order_resp_type: Option<OrderResponseType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    self_trade_prevention_mode: Option<PreventionMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
    timestamp: i64,
}

impl NewOrderParams {
    pub fn new(symbol: &str, side: OrderSide, r#type: OrderType) -> Self {
        Self {
            symbol: symbol.to_owned(),
            side,
            r#type,
            time_in_force: None,
            quantity: None,
            quote_order_qty: None,
            price: None,
            new_client_order_id: None,
            strategy_id: None,
            strategy_type: None,
            stop_price: None,
            trailing_delta: None,
            iceberg_qty: None,
            new_order_resp_type: None,
            self_trade_prevention_mode: None,
            recv_window: None,
            timestamp: Timestamp::now().as_millisecond(),
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

    pub fn quote_order_qty(mut self, quote_order_qty: f64) -> Self {
        self.quote_order_qty = Some(quote_order_qty);
        self
    }

    pub fn price(mut self, price: f64) -> Self {
        self.price = Some(price);
        self
    }

    /// A unique id among open orders. Automatically generated if not sent.
    /// Orders with the same `newClientOrderID` can be accepted only when the
    /// previous one is filled, otherwise the order will be rejected.
    pub fn new_client_order_id(mut self, new_client_order_id: &str) -> Self {
        self.new_client_order_id = Some(new_client_order_id.to_owned());
        self
    }

    pub fn strategy_id(mut self, strategy_id: i64) -> Self {
        self.strategy_id = Some(strategy_id);
        self
    }

    /// The value cannot be less than 1000000.
    pub fn strategy_type(mut self, strategy_type: &str) -> Self {
        self.strategy_type = Some(strategy_type.to_owned());
        self
    }

    /// Used with STOP_LOSS, STOP_LOSS_LIMIT, TAKE_PROFIT, and
    /// TAKE_PROFIT_LIMIT orders.
    pub fn stop_price(mut self, stop_price: f64) -> Self {
        self.stop_price = Some(stop_price);
        self
    }

    /// Used with STOP_LOSS, STOP_LOSS_LIMIT, TAKE_PROFIT, and
    /// TAKE_PROFIT_LIMIT orders.
    pub fn trailing_delta(mut self, trailing_delta: f64) -> Self {
        self.trailing_delta = Some(trailing_delta);
        self
    }

    /// Used with LIMIT, STOP_LOSS_LIMIT, and TAKE_PROFIT_LIMIT to create an
    /// iceberg order.
    pub fn iceberg_qty(mut self, iceberg_qty: f64) -> Self {
        self.iceberg_qty = Some(iceberg_qty);
        self
    }

    /// Set the response JSON. ACK, RESULT, or FULL; MARKET and LIMIT order types
    /// default to FULL, all other orders default to ACK.
    pub fn new_order_resp_type(mut self, new_order_resp_type: OrderResponseType) -> Self {
        self.new_order_resp_type = Some(new_order_resp_type);
        self
    }

    /// The allowed enums is dependent on what is configured on the symbol. The
    /// possible supported values are EXPIRE_TAKER, EXPIRE_MAKER, EXPIRE_BOTH,
    /// NONE.
    pub fn self_trade_prevention_mode(
        mut self,
        self_trade_prevention_mode: PreventionMode,
    ) -> Self {
        self.self_trade_prevention_mode = Some(self_trade_prevention_mode);
        self
    }

    /// The value cannot be greater than 60000.
    pub fn recv_window(mut self, recv_window: i64) -> Self {
        self.recv_window = Some(recv_window);
        self
    }
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum NewOrderResponse {
    Ack(Box<NewOrderAck>),
    Result(Box<NewOrderResult>),
    Full(Box<NewOrderFull>),
}
