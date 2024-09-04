use jiff::Timestamp;
use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{
    enums::{NewOrderRespType, OcoOrderType, OrderSide, SelfTradePreventionMode},
    rest_api::{endpoint, SecurityType},
};

endpoint!(
    "/api/v3/orderList/oco",
    Method::POST,
    SecurityType::Trade,
    NewOrderListOcoEndpoint,
    NewOrderListOcoParams,
    NewOrderListOcoResponse
);

/// Send in an one-cancels-the-other (OCO) pair, where activation of one order
/// immediately cancels the other.
///
/// - Weight: 1
/// - Data Source: Matching Engine
pub struct NewOrderListOcoEndpoint<'r> {
    client: &'r crate::rest_api::RestApiClient,
}

impl<'r> NewOrderListOcoEndpoint<'r> {
    pub fn new(client: &'r crate::rest_api::RestApiClient) -> Self {
        Self { client }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NewOrderListOcoParams {
    symbol: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    list_client_order_id: Option<String>,
    side: OrderSide,
    quantity: Option<f64>,
    above_type: OcoOrderType,
    #[serde(skip_serializing_if = "Option::is_none")]
    above_client_order_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    above_iceberg_qty: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    above_price: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    above_stop_price: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    above_trailing_delta: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    above_time_in_force: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    above_strategy_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    above_strategy_type: Option<i64>,
    below_type: OcoOrderType,
    #[serde(skip_serializing_if = "Option::is_none")]
    below_client_order_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    below_iceberg_qty: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    below_price: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    below_stop_price: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    below_trailing_delta: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    below_time_in_force: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    below_strategy_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    below_strategy_type: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    new_order_resp_type: Option<NewOrderRespType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    self_trade_prevention_mode: Option<SelfTradePreventionMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
    timestamp: i64,
}

impl NewOrderListOcoParams {
    pub fn new(
        symbol: &str,
        side: OrderSide,
        above_type: OcoOrderType,
        below_type: OcoOrderType,
    ) -> Self {
        Self {
            symbol: symbol.to_owned(),
            list_client_order_id: None,
            side,
            quantity: None,
            above_type,
            above_client_order_id: None,
            above_iceberg_qty: None,
            above_price: None,
            above_stop_price: None,
            above_trailing_delta: None,
            above_time_in_force: None,
            above_strategy_id: None,
            above_strategy_type: None,
            below_type,
            below_client_order_id: None,
            below_iceberg_qty: None,
            below_price: None,
            below_stop_price: None,
            below_trailing_delta: None,
            below_time_in_force: None,
            below_strategy_id: None,
            below_strategy_type: None,
            new_order_resp_type: None,
            self_trade_prevention_mode: None,
            recv_window: None,
            timestamp: Timestamp::now().as_millisecond(),
        }
    }

    pub fn list_client_order_id(mut self, list_client_order_id: &str) -> Self {
        self.list_client_order_id = Some(list_client_order_id.to_owned());
        self
    }

    pub fn quantity(mut self, quantity: f64) -> Self {
        self.quantity = Some(quantity);
        self
    }

    pub fn above_client_order_id(mut self, above_client_order_id: &str) -> Self {
        self.above_client_order_id = Some(above_client_order_id.to_owned());
        self
    }

    pub fn above_iceberg_qty(mut self, above_iceberg_qty: i64) -> Self {
        self.above_iceberg_qty = Some(above_iceberg_qty);
        self
    }

    pub fn above_price(mut self, above_price: f64) -> Self {
        self.above_price = Some(above_price);
        self
    }

    pub fn above_stop_price(mut self, above_stop_price: f64) -> Self {
        self.above_stop_price = Some(above_stop_price);
        self
    }

    pub fn above_trailing_delta(mut self, above_trailing_delta: f64) -> Self {
        self.above_trailing_delta = Some(above_trailing_delta);
        self
    }

    pub fn above_time_in_force(mut self, above_time_in_force: f64) -> Self {
        self.above_time_in_force = Some(above_time_in_force);
        self
    }

    pub fn above_strategy_id(mut self, above_strategy_id: i64) -> Self {
        self.above_strategy_id = Some(above_strategy_id);
        self
    }

    pub fn above_strategy_type(mut self, above_strategy_type: i64) -> Self {
        self.above_strategy_type = Some(above_strategy_type);
        self
    }

    pub fn below_client_order_id(mut self, below_client_order_id: &str) -> Self {
        self.below_client_order_id = Some(below_client_order_id.to_owned());
        self
    }

    pub fn below_iceberg_qty(mut self, below_iceberg_qty: i64) -> Self {
        self.below_iceberg_qty = Some(below_iceberg_qty);
        self
    }

    pub fn below_price(mut self, below_price: f64) -> Self {
        self.below_price = Some(below_price);
        self
    }

    pub fn below_stop_price(mut self, below_stop_price: f64) -> Self {
        self.below_stop_price = Some(below_stop_price);
        self
    }

    pub fn below_trailing_delta(mut self, below_trailing_delta: f64) -> Self {
        self.below_trailing_delta = Some(below_trailing_delta);
        self
    }

    pub fn below_time_in_force(mut self, below_time_in_force: f64) -> Self {
        self.below_time_in_force = Some(below_time_in_force);
        self
    }

    pub fn below_strategy_id(mut self, below_strategy_id: i64) -> Self {
        self.below_strategy_id = Some(below_strategy_id);
        self
    }

    pub fn below_strategy_type(mut self, below_strategy_type: i64) -> Self {
        self.below_strategy_type = Some(below_strategy_type);
        self
    }

    /// Set the response JSON. ACK, RESULT, or FULL; MARKET and LIMIT order types
    /// default to FULL, all other orders default to ACK.
    pub fn new_order_resp_type(mut self, new_order_resp_type: NewOrderRespType) -> Self {
        self.new_order_resp_type = Some(new_order_resp_type);
        self
    }

    /// The allowed enums is dependent on what is configured on the symbol. The
    /// possible supported values are EXPIRE_TAKER, EXPIRE_MAKER, EXPIRE_BOTH,
    /// NONE.
    pub fn self_trade_prevention_mode(
        mut self,
        self_trade_prevention_mode: SelfTradePreventionMode,
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
pub struct NewOrderListOcoResponse;
